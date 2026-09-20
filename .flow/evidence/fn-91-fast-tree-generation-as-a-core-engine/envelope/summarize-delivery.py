import json, statistics
from pathlib import Path
p=Path(__file__).resolve().parent
median=statistics.median
geometry=json.loads((p.parent/'position-integration/numeric.json').read_text())
def memory(sample, geo, candidate=True):
    m=sample['stages']; prev=sample['previousTreeGpuBytes']
    v=geo['vertices'];i=geo['proceduralIndices'];u=geo['runs']
    wood=36*v+4*i
    foliage=m['retainedGpuBytes']-wood
    run_table=u*16
    cpu_meta=0 if candidate else m['woodMetadataCpuBytes']
    # Actual final-expansion overlap: prepared CPU positions have already dropped.
    final=prev+foliage+m['woodGpuPeakBytes']+m['baseCpuBytes']+run_table+cpu_meta
    # Conservative envelopes use each measured phase's maximum CPU capacities.
    placement=prev+m['gpuComputePeakBytes']+m['baseCpuBytes']+m['descriptorCpuBytes']+m['sharedMetadataCpuBytes']
    shared=m['sharedMetadataCpuBytes']>0
    prep_cpu=max(m['sharedPrepareCpuBytes'],m['woodPreparedCpuBytes']+m['woodMetadataCpuBytes']+run_table)
    prep_gpu=m.get('positionGpuPeakBytes',0) if candidate else 12*v
    preparation=prev+m['baseCpuBytes']+prep_cpu+prep_gpu+(0 if shared or candidate else foliage)
    return {'previousTree':prev,'woodResident':wood,'foliageResident':foliage,'finalExpansionJoint':final,'placementJointEnvelope':placement,'preparationJointEnvelope':preparation,'accountedJointEnvelope':max(final,placement,preparation),'baseCpu':m['baseCpuBytes'],'positionGpuPeak':m.get('positionGpuPeakBytes',0),'foliageGpuPeak':m['gpuComputePeakBytes'],'woodGpuPeak':m['woodGpuPeakBytes']}
result={'native':{},'browser':[],'limitations':['Native admission screen missed the 20% both-oak target; host authorized exactly one delivery qualification','Two contexts add 4368 native bytes during local growth; no heap allocation; incremental batch guard leaves tables unprepared','Growth frontier/allocator/driver/upload-staging/deferred-destruction overlap is unmeasured; later renderer phase capacities do not qualify a whole-process peak','First-request and initialization observations are desktop browser only; cold startup and phone remain open']}
for mode in ['cpu-output','gpu-output']:
 result['native'][mode]=[]
 for species in ['oregon-white-oak','norway-spruce']:
  rows={}
  for rev in ['baseline','candidate']:
   records=[json.loads(x) for x in (p/f'{mode}-{rev}-{species}.jsonl').read_text().splitlines()]
   samples=[x for x in records if x['event']=='sample']; assert len(samples)==4
   rows[rev]={'initializationMs':records[0]['initializationMs'],'firstMs':samples[0]['totalMs'],'warmMedianMs':median(s['totalMs'] for s in samples[1:]),'warmRangeMs':[min(s['totalMs'] for s in samples[1:]),max(s['totalMs'] for s in samples[1:])]}
  result['native'][mode].append({'species':species,**rows,'gainPercent':100*(1-rows['candidate']['warmMedianMs']/rows['baseline']['warmMedianMs'])})
orig=json.loads((p.parent/'browser-completed-baseline.json').read_text())
a=json.loads((p/'browser-baseline.json').read_text());b=json.loads((p/'browser-candidate.json').read_text())
for old,new,original in zip(a['rows'],b['rows'],orig['rows']):
 assert (old['preset'],old['seed'])==(new['preset'],new['seed'])==(original['preset'],original['seed'])
 bm=median(x['totalToCompletedFrameMs'] for x in old['samples'][1:]);cm=median(x['totalToCompletedFrameMs'] for x in new['samples'][1:]);om=median(x['totalToCompletedFrameMs'] for x in original['samples'][1:])
 row={'species':new['preset'],'seed':new['seed'],'baselineMs':bm,'candidateMs':cm,'gainPercent':100*(1-cm/bm),'originalSpeedup':om/cm,'above100Ms':cm-100,'tenTimesTargetMs':om/10,'baselineInitMs':old['initializationMs'],'candidateInitMs':new['initializationMs'],'firstBaselineMs':old['samples'][0]['totalToCompletedFrameMs'],'firstCandidateMs':new['samples'][0]['totalToCompletedFrameMs'],'samples':6}
 for rev,r in [('baseline',old),('candidate',new)]:
  assert len(r['samples'])==6
  for x in r['samples']:
   stages=x['submitted']['stages'];assert stages['gpuPositions'] is True and stages['positionFallback'] is None and stages['woodFallback'] is None
  row[rev+'Memory']=[memory(x['submitted'],geometry[new['preset']+'-'+str(new['seed'])]['geometry']) for x in r['samples']]
  row[rev+'StageMedians']={k:median(x['submitted']['stages'][k] for x in r['samples'][1:]) for k in r['samples'][0]['submitted']['stages'] if k.endswith('Ms')}
  row[rev+'WasmHighWater']=max(x['wasmMemoryBytes'] for x in r['samples'])
  row[rev+'WarmRangeMs']=[min(x['totalToCompletedFrameMs'] for x in r['samples'][1:]),max(x['totalToCompletedFrameMs'] for x in r['samples'][1:])]
 row['fallbackCount']=0
 row['addedGrowthAdmissionStackBytesNative']=4368
 result['browser'].append(row)
(p/'delivery-summary.json').write_text(json.dumps(result,indent=2)+'\n')
print(json.dumps({'native':result['native'],'browser':[{k:r[k] for k in ['species','seed','baselineMs','candidateMs','gainPercent','originalSpeedup','above100Ms','baselineWasmHighWater','candidateWasmHighWater']} for r in result['browser']]},indent=2))
