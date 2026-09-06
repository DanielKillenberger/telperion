#!/usr/bin/env python3
"""Receipt and process boundary for the native geometry benchmark (stdlib only)."""
import argparse
import copy
import datetime
import hashlib
import json
import math
import os
from pathlib import Path
import platform
import re
import shutil
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[4]
FROZEN = ROOT / '.flow/evidence/fn19/protocol.json'


def read(path):
    return json.loads(Path(path).read_bytes(), parse_constant=lambda v: (_ for _ in ()).throw(ValueError('nonfinite JSON: ' + v)))


def encoded(value):
    return (json.dumps(value, sort_keys=True, indent=2, allow_nan=False) + '\n').encode()


def sha(data):
    return hashlib.sha256(data).hexdigest()


def save(path, value):
    with Path(path).open('xb') as f:
        f.write(encoded(value))
        f.flush()
        os.fsync(f.fileno())


def contained(root, name):
    path = Path(name)
    if path.is_absolute() or '..' in path.parts:
        raise ValueError('path-escape: ' + str(name))
    result = (root / path).resolve()
    if not result.is_relative_to(root.resolve()):
        raise ValueError('path-escape: ' + str(name))
    return result


def validate(value, schema, defs):
    """Validate the vocabulary used by frozen schema.$defs; reject unknown keywords."""
    supported = {'$ref', 'type', 'properties', 'required', 'additionalProperties', 'items', 'minItems', 'maxItems', 'minimum', 'maximum', 'minLength', 'pattern', 'enum', 'const', 'anyOf'}
    if set(schema) - supported:
        raise ValueError('unsupported schema vocabulary: ' + str(set(schema) - supported))
    if '$ref' in schema:
        ref = schema['$ref']
        if not ref.startswith('#/$defs/'):
            raise ValueError('unsupported schema reference')
        return validate(value, defs[ref.removeprefix('#/$defs/')], defs)
    if 'anyOf' in schema:
        for branch in schema['anyOf']:
            try:
                validate(value, branch, defs)
                return
            except (ValueError, KeyError):
                pass
        raise ValueError('no schema alternative matches')
    types = {'object': lambda x: isinstance(x, dict), 'array': lambda x: isinstance(x, list), 'string': lambda x: isinstance(x, str), 'null': lambda x: x is None, 'boolean': lambda x: isinstance(x, bool), 'number': lambda x: type(x) in (int, float) and math.isfinite(x), 'integer': lambda x: type(x) is int}
    if 'type' in schema:
        allowed = schema['type'] if isinstance(schema['type'], list) else [schema['type']]
        if not any(types[t](value) for t in allowed):
            raise ValueError('schema type mismatch')
    if 'const' in schema and value != schema['const'] or 'enum' in schema and value not in schema['enum']:
        raise ValueError('schema enum/constant mismatch')
    if isinstance(value, dict):
        if set(schema.get('required', [])) - value.keys():
            raise ValueError('missing required fields')
        properties = schema.get('properties', {})
        for key, child in value.items():
            sub = properties.get(key, schema.get('additionalProperties', {}))
            if sub is False:
                raise ValueError('unknown property: ' + key)
            if isinstance(sub, dict):
                validate(child, sub, defs)
    if isinstance(value, list):
        if len(value) < schema.get('minItems', 0) or len(value) > schema.get('maxItems', math.inf):
            raise ValueError('array size mismatch')
        for child in value:
            validate(child, schema.get('items', {}), defs)
    if isinstance(value, str):
        if len(value) < schema.get('minLength', 0) or 'pattern' in schema and not re.search(schema['pattern'], value):
            raise ValueError('string constraint mismatch')
    if type(value) in (int, float):
        if not math.isfinite(value) or value < schema.get('minimum', -math.inf) or value > schema.get('maximum', math.inf):
            raise ValueError('numeric constraint mismatch')


def check(kind, value, protocol):
    defs = protocol['schema']['$defs']
    validate(value, defs[kind], defs)
    if kind == 'metric':
        success = value['status'] in ('measured', 'estimated')
        if success != (value['value'] is not None) or success != (value['reason'] is None):
            raise ValueError('metric value/status/reason mismatch')
        if not success and not value['reason']:
            raise ValueError('missing metric reason')


