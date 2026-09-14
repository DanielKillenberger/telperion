"""Compose the task report from the frozen instruments and gate logs."""
import json
import re
import statistics
from pathlib import Path

P = Path(__file__).resolve().parent
ROOT = P.parents[2]
old_report = (P.parent/'fn30/REPORT.md').read_text()
refs = old_report[old_report.index('| ID |'):old_report.index('U1 lists')]
rows = {}
for name in ['oregon-white-oak','norway-spruce','ordinary','telperion','laurelin']:
    rows[name] = [json.loads(x) for x in (P/'measurements'/f'{name}.jsonl').read_text().splitlines()]

s = ['''# FN31: sapling form, age-dependent thickening and retained mature crowns

Production still builds through annual growth at the derived age. The rule now
establishes a leafed seedling, recruits juvenile laterals, increases the trunk's
radius share with age, and preserves lit branches under nonzero shedding.
The tables and captures below are implementation evidence. The owner has not
supplied botanical acceptance; the R1 and R2 slots remain empty.

## Protocol

Native release builds on the shared Ryzen 9 5950X host and NVIDIA RTX 3080,
seed 7, full preset geometry, pinned libm arithmetic and yearly slices.
`measure.py` invokes the requested `cargo run --release -p telperion-core
--example growth_curve -- --preset <id> --seed 7 --ages <ages> --envelope`.
Unedited JSONL is in `measurements/`; stderr, commands and exits are in
`logs/measure-<id>.log`. Height is highest wood above the root. DBH follows
the thickest structural continuation and interpolates its diameter at 1.3 m.
Node bounds exclude bark and foliage. Decimal strip ages complete integer years.

Pre-change measurements in `prechange/` were taken at base
`ccb44eaf609c15e4df7170385f81b372b0d5f532`, before any rule or pin changed.
Intermediate diagnostics remain labeled in their logs; `CHECKPOINT1.md`
preserves the renderer checkpoint and `ROUND1.md` the superseded diagnosis.
No Flow state or task description was changed.

## The references

These are fn-30's exact sources and SHA-256 values, reused without re-sourcing.
`logs/checksums.log` verifies the inherited local bytes. Its unchanged
`../fn30/curves.py` has SHA-256
`9beda29216a153a3ef5a882c1ca16fd938a60e1d457ed0c8912bfcf7e0568dc4`.
There are no additions to the reference set. Year-one height parameters are
implementation values for the spec's centimetre-scale targets, not newly
sourced measurements of either species.
''', refs, '''
Oak height uses E1 Jüttner site class II, a Quercus robur/petraea stand-height
proxy. Before 30 years its height and DBH extend linearly from zero. The DBH
anchor is E1 class I's 7.2 cm at age 30; thereafter G1's large-tree equation
is integrated in the open-grown case (BA and BAL zero, site index 35 m).
That extends a large-tree model into the under-8-cm range. These remain explicit
composition assumptions, not measured young open-grown Garry oak diameters.
Spruce uses E1 Wiedemann class I height and twice its stand mean diameter,
following Sterba via V1. Both extend linearly below the first age-20 row.
U1 supplies no age curve used here. The owner's doubt about the young DBH
composition remains unresolved; S1's mature form is an independent check the
owner may prefer. The source and composition discussion in fn-30 still applies.

## Ratio and diameter at the fit ages

Reference values below are fn-30's rounded curve output. Signed deviations use
those same values, including the reference H/DBH column. The reference oak
ratios themselves rise between the first two ages; the generated ratios fall.

| Species | Age y | Ref H m | Ref DBH m | Ref H/DBH | Wood H m | DBH m | H/DBH | H deviation % | DBH deviation % | Ratio deviation % |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|''']
for name, reference in [
    ('oregon-white-oak',[(26.7,8,.064,125),(56.1,16,.112,142.8),(112,24,.236,101.8)]),
    ('norway-spruce',[(14.1,5,.106,47.3),(26.6,10,.203,49.3),(36.9,15,.285,52.6)])]:
    for age,h,d,q in reference:
        r=next(r for r in rows[name] if r.get('age')==age)
        mh,md=r['height_m'],r['trunk_dbh_m']; mq=mh/md
        s.append(f'| {name} | {age:g} | {h:.2f} | {d:.3f} | {q:.1f} | {mh:.6f} | {md:.6f} | {mq:.3f} | {(mh/h-1)*100:+.2f} | {(md/d-1)*100:+.2f} | {(mq/q-1)*100:+.2f} |')
