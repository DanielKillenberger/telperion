"""Small contract controls; never generates a mature specimen."""
import copy
import json
import tempfile
from pathlib import Path
import unittest
from unittest.mock import patch
import runner as r


class Controls(unittest.TestCase):
    def setUp(self):
        self.p = r.read(r.FROZEN)
        self.refs = r.read(r.ROOT / '.flow/evidence/fn19/references.json')

    def new_protocol(self):
        p = copy.deepcopy(self.p); p['benchmark_id'] = 'fn19-control-new-version'
        return p

    def test_manifest_malformed_duplicate_and_missing_cases(self):
        self.assertEqual(len(r.manifest(self.p, self.refs)), 2)
        for mutation in ('schema', 'duplicate-case', 'duplicate-species', 'duplicate-taxon', 'absent-case', 'unknown-species', 'seed-type', 'parameter-species'):
            p = self.new_protocol()
            if mutation == 'schema': p.pop('schema')
            if mutation == 'duplicate-case': p['cases'].append(p['cases'][0])
            if mutation == 'duplicate-species': p['species'].append(p['species'][0])
            if mutation == 'duplicate-taxon': p['species'][1]['scientific_name'] = '  quercus   GARRYANA '
            if mutation == 'absent-case': p['cases'].pop()
            if mutation == 'unknown-species': p['cases'][0]['species_id'] = 'missing'
            if mutation == 'seed-type': p['cases'][0]['seed'] = 1.5
            if mutation == 'parameter-species': p['cases'][0]['parameter_species_id'] = 'missing'
            with self.subTest(mutation=mutation), self.assertRaises(ValueError): r.manifest(p, self.refs)
        p = copy.deepcopy(self.p); p['cases'].pop()
        with self.assertRaisesRegex(ValueError, 'frozen version changed'): r.manifest(p, self.refs)

    def test_new_inventory_is_not_implemented_and_old_version_unchanged(self):
        original = r.encoded(self.p); p = self.new_protocol(); refs = copy.deepcopy(self.refs)
        extra = copy.deepcopy(p['species'][0]); extra.update(id='fixture-taxon', scientific_name='Fixture taxon', profile_id='fixture-taxon', preset='unknown-generator')
        extra['reference_ids'] = ['fixture-real']; p['species'].append(extra)
        ref = copy.deepcopy(refs['references'][0]); ref.update(id='fixture-real', species_id=extra['id']); refs['references'].append(ref)
        for seed, role in [(n, 'regression') for n in extra['fixed_seeds']] + [(n, 'holdout-at-freeze') for n in extra['holdout_seeds']]:
            p['cases'].append({'id': f'fixture-{seed}', 'species_id': extra['id'], 'parameter_species_id': extra['id'], 'seed': seed, 'seed_role': role})
        self.assertEqual(len(r.manifest(p, refs)), 3)
        self.assertEqual(r.encoded(self.p), original)
        # Evidence failures remain ahead of implementation support in admission.
        result = r.admission(extra, p, refs, 'unused')
        self.assertEqual(result['status'], 'missing-evidence'); self.assertFalse(result['implemented'])
        known = copy.deepcopy(p['species'][0]); known['preset'] = 'unknown-generator'
        with patch.object(r, 'support', return_value={'implemented': False, 'profile_id': None, 'capabilities': []}):
            result = r.admission(known, p, refs, 'unused')
        self.assertEqual(result['status'], 'unsupported-anatomy'); self.assertFalse(result['implemented'])
        self.assertEqual(result['botanical_validation'], 'unassessed')

    def pair(self):
        digest = 'a' * 64
        identity = {'commit': 'fixture', 'dirty': False, 'files': [{'path': 'fixture.rs', 'sha256': digest}], 'sha256': r.sha(f'fixture.rs\0{digest}\n'.encode())}
        cases = self.p['cases'][:2]
        run = {'schema_version': 1, 'benchmark_id': self.p['benchmark_id'], 'run_id': 'baseline', 'created_at': 'fixture', 'protocol_sha256': digest, 'references_sha256': digest, 'source': identity, 'tool': identity, 'binaries': [], 'manifest_sha256': r.sha(r.encoded(cases)), 'conditions_sha256': None, 'parameters': {}, 'cases': cases, 'machine': {}, 'status': 'complete', 'partial': False, 'reason': None}
        rows = {}
        for case in cases:
            row = r.event(run, case, 'completed'); row['numeric_status'] = 'pass'
            row['metrics'] = {'fixture': {'status': 'measured', 'value': 1., 'unit': 'm', 'definition': 'axis-length-v1', 'evidence_ids': [], 'reason': None, 'support': {'included': 1, 'excluded': 0, 'flags': []}}}
            rows[case['id']] = row
        candidate = copy.deepcopy(run); candidate['run_id'] = 'candidate'
        return run, candidate, rows, copy.deepcopy(rows)

    def test_replay_changed_measurements_dropped_duplicates_and_changed_source(self):
        a, b, ar, br = self.pair()
        self.assertEqual(r.compare_runs(a, b, ar, br, self.p)['status'], 'comparable')
        cid = a['cases'][0]['id']; br[cid]['metrics']['fixture']['value'] = 2.
        self.assertIn('unchanged-source measurement drift: ' + cid, r.compare_runs(a, b, ar, br, self.p)['reasons'])
        b['source'] = copy.deepcopy(b['source']); b['source']['sha256'] = 'b' * 64
        self.assertEqual(r.compare_runs(a, b, ar, br, self.p)['status'], 'comparable')
        del br[cid]
        self.assertEqual(r.compare_runs(a, b, ar, br, self.p)['missing_case_ids'], [cid])
        b['cases'].append(b['cases'][0])
        self.assertIn('duplicate case manifest', r.compare_runs(a, b, ar, br, self.p)['reasons'])

    def test_failure_dispositions_and_condition_tool_mismatch(self):
        for disposition in ('pending', 'started', 'failed', 'unavailable'):
            a, b, ar, br = self.pair(); cid = a['cases'][0]['id']; br[cid]['event'] = disposition
            result = r.compare_runs(a, b, ar, br, self.p)
            self.assertEqual(result['failed_case_ids'], [cid]); self.assertEqual(result['status'], 'inconclusive')
        for key in self.p['comparison_equal_fields']:
            a, b, ar, br = self.pair(); b[key] = None
            if a[key] is None: b[key] = 'b' * 64
            if key == 'cases': b[key] = b[key] or []
            self.assertIn('condition mismatch: ' + key, r.compare_runs(a, b, ar, br, self.p)['reasons'])
        a, b, ar, br = self.pair(); b['tool'] = copy.deepcopy(b['tool']); b['tool']['sha256'] = 'b' * 64
        self.assertEqual(r.compare_runs(a, b, ar, br, self.p)['status'], 'inconclusive')

    def test_interruption_duplicate_terminals_and_no_overwrite(self):
        a, _, ar, _ = self.pair()
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp); r.save(root / 'run-started.json', a); r.save(root / 'manifest.json', a['cases'])
            first, second = a['cases']; first_row = ar[first['id']]
            lines = [r.event(a, first), r.event(a, second), first_row, r.event(a, second, 'started')]
            events = b''.join(json.dumps(v).encode() + b'\n' for v in lines)
            (root / 'measurements.jsonl').write_bytes(events + b'{"event":')
            _, rows = r.load_run(root, self.p)
            self.assertEqual(rows[second['id']]['event'], 'started')
            with self.assertRaises(FileExistsError): r.save(root / 'manifest.json', [])
            (root / 'measurements.jsonl').write_bytes(events + json.dumps(first_row).encode() + b'\n')
            with self.assertRaisesRegex(ValueError, 'duplicated terminal'): r.load_run(root, self.p)

    def test_schema_nonfinite_missing_values_and_artifact_escape(self):
        a, _, rows, _ = self.pair(); m = next(iter(rows.values()))['metrics']['fixture']
        r.check('metric', m, self.p)
        for change in ({'value': None}, {'value': float('nan')}, {'status': 'unavailable', 'value': None, 'reason': ''}):
            bad = dict(m, **change)
            with self.assertRaises(ValueError): r.check('metric', bad, self.p)
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp); (root / 'escape').symlink_to('/etc')
            for path in ('../outside', '/etc/passwd', 'escape/passwd'):
                with self.assertRaises(ValueError): r.contained(root, path)


if __name__ == '__main__':
    unittest.main()
