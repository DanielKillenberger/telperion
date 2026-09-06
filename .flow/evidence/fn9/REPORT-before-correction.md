# FN-9.9 cross-seed QA — blocked on botanical fidelity

2026-09-06. **Do not mark FN-9.9 or FN-9 complete.** All 48 recorded specimens were measured; 47 pass their numeric gates. All 50 required/supplemental PNGs were captured at mature preset scale and personally inspected. The healthy leaf-on reference comparison fails for both species. No owner approval was requested or received. No PR or merge is appropriate while these acceptance blockers remain.

The deliverable here is a reproducible QA runner, a retained counterexample set and an explicit rejection of fidelity completion. The prior software-capture limitation is resolved for this sample; passing dimensions and successful images do not resolve the botanical defects. No botanical targets were relaxed, seeds replaced, generation caps introduced or unverified anatomy fixes substituted into the evidence.

## Protocol and durable evidence

Calibration: `b3d4743`; fresh seed manifest committed at `fe1eefa` before generation. [seeds.json](seeds.json) contains 12 fixed + 12 OS-cryptographic fresh u32 seeds for each species, in draw order. Fixed captures are 1/2/3; fresh captures are the first three recorded seeds. Fresh spruce `4250668600` is an additional mandatory case because its numeric width fails. All failing specimens remain in the manifest for corrective work.

- [Numeric cases](cross-seed-numeric.json): all 48 per-case metric/check statuses, dimensions, counts, CPU times and sizes. Only non-gating per-axis length arrays are omitted; full raw JSONL remains outside the checkout.
- [Capture receipts](cross-seed-captures.json): all 50 inspected PNG paths/hashes, full parameters, diagnostics, exact cameras, browser/renderer identity and output geometry hashes. Automated `visual_status: unassessed` is preserved in receipts; the human assessment is below. Numeric, visual, missing and owner fields remain separate.
- [Source manifest](cross-seed-sources.json): source/binary, reference-image and raw numeric hashes. [Costs](cross-seed-costs.json) retain native warmup plus five measured samples for each regression template.
- Bulk output: `/tmp/fn99-qa/`. Replayed numeric output: `/tmp/fn99-replay/`. Compact representative PNGs are durable under [qa-preview/](qa-preview/); the other PNGs can be regenerated from recorded inputs. Temporary paths are host-local, not hosted artifact URLs.

All six photographs in [REFERENCES.md](REFERENCES.md) were retrieved again from the recorded OSU URLs into `/tmp/fn99-refs/`, decoded and personally inspected. Hashes identify the downloaded images; references remain attributed, local-only source photographs. Task-8's six 4 m viewer images were inspected as baseline visibility evidence, never used as mature-species substitutes.

## Numerical results

Oak: 24/24 numeric pass. Height 17.719–22.270 m, DBH proxy 0.836389 m, retained crown width 21.353–25.603 m, 29,661–59,814 individual leaves. All blade length/width gates pass. Spruce: 23/24 numeric pass. Height 15 m; retained crown width 8.456–9.226 m; 431,758–442,722 individual needles. All needle-length gates pass. Every case reports complete growth; none is node/level/attraction capped.

**Retained numeric failure:** spruce `4250668600` has crown width **9.225869594 m**, exceeding the frozen **9.144 m** maximum by **0.081869594 m** (0.90%). Its whole/bare/detail views were all captured and inspected. This is a real per-case failure, not an average or a replaced seed.

Spruce DBH remains **ambiguous**, because the measurement intersects hanging structural axes as well as the dominant bole at breast height. It is contextual in the frozen spruce profile, so this does not explain or erase the width failure. Resolving the dominant-trunk measurement is still needed before using spruce DBH botanically. Axes are operational estimates; unknown biological branch/leaf totals and contextual crown ratios never become passes. Native finite/nondegenerate/topology tests supplement dimensional metrics.

The table lists every seed. Detailed blade/needle extrema, contextual values, branch orders and check reasons are retained in the numeric JSON.

### oregon-white-oak

