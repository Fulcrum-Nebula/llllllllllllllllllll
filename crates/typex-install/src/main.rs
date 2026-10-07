#[cfg(windows)] mod windows_host;
use typex_install::*;
fn main(){if let Err(e)=run(){eprintln!("TYPEX_INSTALL_{e:?}");std::process::exit(1)}}
fn run()->Result<(),Error>{
let a:Vec<String>=std::env::args().collect();
if a.len()==3 && a[1]=="verify-dependency" {return verify_dependency(std::path::Path::new(&a[2]))}
#[cfg(windows)] {return windows_run(&a)}
#[cfg(not(windows))] {Err(Error::Dependency)}
}
#[cfg(any(windows,test))]
fn preflight_backend_present(bytes:&[u8],owned:&Backend)->Result<bool,Error>{
// Validate exact ownership and reject duplicates before checking presence.
// Unrelated entries may contain extension fields outside our Backend schema.
remove(bytes,owned)?;
let entries:Vec<serde_json::Value>=serde_json::from_slice(bytes).map_err(|_|Error::InvalidConfig)?;
Ok(entries.iter().any(|b|b.get("name").and_then(|n|n.as_str())==Some(owned.name.as_str())))
}
#[cfg(test)] mod tests {
use super::*;
#[test] fn preflight_accepts_owned_entry_with_unrelated_extensions() {
let bytes=merge(br#"[{"name":"other","command":"other.exe","workingDir":"other","params":"","future":{"enabled":true}}]"#).unwrap();
assert_eq!(preflight_backend_present(&bytes,&owned_backend()),Ok(true));
}
#[test] fn preflight_requires_exact_unique_owned_entry() {
let owned=owned_backend();
assert_eq!(preflight_backend_present(b"[]",&owned),Ok(false));
let bytes=serde_json::to_vec(&vec![owned.clone()]).unwrap();
assert_eq!(preflight_backend_present(&bytes,&owned),Ok(true));
let duplicate=serde_json::to_vec(&vec![owned.clone(),owned.clone()]).unwrap();
assert_eq!(preflight_backend_present(&duplicate,&owned),Err(Error::Conflict));
for changed in [String::from_utf8(bytes.clone()).unwrap().replace("typex-backend.exe","foreign.exe"),String::from_utf8(bytes).unwrap().replace("typex-demo","TYPEX-DEMO")] {
assert_eq!(preflight_backend_present(changed.as_bytes(),&owned),Err(Error::Conflict));
}
assert_eq!(preflight_backend_present(br#"[{"name":"typex-demo","command":"typex-demo\\typex-backend.exe","workingDir":"typex-demo","params":"","future":true}]"#,&owned),Err(Error::Conflict));
assert_eq!(preflight_backend_present(b"not json",&owned),Err(Error::InvalidConfig));
}
}
#[cfg(windows)] fn windows_run(a:&[String])->Result<(),Error>{
use std::fs;use windows_host::*;
if a.len()<2{return Err(Error::InvalidConfig)}
let owner=identity()?;let root=root()?;let dir=root.join(BACKEND);let config=root.join("backends.json");let icon=dir.join("input_methods/typex/icon.ico");
let profiles=Profiles::new()?;
let read_receipt=||->Result<Receipt,Error>{serde_json::from_slice(&fs::read(dir.join("receipt.json")).map_err(|_|Error::Conflict)?).map_err(|_|Error::Conflict)};
match a[1].as_str(){
"preflight"=>{
if dir.exists(){let r=read_receipt()?;r.verify(&dir,&owner)?;
if r.phase!=Phase::Installed{return Err(Error::Conflict)}
let s=Snapshot::read(&config)?;
if preflight_backend_present(&s.bytes,&r.backend)? && profiles.exists(){std::process::exit(10)}
return Err(Error::Conflict)}
if profiles.exists(){return Err(Error::Conflict)}
merge(&Snapshot::read(&config)?.bytes)?;Ok(())
},
"stage-install" if a.len()==3=>{
let source=std::path::Path::new(&a[2]);
for f in PAYLOAD {if !source.join(f).is_file(){return Err(Error::Conflict)}}
let mut r=Receipt::capture(source,&owner)?;
let stage=root.join("typex-demo.stage");
// Never adopt or recursively erase an old/unknown stage. A failed stage retains its journal.
fs::create_dir(&stage).map_err(|_|Error::Conflict)?;
r.phase=Phase::Deleting;r.save(&stage,true)?;
fs::create_dir_all(stage.join("input_methods/typex")).map_err(|_|Error::Io)?;
for f in &r.files {
use std::io::Write;
let bytes=fs::read(source.join(&f.path)).map_err(|_|Error::Io)?;
if digest(&bytes)!=f.sha256{return Err(Error::Conflict)}
let path=stage.join(&f.path);
let mut out=fs::OpenOptions::new().create_new(true).write(true).open(&path).map_err(|_|Error::Conflict)?;
if out.write_all(&bytes).and_then(|_|out.sync_all()).is_err(){drop(out);let _=fs::remove_file(path);return Err(Error::Io)}
}
r.verify(&stage,&owner)?;r.phase=Phase::Installing;r.save(&stage,false)?;
// Windows rename does not replace an existing destination directory; same-volume publication.
fs::rename(&stage,&dir).map_err(|_|Error::Conflict)?;Ok(())
},
"recover-stage"=>{
let stage=root.join("typex-demo.stage");
let mut r:Receipt=serde_json::from_slice(&fs::read(stage.join("receipt.json")).map_err(|_|Error::Conflict)?).map_err(|_|Error::Conflict)?;
if r.phase!=Phase::Deleting && r.phase!=Phase::Installing{return Err(Error::Conflict)}
r.verify(&stage,&owner)?;r.phase=Phase::Deleting;r.save(&stage,false)?;
r.cleanup(&stage,&owner)?;fs::remove_file(stage.join("receipt.json")).map_err(|_|Error::Io)?;
let _=fs::remove_dir(stage.join("input_methods/typex"));let _=fs::remove_dir(stage.join("input_methods"));
fs::remove_dir(stage).map_err(|_|Error::Conflict)
},
"install"=>{
let mut r=read_receipt()?;r.verify(&dir,&owner)?;
if r.phase!=Phase::Installing || profiles.exists(){return Err(Error::Conflict)}
for f in PAYLOAD {if !r.files.iter().any(|owned|owned.path==*f){return Err(Error::Conflict)}}
let s=Snapshot::read(&config)?;let merged=merge(&s.bytes)?;s.commit(&merged)?;
// Journal remains Installing on any failure; uninstall-prepare retries exact profile/config removal.
// Do not restore a stale config snapshot or hide rollback failures.
profiles.apply(ProfileOperation::Add,&icon)?;
profiles.apply(ProfileOperation::Enable,&icon)?;
r.phase=Phase::Installed;r.save(&dir,false)
},
"uninstall-prepare"=>{
if !dir.exists(){return Ok(())}
let mut r=read_receipt()?;r.verify(&dir,&owner)?;
let s=Snapshot::read(&config)?;let removed=remove(&s.bytes,&r.backend)?;
// Already deleting: recheck config/profile but do not downgrade the durable phase.
if r.phase!=Phase::Deleting {r.phase=Phase::Removing;r.save(&dir,false)?;}
if profiles.exists(){profiles.apply(ProfileOperation::Remove,&icon)?;}
s.commit(&removed)?;
r.phase=Phase::Deleting;r.save(&dir,false)
},
"uninstall-files"=>{
if !dir.exists(){return Ok(())}
let r=read_receipt()?;
// Refuse cleanup if someone restored a profile or our backend name after prepare.
if profiles.exists(){return Err(Error::Conflict)}
merge(&Snapshot::read(&config)?.bytes)?;
r.cleanup(&dir,&owner)?;
// All recorded files are gone. Unknown files are preserved by nonrecursive removal.
fs::remove_file(dir.join("receipt.json")).map_err(|_|Error::Io)?;
let _=fs::remove_dir(dir.join("input_methods/typex"));let _=fs::remove_dir(dir.join("input_methods"));
let _=fs::remove_dir(&dir);Ok(())
},
_=>Err(Error::InvalidConfig)
}
}
