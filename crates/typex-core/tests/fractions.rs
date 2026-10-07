use typex_core::typex::*;
#[test]
fn fractions_recursive_scalar_parens_and_whole_failure() {
    for (raw, text) in [(r"\frac{a+b}{c}", "(a+b)/c"),(r"\frac{1}{2}","1/2"),(r"\frac{-a}{bc}","-a/(bc)"),(r"\frac{\alpha}{\mathbf{A}}","α/𝐀"),(r"\frac{1}{\frac{2}{3}}","1/(2/3)")] {
        assert_eq!(analyze(raw), Analysis { preview: text.into(), issues: vec![] });
    }
    for (raw, kind) in [(r"\frac{",IssueKind::Incomplete),(r"\frac{a}{\foo}",IssueKind::UnknownCommand),(r"\frac{}{2}",IssueKind::Malformed),(r"\frac {a}{b}",IssueKind::Malformed),(r"\frac{\foo}{\alpha}",IssueKind::UnknownCommand)] {
        assert_eq!(analyze(raw), Analysis { preview: raw.into(), issues: vec![Issue {kind, source: Span {start: ScalarOffset(0),end: ScalarOffset(raw.chars().count())}}] });
    }
}
