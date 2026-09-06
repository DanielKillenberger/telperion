# FN-9.9 corrective QA — numeric repaired, botanical fidelity still blocked

2026-09-06. **FN-9.9 remains blocked. No PR, merge, owner approval or completion claim.** The final generation at `1f2ebf4` passes all 48 retained numeric cases. Spruce `4250668600` is repaired: **8.813982965 m** crown width against the unchanged **9.144 m** maximum (previously 9.225869594 m). Healthy leaf-on crown mass and complete terminal/socket fidelity remain acceptance blockers after source comparison.

## Changes and retained counterexamples

- Spruce envelope spread .32 → .31, secondary spacing .35 → .20 m; no seed-specific branch or output clamp. The original width counterexample remains a mandatory capture even after numeric repair and now runs in the native species regression suite.
- Oak adds one scaffold subdivision (4 → 5) and one local lateral (3 → 4). Both species retain their interior foliage (shell depth .45 → 1). This retains foliage on internal leaf-bearing axes instead of stripping it for a shell presentation.
- Oak uses staggered unequal rounded elliptic lobes and a rounded terminal lobe, replacing the periodic sinusoidal outline. Nonuniform longitudinal samples resolve the tip; transverse normalization preserves the authored maximum width. Needle shaft width persists until its short distal point, replacing the long wedge. Connectors remain excluded from blade/needle dimensions.
- Evergreen needles now also attach to slender supporting branchlets, selected by both proximal and distal radius within the existing `shoot_radius` control (.025 of root radius for spruce). Needle stations are .0025 m apart, still single needles, never bundles. Alternate oak leaves and generic/supernatural placement retain their existing eligibility rules. Focused tests cover shaft width, slender-versus-thick support eligibility, unchanged alternate behavior, dimensions and retained spruce width.
- A supplemental `junction-detail` camera keeps full wood and removes local near/far clipping. It improves diagnosis without fabricating isolated joints; occlusion still prevents an unqualified taper/socket pass.

Implementation commits: `3a50326`, then `1f2ebf4` to preserve the width contract and update the analytic projected-area expectation for the new needle polygon. Initial corrective numeric and image attempts are retained under `/tmp/fn99-fix-replay`, `/tmp/fn99-fix-final-replay`, `/tmp/fn99-fix-preview` and `/tmp/fn99-fix-initial-qa`; the first aggregate was deliberately stopped before the width repair. They are not mixed into final receipts.

The original blocked [report](REPORT-before-correction.md), `cross-seed-*.json` and original `qa-preview/*.png` remain unchanged as before-fix evidence. Final corrective files use a separate prefix. [Seeds](seeds.json), [profiles](profiles.json) and [references](REFERENCES.md) are unchanged. No failing seed was dropped or resampled, no gating interval relaxed, and no generation cap substituted for maturity.

## Final protocol and numeric results

[Corrective numeric evidence](corrective-numeric.json) retains all 48 per-case checks, dimensions, counts, growth diagnostics, timing and output sizes; only non-gating per-axis branch-length arrays are omitted. Full JSONL: `/tmp/fn99-fix-verified-replay/`. Every fixed/fresh case passes its own gates and reports complete growth. Oak: height 17.573–23.096 m, crown width 21.900–25.366 m, 85,072–180,427 leaves. Spruce: height 15.000–15.005 m, crown width 8.407–8.974 m, 1,901,963–1,976,748 needles. All blade/needle dimensional gates pass. Spruce DBH remains contextual and ambiguous because the measurement includes hanging axes crossing breast height; it is not promoted to a botanical pass.

All six OSU source photographs were opened again from `/tmp/fn99-refs/` and compared, including whole/branch/foliage for both species. [Source hashes](corrective-sources.json) identify exact source, native/Wasm binary, reference and raw measurement inputs. No source photograph is redistributed.

Final bulk captures: `/tmp/fn99-fix-qa/`. [Capture receipts](corrective-captures.json) preserve exact parameters, source/geometry/PNG hashes, browser/backend, cameras and independent numeric/visual/owner fields. Whole and bare share bounds. Local foliage views retain original wood and adjacent matrices; `junction-detail` uses unrestricted depth. The automated runner always leaves visual status unassessed and is designed to exit 1; the executor's inspection below supplies the failed fidelity disposition, never owner feedback.


## Personally inspected visual rubric

