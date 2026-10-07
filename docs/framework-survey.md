# Input method framework survey — first pass

## Evidence scope

Read upstream source headers/implementation and README documents directly. No framework was installed, built or integration-tested in this pass. URLs below refer to moving branches, not pinned releases. The existing terminal spike remains disposable; its interface is not an approved platform contract.

## Layers and findings

### librime — cross-platform conversion engine

Sources:
- https://github.com/rime/librime/blob/master/README.md
- https://github.com/rime/librime/blob/master/src/rime/engine.cc
- https://github.com/rime/librime/blob/master/src/rime/processor.h
- https://github.com/rime/librime/blob/master/src/rime/context.h

The engine composes processors, segmentors, translators and filters; the inspected implementation also applies formatting and post-processing. Context stores input, caret, composition, selection, options and commit history. ProcessResult distinguishes kAccepted, kRejected and kNoop. Rejection exits the ordinary processor loop, but the inspected engine then runs post-processors before returning unhandled, so do not mistake the header summary for unconditional immediate OS delivery. Frontends integrate the shared engine into Windows, macOS, IBus and other environments.

Lesson: separate platform host from conversion engine. Multi-stage segmentation and candidate streams are useful reference points for future Chinese input, not automatically mandatory components for TyPeX. Distinguish declining a plugin event from explicitly passing it out of a pipeline.

### Fcitx 5 — input-method host/framework

Sources:
- https://github.com/fcitx/fcitx5/blob/master/src/lib/fcitx/inputmethodengine.h
- https://github.com/fcitx/fcitx5/blob/master/src/lib/fcitx/inputcontext.h

InputMethodEngine exposes key events, activation, deactivation, reset, configuration and additional actions. InputContext carries focus, capability flags, surrounding text, input-panel state and per-context properties, with commitString, deleteSurroundingText and forwardKey operations. This is a broader boundary than a string-to-string translator.

Lesson: session/context identity and host capabilities belong in the investigation. Pure plugin state must not accidentally become shared state across applications.

### IBus — bus-based input-method framework

Source: https://github.com/ibus/ibus/blob/main/src/ibusengine.h

IBusEngine is created with a GDBusConnection. Its contract includes key processing, focus, reset, enable/disable, client capabilities, candidate clicks, surrounding text and input content type. Outputs distinguish committed text, preedit, auxiliary text, lookup tables and forwarded keys. update_preedit_text_with_mode explicitly addresses commit behavior on focus loss.

Lesson: preedit is not committed text; focus loss policy must be explicit. Host-side text editing depends on available capabilities. A terminal harness does not exercise this boundary.

### PIME — Windows TSF hosting/bridge

Source: https://github.com/EasyIME/PIME/blob/master/README.md

README describes LibIME as a TSF wrapper and PIMETextService as a text-service backbone, with Python and Node backend-related components and native build/registration requirements. This pass did not inspect its IPC protocol or verify suitability for a Rust backend.

Lesson: investigate reusable TSF hosting before implementing all COM/UI plumbing from scratch. Reuse feasibility and LGPL obligations need a separate review.

### OpenVanilla — current macOS input-method suite

Source: https://github.com/openvanilla/openvanilla/blob/master/README.markdown

Current README describes a macOS suite with built-in methods and importable .cin tables. It explicitly distinguishes current scope from older non-table methods. Do not infer today's architecture from historical descriptions of a cross-platform framework. Only README-level review completed here.

## Proposed consequence for 20l

Keep 20l a portable, typed functional engine and Extension boundary; initially investigate adapters to existing hosts rather than recreating an entire OS input-method framework. TyPeX remains a 20l extension. This is a recommendation, not a frozen architecture or a claim that an adapter already works.

Immediate experiments to inform the contract:
1. Separate per-input-context state and reproduce focus/reset/switch behavior.
2. Preserve preedit, candidate selection and commit as different operations.
3. Normalize key down/up, modifiers and pass-through without turning every key into text.
4. Respect content purpose and capabilities; no implicit availability of surrounding text.
5. Keep byte/code-point/grapheme/platform cursor offsets distinct.
6. For future asynchronous candidates, correlate responses with sessions and input revisions; do not add an effect framework before a real use case.

Next high-value acceptance target: TyPeX preedit and explicit commit through a real host into an ordinary application. Current Linux shell has no DISPLAY/WAYLAND_DISPLAY; real GUI integration needs an accessible desktop or another target environment.
