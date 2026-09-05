# Oregon white oak — fn-9.5 calibration

Native identity: `Preset::OregonWhiteOak`, selected by `Preset::from_id("oregon-white-oak")`; `profile_id()` resolves explicitly to the frozen Quercus garryana profile. Synthetic families have no botanical profile. Unknown IDs fail. The measurement CLI rejects a named species paired with a different profile, while retaining synthetic presets as cross-profile capability probes.

This is native numeric/geometry calibration, **not final visual acceptance**. Commit provenance and verification are in `broadleaf-evidence.json`; retained per-trait measurements are in `broadleaf-measurements.json`. No research ranges changed. Runtime dependencies remain unchanged; serde_json is dev-only. Habit and new foliage remain native-only until task 7; this task does not expose them through viewer/Wasm.

## Parameters and calibration history

Final family: spreading habit, 5 scaffolds, 4 subdivisions, 24-degree natural crookedness; envelope height 24 m, crown base .16, spread .55, fullness .55, shoulder 2.2. Natural bias is NONE; supernatural effects disabled. Trunk radius .018 of envelope height (.432 m at root), fork exponent 2, length taper .6. Three local laterals, length ratio .45, leaf-bearing diameter .03 m; remaining TwigParams defaults (including .25 m twig length, .02 m internodes, one station/internode). Alternate single leaves advance 180 degrees; lobed blade .10 x .075 m, .012 m petiole, canopy size 1 with common isotropic ±20% per-leaf placement variation. Length and width scale together, preserving aspect ratio. Surface stays ordinary neutral (no twist/deep lobing); shell depth .45. Other controls inherit Family defaults. Seed is assigned separately and never selects anatomy.

The first candidate used 3 subdivisions and default twigs: all 12 numeric cases passed, but inspected whole views of seeds 1/2/3 failed healthy leaf-on crown density (3,935–6,396 retained leaves). Crooked spreading low scaffolds were visible; fine subdivision and foliage were insufficient. Raw attempt: `.flow/tmp/oak-initial.jsonl`; images in `.flow/tmp/oak-initial-captures/`.

A dense candidate used 5 subdivisions, 3 laterals, length ratio .5 and bearing diameter .02 m. All 12 numeric cases passed with 108,580–189,369 leaves. Seed 1 required 58,986 nodes / 2,932,600 wood triangles and about 1.74 s including measurement. Its temporary browser capture crashed, so no inspected visual result is attributed to that candidate. Raw attempt: `.flow/tmp/oak-density.jsonl`. The final family reduces subdivision/branch proliferation to retain more foliage than the initial candidate with substantially less geometry than the dense candidate. This is template-only tuning: no shared-rule repair or fixture update was made.

## Fixed-seed measurements

All four source gates pass individually in every case: height 15–27 m, DBH .6–1.0 m, blade length .05–.15 m and width .0508–.127 m. Actual transformed blade extrema across cases are approximately .080–.120 m long and .060–.090 m wide, excluding petioles. DBH is the circular-equivalent centreline proxy at 1.3 m; its seed invariance follows the shared basal radius/taper, while crown dimensions and foliage abundance vary.

Axes are operational estimates, not measured biological truth. Crown spread/ratio, axes, twigs, leaf totals and sheet area are contextual, not botanical gates. The estimated crown ratio interval .8–1.4 is met but remains contextual. Sheet area is counted once, not doubled for rendering.

