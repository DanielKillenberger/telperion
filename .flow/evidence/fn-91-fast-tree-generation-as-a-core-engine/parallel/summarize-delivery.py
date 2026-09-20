import json,statistics
from pathlib import Path
p=Path(__file__).resolve().parent
rss=json.loads((p/'delivery-rss.json').read_text())
original={('oregon-white-oak',1):942.79,('oregon-white-oak',7):1136.90,('norway-spruce',1):5581.80,('norway-spruce',7):5338.82}
results=[]
for mode in ['cpu-output','gpu-output']:
 for species in ['oregon-white-oak','norway-spruce']:
  for seed in [1,7]:
   row={'mode':mode,'species':species,'seed':seed}
   for rev in ['baseline','candidate']:
    records=[json.loads(x) for x in (p/f'{mode}-{rev}-{species}-{seed}.jsonl').read_text().splitlines()]
    samples=[s for s in records if s['event']=='sample'];assert len(samples)==4
    assert records[0]['seed']==seed
    memory=next(r for r in rss if (r['mode'],r['revision'],r['species'],r['seed'])==(mode,rev,species,seed))
    row[rev]={'initializationMs':records[0]['initializationMs'],'firstMs':samples[0]['totalMs'],'warmMedianMs':statistics.median(s['totalMs'] for s in samples[1:]),'warmRangeMs':[min(s['totalMs'] for s in samples[1:]),max(s['totalMs'] for s in samples[1:])],'maxRssKiB':memory['maxRssKiB'],'stageMedians':{k:statistics.median(s['stages'][k] for s in samples[1:]) for k,v in samples[0]['stages'].items() if k.endswith('Ms') and isinstance(v,(float,int))}}
   row['gainPercent']=100*(1-row['candidate']['warmMedianMs']/row['baseline']['warmMedianMs'])
   row['above100Ms']=row['candidate']['warmMedianMs']-100
   row['rssDeltaKiB']=row['candidate']['maxRssKiB']-row['baseline']['maxRssKiB']
   row['originalCpuBaselineSpeedup']=original[species,seed]/row['candidate']['warmMedianMs']
   results.append(row)
(p/'delivery-summary.json').write_text(json.dumps(results,indent=2)+'\n')
print(json.dumps(results,indent=2))
