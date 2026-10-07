use crate::{Disposition, Extension, Outcome};
use super::{analyze, Analysis, ScalarOffset};
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SessionId(pub u64);
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct EffectId(pub u64);
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Revision(pub u64);
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CommitKind { Converted, Raw }
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Draft { pub session: SessionId, pub revision: Revision, pub raw: String, pub cursor: ScalarOffset }
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum State { #[default] Idle, Editing(Draft) }
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Event { Begin(SessionId), Insert(String), Backspace, Delete, Move(Direction), Commit(CommitKind,EffectId), Cancel, Lifecycle(LifecycleReason), UnsupportedKey }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LifecycleReason { ProfileClosed, KeyboardClosed, CompositionEnded { forced: bool }, Disconnected }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction { Left, Right, Home, End }
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Effect { SetPreedit { raw: String, cursor: ScalarOffset }, ClearPreedit, SubmitInCurrentReply {id: EffectId, kind: CommitKind, text: String}, Notice(Notice) }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Notice { Empty, InvalidInput, Limit }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StateTag { Idle, Editing }
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct View { pub raw: String, pub cursor: ScalarOffset, pub analysis: Analysis, pub phase: StateTag }
pub type TyPeXOutcome = Outcome<State,Effect>;
#[derive(Clone, Copy, Debug, Default)]
pub struct TyPeX;
impl Extension for TyPeX {
    type State = State; type Event = Event; type View = View; type Effect = Effect;
    fn step(&self,state: &State,event: Event) -> TyPeXOutcome { step(state,event) }
    fn view(&self,state: &State) -> View { view(state) }
    fn probe(&self,state: &State,event: &Event) -> Disposition { probe(state,event) }
}
pub fn view(state: &State) -> View {
    let (raw,cursor,phase) = match state { State::Idle => (String::new(),ScalarOffset(0),StateTag::Idle), State::Editing(d) => (d.raw.clone(),d.cursor,StateTag::Editing) };
    let analysis = analyze(&raw); View { raw,cursor,analysis,phase }
}
pub fn probe(state: &State,event: &Event) -> Disposition {
    if matches!(event,Event::Lifecycle(_)) { return Disposition::NotKey; }
    if matches!(event,Event::UnsupportedKey) { return Disposition::Pass; }
    if matches!(event,Event::Begin(_)) || matches!(state,State::Editing(_)) { Disposition::Consume } else { Disposition::Pass }
}
pub fn step(state: &State,event: Event) -> TyPeXOutcome {
    let mut out = Outcome {state:state.clone(),disposition:probe(state,&event),effects:vec![]};
    match (state,event) {
        (_,Event::Lifecycle(_)) => out.state = State::Idle,
        (State::Editing(_),Event::Cancel) => { out.state = State::Idle; out.effects.push(Effect::ClearPreedit); }
        (State::Idle,Event::Begin(session)) => out.state = State::Editing(Draft {session,revision:Revision(0),raw:String::new(),cursor:ScalarOffset(0)}),
        (State::Editing(d),Event::Insert(text)) => {
            if text.chars().any(|c| c <= '\u{1f}' || c == '\u{7f}') {
                out.effects.push(Effect::Notice(Notice::InvalidInput)); return out;
            }
            if text.is_empty() { return out; }
            if d.revision.0 == u64::MAX || text.chars().count() > super::MAX_SCALARS.saturating_sub(d.raw.chars().count()) {
                out.effects.push(Effect::Notice(Notice::Limit)); return out;
            }
            let mut next = d.clone();
            let byte = next.raw.char_indices().nth(next.cursor.0).map_or(next.raw.len(), |x|x.0);
            next.raw.insert_str(byte,&text); next.cursor.0 += text.chars().count(); next.revision.0 += 1;
            out.effects.push(Effect::SetPreedit {raw:next.raw.clone(),cursor:next.cursor}); out.state = State::Editing(next);
        }
        (State::Editing(d),event @ (Event::Backspace | Event::Delete | Event::Move(_))) => {
            let mut chars: Vec<char> = d.raw.chars().collect();
            let mut cursor = d.cursor.0.min(chars.len());
            match event {
                Event::Backspace if cursor > 0 => { cursor -= 1; chars.remove(cursor); }
                Event::Delete if cursor < chars.len() => { chars.remove(cursor); }
                Event::Move(direction) => cursor = match direction {
                    Direction::Left => cursor.saturating_sub(1), Direction::Right => (cursor+1).min(chars.len()),
                    Direction::Home => 0, Direction::End => chars.len(),
                },
                _ => {}
            }
            let raw: String = chars.into_iter().collect();
            if raw != d.raw || cursor != d.cursor.0 {
                if d.revision.0 == u64::MAX { out.effects.push(Effect::Notice(Notice::Limit)); return out; }
                let next = Draft { raw, cursor: ScalarOffset(cursor), revision: Revision(d.revision.0+1), ..d.clone() };
                out.effects.push(Effect::SetPreedit{raw:next.raw.clone(),cursor:next.cursor});
                out.state = State::Editing(next);
            }
        }
        (State::Editing(d),Event::Commit(kind,id)) => {
            if d.raw.is_empty() { out.effects.push(Effect::Notice(Notice::Empty)); }
            else {
                let analysis = analyze(&d.raw);
                if kind == CommitKind::Converted && !analysis.issues.is_empty() { out.effects.push(Effect::Notice(Notice::InvalidInput)); }
                else {
                    let text = if kind == CommitKind::Raw { d.raw.clone() } else { analysis.preview };
                    out.effects.push(Effect::SubmitInCurrentReply{id,kind,text}); out.state = State::Idle;
                }
            }
        }
        _ => {}
    }
    out
}
