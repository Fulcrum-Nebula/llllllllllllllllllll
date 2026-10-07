"""Generate helper dependency inventory and verbatim notices from locked Cargo metadata.
Run cargo metadata --locked --filter-platform x86_64-pc-windows-gnu first.
Backend inventory remains an integration gate; do not imply it is covered here.
"""
import json
from pathlib import Path

root = Path(__file__).resolve().parents[1]
metadata_path = root / 'crates/typex-install/dependency-metadata.json'
metadata = json.loads(metadata_path.read_text())
packages = sorted(metadata['packages'], key=lambda p: (p['name'], p['version']))
rows = ['# Helper dependency inventory', '',
        'Generated from typex-install/Cargo.lock for x86_64-pc-windows-gnu.',
        'Includes development/build dependencies, not just linked code.',
        'Does NOT cover the real backend: parent must add its notices before distribution.', '',
        '| Package | Version | Declared license | Source |', '|---|---|---|---|']
notices = ['TyPeX installer/helper third-party licenses',
           'This inventory includes build/test dependencies. It is not the backend license inventory.',
           'PIME is external, not redistributed. Original installer/icon MIT license: NOTICE.txt.', '']
for p in packages:
    rows.append(f"| {p['name']} | {p['version']} | {p['license'] or 'SEE SOURCE'} | {p['repository'] or p['source'] or 'local'} |")
    if p['source'] is None:
        continue
    directory = Path(p['manifest_path']).parent
    candidates = sorted({f for pattern in ('LICENSE*', 'license*', 'COPYING*', 'UNLICENSE*') for f in directory.glob(pattern) if f.is_file()})
    if not candidates:
        raise RuntimeError(f"Missing license text: {p['name']}")
    notices.append(f"\n{'=' * 72}\n{p['name']} {p['version']} — {p['license']}\n{p['repository']}\n")
    for f in candidates:
        notices.append(f"--- {f.name} ---\n{f.read_text()}\n")
toolchain = Path('/tmp/typex-build-tools/root/usr/share/doc')
rows += ['', '## Toolchain / installer runtime', '',
         'NSIS stub/plugins: zlib/libpng license (see verbatim Debian upstream copyright below).',
         'MinGW-w64 CRT: mixed permissive notices; GCC runtime: GPL with runtime exception.',
         'Toolchain copyright texts below include components not necessarily linked.',
         'Rust standard library/compiler runtime and final backend still require final distribution audit.',
         'PE import inspection found only Windows system DLL names (no libwinpthread/libgcc DLL).']
for name in ('nsis', 'nsis-common', 'mingw-w64-common', 'mingw-w64-x86-64-dev', 'gcc-mingw-w64-base', 'gcc-mingw-w64-x86-64-posix-runtime'):
    f = toolchain / name / 'copyright'
    notices.append(f'\n=== Build toolchain notice: {name} ===\n{f.read_text()}\n')
(root / 'packaging/DEPENDENCIES.md').write_text('\n'.join(rows) + '\n')
(root / 'packaging/THIRD-PARTY-LICENSES.txt').write_text('\n'.join(notices))
print(f'Inventoried {len(packages)} packages; verbatim available license files included.')