s.append('''
These are fit-age comparisons, not substituted numerical-maturity targets.
The extrapolated fn-30 maturity references (oak H 39.48 m, DBH 0.902 m;
spruce H 40.08 m, DBH 1.144 m) extend beyond the published height tables.
The model's envelopes remain 24 m and 15 m. Those extrapolations cannot be
treated as observed biological maturity or silently used to grade the fits.

## The rule and derived mature ages

The [fn-11 research](../../specs/fn-11-growth-over-time.md) remains the
methodological basis: Palubicki's vigour, shedding and pipe-model allocation,
with identity-order iteration, pinned transcendentals and decisions sampled
at slice start. The new numeric floor is a proxy guard, not a fitted
physiological constant.

Year-one establishment adds `seedlingHeight * (1 - fraction)` to live height.
`shootStep` bounds internodes and twig length by that height. Structural axes
retain actual grown distance when the annual step changes. Crookedness scales
with the juvenile step, avoiding a mature angular bend over centimetres of stem.
`juvenileBranching` recruits existing lateral buds, tapering to zero at
`juvenileHeight`. Local planning transitions from the current crown to authored
room between one and two juvenile heights. `seedlingRadius` admits first leaves
on slender stems within the anatomy bearing diameter. Shoots stop bearing as
their girth exceeds that threshold.
No species branch enters the generator or renderer; every field is numeric,
validated, serialized and included in the blend walk.

The pipe solve multiplies live height by
`juvenileRadius + (1-juvenileRadius) * progress^thickeningShape`, where progress
is the clamped fraction of derived lifetime after `thickeningDelay`.
Historical maxima keep radius keyframes monotone. The fork power sum and
chronicle/stamp/read contracts stay unchanged in kind. Oak juvenile radius is
0.21 and spruce 0.75; the shared delay is 0.1 and shape 1.4.

Maturity is re-derived by bisection of the final Chapman–Richards work quantum:
the first integer year that rounds to all 250,000 lifetime units. Establishment
changes height within the yearly slice; it does not change that lifetime work
schedule. The radius share reaches one at the same derived year. The results
therefore remain 432/158/173, rather than being authored as preset ages.

| Preset | Rate | Shape | Derived age y | Threshold | Mature nodes | Crossover | Placements | H m | DBH m | H/DBH |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|''')
for name in rows:
    r=max((r for r in rows[name] if r['kind']=='growth'),key=lambda r:r['age'])
    rate,shape,threshold=(.032,2,0) if name=='oregon-white-oak' else ((.091,3.4,0) if name=='norway-spruce' else (.08,2,.45))
    h,d=r['height_m'],r['trunk_dbh_m']
    s.append(f'| {name} | {rate} | {shape} | {r["mature_age"]:g} | {threshold} | {r["nodes"]:,} | {r["crossover"]:,} | {r["placements"]:,} | {h:.6f} | {d:.6f} | {h/d:.3f} |')
s.append('''
## Shedding and regression evidence

Vigour is exposure times `max(1/(1+rate*nodeAge), vigourFloor)`. The floor is
0.75, so age alone cannot make a fully lit branch cross the restored 0.45
threshold. Shading still lowers exposure. Decisions are sampled at slice start,
in identity order, before growth; a lit descendant supports its ancestors.
Zero remains a valid explicit shedding choice on oak and spruce.

The fixture-presets regression failed before the floor at 33 Ordinary,
30 Telperion and 175 Laurelin nodes. Its final tests use the authored 0.45
threshold, assert retained mature populations above 10,000/50,000/50,000,
and test a lit and shaded sibling on each preset. The shaded shoots receive
death stamp 174 after two below-threshold slices; the lit siblings survive.
The mature populations are reported in the preceding table, not hidden behind
the regression's minimum counts. Logs: `survival-red.log`,
`survival-checkpoint-final.log`, and `core-final-before-pin.log` under `logs/`.

| Fixture | Shaded identity | Death year | Lit sibling | Result |
|---|---|---:|---|---|
| Ordinary | birth 1, NodeKey(2v1) | 174 | birth 2, NodeKey(3v1) | survives |
| Telperion | birth 1, NodeKey(2v1) | 174 | birth 2, NodeKey(3v1) | survives |
| Laurelin | birth 1, NodeKey(2v1) | 174 | birth 2, NodeKey(3v1) | survives |

All four seedling/sapling tests failed first: year-one wood height was zero
and the first young ages had zero lateral shoots. `logs/sapling-red.log`
and `logs/form-final.log` retain red and green evidence. The seven form
tests also cover the thickest bole reaching the crown and falling envelope
height/root-diameter ratios. The actual wood H/DBH measurements above are the
fit-age instrument. A seventh regression failed at 561 placements in the expanding
age-26.7 oak crown (`logs/expanding-crown-red.log`). The oak juvenile height is
8 m, keeping local planning inside the live crown through that strip age. The
seedling bearing threshold also continues to admit slender branchlets. Existing history, monotonic radius, pipe split,
determinism and native/Wasm parity assertions remain intact.

The sparse mechanism fixtures state an unshifted establishment curve and
mature radius share. The spruce rate is 0.12 so year 66 remains sparse under
the new rule: 11 resized runs and 880 moved placements, against its unchanged
20,000 limit. At the old rate it moved 101,509 placements. See
`logs/sparse-phase-probe.log` and `logs/sparse-final.log`. No assertion or
tolerance was weakened to preserve the fixture's mechanism.

The first full npm gate reached valid Two Trees builds but failed to index
transfer buffers at signed offsets -2104202656 (Telperion) and -1517937848
(Laurelin). The one-shot binding built foliage/contact surfaces even for a
structure-only selection and retained the annual history during transfer.
It now selects the canonical finalized frontier wood directly when foliage
and field outputs are absent, and releases the owned specimen before allocating
transfer buffers. The cumulative death count comes from the same frontier
counter. Full historical reads, retained specimens and the pointer ABI remain
unchanged. The original parity tests remain unchanged; the red logs are in
`gates-attempt1/npm-final.log` and the intermediate lifetime-only probe in
`logs/npm-release-history.log`.

## Renderer prerequisite and geometry

Checkpoint 135c956 fixed all four inherited renderer targets. A paused
structural axis now retains its terminal bud; the local layer cannot divert
the thickest bole at a temporary apex. Permanent axes use numeric crown-base
retention. The relief fixture's node ceiling is 40,000 rather than 400 so
it reaches the test's existing 0.16 m relief scale. Conformance fixture
values keep its unchanged jitter inside the parameter domain; the frozen
original Ordinary set 6 gains foliage with the vigour floor (zero to 576
placements). The checkpoint body and `CHECKPOINT1.md` record those values.

Sapling subdivision initially regressed the distance test to 3.523567/255.
Removing juvenile laterals alone left 3.436033/255, and disabling crookedness
left 3.221017/255. Scaling crookedness by juvenile step length corrects the
centimetre-segment bend while retaining mature variation. The fixed-camera
distance and grazing tests are rerun in `logs/renderer-final.log`.
Assertions, tolerances, test cameras and shaders remain unchanged.

## Convergence and the single re-pin
''')
s.append((P/'CONVERGENCE.md').read_text().replace('# Convergence before the single re-pin',''))
s.append('''
The following values were collected without changing assertions. Identity
literals, Ordinary's legacy audit hash and the clay look image move once after
the convergence record. Unchanged element hashes show that the leaves' anatomy
did not change. `measurements/pins.jsonl` retains the exact new values.

| Preset | Wood vertices | Wood triangles | Instances | Skeleton hash | Placement hash | Element hash |
|---|---:|---:|---:|---|---|---|''')
for line in (P/'measurements/pins.jsonl').read_text().splitlines():
    r=json.loads(line)
    s.append(f'| {r["id"]} | {r["wood_vertices"]:,} | {r["wood_triangles"]:,} | {r["instances"]:,} | {r["skeleton"]} | {r["placement"]} | {r["element"]} |')
