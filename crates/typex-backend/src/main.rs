#![forbid(unsafe_code)]
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, io::{self, BufRead, Write}};
use typex_core::{Disposition, typex::*};
const PROFILE: &str = "{D93F3E23-0F2D-4D7A-9F28-51DC82B37A60}";
#[derive(Deserialize)]
#[serde(rename_all="camelCase")]
struct Header { method: Method, seq_num: Option<u64> }
#[derive(Deserialize, PartialEq, Eq)]
#[serde(rename_all="camelCase")]
enum Method {
    Init, Close, FilterKeyDown, OnKeyDown, FilterKeyUp, OnKeyUp,
    OnActivate, OnDeactivate, OnKeyboardStatusChanged, OnCompositionTerminated,
    #[serde(other)] Other,
}
#[derive(Deserialize)]
#[serde(rename_all="camelCase")]
struct Sequence { seq_num: Option<u64> }
#[derive(Deserialize)]
#[serde(rename_all="camelCase")]
struct Init { id: String, is_windows8_above: bool, is_metro_app: bool, is_ui_less: bool, is_console: bool }
#[derive(Deserialize)]
#[serde(rename_all="camelCase")]
struct Activation { is_keyboard_open: bool }
#[derive(Deserialize)]
struct KeyboardStatus { opened: bool }
#[derive(Deserialize)]
struct Termination { forced: bool }
#[derive(Deserialize)]
#[serde(rename_all="camelCase")]
struct Key { char_code: u32, key_code: u8, repeat_count: u16, scan_code: u16, is_extended: bool, key_states: Vec<u8> }
impl Key {
    fn down(&self,k:usize)->bool { self.key_states[k]&128!=0 }
    fn event(&self)->Event {
        let ctrl=self.down(17)||self.down(162)||self.down(163);
        let shift=self.down(16)||self.down(160)||self.down(161);
        let alt=self.down(18)||self.down(164)||self.down(165);
        if alt||self.down(91)||self.down(92) {return Event::UnsupportedKey}
        if ctrl&&shift&&self.key_code==77 {return Event::Begin(SessionId(0))}
        if ctrl {return if self.key_code==13&&!shift {Event::Commit(CommitKind::Raw,EffectId(0))} else {Event::UnsupportedKey}}
        match self.key_code {
            13=>Event::Commit(CommitKind::Converted,EffectId(0)),27=>Event::Cancel,
            8=>Event::Backspace,46=>Event::Delete,37=>Event::Move(Direction::Left),39=>Event::Move(Direction::Right),36=>Event::Move(Direction::Home),35=>Event::Move(Direction::End),
            9|16..=18|20|91..=93|160..=165=>Event::UnsupportedKey,
            _=>char::from_u32(self.char_code).filter(|c| !c.is_control()).map_or(Event::UnsupportedKey,|c|Event::Insert(c.to_string()))
        }
    }
}
#[derive(Default, Serialize)]
#[serde(rename_all="camelCase")]
struct Reply {
    #[serde(skip_serializing_if="Option::is_none")] seq_num: Option<u64>, success: bool,
    #[serde(rename="return",skip_serializing_if="Option::is_none")] eaten: Option<bool>,
    #[serde(skip_serializing_if="Option::is_none")] composition_string: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")] composition_cursor: Option<usize>,
    #[serde(skip_serializing_if="Option::is_none")] commit_string: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")] candidate_list: Option<Vec<String>>,
    #[serde(skip_serializing_if="Option::is_none")] show_candidates: Option<bool>,
}
impl Reply {
    fn preview(&mut self,state:&State,notice:bool) {
        let v=view(state);
        let text=if notice||!v.analysis.issues.is_empty() {format!("{} — 未完成/不支持，Enter 不提交；Ctrl+Enter 原文",v.analysis.preview)} else {v.analysis.preview};
        self.candidate_list=Some(vec![text]); self.show_candidates=Some(true);
    }
    fn effects(&mut self,state:&State,effects:Vec<Effect>) {
        for e in effects {match e {
            Effect::SetPreedit{raw,cursor}=>{self.composition_string=Some(raw);self.composition_cursor=Some(cursor.0);self.preview(state,false)},
            Effect::ClearPreedit=>{self.composition_string=Some(String::new());self.show_candidates=Some(false)},
            Effect::SubmitInCurrentReply{text,..}=>{self.commit_string=Some(text);self.show_candidates=Some(false)},
            Effect::Notice(_)=>self.preview(state,true),
        }}
    }
}
#[derive(Default)]
struct Client { state:State, next_id:u64, last_seq:u64, supported:bool, opened:bool, held:std::collections::HashSet<(u8,u16,bool)> }
impl Client {
    fn reset(&mut self,reason:LifecycleReason) {self.state=step(&self.state,Event::Lifecycle(reason)).state;self.held.clear();}
}
fn handle(clients:&mut HashMap<String,Client>,client:&str,json:&str)->Reply {
    let mut r=Reply{seq_num:serde_json::from_str::<Sequence>(json).ok().and_then(|s|s.seq_num),..Reply::default()};
    let Ok(h)=serde_json::from_str::<Header>(json) else {return r};
    r.seq_num=h.seq_num;
    if h.method==Method::Close {if let Some(mut c)=clients.remove(client) {c.reset(LifecycleReason::Disconnected)}r.success=true;return r}
    let Some(seq)=h.seq_num else {return r};
    if h.method==Method::Init {
        clients.remove(client);
        if let Ok(i)=serde_json::from_str::<Init>(json) {
            if i.id.eq_ignore_ascii_case(PROFILE) {clients.insert(client.to_owned(),Client{last_seq:seq,supported:i.is_windows8_above&&!i.is_metro_app&&!i.is_ui_less&&!i.is_console,opened:true,..Client::default()});r.success=true;}
        }
        return r
    }
    let Some(c)=clients.get_mut(client) else {return r};
    if seq<=c.last_seq {return r} c.last_seq=seq;
    match h.method {
        Method::OnActivate=>{let Ok(a)=serde_json::from_str::<Activation>(json) else {return r};c.reset(LifecycleReason::ProfileClosed);c.opened=a.is_keyboard_open;},
        Method::OnDeactivate=>{c.reset(LifecycleReason::ProfileClosed);c.opened=false;},
        Method::OnKeyboardStatusChanged=>{let Ok(k)=serde_json::from_str::<KeyboardStatus>(json) else {return r};c.opened=k.opened;if !k.opened {c.reset(LifecycleReason::KeyboardClosed)}},
        Method::OnCompositionTerminated=>{let Ok(t)=serde_json::from_str::<Termination>(json) else {return r};c.reset(LifecycleReason::CompositionEnded{forced:t.forced});},
        _=>{}
    }
    if matches!(h.method,Method::OnKeyDown|Method::FilterKeyDown|Method::OnKeyUp|Method::FilterKeyUp) {
        let Ok(k)=serde_json::from_str::<Key>(json) else {return r};
        if k.key_states.len()!=256||k.repeat_count==0||char::from_u32(k.char_code).is_none() {return r}
        if !c.supported||!c.opened {r.success=true;r.eaten=Some(false);return r}
        let _=k.repeat_count;
        let physical=(k.key_code,k.scan_code,k.is_extended);
        if h.method==Method::FilterKeyUp||h.method==Method::OnKeyUp {
            r.eaten=Some(if h.method==Method::OnKeyUp {c.held.remove(&physical)} else {c.held.contains(&physical)});r.success=true;return r
        }
        let mut event=k.event();
        if h.method==Method::FilterKeyDown {r.eaten=Some(probe(&c.state,&event)==Disposition::Consume);r.success=true;return r}
        let Some(next)=c.next_id.checked_add(1) else {return r};c.next_id=next;
        match &mut event {Event::Begin(id)=>*id=SessionId(c.next_id),Event::Commit(_,id)=>*id=EffectId(c.next_id),_=>{}}
        let out=step(&c.state,event);c.state=out.state;r.eaten=Some(out.disposition==Disposition::Consume);
        if out.disposition==Disposition::Consume {c.held.insert(physical);}
        r.effects(&c.state,out.effects);
    }
    r.success=true;r
}
const MAX_LINE:usize=1_048_576;
// Keep at most MAX_LINE bytes even for an unterminated hostile stream.
// An oversized frame is drained through its newline before reading the next one.
fn frame<R:BufRead>(input:&mut R)->io::Result<Option<(Vec<u8>,bool)>> {
    let mut bytes=Vec::new();let mut oversized=false;
    loop {
        let chunk=input.fill_buf()?;
        if chunk.is_empty() {return Ok(None)} // incomplete final frame is never dispatched
        let end=chunk.iter().position(|b|*b==b'\n');
        let count=end.unwrap_or(chunk.len());
        let remaining=MAX_LINE.saturating_sub(bytes.len());
        bytes.extend_from_slice(&chunk[..count.min(remaining)]);
        oversized|=count>remaining;
        input.consume(count+usize::from(end.is_some()));
        if end.is_some() {return Ok(Some((bytes,oversized)))}
    }
}
fn run()->io::Result<()> {
    let input=io::stdin();let mut input=input.lock();let mut output=io::stdout().lock();let mut clients=HashMap::new();
    while let Some((bytes,oversized))=frame(&mut input)? {
        let Some(split)=bytes.iter().position(|b|*b==b'|') else {continue};
        let Ok(client)=std::str::from_utf8(&bytes[..split]) else {continue};
        if client.is_empty()||client.len()>256||client.bytes().any(|b|b.is_ascii_control()) {continue}
        let reply=if oversized {Reply::default()} else {match std::str::from_utf8(&bytes[split+1..]) {Ok(json)=>handle(&mut clients,client,json),Err(_)=>Reply::default()}};
        // Write/flush is an intent, NOT an acknowledgement. Any failure terminates
        // this process and drops all local clients; no text is retried or logged.
        write!(output,"PIME_MSG|{}|",client)?;serde_json::to_writer(&mut output,&reply)?;writeln!(output)?;output.flush()?;
    }
    Ok(())
}
fn main() {if run().is_err() {std::process::exit(1)}}
