#!/usr/bin/env python3
"""Retain original diagnostic PNGs and verified receipts; never assign visibility."""
import hashlib, json, pathlib, shutil, sys
raw=pathlib.Path(sys.argv[1]).resolve()
out=pathlib.Path(__file__).resolve().parent
run=json.loads((raw/'run.json').read_text())
assert run['version']=='fn19-visibility-v2'
assert len(run['records'])==24 and len({(r['case_id'],r['view']) for r in run['records']})==24
assert all(r['status'] in ['captured','fail','interrupted'] for r in run['records'])
(out/'images').mkdir(exist_ok=True)
for record in run['records']:
 for artifact in record['artifacts']:
  data=(raw/artifact['path']).read_bytes()
  assert len(data)==artifact['bytes'] and hashlib.sha256(data).hexdigest()==artifact['sha256']
  if artifact['path'].endswith('-128.png'):
   target=out/'images'/(record['case_id']+'-'+record['view']+'.png')
   shutil.copyfile(raw/artifact['path'],target)
   record['retained_original_png']={'path':str(target.relative_to(out)),**{k:artifact[k] for k in ['bytes','sha256']}}
run['raw_root']=str(raw)
run['raw_receipt_sha256']=hashlib.sha256((raw/'run.json').read_bytes()).hexdigest()
(out/'capture.json').write_text(json.dumps(run,indent=2)+'\n')
print(json.dumps({'records':len(run['records']),'captured':sum(r['status']=='captured' for r in run['records'])}))