All 52 final PNGs were opened: 39 required species W/B/D, two isolated elements, two unrestricted-depth junction details, and nine shared-template W/B/D. `F` = observed failure, `P` = this trait passes, `U` = insufficient evidence. These are executor judgments against the frozen rubric. [Per-view inspection hashes and notes](corrective-visual.json) bind every judgment to its PNG. The other 35 numeric specimens remain visually unassessed.

| Species / seed | Silhouette | Habit | Gaps | Taper/socket | Foliage shape/attachment | Observation |
|---|---|---|---|---|---|---|
| oregon-white-oak-1 | F | P | F | U | P | Rounded unequal lobes and alternate petiole-bearing leaves now visible. More crown mass, but upper-left bare ascending sprays and open upper centre still miss O-WHOLE. Unclipped wood reveals many overlapping axes and rounded twig ends; complete socket/taper fidelity unresolved. |
| oregon-white-oak-2 | F | P | F | U | U | Large central crown void and sparse upright top persist despite denser lateral foliage. Rounded lobes visible, but overlapping wood obscures enough attachment to withhold a combined foliage pass; clipped cuts are not seam evidence. |
| oregon-white-oak-3 | F | P | F | U | P | More foliage on left and lower lateral groups, but sparse central ascending scaffolds and disconnected-looking crown masses miss the rounded leaf-on silhouette. Rounded lobes and alternate attached blades visible; camera-cut wood cannot diagnose seams. |
| oregon-white-oak-2666899686 | F | P | F | U | P | Dense left/lower foliage still contrasts with exposed tall central scaffolds and isolated right sprays. Rounded individual blades alternate visibly along local twigs. |
| oregon-white-oak-762807349 | F | P | F | U | P | Large open centre and long exposed curving scaffolds terminate in separate foliage tufts; healthy rounded crown still absent. Rounded blades and alternate petiole attachment are visible. |
| oregon-white-oak-1444323199 | F | P | F | U | P | Broad low lateral crown is fuller, but exposed limbs, top whiskers and sparse intervening foliage remain. Clear rounded lobes with narrow petioles and alternate attachment; rounded wood ends still require dedicated socket/tip diagnosis. |
| norway-spruce-1 | F | F | F | U | U | More needles and shorter pointed tips with persistent shaft width; whole crown remains transparent and wood-dominated, lacking S-BRANCH curtains. Needles surround slender axes; full-depth detail is heavily occluded, so peg contact, upper-side forward bias and complete socket/tip fidelity remain unresolved. |
| norway-spruce-2 | F | F | F | U | U | Conical leader retained but foliage remains a transparent fine network rather than drooping leafy curtains. Needle shaft taper improved and single radial units visible; peg contact and upper-side orientation cannot be resolved confidently in this clipped view. |
| norway-spruce-3 | F | F | F | U | U | Hanging axes and radial needles visible, but no substantial foliage-bearing curtains at whole scale; trunk and regular primary tiers remain exposed. Needle shaft is fuller; crowded/clipped local geometry prevents confident peg and junction judgment. |
| norway-spruce-1982700925 | F | F | F | U | U | Whole crown remains sparse with exposed tier framework. Slender support axes now visibly bear needles; fuller shafts and separate radial units are visible. Peg/forward-bias detail and rounded twig-end fidelity remain unresolved. |
| norway-spruce-281313742 | F | F | F | U | U | Exposed regular tiers and transparent crown remain despite extra hanging shoots and needles. Single needles now clothe a slender supporting axis, but peg contact and full tip/socket fidelity are unresolved at this crop. |
| norway-spruce-2271779095 | F | F | F | U | U | Leader and hanging structural axes present, but transparent whole crown and visible primary wood still fail curtain mass. Fuller needles surround local axes; overlapping wood leaves peg/forward-bias and junction fidelity unassessed. |
| norway-spruce-4250668600 | F | F | F | U | U | Retained width regression now passes numerically, but still shares sparse transparent crown/curtain failure. Radial needles have improved shafts; peg/forward-bias and complete tip/socket fidelity remain unresolved. |


Oak's rounded unequal lobes and narrow petiole are now a clear improvement over the original repeating wave. Seed 2's obscured attachment remains unassessed rather than borrowing a pass from seed 1. Low crooked spreading limbs retain their narrow branch-habit pass. Every inspected oak still has exposed ascending reaches, separated foliage tufts or an open centre inconsistent with healthy O-WHOLE.

Spruce's isolated needle now retains a mostly parallel-sided four-sided shaft before its short point. Radial units and additional needle-bearing supporting axes are visible. Every whole crown nevertheless remains thin and wood-dominated beside S-WHOLE/S-BRANCH; 1.9 million needles do not establish curtain fidelity. Small pegs and upper-side forward bias remain insufficiently resolved for a combined foliage trait pass.