| Seed | Height m | DBH m | Crown m | Width/height | Axes estimate | Twigs | Leaves | Sheet m² | All gates |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---|
| 1 | 19.103 | 0.836 | 25.147 | 1.316 | 4382 | 4352 | 55980 | 223.11 | pass |
| 2 | 19.682 | 0.836 | 24.047 | 1.222 | 4590 | 4538 | 58711 | 233.82 | pass |
| 3 | 20.943 | 0.836 | 22.966 | 1.097 | 4684 | 4642 | 59814 | 237.97 | pass |
| 5 | 20.623 | 0.836 | 24.386 | 1.182 | 3598 | 3570 | 45743 | 181.92 | pass |
| 8 | 20.653 | 0.836 | 24.747 | 1.198 | 3329 | 3297 | 41955 | 166.94 | pass |
| 13 | 22.270 | 0.836 | 21.353 | 0.959 | 2551 | 2529 | 32026 | 127.67 | pass |
| 21 | 20.921 | 0.836 | 23.523 | 1.124 | 2737 | 2703 | 34413 | 136.99 | pass |
| 34 | 20.665 | 0.836 | 22.862 | 1.106 | 3282 | 3249 | 41665 | 165.92 | pass |
| 55 | 20.088 | 0.836 | 22.597 | 1.125 | 2902 | 2867 | 36585 | 145.85 | pass |
| 89 | 21.026 | 0.836 | 25.603 | 1.218 | 2962 | 2900 | 37199 | 148.06 | pass |
| 144 | 19.878 | 0.836 | 24.163 | 1.216 | 3229 | 3218 | 41007 | 163.19 | pass |
| 233 | 20.169 | 0.836 | 23.266 | 1.154 | 3568 | 3529 | 45282 | 180.20 | pass |

All cases complete without node/level/attraction truncation. Tests validate solved topology, local node containment, finite positions/normals/transforms, nonempty retained foliage, index bounds, nondegenerate wood triangles and validated foliage geometry. Same-seed generation, complete surface buffers and attachment matrices repeat exactly without golden files. Cross-seed tests require >1 m variation in height and width and at least six distinct retained leaf totals; these are engineering regression thresholds, not invented botanical targets.

## Visual inspection and pending work

Temporary native exporter `.flow/tmp/oak_capture.rs` exports actual wood mesh, blade prototype and retained instance matrices. `.flow/tmp/oak-capture.mjs` loads those buffers directly into Three.js and the existing `createStage`; it does not use stale Wasm family bindings. Whole/bare camera: stage.frame(24), 1200×1000, DPR 1, FOV 38°, direction normalized (.62,.28,1), margin 1.3; bare hides foliage without reframing. Ordinary clay surface and double-sided clay element material, hemisphere sky only, no key/fill. HeadlessChrome 151, ANGLE Vulkan SwiftShader (Subzero), software rendering, not hardware GPU performance. An initial capture using an incorrect material property was discarded and repeated with `clay.element` before inspection. Large JSON bridge capture stalled and was terminated; direct browser fetch replaced that temporary transport. Failed captures never count as passes.

Final whole-tree capture attempts failed (Chrome target crash, then screenshot timeout at 30 s after the JSON transport repair). Final rubric status: crown silhouette, branching habit, crown gaps, terminal taper, and foliage shape/attachment all **unassessed**. The initial inspected candidate fails crown gaps/density; its low crooked scaffold character is visible, but the bare view also leaves tip/junction quality unresolved. No final image is substituted with an earlier candidate. Browser memory/software-render cost remains a practical limitation to evaluate in task 9.

Task 9 still owns full fixed/fresh neutral visual QA, attached-twig close-ups, junction/socket and terminal-taper inspection, owner feedback and integrated cost evaluation. No fresh seeds have been drawn or substituted here. Any remaining visual mismatch must be retained; numerical success is not species fidelity. Task 7 owns native/Wasm/schema roundtrip and task 8 viewer selection. No task 7+ work was started.

## Verification and reproduction

`cargo test --release -p telperion-core --test species --test species_metrics` (7 tests); `cargo test --release -p telperion-core --test growth --test foliage --test surface` (28 tests, including ordinary and both supernatural families); `cargo check --workspace`; `cargo fmt --all --check`; `git diff --check`. CLI mismatched-profile and unknown-preset cases both fail explicitly and subsequent cases still execute.

Use the toolchain homes from HANDOFF.md. Compile `cargo build --release -p telperion-core --example species_measure`, then invoke `target/release/examples/species_measure --case oak-SEED:oregon-white-oak:oregon-white-oak:SEED` once per frozen seed in one command, with a new `--output PATH`. The committed compact receipt retains all trait checks, branch-order histograms, counts, times and buffer sizes; only per-axis length arrays are omitted (not gating and reproducible from the runner).

Next job: fn-9.6 Norway spruce calibration. No fn-9.6 work, reviews, PR creation or merge was performed in this session.
