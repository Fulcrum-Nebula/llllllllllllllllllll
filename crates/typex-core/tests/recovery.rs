use typex_core::typex::*;
#[test]
fn spaced_font_is_one_opaque_construct_with_independent_neighbor() {
    for name in ["mathbb","mathbf","mathit","mathrm"] {
        let opaque=format!("\\{name} {{\\alpha}}");
        let raw=format!("🙂{opaque}+\\beta");
        assert_eq!(analyze(&raw),Analysis {preview:format!("🙂{opaque}+β"),issues:vec![Issue {kind:IssueKind::Malformed,source:Span{start:ScalarOffset(1),end:ScalarOffset(1+opaque.chars().count())}}]});
    }
}
#[test]
fn opaque_recovery_prefers_outer_span_and_preserves_neighbors() {
    for (opaque,kind) in [
        (r"\foo{\alpha}{\beta}",IssueKind::UnknownCommand),
        (r"\frac{\foo}{\beta}",IssueKind::UnknownCommand),
        (r"{\alpha+\foo{\beta}}",IssueKind::UnknownCommand),
        (r"\mathbf{a{b}}",IssueKind::Unsupported),
        (r"^{12}",IssueKind::Malformed),
    ] {
        let raw=format!("🙂 {opaque}+\\alpha");
        assert_eq!(analyze(&raw),Analysis{preview:format!("🙂 {opaque}+α"),issues:vec![Issue{kind,source:Span{start:ScalarOffset(2),end:ScalarOffset(2+opaque.chars().count())}}]});
    }
    let raw=r"\alpha+{\beta+\foo{";
    assert_eq!(analyze(raw),Analysis{preview:r"α+{\beta+\foo{".into(),issues:vec![Issue{kind:IssueKind::Incomplete,source:Span{start:ScalarOffset(7),end:ScalarOffset(raw.chars().count())}}]});
}
#[test]
fn bounded_generated_sources_are_total_and_spans_are_ordered() {
    let alphabet=['\\','{','}','^','_','$','a','0','🙂',' ','\n'];
    for mut n in 0..14641usize {
        let mut raw=String::new();
        for _ in 0..4 { raw.push(alphabet[n % alphabet.len()]); n /= alphabet.len(); }
        let a=analyze(&raw);
        assert_eq!(a,analyze(&raw));
        let mut end=0;
        for i in &a.issues { assert!(i.source.start.0>=end); assert!(i.source.end.0>i.source.start.0); assert!(i.source.end.0<=4); end=i.source.end.0; }
    }
}
