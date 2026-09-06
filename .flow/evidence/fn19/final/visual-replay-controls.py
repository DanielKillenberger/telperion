"""Compare real independent captures, then a declared substituted-image receipt fixture."""
import argparse
import hashlib
import json
import pathlib
import shutil
import subprocess

parser = argparse.ArgumentParser()
parser.add_argument('--native-controls', type=pathlib.Path, required=True)
parser.add_argument('--visual-controls', type=pathlib.Path, required=True)
parser.add_argument('--output', type=pathlib.Path, required=True)
args = parser.parse_args()
args.output.mkdir()
base = args.visual_controls / 'baseline'
candidate = args.visual_controls / 'candidate'

def compare(name, visual):
    command = ['node', 'scripts/benchmarks/geometry-compare.mjs', '--protocol', str(args.native_controls / 'protocol.json'), '--references', '.flow/evidence/fn19/references.json', '--baseline', str(args.native_controls / 'baseline'), '--candidate', str(args.native_controls / 'candidate'), '--baseline-visual', str(base), '--candidate-visual', str(visual), '--output', str(args.output / name)]
    result = subprocess.run(command, capture_output=True, text=True, timeout=600)
    report = json.loads((args.output / name / 'comparison.json').read_text())
    return {'command': command, 'exit_code': result.returncode, 'status': report['status'], 'reasons': report['reasons'], 'views': report.get('views', []), 'metrics_changed': [key for key, value in report.get('metrics', {}).items() if value['changed']], 'report': str(args.output / name / 'comparison.json')}

unchanged = compare('unchanged', candidate)
assert unchanged['exit_code'] == 0, unchanged
assert not unchanged['metrics_changed']
assert unchanged['views'] and not any(view['image_changed'] or view['geometry_changed'] or view['gaps_changed'] for view in unchanged['views']), 'unchanged visual replay drift; inspect preserved comparison'
fixture = args.output / 'substituted-image-fixture'
shutil.copytree(candidate, fixture)
captures_path = fixture / 'captures.json'
captures = json.loads(captures_path.read_text())
first = next(c for c in captures if c['view'] == 'whole' and c['azimuth_deg'] == 0)
second = next(c for c in captures if c['case_id'] == first['case_id'] and c['view'] == 'whole' and c['azimuth_deg'] == 90)
a = next(a for a in first['artifacts'] if a['path'].endswith('/beauty-128.png'))
b = next(a for a in second['artifacts'] if a['path'].endswith('/beauty-128.png'))
assert a['sha256'] != b['sha256']
target = fixture / a['path']
target.unlink()
shutil.copyfile(candidate / b['path'], target)
a['sha256'] = hashlib.sha256(target.read_bytes()).hexdigest()
a['bytes'] = target.stat().st_size
captures_path.unlink()
captures_path.write_text(json.dumps(captures, indent=2) + '\n')
changed = compare('changed-image', fixture)
assert changed['exit_code'] == 0, changed
assert next(v for v in changed['views'] if v['id'] == '/'.join([first['case_id'], 'whole', '0']))['image_changed']
(args.output / 'controls.json').write_text(json.dumps({'schema_version': 1, 'scope': 'small artificial six-oak cohort, not mature evidence', 'unchanged': unchanged, 'changed_image': changed, 'mutation': 'Existing second-azimuth PNG substituted into first-azimuth artifact with corrected byte/hash receipt; synthetic control, not a generated revision.', 'independent_assessment': 'unassessed'}, indent=2) + '\n')
print('real visual replay and substituted-image control passed')
