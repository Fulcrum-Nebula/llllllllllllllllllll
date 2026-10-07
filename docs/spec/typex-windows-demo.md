# TyPeX Windows 可安装演示规范（Markdown SNL Entries）

版本：`typex.windows.demo/2-route-a`。状态：**路线 A 已获用户选择（“先 A 搞一版看看”）；本次仅规范修订，文档交接冻结，细节待评审，不代表实现或验收通过**。

本文件是本切片的规范性输入，先于测试与实现；`spikes/001-typex` 只是研究证据。以下每个二级标题给出稳定 Entry ID；`kind: entry` 使用现有 `.SNL_Doc/config.json` 的通用条目种类，`role` 说明工程语义，不新增 CAS 种类。本次仅交付 Markdown，不声称已导入 `.SNL_Doc` 或通过 SNL DSL 校验。以后迁入 `content.markdown` 时保留 ID，修订正文而不重编号。

“必须/不得”是验收要求；“延期”不是隐含实现。规范变更须先修改关联 Entry，再修改测试，最后修改实现。

## TyPeX.WinDemo.Scope — 产品边界

- kind: entry; role: requirement
- depends-on: 无

20l 是通用、函数式输入引擎；TyPeX 是首个数学 Extension。第一件产品交付物必须是能在 **Windows 11 x64 普通桌面会话**安装、选择、输入、卸载的真实 TSF IME，而非终端 REPL、剪贴板脚本或网页输入框。

此版采用用户选择的 **路线 A：官方 PIME 1.3.0 stable 为外部前置依赖，TyPeX 为独立强类型 Rust stdio backend/plugin 演示**。不修改、重编译或私带替换 TSF DLL/launcher；不要求新增 native bridge，不承诺独立 CLSID、独立 PIME 根目录或 pipe namespace。官方 PIME 提供 x86/x64 TSF 组件，插件负责自身 backend、manifest、profile 与安装整合；这不是无需安装验证的即插即用承诺。Linux 仅供研究/纯核心测试。Windows 10、ARM64、UWP/UI-less/控制台、提权应用、远程桌面不在支持范围；可识别不支持模式时旁路，其他安全性取决于 Windows gate，不预先保证。

演示功能：显式开启单行数学组字；源串编辑；有界 TeX 子集到 Unicode 预览；同步键响应中的转换提交或原文提交；取消；插件安装/卸载。跨字段不串字是 Windows 必验条件，不是 backend 已有的上下文身份保证。无候选排序、预测、自动补全、在线服务、剪贴板读写或插件输入日志；官方宿主日志另须检查。

**明确延期**：`/tex` 等 slash command 分发、TeX/Unicode 持久模式切换、KaTeX/WebView2 富预览、弹出公式编辑器、父子编辑会话、编辑器内另一 IME、异步返回到原目标、用户宏、中文 Extension、通用动态插件 ABI。它们仍是后续产品方向；普通 TSF composition/候选窗口不等于已实现富编辑器。此版原文提交不是完整 TeX 模式。

## TyPeX.WinDemo.Evidence — 源码依据及差异

- kind: entry; role: evidence
- depends-on: TyPeX.WinDemo.Scope

只读来源：

| 来源 | 固定版本/文件 | 用途 |
|---|---|---|
| UniTeX | `6e6294bdfb6101eb6e3dc440e1b9cb5a1a1efdc0`，`/home/sky/workspace/luye/UniTeX` | 转换知识，不是 IME 状态机 |
| UniTeX | `unitex.ts`; `src/impl/unicode.ts` | parse 导出、表装配、最终 NFC |
| UniTeX | `src/impl/unicode/{fixed,unary,binary,proper,unicode-table}.ts` | 符号、数学字体、上下标、单行分数规则 |
| PIME | 官方 `v1.3.0-stable`，`26fcf6ac8874e76b8f75f6826811b03bfdfc2e89` | 本版 stable 源码基线；不得以 HEAD 替代 |
| PIME | `PIMELauncher/BackendServer.cpp`; `backends.json` | stable launcher 是 C++，Rust 是本项目 backend；stdio 路由与相对根目录配置 |
| PIME | `PIMETextService/PIMEClient.cpp`，特别是 147–229、310–326、635–760 行 | 同步提交、cursor 转换、阻塞 RPC；无 commit ack |
| PIME | `PIMETextService/PIMETextService.cpp`; `PIMEImeModule.cpp`; `installer/installer.nsi` | 空 onFocus、生命周期、固定 CLSID 和共享根目录 |
| libIME2 | stable gitlink `8ad3c9b433d930ce5614c483461dfa78cedb5efd` | 本次仅核对 gitlink；底层 TSF 行为与官方二进制仍需复核 |