def manifest(protocol, references):
    frozen = read(FROZEN)
    for key in ('schema_version', 'benchmark_id', 'reference_version', 'species', 'cases', 'metrics', 'schema', 'comparison_equal_fields'):
        if key not in protocol:
            raise ValueError('malformed protocol: missing ' + key)
    if protocol['schema_version'] != 1 or protocol['metrics'] != frozen['metrics'] or protocol['schema'] != frozen['schema'] or protocol['comparison_equal_fields'] != frozen['comparison_equal_fields']:
        raise ValueError('unsupported protocol definitions/schema')
    if protocol['benchmark_id'] == frozen['benchmark_id'] and protocol != frozen:
        raise ValueError('frozen version changed; create a new benchmark version')
    if protocol['reference_version'] != references['reference_version']:
        raise ValueError('reference version mismatch')
    species = {}; taxa = set(); cases = set(); observed = {}
    refs = {r['id']: r for r in references['references']}
    if len(refs) != len(references['references']):
        raise ValueError('invalid-manifest: duplicate reference identity')
    sources = {s['id'] for s in references['sources']}
    for ref in references['references']:
        check('reference', ref, protocol)
        if ref['source_id'] not in sources:
            raise ValueError('invalid-manifest: unresolved attribution source')
    for s in protocol['species']:
        check('species', s, protocol)
        taxon = tuple(' '.join((s[k] or '').lower().split()) for k in ('scientific_name', 'taxon_rank', 'cultivar'))
        if s['id'] in species or taxon in taxa:
            raise ValueError('duplicate-identity')
        species[s['id']] = s; taxa.add(taxon)
        seeds = s['fixed_seeds'] + s['holdout_seeds']
        if min(len(s['fixed_seeds']), len(s['holdout_seeds'])) < 3 or len(seeds) != len(set(seeds)):
            raise ValueError('invalid-manifest: seed sets')
        if any(r not in refs or refs[r]['species_id'] != s['id'] for r in s['reference_ids']):
            raise ValueError('invalid-manifest: unresolved reference')
        observed[s['id']] = set()
    if not protocol['cases']:
        raise ValueError('invalid-manifest: empty cases')
    for case in protocol['cases']:
        check('case', case, protocol)
        sid = case['species_id']
        if case['id'] in cases or sid not in species or case['parameter_species_id'] != sid:
            raise ValueError('invalid-manifest: duplicate/unresolved case')
        role = 'fixed_seeds' if case['seed_role'] == 'regression' else 'holdout_seeds'
        if case['seed'] not in species[sid][role] or case['seed'] in observed[sid]:
            raise ValueError('invalid-manifest: case seed')
        observed[sid].add(case['seed']); cases.add(case['id'])
    for sid, s in species.items():
        if observed[sid] != set(s['fixed_seeds'] + s['holdout_seeds']):
            raise ValueError('invalid-manifest: absent declared seeds')
    return species


def command(*args):
    return subprocess.check_output(args, cwd=ROOT, text=True).strip()


def identity(paths):
    files = []
    for path in sorted(set(paths)):
        actual = contained(ROOT, path)
        files.append({'path': path, 'sha256': sha(actual.read_bytes())})
    if not files:
        raise ValueError('empty source identity')
    return {'commit': command('git', 'rev-parse', 'HEAD'), 'dirty': bool(command('git', 'status', '--porcelain', '--', *paths)), 'files': files, 'sha256': sha(''.join(f"{f['path']}\0{f['sha256']}\n" for f in files).encode())}


def artifact(root, path, media='application/json'):
    data = contained(root, path).read_bytes()
    if not data:
        raise ValueError('empty artifact')
    return {'path': path, 'sha256': sha(data), 'bytes': len(data), 'media_type': media}


def support(binary, species):
    return json.loads(subprocess.check_output([binary, '--support', species['preset']], text=True))


