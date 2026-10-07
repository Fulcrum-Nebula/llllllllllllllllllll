# 20l project direction

## User-confirmed decisions

- 20l is a minimal, general-purpose input-method engine, not a Chinese or mathematical input method baked into the engine.
- The architecture is functional. The Extension contract must have explicit, precise types; core state/events/effects must not be `Any` or arbitrary JSON dictionaries.
- The first extension is **TyPeX**, providing UniTeX-style mathematical input and TeX-to-Unicode conversion. A Chinese extension may follow independently; it is not a dependency.
- **Windows is the primary product target.** Linux is a research/development environment, not the first shipping IME platform.
- The user selected **route A — “先 A 搞一版看看”: official PIME 1.3.0 stable as a separately installed prerequisite, plus a TyPeX plugin demo**. This supersedes the earlier mandatory custom native bridge and isolated PIME product layout. It does not authorize silently replacing the shared host.
- Work proceeds **SNL specification → derived failing tests → implementation → layered acceptance**. Markdown engineering Entries are allowed. This task is SPEC ONLY, not implementation authorization.

## Frozen route-A specification

[TyPeX Windows installable demo — Markdown SNL Entries](spec/typex-windows-demo.md), version `typex.windows.demo/2-route-a`, preserves the stable `TyPeX.WinDemo.*` IDs. Route selection is approved; detailed contracts are frozen for review, not claimed implemented or accepted.

The slice specifies:

1. A platform-independent typed, pure Extension with authoritative source, scalar cursor, explicit events and effects. Client/session/effect IDs are local bookkeeping, **not native context identities**.
2. The unchanged finite symbol/font/script/fraction conversion contract and C01–C32 examples. Incomplete, unknown and unsupported constructs remain lossless; Converted and Raw are distinct. No whole-string normalization.
3. A typed Rust backend using the official stable launcher's stdio protocol. No mandatory native patch, invented capability handshake, context token, commit acknowledgement or native deduplication.
4. Synchronous commit intent in the current key response: the DLL processes `commitString` in its current TSF edit session, sets the composition text and ends composition. Backend `success:true`, stdout delivery and composition termination are **not proof of insertion**. No async return to a saved target, exactly-once guarantee, acknowledged rollback or reliable recovery after disconnection.
5. A plugin-only installer owning its files, backend config entry and unique TyPeX language profile. Official PIME retains its shared `%ProgramFiles(x86)%\PIME` root, fixed CLSID, launcher, DLLs and startup entries. Shared `backends.json` requires ownership-aware merge/rollback. Uninstall removes only TyPeX profile/config/files; **never shared DLL `regsvr32 /u`, DllUnregisterServer or whole-service Unregister**.
6. Real Windows installation, profile selection, composition/commit/cancel and uninstall acceptance. A terminal REPL, JSON mock or pure-core test is not an installable IME.

Ctrl+Shift+M, narrow grammar, scalar editing and nonselectable textual preview remain detailed specification choices subject to review. No generic dynamic ABI or asynchronous extension pipeline is frozen. Typed interfaces constrain the plugin; they cannot manufacture native guarantees absent from PIME.

## Stable evidence and lifecycle limits

- Stable source baseline: official `v1.3.0-stable`, `26fcf6ac8874e76b8f75f6826811b03bfdfc2e89`. Its launcher source is C++, not the later HEAD Rust launcher; Rust describes the proposed TyPeX backend. HEAD `/tmp/gray-win-ime-research/PIME` is not authoritative for 1.3.0.
- The supplied `/tmp/typex-pime-audit` is absent in this session. Relevant complete stable source files were recovered from the official fixed-commit archive and read at `/tmp/typex-stable-spec-review`; clone/download failures prevented a full checkout. No claim is made that release binaries or libIME2 internals were audited here. Reconcile official binaries/hash/build provenance with the prior audit before packaging.
- Stable `onFocus` is empty; lifecycle callbacks do not identify every field/context change. On observed close/termination/disconnection, discard local editing state without replay. Forced termination may leave raw text in the application; do not call this Converted submission. No suspended-draft resume to a prior target.
- Consecutive non-BMP cursor handling has a source-visible indexing risk. Synchronous pipe calls and connection waits do not justify a 2000 ms RPC bound or guaranteed prompt fallback. Official launcher debug logging includes protocol content; plugin no-logging alone is insufficient.
- Non-BMP editing, cross-field behavior, sensitive controls/host logs, failure recovery and profile-only installation/uninstallation are Windows gates, not assumptions. A gate failure is a blocker requiring explicit scope/route review, not automatic permission to fork native code.

## Preserved scope boundaries

- Rich popup formula editing, slash commands, parent-child editing sessions, another IME inside that editor and asynchronous return to a saved insertion target remain future work, not features supplied by a plain candidate window.
- [Windows framework survey](windows-framework-survey.md) records broader research; [UniTeX research](unitex-research.md) supplies conversion evidence. Older HEAD observations do not override stable-specific findings above.
- `spikes/001-typex` remains exploratory evidence, not the formal parser/transition contract. `.SNL_Doc` is preserved; these Markdown Entries have not been imported or claimed Toolkit-validated.
- This revision modifies only this file and the demo specification. No implementation, test source, commit or push.

## Next authorization gate

Review the frozen route-A specification, then authorize failing tests by Entry/case ID, typed core/stable adapter, plugin installer and real Windows acceptance in that order. The existing A01–A17 matrix remains required before claiming Installable-demo-ready. All Windows gates are **NOT RUN**. Missing Windows access or official-binary verification is a blocker, not permission to substitute Linux results or silently weaken conversion/lifecycle requirements. Behavior changes require specification revision before tests and implementation.
