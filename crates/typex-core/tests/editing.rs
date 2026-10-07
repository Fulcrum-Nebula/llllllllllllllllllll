use typex_core::{Disposition};
use typex_core::typex::*;
fn draft(raw: &str, cursor: usize, revision: u64) -> State {
    State::Editing(Draft { session: SessionId(7), revision: Revision(revision), raw: raw.into(), cursor: ScalarOffset(cursor) })
}
#[test]
fn insert_limits_controls_and_revision_exhaustion_are_atomic() {
    let s = draft(&"🙂".repeat(MAX_SCALARS), MAX_SCALARS, 10);
    let out = step(&s, Event::Insert("a".into()));
    assert_eq!(out.state,s); assert_eq!(out.effects,vec![Effect::Notice(Notice::Limit)]);
    for c in (0u8..=31).chain([127]) {
        let out = step(&s,Event::Insert(format!("a{}b",c as char)));
        assert_eq!(out.state,s); assert_eq!(out.effects,vec![Effect::Notice(Notice::InvalidInput)]);
    }
    let empty = draft("",0,0);
    let full = step(&empty,Event::Insert("🙂".repeat(MAX_SCALARS)));
    assert_eq!(full.state,draft(&"🙂".repeat(MAX_SCALARS),MAX_SCALARS,1));
    assert_eq!(step(&empty,Event::Insert(String::new())).state,empty);
    let exhausted = draft("ab",1,u64::MAX);
    for event in [Event::Insert("x".into()),Event::Backspace,Event::Delete,Event::Move(Direction::Left),Event::Move(Direction::End)] {
        let out = step(&exhausted,event);
        assert_eq!(out.state,exhausted); assert_eq!(out.effects,vec![Effect::Notice(Notice::Limit)]);
    }
    let almost = draft("a",1,u64::MAX-1);
    assert_eq!(step(&almost,Event::Backspace).state,draft("",0,u64::MAX));
}
#[test]
fn scalar_editing_and_clamped_moves() {
    let mut s = draft("𝐀𝐁x",3,0);
    for (event,raw,cursor,revision) in [
        (Event::Move(Direction::Home),"𝐀𝐁x",0,1),
        (Event::Backspace,"𝐀𝐁x",0,1),
        (Event::Delete,"𝐁x",0,2),
        (Event::Move(Direction::Right),"𝐁x",1,3),
        (Event::Insert("🙂".into()),"𝐁🙂x",2,4),
        (Event::Move(Direction::Left),"𝐁🙂x",1,5),
        (Event::Delete,"𝐁x",1,6),
        (Event::Move(Direction::End),"𝐁x",2,7),
        (Event::Move(Direction::Right),"𝐁x",2,7),
        (Event::Delete,"𝐁x",2,7),
        (Event::Backspace,"𝐁",1,8),
    ] {
        let out = step(&s,event);
        assert_eq!(out.disposition,Disposition::Consume);
        assert_eq!(out.state,draft(raw,cursor,revision));
        if out.state == s { assert!(out.effects.is_empty()); }
        else { assert_eq!(out.effects,vec![Effect::SetPreedit{raw:raw.into(),cursor:ScalarOffset(cursor)}]); }
        s=out.state;
    }
}