| Seed | Set | Height m | Crown m | Nodes | Units | Numeric |
|---|---|---:|---:|---:|---:|---|
| 1 | fixed | 19.103 | 25.147 | 11,079 | 55,980 | pass |
| 2 | fixed | 19.682 | 24.047 | 12,104 | 58,711 | pass |
| 3 | fixed | 20.943 | 22.966 | 12,143 | 59,814 | pass |
| 5 | fixed | 20.623 | 24.386 | 9,693 | 45,743 | pass |
| 8 | fixed | 20.653 | 24.747 | 9,183 | 41,955 | pass |
| 13 | fixed | 22.270 | 21.353 | 7,978 | 32,026 | pass |
| 21 | fixed | 20.921 | 23.523 | 8,751 | 34,413 | pass |
| 34 | fixed | 20.665 | 22.862 | 8,788 | 41,665 | pass |
| 55 | fixed | 20.088 | 22.597 | 8,440 | 36,585 | pass |
| 89 | fixed | 21.026 | 25.603 | 8,441 | 37,199 | pass |
| 144 | fixed | 19.878 | 24.163 | 9,016 | 41,007 | pass |
| 233 | fixed | 20.169 | 23.266 | 9,636 | 45,282 | pass |
| 2666899686 | fresh | 19.549 | 24.648 | 10,495 | 50,509 | pass |
| 762807349 | fresh | 20.829 | 22.709 | 9,537 | 45,618 | pass |
| 1444323199 | fresh | 20.726 | 24.762 | 10,058 | 50,786 | pass |
| 3117916676 | fresh | 20.164 | 24.039 | 9,888 | 46,497 | pass |
| 2181659276 | fresh | 21.327 | 25.555 | 8,315 | 37,984 | pass |
| 696710047 | fresh | 20.586 | 23.121 | 7,298 | 29,661 | pass |
| 4067290831 | fresh | 19.330 | 24.278 | 9,477 | 43,676 | pass |
| 2671455042 | fresh | 21.142 | 23.632 | 9,038 | 40,487 | pass |
| 690286437 | fresh | 19.759 | 24.967 | 11,137 | 55,765 | pass |
| 1352641425 | fresh | 17.719 | 24.250 | 10,452 | 52,274 | pass |
| 204973394 | fresh | 19.716 | 24.793 | 10,213 | 50,278 | pass |
| 535441218 | fresh | 19.069 | 22.626 | 10,563 | 52,853 | pass |

### norway-spruce

| Seed | Set | Height m | Crown m | Nodes | Units | Numeric |
|---|---|---:|---:|---:|---:|---|
| 1 | fixed | 15.000 | 8.748 | 14,244 | 435,292 | pass |
| 2 | fixed | 15.000 | 8.696 | 14,176 | 433,517 | pass |
| 3 | fixed | 15.000 | 8.978 | 14,337 | 440,304 | pass |
| 5 | fixed | 15.000 | 8.743 | 14,342 | 440,323 | pass |
| 8 | fixed | 15.000 | 8.905 | 14,138 | 432,936 | pass |
| 13 | fixed | 15.000 | 8.750 | 14,298 | 438,326 | pass |
| 21 | fixed | 15.000 | 9.125 | 14,239 | 435,153 | pass |
| 34 | fixed | 15.000 | 8.790 | 14,158 | 433,575 | pass |
| 55 | fixed | 15.000 | 8.808 | 14,333 | 439,782 | pass |
| 89 | fixed | 15.000 | 8.716 | 14,285 | 440,549 | pass |
| 144 | fixed | 15.000 | 8.852 | 14,078 | 431,758 | pass |
| 233 | fixed | 15.000 | 8.873 | 14,151 | 432,882 | pass |
| 1982700925 | fresh | 15.000 | 8.865 | 14,357 | 439,913 | pass |
| 281313742 | fresh | 15.000 | 8.722 | 14,339 | 440,810 | pass |
| 2271779095 | fresh | 15.000 | 8.647 | 14,309 | 438,224 | pass |
| 1428158753 | fresh | 15.000 | 8.824 | 14,450 | 442,722 | pass |
| 370981056 | fresh | 15.000 | 8.834 | 14,176 | 434,407 | pass |
| 2571381872 | fresh | 15.000 | 8.657 | 14,402 | 439,940 | pass |
| 1986682132 | fresh | 15.000 | 9.007 | 14,285 | 438,003 | pass |
| 2749500550 | fresh | 15.000 | 8.833 | 14,205 | 434,151 | pass |
| 175890410 | fresh | 15.000 | 8.456 | 14,354 | 440,780 | pass |
| 847818543 | fresh | 15.000 | 8.657 | 14,331 | 437,417 | pass |
| 4250668600 | fresh | 15.000 | 9.226 | 14,233 | 438,446 | fail |
| 893982483 | fresh | 15.000 | 9.102 | 14,317 | 438,737 | pass |

