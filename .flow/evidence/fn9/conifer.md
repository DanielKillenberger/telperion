# Norway spruce — fn-9.6 calibration

`Preset::NorwaySpruce` / `norway-spruce` resolves explicitly to frozen `norway-spruce` (Picea abies). Family anatomy is independent of seed. This completes native numeric/geometry calibration, not final visual acceptance. Habit and foliage remain native-only until task 7; no viewer/Wasm exposure is claimed. No shared generation rule, profile range, runtime dependency or deterministic fixture changed.

## Parameters

One family, all 12 frozen seeds: envelope height 15 m, crown base .04, spread .32, fullness .15, shoulder 1. Tiered habit: 16 tiers, 5 primaries/tier, secondary spacing .35 m, secondary length .3 of primary length, upturn .12. Natural bias NONE, supernatural disabled. Radius .015 of height (.225 m at root); remaining radius defaults. Local twig defaults except .004 m needle station internodes and .02 m bearing diameter: .005 m diameter, .25 m length, one station/internode, 2 laterals, length ratio .4. FourSidedNeedle length .018 m, cross-section width .0015 m, peg .001 m. RadialNeedles attachment, default 137.508-degree divergence, canopy size 1, isotropic ±20% variation. Other Family defaults remain, including neutral ordinary surface and shell depth .45.

The first candidate passed all 12 gates and is the final calibration. Raw initial run: `.flow/tmp/spruce-initial.jsonl`. No showcase-specific tuning or rejected numeric seed exists. Height is deliberately invariant because Tiered constructs its persistent leader to authored envelope height; seed varies azimuth, primary lengths, secondary bends and local shoots. Tests require crown-width and foliage-count variation without borrowing oak's >1 m height-variation threshold.

## Measurements and integrity

Committed receipts: `conifer-measurements.json` and `conifer-oak-regression.json`. Each preserves all per-trait checks, counts, growth flags, machine, source SHA, timings and output sizes; only per-axis branch-length arrays are omitted. The 24-case run is scoped by each receipt's cases array. Source gates are unchanged: height 12–18 m, retained-foliage crown width 7.62–9.144 m, individual needle length .012–.025 m. Every gate passes for every fixed seed. Needle extrema are approximately .0144–.0216 m; cross-section .0012–.0018 m is contextual. One retained instance equals one individual needle, never a fascicle. Pegs are excluded from dimensional and needle surface accounting; closed external surface and projected area stay separately reported.

| Seed | Height m | Crown m | Lowest foliage m | Twigs | Needles | Total ms | Gates |
|---|---:|---:|---:|---:|---:|---:|---|
| 1 | 15.000 | 8.748 | 0.680 | 7020 | 435292 | 1047.6 | pass |
| 2 | 15.000 | 8.696 | 0.672 | 6979 | 433517 | 1009.3 | pass |
| 3 | 15.000 | 8.978 | 0.699 | 7092 | 440304 | 1066.0 | pass |
| 5 | 15.000 | 8.743 | 0.738 | 7092 | 440323 | 1042.9 | pass |
| 8 | 15.000 | 8.905 | 0.705 | 6963 | 432936 | 1020.3 | pass |
| 13 | 15.000 | 8.750 | 0.604 | 7040 | 438326 | 1032.2 | pass |
| 21 | 15.000 | 9.125 | 0.676 | 7011 | 435153 | 1023.7 | pass |
| 34 | 15.000 | 8.790 | 0.676 | 6977 | 433575 | 982.7 | pass |
| 55 | 15.000 | 8.808 | 0.657 | 7075 | 439782 | 1002.3 | pass |
| 89 | 15.000 | 8.716 | 0.688 | 7098 | 440549 | 1083.1 | pass |
| 144 | 15.000 | 8.852 | 0.689 | 6946 | 431758 | 1010.1 | pass |
| 233 | 15.000 | 8.873 | 0.652 | 6965 | 432882 | 996.9 | pass |

All cases validate solved topology, uncapped growth, local containment, exact repeated skeleton/surface/attachment output, finite geometry, valid indices, nondegenerate wood triangles and validated needle geometry. Separate structural assertions check a continuous axial leader, at least 12 multi-primary tiers, low primary insertion below 2 m, hanging structural edges and upturned primary ends. Retained foliage extends below 3 m. These are engineering structural invariants, not invented botanical ranges. Existing foliage tests independently verify local attached origins, radial orientation and four-sided needle anatomy. All oak gates and its original variation thresholds still pass.

## Retained counterexamples and targeted follow-up

