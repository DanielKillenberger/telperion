from pathlib import Path
import json,statistics
p=Path(__file__).resolve().parent
rss=json.loads((p/'delivery-rss.json').read_text())
original=[942.79,1136.90,5581.80,5338.82]
rows=[]
for mode in ['cpu-output','gpu-output']:
 for idx,(species,seed) in enumerate((s,n) for s in ['oregon-white-oak','norway-spruce'] for n in [1,7]):
  row={'mode':mode,'species':species,'seed':seed};hashes=[]
  for variant in ['baseline','candidate']:
   allrows=[json.loads(l) for l in (p/f'{mode}-{variant}-{species}-{seed}.jsonl').read_text().splitlines()]
   samples=[s for s in allrows if s['event']=='sample']; assert len(samples)==4
   hashes.append([json.dumps([s['instances'],s['bounds']]) for s in samples])
   assert len(set(hashes[-1]))==1
   times=[s['totalMs'] for s in samples[1:]]
   row[variant]={'medianMs':statistics.median(times),'firstMs':samples[0]['totalMs'],'warmRangeMs':[min(times),max(times)],'maxRssKiB':next(r['maxRssKiB'] for r in rss if (r['mode'],r['revision'],r['species'],r['seed'])==(mode,variant,species,seed)),'countAndBounds':json.loads(hashes[-1][0])}
  assert hashes[0]==hashes[1],(mode,species,seed,'hash mismatch')
  row['gainPercent']=100*(1-row['candidate']['medianMs']/row['baseline']['medianMs']);row['originalSpeedup']=original[idx]/row['candidate']['medianMs'];row['rssDeltaKiB']=row['candidate']['maxRssKiB']-row['baseline']['maxRssKiB'];rows.append(row)
(p/'native-summary.json').write_text(json.dumps(rows,indent=2)+'\n')
for r in rows:print(r['mode'],r['species'],r['seed'],r['baseline']['medianMs'],r['candidate']['medianMs'],r['gainPercent'],r['rssDeltaKiB'],r['originalSpeedup'])
