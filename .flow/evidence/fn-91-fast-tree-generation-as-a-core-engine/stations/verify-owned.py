from pathlib import Path
import subprocess,json,os
p=Path(__file__).resolve().parent
bins={'baseline':'/tmp/telperion-fn91-tools/generation-parallel-candidate','candidate':'/tmp/telperion-fn91-tools/generation-stations-candidate'}
rows=[]
for mode in ['cpu-output','gpu-output']:
 for species in ['oregon-white-oak','norway-spruce']:
  for seed in [1,7]:
   pair={}
   for variant,exe in bins.items():
    r=subprocess.run([exe,species,mode],env={**os.environ,'GENERATION_VERIFY':'1','GENERATION_SAMPLES':'1','GENERATION_SEED':str(seed)},capture_output=True,text=True,check=True)
    (p/f'verify-{mode}-{variant}-{species}-{seed}.jsonl').write_text(r.stdout)
    sample=next(json.loads(l) for l in r.stdout.splitlines() if json.loads(l)['event']=='sample')
    pair[variant]=sample['hash'];assert pair[variant] is not None
   assert pair['baseline']==pair['candidate'],(mode,species,seed,pair)
   rows.append({'mode':mode,'species':species,'seed':seed,**pair})
(p/'owned-output-verification.json').write_text(json.dumps(rows,indent=2)+'\n')
print('PASS: all8 requested-output pairs have identical foliage hashes; standalone untimed verification only')
