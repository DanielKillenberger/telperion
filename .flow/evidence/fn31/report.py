"""Compose the current task report; preserve the rejected Round 2 report separately."""
import json
import re
import statistics
from pathlib import Path
P = Path(__file__).resolve().parent
R = P/'round3'
names = ['oregon-white-oak','norway-spruce','ordinary','telperion','laurelin']
rows = {n: [json.loads(x) for x in (P/'measurements'/f'{n}.jsonl').read_text().splitlines()] for n in names}
def growth(n): return [r for r in rows[n] if r['kind']=='growth']
def mature(n): return max(growth(n),key=lambda r:r['age'])
old = (P.parent/'fn30/REPORT.md').read_text()
refs = old[old.index('| ID |'):old.index('U1 lists')]
s = ['''# FN31: sapling form, thickening and retained crowns

Round 3 continues from bf92380 after the owner rejected the preceding strips.
The current evidence is below. `round3/ROUND2.md` preserves the rejected report.
The owner verdict slots remain empty.

## Protocol

Native release builds on the Ryzen 9 5950X and NVIDIA RTX 3080, seed 7.
Production still grows the same yearly rule to each preset's derived age.
`measure.py` runs `cargo run --release -p telperion-core --example growth_curve
-- --preset <id> --seed 7 --ages <ages> --envelope`. The JSONL files in
`measurements/` are the current measurements; command logs are in `logs/measure-<id>.log`.
The instrument now also counts lateral shoots and structural laterals, while
retaining the inherited height, DBH, bounds and placement measurements.
DBH follows the thickest structural continuation and interpolates at 1.3 m.
Height and bounds describe wood above the root, excluding leaves and bark.
Decimal ages complete integer slices. Read-at-age remains a filter over stamps.

## References reused

These are fn30's URLs and SHA-256 values, reused without re-sourcing or additions.
`logs/checksums.log` records the inherited source-byte checks; Round 3 rechecks
`../fn30/curves.py` in `round3/logs/checksums.log`. Its SHA-256 remains
`9beda29216a153a3ef5a882c1ca16fd938a60e1d457ed0c8912bfcf7e0568dc4`.
The seedling heights remain implementation values for the spec's centimetre-scale
form, rather than new sourced observations.
''', refs, '''
Oak height uses E1 Jüttner site class II as a Quercus robur/petraea stand-height
proxy. The young DBH composition extends the 7.2 cm age-30 E1 class-I anchor
linearly below 30, then integrates G1's large-tree equation with BA and BAL zero
and site index 35 m. Spruce uses E1 Wiedemann class-I height and twice its stand
mean diameter, following Sterba via V1, with linear extension below age 20.
U1 supplies no age curve used here. The owner's doubt about the young-age
composition remains; Stein's independent mature form is available in S1.

## Ratio and diameter at the fit ages

Reference values are fn30's rounded composed-curve outputs, unchanged. Signed
percent deviations use those values. H/D here uses measured height and DBH;
the separate radius-history test uses live-envelope height and root diameter.

| Preset | Age | Height m | DBH m | Reference DBH m | DBH deviation | H/DBH | Reference H/DBH | Ratio deviation |
|---|---:|---:|---:|---:|---:|---:|---:|---:|''']
fits = {'oregon-white-oak':[(26.7,.064,125),(56.1,.112,142.8),(112,.236,101.8)],
        'norway-spruce':[(14.1,.106,47.3),(26.6,.203,49.3),(36.9,.285,52.6)]}
misses=[]
for n, ref in fits.items():
    for age, dbh, ratio in ref:
        a=next(r for r in growth(n) if r['age']==age)
        d=a['trunk_dbh_m']; q=a['height_m']/d; dev=(d/dbh-1)*100
        s.append(f'| {n} | {age:g} | {a["height_m"]:.6f} | {d:.9f} | {dbh:.3f} | {dev:+.2f}% | {q:.3f} | {ratio:.1f} | {(q/ratio-1)*100:+.2f}% |')
        if abs(dev)>15: misses.append(f'{n} at {age:g} y, DBH deviation {dev:+.2f}%')
s += ['\nR2 tolerance misses: '+('; '.join(misses) if misses else 'none against the inherited composed references.')+' The owner chooses the accepted reference in the empty R2 slot.\n',
      '## Derived mature ages\n', '| Preset | Rate | Shape | Derived years |', '|---|---:|---:|---:|']
