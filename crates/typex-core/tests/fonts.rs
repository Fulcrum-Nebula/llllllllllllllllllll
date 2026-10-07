use typex_core::typex::*;
#[test]
fn fonts_complete_maps_and_opaque_invalid_payloads() {
    let input = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789 +-=()";
    let tables = [
        ("mathbb", "𝔸𝔹ℂ𝔻𝔼𝔽𝔾ℍ𝕀𝕁𝕂𝕃𝕄ℕ𝕆ℙℚℝ𝕊𝕋𝕌𝕍𝕎𝕏𝕐ℤ𝕒𝕓𝕔𝕕𝕖𝕗𝕘𝕙𝕚𝕛𝕜𝕝𝕞𝕟𝕠𝕡𝕢𝕣𝕤𝕥𝕦𝕧𝕨𝕩𝕪𝕫𝟘𝟙𝟚𝟛𝟜𝟝𝟞𝟟𝟠𝟡 +-=()"),
        ("mathbf", "𝐀𝐁𝐂𝐃𝐄𝐅𝐆𝐇𝐈𝐉𝐊𝐋𝐌𝐍𝐎𝐏𝐐𝐑𝐒𝐓𝐔𝐕𝐖𝐗𝐘𝐙𝐚𝐛𝐜𝐝𝐞𝐟𝐠𝐡𝐢𝐣𝐤𝐥𝐦𝐧𝐨𝐩𝐪𝐫𝐬𝐭𝐮𝐯𝐰𝐱𝐲𝐳𝟎𝟏𝟐𝟑𝟒𝟓𝟔𝟕𝟖𝟗 +-=()"),
        ("mathit", "𝐴𝐵𝐶𝐷𝐸𝐹𝐺𝐻𝐼𝐽𝐾𝐿𝑀𝑁𝑂𝑃𝑄𝑅𝑆𝑇𝑈𝑉𝑊𝑋𝑌𝑍𝑎𝑏𝑐𝑑𝑒𝑓𝑔ℎ𝑖𝑗𝑘𝑙𝑚𝑛𝑜𝑝𝑞𝑟𝑠𝑡𝑢𝑣𝑤𝑥𝑦𝑧0123456789 +-=()"),
        ("mathrm", input),
    ];
    for (font, expected) in tables {
        for (c, mapped) in input.chars().zip(expected.chars()) {
            assert_eq!(analyze(&format!("\\{font}{{{c}}}")), Analysis { preview: mapped.to_string(), issues: vec![] });
        }
        for payload in [r"\alpha", "a{b}", "🙂", "a/b"] {
            let raw = format!("\\{font}{{{payload}}}");
            let a = analyze(&raw);
            assert_eq!(a.preview, raw);
            assert_eq!(a.issues[0].kind, IssueKind::Unsupported);
            assert_eq!(a.issues.len(), 1);
        }
        assert_eq!(analyze(&format!("\\{font}{{}}")).issues[0].kind, IssueKind::Malformed);
    }
}
