"""Prepare pinned mature and elevation/azimuth jobs from durable pass-6 receipts.
Run from repo root after building Wasm: python scripts/prepare-species-pass7.py OUT
Execute each job with node tests/browser/species.mjs --worker JOB.json.
"""
import hashlib,json,pathlib,subprocess,sys,math
out=pathlib.Path(sys.argv[1]);out.mkdir(parents=True,exist_ok=True)
sha=lambda p:hashlib.sha256(pathlib.Path(p).read_bytes()).hexdigest()
files=subprocess.check_output(['git','ls-files','src/browser','harness/stage.ts','harness/skeleton-view.ts','package-lock.json'],text=True).splitlines()
hashes={p:sha(p) for p in files}
provenance=dict(sourceHashes=hashes,sourceSha256=hashlib.sha256(json.dumps(hashes,separators=(',',':')).encode()).hexdigest(),commit=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),wasmSha256=sha('src/browser/telperion.wasm'),profilesSha256=sha('.flow/evidence/fn9/profiles.json'),runnerSha256=sha('tests/browser/species.mjs'),rustSources={p:sha(p) for p in subprocess.check_output(['git','ls-files','crates'],text=True).splitlines() if p.endswith('.rs')})
old=json.loads(pathlib.Path('.flow/evidence/fn9/pass6-captures.json').read_text())['captures']
jobs=[]
for c in old:
 if 'scan' in c['view']:continue
 j={k:c[k] for k in ['id','preset','seed','view']}
 j.update(batchInstances=True,frustumCull=True,fixedTarget=c.get('fixedTarget'),provenance=provenance)
 jobs.append(j)
for view in ['peg-clear-front','socket-root-front','socket-tip-front']:
 base=next(j for j in jobs if j['id']=='norway-spruce-1' and j['view']==view)
 for e in [-60,-30,30,60]:
  for a in range(0,360,60):
   j=base.copy();j.update(view=view.replace('front',f'elev{e:+03}az{a:03}-front'),contactAngle=math.radians(a),contactElevation=math.radians(e));jobs.append(j)
for j in jobs:
 name=j['id']+'-'+j['view'];j.update(png=str(out/(name+'.png')),result=str(out/(name+'.json')))
 (out/(name+'-job.json')).write_text(json.dumps(j,indent=2))
(out/'capture-plan.json').write_text(json.dumps(jobs,indent=2))
print(len(jobs),'jobs')
