use typex_install::*;
#[test] fn dependency_requires_exact_release_files() { assert_eq!(verify_dependency(std::path::Path::new("/nonexistent-typex")),Err(Error::Dependency)); }
#[test] fn identity_rejects_over_the_shoulder() { assert_eq!(identity_policy("S-1-a","S-1-b",true),Err(Error::Identity)); assert_eq!(identity_policy("S-1-a","S-1-a",false),Err(Error::Identity)); assert_eq!(identity_policy("S-1-a","S-1-a",true),Ok(())); }
#[test] fn fixed_profile_operations_never_service_registration() {
assert_eq!(profile_plan(true),vec![ProfileOperation::Add,ProfileOperation::Enable]);
assert_eq!(profile_plan(false),vec![ProfileOperation::Remove]);
}
