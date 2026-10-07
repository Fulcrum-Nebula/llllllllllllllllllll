# TyPeX 纯核心完成报告与 API 冻结

规范：`typex.windows.demo/2-route-a`。范围仅 `crates/typex-core/**` 与本报告；没有修改 adapter、installer、根配置或上游，没有 commit/push。Linux 核心完成；**Windows 所有 gate NOT RUN**，不是 Installable-demo-ready，decoder/ClientEpoch 隔离仍由 adapter 验证。

## 实际执行

工具链：rustc 1.98.1 / cargo 1.98.1。零外部依赖，`#![forbid(unsafe_code)]`。

```sh
CARGO_HOME=/tmp/typex-core-cargo-home cargo test --offline --manifest-path crates/typex-core/Cargo.toml
CARGO_HOME=/tmp/typex-core-cargo-home cargo test --offline --release --manifest-path crates/typex-core/Cargo.toml
CARGO_HOME=/tmp/typex-core-cargo-home cargo doc --offline --no-deps --manifest-path crates/typex-core/Cargo.toml
```

三项退出码均为 0；debug/release **18 tests passed、0 failed**。其中 golden 一个测试遍历 C01–C32，逐一比较完整 Analysis（preview、issue 类型、scalar span），验证每例 Converted 与 Raw；生成测试枚举 14,641 个四 scalar 源串检查确定性、无 panic、issue 边界/顺序。另覆盖字体全部表项、脚标完整表、分数递归、非 BMP、非 NFC、4096/4097、16/17 层、所有 C0/DEL、revision 上限、纯 probe 与独立 State。

最终日志：`crates/typex-core/evidence/final-{debug,release,doc}.txt`。环境未安装 rustfmt，写文件工具的 lint 提示是工具缺失；没有声称通过格式化检查。编译/测试/rustdoc 成功。

### 接续 RED → GREEN 证据

均在 `crates/typex-core/evidence/`；早期 01–08 是前一 worker 留存，本轮没有将其冒充重新执行。

| 切片 | 实际 RED | 实际 GREEN |
|---|---|---|
| 09 提交 | `09-resumed-red.txt`，cargo 101，缺 Notice/Effect::Notice | `09-green.txt`，0；空提交/invalid Converted 拒绝，Raw 原串 |
| 10 scalar 编辑 | `10-red.txt`，101，缺编辑事件/Direction | `10-green.txt`，0；移动、夹紧、前后删除、raw preedit |
| 11 原子限制 | `11-red.txt`，101，超限 Insert 未拒绝 | `11-green.txt`，0；长度/控制符/空 Insert/revision overflow |
| 12 生命周期 | `12-red.txt`，101，缺 Cancel/Lifecycle/UnsupportedKey | `12-green.txt`，0；无目标文本 effect 的清理、纯 probe |
| 13 golden 完整覆盖 | `13-golden.txt` 首次即通过；是已有行为回归验证，不伪称新 RED | 全部 C01–C32 精确 span/提交验证通过 |
| 14 失败构造恢复 | `14-red.txt`，101；`\\mathbb {\\alpha}` 错误变为 `\\mathbb α`、span 过短 | `14-green.txt`，0；四种字体均整体 opaque，相邻 beta 仍转换 |

修复主要位于 `src/state.rs`、`src/typex.rs`；新增 `tests/{editing,lifecycle,golden,recovery,invariants}.rs`；保留并通过原有测试。`THIRD-PARTY-NOTICES.md` 已附从只读 UniTeX LICENSE 核实的完整 MIT 文本（2023 kokic），说明表与单行分数知识来源及差异。打包方需随包携带此文件。

## 冻结 API（adapter 可直接依赖）

crate：Cargo 包名 `typex-core`，Rust 路径 `typex_core`。根导出 `Disposition::{Pass,Consume,NotKey}`、`Outcome<S,E> { state:S, disposition:Disposition, effects:Vec<E> }`、`Extension` 与 `dispatch`。数学 API 全部位于 `typex_core::typex::*`（state 模块本身私有并 re-export）。