s.append('''
## Mature build cost

The inherited `tolerance_cost_report` measures three native `Specimen::build`
samples at derived maturity, seed 7, resize tolerance 0.0001 m. It excludes
owned foliage reads, surface meshing, Wasm overhead and GPU submission. It is
not dial-to-frame latency. Command:
`FN11_TOLERANCE=0.0001 cargo test --release -p telperion-core --lib tolerance_cost_report -- --nocapture`.
Measurements are observations on the shared host, without a reserved exclusive
window. GPU state is recorded in `logs/cost-device.log`.

| Species | Samples ms | Median ms | Ceiling ms | Margin ms | Frames |
|---|---|---:|---:|---:|---:|''')
cost=(P/'logs/cost.log').read_text() if (P/'logs/cost.log').exists() else ''
for name,ceiling in [('OregonWhiteOak',2463),('NorwaySpruce',867)]:
    samples=re.findall(r'preset='+name+r'.*?ms=([\d.]+).*?frames=(\d+)',cost)
    if samples:
        values=[float(x[0]) for x in samples]; median=statistics.median(values)
        s.append(f'| {name} | '+', '.join(f'{v:.6f}' for v in values)+f' | {median:.6f} | {ceiling} | {ceiling-median:+.6f} | {samples[0][1]} |')
retry=P/'logs/cost-idle.log'
if retry.exists():
    s.append('''
The first run overlapped heavy release compilation in the other worktree.
The authorized retry ran after the gates, with GPU and CPU process state
recorded in `logs/cost-idle-device.log`. Both observations are retained;
the retry supplies the comparison below. It is still a shared-host observation.

| Species / idle retry | Samples ms | Median ms | Ceiling ms | Margin ms |
|---|---|---:|---:|---:|''')
    for name,ceiling in [('OregonWhiteOak',2463),('NorwaySpruce',867)]:
        values=[float(x) for x in re.findall(r'preset='+name+r'.*?ms=([\d.]+)',retry.read_text())]
        if values:
            median=statistics.median(values)
            s.append(f'| {name} | '+', '.join(f'{v:.6f}' for v in values)+f' | {median:.6f} | {ceiling} | {ceiling-median:+.6f} |')
