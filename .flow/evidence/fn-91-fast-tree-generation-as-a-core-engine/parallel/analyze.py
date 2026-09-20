from pathlib import Path
import json, statistics
p=Path(__file__).resolve().parent
allrows={m:[r for line in (p/f'{m}.jsonl').read_text().splitlines() if (r:=json.loads(line)).get('event')=='sample'] for m in ['baseline','candidate']}
results=[]
for species in ['oregon-white-oak','norway-spruce']:
 for seed in [1,7]:
  groups={m:[r for r in rows if r['preset']==species and r['seed']==seed] for m,rows in allrows.items()}
  med={m:statistics.median(r['wood_ms'] for r in rows if r['sample']>0) for m,rows in groups.items()}
  results.append({'preset':species,'seed':seed,'baseline_ms':med['baseline'],'candidate_ms':med['candidate'],'speedup':med['baseline']/med['candidate'],'first_ms':{m:rows[0]['wood_ms'] for m,rows in groups.items()},'parallel':groups['candidate'][-1]['parallel']})
(p/'summary.json').write_text(json.dumps(results,indent=2)+'\n')
print(json.dumps(results,indent=2))
