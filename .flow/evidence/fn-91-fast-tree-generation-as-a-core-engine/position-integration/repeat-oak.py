import os,json,time,hashlib,subprocess
from pathlib import Path
out=Path(__file__).resolve().parent
rows=[]
for revision,exe in [('baseline','/tmp/telperion-fn91-tools/generation-gpu-shared-rings'),('candidate','/tmp/telperion-fn91-tools/generation-gpu-positions')]:
    path=out/f'oak-rss-followup-{revision}.jsonl'
    start=time.monotonic()
    with path.open('w') as stdout,path.with_suffix('.stderr').open('w') as stderr:
        child=subprocess.Popen([exe,'oregon-white-oak','gpu-render'],stdout=stdout,stderr=stderr,env={**os.environ,'GENERATION_SAMPLES':'4'})
        _,status,usage=os.wait4(child.pid,0)
        child.returncode=os.waitstatus_to_exitcode(status)
    rows.append({'revision':revision,'exit':child.returncode,'seconds':time.monotonic()-start,'maxRssKiB':usage.ru_maxrss,'executableSha256':hashlib.sha256(Path(exe).read_bytes()).hexdigest(),'cacheState':'unknown; after primary browser/native runs'})
    (out/'oak-rss-followup.json').write_text(json.dumps(rows,indent=2)+'\n')
    if child.returncode:raise SystemExit(child.returncode)
