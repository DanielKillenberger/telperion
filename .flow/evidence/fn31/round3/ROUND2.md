# FN31: sapling form, age-dependent thickening and retained mature crowns

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

| ID | File | Source | SHA-256 | Bytes |
|---|---|---|---|---:|
| E1 | `ertragstafeln.pdf` | [Bavarian yield-table extracts, Jüttner 1955 oak and Wiedemann 1936/42 spruce](https://www.forstpraxis.de/sites/forstpraxis.de/files/2023-07/AFZ_FHJ_Kalender_2024_306_318_Ertragstafeln_ste_OK.pdf) | `c6c7d6558fe6f8157c2fea3a67e8aaed133902b18446eb32275972fc2a5f9885` | 351,827 |
| G1 | `gould2011.pdf` | [Gould, Harrington and Devine 2011](https://cascadiaprairieoak.org/wp-content/uploads/2014/07/Gould-P.J.-C.A.-Harrington-and-W.D.-Devine.-2011.-Growth-of-Oregon-White-Oak-Quercus-garryana.pdf) | `3bac24bce07da85b47b280a7a78c0e6a220de2e86930e5ddbe56249d63916341` | 605,406 |
| S1 | `silvics-oregon-white-oak.html` | [Stein 1990, Silvics](https://research.fs.usda.gov/silvics/oregon-white-oak) | `21d0db9ce11e8ee1966c5adc97bb3140f64ef2efc8744de0be72f587fdbeeb5f` | 103,163 |
| V1 | `pmc2987550.html` | [Vospernik, Monserud and Sterba 2010](https://pmc.ncbi.nlm.nih.gov/articles/PMC2987550/) | `3193a0d83020a90fa21da2e2f49b0bab6bfde3304b04fbaf2de1ce5214bb06ad` | 314,453 |
| U1 | `utd-gtr253.pdf` | [Urban Tree Database report](https://research.fs.usda.gov/download/treesearch/52933.pdf) | `0765570cf3513a8cc91ba07b4abd7702611b5e765b07e93cde67555b4097b1c7` | 10,558,386 |



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
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| oregon-white-oak | 26.7 | 8.00 | 0.064 | 125.0 | 7.710741 | 0.057018 | 135.234 | -3.62 | -10.91 | +8.19 |
| oregon-white-oak | 56.1 | 16.00 | 0.112 | 142.8 | 16.522651 | 0.126275 | 130.847 | +3.27 | +12.75 | -8.37 |
| oregon-white-oak | 112 | 24.00 | 0.236 | 101.8 | 22.312851 | 0.221363 | 100.797 | -7.03 | -6.20 | -0.98 |
| norway-spruce | 14.1 | 5.00 | 0.106 | 47.3 | 4.877722 | 0.099785 | 48.882 | -2.45 | -5.86 | +3.35 |
| norway-spruce | 26.6 | 10.00 | 0.203 | 49.3 | 10.577722 | 0.230683 | 45.854 | +5.78 | +13.64 | -6.99 |
| norway-spruce | 36.9 | 15.00 | 0.285 | 52.6 | 12.977722 | 0.286739 | 45.260 | -13.48 | +0.61 | -13.95 |

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
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| oregon-white-oak | 0.032 | 2 | 432 | 0 | 19,513 | 3,614 | 132,875 | 23.956756 | 0.836201 | 28.650 |
| norway-spruce | 0.091 | 3.4 | 158 | 0 | 65,604 | 18,669 | 4,810,878 | 15.000000 | 0.426393 | 35.179 |
| ordinary | 0.08 | 2 | 173 | 0.45 | 16,066 | 1,802 | 110,297 | 16.201272 | 0.927729 | 17.463 |
| telperion | 0.08 | 2 | 173 | 0.45 | 99,128 | 1,456 | 726,463 | 97.065382 | 14.697044 | 6.604 |
| laurelin | 0.08 | 2 | 173 | 0.45 | 151,220 | 4,371 | 1,241,165 | 131.169349 | 14.465550 | 9.068 |

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
centimetre-segment bend while retaining mature variation. The final fixed-camera 4x distance mean is 2.725967/255 (p95 8.75),
against limits 3.0 and 12.0. The oak grazing mask holds 7,513 pixels and
spruce 2,079. All four targets pass in `logs/renderer-final.log`.
Assertions, tolerances, test cameras and shaders remain unchanged.

## Convergence and the single re-pin



The wire audit's HELD inventory is also recorded before its first move: 28
paths become 32. Add growth.juvenileBranching, shootStep, thickeningDelay,
thickeningShape and vigourFloor, whose numeric values are shared by every row;
remove skeleton.habit.sheddingThreshold, now varied (0 versus the restored
0.45). The exact set equality and the assertions proving all varying paths
blend are unchanged. The first full gate exposed this stale schema inventory
after the geometry pins moved; no geometry, identity or look pin moves again.

Pre-change measurements are from base ccb44eaf609c15e4df7170385f81b372b0d5f532, seed 7. Both columns use production growth at derived maturity. Bounds are node bounds in metres, excluding bark and leaves.

| Preset / state | Age | Nodes | Crossover | Min bounds | Max bounds |
|---|---:|---:|---:|---|---|
| oregon-white-oak / before | 432 | 196,901 | 5,394 | (-12.571036, 0.000000, -13.000526) | (13.162612, 23.729755, 12.981341) |
| oregon-white-oak / after | 432 | 19,513 | 3,614 | (-12.061294, 0.000000, -12.542657) | (13.107141, 23.956756, 13.023946) |
| norway-spruce / before | 158 | 76,386 | 21,442 | (-3.859580, 0.000000, -4.160924) | (3.818373, 15.000000, 4.068296) |
| norway-spruce / after | 158 | 65,604 | 18,669 | (-4.057145, 0.000000, -3.927990) | (3.838197, 15.000000, 3.740467) |
| ordinary / before | 173 | 44,235 | 2,434 | (-5.902908, 0.000000, -6.630118) | (6.234025, 19.324309, 4.998745) |
| ordinary / after | 173 | 16,066 | 1,802 | (-5.151740, 0.000000, -5.096988) | (5.704958, 16.201272, 4.680791) |
| telperion / before | 173 | 190,340 | 1,871 | (-32.897162, 0.000000, -26.297783) | (22.685398, 108.883526, 34.240767) |
| telperion / after | 173 | 99,128 | 1,456 | (-20.019164, 0.000000, -26.199466) | (20.194571, 97.065382, 22.819690) |
| laurelin / before | 173 | 194,517 | 4,588 | (-45.336488, 0.000000, -67.678618) | (58.408722, 110.922843, 71.877712) |
| laurelin / after | 173 | 151,220 | 4,371 | (-64.092891, 0.000000, -67.004836) | (72.321986, 131.169349, 49.315379) |

oregon-white-oak: nodes -177,388 (-90.09%), crossover -1,780 (-33.00%).


norway-spruce: nodes -10,782 (-14.12%), crossover -2,773 (-12.93%).


ordinary: nodes -28,169 (-63.68%), crossover -632 (-25.97%).


telperion: nodes -91,212 (-47.92%), crossover -415 (-22.18%).


laurelin: nodes -43,297 (-22.26%), crossover -217 (-4.73%).


Permanent structural buds no longer flush prematurely as local terminals. Variable juvenile steps retain actual axis length; seedling shoots fill current space before planning adult extensions. The age-dependent pipe scale changes when local stations become eligible. Those mechanisms change the number and position of wood segments and their leaf contacts. Restoration of the 0.45 threshold also removes shaded shoots on Ordinary and the Two Trees. The three mature crowns remain far above tens of nodes; their exact populations are reported rather than assumed equal to the zero-threshold build.

The legacy envelope rows remain in each JSONL file as a separate comparison. Ordinary’s seed-42 legacy audit returns to hash 9848876633805652422 from 4584312898131064280 because its authored 0.45 threshold returns. The species legacy audit hashes and both element hashes must remain unchanged. Production skeleton/placement pins and mesh bounds/counts will move once to these measured populations. The Ordinary clay look pin will move once for the restored threshold and new grown form; its shaders, camera and drift limits remain unchanged.


The following values were collected without changing assertions. Identity
literals, Ordinary's legacy audit hash and the clay look image move once after
the convergence record. Unchanged element hashes show that the leaves' anatomy
did not change. `measurements/pins.jsonl` retains the exact new values.

| Preset | Wood vertices | Wood triangles | Instances | Skeleton hash | Placement hash | Element hash |
|---|---:|---:|---:|---|---|---|
| oregon-white-oak | 717,466 | 1,375,440 | 132,875 | 17147586106779270108 | 3545768933671885177 | 4207404028969543471 |
| norway-spruce | 2,186,602 | 4,214,200 | 4,810,878 | 16388404269218903040 | 11831058783261786492 | 7287062639823569932 |

## Mature build cost

The inherited `tolerance_cost_report` measures three native `Specimen::build`
samples at derived maturity, seed 7, resize tolerance 0.0001 m. It excludes
owned foliage reads, surface meshing, Wasm overhead and GPU submission. It is
not dial-to-frame latency. Command:
`FN11_TOLERANCE=0.0001 cargo test --release -p telperion-core --lib tolerance_cost_report -- --nocapture`.
Measurements are observations on the shared host, without a reserved exclusive
window. GPU state is recorded in `logs/cost-device.log`.

| Species | Samples ms | Median ms | Ceiling ms | Margin ms | Frames |
|---|---|---:|---:|---:|---:|
| OregonWhiteOak | 2221.123899, 2022.168415, 2132.316304 | 2132.316304 | 2463 | +330.683696 | 1689773 |
| NorwaySpruce | 2552.432104, 2483.965985, 2399.395348 | 2483.965985 | 867 | -1616.965985 | 479756 |

The first run overlapped heavy release compilation in the other worktree.
The authorized retry ran after the gates, with GPU and CPU process state
recorded in `logs/cost-idle-device.log`. Both observations are retained;
the retry supplies the comparison below. It is still a shared-host observation.

| Species / idle retry | Samples ms | Median ms | Ceiling ms | Margin ms |
|---|---|---:|---:|---:|
| OregonWhiteOak | 692.965719, 604.873798, 611.681577 | 611.681577 | 2463 | +1851.318423 |
| NorwaySpruce | 700.060003, 716.807856, 718.509411 | 716.807856 | 867 | +150.192144 |

OregonWhiteOak: +111.682 ms against the 500 ms wood-build target; NorwaySpruce: +216.808 ms against the 500 ms wood-build target.

The half-second target is assessed against these CPU-only medians; even a
sub-500 ms wood build does not establish a sub-500 ms rendered dial response.

## Seedling and sapling measurements

These counts precede canopy shell culling. The centimetre-scale establishment
values are implementation choices under R1, not observations added to the
reference set. The images, not these counts alone, are the owner's instrument.

| Species | Age y | Wood height m | Nodes | Placements |
|---|---:|---:|---:|---:|
| oregon-white-oak | 1 | 0.222003 | 6 | 15 |
| oregon-white-oak | 10 | 1.880392 | 55 | 438 |
| oregon-white-oak | 26.7 | 7.710741 | 1,490 | 18,367 |
| norway-spruce | 1 | 0.043689 | 6 | 5 |
| norway-spruce | 5 | 0.452722 | 37 | 190 |
| norway-spruce | 14.1 | 4.877722 | 3,509 | 80,861 |

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
|---|---:|---|
| `cargo fmt --all -- --check` | 0 | `/home/daniel/Projects/telperion/.flow/evidence/fn31/logs/fmt-final.log` |
| `cargo clippy --workspace --all-targets -- -D warnings` | 0 | `/home/daniel/Projects/telperion/.flow/evidence/fn31/logs/clippy-final.log` |
| `cargo test --release --workspace` | 0 | `/home/daniel/Projects/telperion/.flow/evidence/fn31/logs/workspace-final.log` |
| `npm test` | 0 | `/home/daniel/Projects/telperion/.flow/evidence/fn31/logs/npm-final.log` |
| `npm run typecheck` | 0 | `/home/daniel/Projects/telperion/.flow/evidence/fn31/logs/typecheck-final.log` |

## Blocked

R1 requires the owner's accepting words for both strips and the continuation
into maturity. R2's reference composition and fit deviations remain for the
owner to judge. Empty slots below are not acceptance.
All six fit diameters are within 15 percent of the inherited references.
No additional numeric or gate blocker remains after the recorded idle retry
and final full gates.

## Owner verdict

- R1, oak strip and mature comparison:
- R1, spruce strip and mature comparison:
- R2, reference composition and diameter deviations by age:
