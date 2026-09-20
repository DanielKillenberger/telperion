import os, json, time, hashlib, subprocess
from pathlib import Path
out = Path('.flow/evidence/fn-91-fast-tree-generation-as-a-core-engine')
executables = {'baseline': '/tmp/telperion-fn91-tools/generation-gpu-readback-baseline',
               'candidate': '/tmp/telperion-fn91-tools/generation-gpu-readback-candidate'}
records = []
for species in ['oregon-white-oak', 'norway-spruce']:
    for revision, exe in executables.items():
        path = out / f'readback-{revision}-{species}.jsonl'
        start = time.monotonic()
        with path.open('w') as stdout, path.with_suffix('.stderr').open('w') as stderr:
            child = subprocess.Popen([exe, species, 'gpu-output'], stdout=stdout, stderr=stderr,
                                     env={**os.environ, 'GENERATION_SAMPLES': '4'})
            _, status, usage = os.wait4(child.pid, 0)
            child.returncode = os.waitstatus_to_exitcode(status)
        records.append({'revision': revision, 'species': species, 'exit': child.returncode,
                        'seconds': time.monotonic() - start, 'maxRssKiB': usage.ru_maxrss,
                        'executableSha256': hashlib.sha256(Path(exe).read_bytes()).hexdigest()})
        (out / 'readback-rss.json').write_text(json.dumps(records, indent=2) + '\n')
        if child.returncode:
            raise SystemExit(child.returncode)
for species in ['oregon-white-oak', 'norway-spruce']:
    for revision, exe in executables.items():
        with (out / f'readback-hash-{revision}-{species}.jsonl').open('w') as stdout:
            subprocess.run([exe, species, 'gpu-output'], stdout=stdout, check=True,
                           env={**os.environ, 'GENERATION_SAMPLES': '2', 'GENERATION_VERIFY': '1'})
