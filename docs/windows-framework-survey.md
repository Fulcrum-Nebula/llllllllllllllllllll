# Windows-first host survey for 20l

## Scope and evidence

Windows is the primary product target. The Linux server is only the research/build environment. This supersedes any Linux-first recommendation in framework-survey.md. Research inspected upstream source snapshots, not merely project descriptions. No Windows build, installation, sensitive-field test, or editor round-trip was performed. No claim of universal application compatibility is made.

Inspected snapshots:
- EasyIME/PIME: `9f6a1e9161b7f609eb1fadf282048c2907da04c9` (commit timestamp 2026-07-25).
- Its actual libIME2 submodule: `717b1901a417667405399cfbf25b25664efcf0e4`.
- rime/weasel: `d73f6295e8252ed2f7b9c12bae32e9001b1afdaa` (commit timestamp 2026-08-18).
- Also inspected standalone EasyIME/libIME2 documentation and source; conclusions about PIME's base were checked against its pinned submodule.

Local read-only research clones: /tmp/gray-win-ime-research/{PIME,libIME2,weasel}. These are disposable, not project dependencies.

## Requirements

Reuse native Windows input infrastructure instead of recreating keyboard hooks, password-field heuristics, composition and submission. Keep the typed functional core independent. Support slash commands, mode switching, TeX/Unicode output and a separate formula editor with continuing IME support. Rich preview is independent of committed text. Opening a real editor creates a child editing session, not merely a candidate popup.

## Finding 1: PIME is a real out-of-process backend host

Sources at the pinned PIME revision:
- backends.json: registers Python, Node and Go executable backends.
- PIMELauncher/src/protocol.rs: input framing `<client_id>|<payload>`, output `PIME_MSG|<client_id>|<payload>`.
- PIMETextService/PIMEClient.cpp: filterKeyDown/onKeyDown, corresponding key-up calls, activation/deactivation, preserved keys, commands, compartment changes, keyboard-open changes, composition termination; init conveys isMetroApp/isUiLess/isConsole.
- PIMEClient.cpp:148-168 implements commitString using the active EditSession context and composition operations. Other response fields update compositionString, compositionCursor and candidates.

Consequence: a Rust backend is a credible integration route without translating 20l into Rime components. It is NOT an existing turnkey Rust 20l adapter. Define Rust structs/enums at the boundary; wire JSON does not require exposing untyped dictionaries to the core. Launcher client_id is routing identity, not proof of a stable per-text-field or saved insertion target.

## Finding 2: important protocol gaps remain

The pinned libIME2 exposes onSetFocus/onKillFocus virtual callbacks. Inspected PIMETextService has an empty onFocus method and does not wire these base callbacks into corresponding backend RPC events. The inspected PIMEClient event surface does not provide the complete focused-document identity/lifetime/target-range protocol needed for parent-child editing sessions. Do not equate language-profile activation with text-field focus.

No InputScope/IS_PASSWORD handling was found in the inspected relevant PIME/base source scan. This does not establish that every sensitive application is unsupported: native keyboard-disabled gating is implemented separately.

The DLL performs synchronous pipe RPC with a 2000 ms per-I/O wait parameter in callRpcMethod. This is an upper wait setting, not measured normal latency or a strict total-RPC bound. Renderer startup, WebView interaction and user editing must never run as long work inside the key-response path.

The commit implementation consumes an EditSession. A delayed external editor result cannot simply be treated as an unsolicited commitString and assumed to work. It needs native-side scheduling, target validity checks and edit-session handling.

## Finding 3: libIME2 is the thinner reuse alternative

https://github.com/EasyIME/libIME2 describes a thin C++ layer over Windows TSF. In the pinned PIME submodule, TextService.cpp:127-129 checks GUID_COMPARTMENT_KEYBOARD_DISABLED and GUID_COMPARTMENT_EMPTYCONTEXT. OnTestKeyDown/OnKeyDown and corresponding up handlers return uneaten when disabled or closed; key processing requests a TSF edit session. TextService also provides composition operations and focus callbacks.

This is a plausible base for a small custom Windows host with an explicit typed IPC contract. It reuses COM/TSF machinery rather than building from raw hooks, but still leaves host packaging, protocol, failure handling and Windows compatibility validation to us. It is not a complete desktop-host product on its own.

