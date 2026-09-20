import os, json, time, hashlib, subprocess, sys
from pathlib import Path
out = Path(__file__).resolve().parent
mode = sys.argv[1] if len(sys.argv) > 1 else 'gpu-render'
verify = '--verify' in sys.argv
executables = {'baseline': '/tmp/telperion-fn91-tools/generation-gpu-admission-candidate', 'candidate': '/tmp/telperion-fn91-tools/generation-gpu-shared-rings'}
records = []
for species in ['oregon-white-oak','norway-spruce']:
    for revision, exe in executables.items():
        path = out / f'{mode}{"-verify" if verify else ""}-{revision}-{species}.jsonl'
        env = {**os.environ, 'GENERATION_SAMPLES': '1' if verify else '4'}
        if verify: env['GENERATION_VERIFY'] = '1'
        start = time.monotonic()
        with path.open('w') as stdout, path.with_suffix('.stderr').open('w') as stderr:
            child = subprocess.Popen([exe, species, mode], stdout=stdout, stderr=stderr, env=env)
            _, status, usage = os.wait4(child.pid, 0)
            child.returncode = os.waitstatus_to_exitcode(status)
        records.append({'revision':revision, 'species':species,'exit':child.returncode,'seconds':time.monotonic()-start,'maxRssKiB':usage.ru_maxrss,'executableSha256':hashlib.sha256(Path(exe).read_bytes()).hexdigest()})
        (out / f'{mode}{"-verify" if verify else ""}-rss.json').write_text(json.dumps(records,indent=2)+'\n')
        if child.returncode: raise SystemExit(child.returncode)