证据限制：任务给定的 `/tmp/typex-pime-audit` 在本会话不存在，不能声称读取了其二进制。本次从官方固定 commit 的源码归档恢复并阅读上述完整源文件（`/tmp/typex-stable-spec-review`）；clone 失败、归档下载超时，未完整重建上游。release 二进制与这些源码的对应关系、hash、实际日志/超时行为必须在打包前回到原审计产物或官方发行包复核。先前 HEAD 的 Rust launcher、1048576 行上限和 2000 ms 等待描述不适用于此 stable 源码。

配套报告：[Windows 框架调查](../windows-framework-survey.md)、[UniTeX 调查](../unitex-research.md)。前者覆盖长期富编辑器要求；本规范收窄首版，不删改调查结论。

本次实际执行现有 `unitex.bundle.cjs` 的 `parse` 并与源码比较：`\alpha → α`、`\mathbf{Az09} → 𝐀𝐳𝟎𝟗`、`x^{12}+y_{34} → x¹²+y₃₄`、`\frac{a+b}{c} → (a+b)/c`、`\frac{1}{2} → 1/2`、`\frac{-a}{bc} → -a/(bc)`。这是参考 bundle 输出，不是本项目转换器测试，也不是重新构建该 bundle 的证据。

UniTeX 对单个反斜杠输出空串，对 `\frac{` 输出 `\frac`，对 `a_` 输出 `a`，对 `x^{` 输出 `x`。**这些行为不得继承到 IME**。它的 mathfrak 大写 H/I 表亦有重复 F/G 问题，本版不纳入 mathfrak。UniTeX 的 UTF-16 length/部分 fallback/NFC 不自动成为本版标准：本版按 Unicode scalar 定位，保留原文且不做整体 normalization。

## TyPeX.WinDemo.Types — 强类型数据边界

- kind: entry; role: definition
- depends-on: TyPeX.WinDemo.Scope

下列为语言无关 ADT 契约，不是待编译代码。Rust 可使用 newtype、enum、关联类型实现；核心不得含 `Any`、`dyn Any`、`serde_json::Value`、任意字段字典或 OS 指针。

```text
Source = 有效 Unicode scalar 序列（无孤立 surrogate）
ScalarOffset = 在相应 Source 中的 scalar 边界索引
Span = { start: ScalarOffset, end: ScalarOffset }   // 半开区间
ClientEpoch, SessionId, Revision, EffectId = 各自独立的本地不透明标识
CommitKind = Converted | Raw
LifecycleReason = ProfileClosed | KeyboardClosed | CompositionEnded(forced: Bool) | Disconnected
Disposition = Pass | Consume | NotKey
IssueKind = Incomplete | UnknownCommand | Unsupported | Malformed | LimitExceeded
Issue = { kind: IssueKind, source: Span }
Analysis = { preview: Source, issues: List<Issue> }
Draft = { session: SessionId, revision: Revision, raw: Source, cursor: ScalarOffset }
State = Idle | Editing(Draft)
Event = Begin(SessionId)
      | Insert(Source) | Backspace | Delete | Move(Left | Right | Home | End)
      | Commit(CommitKind, EffectId) | Cancel | Lifecycle(LifecycleReason)
      | UnsupportedKey
Effect = SetPreedit { raw: Source, cursor: ScalarOffset }
       | ClearPreedit
       | SubmitInCurrentReply { id: EffectId, kind: CommitKind, text: Source }
       | Notice(Empty | InvalidInput | Limit)
Outcome = { state: State, disposition: Disposition, effects: List<Effect> }
View = { raw: Source, cursor: ScalarOffset, analysis: Analysis, phase: StateTag }
```

`View` 只读，derived analysis 不得反写 raw。20l 提供 `Extension<State, Event, View, Effect>` 的纯 dispatch 接口；上述 TyPeX 特有 grammar/state 留在 Extension，平台 adapter 解释 Effect 并分配本地 ID。通用核心不能含 `alpha`/`frac` 分支。配置此版固定为本文件的表和限制，不引入任意扩展配置对象。

`ClientEpoch` 只用于 adapter 的连接隔离，绝非 TSF context 身份；client_id、SessionId 和 EffectId 均不证明输入框/range、原目标或 native 去重。路线 A 没有 ContextToken、CommitResult/Applied、Pending-ack 或 Resume 契约。`SubmitInCurrentReply` 仅表示一次同步回复意图，不是已交付事实。offset 在核心永不指向 UTF-8 字节中部或 UTF-16 surrogate 中部；native cursor 能力另受 Windows gate 约束。删除/移动按 scalar，不保证一个 grapheme 一步删除。

## TyPeX.WinDemo.Conversion — 确定性有界转换

- kind: entry; role: definition
- depends-on: TyPeX.WinDemo.Types, TyPeX.WinDemo.Evidence

