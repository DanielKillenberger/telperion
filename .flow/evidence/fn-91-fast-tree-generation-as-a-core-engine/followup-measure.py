import os,json,time,hashlib,subprocess
from pathlib import Path
out=Path('.flow/evidence/fn-91-fast-tree-generation-as-a-core-engine')
executables={'previous':'/tmp/telperion-fn91-tools/generation-gpu-first-candidate','candidate':'target/release/examples/generation_gpu'}
records=[]
for species in ['oregon-white-oak','norway-spruce']:
 for mode in ['cpu-output','gpu-output','cpu-render','gpu-render']:
  for revision,exe in executables.items():
   path=out/f'followup-{revision}-{species}-{mode}.jsonl'
   start=time.monotonic()
   with path.open('w') as stdout, path.with_suffix('.stderr').open('w') as stderr:
    child=subprocess.Popen([exe,species,mode],stdout=stdout,stderr=stderr,env={**os.environ,'GENERATION_SAMPLES':'4'})
    _,status,usage=os.wait4(child.pid,0)
    child.returncode=os.waitstatus_to_exitcode(status)
   record={'revision':revision,'species':species,'mode':mode,'exit':child.returncode,'seconds':time.monotonic()-start,'maxRssKiB':usage.ru_maxrss,'executableSha256':hashlib.sha256(Path(exe).read_bytes()).hexdigest()}
   records.append(record)
   (out/'followup-rss.json').write_text(json.dumps(records,indent=2)+'\n')
   print(json.dumps(record),flush=True)
   if child.returncode: raise SystemExit(child.returncode)