## Finding 4: Rime/Weasel is capable but at a different boundary

WeaselTSF/Compartment.cpp checks keyboard-disabled and empty-context compartments; KeyEventSink.cpp uses the disabled check to avoid eating keys. Weasel already supplies a Windows frontend and server architecture, while librime supplies input processing/composition/candidates.

Good fit: deploy mathematical input as Rime components and reuse its ordinary text UI. Less direct fit: 20l owns a separate command/editor state machine and needs a new rich-editor lifecycle protocol. Lua processors do not automatically get all native frontend context capabilities. Reusing Weasel's TSF shell without librime is engineering work, not simply loading a scheme.

Licenses: Weasel root LICENSE.txt is GPLv3, librime is separately BSD-licensed. libIME2 root LICENSE.txt is LGPL 2.1. PIME README describes LGPL 2.1 but root LICENSE.txt lists older directories under LGPL 2.0 and other component licenses. Distribution needs a file/component-level license audit; do not call the whole stack permissively licensed or infer one version from a README.

## Official Windows constraints

1. Microsoft InputScope documentation for IS_PASSWORD states it is not supported and provides no password security. Password fields should disable text services. Therefore do not build a password-safety policy around this enum alone; respect TSF disablement and verify behavior on real applications.
2. ITfContext::RequestEditSession requires correct locking/scheduling. TF_ES_SYNC should only be used in documented situations such as keystroke handling. Requests can fail with TF_E_LOCKED or TF_E_DISCONNECTED; read/write requests can report TS_E_READONLY. S_OK alone does not mean the edit itself completed successfully; phrSession carries additional result information.
3. These constraints do NOT prove all unfocused edits are impossible; they prove an arbitrary saved window handle or session ID is not sufficient to promise deferred insertion.

Sources:
- https://learn.microsoft.com/en-us/windows/win32/api/inputscope/ne-inputscope-inputscope
- https://learn.microsoft.com/en-us/windows/win32/api/msctf/nf-msctf-itfcontext-requesteditsession
- https://learn.microsoft.com/en-us/windows/win32/tsf/predefined-compartments

Microsoft Learn extraction had TLS failures. The same documents were retrieved successfully from MicrosoftDocs/sdk-api and MicrosoftDocs/win32 raw GitHub sources using curl.

## Recommendation

Use PIME + a typed Rust backend for a narrowly scoped Windows feasibility experiment. Explicitly include the required native bridge changes in scope, rather than assuming a backend-only plugin can satisfy the rich-editor requirements. If focus/context/asynchronous-result changes make that bridge overly invasive, use libIME2 directly for a small custom host. Rime/Weasel remains a strong alternative only if adopting Rime's input model becomes desirable. Raw TSF implementation is the final fallback, not the starting point.

Proposed separation:
Windows application ↔ TSF DLL based on reused code ↔ short-latency IPC ↔ 20l Rust core.
A separate companion editor/rendering process communicates with session management, not directly with COM pointers. Carry typed events/results, opaque native target tokens with validity, and explicit failure outcomes. Do not transfer ITfContext pointers across processes as identifiers. Formula editor can use a normal IME-capable input control plus KaTeX/WebView2 preview; this is a design candidate, not validated integration.

Windows acceptance gates before selecting production host:
- Build/register/unregister reproducibly; x64 and required x86 application coverage. ARM64 remains unverified.
- Plain composition/commit/cancel and UTF-16 cursor behavior in Win32, browser/Electron and Office-style applications.
- Actual password fields bypass engine processing as appropriate; no sensitive logs, preview or external dispatch.
- /tex opens an editor without blocking key RPC; editor supports both 20l and an existing Chinese IME without duplicate composition.
- Focus switch, profile switch, target closure, moved cursor, backend crash and editor cancellation preserve draft or fail explicitly.
- Confirm result uses a valid native edit session; stale/ambiguous destination never silently receives text.
- DPI/multi-monitor placement, UI-less behavior and elevated-process boundaries are tested rather than inferred.

No Windows GUI environment is available in the inspected Linux session, so all of these gates remain unexecuted. This report is source-level research, not a working Windows deliverable.