## Inspected visual rubric

Scores apply to the exact mature specimens listed below. `F` = observed failure, `P` = the specific rubric trait passes in these views, `U` = evidence insufficient. These are executor judgments against the frozen source rubric, not owner feedback. W/B/D means whole, bare and attached foliage detail were each opened and inspected. Two isolated-unit `element` images supplement the attached details.

| Species / seed | Views | Silhouette | Branch habit | Gaps | Terminal taper | Foliage shape/attachment | Case observation |
|---|---|---|---|---|---|---|---|
| Oak 1 | W/B/D + element | F | P | F | U | F | Broad scaffold fan, but thin foliage whiskers and large exposed upper limbs; repeated angular lobes. |
| Oak 2 | W/B/D | F | P | F | U | F | Deep central crown void with a sparsely clothed ascending axis; blades alternate along a shoot but miss rounded outline. |
| Oak 3 | W/B/D | F | P | F | U | F | Crooked low scaffold divisions present; central upward sparsity and isolated terminal leaf sprays. |
| Oak 2666899686 | W/B/D | F | P | F | U | F | Broad asymmetric fan with long bare upper/right reaches, not the healthy rounded leaf-on reference. |
| Oak 762807349 | W/B/D | F | P | F | U | F | Open centre and a tall narrow sparse upper crown; detail is largely edge-on, with the same regular lobed prototype. |
| Oak 1444323199 | W/B/D | F | P | F | U | F | Low lateral spread present, central vertical whiskers and large gaps; close-up includes thick rounded wood ends needing dedicated uncropped follow-up. |
| Spruce 1 | W/B/D + element | F | F | F | U | F | Cone and leader components present but skeletal overall; needle is a long wedge, not the reference's mostly parallel-sided needle. |
| Spruce 2 | W/B/D | F | F | F | U | F | Regular bare tiers dominate, little curtain mass; radial needle stations visible but sparse. |
| Spruce 3 | W/B/D | F | F | F | U | F | Hanging wood present, healthy drooping foliage curtains absent; pegs too small to validate contact at this view. |
| Spruce 1982700925 | W/B/D | F | F | F | U | F | Same sparse tapering tower; single-shoot needle detail shows discrete radial stations and excessive longitudinal taper. |
| Spruce 281313742 | W/B/D | F | F | F | U | F | Strongly visible primary tiers, hanging wood not clothed like S-BRANCH; lower twig end is blunt in local view. |
| Spruce 2271779095 | W/B/D | F | F | F | U | F | Conical framework with transparent crown; overlapping shoot geometry limits peg-contact assessment. |
| Spruce 4250668600 | W/B/D | F | F | F | U | F | Numeric width counterexample also repeats sparse crown/curtain failure; needles visible around local twig. |

Oak branching habit passes only the narrow rubric requirement for low substantial, crooked, spreading limbs with finer subdivisions, compared with O-BARE. This does not award whole-tree identity. Oak O-WHOLE is a substantial rounded leaf-on crown with internal windows; all six outputs remain predominantly exposed wood with sprays at endpoints. Earlier calibration already recorded an initial sparse oak failure, but final maturity had been unassessed; these images establish that the final preset still fails it. O-LEAF has rounded, irregular lobes and a rounded terminal lobe; the generated repeating sinusoidal pinches and angular lateral ends fail that shape despite alternate single attachment being visible.

Spruce has the correct architectural components—one leader, low primaries, upturned tips, hanging secondary axes—but not the source's foliage-bearing curtains. The almost leafless full view fails the healthy context even though the bare framework tapers conically. S-NEEDLE has single, four-sided needles with pegs and forward bias. The geometry has distinct faces and single radial units, but its needle-wide taper makes it spike-like. Peg contact and upper-side forward bias remain visually unassessed at this crop; native anatomy tests alone cannot promote them. The combined foliage trait fails on visible shape.

