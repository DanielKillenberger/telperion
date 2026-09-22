"""Offline authored study preparation only; never calls a model."""
import hashlib,json
from pathlib import Path
r=Path(__file__).parent
assert not (r/'v3-calibration-accounting.json').exists(), 'study dispatched: frozen artifacts cannot be regenerated'
def serial(v):return json.dumps(v,sort_keys=True,separators=(',',':'),ensure_ascii=False)
effects={'envelope':'Higher increases seeded radial envelope variation; lower reduces it. Not a visual guarantee.', 'leaves':'Higher adds leaves per short shoot; lower removes them.', 'spacing':'Higher spaces short shoots farther apart; lower closer.', 'limbs':'Higher adds lateral births per leader station; lower removes them; other axes still one.'}
sources={'envelope':'crates/telperion-core/src/envelope.rs:121','leaves':'crates/telperion-core/src/foliage/short_shoots.rs:122','spacing':'crates/telperion-core/src/foliage/short_shoots.rs:201','limbs':'crates/telperion-core/src/branching/scaffold.rs:279'}
# id, parameter, current,min,max,small,large, observation, direction,size
rows=[
 ('lobes_probe','envelope',.12,0,.5,.06,.12,'Boundary variation is too weak. A small isolated probe is informative; no larger-step evidence.','increase','small'),
 ('cluster_recovery','leaves',4,1,12,2,4,'Too few leaves per shoot. Repeated +2 probes helped but remained insufficient; +4 is supported.','increase','substantial'),
 ('gap_probe','spacing',.07,.01,.1,.01,.02,'Short shoots are too far apart. A small reduction is supported; larger reduction previously crowded them.','decrease','small'),
 ('birth_reduction','limbs',6,1,10,1,2,'Too many births per leader station. Repeated -1 probes helped but remained excessive; -2 is supported.','decrease','substantial'),
 ('accepted_cluster','leaves',5,1,12,1,2,'Owner accepts leaves per shoot; no change needed.','hold',None),
 ('contested_spacing','spacing',.06,.01,.1,.01,.02,'Equally reliable matched observations disagree: shoots too close versus too far. No resolution.','insufficient_evidence',None),
 ('bark_only','limbs',4,1,10,1,2,'Only bark hue is wrong. Birth count is unassessed and does not control hue.','insufficient_evidence',None),
 ('harmful_steps','envelope',.24,0,.5,.04,.08,'More boundary variation is supported. Both +.04 and +.08 caused unresolved construction defects; neither is useful now.','increase','insufficient_evidence')]
policy='Which direction is a supported hypothesis to correct the named defect using documented effects? Not a visual guarantee or action permission. Judge direction, not whether a step is safe/useful. Prior evidence can refute causality; harmful sizes block experiments, not an otherwise supported direction. Hold needs explicit acceptance; missing/conflicting causal direction means insufficient_evidence.'
sizepolicy='Choose a worthwhile experiment, not an optimum. Small needs an informative conservative probe; substantial needs larger-probe support. Assess each candidate separately: unresolved harm blocks that candidate, not another supported step. Bounds alone justify nothing; no useful size means insufficient_evidence. Never reverse direction.'
cases=[];ds={};dq={};ss={};sq={}
for i,p,c,lo,hi,small,large,obs,direction,size in rows:
 vals={}
 for sign,name in [(1,'increase'),(-1,'decrease')]:
  available={}
  for label,step in [('small',small),('substantial',large)]:
   n=float(c)+sign*float(step)
   if lo<=n<=hi and n!=c and n not in available.values():available[label]=n
  if available:vals[name]=available
 cases.append({'id':i,'provenance':'new authored development policy case, not historical heldout or biological evidence','parameter':p,'current':c,'min':lo,'max':hi,'small':small,'substantial':large,'observation':obs,'expected':{'direction':direction,'size':size},'available':vals})
 ds[i]={'effect':effects[p],'observation':obs}
 dq[i]={'type':'choice','instructions':f'Apply policy to cases.{i}.','criteria':{k:v for k,v in {'increase':'Increase to address the defect','decrease':'Decrease to address the defect','hold':'Accepted; no change','insufficient_evidence':'Causal direction unsupported'}.items() if k not in ['increase','decrease'] or k in vals}}
 variants=[]
 for name,values in vals.items():
  state={'effect':effects[p],'observation':obs,'current':c,'direction':name,'candidates':values}
  question={'type':'choice','instructions':f'Apply policy to cases.{i}.','criteria':{k:('Supported smaller probe' if k=='small' else 'Supported larger probe') for k in values}}
  question['criteria']['insufficient_evidence']='No justified size'
  variants.append((len(serial(state).encode())+len(serial(question).encode()),state,question))
 _,ss[i],sq[i]=max(variants,key=lambda x:x[0])
manifest={'schema':'authored-v3-offline-review','status':'NOT_AUTHORIZED_FOR_DISPATCH','budget':{'prior':208907,'additional':6000,'cumulative':270000,'calls':2},'confidence':.5,'min_accuracy':.8,'max_unsafe':0,'direction_policy':policy,'size_policy':sizepolicy,'effect_sources':sources,'cases':cases,'scoring':'Separate direction and conditional size fidelity; missing expected size stages fail unconditional fidelity. Composed compares actual action versus no-action disposition; report strict-stage fidelity separately. Actionable coverage denominator four. Unsafe: wrong direction, unavailable value, harmful step, any action on nonactionable case.'}
manifest['effects']=effects
manifest['budget']['additional']=11000
manifest['budget']['authority']='Owner approved11000; host final dispatch review pending'
manifest['provenance']='Fresh authored development variants closely related to v2; not independent heldout or biological efficacy evidence.'
manifest['gates']={k:.8 for k in ['direction','conditional_size','unconditional_expected_size','strict_stage','composed','actionable_coverage']}
manifest['unsafe_candidates']={'gap_probe':['substantial'],'harmful_steps':['small','substantial']}
out=r/'v3-generated';out.mkdir(exist_ok=True)
objects={'manifest':manifest,'direction-state':{'policy':policy,'cases':ds},'direction-questions':dq,'size-worst-state':{'policy':sizepolicy,'cases':ss},'size-worst-questions':sq}
for name,obj in objects.items():
 path=out/(name+'.json');path.write_text(serial(obj)+'\n')
reserve={k:len(serial(objects[k+'-state']).encode())+len(serial(objects[k+'-questions']).encode())+1024 for k in ['direction','size-worst']}
reserve['total']=sum(reserve.values());reserve['hashes']={name+'.json':hashlib.sha256((out/(name+'.json')).read_bytes()).hexdigest() for name in objects};reserve['dispatch_authorized']=False
# Deterministic composition/mock checks, no model calls.
def compose(d,s,dc=.9,sc=.9):return (d,s) if dc>=.5 and d in ['increase','decrease'] and sc>=.5 and s in ['small','substantial'] else None
assert compose('increase','small')==('increase','small')
assert compose('increase','small',.4) is None
assert compose('increase','small',sc=.4) is None
assert compose('hold','small') is None and compose('increase','insufficient_evidence') is None
assert len(cases)==8 and sum(c['expected']['size'] in ['small','substantial'] for c in cases)==4
for c in cases:
 for values in c['available'].values():assert len(set(values.values()))==len(values) and all(c['min']<=v<=c['max'] for v in values.values())
assert all('expected' not in serial(objects[n]) for n in objects if n!='manifest')
reserve['offline_checks']='passed availability/dedup/schema-label-isolation and composition confidence/hold/unknown mocks'
(out/'preflight.json').write_text(json.dumps(reserve,indent=2)+'\n');print(json.dumps(reserve))
