import concurrent.futures,hashlib,json,pathlib,subprocess,os
out=pathlib.Path('/tmp/fn9-complete-20260906');out.mkdir(exist_ok=True)
prior=json.load(open('/tmp/fn9-final-20260906/capture-plan.json'))
sha=lambda p: hashlib.sha256(pathlib.Path(p).read_bytes()).hexdigest()
provenance=dict(prior[0]['provenance'])
assert all(sha(p)==digest for p,digest in provenance['sourceHashes'].items())
provenance.update(commit=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),wasmSha256=sha('src/browser/telperion.wasm'),runnerSha256=sha('tests/browser/species.mjs'))
assert sha('.flow/evidence/fn9/profiles.json')==provenance['profilesSha256']
jobs=[]
for old in prior:
 j=dict(old,provenance=provenance,capture_status='pending')
 for key in ['png','result']:j[key]=str(out/pathlib.Path(j[key]).name)
 jobs.append(j)
(out/'capture-plan.json').write_text(json.dumps(jobs,indent=2))
(out/'provenance.json').write_text(json.dumps(provenance,indent=2))
env=dict(os.environ,CHROMIUM_EXECUTABLE='/tmp/fn9-chromium-gpu',PLAYWRIGHT_MODULE='/tmp/fn9-browser/node_modules/playwright/index.mjs')
def run(j):
 job=out/(j['id']+'-'+j['view']+'-job.json');job.write_text(json.dumps(j))
 with open(out/(j['id']+'-'+j['view']+'-process.log'),'w') as log:
  try: result=subprocess.run(['node','tests/browser/species.mjs','--worker',str(job)],env=env,stdout=log,stderr=subprocess.STDOUT,timeout=300)
  except subprocess.TimeoutExpired:return j,{'capture_status':'fail','error':'Coordinator timeout'}
 if result.returncode:return j,{'capture_status':'fail','error':f'worker exited {result.returncode}'}
 r=json.load(open(j['result']));assert sha(j['png'])==r['pngSha256'];return j,r
with concurrent.futures.ThreadPoolExecutor(max_workers=4) as pool:
 for future in concurrent.futures.as_completed([pool.submit(run,j) for j in jobs]):
  j,r=future.result();j.update(r)
  (out/'captures.json').write_text(json.dumps(jobs,indent=2))
  print(j['id'],j['view'],j['capture_status'],flush=True)
(out/'capture-process.json').write_text(json.dumps({'concurrency':4,'count':len(jobs),'capture_status':'pass' if all(j['capture_status']=='pass' for j in jobs) else 'fail','visual_status':'unassessed','owner_feedback':None},indent=2))
raise SystemExit(not all(j['capture_status']=='pass' for j in jobs))