Terminal/junction fidelity is **unassessed**, not passed: whole scale cannot resolve sockets; the clipped local-shoot view intentionally cuts unrelated wood at near/far planes, so apparent detached pieces and concave cuts there cannot be diagnosed as actual seams. Several original rounded twig ends also deserve dedicated, uncropped junction/tip views. No image-space crop artifact is offered as a proven generator bug. These unresolved R6 checks independently prevent completion.

The other 35 numeric specimens were not selected for the visual subset. Their visual status is unassessed, with no missing required image inferred; this is the frozen engineering sample, not proof about every seed.

## Capture environment and replay

Headless Playwright bundled Chromium **153.0.8010.12**, renderer **ANGLE (Google, Vulkan 1.3.0 (SwiftShader Device (Subzero) (0x0000C0DE)), SwiftShader driver)**. Exact browser/user-agent fields are in receipts. 960×720, DPR 1, antialiasing, neutral wood and double-sided flat foliage, no tone mapping, one hemisphere light, no ground/scale figure/shadows. Whole/bare use identical full bounds, direction normalized `(0.62,0.28,1)`, FOV 38° and margin 1.3; actual camera position/target/clip planes are recorded per image. Details retain original nearby instances and original wood, with a local clipped camera. No specimen is shrunk or capped.

The runner draws one frame, calls GPU completion, then saves the canvas PNG. This resolved the earlier continuous-frame software capture starvation for every requested mature oak/spruce view. All 39 required species images, 2 supplemental elements and 9 full-size regression-template images succeeded. No software-renderer GPU speed claim is made.

The first aggregate capture process exited 143 after saving Telperion whole; its cause was not established. Earlier receipts and PNGs survived. Telperion was retried in an isolated process group and reproduced whole/bare/detail; Laurelin also succeeded separately. The runner now bounds subprocess groups, checkpoints pending/started/results, retains previous retry receipts, checks PNG hashes and source provenance before reusing captures, rejects numeric overwrites and keeps partial-run aggregate files separate. Do not interpret an interrupted parent as a successful aggregate run.

