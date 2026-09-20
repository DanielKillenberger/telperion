from pathlib import Path
import json,statistics as st,platform,subprocess,hashlib
p=Path(__file__).parent
rows={m:[json.loads(l) for l in (p/f'{m}.jsonl').read_text().splitlines()] for m in ['control','instrumented']}
out={}
for species in ['oregon-white-oak','norway-spruce']:
 for seed in [1,7]:
  key=f'{species}-{seed}'; subset={m:[r for r in rr if r.get('preset')==species and r.get('seed')==seed] for m,rr in rows.items()}
  prep={m:[r for r in rr if r['event']=='preparation'] for m,rr in subset.items()}
  cpu={m:[r for r in rr if r['event']=='sample'] for m,rr in subset.items()}
  assert all(len(v)==4 for v in [*prep.values(),*cpu.values()])
  params=[r['family'] for rr in subset.values() for r in rr if r['event']=='parameters']; assert params[0]==params[1]
  hashes={r['output_fnv1a64'] for rr in cpu.values() for r in rr}; assert len(hashes)==1
  compact_hashes={r['compact_hash'] for rr in prep.values() for r in rr}; assert len(compact_hashes)==1
  historical=[json.loads(l) for l in (p.parent/f'cpu-candidate-{key}.jsonl').read_text().splitlines()]
  assert hashes=={r['output_fnv1a64'] for r in historical if r['event']=='sample'}
  for field in ['nodes','placed','retained','wood_vertices','wood_triangles','wood_dropped','complete','height_m']:
   assert len({r[field] for rr in cpu.values() for r in rr})==1,field
  stats={}
  for m,rr in prep.items():
   stats[m]={k:{'first':rr[0][k],'warm': [r[k] for r in rr[1:]],'median':st.median(r[k] for r in rr[1:])} for k in ['growth_ms','compact_ms','stations_ms']}
   sums=[sum(r[k] for k in ['growth_ms','compact_ms','stations_ms']) for r in rr]
   stats[m]['shared_ms']={'first':sums[0],'warm':sums[1:],'median':st.median(sums[1:])}
  phases=[]
  for r in prep['instrumented'][1:]:
   d={}
   for n,v in r['stages']: d[n]=d.get(n,0)+v
   phases.append(d)
  med={k:st.median(d[k] for d in phases) for k in phases[0]}
  out[key]={'timings':stats,'overhead_percent':{k:(stats['instrumented'][k]['median']/stats['control'][k]['median']-1)*100 for k in stats['control']},'phase_medians_ms':med,'canonical_fnv1a64':next(iter(hashes)),'compact_rings_fnv1a64':next(iter(compact_hashes)),'parameter_sha256':hashlib.sha256(json.dumps(params[0],sort_keys=True).encode()).hexdigest(),'counts':{k:cpu['control'][0][k] for k in ['nodes','placed','retained','wood_vertices','wood_triangles','wood_dropped']}}
(p/'summary.json').write_text(json.dumps(out,indent=2)+'\n')
print(json.dumps({k:{'overhead':v['overhead_percent'],'phases':v['phase_medians_ms']} for k,v in out.items()},indent=2))
