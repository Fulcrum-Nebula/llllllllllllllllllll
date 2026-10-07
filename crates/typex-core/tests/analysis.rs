use typex_core::typex::*;
#[test]
fn groups_recover_outermost_opaque_spans() {
    assert_eq!(analyze(r"{\alpha{b}}"), Analysis { preview: "αb".into(), issues: vec![] });
    for (raw, preview, kind, start, end) in [
        ("abc{", "abc{", IssueKind::Incomplete, 3, 4),
        ("a}", "a}", IssueKind::Malformed, 1, 2),
        (r"\alpha+\foo{\beta}", r"α+\foo{\beta}", IssueKind::UnknownCommand, 7, 18),
        (r"{\alpha+\foo}", r"{\alpha+\foo}", IssueKind::UnknownCommand, 0, 13),
        (r"\foo{\alpha", r"\foo{\alpha", IssueKind::UnknownCommand, 0, 11),
    ] {
        assert_eq!(analyze(raw), Analysis { preview: preview.into(), issues: vec![issue_at(kind,start,end)] }, "{raw}");
    }
}
fn issue_at(kind: IssueKind, start: usize, end: usize) -> Issue {
    Issue { kind, source: Span { start: ScalarOffset(start), end: ScalarOffset(end) } }
}
#[test]
fn symbols_longest_commands_and_literal_preservation() {
    for (raw, expected) in [(r"\alpha+\beta", "α+β"), (r"\gamma \pi \rightarrow \infty \leq \neq", "γ π → ∞ ≤ ≠"), ("中文🙂e\u{301}", "中文🙂e\u{301}")] {
        assert_eq!(analyze(raw), Analysis { preview: expected.into(), issues: vec![] });
    }
    for (raw, kind) in [(r"\", IssueKind::Incomplete), (r"\al", IssueKind::Incomplete), (r"\alphabeta", IssueKind::UnknownCommand)] {
        assert_eq!(analyze(raw), Analysis { preview: raw.into(), issues: vec![Issue { kind, source: Span { start: ScalarOffset(0), end: ScalarOffset(raw.chars().count()) } }] });
    }
}
