# 20l / TyPeX

Windows-first functional input-method experiment. TyPeX is the initial mathematical Unicode extension, based on UniTeX conversion conventions.

## Experimental Windows build

**Not Windows-runtime validated. Unsigned. Use a disposable Windows 11 x64 VM first, not your primary input environment.** Linux core/IPC tests and Windows cross-compilation pass; native TSF, focus/password behavior, installation/uninstallation and coexistence remain untested. Full distribution-license audit is not complete.

The plugin requires [official PIME 1.3.0-stable](https://github.com/EasyIME/PIME/releases/tag/v1.3.0-stable), installed separately. It neither bundles nor replaces the shared PIME host. Installation currently requires same-user local administrator elevation (not credentials from another account).

After installation, sign out/in and choose **20l TyPeX Demo**, provisionally associated with Traditional Chinese (Taiwan). This is a Windows profile association, not a restriction on mathematical text.

- `Ctrl+Shift+M`: begin mathematical composition.
- Type `\alpha`, then `Enter`: request converted Unicode submission.
- `Ctrl+Enter`: submit original source.
- `Esc`: cancel composition.

The original PIME protocol provides neither commit acknowledgement nor persistent text-field identity. Do not assume draft recovery or safe cross-field continuation. Save or cancel drafts before changing targets.

## Specification and evidence

Engineering order: **SNL specification (Markdown entries) → failing tests → implementation → layered verification**.

- [Specification](docs/spec/typex-windows-demo.md)
- [Integrated build report](docs/reports/typex-integrated-build.md)
- [Core](docs/reports/typex-core.md)
- [Backend](docs/reports/typex-backend.md)
- [Installer, recovery and build commands](docs/reports/typex-install.md)
- [Third-party notices](packaging/NOTICE.txt)

Each crate currently has a standalone Cargo manifest. Tests:

```sh
cargo test --locked --manifest-path crates/typex-core/Cargo.toml
cargo test --locked --manifest-path crates/typex-backend/Cargo.toml
cargo test --locked --manifest-path crates/typex-install/Cargo.toml
python3 -m unittest discover -s packaging -v
```

Backend subprocess tests require Python on the test host; the delivered Windows backend does not require Python or Node.