def admission(s, protocol, refs, binary):
    result = {'species_id': s['id'], 'status': 'admitted', 'reasons': [], 'missing_capabilities': [], 'evidence_ids': s['reference_ids'], 'implemented': False, 'botanical_validation': 'unassessed'}
    try:
        profile = contained(ROOT, s['profile_path'])
        if sha(profile.read_bytes()) != s['profile_sha256']:
            raise ValueError('profile hash mismatch')
        profiles = read(profile)['profiles']
        if not any(p['id'] == s['profile_id'] and ' '.join(p['scientific_name'].lower().split()) == ' '.join(s['scientific_name'].lower().split()) for p in profiles):
            raise ValueError('profile identity mismatch')
        if not any(r['id'] in s['reference_ids'] and r['kind'] == 'real' and r['matching'] != 'unavailable' and r['attribution'] for r in refs['references']):
            raise ValueError('no usable attributed real reference')
    except (ValueError, OSError, KeyError) as e:
        result.update(status='missing-evidence', reasons=[str(e)])
        return result
    found = support(binary, s)
    result['missing_capabilities'] = sorted(set(s['required_capabilities']) - set(found['capabilities']))
    result['implemented'] = found['implemented'] and found['profile_id'] == s['profile_id'] == s['id']
    if not result['implemented'] or result['missing_capabilities']:
        result.update(status='unsupported-anatomy', reasons=['generator identity or required anatomy unavailable'])
    return result


def event(run, case, event='pending', reason=None, metrics=None, costs=None, artifacts=None):
    return {'schema_version': 1, 'run_id': run['run_id'], 'case_id': case['id'], 'event': event, 'numeric_status': 'unassessed', 'metrics': metrics or {}, 'costs': costs or [], 'artifacts': artifacts or [], 'reason': reason}


def run(args):
    protocol = read(args.protocol); refs = read(args.references)
    species = manifest(protocol, refs)
    output = Path(args.output).absolute()
    output.parent.mkdir(parents=True, exist_ok=True)
    output = output.parent.resolve() / output.name
    output.mkdir(exist_ok=False)
    binary = str(Path(args.binary).resolve())
    subprocess.run(["cargo", "build", "--release", "-p", "telperion-core", "--example", "geometry_benchmark"], cwd=ROOT, check=True, timeout=600)
    cases = protocol['cases']; resolved = {}
    for case in cases:
        p = copy.deepcopy(species[case['species_id']]['parameters']); p['skeleton']['seed'] = case['seed']; resolved[case['id']] = p
    paths = command('git', 'ls-files', '--cached', '--others', '--exclude-standard', '--', 'crates/telperion-core/src', 'crates/telperion-core/Cargo.toml', 'Cargo.toml', 'Cargo.lock').splitlines()
    toolpaths = command('git', 'ls-files', '--cached', '--others', '--exclude-standard', '--', 'crates/telperion-core/examples/geometry_benchmark.rs', 'crates/telperion-core/examples/geometry_benchmark', 'crates/telperion-core/examples/species_metrics', 'crates/telperion-core/tests/geometry_benchmark.rs', 'Cargo.lock').splitlines()
    toolpaths = [p for p in toolpaths if '__pycache__' not in p and not p.endswith('.pyc')]
    shutil.copyfile(binary, output / 'native-binary')
    save(output / 'manifest.json', cases)
    run = {'schema_version': 1, 'benchmark_id': protocol['benchmark_id'], 'run_id': output.name, 'created_at': datetime.datetime.now(datetime.timezone.utc).isoformat(), 'protocol_sha256': sha(Path(args.protocol).read_bytes()), 'references_sha256': sha(Path(args.references).read_bytes()), 'source': identity(paths), 'tool': identity(toolpaths), 'binaries': [artifact(output, 'native-binary', 'application/octet-stream')], 'manifest_sha256': sha((output / 'manifest.json').read_bytes()), 'conditions_sha256': sha(Path(args.conditions).read_bytes()) if args.conditions else None, 'parameters': resolved, 'cases': cases, 'machine': {'os': platform.platform(), 'python': sys.version, 'rustc': command('rustc', '--version'), 'lifecycle': 'cold subprocess per case', 'samples': 1, 'exclusive_window': False}, 'status': 'started', 'partial': True, 'reason': 'numeric run in progress; missing terminals imply interruption'}
    check('run', run, protocol); save(output / 'run-started.json', run)
    admissions = {sid: admission(s, protocol, refs, binary) for sid, s in species.items()}
    save(output / 'admission.json', list(admissions.values()))
    all_pass = True
    with (output / 'measurements.jsonl').open('xb') as stream:
        def emit(row):
            check('case_result', row, protocol)
            stream.write(json.dumps(row, allow_nan=False).encode() + b'\n'); stream.flush(); os.fsync(stream.fileno())
        for case in cases:
            emit(event(run, case))
        for index, case in enumerate(cases):
            emit(event(run, case, 'started'))
            folder = output / 'cases' / str(index); folder.mkdir(parents=True)
            save(folder / 'parameters.json', resolved[case['id']])
            row = event(run, case)
            try:
                subprocess.run([binary, '--validate-parameters', str(folder / 'parameters.json')], capture_output=True, text=True, check=True, timeout=30)
                if admissions[case['species_id']]['status'] != 'admitted':
                    row = event(run, case, 'unavailable', '; '.join(admissions[case['species_id']]['reasons']))
                else:
                    result = subprocess.run([binary, '--worker', str(folder / 'parameters.json')], capture_output=True, text=True, timeout=300, check=True)
                    data = json.loads(result.stdout)
                    for m in data['metrics'].values():
                        check('metric', m, protocol)
                    save(folder / 'native.json', data)
                    row = event(run, case, 'completed', metrics=data['metrics'], costs=data['costs'], artifacts=[artifact(output, f'cases/{index}/native.json')])
                    row['numeric_status'] = 'pass' if all(m['status'] in ('measured', 'estimated') for m in data['metrics'].values()) else 'unassessed'
                    if row['numeric_status'] != 'pass':
                        row['reason'] = 'required distribution unavailable'
            except subprocess.TimeoutExpired:
                row = event(run, case, 'failed', 'timeout: native process exceeded case bound'); row['numeric_status'] = 'fail'
            except (subprocess.CalledProcessError, ValueError, OSError) as e:
                row = event(run, case, 'failed', getattr(e, 'stderr', None) or str(e)); row['numeric_status'] = 'fail'
            all_pass &= row['numeric_status'] == 'pass'; emit(row)
    run.update(status='complete' if all_pass else 'failed', partial=not all_pass, reason=None if all_pass else 'required numeric cases failed or unavailable; see all terminal events')
    check('run', run, protocol); save(output / 'run.json', run)
    (output / 'REPORT.md').write_text(f"Numeric collection: {run['status']}. {len(cases)} declared cases retained.\n\nOperational axes are estimated; DBH ambiguity stays in native.json legacy_metrics. Costs are observations from a nonexclusive window. Capture and independent botanical assessment are unassessed.\n")
    return 0 if all_pass else 1


