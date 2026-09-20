#!/usr/bin/env bash
set -euo pipefail
p=.flow/evidence/fn-91-fast-tree-generation-as-a-core-engine/cpu-profile
python3 "$p/analyze.py"
python3 "$p/analyze-followup.py"
rustfmt --check --edition 2021 "$p/cpu_profile.rs"
bash -n "$p/run.sh" "$p/followup.sh"
python3 - <<'PY'
import pathlib, subprocess, tempfile
p=pathlib.Path('.flow/evidence/fn-91-fast-tree-generation-as-a-core-engine/cpu-profile')
for f in p.glob('*.py'): compile(f.read_text(),str(f),'exec')
root=pathlib.Path(tempfile.mkdtemp(prefix='telperion-cpu-profile-transform-check-'))
for f in ['lib.rs','branching.rs','branching/specimen.rs','branching/local/advance.rs','branching/scaffold/frontier.rs','radius.rs','surface/compact.rs','foliage/prepared.rs','branching/local/planner.rs']:
 dst=root/'crates/telperion-core/src'/f; dst.parent.mkdir(parents=True,exist_ok=True)
 dst.write_text((pathlib.Path('crates/telperion-core/src')/f).read_text())
dst=root/'crates/telperion-core/examples/cpu_profile.rs';dst.parent.mkdir(parents=True);dst.write_text((p/'cpu_profile.rs').read_text())
for script,args in [('instrument.py',[]),('followup.py',['prepare']),('followup.py',['sample'])]: subprocess.run(['python3',str(p/script),str(root),*args],check=True)
assert 'planner.clip_sample' in (root/'crates/telperion-core/src/branching/local/planner.rs').read_text()
assert 'let wood =' not in dst.read_text()
print('PASS: 32 canonical CPU hashes, 64 compact hashes; syntax, scoped Rust format and scratch transformations')
PY
