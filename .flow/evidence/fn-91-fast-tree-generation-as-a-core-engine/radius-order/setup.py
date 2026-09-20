"""Rebuild the measured baseline/candidate pair in scratch; do not edit production."""
import json, os, shutil, subprocess, tempfile
from pathlib import Path
root = Path.cwd()
ev = Path(__file__).resolve().parent
base = json.loads((ev / 'provenance.json').read_text())['base']
scratch = Path(tempfile.mkdtemp(prefix='telperion-radius-order-repro-'))
subprocess.run(['bash', '-c', 'git archive "$1" | tar -x -C "$2"', 'bash', base, str(scratch)], check=True)
(ev / 'scratch-path.txt').write_text(str(scratch) + '\n')
shutil.copy2(ev / 'radius_order.rs', scratch / 'crates/telperion-core/examples/radius_order.rs')
p = scratch / 'crates/telperion-core/src/surface/compact.rs'
p.write_text(p.read_text().replace("impl CompactWithContacts<'_> {", "impl CompactWithContacts<'_> {\n    pub fn diagnostic_edges(&self) -> &[Option<[usize; 4]>] { &self.edges }\n"))
env = dict(os.environ, CARGO_TARGET_DIR=str(scratch / 'target'))
for mode in ['baseline', 'candidate']:
    if mode == 'candidate':
        subprocess.run(['git', 'apply', str(ev / 'candidate.patch')], cwd=scratch, check=True)
    with (ev / f'{mode}-build.log').open('w') as log:
        subprocess.run(['cargo', 'build', '--release', '-p', 'telperion-core', '--example', 'radius_order'], cwd=scratch, env=env, stdout=log, stderr=subprocess.STDOUT, timeout=600, check=True)
    shutil.copy2(scratch / 'target/release/examples/radius_order', scratch / mode)
print(scratch)