```rust
pub fn analyze(raw: &str) -> Analysis;
pub fn step(state: &State, event: Event) -> TyPeXOutcome;
pub fn view(state: &State) -> View;
pub fn probe(state: &State, event: &Event) -> Disposition;
pub type TyPeXOutcome = Outcome<State, Effect>;
pub struct TyPeX; // Clone, Copy, Debug, Default; impl Extension

// 以下字段/tuple 内值均公开；String 原生保证有效 Unicode。
pub struct SessionId(pub u64);
pub struct EffectId(pub u64);
pub struct Revision(pub u64);
pub struct ScalarOffset(pub usize);
pub struct Span { pub start: ScalarOffset, pub end: ScalarOffset }
pub enum IssueKind { Incomplete, UnknownCommand, Unsupported, Malformed, LimitExceeded }
pub struct Issue { pub kind: IssueKind, pub source: Span }
pub struct Analysis { pub preview: String, pub issues: Vec<Issue> }
pub struct Draft {
    pub session: SessionId, pub revision: Revision,
    pub raw: String, pub cursor: ScalarOffset,
}
pub enum State { Idle, Editing(Draft) } // Default = Idle
pub enum Direction { Left, Right, Home, End }
pub enum CommitKind { Converted, Raw }
pub enum LifecycleReason {
    ProfileClosed, KeyboardClosed, CompositionEnded { forced: bool }, Disconnected,
}
pub enum Event {
    Begin(SessionId), Insert(String), Backspace, Delete, Move(Direction),
    Commit(CommitKind, EffectId), Cancel, Lifecycle(LifecycleReason), UnsupportedKey,
}
pub enum Notice { Empty, InvalidInput, Limit }
pub enum Effect {
    SetPreedit { raw: String, cursor: ScalarOffset }, ClearPreedit,
    SubmitInCurrentReply { id: EffectId, kind: CommitKind, text: String }, Notice(Notice),
}
pub enum StateTag { Idle, Editing }
pub struct View {
    pub raw: String, pub cursor: ScalarOffset,
    pub analysis: Analysis, pub phase: StateTag,
}
pub const MAX_SCALARS: usize = 4096;
pub const MAX_DEPTH: usize = 16;
```

`Extension` 的关联类型为 `State/Event/View/Effect`，方法为 `step(&self,&Self::State,Self::Event)->Outcome<Self::State,Self::Effect>`、`view(&self,&Self::State)->Self::View`、`probe(&self,&Self::State,&Self::Event)->Disposition`；`dispatch<X:Extension>(&X,&X::State,X::Event)` 返回相应 Outcome。generic dummy 非数学 Extension 已测试，不在通用层写数学分支。

### 接入约束

- 固定配置，没有 Config 参数。probe 用 `&Event` 而不是额外 Key 类型；adapter 解码为同种 Event 后预测，不调用 step，不产生 effect/ID。Begin 的实际 SessionId、Commit 的实际 EffectId 在 down 分配，filter 可用不落状态的占位值；ID 不影响 disposition。
- adapter 从 `State::Idle` 开始，仅保留 step 返回的 state；不要自行造不满足 `cursor <= raw.chars().count()`、长度/控制符约束的 Draft。公开 Draft 便于审计/测试，不意味着任意外部字段组合是合法输入状态。
- offsets 为 scalar，非 UTF-8 bytes/UTF-16 code units；删除组合字符按 scalar。revision 初始 0，仅实际 raw/cursor 变化递增；空 Insert/边界 no-op 不变。需要修改而 revision=u64::MAX 时保持状态并 Notice(Limit)；commit/cancel/lifecycle 不受其阻塞。
- SetPreedit 仅 raw，不能换成 preview；View 不反写。无效 Insert 整次拒绝。Converted 遵循规范明确条件：raw 非空且 issues 为空；普通完整空 group 可去括号，因此 `{}` 的 preview 可为空，并非 InvalidInput。Raw 始终直接复制 raw，不 normalization。
- Cancel 仅 Editing 产生 ClearPreedit；Lifecycle 任意状态回 Idle、NotKey、**无任何 effect**；UnsupportedKey 永远 Pass。Editing 重复 Begin 不覆盖。成功 Commit 只一个 SubmitInCurrentReply 然后 Idle，再次 Enter Pass。
- ClientEpoch、wire JSON/UTF-8/孤立 surrogate 拒绝、seqNum/filter/up 跟踪、连接隔离由 adapter 持有；核心没有 ClientEpoch 类型或 native context token。核心 String 不可能包含孤立 surrogate，但不能替代 wire decoder 的严格拒绝测试。
- SubmitInCurrentReply 只是当前回复提交意图，不是 Applied/ack。失败不自动重试，禁止声称 exactly-once 或原字段身份保证。

## 最终源码 SHA-256

```text
f0036158e5fa6c50c4c419f83a2827bab1a095125f33ea9537c0b6e8c92ab74e  src/lib.rs
7f545dcf93f4fcb72f2fb551a7f73a3cc311b271f66497acb6437db1a620aab4  src/state.rs
0c63a82f0e43bb8eb674d7ecdb674ca2a86c1b45032bdeb1687b1379407b6b18  src/typex.rs
```

限制：无 Windows/PIME/TSF/installer/性能 RPC 证据；A03 wire 解码、A04 ClientEpoch 和所有 Windows 部分不能由本报告升级为 PASS。
