use typex_core::{Disposition, Extension};
use typex_core::typex::*;
#[test]
fn lifecycle_and_cancel_never_submit_or_clear_unknown_target() {
    let begin = step(&State::Idle,Event::Begin(SessionId(1))).state;
    let editing = step(&begin,Event::Insert(r"\alpha".into())).state;
    for s in [&State::Idle,&begin,&editing] {
        for reason in [LifecycleReason::ProfileClosed,LifecycleReason::KeyboardClosed,LifecycleReason::CompositionEnded{forced:false},LifecycleReason::CompositionEnded{forced:true},LifecycleReason::Disconnected] {
            let event = Event::Lifecycle(reason);
            assert_eq!(probe(s,&event),Disposition::NotKey);
            let out = step(s,event);
            assert_eq!(out.state,State::Idle); assert_eq!(out.disposition,Disposition::NotKey); assert!(out.effects.is_empty());
        }
    }
    let out = step(&editing,Event::Cancel);
    assert_eq!(out.state,State::Idle); assert_eq!(out.disposition,Disposition::Consume); assert_eq!(out.effects,vec![Effect::ClearPreedit]);
    assert_eq!(step(&editing,Event::Begin(SessionId(99))).state,editing);
}
#[test]
fn probe_is_read_only_and_matches_step_for_every_event() {
    let editing=step(&State::Idle,Event::Begin(SessionId(1))).state;
    for s in [State::Idle,editing] {
        for event in [Event::Begin(SessionId(2)),Event::Insert("a".into()),Event::Backspace,Event::Delete,Event::Move(Direction::Home),Event::Commit(CommitKind::Raw,EffectId(1)),Event::Cancel,Event::UnsupportedKey,Event::Lifecycle(LifecycleReason::Disconnected)] {
            let copy=s.clone();
            let p=TyPeX.probe(&s,&event);
            assert_eq!(TyPeX.probe(&s,&event),p); assert_eq!(s,copy);
            let out=step(&s,event.clone());
            assert_eq!(out.disposition,p); assert_eq!(out,step(&s,event.clone()));
            if matches!(event,Event::UnsupportedKey) { assert_eq!(p,Disposition::Pass); assert_eq!(out.state,s); assert!(out.effects.is_empty()); }
            if s==State::Idle && !matches!(event,Event::Begin(_) | Event::Lifecycle(_)) { assert_eq!(p,Disposition::Pass); assert!(out.effects.is_empty()); }
        }
    }
}
