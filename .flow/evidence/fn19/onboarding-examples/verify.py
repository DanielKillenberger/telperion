#!/usr/bin/env python3
"""Bounded packet examples: actual native admission, no specimen generation."""
import copy
import importlib.util
import json
from pathlib import Path
import subprocess
import sys
import tempfile

sys.dont_write_bytecode = True

ROOT = Path(__file__).resolve().parents[4]
EXAMPLES = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location('benchmark', ROOT / 'crates/telperion-core/examples/geometry_benchmark/runner.py')
r = importlib.util.module_from_spec(spec)
spec.loader.exec_module(r)
binary = str(ROOT / 'target/release/examples/geometry_benchmark')
p = r.read(r.FROZEN)
refs = r.read(ROOT / '.flow/evidence/fn19/references.json')
later = r.read(EXAMPLES / 'later-protocol.json')
later_refs = r.read(EXAMPLES / 'later-references.json')
identity = r.read(EXAMPLES / 'frozen-identity.json')
checks = []


def checked(name, condition):
    if not condition:
        raise AssertionError(name)
    checks.append(name)


def rejected(name, candidate, message):
    try:
        r.manifest(candidate, later_refs)
    except ValueError as error:
        checked(name, message in str(error))
    else:
        raise AssertionError(name + ': incorrectly accepted')


r.manifest(p, refs)
r.manifest(later, later_refs)
checked('old protocol bytes preserved', r.sha(r.FROZEN.read_bytes()) == identity['protocol_sha256'])
checked('old references bytes preserved', r.sha((ROOT / identity['references_path']).read_bytes()) == identity['references_sha256'])
checked('old cohort preserved in later version', later['cases'][:12] == identity['cases'] == p['cases'] and later['species'][:2] == p['species'])
checked('later cohort adds one species and six cases', len(later['species']) == 3 and len(later['cases']) == 18 and later['benchmark_id'] != p['benchmark_id'])
admissions = []
for s in later['species']:
    row = r.admission(s, later, later_refs, binary)
    r.check('admission', row, p)
    admissions.append(row)
checked('oak and spruce runnable, botanical assessment unassessed', all(a['status'] == 'admitted' and a['implemented'] and a['botanical_validation'] == 'unassessed' for a in admissions[:2]))
checked('pine unsupported, never admitted with implemented=false', admissions[2]['status'] == 'unsupported-anatomy' and not admissions[2]['implemented'] and 'paired-needle-fascicle' in admissions[2]['missing_capabilities'])
for original in p['species']:
    packet = r.read(EXAMPLES / original['id'] / 'species.json')
    r.check('species', packet, p)
    packet_refs = r.read(EXAMPLES / original['id'] / 'references.json')
    independent = copy.deepcopy(p)
    independent.update(benchmark_id='packet-' + original['id'], species=[packet], cases=[c for c in p['cases'] if c['species_id'] == original['id']])
    r.manifest(independent, packet_refs)
    result = r.admission(packet, independent, packet_refs, binary)
    checked(original['id'] + ' independent profile admitted', result['status'] == 'admitted' and result['implemented'])
    specimens = r.read(EXAMPLES / original['id'] / 'specimens.json')
    checked(original['id'] + ' specimen manifest preserved', specimens['cases'] == [c for c in p['cases'] if c['species_id'] == original['id']])
    # The native parameter parser consumes the complete family, not the manifest wrapper.
    with tempfile.TemporaryDirectory() as directory:
        target = Path(directory) / 'parameters.json'
        target.write_text(json.dumps(packet['parameters']))
        subprocess.run([binary, '--validate-parameters', str(target)], check=True, capture_output=True)
    checked(original['id'] + ' full native parameters valid', True)
q = copy.deepcopy(later); q['species'].append(copy.deepcopy(q['species'][0]))
rejected('duplicate species ID rejected', q, 'duplicate-identity')
q = copy.deepcopy(later); q['species'][2]['reference_ids'] = ['absent-reference']
rejected('missing reference ID rejected', q, 'unresolved reference')
q = copy.deepcopy(later); q['species'][2]['holdout_seeds'][0] = q['species'][2]['fixed_seeds'][0]
rejected('reused holdout rejected', q, 'seed sets')
s = copy.deepcopy(later['species'][0]); s['profile_sha256'] = '0' * 64
checked('missing evidence hash rejected', r.admission(s, later, later_refs, binary)['status'] == 'missing-evidence')
s = copy.deepcopy(later['species'][0]); s['required_capabilities'].append('paired-needle-fascicle')
checked('unsupported anatomy on implemented species rejected', r.admission(s, later, later_refs, binary)['status'] == 'unsupported-anatomy')
# These are coordinator examples, not additional runner capabilities or a scheduler.
workflow = r.read(EXAMPLES / 'workflow-examples.json')
for example in workflow:
    if example['kind'] == 'shared-write':
        claims = example['claims']
        conflict = any(a['path'] == b['path'] and a['owner'] != b['owner'] for i, a in enumerate(claims) for b in claims[i + 1:])
        observed = 'ownership-conflict' if conflict else 'ready-for-handoff'
    elif example['kind'] == 'resource-window':
        observed = 'resource-conflict' if len(set(example['owners'])) > 1 else 'ready-for-handoff'
    elif example['kind'] == 'expert-feedback':
        observed = 'unassessed' if example['feedback'] is None else 'feedback-present'
    else:
        raise AssertionError('unknown workflow example')
    checked(example['name'], observed == example['expected'])
print(json.dumps({'checks': checks, 'admissions': admissions, 'generation': 'not-run', 'expert_status': 'unassessed'}, indent=2))
