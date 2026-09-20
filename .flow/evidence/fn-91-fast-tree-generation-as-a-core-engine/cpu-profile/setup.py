from pathlib import Path
import json, subprocess, tempfile
repo=Path.cwd(); ev=repo/'.flow/evidence/fn-91-fast-tree-generation-as-a-core-engine/cpu-profile'
revision=json.loads((ev/'provenance.json').read_text())['base_commit']
scratch=Path(tempfile.mkdtemp(prefix='telperion-cpu-profile-'))
subprocess.run(['bash','-c','git archive "$1" | tar -x -C "$2"','bash',revision,str(scratch)],check=True)
(ev/'scratch-path.txt').write_text(str(scratch)+'\n')
(scratch/'crates/telperion-core/examples/cpu_profile.rs').write_text((ev/'cpu_profile.rs').read_text())
print(scratch)
