import subprocess,os,json
from pathlib import Path
out=Path('.flow/evidence/fn-91-fast-tree-generation-as-a-core-engine')
results=[]
for species in ['oregon-white-oak','norway-spruce']:
 for seed in [1,7]:
  p=out/f'followup-fingerprint-{species}-{seed}.jsonl'
  with p.open('w') as f:
   subprocess.run(['target/release/examples/generation_stages',species,str(seed)],env={**os.environ,'GENERATION_SAMPLES':'1'},stdout=f,check=True)
  current=json.loads(p.read_text().splitlines()[1])
  previous=json.loads((out/f'cpu-candidate-{species}-{seed}.jsonl').read_text().splitlines()[1])
  result={'species':species,'seed':seed,'previous':previous['output_fnv1a64'],'candidate':current['output_fnv1a64'],'equal':previous['output_fnv1a64']==current['output_fnv1a64']}
  results.append(result);print(json.dumps(result),flush=True)
  assert result['equal']
(out/'followup-fingerprints.json').write_text(json.dumps(results,indent=2)+'\n')
