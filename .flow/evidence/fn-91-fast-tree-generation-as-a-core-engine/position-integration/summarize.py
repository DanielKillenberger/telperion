import json, statistics, hashlib
from pathlib import Path
p=Path(__file__).resolve().parent
median=statistics.median
geometry={}
for line in (p/'mature-production.log').read_text().splitlines():
    if 'PRODUCTION_POSITION ' in line:
        row=json.loads(line.split('PRODUCTION_POSITION ',1)[1]);geometry[row['geometry']['label']]=row
(p/'numeric.json').write_text(json.dumps(geometry,indent=2)+'\n')
def memory(sample, geo, candidate):
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
result={'native':{},'browser':[],'limitations':['No phone/cold-start/full-process peak qualification','Memory excludes allocator overhead, driver/compiler resources, upload staging and deferred destruction','Phase envelopes conservatively combine measured capacities; final expansion snapshot follows actual ownership','Isolated preflight-normal-only rejection fixture remains unproven; shared helper and actual-position normal oracle pass']}
for mode in ['gpu-render','cpu-output','gpu-output']:
    result['native'][mode]=[]
    for species in ['oregon-white-oak','norway-spruce']:
        rows={}
        for rev in ['baseline','candidate']:
            file=p/f'{mode}-{rev}-{species}.jsonl'
            if not file.exists():continue
            records=[json.loads(x) for x in file.read_text().splitlines()]
            samples=[x for x in records if x['event']=='sample']
            if len(samples)!=4:continue
            row={'initializationMs':records[0]['initializationMs'],'firstMs':samples[0]['totalMs'],'warmMedianMs':median(s['totalMs'] for s in samples[1:]),'warmRangeMs':[min(s['totalMs'] for s in samples[1:]),max(s['totalMs'] for s in samples[1:])]}
            if mode=='gpu-render':
                if rev=='candidate':assert all(s['stages']['gpuPositions'] and s['stages']['positionFallback'] is None for s in samples)
                row['memory']=[memory(s,geometry[species+'-1']['geometry'],rev=='candidate') for s in samples]
                row['stageMedians']={k:median(s['stages'][k] for s in samples[1:]) for k in samples[0]['stages'] if k.endswith('Ms')}
            rows[rev]=row
        if len(rows)==2:
            result['native'][mode].append({'species':species,**rows,'gainPercent':100*(1-rows['candidate']['warmMedianMs']/rows['baseline']['warmMedianMs'])})
if (p/'browser-candidate.json').exists():
    orig=json.loads((p.parent/'browser-completed-baseline.json').read_text())
    a=json.loads((p/'browser-baseline.json').read_text());b=json.loads((p/'browser-candidate.json').read_text())
    for old,new,original in zip(a['rows'],b['rows'],orig['rows']):
        assert (old['preset'],old['seed'])==(new['preset'],new['seed'])==(original['preset'],original['seed'])
        bm=median(x['totalToCompletedFrameMs'] for x in old['samples'][1:]);cm=median(x['totalToCompletedFrameMs'] for x in new['samples'][1:]);om=median(x['totalToCompletedFrameMs'] for x in original['samples'][1:])
        for x in new['samples']:assert x['submitted']['stages']['gpuPositions'] is True and x['submitted']['stages']['positionFallback'] is None
        row={'species':new['preset'],'seed':new['seed'],'baselineMs':bm,'candidateMs':cm,'gainPercent':100*(1-cm/bm),'originalSpeedup':om/cm,'above100Ms':cm-100,'tenTimesTargetMs':om/10,'baselineInitMs':old['initializationMs'],'candidateInitMs':new['initializationMs'],'firstBaselineMs':old['samples'][0]['totalToCompletedFrameMs'],'firstCandidateMs':new['samples'][0]['totalToCompletedFrameMs'],'fallbackCount':0,'samples':6}
        for rev,r in [('baseline',old),('candidate',new)]:
            row[rev+'Memory']=[memory(x['submitted'],geometry[new['preset']+'-'+str(new['seed'])]['geometry'],rev=='candidate') for x in r['samples']]
            row[rev+'StageMedians']={k:median(x['submitted']['stages'][k] for x in r['samples'][1:]) for k in r['samples'][0]['submitted']['stages'] if k.endswith('Ms')}
            row[rev+'WasmHighWater']=max(x['wasmMemoryBytes'] for x in r['samples'])
        result['browser'].append(row)
(p/'summary.json').write_text(json.dumps(result,indent=2)+'\n')
print(json.dumps({k:([{x:r[x] for x in ['species','seed','baselineMs','candidateMs','gainPercent','originalSpeedup','above100Ms']} for r in v] if k=='browser' else {m:[{'species':r['species'],'baseline':r['baseline']['warmMedianMs'],'candidate':r['candidate']['warmMedianMs'],'gainPercent':r['gainPercent']} for r in rows] for m,rows in v.items()}) for k,v in result.items() if k in ['browser','native']},indent=2))
