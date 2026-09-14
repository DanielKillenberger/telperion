"""Run targets after bark_distance that the exact workspace gate cannot reach."""
import json
import subprocess
from pathlib import Path
root = Path(__file__).resolve().parents[4]
out = root / '.flow/evidence/fn31/round3'
targets = ['bark_field', 'bark_filter', 'bark_resolution', 'bark_structure',
           'conformance', 'headless', 'leaf_detail', 'look', 'shadow', 'submit',
           'timing', 'transmission', 'walk']
commands = [
    ('cargo test --release -p telperion-render ' +
     ' '.join('--test ' + name for name in targets) + ' --no-fail-fast', 'renderer-remaining'),
    ('cargo test --release -p telperion-wasm', 'wasm-remaining')]
rows = []
for command, name in commands:
    log = out / 'logs' / (name + '.log')
    print('RUN', command, flush=True)
    with log.open('w') as stream:
        result = subprocess.run(command, shell=True, cwd=root, stdout=stream, stderr=stream)
    rows.append({'command': command, 'exit': result.returncode, 'log': str(log)})
    (out / 'remaining-gates.json').write_text(json.dumps(rows, indent=2) + '\n')
    print('EXIT', result.returncode, str(log), flush=True)
