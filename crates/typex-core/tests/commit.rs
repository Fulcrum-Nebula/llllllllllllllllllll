use typex_core::{Disposition};
use typex_core::typex::*;
#[test]
fn invalid_converted_retains_draft_raw_commits_exact_source_and_empty_rejected() {
    let begin = step(&State::Idle,Event::Begin(SessionId(1))).state;
    for kind in [CommitKind::Raw,CommitKind::Converted] {
        let out = step(&begin,Event::Commit(kind,EffectId(1)));
        assert_eq!(out.state,begin); assert_eq!(out.effects,vec![Effect::Notice(Notice::Empty)]);
    }
    for raw in [r"\frac{",r"\foo{\alpha}",r"\alpha", "e\u{301}"] {
        let draft = step(&begin,Event::Insert(raw.into())).state;
        let converted = step(&draft,Event::Commit(CommitKind::Converted,EffectId(2)));
        if !analyze(raw).issues.is_empty() {
            assert_eq!(converted.state,draft); assert_eq!(converted.effects,vec![Effect::Notice(Notice::InvalidInput)]);
        }
        let out = step(&draft,Event::Commit(CommitKind::Raw,EffectId(3)));
        assert_eq!(out.state,State::Idle); assert_eq!(out.disposition,Disposition::Consume);
        assert_eq!(out.effects,vec![Effect::SubmitInCurrentReply{id:EffectId(3),kind:CommitKind::Raw,text:raw.into()}]);
    }
}
