use typex_install::*;
#[test] fn refuses_foreign_same_name_even_identical() {
 let b=serde_json::to_vec(&vec![owned_backend()]).unwrap(); assert_eq!(merge(&b),Err(Error::Conflict));
}
#[test] fn duplicate_or_case_alias_refused() {
 let b=br#"[{"name":"TYPEX-DEMO"}]"#; assert_eq!(merge(b),Err(Error::Conflict));
}
#[test] fn remove_only_exact_owned_item() {
 let b=merge(br#"[{"name":"other","future":42}]"#).unwrap();
 let r=remove(&b,&owned_backend()).unwrap();
 assert_eq!(serde_json::from_slice::<serde_json::Value>(&r).unwrap(),serde_json::json!([{"name":"other","future":42}]));
 let changed=String::from_utf8(b).unwrap().replace("typex-backend.exe","foreign.exe");
 assert_eq!(remove(changed.as_bytes(),&owned_backend()),Err(Error::Conflict));
 assert_eq!(remove(b"[]",&owned_backend()).unwrap(),b"[]");
}
#[test] fn stale_snapshot_never_overwrites() {
 let dir=tempfile::tempdir().unwrap(); let p=dir.path().join("backends.json");
 std::fs::write(&p,b"[]").unwrap(); let s=Snapshot::read(&p).unwrap();
 std::fs::write(&p,b"[ ]").unwrap(); assert_eq!(s.commit(b"[]"),Err(Error::ConcurrentModification));
 assert_eq!(std::fs::read(p).unwrap(),b"[ ]");
}
