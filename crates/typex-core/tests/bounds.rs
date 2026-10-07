use typex_core::typex::*;
#[test]
fn analysis_bounds_and_unsupported_control_syntax() {
    for raw in ["a".repeat(4097), format!("{}a{}", "{".repeat(17), "}".repeat(17)), format!("\\foo{}", "{".repeat(17))] {
        assert_eq!(analyze(&raw), Analysis {preview:raw.clone(),issues:vec![Issue{kind:IssueKind::LimitExceeded,source:Span{start:ScalarOffset(0),end:ScalarOffset(raw.chars().count())}}]});
    }
    for raw in ["a".repeat(4096), format!("{}a{}", "{".repeat(16), "}".repeat(16))] { assert!(analyze(&raw).issues.is_empty()); }
    for raw in ["$x$", "$$", "a\nb", "\t", "\u{7f}", r"\%"] {
        let result = analyze(raw);
        assert_eq!(result.preview,raw);
        assert!(!result.issues.is_empty(),"{raw:?}");
        assert!(result.issues.iter().all(|i|i.kind == IssueKind::Unsupported),"{result:?}");
    }
}