DBH is **ambiguous in all 12 spruce cases**, not a numeric pass or a summed diameter. The existing measurement helper examines every structural edge crossing 1.3 m, including the hanging secondaries of this low-branched tree. Seed 1 reproduces a .42420 m leader crossing plus four approximately .0139 m secondary crossings. The output retains every crossing diameter. The frozen definition asks for dominant-trunk DBH and ambiguity for genuinely multiple stems; this conservative helper cannot distinguish the two. Targeted measurement follow-up before final task-9 QA: traverse/identify the dominant trunk at the measurement plane, distinguish hanging lateral crossings from multiple basal stems, and add both analytic counterexamples while preserving existing true multi-stem ambiguity tests. This task leaves the shared helper unchanged and DBH contextual/unavailable; it does not lift low branches to evade the issue.

Seed 21 crown width is 9.12464 m, close to the 9.144 m upper gate. Retain it as a boundary case; no proof about all possible seeds is claimed. Task 9 must draw and retain all fresh seeds before measuring, with no resampling.

## Capture status

Native exporter `.flow/tmp/spruce_capture.rs` mirrors oak and exports actual mesh, needle prototype and all retained instance matrices. To reproduce, temporarily copy it to `crates/telperion-core/examples/spruce_capture.rs`, run that example with Cargo, then remove the temporary copy. Initial JSON helper: `.flow/tmp/spruce-capture.mjs`. Binary retry: `.flow/tmp/spruce-capture-binary.mjs SEED`, using identical matrices in a Float32 binary buffer to avoid large JSON matrix parsing. Both load Three.js and the existing neutral `createStage`, not stale Wasm bindings.

The provided `/tmp/fn20-browser/...` Playwright and `/usr/bin/chromium` paths were absent. Fallback used `/workspace/telperion/node_modules/playwright-core/index.mjs` and `/usr/bin/google-chrome`, headless only. Backend probe `.flow/tmp/spruce-renderer.json`: HeadlessChrome 151; ANGLE Vulkan SwiftShader (Subzero), software. Camera: 1200×1000, DPR 1, FOV 38°, direction normalized (.62,.28,1), margin 1.3, `stage.frame(15)`, clay wood and double-sided clay needles, sky illumination only. Bare views hide foliage after framing the complete bounds, without reframing.

Successful and inspected: `.flow/tmp/spruce-1-bare.png`, `spruce-2-bare.png`, `spruce-3-bare.png`. All show a continuous upright leader, low primaries, tapering crown and hanging secondary wood. Trait-level diagnostic status: branching components present, but overall branching-habit fidelity **unassessed**; the very straight leader and regular tier spacing remain naturalness concerns. Crown silhouette and crown gaps **unassessed** for leaf-on output. Terminal taper **unassessed** at whole-tree scale; the long exposed leader and flared base need detail inspection. Foliage shape/attachment **unassessed visually**, with native anatomy tests green. No final visual trait is promoted to a pass from a bare image.

Attempted `.flow/tmp/spruce-1-whole.png`, `spruce-2-whole.png`, `spruce-3-whole.png` all failed at screenshot timeout (30 seconds); no successful whole images exist. The initial JSON whole attempt also timed out. Per-seed binary retries retained successful bare captures before enabling all needles, with 55-second process bounds. Logs: `.flow/tmp/spruce-capture.log`, `spruce-capture-fallback.log`, and `spruce-capture-{1,2,3}.log`. Failed captures never count as passes. Attached foliage detail was not captured. The family has about 432–441 thousand retained needles (56 prototype triangles each, including peg), plus roughly 858 thousand wood triangles for seed 1; software capture cost remains a concrete task-9 concern, not a hardware performance verdict. No owner acceptance or fresh-seed result is claimed.

## Verification and reproduction

`cargo test --release -p telperion-core --test species --test species_metrics --test foliage --test growth --test surface`: 37 passed. `cargo check --workspace`, `cargo fmt --all --check`, `git diff --check`: passed. The committed measurement runner passed all 12 spruce and all 12 oak cases. No golden fixture updates. No reviews, PR or merge.

Use the Rust homes in HANDOFF.md, compile `cargo build --release -p telperion-core --example species_measure`, then pass `--case spruce-SEED:norway-spruce:norway-spruce:SEED` for seeds 1,2,3,5,8,13,21,34,55,89,144,233 and a new `--output PATH`. Oak regression uses `oak-SEED:oregon-white-oak:oregon-white-oak:SEED`. Temporary `.flow/tmp/spruce-measure.py` batches the frozen seeds. Commit provenance in conifer-evidence.json identifies the implementation used by Flow completion; subsequent documentation commits do not change geometry.

Next job: fn-9.7 native/Wasm/schema and typed consumers. Not started here because final whole-tree visual readiness remains uncertain. Task 9 owns fixed/fresh visual acceptance, detail/junction QA, the DBH measurement follow-up and cost assessment.
