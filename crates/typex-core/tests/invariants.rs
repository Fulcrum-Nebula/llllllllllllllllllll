use typex_core::typex::*;
#[test]
fn independent_sessions_and_scalar_invariants_through_event_sequences() {
    let a=step(&State::Idle,Event::Begin(SessionId(1))).state;
    let b=step(&State::Idle,Event::Begin(SessionId(2))).state;
    let b=step(&b,Event::Insert("unchanged".into())).state;
    let snapshot=b.clone();
    let mut s=a;
    let events=[Event::Insert("🙂🙂a".into()),Event::Move(Direction::Home),Event::Delete,Event::Move(Direction::Right),Event::Backspace,Event::Insert("e\u{301}".into()),Event::Move(Direction::End),Event::Move(Direction::Left),Event::UnsupportedKey];
    for _ in 0..50 {
        for event in &events {
            let before=s.clone();
            let out=step(&s,event.clone());
            assert_eq!(out,step(&s,event.clone()));
            assert_eq!(s,before);
            s=out.state;
            if let State::Editing(d)=&s {
                assert!(d.cursor.0<=d.raw.chars().count());
                assert!(d.raw.chars().count()<=MAX_SCALARS);
                assert_eq!(d.session,SessionId(1));
                assert_eq!(view(&s).raw,d.raw);
            }
            assert_eq!(b,snapshot);
        }
    }
    let decomposed=step(&State::Editing(Draft{session:SessionId(3),revision:Revision(0),raw:"e\u{301}".into(),cursor:ScalarOffset(2)}),Event::Backspace);
    assert_eq!(view(&decomposed.state).raw,"e");
}