for n in names:
    rate, shape = (.032,2) if n==names[0] else ((.091,3.4) if n==names[1] else (.08,2))
    s.append(f'| {n} | {rate:g} | {shape:g} | {mature(n)["mature_age"]:g} |')
s += ['''
The derivation finds the first integer slice where the Chapman-Richards fraction
snaps to one at `1 - 0.5/250000`. Re-evaluating the final traits gives the ages
above. Geometry and secondary thickening changed; that height-curve saturation
criterion remains unchanged. These are numerical saturation ages, not sourced
biological maturity observations.

## Young-age evidence

Laterals count nodes stamped with lateral fate; they include local woody shoots
and twigs. The focused test additionally requires woody lateral shoots and checks
that at least half of upper lateral shoots have foliage on themselves or their
descendants. It does not require a structural fork at the base of a seedling.

| Preset | Age | Nodes | Laterals | Structural laterals | Placements |
|---|---:|---:|---:|---:|---:|''']
for n, ages in [(names[0],[1,2,10,26.7]),(names[1],[1,2,5,14.1]),('ordinary',list(range(1,11)))]:
    for age in ages:
        a=next(r for r in growth(n) if r['age']==age)
        s.append(f'| {n} | {age:g} | {a["nodes"]:,} | {a["laterals"]:,} | {a["structural_laterals"]:,} | {a["placements"]:,} |')
s += ['''
The all-five-presets test builds every whole year 1 through 10 and rejects an
empty placement vector or an empty production foliage mesh after shell culling. Its initial red named Ordinary 2–4, both Two Trees 1–10,
and the separate woody-lateral test was red for oak 10 with zero woody laterals.
`round3/logs/form-red.log` and `round3/logs/woody-laterals-red.log` retain those
failures. The latter uses an isolated bf92380 archive and separate target directory.
`round3/logs/cohort-spread-red.log` proves that the first needle cohort initially
covered only the proximal shoot base. The final full workspace gate reruns these
and the inherited seedling, sapling, expanding-crown, bole, ratio and survival tests.

The provisional aggregate upper-half leaf percentage is preserved in
`round3/provisional_form_test.rs`. It was replaced before committing with a
shoot-coverage test because cohort age affects aggregate leaf percentages. The
provisional structural-only lateral requirement likewise excluded local woody
branches, although those are part of the specified sapling form. No inherited
assertion, camera, shader or tolerance was weakened.

## Shedding evidence

Ordinary and the Two Trees retain the authored 0.45 threshold. Oak and spruce
retain their authored zero threshold. The common floor remains 0.75, applied to
exposure before the threshold test. Slice-start snapshots, identity order,
monotone radius records and the pipe-model fork split remain intact.

| Preset | Threshold | Mature age | Nodes | Placements |
|---|---:|---:|---:|---:|''']
for n in names:
    a=mature(n)
    s.append(f'| {n} | {0 if n in names[:2] else .45:g} | {a["age"]:g} | {a["nodes"]:,} | {a["placements"]:,} |')
s += ['''
The fixture-presets survival tests assert retained mature crowns and a shaded
interior death stamp while a lit sibling survives. With the frozen synthetic
siblings, the shaded shoot (birth 1, NodeKey 2v1) dies at slice 174 and the lit
shoot (birth 2, NodeKey 3v1) survives on all three presets. The new run is recorded
in `round3/logs/survival.log`; the original failing floor test remains in `logs/`.

## Convergence and the re-pin

`CONVERGENCE.md` states node counts, crossover counts, placements and both bounds
for fn30, the rejected Round 2 state, and the current rule. It is written before
the one additional re-pin authorized by the owner in Round 3.

| Preset | fn30 nodes | Current nodes | Node deviation | fn30 placements | Current placements |
|---|---:|---:|---:|---:|---:|''']
for n in names:
    a=max((json.loads(x) for x in (P/'prechange'/f'{n}.jsonl').read_text().splitlines() if json.loads(x)['kind']=='growth'),key=lambda x:x['age']); b=mature(n)
    s.append(f'| {n} | {a["nodes"]:,} | {b["nodes"]:,} | {(b["nodes"]/a["nodes"]-1)*100:+.2f}% | {a["placements"]:,} | {b["placements"]:,} |')
