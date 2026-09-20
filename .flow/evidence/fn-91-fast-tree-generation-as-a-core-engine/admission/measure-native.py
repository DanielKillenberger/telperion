import os, json, time, hashlib, subprocess
from pathlib import Path
out = Path('.flow/evidence/fn-91-fast-tree-generation-as-a-core-engine/admission')
executables = {'baseline': '/tmp/telperion-fn91-tools/generation-gpu-resident-wood-final',
               'candidate': os.environ.get('WOOD_CANDIDATE', '/tmp/telperion-fn91-tools/generation-gpu-admission-candidate')}
prefix = os.environ.get('WOOD_PREFIX', 'resident')
records = []
for species in ['oregon-white-oak', 'norway-spruce']:
    for revision, exe in executables.items():
        path = out / f'{prefix}-{revision}-{species}.jsonl'
        start = time.monotonic()
        with path.open('w') as stdout, path.with_suffix('.stderr').open('w') as stderr:
            child = subprocess.Popen([exe, species, 'gpu-render'], stdout=stdout, stderr=stderr,
                                     env={**os.environ, 'GENERATION_SAMPLES': '4'})
            _, status, usage = os.wait4(child.pid, 0)
            child.returncode = os.waitstatus_to_exitcode(status)
        records.append({'revision': revision, 'species': species, 'exit': child.returncode,
                        'seconds': time.monotonic() - start, 'maxRssKiB': usage.ru_maxrss,
                        'executableSha256': hashlib.sha256(Path(exe).read_bytes()).hexdigest()})
        (out / f'{prefix}-rss.json').write_text(json.dumps(records, indent=2) + '\n')
        if child.returncode:
            raise SystemExit(child.returncode)
