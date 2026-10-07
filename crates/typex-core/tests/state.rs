use typex_core::{Disposition, Extension};
use typex_core::typex::*;
fn begin() -> State { step(&State::Idle, Event::Begin(SessionId(7))).state }
fn insert(state: &State, text: &str) -> State { step(state,Event::Insert(text.into())).state }
#[test]
fn begin_insert_converted_intent_is_pure_and_not_replayed() {
    let begun = step(&State::Idle, Event::Begin(SessionId(7)));
    assert_eq!(begin(), begun.state);
    assert_eq!(view(&insert(&begun.state, "a")).raw, "a");
    assert_eq!(begun.disposition,Disposition::Consume);
    assert!(begun.effects.is_empty());
    let edited = step(&begun.state,Event::Insert(r"\alpha".into()));
    assert_eq!(edited.effects,vec![Effect::SetPreedit{raw:r"\alpha".into(),cursor:ScalarOffset(6)}]);
    let v = view(&edited.state);
    assert_eq!(v.raw,r"\alpha"); assert_eq!(v.analysis.preview,"α"); assert_eq!(v.phase,StateTag::Editing);
    let event = Event::Commit(CommitKind::Converted,EffectId(9));
    assert_eq!(probe(&edited.state,&event),Disposition::Consume);
    let result = step(&edited.state,event.clone());
    assert_eq!(result,step(&edited.state,event.clone()));
    assert_eq!(result,TyPeX.step(&edited.state,event.clone()));
    assert_eq!(TyPeX.view(&edited.state),v);
    assert_eq!(TyPeX.probe(&edited.state,&event),Disposition::Consume);
    assert_eq!(result.state,State::Idle);
    assert_eq!(result.effects,vec![Effect::SubmitInCurrentReply{id:EffectId(9),kind:CommitKind::Converted,text:"α".into()}]);
    let repeat = step(&result.state,event);
    assert_eq!(repeat.disposition,Disposition::Pass); assert!(repeat.effects.is_empty());
}
