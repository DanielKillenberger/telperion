import json, statistics
from pathlib import Path
ev=Path(__file__).resolve().parent
raw={m:[json.loads(l) for l in (ev/f'{m}.jsonl').read_text().splitlines()] for m in ['baseline','candidate']}
result=[]
for preset in ['oregon-white-oak','norway-spruce']:
 for seed in [1,7]:
  paired={m:[r for r in rs if r['event']=='sample' and r['preset']==preset and r['seed']==seed] for m,rs in raw.items()}
  params={m:[r['family'] for r in rs if r['event']=='parameters' and r['preset']==preset and r['seed']==seed] for m,rs in raw.items()}
  assert params['baseline']==params['candidate'] and len(params['baseline'])==1
  assert all([r['sample'] for r in rs]==[0,1,2,3] for rs in paired.values())
  medians={m:statistics.median(r['growth_ms'] for r in rs[1:]) for m,rs in paired.items()}
  row={'preset':preset,'seed':seed,'medians_ms':medians,'change_percent':100*(medians['candidate']/medians['baseline']-1),'first_ms':{m:rs[0]['growth_ms'] for m,rs in paired.items()},'warm_range_ms':{m:[min(r['growth_ms'] for r in rs[1:]),max(r['growth_ms'] for r in rs[1:])] for m,rs in paired.items()}}
  for field in ['nodes','exact_bytes','complete']:
   assert len({r[field] for rs in paired.values() for r in rs})==1
   row[field]=paired['baseline'][0][field]
  result.append(row)
(ev/'summary.json').write_text(json.dumps(result,indent=2)+'\n')
for row in result: print(json.dumps(row))
print('oak_target_pass',all(r['change_percent']<=-20 for r in result if r['preset']=='oregon-white-oak'))
