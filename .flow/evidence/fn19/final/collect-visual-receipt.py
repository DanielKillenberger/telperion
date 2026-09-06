"""Collect terminal visual receipts without dropping unsuccessful views or editing images."""
import collections
import hashlib
import json
import pathlib
import shutil
import sys

source = pathlib.Path(sys.argv[1])
destination = pathlib.Path(sys.argv[2])
run = json.loads((source / 'run.json').read_text())
captures = json.loads((source / 'captures.json').read_text())
assert run['status'] in ('complete', 'failed')
assert len(captures) == 84
assert all(c['capture_status'] not in ('pending', 'started') for c in captures)

def receipt(path):
    digest = hashlib.sha256()
    with path.open('rb') as stream:
        for block in iter(lambda: stream.read(1024 * 1024), b''):
            digest.update(block)
    return {'path': str(path), 'bytes': path.stat().st_size, 'sha256': digest.hexdigest()}

verified = {}
views = []
geometry = {}
previews = []
for capture in captures:
    for artifact in capture['artifacts']:
        path = source / artifact['path']
        if str(path) not in verified:
            actual = receipt(path)
            assert actual['bytes'] == artifact['bytes'] and actual['sha256'] == artifact['sha256'], str(path)
            verified[str(path)] = actual
        if path.name == 'geometry.json':
            geometry[capture['case_id']] = json.loads(path.read_text())
    view = dict(capture)
    gap = next((a for a in capture['artifacts'] if a['path'].endswith('/gaps.json')), None)
    view['projected_gaps'] = json.loads((source / gap['path']).read_text()) if gap else {'status': 'unavailable', 'reason': capture['reason'] or ('Projected gaps apply only to whole views.' if capture['view'] != 'whole' else 'No retained gap artifact.')}
    views.append(view)
    index = next(i for i, case in enumerate(run['cases']) if case['id'] == capture['case_id'])
    # Both complete seed1 view sets plus each remaining specimen's whole frontal view.
    if index in (0, 6) or (index == 5 and capture['view'] == 'fork') or (capture['view'] == 'whole' and capture['azimuth_deg'] == 0):
        image = next((a for a in capture['artifacts'] if a['path'].endswith('/beauty-128.png')), None)
        if image:
            target = destination / 'previews' / f"{capture['case_id']}-{capture['view']}-{capture['azimuth_deg']}.png"
            target.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(source / image['path'], target)
            previews.append({'case_id': capture['case_id'], 'view': capture['view'], 'azimuth_deg': capture['azimuth_deg'], **receipt(target)})
result = {'schema_version': 1, 'raw_root': str(source), 'run': run, 'status_counts': dict(collections.Counter(c['capture_status'] for c in captures)), 'raw_receipts': [receipt(source / name) for name in ('run.json', 'captures.json', 'conditions.json', 'manifest.json')], 'verified_artifact_count': len(verified), 'views': views, 'geometry': geometry, 'previews': previews, 'independent_assessment': 'unassessed'}
(destination / 'visual.json').write_text(json.dumps(result, indent=2) + '\n')
print(json.dumps({'status_counts': result['status_counts'], 'verified_artifact_count': len(verified), 'previews': len(previews)}))
