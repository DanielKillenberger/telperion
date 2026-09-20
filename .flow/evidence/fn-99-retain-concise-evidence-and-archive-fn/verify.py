from pathlib import Path
import hashlib,json,subprocess,tarfile,re
root=Path.cwd();ev=root/'.flow/evidence/fn-91-fast-tree-generation-as-a-core-engine';m=json.loads((ev/'ARCHIVE.json').read_text());archive=Path.home()/'.local/share/telperion/evidence-archives'/('fn-91-'+m['source_revision'][:12]+'.tar.gz')
assert hashlib.sha256(archive.read_bytes()).hexdigest()==m['archive_sha256']
with tarfile.open(archive) as t:
 for row in m['files']:
  p=ev/row['path'];assert not p.exists(),p
  b=t.extractfile(str(p.relative_to(root))).read();assert len(b)==row['bytes'];assert hashlib.sha256(b).hexdigest()==row['sha256']
  assert subprocess.run(['git','check-ignore','--no-index','-q',str(p)]).returncode==0,p
for sample in ['.flow/evidence/future-spec/raw/test.json','demo-video/test.mp4']:
 assert subprocess.run(['git','check-ignore','--no-index','-q',sample]).returncode==0,sample
missing=[]
for p in ev.rglob('*.md'):
 for target in re.findall(r'\]\(([^\s)]+)\)',p.read_text()):
  if '://' in target or target.startswith('#'):continue
  dest=(p.parent/target.split('#')[0]).resolve()
  if not dest.exists():missing.append((str(p.relative_to(root)),target))
assert not missing,missing
changed=subprocess.check_output(['git','diff','--name-only'],text=True).splitlines()
assert not any(x.startswith(('crates/','src/','harness/','tests/','.github/','demo-video/')) for x in changed)
assert not subprocess.check_output(['git','ls-files','demo-video'],text=True).strip()
print(f"PASS: {len(m['files'])} archived files verified; ignore rules, retained Markdown links and code/test/media exclusions checked")