`analyze : Source → Analysis` 为总纯函数。完整支持的构造才替换；未知/未完成/错误/越界构造原样保留并产生 issue。Converted 仅允许 `issues` 为空的非空组字。Raw 永远提交原串，不以“修复”名义删括号、补参数或规范化。

### 词法与语法

1. 反斜杠后最长 ASCII 字母串为一个 command，大小写敏感。完整命令在 EOF 或首个非字母处结束；`\alphabeta` 不拆成 `\alpha` 加文本。单独反斜杠与已知命令的真前缀（如 `\al`）标 Incomplete；其他命令标 UnknownCommand。此版不识别反斜杠加非字母的 escape。
2. 普通非控制 scalar 原样输出，包括空格、中文、emoji。特殊字符 `\ { } ^ _ $` 必须交由语法处理；`$`/`$$` 是 Unsupported，不剥掉。空白保留，不应用 TeX 吞空格规则。函数 command 与 `{` 之间不得有空格；例如 `\frac {a}{b}` 是 Malformed 且整个构造保持原样。
3. `sequence := item*`；`item := literal | symbol | group | font | fraction | script`。`group := '{' sequence '}'`，完整普通 group 去掉外层括号。font 必须紧接一个 group；fraction 必须紧接两个非空 group。递归 group/fraction 支持，但字体 group 限制见下。
4. `script := ('^' | '_') argument`，同一 sequence 中必须紧接非空 base item，或紧接该 base 已有的另一类 script；base 本身不是空白或 script。base 可跟一上标和一下标，顺序不限，同类重复是 Malformed。argument 为一位 ASCII 数字或紧接的非空 `{payload}`。`x^12` 是 `x¹2`，多位必须写 `x^{12}`。脚标不递归解释 payload，只接受下列表中字符。
5. font 名只有 `mathbb`、`mathbf`、`mathit`、`mathrm`。其非空 payload 仅由 ASCII A–Z/a–z/0–9/空格及 `+ - = ( )` 组成，无子命令或嵌套 group。非表字符使整个 font 构造 Unsupported，不部分丢弃包装。
6. 固定 symbols 仅为：`alpha → α`、`beta → β`、`gamma → γ`、`pi → π`、`rightarrow → →`、`infty → ∞`、`leq → ≤`、`neq → ≠`。其余即使 UniTeX 支持也不在本版转换表。
7. font 表：`mathbb` 的大写序列为 `𝔸𝔹ℂ𝔻𝔼𝔽𝔾ℍ𝕀𝕁𝕂𝕃𝕄ℕ𝕆ℙℚℝ𝕊𝕋𝕌𝕍𝕎𝕏𝕐ℤ`，小写 U+1D552…U+1D56B，数字 U+1D7D8…U+1D7E1；`mathbf` 大写 U+1D400…U+1D419、小写 U+1D41A…U+1D433、数字 U+1D7CE…U+1D7D7；`mathit` 大写 U+1D434…U+1D44D、小写按 U+1D44E…U+1D467 对应位置但 h 用 U+210E `ℎ`，数字不变；`mathrm` 全部不变。允许的空格与标点不变。
8. 上标输入 `0123456789+-=()` 对应 `⁰¹²³⁴⁵⁶⁷⁸⁹⁺⁻⁼⁽⁾`；下标相同输入对应 `₀₁₂₃₄₅₆₇₈₉₊₋₌₍₎`。其余不做猜测，整个 script 保留并标 Unsupported。例如 `x^{q}` 保留，虽然参考 UniTeX 可生成 `x𐞥`。
9. fraction 仅 `\frac{numerator}{denominator}`，递归渲染成功后 `paren(numerator) + '/' + paren(denominator)`。`paren(t)` 去除前导 ASCII `-` 后若至多一 scalar 则不加括号，否则加一层括号。沿用 UniTeX 单行规则，但长度按 scalar；不约分、不生成 `½`、不做 display layout。`dfrac/tfrac/cfrac` 暂不支持。
10. raw 上限 4096 scalar；Insert 超限整体拒绝而不截断，保留旧 raw/cursor。组嵌套上限 16 层；超过时 analyze 返回 raw 全串预览和 LimitExceeded，不继续递归。换行、制表符、其他 C0 控制字符/DEL 不得 Insert：整次插入拒绝。core 类型解码即拒绝无效 Unicode，不替换为 U+FFFD 后冒充无损。

### 无损恢复

解析须保留 source span。遇到未知 command，连同紧随的完整/未闭合 braced arguments 作为一个 opaque 构造；不得转换其内部。已知构造的任一必需参数出错时，该构造整体 opaque。未闭合 group 从其起始 `{` 到 EOF 原样保留；额外 `}` 原样保留并标 Malformed。`^/_` 缺 base/argument 时连同可识别 argument 原样保留。最外层失败 span 优先，issues 按起始位置排序且不重复报告其子错误。相邻独立成功构造仍可预览，但整串 Converted 被拒绝。

