use typex_install::*;
#[test]
fn partial_deletion_retry_requires_persisted_deleting_phase() {
    let d = tempfile::tempdir().unwrap();
    std::fs::write(d.path().join("typex-backend.exe"), b"backend").unwrap();
    std::fs::write(d.path().join("NOTICE.txt"), b"notice").unwrap();
    let r = Receipt::capture(d.path(), "owner").unwrap();
    let mut value = serde_json::to_value(&r).unwrap();
    value["phase"] = "deleting".into();
    // Model restart from the on-disk journal after the first deletion succeeded.
    std::fs::remove_file(d.path().join("typex-backend.exe")).unwrap();
    assert_eq!(r.verify(d.path(), "owner"), Err(Error::Conflict));
    let retry = serde_json::from_value::<Receipt>(value);
    assert!(retry.is_ok(), "persisted deletion phase must be readable");
    let retry = retry.unwrap();
    assert_eq!(retry.verify(d.path(), "owner"), Ok(()));
    std::fs::write(d.path().join("NOTICE.txt"), b"foreign replacement").unwrap();
    assert_eq!(retry.verify(d.path(), "owner"), Err(Error::Conflict));
}

#[test]
fn staging_failure_and_partial_cleanup_are_retryable_without_deleting_unknown_files() {
    let d = tempfile::tempdir().unwrap();
    std::fs::write(d.path().join("typex-backend.exe"), b"backend").unwrap();
    std::fs::write(d.path().join("NOTICE.txt"), b"notice").unwrap();
    let mut r = Receipt::capture(d.path(), "owner").unwrap();
    r.phase = Phase::Deleting;
    // Simulate a file never copied during staging, or already deleted on a prior attempt.
    std::fs::remove_file(d.path().join("NOTICE.txt")).unwrap();
    std::fs::write(d.path().join("unknown.txt"), b"foreign").unwrap();
    r.save(d.path(), true).unwrap();
    r.cleanup(d.path(), "owner").unwrap();
    r.cleanup(d.path(), "owner").unwrap();
    assert!(d.path().join("unknown.txt").exists());
    assert!(d.path().join("receipt.json").exists());
    assert!(!d.path().join("typex-backend.exe").exists());
}
#[test]
fn cleanup_prechecks_all_remaining_files_before_deleting_anything() {
    let d = tempfile::tempdir().unwrap();
    std::fs::write(d.path().join("typex-backend.exe"), b"backend").unwrap();
    std::fs::write(d.path().join("NOTICE.txt"), b"notice").unwrap();
    let mut r = Receipt::capture(d.path(), "owner").unwrap();
    assert_eq!(r.cleanup(d.path(), "owner"), Err(Error::Conflict));
    r.phase = Phase::Deleting;
    std::fs::write(d.path().join("NOTICE.txt"), b"foreign").unwrap();
    assert_eq!(r.cleanup(d.path(), "owner"), Err(Error::Conflict));
    assert!(d.path().join("typex-backend.exe").exists());
}
#[test] fn receipt_refuses_other_user_version_and_modified_file() {
let d=tempfile::tempdir().unwrap();std::fs::write(d.path().join("typex-backend.exe"),b"actual fixture").unwrap();
let r=Receipt::capture(d.path(),"owner").unwrap();
assert!(r.verify(d.path(),"owner").is_ok());assert_eq!(r.verify(d.path(),"other"),Err(Error::Identity));
std::fs::write(d.path().join("typex-backend.exe"),b"changed").unwrap();assert_eq!(r.verify(d.path(),"owner"),Err(Error::Conflict));
}
#[test] fn receipt_refuses_traversal() {
let r:Receipt=serde_json::from_str(r#"{"version":"0.1.0","profile":"{D93F3E23-0F2D-4D7A-9F28-51DC82B37A60}","owner":"u","backend":{"name":"typex-demo","command":"typex-demo\\typex-backend.exe","workingDir":"typex-demo","params":""},"files":[{"path":"../foreign","sha256":"bad"}]}"#).unwrap();
assert_eq!(r.verify(std::path::Path::new("."),"u"),Err(Error::Conflict));
}