def load_run(directory, protocol, protocol_sha256=None):
    root = Path(directory).resolve()
    run = read(root / ('run.json' if (root / 'run.json').exists() else 'run-started.json'))
    check('run', run, protocol)
    if protocol_sha256 is not None and (run['protocol_sha256'] != protocol_sha256 or run['cases'] != protocol['cases']):
        raise ValueError('run does not match supplied protocol/cohort')
    for binary in run['binaries']:
        if artifact(root, binary['path'], binary['media_type']) != binary:
            raise ValueError('binary identity mismatch')
    for domain in ('source', 'tool'):
        identity = run[domain]
        entries = identity['files']
        names = [f['path'] for f in entries]
        if names != sorted(set(names)) or sha(''.join(f"{f['path']}\0{f['sha256']}\n" for f in entries).encode()) != identity['sha256']:
            raise ValueError('source/tool identity digest mismatch')
    if sha((root / 'manifest.json').read_bytes()) != run['manifest_sha256'] or read(root / 'manifest.json') != run['cases']:
        raise ValueError('manifest identity mismatch')
    rows = {}; terminals = set()
    for line in (root / 'measurements.jsonl').read_bytes().splitlines(keepends=True):
        if not line.endswith(b'\n'):
            break
        row = json.loads(line); check('case_result', row, protocol)
        cid = row['case_id']
        if row['run_id'] != run['run_id'] or cid not in {c['id'] for c in run['cases']}:
            raise ValueError('unexpected case/run identity')
        if cid in terminals:
            raise ValueError('duplicated terminal case: ' + cid)
        if row['event'] in ('completed', 'failed', 'unavailable'):
            terminals.add(cid)
        for m in row['metrics'].values():
            check('metric', m, protocol)
        for a in row['artifacts']:
            if artifact(root, a['path'], a['media_type']) != a:
                raise ValueError('artifact identity mismatch')
            if a['path'].endswith('/native.json') and read(contained(root, a['path']))['metrics'] != row['metrics']:
                raise ValueError('native artifact/metric mismatch')
        rows[cid] = row
    return run, rows