精确 golden examples（反斜杠均为实际一个字符；“原样”表示与输入逐 scalar 一致）：

| case ID | raw | preview / converted text | issues |
|---|---|---|---|
| C01 | `\alpha+\beta` | `α+β` | 无 |
| C02 | `\gamma \pi \rightarrow \infty \leq \neq` | `γ π → ∞ ≤ ≠` | 无 |
| C03 | `\mathbb{R}` | `ℝ` | 无 |
| C04 | `\mathbf{Az09}` | `𝐀𝐳𝟎𝟗` | 无 |
| C05 | `\mathit{h}` | `ℎ` | 无 |
| C06 | `\mathrm{AB 12}` | `AB 12` | 无 |
| C07 | `x^2 + y_1` | `x² + y₁` | 无 |
| C08 | `x^{12}+y_{34}` | `x¹²+y₃₄` | 无 |
| C09 | `x_{1}^{-2}` | `x₁⁻²` | 无 |
| C10 | `\frac{a+b}{c}` | `(a+b)/c` | 无 |
| C11 | `\frac{1}{2}` | `1/2` | 无 |
| C12 | `\frac{-a}{bc}` | `-a/(bc)` | 无 |
| C13 | `\frac{\alpha}{\mathbf{A}}` | `α/𝐀` | 无；scalar 长度规则 |
| C14 | `\frac{1}{\frac{2}{3}}` | `1/(2/3)` | 无 |
| C15 | `中文🙂` | 原样 | 无 |
| C16 | `\` | 原样 | Incomplete |
| C17 | `\al` | 原样 | Incomplete |
| C18 | `\alphabeta` | 原样 | UnknownCommand |
| C19 | `\frac{` | 原样 | Incomplete |
| C20 | `a_` | 原样 | Incomplete |
| C21 | `x^{` | 原样 | Incomplete |
| C22 | `abc{` | 原样 | Incomplete |
| C23 | `\alpha+\foo{\beta}` | `α+\foo{\beta}` | UnknownCommand |
| C24 | `\frac{a}{\foo}` | 原样 | UnknownCommand（整个 frac span） |
| C25 | `x^{q}` | 原样 | Unsupported |
| C26 | `\mathbb{\alpha}` | 原样 | Unsupported |
| C27 | `\frac{}{2}` | 原样 | Malformed |
| C28 | `a}` | 原样 | Malformed |
| C29 | `x^2^3` | `x²^3` | Malformed（第二个 script） |
| C30 | `x^12` | `x¹2` | 无 |
| C31 | `$x$` | 原样 | Unsupported |
| C32 | `\frac {a}{b}` | 原样 | Malformed |

## TyPeX.WinDemo.Transition — 纯状态转移

- kind: entry; role: contract
- depends-on: TyPeX.WinDemo.Types, TyPeX.WinDemo.Conversion

`step : Config × State × Event → Outcome` 与 `view : State → View` 必须确定且无 I/O，无时钟、随机数、全局可变状态、IPC、文件或 UI 调用。相同输入产生结构相同结果；ID 由 Event 提供。`probe : Config × State × Key → Disposition` 仅预测是否吃键，不推进 state、不分配 ID、不产生 Effect。

| 当前状态/事件 | 新状态与效果 |
|---|---|
| Idle + Begin(s) | Editing，空 raw、cursor=0、初始 revision；Consume；不必建立空 native composition |
| Idle + 其他编辑/提交/取消/UnsupportedKey | Idle；Pass；无 effect |
| Editing + Insert(t) | 在 cursor 插入，cursor 前移相应 scalar 数，revision 递增；Consume；SetPreedit。非法/超限则不变并 Notice |
| Editing + Backspace/Delete | 删除 cursor 前/后一个 scalar；边界 no-op；Consume；实际改变才递增 revision 并 SetPreedit |
| Editing + Move | 在合法边界移动/夹紧；Consume；改变 cursor 则递增 revision 并 SetPreedit |
| Editing + Commit(Converted,id) | raw 非空且 issues 空：Idle，Consume，一次 SubmitInCurrentReply；否则留在 Editing，Consume，Notice(Empty/InvalidInput) |
| Editing + Commit(Raw,id) | raw 非空：Idle，一次 SubmitInCurrentReply 原 raw；空则保持、Notice(Empty)；Consume |
| Editing + Cancel | Idle；Consume；ClearPreedit；无提交 |
| Editing + UnsupportedKey | 状态不变；Pass；无 effect；不假设随后必有 focus 通知 |
| 任意状态 + Lifecycle(reason) | Idle；NotKey；仅丢弃本地 draft/按键跟踪，不向未知目标清 preedit 或提交；无文本 effect |
| Editing + Begin | 保持；Consume；不覆盖已有 raw |

未列事件总定义：保持原状态、无 effect；键事件 Pass，非键 NotKey。revision 不溢出复用：到实现上限拒绝继续编辑并报告 Limit。adapter 每个 ClientEpoch 单独串行化，拒绝旧连接事件；这只证明客户端隔离，不证明字段隔离。

提交由当前 onKeyDown 响应携带 commitString；核心回 Idle 表示意图已生成，不表示 TSF 插入成功。adapter 编码/写出失败则丢弃该连接的本地会话，不自动重试文本；stdout 写成功也不能升级为 Applied。没有 ack 就不能可靠区分“未插入”和“已插入但连接断开”，故不声称 exactly-once、失败回滚或可靠草稿恢复。强类型只约束本地语义，不能补出宿主未提供的保证。生命周期丢弃后必须重新显式 Begin。

## TyPeX.WinDemo.Interaction — 键盘、组字与可见反馈

- kind: entry; role: requirement
- depends-on: TyPeX.WinDemo.Transition

选择 Windows 的 “20l TyPeX Demo” profile 后，默认 Idle，普通输入旁路。`Ctrl+Shift+M` 的 key-down 启动数学组字，重复启动不清 raw。此组合为本版约定，不抢占 Windows profile 切换键；在不受支持/disabled/empty context 不注册为强制全局 hook。

Editing 中：无 Ctrl/Alt/Win 的可打印文本（Shift 用于大小写/符号）→ Insert；Space 是普通空格；Backspace/Delete/Left/Right/Home/End 对应编辑事件；Enter → Converted；Ctrl+Enter → Raw；Esc → Cancel。Enter 提交不额外插换行；再次 Enter 在 Idle 必须旁路，不重复提交。Tab 与其他快捷键不作为提交键，不吃掉应用快捷键；其改变焦点时不能假设 native 会先通知 backend。键盘布局/死键不得按 VK 猜字符；无法解码为有效文本的键必须旁路。auto-repeat 每个实际 onKeyDown 仅产生一次编辑；modifier-only 不插入。

TSF preedit 显示 **raw**，不是长度不同的 converted preview。预览使用 PIME 普通文字候选/提示 UI，只有一个非可选 preview，不支持候选选择或数字选词；有 issues 时可见标记“未完成/不支持，Enter 不提交；Ctrl+Enter 原文”。不另开可聚焦编辑窗口。空 preedit 可以暂不建立 native composition。支持场景必须能看到 source、预览与失败提示；UI-less 应用安全旁路。

Esc 在当前有效 key edit session 以空 compositionString、隐藏候选请求清除本次 preedit，不删除应用既有文本。收到 profile 关闭、keyboard closed、composition terminated 或可观察断连时丢弃本地会话；不提供挂起恢复。stable 的 onFocus 为空，字段切换/目标销毁并不总有完整 backend 通知；不得承诺在任意控件自动清理原目标。强制结束可能把 raw 留在应用中，这不是 Converted 提交，也不得为补救而向新目标重发或回删。说明必须披露草稿可能丢失/原文可能保留，受控演示建议切目标前 Esc 或提交；这不能替代 A11 的串字测试。

## TyPeX.WinDemo.PimeProtocol — 官方 stable 宿主与 typed backend

- kind: entry; role: interface
- depends-on: TyPeX.WinDemo.Transition, TyPeX.WinDemo.Interaction

### 已有 wire surface（不可假冒新协议已存在）

UTF-8 newline 分帧：launcher → backend 是 `<client_id>|<JSON>\n`，backend → launcher 是 `PIME_MSG|<client_id>|<JSON>\n`。只分割路由分隔符，JSON 内部 `|` 不截断，换行按 JSON escape。stdout 只能协议；stderr 只写去敏错误码，不记录 raw/preview/commit。插件 decoder 自定 1048576 bytes 有界行缓冲，超限拒绝；这不是 stable launcher 已有上限。stable BackendServer 用 stringstream 缓冲并有 SEND/RECV debug 日志，必须核查官方发行包日志配置，不能将插件无日志等同宿主不记录输入。

初始化字段：`method:"init"`、`seqNum`、`id`（profile GUID）、`isWindows8Above/isMetroApp/isUiLess/isConsole` 布尔。响应逐次回显 `seqNum` 和 `success`。键请求 method 使用 **`filterKeyDown`、`onKeyDown`、`filterKeyUp`、`onKeyUp`**；带 `charCode/keyCode/repeatCount/scanCode/isExtended/keyStates[256]`，禁止误写成 `onFilterKeyDown`。键响应 `return` 为是否吃键的布尔。

还须解码 `onActivate`、`onDeactivate`、`onKeyboardStatusChanged(opened)`、`onCompositionTerminated(forced)`；断开路由要释放该 client 资源。其余有效但本版不处理的 method 返回无副作用响应；畸形/字段类型不符则 `success:false`（可解码 seqNum 时回显），不调用核心。不得将 arbitrary JSON map 透传核心。

`filterKeyDown`/`filterKeyUp` 只 probe；重复 filter 不修改 draft。仅实际 onKeyDown step 一次。adapter 记录已吃下的 physical key，在相应 up 消费且不重复插字；未匹配 up 旁路。每个 client 请求串行处理，重复/过期 seqNum 拒绝且不返回带文本副作用的缓存回复；不得重放 commitString。seqNum 仅 RPC 关联，不是 native 去重键。client_id 不能区分同一客户端内的输入框。断连检测只限真实可观察信号，不能臆造逐字段断连通知；重新 init 必须重置本地状态。

常用响应字段 `compositionString`、`compositionCursor`、`commitString`、`candidateList`、`showCandidates` 需要强类型 wire structs，未使用字段省略。普通更新传 raw；提交响应不附非空 compositionString，避免提交后重启旧组字；取消先隐藏候选再将 compositionString 置空。不在无 EditSession 的生命周期 callback 响应里直接发送 commitString。

### 同步提交的实际含义与不可承诺项

1. **当前 edit session 消费响应**：onKeyDown 同步等待 RPC；handleRpcResponse 见 backend `success:true` 后调用 updateStatus。在 session 非空时依次更新候选、commitString、compositionString/cursor。非空 commitString 若无 composition 则 startComposition，然后 setCompositionString、endComposition。这是 DLL 在当前 TSF edit session 的处理路径，不是异步回到保存目标，也不是 backend 能观察的原子事务。
2. **没有 ack/context token**：上述调用不向 backend 返回 TSF HRESULT 或 Applied；`success` 来自 backend 自己。`onCompositionTerminated(forced:false)` 也不是与某次提交绑定的成功确认。stable 不提供 effectId 去重、context generation、原 range 身份或 demoProtocolVersion 握手；不得新增这些字段后自称 native 已识别。插件只发既有 wire 字段。
3. **光标已知风险**：stable 210–223 行试图按 surrogate 补偿，但循环以 scalar 数作次数、以 UTF-16 索引读取，连续非 BMP 有错位风险。先按上游预期 scalar offset 建立测试，不能盲目先转 UTF-16 再重复补偿。A10 未过不得宣称完整 scalar native 编辑；若需 adapter 映射或缩小交互范围，必须先修规范/测试，不能私改共享 DLL。核心转换与 C01–C32 不因此缩水。
4. **生命周期有限**：onActivate/onDeactivate、keyboard status、composition terminated 仅是可用通知，不覆盖全部 focus/context 变化。无 edit session 的 callback 回复不发 composition/commit/showMessage 等依赖 session 的 UI 更新（stable updateMessageWindow 在空 session 下也可能解引用）；只做本地清理。关闭、崩溃或强制结束不保证撤回 raw、清 UI 或恢复草稿。
5. **安全边界必须实测**：init 可识别的 Metro/UI-less/console 模式旁路；keyboard-disabled/empty/password/read-only 不能由 backend 臆测。敏感内容不应进入插件/宿主日志；以无真实密码的 fixture 验证。缺保护、串字、残留或挂死是 blocker，不因选择 A 而变成已满足，也不自动授权 native fork。

响应路径不启动渲染器/下载/编辑器或网络请求。stable Client 使用同步 TransactNamedPipe/ReadFile，连接 WaitNamedPipe 参数 30000 ms、最多 5 次尝试；BackendServer 有 30 秒常量，不能仅凭声明证明完整故障时限。不存在本版可声称的 2000 ms RPC 硬上限。正常 4096 scalar 范围 key RPC p95 目标仍为 ≤50 ms；故障恢复时延和是否阻塞宿主必须实测。若官方宿主不能满足 gate，记录 blocker 并请求路线/范围评审，而不是隐式转成自研 native bridge。

## TyPeX.WinDemo.Installation — 安装与卸载生命周期

- kind: entry; role: lifecycle
- depends-on: TyPeX.WinDemo.PimeProtocol

交付为版本化 Windows **插件安装包**（具体打包器不冻结），包内只有 TyPeX backend、profile manifest/icon、必要的 profile 安装辅助程序、许可证、校验清单与卸载入口；不捆绑/替换官方 DLL/launcher。官方 PIME 1.3.0 stable 由用户另行安装，提供明确前置依赖说明及离线准备步骤；未安装则停止并引导，不自动升级/卸载 PIME。TyPeX backend 本身无需 Node/Bun/Python/开发工具，这不代表官方 PIME 包不含这些运行时。

1. **预检与共享边界**：校验 Windows/架构、权限、官方版本/文件与 x86/x64 注册。stable 固定根为 `%ProgramFiles(x86)%\PIME`，共享 CLSID 为 `{35F67E9D-A54D-4177-9697-8B0AB71A9E04}`；不可将目录改名当成隔离。插件仅独占 backend 名称/子目录和 profile GUID。标识在 manifest 一次固定；发现同名未知所有者或其他 PIME 版本即停止，不覆盖。
2. **安装所有权**：在共享根下写本插件子目录，如 `<backend-name>\input_methods\typex\ime.json`，backend 可执行文件及 icon 归插件所有。对共享 `backends.json` 只原子合并本项 name/command/workingDir/params，保留其他项、检查并发修改、记录本项所有权；stable command/workingDir 按 PIME 根解析，不假定任意绝对路径可用。按 Windows profile API 在既有 PIME CLSID 下添加/启用本 TyPeX GUID，验证目标用户（不是误用提权账户）和 x86/x64 应用。不得重注册/注销整个 COM service；具体 profile API 流程须测试验证。
3. **完成/回滚**：不强制切换或禁用其他输入法。日志仅版本/错误码/路径；提示启用与必要的注销/重启步骤。配置重载若需重启共享 launcher，应明确告知影响并获同意，优先引导注销，不强杀其他 backend。失败只撤销本次 profile、本项配置和自有文件；不以整份旧 backends.json 覆盖别人后续修改。回滚冲突则停止并报告。
4. **重复/升级**：同版本修复或 no-op，不产生重复 profile；不同插件版本首版拒绝覆盖并提示先卸载。不负责 PIME 升级/修复；PIME 被外部升级/卸载后应报依赖失效，不重新注册共享 DLL 补救。
5. **profile-only 卸载**：提示用户先提交/取消并切离 TyPeX，不能声称可远程探知所有草稿。只移除 TyPeX language profile（对应 CLSID+LANGID+profile GUID）、自身 backend 配置项和自有文件/卸载记录。**绝不对共享 DLL 执行 `regsvr32 /u`、DllUnregisterServer、整个 service 的 Unregister 或删除共享 CLSID；绝不删除 PIME 根、DLL、launcher、共享启动项、其他 profile。** 只停止可确认归本插件所有的 backend；不得杀共享 launcher。锁定自有文件提示注销/重启再清理；不得安排删除共享 DLL。配置项被他人改写则保留并报冲突，不盲删。重复卸载安全；其他 PIME profile 必须仍能输入。
6. **许可证与安全**：分发复用 UniTeX 表/代码须附 MIT notice；官方 PIME 为独立前置依赖，仍须说明其许可证；任何实际复制进插件的 PIME/libIME2 代码逐项审计 LGPL 等义务。签名/SmartScreen 状态真实披露，不要求关闭系统防护。插件无遥测/输入日志/运行网络依赖；宿主日志独立通过 A13 审查。

安装命令、产品标识和产物 hash 必须在实现后的 release manifest/验收报告中给出。此规范阶段不伪造 exe 路径、GUID、注册成功输出或下载地址。

## TyPeX.WinDemo.Acceptance — 验收矩阵与顺序

- kind: entry; role: acceptance
- depends-on: TyPeX.WinDemo.Conversion, TyPeX.WinDemo.Transition, TyPeX.WinDemo.Interaction, TyPeX.WinDemo.PimeProtocol, TyPeX.WinDemo.Installation

**必须按“规范评审 → 失败测试 → 实现 → 分层验收”进行。** 下表是待实现测试的要求，不是已经通过的测试报告。每项结果记录 spec version、case ID、构建 hash、平台/应用版本、实际输出及证据位置。

| gate / test ID | 操作与要求 | 层级 |
|---|---|---|
| A01 | C01–C32 逐 scalar 比较 preview、issues 类型/span；每个有 issues 的 Converted 都被拒绝而 raw 不变；Raw 输出完全相同源串 | 核心 |
| A02 | 字体所有允许字母/数字、上下标完整表逐项测试；组合 Unicode 不做 NFC；`𝐀𝐁x` 的三个 scalar offset/删除不拆 surrogate | 核心 |
| A03 | 未闭合/未知/非法、空参、嵌套边界、4096 与超限插入、控制字符、无效 wire Unicode；无 panic/无截断 | 核心+decoder |
| A04 | 同一 state/event 重跑结构相等；通用 dummy 非数学 Extension 能运行；两个 ClientEpoch 的 raw 互不影响，不伪称字段身份隔离 | 核心 |
| A05 | Begin→输入 `\alpha`→Enter 只生成一次含 `α` 的同步回复意图并回 Idle；再次 Enter 为 Pass；不生成/等待虚构 Applied | 转移 |
| A06 | `\frac{`→Enter 拒绝并保留→Ctrl+Enter 精确提交原文；`\alpha`→Esc 无 commit 且清本次 preedit | 转移+Windows |
| A07 | 编码/写出失败、结果不明均不自动重发；重复 seqNum 不返回文本副作用；生命周期/重新 init 丢弃旧草稿；stdout 成功不当作 TSF ack | 故障模型 |
| A08 | 同一键 filter 调两次、down 一次、up 一次，只插入一次；键重复、快捷键、Space、Tab、键盘布局与死键符合 Interaction | IPC+Windows |
| A09 | 官方 stable launcher 分帧 init/seqNum/key/reply；拆包/粘包、JSON 内 `|`、插件超长行限制、坏字段、多个 client、重连受测；无新增握手/ack 字段依赖 | 协议+Windows |
| A10 | 连续非 BMP raw `🙂🙂a` 各光标位置、删除与 native UTF-16 位置一致；提交 `𝐀𝐳𝟎𝟗` code points 精确 | Windows |
| A11 | 非空 composition 下以鼠标/Tab/快捷键切字段、应用、profile、关闭目标：记录实际通知和 raw 留存/丢失，不误入新目标、不隐式转换；收到生命周期后须重新 Begin；不能假装每次都有 focus callback | Windows |
| A12 | backend kill/超时/畸形 reply/launcher 重启：应用仍可普通输入、无重复 commit、无残留候选/永久卡键；记录实际恢复时延 | Windows 故障注入 |
| A13 | 实际密码字段、disabled/empty/read-only context 不进入 backend、不显示数学 UI；插件及官方 launcher debug/发行配置和落盘日志均核查无输入泄漏；仅用假密码 fixture | Windows 安全 |
| A14 | 干净 VM：缺 PIME 时插件拒装→单独安装官方 1.3.0→离线装插件→语言 UI 选择/输入→注销登录→重复安装→仅卸载插件→重启→再装；官方 PIME 不被插件拥有或删除 | 安装 |
| A15 | 已有其他 PIME profile 的 VM 重做 A14；错误版本/权限/写盘/profile 注册失败/共享配置冲突回滚；仅增删 TyPeX profile，x86/x64 共享 COM 注册及其他 profile 保持可用，绝不 shared DLL /u | 安装共享安全 |
| A16 | 4096 scalar 与普通短组字各至少 1000 次 key RPC，报告 p50/p95/max、测试机；p95 目标满足且 key 路径无外部编辑长任务 | 性能 |
| A17 | 包内清单、licenses/source-offer（适用时）、依赖与卸载指南、未签名提示核对 | 分发 |

Windows 成功输入矩阵最低要求：x64 Win32 测试编辑控件或 Notepad、x86 Win32 edit-control harness、Edge 普通 textarea/contenteditable、VS Code（Electron）普通文本编辑器。逐应用执行符号/字体/脚标/分数、Enter/Raw/Esc、切焦点、非 BMP。记录应用准确版本；不以网页自身输入逻辑替代 TSF。Office/Word 仅探索性记录，未测不宣传。DPI 100%/150% 下 preview 可见、不抢焦点；多显示器完整排版优化延期。

完成分级不得混淆：
- **Spec-ready**：条目、例子、依赖与测试映射评审通过；当前交接最多到文档冻结，不自动达成评审。
- **Core-ready**：纯核心/decoder/模拟转移测试实际通过，不代表 Windows 可安装。
- **Installable-demo-ready**：A01–A17 和最低 Windows 应用矩阵实际执行，所有必须项通过，提供真实安装包、校验值与证据。

本 Linux 写作会话未构建/安装/运行 Windows IME；所有 Windows gate 仍为 **NOT RUN**。不允许用 `cargo test`、协议 mock 或 UniTeX bundle 的输出把此状态改成 PASS。

## TyPeX.WinDemo.Handoff — 冻结与后续授权边界

- kind: entry; role: handoff
- depends-on: TyPeX.WinDemo.Acceptance

本交接仅修订此文件及 `docs/project-direction.md`，不修改 `.SNL_Doc`、spike 或上游源码，不实现代码，不创建测试源文件，不 commit/push。

路线 A 已获选择；本次冻结的是其 spec-only 交接，不是所有细节已获批准。后续评审特别关注 Ctrl+Shift+M、scalar 编辑、未知语法拒绝 Converted、无候选选择、富编辑器延期，以及无 ack/context identity、可能遗留 raw、共享 PIME 所有权和故障卡顿风险。不得悄悄重新加入强制 native bridge/独立 CLSID 产品线。

后续授权后：按上述 IDs 写失败的核心/协议测试 → 强类型纯核心与 stable stdio adapter → plugin-only installer/profile API → Windows VM/真机验收。当前 blockers：原 `/tmp/typex-pime-audit`/官方二进制需复核；非 BMP cursor、字段切换、敏感控件/宿主日志、阻塞恢复和 profile-only 注册/卸载尚无 Windows 证据。所有 Windows gates **NOT RUN**。任何 gate 失败须先评审范围/路线，不以强类型接口、mock 或受控演示冒充 native 保证，不自动获准修改共享宿主。
