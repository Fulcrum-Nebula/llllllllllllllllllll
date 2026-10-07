use typex_core::typex::*;
#[test]
fn scripts_full_tables_base_rules_and_recovery() {
    for (raw, preview) in [("x^2 + y_1", "x² + y₁"),("x^{12}+y_{34}","x¹²+y₃₄"),("x_{1}^{-2}","x₁⁻²"),("x^12","x¹2"),("x^{0123456789+-=()}","x⁰¹²³⁴⁵⁶⁷⁸⁹⁺⁻⁼⁽⁾"),("x_{0123456789+-=()}","x₀₁₂₃₄₅₆₇₈₉₊₋₌₍₎")] {
        assert_eq!(analyze(raw), Analysis {preview:preview.into(),issues:vec![]});
    }
    for (raw, preview, kind, start) in [("a_","a_",IssueKind::Incomplete,1),("x^{","x^{",IssueKind::Incomplete,1),("x^{q}","x^{q}",IssueKind::Unsupported,1),("x^2^3","x²^3",IssueKind::Malformed,3),("^2","^2",IssueKind::Malformed,0),("x ^2","x ^2",IssueKind::Malformed,2),("{}^2","^2",IssueKind::Malformed,2),("x^{}","x^{}",IssueKind::Malformed,1)] {
        assert_eq!(analyze(raw),Analysis{preview:preview.into(),issues:vec![Issue{kind,source:Span{start:ScalarOffset(start),end:ScalarOffset(raw.chars().count())}}]},"{raw}");
    }
}
