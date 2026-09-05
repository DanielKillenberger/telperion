"""Collect raw local measurement runs after the documented runners finish."""
import collections
import hashlib
import json
from pathlib import Path
import shutil
import subprocess

root = Path(__file__).resolve().parents[3]
out = Path(__file__).resolve().parent
sources = {
    'fn6-headless.json': '/tmp/fn8-headless-fn6-baseline/results.json',
    'fn6-context.json': '/tmp/fn8-headless-fn6-baseline/context.json',
    'rust-before.json': '/tmp/fn8-headless-rust-before/results.json',
    'rust-headless.json': '/tmp/fn8-headless-rust-final/results.json',
    'rust-memory.json': '/tmp/fn8-rust-memory/results.json',
    'fn6-memory.json': '/tmp/fn8-headless-fn6-memory/results.json',
    'browser-field.json': '/tmp/fn8-field/results.json',
}
for name, source in sources.items():
    shutil.copyfile(source, out / name)
(out/'native').mkdir(exist_ok=True)
for source in Path('/tmp/fn8-native').iterdir():
    shutil.copyfile(source, out/'native'/source.name)
profile = json.loads(Path('/tmp/fn8-wasm-profile.json').read_text())['profile']
nodes = {n['id']: n for n in profile['nodes']}
ranked = [{'samples': count, 'function': nodes[i]['callFrame']['functionName']}
          for i, count in collections.Counter(profile['samples']).most_common()]
(out/'profile-summary.json').write_text(json.dumps({'sampleCount':len(profile['samples']), 'ranked':ranked},indent=2)+'\n')
files = sorted([*root.glob('crates/*/src/**/*.rs'), *root.glob('src/**/*.ts'), *root.glob('harness/**/*.ts')])
(out/'source-hashes.json').write_text(json.dumps({str(p.relative_to(root)):hashlib.sha256(p.read_bytes()).hexdigest() for p in files if not p.name.endswith('.test.ts')},indent=2)+'\n')
def count(paths):
    bodies=[p.read_text().splitlines() for p in paths]
    return {'files':len(bodies), 'lines':sum(map(len,bodies)), 'nonblank':sum(bool(x.strip()) for b in bodies for x in b)}
reference='fdafb099b1495519de75a6b9a66d37f7d07e47bd'
names=subprocess.check_output(['git','ls-tree','-r','--name-only',reference,'src'],cwd=root,text=True).splitlines()
bodies=[subprocess.check_output(['git','show',f'{reference}:{name}'],cwd=root,text=True).splitlines() for name in names if name.endswith('.ts') and not name.endswith('.test.ts')]
counts={'method':'Raw source lines including comments/blanks; nonblank also reported. Not semantic code LOC or whole-project savings. Generated artifacts separated.', 'fn6Core':{'files':len(bodies),'lines':sum(map(len,bodies)),'nonblank':sum(bool(x.strip())for b in bodies for x in b)}}
for label,paths in {
    'rustCore':list(root.glob('crates/telperion-core/src/**/*.rs')),
    'rustBinding':list(root.glob('crates/telperion-wasm/src/**/*.rs')),
    'browserAdapter':[p for p in root.glob('src/**/*.ts') if not p.name.endswith('.generated.ts')],
    'generatedMetadata':list(root.glob('src/**/*.generated.ts')),
    'nativeTests':[*root.glob('crates/telperion-core/tests/**/*.rs'),root/'tests/migration/foundation.rs'],
    'browserTests':list(root.glob('tests/browser/*.mjs')),
    'harnessTests':list(root.glob('harness/**/*.test.ts')),
    'nativeExamples':list(root.glob('crates/telperion-core/examples/*.rs')),
}.items():counts[label]=count(paths)
for label,pattern in [('rustCore','crates/telperion-core/src/**/*.rs'),('rustBinding','crates/telperion-wasm/src/**/*.rs')]:
    inline=[]
    for p in root.glob(pattern):
        body=p.read_text()
        if '#[cfg(test)]' in body:
            test=body.split('#[cfg(test)]',1)[1]
            inline.extend(('#[cfg(test)]'+test).splitlines())
    counts[label]['lines']-=len(inline)
    counts[label]['nonblank']-=sum(bool(x.strip()) for x in inline)
    counts[label+'InlineTests']={'lines':len(inline),'nonblank':sum(bool(x.strip()) for x in inline)}
(out/'loc.json').write_text(json.dumps(counts,indent=2)+'\n')