Replay commands are in [the migration guide](../../../tests/migration/README.md#species-evidence-and-replay) and `node tests/browser/species.mjs --help`. `npm run species:measure -- --output /tmp/fn99-replay` repeated all 48 metric/count results exactly and exited 1 on the same spruce width counterexample. The capture runner intentionally exits 1 until human visual assessment is supplied separately; no image existence check can approve fidelity. A negative replay with a missing numeric directory and unavailable browser URL preserved 48 unassessed numeric cases and three explicit capture failures under `/tmp/fn99-negative/`, also exiting 1.

## Shared-template regression and same-host costs

No growth, placement, shape or template tuning was applied in this QA session. The only core change is Rust 1.98's equivalent `chunks_exact(3)` → `as_chunks::<3>().0.iter()` validation iteration; matching fixes apply to measurement/tests. These changes do not change anatomy. Capture/measurement source hashes identify the pre-lint evidence. The rebuilt Wasm binary hash changed; post-lint replay of the failing spruce case reproduced all three geometry and PNG hashes exactly. Focused species/foliage/measurement tests also passed. Historical FN8 counts differ because FN9.3–9.4 intentionally separated natural/supernatural defaults and retained clipped laterals; do not attribute those changes to this QA runner.

Ordinary, Telperion and Laurelin each have inspected full-size W/B/D. Ordinary retains an upright narrow, sparse crown and flared root; the old baseline already recorded sparse/upward growth and a flared lower trunk. Telperion retains strong authored spiral and narrow irregular crown; Laurelin retains broad sweeping limbs and a wide lobed flare. Their foliage is sparse at full scale and their fine wood ends still need dedicated fidelity checks. Exact matched pre-FN9 giant images were not available here, so the age of every visible giant defect remains **unassessed**. No new geometry regression was introduced by template tuning in this session because none occurred; this is not a claim that all FN9 shared-rule visual defects are resolved.

Host: Linux x86_64 `cursor`, Intel Xeon, rustc 1.98.1; see machine record in [costs](cross-seed-costs.json). Native release build times below are one warmup plus five measured samples, same host, no affinity/frequency control. Wood bytes include Float32 positions/normals and Uint32 indices; matrices are 64 bytes per instance. Foliage prototype bytes are excluded from this subtotal, as are allocator, JS, Wasm, field and GPU domains.

| Template | Median native build ms | Nodes | Wood vertices | Wood triangles | Retained units | Wood + matrices bytes | Peak RSS KiB |
|---|---:|---:|---:|---:|---:|---:|---:|
| Ordinary | 61.92 | 11,766 | 330,668 | 644,000 | 55,438 | 19,212,064 | 27,160 |
| Telperion | 1637.32 | 177,543 | 6,637,704 | 13,053,208 | 1,360,279 | 403,001,248 | 524,436 |
| Laurelin | 809.79 | 105,032 | 2,263,118 | 4,396,768 | 802,547 | 158,439,056 | 255,700 |

For the 24 oak cases, growth+surface+foliage costs 87.6–155.3 ms; measurement-inclusive totals 342.7–528.0 ms; wood+matrix subtotal 11.29–20.21 MB. Spruce costs 302.0–346.2 ms, measurement-inclusive 962.3–1027.3 ms, 48.33–49.57 MB. These are one cold native process per species seed, not five-sample medians, and exclude browser copying/rendering. Exact stages/counts/sizes are per-case. No cross-host speedup, before/after biological repair gain, GPU frame-time or total-memory claim follows from these data.

## Acceptance disposition and next job

- Recorded fixed/fresh measurements and required source-compared mature images: complete as evidence, with failures retained.
- R2: **fail** on fresh spruce width; contextual spruce DBH remains ambiguous.
- R3–R5: public identity/determinism/anatomy paths are tested, but **fidelity fails** on healthy crown mass and foliage shape. No owner approval.
- R6: **unmet** while terminal/junction evidence remains unresolved and reference-driven architecture corrections remain outstanding.
- CPU-only numeric/capture replay: succeeds operationally; expected nonzero QA status correctly records botanical rejection. No required mature PNG is missing.
- Integrated gates: final receipts below; task completion remains blocked regardless of test success.

Next job is corrective FN-9.9 work using these same seeds: address foliage-bearing subdivision/curtain density, rounded oak outline and spruce needle taper; preserve and correct `4250668600` without relaxing the target; add uncropped terminal/socket views, and rerun the protocol plus shared-template checks after generation changes. A foliage count increase alone is not acceptance; compare the same references again. Keep the current failing images as regressions. Do not advance to make-pr or merge while the task is blocked.

## Integrated verification receipt

The initial integrated pass ran all requested gates once. Native workspace tests, actual Wasm/browser integration (`npm run rust:test:wasm` invokes `tests/browser/integration.mjs`), 106 harness tests, formatting, typecheck and build passed. Clippy exposed six existing FN9 uses of constant-size `chunks_exact`; equivalent `as_chunks` iteration fixed the validator, measurement helper and tests. Final Clippy and formatting passed; 19 focused foliage/species/measurement tests and build passed after that fix. The final Wasm/browser integration was repeated against the rebuilt module and passed, including selection, seed variation, bounds, orbit visibility and load/build recovery. No review workflow was invoked.

`node --check tests/browser/species.mjs`, `--help`, missing-output/unavailable-browser negative replay, exact 48-case numeric replay, failing-specimen three-view replay, `git diff --check` and Flow spec validation passed their expected contracts. Species numeric and capture QA correctly exit **1** for the retained failure/unassessed fidelity; they are not listed as green botanical gates. [Gate receipt](cross-seed-gates.json) retains the initial Clippy failure and subsequent successful checks with log hashes. Final integration artifacts are `/tmp/fn99-integration-final/`.

Commits: `fe1eefa` freezes fresh seeds before generation; the final QA/docs/lint/evidence commit is recorded in branch history. Flow state is **blocked**, not done. Push is authorized; PR creation and merging are not performed for this failed acceptance run.