def compare_runs(baseline, candidate, arows, brows, protocol):
    reasons = []; missing = []; failed = []; measured = {}
    for key in protocol['comparison_equal_fields']:
        if baseline.get(key) != candidate.get(key):
            reasons.append('condition mismatch: ' + key)
    if baseline['tool']['sha256'] != candidate['tool']['sha256']:
        reasons.append('measurement tool mismatch; replay both sources with identical tool')
    cases = [c['id'] for c in baseline['cases']]
    candidate_ids = [c['id'] for c in candidate['cases']]
    if len(cases) != len(set(cases)) or len(candidate_ids) != len(set(candidate_ids)):
        reasons.append('duplicate case manifest')
    if candidate_ids != cases:
        reasons.append('ordered case manifest mismatch')
    same_source = baseline['source']['sha256'] == candidate['source']['sha256']
    for cid in dict.fromkeys(cases + candidate_ids):
        a = arows.get(cid); b = brows.get(cid)
        if a is None or b is None or cid not in cases or cid not in candidate_ids:
            missing.append(cid); continue
        if a['event'] != 'completed' or b['event'] != 'completed' or a['numeric_status'] != 'pass' or b['numeric_status'] != 'pass':
            failed.append(cid)
        changed = a['metrics'] != b['metrics']
        measured[cid] = {'changed': changed, 'baseline': a['metrics'], 'candidate': b['metrics'], 'baseline_disposition': a['event'], 'candidate_disposition': b['event'], 'baseline_reason': a['reason'], 'candidate_reason': b['reason']}
        if same_source and changed:
            reasons.append('unchanged-source measurement drift: ' + cid)
    if missing: reasons.append('missing required cases')
    if failed: reasons.append('failed, unavailable or interrupted cases')
    if baseline['partial'] or candidate['partial']: reasons.append('partial run')
    return {'schema_version': 1, 'baseline_run_id': baseline['run_id'], 'candidate_run_id': candidate['run_id'], 'status': 'inconclusive' if reasons else 'comparable', 'reasons': reasons, 'case_ids': cases, 'failed_case_ids': failed, 'missing_case_ids': missing, 'metrics': measured, 'biological_superiority': 'unestablished', 'independent_assessment': 'unassessed'}


def main():
    p = argparse.ArgumentParser(description='Native structural benchmark. New output directories only; every declared case retained; each process bounded to 300 seconds. No render outputs required. Python 3 standard library receipts, native Rust measurements. Costs are nonexclusive observations.')
    p.add_argument('--binary', required=True, help=argparse.SUPPRESS)
    p.add_argument('--protocol', default=str(FROZEN)); p.add_argument('--references', default=str(ROOT / '.flow/evidence/fn19/references.json'))
    p.add_argument('--conditions', help='optional frozen shared capture conditions; not consumed by numeric measurements')
    p.add_argument('--output', required=True); p.add_argument('--baseline'); p.add_argument('--candidate')
    args = p.parse_args()
    if args.baseline or args.candidate:
        if not args.baseline or not args.candidate: p.error('comparison requires both --baseline and --candidate')
        protocol = read(args.protocol)
        digest = sha(Path(args.protocol).read_bytes())
        a, ar = load_run(args.baseline, protocol, digest); b, br = load_run(args.candidate, protocol, digest)
        result = compare_runs(a, b, ar, br, protocol); check('comparison', result, protocol)
        out = Path(args.output); out.mkdir(parents=True, exist_ok=False); save(out / 'comparison.json', result)
        return 0 if result['status'] == 'comparable' else 1
    return run(args)


if __name__ == '__main__':
    try:
        sys.exit(main())
    except (ValueError, KeyError, OSError, subprocess.SubprocessError) as error:
        print('geometry_benchmark:', error, file=sys.stderr)
        sys.exit(2)