s += ['''
Primary shoot planning now uses the pipe allocation before secondary thickening,
so juvenile thinness cannot permanently shorten the mature crown. The emitted
radii retain the annual secondary scale. Crown-room planning caps a future crown base at the branch's birth station,
which admits the young branch without granting permanent room below its attachment. The leader keeps its structural terminal;
other axes keep the local terminals that fill the crown. One existing lateral bud
can form a short leafy shoot without being allocated twice. Twig length fits
current room. Cohort sites interleave along the shoot, with stable identities.
Annual scheduling and preset values remain unchanged from Round 2.

Production skeleton, placement, mesh-count and bound pins move for those geometry
changes. Legacy envelope audit hashes and element hashes remain unchanged.
The Ordinary clay look reference moves with the grown geometry. The exact HELD
trait inventory, renderer cameras, shaders and drift thresholds remain unchanged.

## Mature build cost

The authoritative current cost is the median of the three samples per species
from one clean measurement window. The prebuilt test measures chronicle and wood
generation through growth, as fn30 did; it excludes foliage derivation, meshing
and rendering. `round3/logs/cost-idle.log` includes `nvidia-smi` and `top` before
the measurement and the exact command. No other task build or render runs in that
window. Desktop activity visible in those commands remains part of the record.

| Preset | Current idle median ms | fn30 ceiling ms | Difference ms | Half-second target |
|---|---:|---:|---:|---|''']
cost=R/'logs/cost-idle.log'
for n, tag, ceiling in [(names[0],'OregonWhiteOak',2463),(names[1],'NorwaySpruce',867)]:
    samples=[float(x) for x in re.findall(rf'TOLERANCE preset={tag} .*? ms=([\d.]+)',cost.read_text() if cost.exists() else '')]
    if samples:
        ms=statistics.median(samples)
        s.append(f'| {n} | {ms:.3f} | {ceiling} | {ms-ceiling:+.3f} | {"met" if ms<=500 else "missed"} |')
    else: s.append(f'| {n} | pending | {ceiling} | pending | pending |')
s += ['''
Earlier Round 2 medians were oak 2,132.316 / spruce 2,483.966 ms in a contended
run and 611.682 / 716.808 ms in its idle retry. They measured the rejected smaller
crowns. `round3/ROUND2.md` and the original cost logs preserve those observations;
the current table above is the single result used for this round's R5 judgment.

## Captures

The final capture uses `target/release/examples/headless --preset <id> --seed 7
--age <years> --size 700x1000 --out .flow/evidence/fn31/strips/<id>-age-<years>.png`.
Year one also passes the generic `--frame-min-y 0` parameter to frame the seedling
above ground. Oak ages are 1, 10, 26.7, 56.1, 112, 200, 432; spruce ages are
1, 5, 14.1, 26.6, 36.9, 60, 158. Each strip uses `montage -tile 7x1 -geometry +4+0`.
Mature heroes use `--size 1600x1000`, seed 7 and no `--age`, then a beside-fn30 montage.

Four small 350×500 previews preceded the full capture; four images were viewed.
The full-capture budget is one run after the code checkpoint and zero full-capture
images viewed by the implementer. Commands and exits are in `round3/logs/capture.log`.
`CAPTURE.json` records image hashes and dimensions. Individual frames stay on disk.

| Preset | Seven-age strip | Beside fn30 strip | Mature comparison |
|---|---|---|---|''']
for n in names[:2]:
    s.append(f'| {n} | [strip](strips/{n}-strip.png) | [strips beside fn30]({n}-strips-beside-fn30.png) | [mature beside fn30]({n}-beside-fn30.png) |')
s += ['\n## Gates\n','| Command | Exit code | Absolute log path |','|---|---:|---|']
gates=R/'gates-final.json'
if gates.exists():
    for g in json.loads(gates.read_text()): s.append(f'| `{g["command"]}` | {g["exit"]} | `{g["log"]}` |')
else: s.append('| Full final gates | pending | pending |')
s += ['''
## Owner verdict

### R1: sapling form and continuity


### R2: accepted diameter references and deviations


### Re-judgment of fn30's rejected strips

''']
capture = json.loads((P/'CAPTURE.json').read_text())
if capture.get('status') != 'complete':
    s.insert(1, '\nCode-checkpoint report. The full capture, re-pin and final gates are pending; linked images still show Round 2.\n')
(P/'REPORT.md').write_text('\n'.join(s).rstrip()+'\n')
