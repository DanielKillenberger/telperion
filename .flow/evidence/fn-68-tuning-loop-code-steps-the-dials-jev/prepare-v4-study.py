"""Offline independent candidate-admissibility development hypothesis."""
import json,hashlib,math
from pathlib import Path
r=Path(__file__).parent;o=r/'v4-generated';o.mkdir(exist_ok=True)
def serial(v):return json.dumps(v,sort_keys=True,separators=(',',':'),ensure_ascii=False)
policy='Evidence supports this concrete candidate as a worthwhile next bounded experiment: it offers useful new information or progress, without unresolved candidate-specific contrary harm. Bounds alone, unknown evidence, or repeating a known inadequate same probe do not justify it. A harmless but uninformative tiny step is not worthwhile. Judge each candidate independently; both may qualify. Direction is already supported, not itself permission.'
rows=[
 ('two_informative','increase',.05,.01,.12,.01,.03,'Spacing must increase. Neither .06 nor .08 was tried; both are useful isolated informative probes with no contrary harm evidence.',[True,True],['small','substantial']),
 ('harmful_small','increase',3,1,10,1,3,'Count must increase. In this hypothetical scenario candidate4 repeatedly triggers a value-specific construction defect; candidate6 avoids it and is supported by an isolated matched trial. Cause at4 unresolved.',[False,True],['substantial']),
 ('harmful_large','decrease',10,1,12,2,4,'Count must decrease. Candidate8 is supported as an informative bounded probe; candidate6 caused unresolved structural collapse.',[True,False],['small']),
 ('no_basis','decrease',.08,.01,.1,.01,.03,'Spacing must decrease, but no evidence establishes either candidate useful or harmless; prior unexplained defects affected both.',[False,False],[None]),
 ('birth_reduction_replay','decrease',6,1,10,1,2,'Too many births per leader station. Repeated -1 probes helped but remained excessive; -2 is supported.',[True,True],['small','substantial'])]
cases=[];statecases={};questions={}
for i,d,c,lo,hi,small,large,obs,labels,actions in rows:
 vals={};seen=set()
 for size,step in [('small',small),('substantial',large)]:
  v=float(c)+(1 if d=='increase' else -1)*float(step)
  assert math.isfinite(v) and lo<=v<=hi and v!=c and v not in seen
  seen.add(v);vals[size]=v
 case={'id':i,'group':'v3-development-replay' if i.endswith('replay') else 'fresh-authored-development','direction':d,'current':c,'min':lo,'max':hi,'small':small,'substantial':large,'observation':obs,'candidates':vals,'expected':dict(zip(vals,labels)),'acceptable_actions':actions,'unsafe_candidates':['small'] if i=='harmful_small' else ['substantial'] if i=='harmful_large' else list(vals) if i=='no_basis' else []}
 if i.endswith('replay'):case['label_basis']='Both may help: smaller probes helped, not proven futile; larger supported. New admissibility contract differs from v3 exact preferred-size label; v3 result remains unchanged.'
 cases.append(case);statecases[i]={'direction':d,'current':c,'observation':obs,'candidates':vals}
 for size in vals:
  questions[i+'_'+size]={'type':'noul','instructions':f'Apply policy to cases.{i}, candidate {size}.','criteria':{'true':'This concrete candidate is a worthwhile bounded experiment.','false':'This candidate lacks justified useful progress/information or has unresolved contrary harm.'}}
manifest={'schema':'v4-independent-admissibility-development','dispatch_authorized':False,'prior_tokens':212422,'max_additional_tokens':8000,'cumulative_cap':270000,'max_calls':1,'noul_threshold':.8,'gate':{'candidate_accuracy':.8,'actionable_coverage':.8,'accepted_action_accuracy':.8,'unsafe_actions':0},'policy':policy,'selection':'small if probability>=.8; else substantial if>=.8; else abstain; no opposite direction; bounds validated first','provenance':'Four fresh authored policy cases plus unchanged v3 birth_reduction observation, development only. No historical heldout or biological efficacy claim. Noul threshold new, not transferred Choice confidence.','cases':cases}
objects={'manifest':manifest,'state':{'policy':policy,'cases':statecases},'questions':questions}
assert not (r/'v4-calibration-accounting.json').exists()
for name,v in objects.items():(o/(name+'.json')).write_text(serial(v)+'\n')
pre={'reservation':len(serial(objects['state']).encode())+len(serial(questions).encode())+1024,'hashes':{name:hashlib.sha256(serial(v).encode()).hexdigest() for name,v in objects.items()},'dispatch_authorized':False}
(o/'preflight.json').write_text(json.dumps(pre,indent=2)+'\n');print(json.dumps(pre))
