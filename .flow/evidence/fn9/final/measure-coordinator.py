import concurrent.futures,json,pathlib,subprocess,time
out=pathlib.Path('/tmp/fn9-complete-20260906');out.mkdir(exist_ok=True)
assert not list(out.glob("*.jsonl")), "Existing numeric evidence"
seeds=json.loads(pathlib.Path('.flow/evidence/fn9/seeds.json').read_text())
(out/'seeds.json').write_text(json.dumps(seeds,indent=2))
jobs=[(species,seed) for species in seeds['fresh'] for seed in seeds['fixed']+seeds['fresh'][species]]
def run(job):
 species,seed=job;name=f'{species}-{seed}'
 cmd=['target/release/examples/species_measure','--case',f'{name}:{species}:{species}:{seed}','--output',str(out/(name+'.jsonl'))]
 start=time.monotonic()
 try:
  p=subprocess.run(cmd,capture_output=True,text=True,timeout=300)
  result={'command':cmd,'code':p.returncode,'stdout':p.stdout,'stderr':p.stderr,'seconds':time.monotonic()-start,'concurrency':3}
 except subprocess.TimeoutExpired as e:
  result={'command':cmd,'code':-1,'expired':True,'seconds':time.monotonic()-start,'concurrency':3}
 (out/(name+'-process.json')).write_text(json.dumps(result,indent=2))
 print(name,result['code'],flush=True)
 return result['code']
with concurrent.futures.ThreadPoolExecutor(max_workers=3) as pool: results=list(pool.map(run,jobs))
raise SystemExit(any(results))
