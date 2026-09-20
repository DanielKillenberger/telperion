import json, statistics
from pathlib import Path
ev=Path(__file__).resolve().parent
raw={mode:[json.loads(line) for line in (ev/f'{mode}.jsonl').read_text().splitlines()] for mode in ['baseline','candidate']}
result=[]
for preset in ['oregon-white-oak','norway-spruce']:
 for seed in [1,7]:
  paired={mode:[r for r in rows if r['event']=='sample' and r['preset']==preset and r['seed']==seed] for mode,rows in raw.items()}
  assert all(len(rows)==4 for rows in paired.values())
  row={'preset':preset,'seed':seed}
  for field in ['compact_ms','wood_ms']:
   medians={mode:statistics.median([r[field] for r in rows if r['sample']>0]) for mode,rows in paired.items()}
   row[field]={'medians':medians,'change_percent':100*(medians['candidate']/medians['baseline']-1),'first':{m:rs[0][field] for m,rs in paired.items()},'warm_range':{m:[min(r[field] for r in rs[1:]),max(r[field] for r in rs[1:])] for m,rs in paired.items()}}
  for field in ['exact_bytes','nodes','vertices','dropped','contact_bytes']:
   assert len({r[field] for rows in paired.values() for r in rows})==1
   row[field]=paired['baseline'][0][field]
  result.append(row)
(ev/'summary.json').write_text(json.dumps(result,indent=2)+'\n')
for r in result: print(r['preset'],r['seed'], 'compact',r['compact_ms'],'wood',r['wood_ms'])
print('oak_target_pass',all(r['compact_ms']['change_percent']<=-5 for r in result if r['preset']=='oregon-white-oak'))
