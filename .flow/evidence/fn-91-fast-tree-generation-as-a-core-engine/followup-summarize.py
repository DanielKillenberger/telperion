import json,statistics
from pathlib import Path
out=Path('.flow/evidence/fn-91-fast-tree-generation-as-a-core-engine');summary=[]
for record in json.loads((out/'followup-rss.json').read_text()):
 rows=[json.loads(l) for l in (out/f"followup-{record['revision']}-{record['species']}-{record['mode']}.jsonl").read_text().splitlines()]
 samples=[r for r in rows if r.get('event')=='sample'];warm=samples[1:]
 r={**record,'initializationMs':rows[0]['initializationMs'],'coldTotalMs':samples[0]['totalMs'],'warmP50Ms':statistics.median(x['totalMs'] for x in warm),'warmMaxMs':max(x['totalMs'] for x in warm),'count':samples[0]['instances'],'stagesP50':{k:statistics.median(x['stages'][k] for x in warm) for k,v in warm[0]['stages'].items() if isinstance(v,(float,int))}}
 summary.append(r)
(out/'followup-summary.json').write_text(json.dumps(summary,indent=2)+'\n')
for species in ['oregon-white-oak','norway-spruce']:
 for mode in ['cpu-output','gpu-output','cpu-render','gpu-render']:
  pair=[r for r in summary if r['species']==species and r['mode']==mode]
  print(species,mode,[(round(r['warmP50Ms'],2),r['maxRssKiB']) for r in pair])
