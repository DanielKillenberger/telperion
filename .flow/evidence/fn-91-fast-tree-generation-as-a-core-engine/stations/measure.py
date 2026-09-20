"""Adapted from position-integration/measure.py; serial all-four owned-output runs."""
import os,json,time,hashlib,subprocess
from pathlib import Path
out=Path(__file__).resolve().parent
executables={'baseline':'/tmp/telperion-fn91-tools/generation-parallel-candidate','candidate':'/tmp/telperion-fn91-tools/generation-stations-candidate'}
records=[]
for mode in ['cpu-output','gpu-output']:
 for species in ['oregon-white-oak','norway-spruce']:
  for seed in [1,7]:
   for revision,exe in executables.items():
    path=out/f'{mode}-{revision}-{species}-{seed}.jsonl'
    env={**os.environ,'GENERATION_SAMPLES':'4','GENERATION_SEED':str(seed)}
    start=time.monotonic()
    with path.open('w') as stdout,path.with_suffix('.stderr').open('w') as stderr:
     child=subprocess.Popen([exe,species,mode],stdout=stdout,stderr=stderr,env=env)
     _,status,usage=os.wait4(child.pid,0)
     child.returncode=os.waitstatus_to_exitcode(status)
    records.append({'mode':mode,'revision':revision,'species':species,'seed':seed,'exit':child.returncode,'seconds':time.monotonic()-start,'maxRssKiB':usage.ru_maxrss,'executableSha256':hashlib.sha256(Path(exe).read_bytes()).hexdigest()})
    (out/'delivery-rss.json').write_text(json.dumps(records,indent=2)+'\n')
    if child.returncode: raise SystemExit(child.returncode)
