# TyPeX PIME stable stdio backend — 冻结报告

规范：`typex.windows.demo/2-route-a`。本切片仅创建 `crates/typex-backend/**` 与本报告，没有改核心、installer、packaging、根配置或上游；没有 commit/push。

## 交付与真实执行

独立 Cargo package `typex-backend`，路径依赖 `../typex-core`，serde typed request structs / Method enum / Reply；生产代码无 `serde_json::Value`、无 unsafe、无文本日志、无网络调用。backend 名称供 installer 使用为 `typex-demo`；exe 为 `typex-backend.exe`。固定 GUID `{D93F3E23-0F2D-4D7A-9F28-51DC82B37A60}`（接受大小写差异；与 stable StringFromCLSID 花括号输出一致）。

Linux 实际 debug/release 验证均成功：一个 Rust integration harness 调用 **6 个 Python unittest 场景**，每个启动真实可执行文件并交换 stdin/stdout；不是 mock。生产 exe 不依赖 Python，Python 仅为 Linux build-host 测试依赖。

```sh
cd /home/sky/workspace/cat/llllllllllllllllllll/crates/typex-backend
cargo test --offline
cargo test --offline --release
source /tmp/typex-build-tools/env.sh
cargo build --offline --release --target x86_64-pc-windows-gnu
x86_64-w64-mingw32-objdump -p target/x86_64-pc-windows-gnu/release/typex-backend.exe
sha256sum target/x86_64-pc-windows-gnu/release/typex-backend.exe
```

所有上述构建/测试已实际执行，退出码 0。证据：`evidence/final-debug.txt`、`final-release.txt`、`windows-build.txt`、`windows-imports.txt`、`frozen-sha256.txt`。

**实际 Windows release 二进制完整路径：**

`/home/sky/workspace/cat/llllllllllllllllllll/crates/typex-backend/target/x86_64-pc-windows-gnu/release/typex-backend.exe`

file 判定：`PE32+ executable (console) x86-64 (stripped to external PDB), for MS Windows`。

SHA-256：`c64a3d661725df3af6045c0759f87316c6daf6f99a986e585cc242cb2740f852`。

实际导入 DLL：`api-ms-win-core-synch-l1-2-0.dll`、`bcryptprimitives.dll`、`KERNEL32.dll`、`msvcrt.dll`、`ntdll.dll`、`USERENV.dll`、`WS2_32.dll`。没有 MinGW runtime/Node/Python 动态依赖；WS2_32 是 Rust std 链接产物，不代表插件发起网络请求。**只交叉构建和检查导入，未在 Windows 执行。**

## 已实现行为

- UTF-8 newline 路由，仅分割第一个 `|`，响应 `PIME_MSG|client|JSON`，逐回复 flush。最大保存 1,048,576 字节（不含 newline）；超长继续有界 drain 至 newline，再返回失败。无完整 newline 的 EOF 尾帧不执行。无有效 route 的帧丢弃；route 限 256 bytes 且不得含 ASCII 控制字符。
- 独立 client draft、seq high-water、physical key 集合及本地 ID。init 为新本地 epoch，允许 seq 重置并清掉旧 draft；不表示 TSF context 身份。其他请求 seq 必须严格递增，拒绝旧/重复序号且不缓存文本回复。bad request 有可解码 seq 时回显；没有调用核心编辑。close 是允许无 seq 的 stable 断连通知，释放 client。
- filterKeyDown/filterKeyUp 仅预测，不 step、不分配本地 ID、不改变 draft/held；传输 seq 高水位仍前进。实际 onKeyDown 才推进核心一次，repeatCount 不乘法插字；physical tuple `(keyCode,scanCode,isExtended)` 跟踪已消费 down，对应 up 只消费一次。
- Ctrl+Shift+M Begin；Enter Converted；Ctrl+Enter 原串；Esc 清 preedit 隐藏候选；raw scalar 编辑、Space、Shift 文本、移动、删除；未知 shortcut/Tab/dead charCode=0/modifier-only 旁路。只使用宿主 charCode，不从 VK 推测布局。
- preedit 永远 raw，compositionCursor 为上游预期 scalar 数；单项 candidateList 作为普通预览，issue/拒绝提交提示含 Ctrl+Enter。没有候选选择处理或数字选词，数字作为普通 source；native UI 可见性/点击行为仍需 Windows gate。
- init 可观察的 Metro/UI-less/console（以及 Windows8Above=false）禁用编辑。onActivate 使用 stable `isKeyboardOpen`；onDeactivate、keyboard closed、composition termination（forced true/false）丢弃本地 draft/held，不向 callback 回复附任何 UI/text 字段。
- commit 仅在实际 down reply 携带；不附非空 compositionString。stdout 写出/flush 失败立即退出并释放全部本地 clients，无日志、无重试、无伪 ack/context token。EOF 释放全部内存状态。