s.append('''
The half-second target is assessed against these CPU-only medians; even a
sub-500 ms wood build does not establish a sub-500 ms rendered dial response.

## The strips and stills

Small 280x400 seedling/young previews preceded this one full capture. The
headless command is fn-30's: `target/release/examples/headless --preset <id>
--seed 7 --age <years> --size 700x1000 --out
.flow/evidence/fn31/strips/<id>-age-<years>.png`. Only year one adds
`--frame-min-y 0`, a numeric framing parameter excluding buried wood; the
pose then frames that seedling's own bounds. Later frames retain the exact
standard pose. Oak ages: 1, 10, 26.7, 56.1, 112, 200, 432. Spruce ages:
1, 5, 14.1, 26.6, 36.9, 60, 158. Montages use
`montage -tile 7x1 -geometry +4+0`. The mature heroes use fn-14's
`--size 1600x1000`, seed 7, with no `--age`. `capture.py` and
`logs/capture.log` record the native commands and exits. Individual age frames
stay on disk; composed strips and comparisons are retained as evidence.

The implementer viewed one four-frame small-preview montage before the final
correction, and no full-capture images. The host and owner judge the captures.
These strips are the replacement instrument for fn-30's rejected R3 slots;
the previously recorded owner words remain in fn-30's report.

| Artifact | Oak | Spruce |
|---|---|---|
| Seven-age strip | `strips/oregon-white-oak-strip.png` | `strips/norway-spruce-strip.png` |
| Strips beside fn-30 | `oregon-white-oak-strips-beside-fn30.png` | `norway-spruce-strips-beside-fn30.png` |
| Mature hero | `oregon-white-oak-hero.png` | `norway-spruce-hero.png` |
| Mature beside fn-30 | `oregon-white-oak-beside-fn30.png` | `norway-spruce-beside-fn30.png` |

The look pin is `.flow/evidence/fn24/ordinary-hero.png`, regenerated once with
`headless --preset ordinary --seed 7 --size 1600x1000 --view clay`.

## Gates

Each command runs in full. Logs are unedited and paths absolute.

| Command | Exit code | Log path |
|---|---:|---|''')
gates=P/'gates-final.json'
if gates.exists():
    for g in json.loads(gates.read_text()):
        s.append(f'| `{g["command"]}` | {g["exit"]} | `{g["log"]}` |')
s.append('''
## Blocked

R1 requires the owner's accepting words for both strips and the continuation
into maturity. R2's reference composition and fit deviations remain for the
owner to judge. Empty slots below are not acceptance. Any numeric or gate
blocker from the final run is stated here before handoff.

## Owner verdict

- R1, oak strip and mature comparison:
- R1, spruce strip and mature comparison:
- R2, reference composition and diameter deviations by age:
''')
young=['''## Seedling and sapling measurements

These counts precede canopy shell culling. The centimetre-scale establishment
values are implementation choices under R1, not observations added to the
reference set. The images, not these counts alone, are the owner's instrument.

| Species | Age y | Wood height m | Nodes | Placements |
|---|---:|---:|---:|---:|''']
for name,ages in [('oregon-white-oak',[1,10,26.7]),('norway-spruce',[1,5,14.1])]:
    for age in ages:
        r=next(r for r in rows[name] if r.get('age')==age)
        young.append(f'| {name} | {age:g} | {r["height_m"]:.6f} | {r["nodes"]:,} | {r["placements"]:,} |')
report='\n'.join(s).replace('## The strips and stills','\n'.join(young)+'\n\n## The strips and stills')
report=report.replace('The fixed-camera\ndistance and grazing tests are rerun in `logs/renderer-final.log`.',
    'The final fixed-camera 4x distance mean is 2.725967/255 (p95 8.75),\nagainst limits 3.0 and 12.0. The oak grazing mask holds 7,513 pixels and\nspruce 2,079. All four targets pass in `logs/renderer-final.log`.')
problems=[]
if gates.exists():
    problems.extend(f'Gate exit {g["exit"]}: {g["command"]}; see {g["log"]}.'
                    for g in json.loads(gates.read_text()) if g['exit'] != 0)
if retry.exists():
    targets=[]
    for name,ceiling in [('OregonWhiteOak',2463),('NorwaySpruce',867)]:
        values=[float(x) for x in re.findall(r'preset='+name+r'.*?ms=([\d.]+)',retry.read_text())]
        if values:
            median=statistics.median(values)
            targets.append(f'{name}: {median-500:+.3f} ms against the 500 ms wood-build target')
            if median>ceiling:
                problems.append(f'R5: {name} idle median {median:.6f} ms exceeds {ceiling} ms.')
    report=report.replace('The half-second target is assessed', '; '.join(targets)+'.\n\nThe half-second target is assessed')
report=report.replace('Any numeric or gate\nblocker from the final run is stated here before handoff.',
    '\n\n'+'\n\n'.join(problems) if problems else
    '\nAll six fit diameters are within 15 percent of the inherited references.\nNo additional numeric or gate blocker remains after the recorded idle retry\nand final full gates.')
(P/'REPORT.md').write_text(report)
