import json
from pathlib import Path
from statistics import median
p = Path(__file__).resolve().parent
names = ['validation_paths_allocation', 'ranking_samples', 'ranking_sort', 'emission_samples_frames', 'vertices_coords', 'indices_caps', 'normal_buffer_resize', 'normal_accumulation', 'normal_normalization', 'run_table', 'final_bounds']
rows = {mode: [json.loads(line) for line in (p / f'{mode}.jsonl').read_text().splitlines() if json.loads(line)['event'] == 'sample'] for mode in ['control', 'instrumented']}
assert all(len(r) == 16 for r in rows.values())
results = []
for species in ['oregon-white-oak', 'norway-spruce']:
    for seed in [1, 7]:
        groups = {mode: [r for r in rr if (r['preset'], r['seed']) == (species, seed)] for mode, rr in rows.items()}
        assert all([r['sample'] for r in group] == [0,1,2,3] for group in groups.values())
        assert len({r['all_surface_fnv1a64'] for g in groups.values() for r in g}) == 1
        control, instrumented = groups['control'], groups['instrumented']
        warm = lambda g, key: median(r[key] for r in g[1:])
        c, i = warm(control, 'wood_ms'), warm(instrumented, 'wood_ms')
        stages = {name: median(r['stages_ms'][n] for r in instrumented[1:]) for n,name in enumerate(names)}
        assert all(0 < sum(r['stages_ms']) <= r['wood_ms'] for r in instrumented)
        results.append(dict(preset=species, seed=seed, control_first_ms=control[0]['wood_ms'], instrumented_first_ms=instrumented[0]['wood_ms'], control_warm_ms=c, instrumented_warm_ms=i, observed_delta_ms=i-c, observed_delta_percent=100*(i-c)/c, radius_control_warm_ms=warm(control,'radius_ms'), radius_instrumented_warm_ms=warm(instrumented,'radius_ms'), stages_warm_ms=stages, stage_total_warm_ms=median(sum(r['stages_ms']) for r in instrumented[1:]), unassigned_warm_ms=median(r['wood_ms']-sum(r['stages_ms']) for r in instrumented[1:]), all_surface_fnv1a64=control[0]['all_surface_fnv1a64'], vertices=control[0]['vertices'], triangles=control[0]['triangles'], runs=control[0]['runs'], dropped=control[0]['dropped'], surface_capacity_bytes=control[0]['surface_capacity_bytes'], radius_bytes=control[0]['radius_bytes']))
(p / 'summary.json').write_text(json.dumps(results, indent=2)+'\n')
print('PASS: 32 samples, 4 fixtures, full surface hash parity; stage accounting positive and bounded.')
for r in results:
 print(r['preset'],r['seed'], round(r['control_warm_ms'],2),round(r['instrumented_warm_ms'],2),round(r['observed_delta_percent'],2),r['stages_warm_ms'])
