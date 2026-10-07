// Linux build-host verification of the actual stdio executable, not an adapter mock.
#[test]
#[cfg(unix)]
fn real_stdio_contract() {
    let status = std::process::Command::new("python3")
        .arg(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/stdio.py"))
        .env("TYPEX_BACKEND_EXE", env!("CARGO_BIN_EXE_typex-backend"))
        .status()
        .expect("Python3 is required for build-host subprocess verification");
    assert!(status.success());
}
