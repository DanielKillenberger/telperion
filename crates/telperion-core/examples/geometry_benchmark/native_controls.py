#!/usr/bin/env python3
"""Run tiny native fixtures and save auditable replay controls; no mature/holdout generation."""
import argparse
import copy
import json
from pathlib import Path
import subprocess
import runner as r


def execute(binary, *args, expected=0):
    result = subprocess.run([str(binary), *map(str, args)], capture_output=True, text=True, timeout=120)
    if result.returncode != expected:
        raise AssertionError(f'expected {expected}, got {result.returncode}: {result.stderr}')
    return result


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--binary', required=True, type=Path)
    parser.add_argument('--output', required=True, type=Path)
    parser.add_argument('--receipt', required=True, type=Path)
    args = parser.parse_args(); args.output.mkdir(parents=True, exist_ok=False)
    binary = args.binary.resolve()
    protocol = r.read(r.FROZEN)
    protocol['benchmark_id'] = 'fn19-tiny-controls-v1'
    protocol['population'] = '2m artificial smoke fixture, not a botanical specimen cohort'
    species = protocol['species'][:1]; protocol['species'] = species
    s = species[0]; s['fixed_seeds'] = [1, 2, 3]; s['holdout_seeds'] = [4, 5, 6]
    protocol['seed_freeze'] = {'holdouts': {s['id']: [4, 5, 6]}, 'method': 'Previously used historical seeds for mechanical controls only; no fresh holdout claim.'}
    p = s['parameters']; p['skeleton']['envelope']['height'] = 2.; p['skeleton']['step'] = 0.15
    p['skeleton']['habit'].update(scaffoldLimbs=2, subdivisions=1)
    p['skeleton']['twigs']['twig'].update(internodeLength=0.08, length=0.16)
    p['skeleton']['twigs']['laterals'] = 2; p['canopy']['maxInstances'] = 10000
    protocol['cases'] = [{'id': f'control-{seed}', 'species_id': s['id'], 'parameter_species_id': s['id'], 'seed': seed, 'seed_role': 'regression' if seed < 4 else 'holdout-at-freeze'} for seed in range(1, 7)]
    path = args.output / 'protocol.json'; r.save(path, protocol)
    for name in ('baseline', 'candidate'):
        execute(binary, '--protocol', path, '--output', args.output / name)
    execute(binary, '--protocol', path, '--baseline', args.output / 'baseline', '--candidate', args.output / 'candidate', '--output', args.output / 'comparison')
    a, ar = r.load_run(args.output / 'baseline', protocol, r.sha(path.read_bytes()))
    b, br = r.load_run(args.output / 'candidate', protocol, r.sha(path.read_bytes()))
    result = r.read(args.output / 'comparison/comparison.json')
    assert result['status'] == 'comparable'
    assert all(not m['changed'] for m in result['metrics'].values())
    drift = copy.deepcopy(br); drift['control-1']['metrics']['axes']['value']['raw_axis_count'] += 1
    drift_result = r.compare_runs(a, b, ar, drift, protocol)
    dropped = copy.deepcopy(br); del dropped['control-1']
    dropped_result = r.compare_runs(a, b, ar, dropped, protocol)
    duplicate = copy.deepcopy(b); duplicate['cases'].append(duplicate['cases'][0])
    duplicate_result = r.compare_runs(a, duplicate, ar, br, protocol)
    assert all(v['status'] == 'inconclusive' for v in (drift_result, dropped_result, duplicate_result))
    cap = copy.deepcopy(a['parameters']['control-1']); cap['skeleton']['growth']['maxNodes'] = 2
    cap_path = args.output / 'capped.json'; r.save(cap_path, cap)
    capped = execute(binary, '--worker', cap_path, expected=2)
    assert 'resource-cap' in capped.stderr
    capped_protocol = copy.deepcopy(protocol); capped_protocol['benchmark_id'] = 'fn19-tiny-capped-controls-v1'
    capped_protocol['species'][0]['parameters']['skeleton']['growth']['maxNodes'] = 2
    r.save(args.output / 'capped-protocol.json', capped_protocol)
    execute(binary, '--protocol', args.output / 'capped-protocol.json', '--output', args.output / 'capped-run', expected=1)
    capped_run, capped_rows = r.load_run(args.output / 'capped-run', capped_protocol)
    assert capped_run['partial'] and len(capped_rows) == 6
    assert all(row['event'] == 'failed' and 'resource-cap' in row['reason'] for row in capped_rows.values())
    unknown = json.loads(execute(binary, '--support', 'not-an-implemented-species').stdout)
    assert unknown == {'implemented': False, 'profile_id': None, 'capabilities': []}
    existing = execute(binary, '--protocol', path, '--output', args.output / 'baseline', expected=2)
    assert 'File exists' in existing.stderr
    receipt = {'schema_version': 1, 'purpose': 'Mechanical controls only; 2m artificial fixtures, six previously used historical seeds. No mature generation, fresh holdouts, optimization or botanical claims.', 'baseline': {'source': a['source'], 'tool': a['tool'], 'binary': a['binaries'], 'protocol_sha256': a['protocol_sha256'], 'run_id': a['run_id']}, 'candidate': {'source': b['source'], 'tool': b['tool'], 'binary': b['binaries'], 'protocol_sha256': b['protocol_sha256'], 'run_id': b['run_id']}, 'controls': {'unchanged_source_replay': result['status'], 'cases': len(result['case_ids']), 'changed_measurement': drift_result['reasons'], 'dropped_case': dropped_result['missing_case_ids'], 'duplicate_case': duplicate_result['reasons'], 'capped_generation': capped.stderr.strip(), 'capped_cases_retained': len(capped_rows), 'unknown_generator': unknown, 'no_overwrite': existing.returncode}, 'costs': {'generation': 'nonexclusive single-sample observation, excluded from equality', 'capture-preparation': 'unavailable', 'cpu-rss': 'unavailable', 'wasm-capacity': 'unavailable', 'gpu-allocation': 'unavailable', 'gpu-time': 'unavailable'}, 'reproduce': 'python3 -B crates/telperion-core/examples/geometry_benchmark/native_controls.py --binary target/release/examples/geometry_benchmark --output NEW_DIR --receipt NEW_FILE', 'artifacts_note': 'Raw tiny run directories are local verification artifacts. Reproduce creates new directories; never overwrites a prior run.'}
    r.save(args.receipt, receipt)
    print(json.dumps(receipt['controls'], indent=2))


if __name__ == '__main__':
    main()
