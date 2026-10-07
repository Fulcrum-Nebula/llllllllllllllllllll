use typex_core::typex::*;
use typex_core::Disposition;
use IssueKind::*;
#[test]
fn c01_c32_exact_preview_issue_spans_and_both_commit_paths() {
    let cases: &[(&str,&str,&str,&[(IssueKind,usize,usize)])] = &[
        ("C01",r"\alpha+\beta","α+β",&[]),
        ("C02",r"\gamma \pi \rightarrow \infty \leq \neq","γ π → ∞ ≤ ≠",&[]),
        ("C03",r"\mathbb{R}","ℝ",&[]),
        ("C04",r"\mathbf{Az09}","𝐀𝐳𝟎𝟗",&[]),
        ("C05",r"\mathit{h}","ℎ",&[]),
        ("C06",r"\mathrm{AB 12}","AB 12",&[]),
        ("C07",r"x^2 + y_1","x² + y₁",&[]),
        ("C08",r"x^{12}+y_{34}","x¹²+y₃₄",&[]),
        ("C09",r"x_{1}^{-2}","x₁⁻²",&[]),
        ("C10",r"\frac{a+b}{c}","(a+b)/c",&[]),
        ("C11",r"\frac{1}{2}","1/2",&[]),
        ("C12",r"\frac{-a}{bc}","-a/(bc)",&[]),
        ("C13",r"\frac{\alpha}{\mathbf{A}}","α/𝐀",&[]),
        ("C14",r"\frac{1}{\frac{2}{3}}","1/(2/3)",&[]),
        ("C15","中文🙂","中文🙂",&[]),
        ("C16",r"\",r"\",&[(Incomplete,0,1)]),
        ("C17",r"\al",r"\al",&[(Incomplete,0,3)]),
        ("C18",r"\alphabeta",r"\alphabeta",&[(UnknownCommand,0,10)]),
        ("C19",r"\frac{",r"\frac{",&[(Incomplete,0,6)]),
        ("C20","a_","a_",&[(Incomplete,1,2)]),
        ("C21","x^{","x^{",&[(Incomplete,1,3)]),
        ("C22","abc{","abc{",&[(Incomplete,3,4)]),
        ("C23",r"\alpha+\foo{\beta}",r"α+\foo{\beta}",&[(UnknownCommand,7,18)]),
        ("C24",r"\frac{a}{\foo}",r"\frac{a}{\foo}",&[(UnknownCommand,0,14)]),
        ("C25","x^{q}","x^{q}",&[(Unsupported,1,5)]),
        ("C26",r"\mathbb{\alpha}",r"\mathbb{\alpha}",&[(Unsupported,0,15)]),
        ("C27",r"\frac{}{2}",r"\frac{}{2}",&[(Malformed,0,10)]),
        ("C28","a}","a}",&[(Malformed,1,2)]),
        ("C29","x^2^3","x²^3",&[(Malformed,3,5)]),
        ("C30","x^12","x¹2",&[]),
        ("C31","$x$","$x$",&[(Unsupported,0,1),(Unsupported,2,3)]),
        ("C32",r"\frac {a}{b}",r"\frac {a}{b}",&[(Malformed,0,12)]),
    ];
    for &(id,raw,preview,issues) in cases {
        let expected = Analysis {preview:preview.into(),issues:issues.iter().map(|&(kind,start,end)| Issue {kind,source:Span{start:ScalarOffset(start),end:ScalarOffset(end)}}).collect()};
        assert_eq!(analyze(raw),expected,"{id}");
        let begin=step(&State::Idle,Event::Begin(SessionId(1))).state;
        let state=step(&begin,Event::Insert(raw.into())).state;
        let out=step(&state,Event::Commit(CommitKind::Converted,EffectId(2)));
        assert_eq!(out.disposition,Disposition::Consume,"{id}");
        if issues.is_empty() {
            assert_eq!(out.state,State::Idle,"{id}");
            assert_eq!(out.effects,vec![Effect::SubmitInCurrentReply{id:EffectId(2),kind:CommitKind::Converted,text:preview.into()}],"{id}");
        } else {
            assert_eq!(out.state,state,"{id}");
            assert_eq!(out.effects,vec![Effect::Notice(Notice::InvalidInput)],"{id}");
        }
        let raw_out=step(&state,Event::Commit(CommitKind::Raw,EffectId(3)));
        assert_eq!(raw_out.state,State::Idle,"{id}");
        assert_eq!(raw_out.effects,vec![Effect::SubmitInCurrentReply{id:EffectId(3),kind:CommitKind::Raw,text:raw.into()}],"{id}");
    }
}