Unrestricted-depth details remove camera cuts from the selected subject depth but show substantial occlusion and rounded wood ends. They do not establish complete tip/socket fidelity. Local clipped cuts remain capture artifacts, never evidence of an actual seam. The next detail pass should select exterior twig endpoints and follow their actual parent junctions, with multiple outward-looking cameras and original connected geometry.

## Shared-template regression and costs

All nine Ordinary/Telperion/Laurelin images were inspected. **Every geometry and PNG hash exactly matches the pre-correction QA receipt**, not merely approximate counts. Thus no corrective shared-template geometry regression appears in this retained sample. Ordinary's narrow sparse upright crown and flare, Telperion's strong spiral and sparse narrow crown, and Laurelin's broad sweeping wood and wide flare all persist. Those are historical relative to this corrective work; the age of every giant defect before FN9 is still unassessed.

[Same-host native costs](corrective-costs.json) record one warmup plus five measured samples for each template, after mature capture completion while the separate browser UI retry was still active. Possible CPU contention and uncontrolled frequency/affinity make timings contextual; no speedup claim follows. Counts and bytes exactly retain the prior sample. Wood bytes include positions/normals/indices; matrices are 64 bytes per instance; prototype, allocator, JS/Wasm and GPU memory domains are excluded from the subtotal.

| Template | Median build ms | Nodes | Units | Wood + matrices bytes | Peak RSS KiB |
|---|---:|---:|---:|---:|---:|
| ordinary | 60.14 | 11,766 | 55,438 | 19,212,064 | 27,168 |
| telperion | 1670.40 | 177,543 | 1,360,279 | 403,001,248 | 524,284 |
| laurelin | 844.00 | 105,032 | 802,547 | 158,439,056 | 255,984 |


Species costs and cardinalities increased materially. Raw per-case timings were gathered during other test/capture activity, so they are not controlled before/after benchmarks. Relative to the previous numeric QA, oak rises from 29,661–59,814 to 85,072–180,427 leaves; spruce from 431,758–442,722 to 1,901,963–1,976,748 needles. Those costs buy partial anatomy/distribution improvement, not acceptance.

## Verification and disposition

[Gate receipt](corrective-gates.json) preserves initial failures and subsequent checks. Final native workspace tests pass, including retained width and focused anatomy/measurement regressions. Harness (106 tests), formatting, Clippy with `-D warnings`, typecheck, build, diff whitespace and Flow validation pass. Final source-stable Wasm/browser integration passes with explicit subprocess exit **0** in an isolated process group (`/tmp/fn99-fix-browser-isolated/`). Selection, seed variation, orbit visibility, bindings, empty/malformed output and load/build recovery assertions all pass.

The initial native pass exposed that the new sampled outline did not preserve authored maximum width. Geometry normalization fixed that contract; the analytic needle projected-polygon area expectation was updated for the fuller shaft. Initial browser attempts hit mixed live-module initialization and software-render queue stalls; an intervening build also invalidated a retry through Vite reload. These attempts are retained. Per-frame GPU completion was added to the integration test without reducing geometry or dropping orbit/UI assertions. Mature fidelity captures still use their separate one-frame runner, Chromium 153.0.8010.12, ANGLE/Vulkan SwiftShader, 960×720 DPR 1 and the existing neutral material/camera protocol. The original capture outer process returned 143 after all artifacts were saved; a clean receipt/hash replay then returned the expected 1, reusing all 52 matching captures without generation. A browser run printed every assertion passing but its outer process also returned 143; its isolated process-group replay completed with exit 0 and is recorded separately. No renderer throughput claim is made.

R2 numeric gates now pass for all 48 retained cases, with contextual DBH uncertainty preserved. R3–R6 fidelity remains unmet: healthy crown mass and spruce curtains fail; complete terminal/socket and spruce peg/orientation evidence remain unresolved. No required final PNG is missing. Flow state is **blocked**; no make-pr, PR, merge, completion or owner approval.

Next corrective work must improve placement of foliage-bearing subdivisions along the currently exposed scaffold reaches, and give spruce secondary axes substantial pendant branchlet mass. Do not substitute another global foliage-count increase for that architectural comparison. Use exterior actual twig/socket selections for uncropped multi-angle detail, retain `4250668600` plus all other seeds, and replay dimensions, images and shared-template hashes after changes. Frozen botanical ranges and the same six OSU references remain authoritative.
