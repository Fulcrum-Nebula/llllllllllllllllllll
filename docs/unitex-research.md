# UniTeX implementation survey and IME language recommendation

## Scope and sources

- Shared workspace: `/home/sky/workspace/cat/llllllllllllllllllll` (moved from muchen; existing Git metadata and untracked files preserved).
- Upstream: https://github.com/kokic/UniTeX
- Inspected local checkout: `/home/sky/workspace/luye/UniTeX`, commit `6e6294bdfb6101eb6e3dc440e1b9cb5a1a1efdc0`, matching remote main at inspection.
- This is a research recommendation, not an approved implementation specification.

## What UniTeX implements

UniTeX is a TypeScript TeX-to-Unicode transpiler, not an operating-system input method. package.json uses Bun for build/tests. Its exported `parse` function accepts a complete string and returns a string.

Pipeline:

1. `unitex.ts` exports the translator and macro tables.
2. `src/impl/unicode.ts` wires macro tables and rendering callbacks into `createTranslator`.
3. `src/cli.ts` means Common LaTeX Implements, not a command-line interface. It uses custom parser combinators from `src/parsec` for commands, groups, scripts and environments.
4. Fixed macros map names to symbols; unary/binary macros render arguments; Unicode tables handle mathematical alphabets, scripts and accents.
5. Inline output is a string; display output uses `Block` (rows, width, height, baseline) for multiline layout.
6. Output restores placeholders and normalizes to NFC.

Parsing and rendering are interleaved; this does not expose a source-spanned editable formula AST or an IME composition state machine.

## Executed checks

Bun was unavailable. Built the actual source with installed esbuild into `/tmp/unitex-audit-MIQ8FO/` and executed via Node. The existing upstream `test-runner.js` passed: `All tests passed!`. This is a bundled runtime test, not TypeScript type checking or native IME validation.

Observed outputs (JSON strings):

| Input | Output |
| --- | --- |
| `\alpha` | `α` |
| `\mathbb{R}` | `ℝ` |
| `x^2 + y_1` | `x² + y₁` |
| `\frac{a+b}{c}` | `(a+b)/c` |
| `\foo` | `\foo` |
| lone backslash | empty string |
| `\frac{` | `\frac` |
| `a_` | `a` |
| `x^{` | `x` |
| `abc{` | `abc` |

The mathfrak uppercase table also maps H/I to repeated F/G glyphs in this revision. Existing tests do not catch this.

Consequences: do not feed its output directly into committed text on every keystroke. Preserve raw composition text and distinguish complete/incomplete/invalid input. Audit mapping tables instead of copying blindly. `Block` uses JavaScript string length, not grapheme or rendered display width.

## Language recommendation

Recommend Rust for the production core, subject to user agreement. Both Rust and Haskell can implement the grammar and a pure state-transition engine. The deciding factor is native IME integration and deployment, not parser expressiveness or a measured speed comparison.

- Rust: no mandatory tracing-GC runtime; straightforward native library deployment; Windows COM bindings exist in Microsoft's windows crate. FFI, callbacks and platform lifecycles still require careful engineering.
- Haskell: excellent parser/ADT/pure-model fit; native embedding entails the GHC runtime, initialization and GC considerations. This is feasible, not a prohibition, but adds deployment/integration work for a small system IME.
- Windows adapter: TSF COM DLL, with Rust bindings or a minimal C++ bridge where needed.
- Android adapter: Kotlin InputMethodService/UI and InputConnection; Rust core via JNI/NDK. Do not force the Android UI into Rust.

Suggested separation:

`platform events -> composition state machine -> tolerant parser + symbol registry -> Unicode candidate/preview -> explicit commit`

Keep the source buffer and cursor authoritative. Start with symbols, alphabets and scripts; add structured formula conversion separately. Reuse UniTeX's mapping knowledge, examples and inline fallback conventions, not its entire batch parser as the IME engine. No language scaffold has been created yet.

## License and references

UniTeX is MIT, copyright 2023 kokic. Retain its copyright and permission notice when incorporating substantial code or tables.

Verified documentation endpoints:
- https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/UI/TextServices/index.html (ITfTextInputProcessor, ITfKeyEventSink)
- https://developer.android.com/develop/ui/views/touch-and-input/creating-input-method (InputMethodService, InputConnection)
- https://downloads.haskell.org/ghc/latest/docs/users_guide/exts/ffi.html (foreign export, hs_init, hs_exit)

The existing muchen IME report was read as background, not treated as independently verified authority for every claim.
