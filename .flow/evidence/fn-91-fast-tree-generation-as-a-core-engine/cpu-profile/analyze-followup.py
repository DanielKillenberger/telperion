from pathlib import Path
import json,statistics as st
p=Path(__file__).parent
rows={m:[json.loads(l) for l in (p/f'followup-{m}.jsonl').read_text().splitlines()] for m in ['coarse','sampled']}
original=json.loads((p/'summary.json').read_text()); out={}
for key in original:
 species,seed=key.rsplit('-',1); seed=int(seed)
 rr={m:[r for r in rs if r.get('preset')==species and r.get('seed')==seed and r['event']=='preparation'] for m,rs in rows.items()}
 assert all(len(v)==4 for v in rr.values())
 assert {r['compact_hash'] for rs in rr.values() for r in rs}=={original[key]['compact_rings_fnv1a64']}
 estimates=[]
 for r in rr['sampled']:
  sums={}; counts={}
  for n,v in r['stages']: sums[n]=sums.get(n,0)+v; counts[n]=counts.get(n,0)+1
  calls=sums['planner.calls']; sampled=counts['planner.setup_sample']; scale=calls/sampled
  phases={n:sums[n]*scale for n in ['planner.setup_sample','planner.trace_sample','planner.clip_sample','planner.tail_sample']}
  phases['planner.total_estimate']=sum(phases[n] for n in ['planner.setup_sample','planner.trace_sample','planner.tail_sample'])
  estimates.append({'sample':r['sample'],'calls':calls,'sampled':sampled,'sampled_clips':counts['planner.clip_sample'],'estimated_ms':phases,'local_advance_ms':sums['step.local_advance']})
 median={m:st.median(r['growth_ms'] for r in rs[1:]) for m,rs in rr.items()}
 out[key]={'growth_medians_ms':median,'incremental_sampling_overhead_percent':(median['sampled']/median['coarse']-1)*100,'samples':estimates,'warm_estimated_median_ms':{n:st.median(r['estimated_ms'][n] for r in estimates[1:]) for n in estimates[0]['estimated_ms']}}
(p/'followup-summary.json').write_text(json.dumps(out,indent=2)+'\n')
print(json.dumps({k:{a:b for a,b in v.items() if a!='samples'} for k,v in out.items()},indent=2))
