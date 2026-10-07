# 路线 A 安装器交接（未通过 Windows 验收）

## 交付范围与结果

仅修改 `crates/typex-install/**`、`packaging/**` 与本文。未修改 core/backend；未提交或 push。固定 backend `typex-demo`、profile `{D93F3E23-0F2D-4D7A-9F28-51DC82B37A60}` 不变。官方 PIME 1.3.0-stable 独立安装，不随插件捆绑。

真实执行结果：
- Rust Linux 测试 **13 passed**（lib 1、config 4、policy 3、receipt 5）。新增删除中断重试测试先失败，再实现并通过；静态 NSIS 恢复测试同样经历 RED/GREEN。
- `python3 -m unittest discover -s packaging -v`：**3 passed**；这不是 NSIS 运行测试。
- `verify-dependency /tmp/typex-pime-extracted`：退出 0，匹配四个固定 release 文件 SHA-256。
- 最终 helper 已重新 Windows GNU release 构建成功；`file` 确认 PE32+ x86-64。
- helper：`crates/typex-install/target/x86_64-pc-windows-gnu/release/typex-install.exe`。
- 本次最终 SHA-256：`f0e8eb78e3b049ba48ccf52769b0fb65a57cfb2f7ec35c405bc8b207019b0896`（重建可能因 PE 时间戳改变）。
- objdump imports 仅见 Windows 系统 DLL，无独立 libwinpthread/libgcc DLL。
- 最终 NSIS compile **NOT RUN**：等待父任务真实 backend，不以 helper/假 backend 冒充成功。
- Windows 安装、卸载、重试、TSF 行为与 A10–A17 **全部 NOT RUN**。

## 卸载修复及安全边界

旧问题：NSIS 逐文件 Delete，部分成功后失败；下一次 receipt 要求所有文件仍存在，因此永久无法重试。

现在 receipt 持久记录 `Installing → Installed → Removing → Deleting`。只有 `Deleting` 容许已记录文件不存在；其余阶段仍严格拒绝缺失。存在的文件必须匹配记录哈希，路径必须在固定 PAYLOAD 白名单且不重复。删除前先验证所有剩余文件，再在每次删除前重新验证；不扫描未知文件来删除、不递归删除、不借“缺失容忍”接管未知文件。

`uninstall-prepare` 校验 identity/dependency/receipt/config，持久化 Removing，仅 RemoveLanguageProfile 本 profile，然后从最新已验证 snapshot 去除精确匹配 backend，最后持久化 Deleting。任何失败都保留可重试阶段。

`uninstall-files` 再检查 profile 已不存在、配置没有同名 backend，然后按 receipt 清理。所有文件删除后才删 receipt。NSIS 从自己内嵌的 helper 解压到 `$PLUGINSDIR` 执行，避免删正在执行的安装目录 helper。Uninstall.exe 放在删除列表最后；若它或 receipt 最终删除失败，可用相同版本独立 helper 恢复。未知文件/非空目录保留。

**不做**共享服务 Register/Unregister、共享 DLL `/u`、launcher/backend kill、切换默认输入法、禁用其他 profile、重启宿主或回写旧 backends.json。TSF Add/RemoveLanguageProfile 的实际系统范围/其他用户可见性仍须 Windows 多用户验收；纯代码调用范围不能代替该验证。

## 暂存、安装失败和恢复

NSIS 先完整解压到临时目录并检查 Errors。helper `stage-install` 校验完整 payload，在 PIME 根下独占创建 `typex-demo.stage`；在复制任何文件前记录期望哈希（Deleting recovery 状态），文件使用 create_new，完成后变为 Installing，再同卷 rename 为最终目录，拒绝覆盖已有目录。配置/profile 只在发布后修改。

`install` 每一步失败均保留 Installing journal，不声称事务回滚已经成功；用 Uninstall.exe 执行补偿清理。不会用旧配置快照覆盖用户或别的插件更新。Add 成功 Enable 失败、配置已写但 Add 失败、Installed journal 写失败，均有明确卸载恢复入口。

同用户本地桌面管理员 PowerShell（先退出 TyPeX、保存草稿；工具必须与 receipt 同版本）：

```powershell
# 如 NSIS 解压后、发布前失败：从可信构建产物位置运行，不在 stage 内运行 helper
& 'C:\path\to\typex-install.exe' recover-stage
# 如安装已发布或卸载部分完成：
& 'C:\path\to\typex-install.exe' uninstall-prepare
if ($LASTEXITCODE -eq 0) { & 'C:\path\to\typex-install.exe' uninstall-files }
```

普通写入失败会尝试删除当前尚未完整写入的 stage 文件。**硬断电/强制终止恰在 receipt 首次写入或文件复制途中**可能留下截断 journal/哈希不匹配文件；恢复工具必须拒绝自动删除，需管理员人工核对原包与所有权后处理，不能把不匹配当成“我们的文件”。遗留 `.typex-lock` / `.typex-new` 同样 fail closed，需人工确认无活跃操作后处理。不是全断电原子事务。

