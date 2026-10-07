# TyPeX route A integrated experimental build

Status: experimental installer compiled; NOT Windows-validated or installable-demo-ready.

Final pre-publication fixes address unrelated backend extension fields in same-version preflight and NSIS /D destination override. Parent independently executed installer tests (15 passing), packaging static tests (4 passing), and verified the rebuilt installer hash. Earlier parent verification covered core tests (18 passing) and backend real subprocess tests (6 scenarios passing). Real Windows GNU backend/helper binaries were packaged using NSIS, not placeholder executables.

Artifact: `packaging/typex-demo-0.1.0-setup.exe`
Bytes: 2127316
SHA-256: `77c9085c0c7d0949961e9a7aabfda0e9d1567d7ce69d9bd0e296d550e72f77b6`
Format: x86 PE32 NSIS GUI installer containing x64 backend/helper.

Prerequisite: separately installed official PIME 1.3.0-stable. Installer targets Windows 11 x64, same-user local administrator elevation. Experimental language profile is associated with Traditional Chinese (Taiwan). No shared host binaries are bundled or replaced.

Windows installation, TSF operation, UI discoverability, password/focus behavior, uninstall/recovery and coexistence have NOT RUN. Backend/runtime full distribution-license audit remains open. Do not treat compiled artifacts or Linux tests as passing these gates. Prefer a disposable Windows VM, not the user's primary input environment.

Component reports describe their respective earlier handoffs; this report identifies the final integrated artifact. No Windows execution or code signing was performed.
