import json,statistics
from pathlib import Path
here=Path(__file__).resolve().parent
root=here.parent
median=statistics.median
geo=json.loads((root/'position-integration/numeric.json').read_text())
cap=json.loads((here/'capacity-results.json').read_text())
orig=json.loads((root/'browser-completed-baseline.json').read_text())
a=json.loads((here/'browser-baseline.json').read_text()); b=json.loads((here/'browser-candidate.json').read_text())
rows=[]
for old,new,original in zip(a['rows'],b['rows'],orig['rows']):
 assert (old['preset'],old['seed'])==(new['preset'],new['seed'])==(original['preset'],original['seed'])
 label=new['preset']+'-'+str(new['seed']);g=geo[label]['geometry']
 r={'species':new['preset'],'seed':new['seed'],'targetMs':median(s['totalToCompletedFrameMs'] for s in original['samples'][1:])/10}
 for variant,row in [('baseline',old),('candidate',new)]:
  c=next(x for x in cap if x['variant']==variant and (x['species'],x['seed'])==(row['preset'],row['seed']))
  times=[s['totalToCompletedFrameMs'] for s in row['samples'][1:]]
  memories=[]
  for s in row['samples']:
   sub=s['submitted'];m=sub['stages'];prev=sub['previousTreeGpuBytes']
   assert m['gpuPositions'] and m['positionFallback'] is None and m['woodFallback'] is None
   wood=36*g['vertices']+4*g['proceduralIndices'];foliage=m['retainedGpuBytes']-wood
   final=prev+foliage+m['woodGpuPeakBytes']+m['baseCpuBytes']+g['runs']*16
   placement=prev+m['gpuComputePeakBytes']+m['baseCpuBytes']+m['descriptorCpuBytes']+m['sharedMetadataCpuBytes']
   if variant=='candidate':
    preparation=prev+m['baseCpuBytes']+m['positionCpuBytes']+m['positionGpuPeakBytes']+c['preparationPeakWithOutputReallocation']
   else:
    preparation=max(prev+m['baseCpuBytes']+c['preparationPeakWithOutputReallocation'],prev+m['baseCpuBytes']+m['sharedPrepareCpuBytes']+m['positionGpuPeakBytes'])
   memories.append({'finalExpansion':final,'placement':placement,'preparationEnvelope':preparation,'jointEnvelope':max(final,placement,preparation)})
  r[variant]={'medianMs':median(times),'warmRangeMs':[min(times),max(times)],'warmSamplesMs':times,'firstMs':row['samples'][0]['totalToCompletedFrameMs'],'initMs':row['initializationMs'],'wasmHighWater':max(s['wasmMemoryBytes'] for s in row['samples']),'memory':memories,'stageMedians':{k:median(s['submitted']['stages'][k] for s in row['samples'][1:]) for k in row['samples'][0]['submitted']['stages'] if k.endswith('Ms')}}
 r['savingMs']=r['baseline']['medianMs']-r['candidate']['medianMs'];r['originalSpeedup']=r['targetMs']*10/r['candidate']['medianMs'];r['above10xMs']=r['candidate']['medianMs']-r['targetMs']
 assert max(x['jointEnvelope'] for x in r['candidate']['memory'])<=max(x['jointEnvelope'] for x in r['baseline']['memory'])
 rows.append(r)
(here/'browser-summary.json').write_text(json.dumps(rows,indent=2)+'\n')
for r in rows:print(r['species'],r['seed'],'baseline',r['baseline']['medianMs'],'candidate',r['candidate']['medianMs'],'originalSpeedup',r['originalSpeedup'],'above10xMs',r['above10xMs'],'memory',max(x['jointEnvelope'] for x in r['candidate']['memory']),'wait',r['candidate']['stageMedians']['positionWaitMs'])
