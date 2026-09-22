"""Compare guarded delivery code against screen baselines without timing it."""
import os, shutil, subprocess
from pathlib import Path
ev=Path(__file__).resolve().parent
scratch=Path((ev/'scratch-path.txt').read_text().strip())
base=(Path.cwd()/'.flow/tmp/base_commit').read_text().strip()
subprocess.run(['bash','-c','git archive "$1" crates/telperion-core | tar -x -C "$2"','bash',base,str(scratch)],check=True)
(scratch/'crates/telperion-core/src/branching/local/admission.rs').unlink(missing_ok=True)
subprocess.run(['git','apply',str(ev/'delivery.patch')],cwd=scratch,check=True)
s=(ev/'envelope_screen.rs').read_text().replace('path::Path, time::Instant','path::Path').replace('0..4','0..1').replace('        let start = Instant::now();\n','').replace('        let growth_ms = start.elapsed().as_secs_f64() * 1000.;\n','').replace('"growth_ms":growth_ms,','')
(scratch/'crates/telperion-core/examples/envelope_screen.rs').write_text(s)
with (ev/'final-output-build.log').open('w') as log:
 subprocess.run(['cargo','build','--release','-p','telperion-core','--example','envelope_screen'],cwd=scratch,env=dict(os.environ,CARGO_TARGET_DIR=str(scratch/'target')),stdout=log,stderr=subprocess.STDOUT,timeout=600,check=True)
(scratch/'final-output').mkdir(exist_ok=True)
with (ev/'final-output-check.jsonl').open('w') as log:
 for species in ['oregon-white-oak','norway-spruce']:
  for seed in [1,7]:
   subprocess.run([str(scratch/'target/release/examples/envelope_screen'),species,str(seed),str(scratch/'final-output')],stdout=log,timeout=120,check=True)
   assert (scratch/f'final-output/{species}-{seed}.bin').read_bytes()==(scratch/f'baseline-output/{species}-{seed}.bin').read_bytes()
print('guarded full Tree and shed equal for all four fixtures')