原版本缺 phase receipt 视为 Installing：不误报已安装 no-op，仍可安全卸载。same-version no-op 还要求 Installed、文件完好、profile 存在、配置有本精确 backend。

共享 backends.json 是 cooperating-writer lock + snapshot compare + atomic replace；不合作 writer 在最终比较与 replace 间仍有 TOCTOU。文件哈希检查与删除之间也有窗口；不宣称抵抗恶意管理员/路径替换或提供文件句柄级事务。安装期间应关闭 PIME 设置及其他安装器，Program Files ACL 是前提。

## LANGID 与 manifest 核查

已阅读官方固定 PIME commit `26fcf6ac8874e76b8f75f6826811b03bfdfc2e89` 的 `PIMETextService/DllEntry.cpp` 45–75：读取 name/guid/locale/fallbackLocale/icon。libIME2 固定 gitlink `8ad3c9b433d930ce5614c483461dfa78cedb5efd` 的 `src/ImeModule.cpp` 147–155：`LocaleNameToLCID(locale,0)`，仅失败时尝试 fallback，再 `LANGIDFROMLCID`。

来源：
- https://github.com/EasyIME/PIME/blob/26fcf6ac8874e76b8f75f6826811b03bfdfc2e89/PIMETextService/DllEntry.cpp
- https://github.com/EasyIME/libIME2/blob/8ad3c9b433d930ce5614c483461dfa78cedb5efd/src/ImeModule.cpp

保留 `zh-Hant-TW` / `zh-TW` 与 helper `0x0404`（Traditional Chinese/Taiwan）作为**临时 Windows profile 归属**，沿用 PIME 的 locale/fallback 机制；不是数学只能使用中文、不是用户语言要求或长期产品决策。实际 Windows LocaleNameToLCID 返回值、未装该语言时的可发现性、语言设置 UI 均待验证。不调用官方 DLL 全服务注册路径。删除无意义的 Python-only `moduleName`/`serviceName` 与未用 `win8_icon`；Rust stdio backend 不通过 Python 模块加载。

## 许可证与库存

`packaging/DEPENDENCIES.md` 从 helper locked Cargo metadata 生成，列出 36 个包（含 dev/build）。`THIRD-PARTY-LICENSES.txt` 收集 registry 原始 LICENSE 文本和 NSIS/MinGW/GCC toolchain copyright；安装器将其作为受 receipt 管理的文件安装。`NOTICE.txt` 含本安装器/icon MIT 与 UniTeX attribution。原始 metadata 为本地生成中间文件，含本机路径，可重新生成。

**发布 gate 未关闭**：真实 backend 的完整依赖/UniTeX 来源声明、Rust std/compiler runtime 全量分发审计仍须父任务确认。库存不声称覆盖尚未整合的 backend。PIME/libIME2 外部前置不复制，其 LGPL 等义务参照官方发行。

## 精确整合命令

```bash
cd /home/sky/workspace/cat/llllllllllllllllllll
source /tmp/typex-build-tools/env.sh
cargo test --manifest-path crates/typex-install/Cargo.toml --locked
python3 -m unittest discover -s packaging -v
cargo build --manifest-path crates/typex-install/Cargo.toml --locked \
  --release --target x86_64-pc-windows-gnu
cargo metadata --manifest-path crates/typex-install/Cargo.toml --locked \
  --format-version 1 --filter-platform x86_64-pc-windows-gnu \
  > crates/typex-install/dependency-metadata.json
python3 packaging/license_inventory.py
# 父任务完成真实 backend 后执行（本次没有代为执行/伪造）：
cargo test --manifest-path crates/typex-backend/Cargo.toml --locked
cargo build --manifest-path crates/typex-backend/Cargo.toml --locked \
  --release --target x86_64-pc-windows-gnu
REPO="$PWD"
cd packaging
makensis \
  -DBACKEND_EXE="$REPO/crates/typex-backend/target/x86_64-pc-windows-gnu/release/typex-backend.exe" \
  -DHELPER_EXE="$REPO/crates/typex-install/target/x86_64-pc-windows-gnu/release/typex-install.exe" \
  -DOUTPUT_EXE="$REPO/packaging/typex-demo-0.1.0-setup.exe" typex.nsi
```

rustfmt 未安装；尝试 `rustup component add rustfmt` 下载超时，未宣称格式检查成功。web_extract 后端不支持抽取，固定 libIME2 源码通过 curl 真正取得并核查。Linux 单元测试不能验证 Windows 文件锁、NSIS 自拷贝卸载、UAC SID/session、Add/Enable/RemoveLanguageProfile 与 reload 语义，必须在 VM（含锁文件故障注入、部分删除、未知替换文件、共享 PIME 原 profile、不同用户）逐项验收。
