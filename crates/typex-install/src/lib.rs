use serde::{Deserialize, Serialize};
pub const PAYLOAD: &[&str]=&["typex-backend.exe","typex-install.exe","input_methods/typex/ime.json","input_methods/typex/icon.ico","NOTICE.txt","THIRD-PARTY-LICENSES.txt","Uninstall.exe"];
pub const BACKEND: &str = "typex-demo";
pub const PROFILE: &str = "{D93F3E23-0F2D-4D7A-9F28-51DC82B37A60}";
pub const PIME: &str = "{35F67E9D-A54D-4177-9697-8B0AB71A9E04}";
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Backend { pub name: String, pub command: String, pub working_dir: String, pub params: String }
pub fn owned_backend() -> Backend { Backend { name: BACKEND.into(), command: "typex-demo\\typex-backend.exe".into(), working_dir: BACKEND.into(), params: "".into() } }
#[derive(Debug, PartialEq, Eq)]
pub enum Error { InvalidConfig, Conflict, ConcurrentModification, Io, Dependency, Identity }
pub fn merge(bytes: &[u8]) -> Result<Vec<u8>, Error> {
let mut entries: Vec<Box<serde_json::value::RawValue>> = serde_json::from_slice(bytes).map_err(|_| Error::InvalidConfig)?;
for entry in &entries { if name(entry)?.eq_ignore_ascii_case(BACKEND) { return Err(Error::Conflict); } }
entries.push(serde_json::value::to_raw_value(&owned_backend()).map_err(|_| Error::InvalidConfig)?);
serde_json::to_vec_pretty(&entries).map_err(|_| Error::InvalidConfig)
}
fn name(raw: &serde_json::value::RawValue) -> Result<String,Error> {
#[derive(Deserialize)] struct Named { name: String }
serde_json::from_str::<Named>(raw.get()).map(|n|n.name).map_err(|_|Error::InvalidConfig)
}
pub fn remove(bytes: &[u8], owned: &Backend) -> Result<Vec<u8>,Error> {
let entries: Vec<Box<serde_json::value::RawValue>> = serde_json::from_slice(bytes).map_err(|_|Error::InvalidConfig)?;
let mut result=Vec::new(); let mut found=false;
for entry in entries {
if name(&entry)?.eq_ignore_ascii_case(BACKEND) {
if found || serde_json::from_str::<Backend>(entry.get()).map_err(|_|Error::Conflict)? != *owned { return Err(Error::Conflict); }
found=true;
} else { result.push(entry); }
}
serde_json::to_vec_pretty(&result).map_err(|_|Error::InvalidConfig)
}
pub struct Snapshot { path: std::path::PathBuf, pub bytes: Vec<u8> }
impl Snapshot {
pub fn read(path: &std::path::Path) -> Result<Self,Error> {
Ok(Self { path: path.into(), bytes: std::fs::read(path).map_err(|_|Error::Io)? })
}
pub fn commit(self, bytes: &[u8]) -> Result<(),Error> {
use std::io::Write;
let lock=self.path.with_extension("typex-lock");
let _guard=std::fs::OpenOptions::new().write(true).create_new(true).open(&lock).map_err(|_|Error::ConcurrentModification)?;
let result=(|| {
if std::fs::read(&self.path).map_err(|_|Error::Io)? != self.bytes { return Err(Error::ConcurrentModification); }
let temp=self.path.with_extension("typex-new");
let mut f=std::fs::OpenOptions::new().write(true).create_new(true).open(&temp).map_err(|_|Error::Conflict)?;
let r=(|| {f.write_all(bytes).map_err(|_|Error::Io)?; f.sync_all().map_err(|_|Error::Io)?;
if std::fs::read(&self.path).map_err(|_|Error::Io)? != self.bytes { return Err(Error::ConcurrentModification); }
replace(&temp,&self.path)})();
if r.is_err() { let _=std::fs::remove_file(&temp); } r
})();
drop(_guard); let _=std::fs::remove_file(lock); result
}
}
#[cfg(not(windows))] fn replace(from: &std::path::Path,to: &std::path::Path)->Result<(),Error> {std::fs::rename(from,to).map_err(|_|Error::Io)}
#[cfg(windows)] fn replace(from: &std::path::Path,to: &std::path::Path)->Result<(),Error> {
use windows::{core::PCWSTR,Win32::Storage::FileSystem::{MoveFileExW,MOVEFILE_REPLACE_EXISTING,MOVEFILE_WRITE_THROUGH}};
use std::os::windows::ffi::OsStrExt;
let f:Vec<u16>=from.as_os_str().encode_wide().chain(Some(0)).collect();
let t:Vec<u16>=to.as_os_str().encode_wide().chain(Some(0)).collect();
unsafe {MoveFileExW(PCWSTR(f.as_ptr()),PCWSTR(t.as_ptr()),MOVEFILE_REPLACE_EXISTING|MOVEFILE_WRITE_THROUGH)}.map_err(|_|Error::Io)
}
pub const DEPENDENCY_FILES: &[(&str,&str)] = &[
("version.txt","b56a1cde7e9aff5609cfe8076e8c7b4d119722c7b12b848ddcd53846cb4cf58f"),
("PIMELauncher.exe","c6da2c6545e902511087993ccd9002a50e14c1f26ea6be35a280814f8db6786c"),
("x86/PIMETextService.dll","7b77e8d0cdc2052b1da54ac212cb48bea391930811b4020c3e0773f18b9fd1fb"),
("x64/PIMETextService.dll","00e3a27cbc3c038b745e9fe4b3e74e433347f84125f9ab92730d239b5e9858a4")];
pub fn digest(bytes:&[u8])->String {use sha2::{Digest,Sha256}; format!("{:x}",Sha256::digest(bytes))}
pub fn verify_dependency(root: &std::path::Path)->Result<(),Error>{
for (path,hash) in DEPENDENCY_FILES {if digest(&std::fs::read(root.join(path)).map_err(|_|Error::Dependency)?) != *hash {return Err(Error::Dependency)}} Ok(())
}
pub fn identity_policy(token: &str,session: &str,elevated:bool)->Result<(),Error>{
if token.is_empty() || token!=session || !elevated {Err(Error::Identity)} else {Ok(())}
}
#[derive(Debug,PartialEq,Eq)] pub enum ProfileOperation {Add,Enable,Remove}
pub fn profile_plan(add:bool)->Vec<ProfileOperation>{if add {vec![ProfileOperation::Add,ProfileOperation::Enable]}else{vec![ProfileOperation::Remove]}}
#[derive(Debug,Serialize,Deserialize)] #[serde(deny_unknown_fields)]
pub struct OwnedFile {pub path:String,pub sha256:String}
#[derive(Debug,Clone,Copy,PartialEq,Eq,Default,Serialize,Deserialize)]
#[serde(rename_all="kebab-case")]
pub enum Phase { #[default] Installing, Installed, Removing, Deleting }
#[derive(Debug,Serialize,Deserialize)] #[serde(deny_unknown_fields)]
pub struct Receipt {pub version:String,pub profile:String,pub owner:String,pub backend:Backend,pub files:Vec<OwnedFile>, #[serde(default)] pub phase:Phase}
impl Receipt {
/// Atomically persist a phase before any destructive step; never overwrite an unknown journal.
pub fn save(&self,root:&std::path::Path,initial:bool)->Result<(),Error>{
use std::io::Write;
let bytes=serde_json::to_vec_pretty(self).map_err(|_|Error::Io)?;
let path=root.join("receipt.json");
if initial {
let mut f=std::fs::OpenOptions::new().write(true).create_new(true).open(&path).map_err(|_|Error::Conflict)?;
f.write_all(&bytes).map_err(|_|Error::Io)?;f.sync_all().map_err(|_|Error::Io)
} else {Snapshot::read(&path)?.commit(&bytes)}
}
/// Retain the journal until every owned file is gone. Unknown paths are never enumerated for deletion.
pub fn cleanup(&self,root:&std::path::Path,owner:&str)->Result<(),Error>{
if self.phase!=Phase::Deleting{return Err(Error::Conflict)}
self.verify(root,owner)?;
for f in &self.files {
let path=root.join(&f.path);
match std::fs::read(&path) {
Ok(bytes) if digest(&bytes)==f.sha256 => std::fs::remove_file(&path).map_err(|_|Error::Io)?,
Err(e) if e.kind()==std::io::ErrorKind::NotFound => {},
_=>return Err(Error::Conflict),
}
} Ok(())
}
pub fn capture(root: &std::path::Path,owner:&str)->Result<Self,Error>{
let mut files=Vec::new();for path in PAYLOAD {let p=root.join(path);if p.exists(){files.push(OwnedFile{path:(*path).into(),sha256:digest(&std::fs::read(p).map_err(|_|Error::Io)?)});}}
Ok(Self{version:env!("CARGO_PKG_VERSION").into(),profile:PROFILE.into(),owner:owner.into(),backend:owned_backend(),files,phase:Phase::Installing})
}
pub fn verify(&self,root:&std::path::Path,owner:&str)->Result<(),Error>{
if self.owner!=owner{return Err(Error::Identity)}
if self.version!=env!("CARGO_PKG_VERSION") || self.profile!=PROFILE || self.backend!=owned_backend(){return Err(Error::Conflict)}
let mut seen=std::collections::HashSet::new();
for f in &self.files {if !PAYLOAD.contains(&f.path.as_str()) || !seen.insert(&f.path) {return Err(Error::Conflict)}
match std::fs::read(root.join(&f.path)) {
Ok(bytes) if digest(&bytes)==f.sha256 => {},
Err(e) if e.kind()==std::io::ErrorKind::NotFound && self.phase==Phase::Deleting => {},
_=>return Err(Error::Conflict),
}} Ok(())
}
}
#[cfg(test)] mod tests {
use super::*;
#[test] fn merge_preserves_unrelated_unknown_fields() {
let old = br#"[{"name":"python","command":"python.exe","workingDir":"python","params":"server.py","future":{"enabled":true}}]"#;
let result = merge(old).unwrap();
let v: serde_json::Value = serde_json::from_slice(&result).unwrap();
assert_eq!(v[0]["future"]["enabled"], true);
assert_eq!(v[1]["name"], BACKEND);
assert_eq!(v.as_array().unwrap().len(), 2);
}
}
