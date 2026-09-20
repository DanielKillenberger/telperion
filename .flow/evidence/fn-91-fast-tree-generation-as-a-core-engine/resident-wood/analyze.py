import json,statistics
from pathlib import Path
p=Path(__file__).parent
parent=p.parent
def stats(values):
    return {'p50':statistics.median(values),'min':min(values),'max':max(values),'samples':values}
def native(prefix):
    out=[]
    rss=json.loads((p/f'{prefix}-rss.json').read_text())
    for species in ['oregon-white-oak','norway-spruce']:
        row={'species':species}
        for revision in ['baseline','candidate']:
            data=list(map(json.loads,(p/f'{prefix}-{revision}-{species}.jsonl').read_text().splitlines()))
            samples=[x for x in data if x['event']=='sample']
            row[revision]={'completedMs':stats([s['totalMs'] for s in samples[1:]]),'coldMs':samples[0]['totalMs'],'initializationMs':data[0]['initializationMs'],'maxRssKiB':next(x['maxRssKiB'] for x in rss if x['species']==species and x['revision']==revision)}
        row['improvementPercent']=100*(1-row['candidate']['completedMs']['p50']/row['baseline']['completedMs']['p50'])
        out.append(row)
    return out
cpu=[]
for mode in ['cpu-output','gpu-output']:
    for species in ['oregon-white-oak','norway-spruce']:
        row={'mode':mode,'species':species}
        for revision in ['baseline','candidate']:
            data=list(map(json.loads,(p/f'{mode}-{revision}-{species}.jsonl').read_text().splitlines()))
            samples=[x for x in data if x['event']=='sample']
            row[revision]={'totalMs':stats([s['totalMs'] for s in samples[1:]]),'coldMs':samples[0]['totalMs'],'initializationMs':data[0]['initializationMs']}
        row['changePercent']=100*(row['candidate']['totalMs']['p50']/row['baseline']['totalMs']['p50']-1)
        cpu.append(row)
a=json.loads((parent/'browser-completed-gpu.json').read_text())
b=json.loads((p/'browser-completed.json').read_text())
original=json.loads((parent/'browser-completed-baseline.json').read_text())
browser=[]
for before,after,orig in zip(a['rows'],b['rows'],original['rows']):
    assert (before['preset'],before['seed'])==(after['preset'],after['seed'])==(orig['preset'],orig['seed'])
    row={'species':after['preset'],'seed':after['seed']}
    for name,data in [('baseline',before),('candidate',after),('original',orig)]:
        row[name]={'completedMs':stats([s['totalToCompletedFrameMs'] for s in data['samples'][1:]]),'coldMs':data['samples'][0]['totalToCompletedFrameMs'],'initializationMs':data['initializationMs']}
        if name!='original': row[name]['wasmMemoryBytes']=max(s['wasmMemoryBytes'] for s in data['samples'])
    row['improvementPercent']=100*(1-row['candidate']['completedMs']['p50']/row['baseline']['completedMs']['p50'])
    row['originalSpeedup']=row['original']['completedMs']['p50']/row['candidate']['completedMs']['p50']
    browser.append(row)
result={'nativeFirst':native('resident'),'nativeFinal':native('final-resident'),'cpu':cpu,'browser':browser}
(p/'summary.json').write_text(json.dumps(result,indent=2)+'\n')
for r in result['nativeFinal']: print(r['species'],r['baseline']['completedMs']['p50'],r['candidate']['completedMs']['p50'],r['improvementPercent'],r['baseline']['maxRssKiB'],r['candidate']['maxRssKiB'])
