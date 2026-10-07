//! Windows-only profile API boundary. No service registration/unregistration.
use std::{path::{Path,PathBuf},mem::size_of};
use typex_install::*;
use windows::{core::{GUID,PCWSTR},Win32::{Foundation::*,Security::*,System::{Com::*,Threading::*,Registry::*},UI::{TextServices::*,WindowsAndMessaging::*}}};
const CLSID: GUID=GUID::from_u128(0x35f67e9d_a54d_4177_9697_8b0ab71a9e04);
const GUID_PROFILE: GUID=GUID::from_u128(0xd93f3e23_0f2d_4d7a_9f28_51dc82b37a60);
// Provisional Windows profile association (Traditional Chinese/Taiwan), NOT a
// restriction on the language of mathematics. Matches manifest zh-Hant-TW / zh-TW;
// PIME stable DllEntry.cpp -> libIME2 ImeModule.cpp uses LocaleNameToLCID then
// LANGIDFROMLCID. Native locale resolution/profile discoverability remains a Windows gate.
pub const LANGID:u16=0x0404;
fn wide(s:&str)->Vec<u16>{s.encode_utf16().chain(Some(0)).collect()}
struct Handle(HANDLE);
impl Drop for Handle {fn drop(&mut self){unsafe{let _=CloseHandle(self.0);}}}
unsafe fn token(process:HANDLE)->Result<Handle,Error>{let mut h=HANDLE::default();OpenProcessToken(process,TOKEN_QUERY,&mut h).map_err(|_|Error::Identity)?;Ok(Handle(h))}
unsafe fn info(h:HANDLE,class:TOKEN_INFORMATION_CLASS)->Result<Vec<usize>,Error>{
 let mut n=0;let _=GetTokenInformation(h,class,None,0,&mut n);
 if n==0 || n>65536{return Err(Error::Identity)}
 let mut b=vec![0usize;(n as usize+size_of::<usize>()-1)/size_of::<usize>()];
 GetTokenInformation(h,class,Some(b.as_mut_ptr().cast()),n,&mut n).map_err(|_|Error::Identity)?;Ok(b)
}
/// Requires an interactive Explorer shell in THIS session, same SID, elevated token.
/// OTS elevation, service sessions, RDP and missing shell fail closed.
pub fn identity()->Result<String,Error>{unsafe{
 if GetSystemMetrics(SM_REMOTESESSION)!=0{return Err(Error::Identity)}
 let shell=GetShellWindow();if shell.0.is_null(){return Err(Error::Identity)}
 let mut pid=0;GetWindowThreadProcessId(shell,Some(&mut pid));
 let p=Handle(OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION,false,pid).map_err(|_|Error::Identity)?);
 let t=token(GetCurrentProcess())?;let s=token(p.0)?;
 let tb=info(t.0,TokenUser)?;let sb=info(s.0,TokenUser)?;
 let tu=&*(tb.as_ptr() as *const TOKEN_USER);let su=&*(sb.as_ptr() as *const TOKEN_USER);
 EqualSid(tu.User.Sid,su.User.Sid).map_err(|_|Error::Identity)?;
 let elevation=info(t.0,TokenElevation)?;
 if (*(elevation.as_ptr() as *const TOKEN_ELEVATION)).TokenIsElevated==0{return Err(Error::Identity)}
 let sid=std::slice::from_raw_parts(tu.User.Sid.0 as *const u8,GetLengthSid(tu.User.Sid) as usize);
 Ok(digest(sid))
}}
fn registry(view:REG_ROUTINE_FLAGS,key:&str)->Result<String,Error>{unsafe{
 let k=wide(key);let mut buf=vec![0u16;32768];let mut len=(buf.len()*2) as u32;
 RegGetValueW(HKEY_LOCAL_MACHINE,PCWSTR(k.as_ptr()),PCWSTR::null(),RRF_RT_REG_SZ|view,None,Some(buf.as_mut_ptr().cast()),Some(&mut len)).ok().map_err(|_|Error::Dependency)?;
 Ok(String::from_utf16_lossy(&buf[..len as usize/2]).trim_end_matches('\0').to_owned())
}}
pub fn root()->Result<PathBuf,Error>{
 let root=PathBuf::from(std::env::var_os("ProgramFiles(x86)").ok_or(Error::Dependency)?).join("PIME");
 verify_dependency(&root)?;
 for (view,arch) in [(RRF_SUBKEY_WOW6432KEY,"x86"),(RRF_SUBKEY_WOW6464KEY,"x64")] {
 let registered=registry(view,&format!("Software\\Classes\\CLSID\\{}\\InprocServer32",PIME))?;
 if !registered.trim_matches('"').eq_ignore_ascii_case(&root.join(arch).join("PIMETextService.dll").to_string_lossy()){return Err(Error::Dependency)}
 } Ok(root)
}
pub struct Profiles(ITfInputProcessorProfiles);
impl Profiles {
 pub fn new()->Result<Self,Error>{unsafe{CoInitializeEx(None,COINIT_APARTMENTTHREADED).ok().map_err(|_|Error::Io)?;
 let p=CoCreateInstance(&CLSID_TF_InputProcessorProfiles,None,CLSCTX_INPROC_SERVER).map_err(|_|Error::Io)?;Ok(Self(p))}}
 pub fn exists(&self)->bool{unsafe{self.0.GetLanguageProfileDescription(&CLSID,LANGID,&GUID_PROFILE).is_ok()}}
 pub fn apply(&self,op:ProfileOperation,icon:&Path)->Result<(),Error>{unsafe{
 let r=match op {
 ProfileOperation::Add=>self.0.AddLanguageProfile(&CLSID,LANGID,&GUID_PROFILE,&"20l TyPeX Demo".encode_utf16().collect::<Vec<_>>(),&icon.to_string_lossy().encode_utf16().collect::<Vec<_>>(),0),
 ProfileOperation::Enable=>self.0.EnableLanguageProfile(&CLSID,LANGID,&GUID_PROFILE,true),
 ProfileOperation::Remove=>self.0.RemoveLanguageProfile(&CLSID,LANGID,&GUID_PROFILE),
 };r.map_err(|_|Error::Io)
 }}
}
impl Drop for Profiles{fn drop(&mut self){unsafe{CoUninitialize()}}}
