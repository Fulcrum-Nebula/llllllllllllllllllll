Unicode true
RequestExecutionLevel admin
!include "LogicLib.nsh"
!include "x64.nsh"
!ifndef BACKEND_EXE
!error "Supply /DBACKEND_EXE=absolute path to the REAL Windows adapter binary"
!endif
!ifndef HELPER_EXE
!define HELPER_EXE "../crates/typex-install/target/x86_64-pc-windows-gnu/release/typex-install.exe"
!endif
!ifndef OUTPUT_EXE
!define OUTPUT_EXE "typex-demo-0.1.0-setup.exe"
!endif
Name "20l TyPeX Demo 0.1.0 (unsigned experimental plugin)"
OutFile "${OUTPUT_EXE}"
InstallDir "$PROGRAMFILES32\PIME\typex-demo"
ShowInstDetails show
ShowUninstDetails show
Page instfiles
UninstPage uninstConfirm
UninstPage instfiles
Function .onInit
  ${IfNot} ${RunningX64}
    MessageBox MB_ICONSTOP "Windows 11 x64 is required."
    Abort
  ${EndIf}
  SetRegView 64
  ReadRegStr $0 HKLM "SOFTWARE\Microsoft\Windows NT\CurrentVersion" "CurrentBuildNumber"
  IntCmp $0 22000 supported unsupported supported
  unsupported:
    MessageBox MB_ICONSTOP "Windows 11 x64 is required."
    Abort
  supported:
  MessageBox MB_OKCANCEL|MB_ICONEXCLAMATION "Requires separately installed official PIME 1.3.0-stable. No download, update or restart will be performed. Use same-user UAC elevation from your local desktop; another administrator's credentials are unsupported. Save drafts and close PIME settings/installers. This unsigned demo has NOT passed Windows runtime acceptance." IDOK +2
  Abort
FunctionEnd
Section "Install"
  StrCpy $INSTDIR "$PROGRAMFILES32\PIME\typex-demo"
  InitPluginsDir
  SetOutPath "$PLUGINSDIR"
  File /oname=typex-install.exe "${HELPER_EXE}"
  nsExec::ExecToLog '$\"$PLUGINSDIR\typex-install.exe$\" preflight'
  Pop $0
  ${If} $0 == 10
    MessageBox MB_OK "Same version already installed: no files or profiles changed."
    Quit
  ${EndIf}
  ${If} $0 != 0
    MessageBox MB_ICONSTOP "Preflight failed. Install official PIME 1.3.0-stable separately, use the same interactive administrator, and resolve dependency/ownership conflicts. See details."
    Abort
  ${EndIf}
  ; Extract completely in disposable NSIS temp space BEFORE claiming any shared-root path.
  ClearErrors
  SetOutPath "$PLUGINSDIR\payload"
  File /oname=typex-backend.exe "${BACKEND_EXE}"
  File /oname=typex-install.exe "${HELPER_EXE}"
  File "NOTICE.txt"
  File "THIRD-PARTY-LICENSES.txt"
  WriteUninstaller "$PLUGINSDIR\payload\Uninstall.exe"
  SetOutPath "$PLUGINSDIR\payload\input_methods\typex"
  File "ime.json"
  File "icon.ico"
  ${If} ${Errors}
    Abort "Extraction failed before publication; no PIME files changed."
  ${EndIf}
  SetOutPath "$PLUGINSDIR"
  nsExec::ExecToLog '$\"$PLUGINSDIR\typex-install.exe$\" stage-install $\"$PLUGINSDIR\payload$\"'
  Pop $0
  ${If} $0 != 0
    Abort "Staging failed. No profile/config changed. See the recovery instructions for typex-demo.stage; never erase unknown files."
  ${EndIf}
  nsExec::ExecToLog '$\"$PLUGINSDIR\typex-install.exe$\" install'
  Pop $0
  ${If} $0 != 0
    MessageBox MB_ICONSTOP "Installation failed. Owned files are retained for safe recovery; do not restore an old backends.json. Use Uninstall.exe after resolving the reported conflict. No shared host was stopped or re-registered."
    Abort
  ${EndIf}
  WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\TyPeX-D93F3E23" "DisplayName" "20l TyPeX Demo"
  WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\TyPeX-D93F3E23" "DisplayVersion" "0.1.0"
  WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\TyPeX-D93F3E23" "UninstallString" '$\"$INSTDIR\Uninstall.exe$\"'
  MessageBox MB_OK "Plugin installed. Sign out and back in to reload PIME safely, then choose 20l TyPeX Demo in the language UI. Other profiles were not disabled or switched."
SectionEnd
Section "Uninstall"
  StrCpy $INSTDIR "$PROGRAMFILES32\PIME\typex-demo"
  MessageBox MB_OKCANCEL|MB_ICONEXCLAMATION "Submit/cancel drafts and switch away from TyPeX. No launcher or backend will be killed. Sign out first if files are in use. Same-user elevation is required." IDOK +2
  Abort
  ; Embedded recovery helper survives deletion of the installed helper and uninstaller.
  InitPluginsDir
  SetOutPath "$PLUGINSDIR"
  File /oname=typex-install.exe "${HELPER_EXE}"
  nsExec::ExecToLog '$\"$PLUGINSDIR\typex-install.exe$\" uninstall-prepare'
  Pop $0
  ${If} $0 != 0
    MessageBox MB_ICONSTOP "Ownership, identity, dependency or config conflict: cleanup stopped. Files and other profiles preserved. See details."
    Abort
  ${EndIf}
  nsExec::ExecToLog '$\"$PLUGINSDIR\typex-install.exe$\" uninstall-files'
  Pop $0
  ${If} $0 != 0
    MessageBox MB_ICONSTOP "Cleanup incomplete. Receipt retained. Sign out and retry Uninstall.exe, or use the same-version standalone helper uninstall-prepare then uninstall-files. Unknown/replaced files are never deleted."
    Abort
  ${EndIf}
  RMDir "$INSTDIR\input_methods\typex"
  RMDir "$INSTDIR\input_methods"
  IfFileExists "$INSTDIR\*.*" remaining
  RMDir "$INSTDIR"
  remaining:
  SetRegView 64
  DeleteRegKey HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\TyPeX-D93F3E23"
SectionEnd