## RED → GREEN 证据

按端到端纵向切片逐次新增场景并实现，不是最后才补测试。以下日志均真实留存于 backend/evidence：

| 切片 | RED | GREEN |
|---|---|---|
| 01 init/flush | `01-red.txt`：真实 exe 尚不存在，assertion failure | `01-green.txt` |
| 02 转换/Raw/scalar 编辑/取消 | `02-red.txt`：缺 `return` 键语义 | `02-green.txt` |
| 03 probe/up/seq/client | `03-red.txt`：filter 无返回语义 | `03-green.txt` |
| 04 lifecycle/模式 | `04-red.txt`：callback 后仍保留编辑态 | `04-green.txt` |
| 05 decoder/framing | `05-red.txt`：坏 method 丢失可解码 seqNum | `05-green.txt` |

RED 中的 KeyError 指向未实现的 wire 回复字段，不是导入/语法错误。Method string→enum 重构后五场景仍绿（`06-refactor-green.txt`）。第六场景是已实现 write failure/EOF/repeat 的额外回归检查，首次即绿，不冒称有新增 RED。

覆盖：转换 α；未完成 `\\frac{` 拒绝 Converted 后 Raw 原样；组合 Unicode 不 NFC；非 BMP scalar 光标；JSON 内 `|`；重复 Begin/filter/seq/down/up；两 client 不串字；re-init；close 无 seq；坏类型/缺字段/孤立 surrogate/超出 scalar/坏 keyStates/非法数字；超长/非 UTF-8/拆包/粘包；callback 无 UI；unsupported modes；真实断 stdout 导致非零退出且 stderr 空；EOF 半帧不响应。完整 `C01–C32` 依赖被冻结核心测试，未在此报告冒称逐例 Windows 通过。

## stable 源码核对与限制

读取 `/tmp/typex-stable-spec-review/PIMETextService/PIMEClient.cpp`：74 起 key payload、310–326 callback EditSession 限制、344–383 candidate 更新、386–405 activation、408–453 filter/down/up、614 起 init、635–760 同步 pipe。stable profile GUID 由 StringFromCLSID 生成带花括号小写，后端大小写不敏感校验。没有使用 HEAD Rust launcher。

环境缺 rustfmt，write 工具提示未安装；未声称通过格式检查。编译、真实 IPC debug/release 与 Windows cross-build 成功。测试文件动态 Popen 的 Optional pipe 属性会触发编辑器 Pyright 静态提示，不影响已运行的 Python 测试。

**全部 Windows gate NOT RUN**：没有 PIME launcher/TSF 实机运行、安装、字段切换、密码/disabled/empty/read-only、A10 native 非 BMP cursor、宿主 debug 日志、候选可见性、A12 故障恢复或 A16 Windows RPC 性能证据。stable 缺 context identity 与 ack，不能证明任意跨字段隔离、exactly-once 或 TSF 已插入；生命周期可能丢草稿/残留 raw。后端测试通过不升级为 Installable-demo-ready。
