#[path = "state.rs"]
mod state;
pub use state::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScalarOffset(pub usize);
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Span { pub start: ScalarOffset, pub end: ScalarOffset }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IssueKind { Incomplete, UnknownCommand, Unsupported, Malformed, LimitExceeded }
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Issue { pub kind: IssueKind, pub source: Span }
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Analysis { pub preview: String, pub issues: Vec<Issue> }
const SYMBOLS: [(&str, &str); 8] = [("alpha","α"),("beta","β"),("gamma","γ"),("pi","π"),("rightarrow","→"),("infty","∞"),("leq","≤"),("neq","≠")];
const FUNCTIONS: [&str; 5] = ["mathbb", "mathbf", "mathit", "mathrm", "frac"];
fn issue(kind: IssueKind, start: usize, end: usize) -> Issue {
    Issue { kind, source: Span { start: ScalarOffset(start), end: ScalarOffset(end) } }
}
pub const MAX_SCALARS: usize = 4096;
pub const MAX_DEPTH: usize = 16;
pub fn analyze(raw: &str) -> Analysis {
    let length = raw.chars().count();
    let mut depth = 0usize;
    let mut exceeded = length > MAX_SCALARS;
    for c in raw.chars() {
        if c == '{' { depth += 1; exceeded |= depth > MAX_DEPTH; }
        if c == '}' { depth = depth.saturating_sub(1); }
    }
    if exceeded { return Analysis { preview: raw.into(), issues: vec![issue(IssueKind::LimitExceeded,0,length)] }; }
    Parser { chars: raw.chars().collect(), pos: 0 }.sequence(false)
}
// Mapping knowledge from UniTeX unicode-table.ts (MIT); see THIRD-PARTY-NOTICES.md.
fn font_char(name: &str, c: char) -> Option<char> {
    if !(c.is_ascii_alphanumeric() || " +-=()".contains(c)) { return None; }
    if name == "mathrm" || !c.is_ascii_alphanumeric() { return Some(c); }
    let (upper, lower, digit) = match name {
        "mathbb" => (0, 0x1d552, 0x1d7d8),
        "mathbf" => (0x1d400, 0x1d41a, 0x1d7ce),
        _ => (0x1d434, 0x1d44e, '0' as u32),
    };
    if name == "mathbb" && c.is_ascii_uppercase() { return "𝔸𝔹ℂ𝔻𝔼𝔽𝔾ℍ𝕀𝕁𝕂𝕃𝕄ℕ𝕆ℙℚℝ𝕊𝕋𝕌𝕍𝕎𝕏𝕐ℤ".chars().nth(c as usize - 'A' as usize); }
    if name == "mathit" && c == 'h' { return Some('ℎ'); }
    let code = if c.is_ascii_uppercase() { upper + c as u32 - 'A' as u32 } else if c.is_ascii_lowercase() { lower + c as u32 - 'a' as u32 } else { digit + c as u32 - '0' as u32 };
    char::from_u32(code)
}
struct Parser { chars: Vec<char>, pos: usize }
impl Parser {
    fn raw(&self, start: usize, end: usize) -> String { self.chars[start..end].iter().collect() }
    fn peek(&self) -> Option<char> { self.chars.get(self.pos).copied() }
    // Scan rather than interpret opaque arguments, retaining even unknown nested commands.
    fn opaque_group(&mut self) -> bool {
        let mut depth = 0;
        while let Some(c) = self.peek() {
            self.pos += 1;
            if c == '{' { depth += 1; }
            if c == '}' { depth -= 1; if depth == 0 { return true; } }
        }
        false
    }
    fn sequence(&mut self, nested: bool) -> Analysis {
        let mut result = Analysis { preview: String::new(), issues: vec![] };
        let mut base = false;
        let mut scripts = 0u8;
        while let Some(c) = self.peek() {
            if c == '}' && nested { break; }
            let start = self.pos;
            let is_script = c == '^' || c == '_';
            let bit = if c == '^' { 1 } else { 2 };
            let item = if is_script { self.script(base && scripts & bit == 0) } else { self.item() };
            match item {
                Ok(text) => {
                    if is_script { scripts |= bit; }
                    else { base = !text.is_empty() && !text.chars().all(char::is_whitespace); scripts = 0; }
                    result.preview.push_str(&text);
                }
                Err(kind) => {
                    base = false; scripts = 0;
                    result.preview.push_str(&self.raw(start,self.pos));
                    result.issues.push(issue(kind,start,self.pos));
                }
            }
        }
        result
    }
    fn script(&mut self, valid_base: bool) -> Result<String, IssueKind> {
        let upper = self.peek() == Some('^');
        self.pos += 1;
        let start = self.pos;
        let payload = if self.peek() == Some('{') {
            if !self.opaque_group() { return Err(IssueKind::Incomplete); }
            self.raw(start+1,self.pos-1)
        } else if self.peek().is_some_and(|c| c.is_ascii_digit()) {
            self.pos += 1; self.raw(start,self.pos)
        } else { return Err(if self.peek().is_none() { IssueKind::Incomplete } else { IssueKind::Malformed }); };
        if !valid_base || payload.is_empty() { return Err(IssueKind::Malformed); }
        let map = if upper { "⁰¹²³⁴⁵⁶⁷⁸⁹⁺⁻⁼⁽⁾" } else { "₀₁₂₃₄₅₆₇₈₉₊₋₌₍₎" };
        payload.chars().map(|c| "0123456789+-=()".chars().position(|x| x == c).and_then(|n|map.chars().nth(n)).ok_or(IssueKind::Unsupported)).collect()
    }
    fn group(&mut self) -> Result<String, IssueKind> {
        self.pos += 1;
        let result = self.sequence(true);
        if self.peek() != Some('}') { return Err(IssueKind::Incomplete); }
        self.pos += 1;
        if let Some(first) = result.issues.first() { Err(first.kind) } else { Ok(result.preview) }
    }
    fn required_group(&mut self) -> Result<String, IssueKind> {
        if self.peek() == Some('{') {
            let text = self.group()?;
            return if text.is_empty() { Err(IssueKind::Malformed) } else { Ok(text) };
        }
        if self.peek().is_none() { return Err(IssueKind::Incomplete); }
        while self.peek() == Some(' ') { self.pos += 1; }
        if self.peek() == Some('{') { self.opaque_group(); }
        Err(IssueKind::Malformed)
    }
    fn fraction(&mut self) -> Result<String, IssueKind> {
        let numerator = self.required_group();
        let denominator = self.required_group();
        let paren = |s: String| if s.trim_start_matches('-').chars().count() <= 1 { s } else { format!("({s})") };
        Ok(format!("{}/{}", paren(numerator?), paren(denominator?)))
    }
    fn font(&mut self, name: &str) -> Result<String, IssueKind> {
        if self.peek() != Some('{') {
            if self.peek().is_none() { return Err(IssueKind::Incomplete); }
            while self.peek() == Some(' ') { self.pos += 1; }
            if self.peek() == Some('{') { self.opaque_group(); }
            return Err(IssueKind::Malformed);
        }
        let start = self.pos;
        if !self.opaque_group() { return Err(IssueKind::Incomplete); }
        let payload = &self.chars[start+1..self.pos-1];
        if payload.is_empty() { return Err(IssueKind::Malformed); }
        payload.iter().map(|&c| font_char(name,c).ok_or(IssueKind::Unsupported)).collect()
    }
    fn item(&mut self) -> Result<String, IssueKind> {
        let c = self.peek().unwrap();
        if c == '{' { return self.group(); }
        self.pos += 1;
        if c == '}' { return Err(IssueKind::Malformed); }
        if c == '$' || c <= '\u{1f}' || c == '\u{7f}' { return Err(IssueKind::Unsupported); }
        if c != '\\' { return Ok(c.to_string()); }
        let name_start = self.pos;
        while self.peek().is_some_and(|c|c.is_ascii_alphabetic()) { self.pos += 1; }
        let name = self.raw(name_start,self.pos);
        if name.is_empty() && self.peek().is_some() { self.pos += 1; return Err(IssueKind::Unsupported); }
        if let Some((_, text)) = SYMBOLS.iter().find(|(key,_)| *key == name) { return Ok((*text).into()); }
        if name == "frac" { return self.fraction(); }
        if FUNCTIONS[..4].contains(&name.as_str()) { return self.font(&name); }
        let kind = if name.is_empty() || SYMBOLS.iter().map(|x|x.0).chain(FUNCTIONS).any(|x|x.starts_with(&name)) { IssueKind::Incomplete } else { IssueKind::UnknownCommand };
        while self.peek() == Some('{') { self.opaque_group(); }
        Err(kind)
    }
}
