# FN-9.9 pass 3 — upper shedding repaired; visual fidelity remains blocked

2026-09-06. **FN-9.9 remains blocked. No PR, merge, completion or owner approval.** Production `d797508` passes all **48/48** retained numeric cases. Spruce `4250668600` measures **8.685860226 m**, below the unchanged **9.144 m** maximum. Some oak crowns improve substantially after targeted upper-support retention, but required counterexamples still fail; the spruce curtain and complete junction/contact rubric remains unmet.

## Implementation and structural diagnosis

[Scaffold audit](pass3-scaffold-audit.json) rebuilds and compares structural radius solve, local handoff and shedding for all 13 retained visual counterexamples. Highest-support positions, radii and handoff eligibility are unchanged for the six oak cases. The old `shed(..., 0.45)` removes foliage-bearing descendants despite those upper supports qualifying for local growth. Seed 1 loses 10,390 upper twig descendants and retains only 5,093; two of its five highest supports lose every twig descendant. This is a demonstrated cause of bare upper sprays, not a foliage-count hypothesis.

The spreading habit now protects local descendants rooted in the upper quarter of its structural height. Existing fine subdivisions survive, without adding another scaffold layer or disabling the lower structural shed. The regression verifies that a lower interior twig is still removed. All six audited oak counterexamples now lose zero upper twig descendants; full per-support traces and hashes remain under `/tmp/fn99-pass3-audit-*.json`, with highest twelve supports per specimen committed. Structural shell depth stays 0.45; foliage shell and placement controls are unchanged.

Spruce upper descendants were not removed by the old shed, so no blanket retention change was applied to spruce. Its descending secondary fans are narrower and longer; local descendants retain a downward, alternating departure instead of radial directions that forget their supporting axis. Pendant local run lengths increase by 35%. Spruce twig diameter changes from 5 to 2 mm; both species' terminal twig edges narrow to one quarter of their proximal radius. The ordinary colonizing law is retained. Focused regressions check continuing/departing scaffold forks, downward pendant descendants, narrowing twig ends and lower shedding. The architecture changes increase output cost; no global foliage-count, spacing, blade/needle-size or profile-range increase is used.

## Numeric replay and costs

[Numeric evidence](pass3-numeric.json) retains all 48 case results, dimensions, completion diagnostics, counts, timings and bytes; only non-gating per-axis length arrays are omitted. Raw replay: `/tmp/fn99-fix3-final-replay`. All cases complete without truncation, and every frozen seed remains present. [Source hashes](pass3-sources.json) identify native/Wasm inputs and the same six locally inspected OSU photographs. No reference image is redistributed and no new research target is substituted.

| Species | Height m | Crown width m | Individual foliage units |
|---|---:|---:|---:|
| oregon-white-oak | 16.422–21.519 | 20.229–24.633 | 255,580–541,554 |
| norway-spruce | 15.000–15.006 | 8.351–8.980 | 5,999,524–6,145,818 |

Counts increase through retained or lengthened supporting architecture. Counts are contextual, not botanical acceptance targets. Spruce DBH remains contextual/ambiguous. [Native shared-template samples](pass3-costs.json) retain one warmup and five measured samples; counts/bytes can be compared with pass 2. Timings overlap rendering and unrelated host activity and do not support a speedup claim. RSS aggregation failed after collection; the final cost receipt explicitly leaves RSS unassessed rather than inventing a value. Wood/matrix bytes exclude allocator, prototypes, JS/Wasm and GPU memory.

## Personally inspected final views

[Capture receipts](pass3-captures.json) and [per-view observations](pass3-visual.json) bind every final PNG to its exact hash. All **70** images were personally opened: 39 required mature species whole/bare/detail, 22 supplemental element/junction/exterior/peg, and nine shared-template views. Fixed 1/2/3, first three fresh seeds per species, and retained spruce `4250668600` are included. Whole/bare share bounds. Other 35 numeric specimens remain visually unassessed. Cameras use neutral materials, 960×720, DPR 1, one completed frame and Chromium 153.0.8010.12 ANGLE/Vulkan SwiftShader; no GPU throughput claim follows.

All six OSU originals were reopened from `/tmp/fn99-refs/`. These are executor judgments against the frozen rubric, not owner feedback. `P` = pass, `F` = fail, `U` = insufficient evidence. Narrow local passes are not transferred to other specimens.

| Species / seed | Silhouette | Habit | Gaps | Taper/socket | Foliage shape/attachment | Observation |
|---|---|---|---|---|---|---|
| oregon-white-oak-1 | P | P | P | U | P | Previously naked upper-left ascending spray now carries substantial foliage. Broad irregular rounded crown with internal windows; seed-1 healthy mass/gaps now pass. |
| oregon-white-oak-2 | F | P | F | U | P | Upper bare spray is now foliated, but a deep open V still divides a tall left/central mass from the right crown. Healthy rounded crown mass and gaps remain failed. |
| oregon-white-oak-3 | F | P | F | U | P | Upper centre carries foliage now, but a high stacked crown mass and separate right lobe still leave an uneven outline and broad intervening window; rounded crown distribution remains failed. |
| oregon-white-oak-2666899686 | F | P | F | U | P | Former bare high spray is now a leafy arch, but the crown retains a large upper-central opening and separated right mass. Healthy rounded mass/gaps still fail. |
| oregon-white-oak-762807349 | P | P | P | U | P | Former naked upper scaffold now forms a broad rounded leafy top. Irregular internal windows and exposed spreading limbs remain, without the prior sparse upper whiskers; silhouette and gap distribution pass this view. |
| oregon-white-oak-1444323199 | P | P | P | U | U | Previously naked central ascending spray is now a connected leafy upper mass. Broad irregular crown and internal windows now pass healthy mass/gaps in this view, with low spreading limbs retained. |
| norway-spruce-1 | F | F | F | U | U | Longer narrower pendant systems increase lower branchlet mass, but the mature crown remains transparent and fine-wood dominated, without substantial hanging foliage curtains or clear irregular tier separation. Healthy crown/curtain fidelity still fails. |
| norway-spruce-2 | F | F | F | U | U | Conical leader and greater lower branchlet mass retained, but crown remains transparent with an exposed fine framework rather than substantial hanging foliage curtains. Silhouette/tier mass, habit and gaps still fail. |
| norway-spruce-3 | F | F | F | U | U | Persistent leader and lengthened lower hanging systems remain, but fine wood is conspicuous through a transparent crown with insufficient substantial curtain mass; required crown/habit/gaps fail. |
| norway-spruce-1982700925 | F | F | F | U | U | Fresh specimen retains leader and conical framework, but transparent wood-dominated crown and inadequate hanging foliage curtain mass remain. Crown silhouette/habit/gaps fail. |
| norway-spruce-281313742 | F | F | F | U | U | Long hanging systems are visible near lower margins, but mass is still transparent and fine-wood dominated, lacking substantial foliage curtains and clear irregular tier masses. |
| norway-spruce-2271779095 | F | F | F | U | U | Transparent wood-dominated cone; lower hanging fringe does not form substantial OSU curtains. |
| norway-spruce-4250668600 | F | F | F | U | U | Retained width counterexample remains a transparent wood-dominated crown, lacking substantial curtain masses. |

## Exterior tips and rendered contact

The protocol selects actual terminal Twig IDs, parents and sockets and keeps connected original wood and foliage at unrestricted depth. A second exterior azimuth supplies an additional target, each viewed at three angles; spruce has upper/lower peg views at both targets. Two additional tighter views of the alternate contact change only the camera bounds from 45 to 15 mm half-width; their receipts retain the exact source patch/hash and verify unchanged geometry hashes. Leaf/wood occlusion remains insufficient evidence, never an inferred seam. Local depth-clipped foliage views do not diagnose terminal cuts.

The twig radius change removes the constant-radius terminal-cylinder law; final exterior judgments and any still-visible caps are recorded per view. The inspected seed-1 exterior endpoints now narrow visibly rather than showing the previous blunt cylinder cap; this is a supplemental twig-tip pass only. Both tighter peg views resolve needles but do not clearly expose woody peg relief and complete socket continuity. Full socket/contact acceptance is separate. Peg selection first matches interpolated radius, then intersects the actual rendered polygonal wood mesh along the attachment radial direction. This exposes faceting/socket differences hidden by a nearly zero centreline residual. The receipt records face index, hit point, surface distance and signed origin gap; a ray residual alone does not establish visually resolved peg anatomy.

| Spruce seed-1 peg view | Rendered origin gap mm |
|---|---:|
| peg-upper | 0.064030 |
| peg-lower | 0.064030 |
| peg-alt-upper | 0.010858 |
| peg-alt-lower | 0.010858 |
| peg-alt-close-upper | 0.010858 |
| peg-alt-close-lower | 0.010858 |

## Shared regression, gates and retained attempts

All nine Ordinary/Telperion/Laurelin geometry and PNG hashes exactly match pass 2 (and therefore pass 1). [Shared evidence](pass3-shared.json) includes personal inspection. Sparse/upright or sweeping crowns, supernatural spiral and root flare remain historical relative to this pass; their age before FN9 remains unassessed.

[Gate evidence](pass3-gates.json) records native workspace, harness 106, fmt, Clippy `-D warnings`, typecheck, build, diff, Flow validation and explicit isolated Wasm/browser results. Software fixtures remain separate from uncapped mature captures. Capture-runner exit 1 means automated visual approval is unassessed, not that a saved/verified PNG failed rendering.

The shorter-pendant first pilot remains under `/tmp/fn99-fix3-pilot`, with a separate 48-case numeric replay under `/tmp/fn99-fix3-replay`. Two inward-facing oak exterior page targets crashed; those receipts are preserved in `.previous-*` files and the established outward-looking protocol was restored (`2dfa2d2`). Final captures were replayed with final source/binary/runner hashes. A prior browser process overlapped a binding rebuild and failed after live reload; mixed log output and subsequent explicit exits are retained. A later prebuilt-server check also failed with an uninitialized core module. The final verification uses a fresh isolated server and separate log; all attempt exits remain in the gate receipt. That fresh run wrote all passing assertions and its viewer receipt, but the recorder terminated with 143 before saving the child exit. Its child exit remains unassessed; the earlier explicit exit 0 is retained with its mixed-log limitation. Numeric/cost aggregation hiccups are not generation failures; completed raw samples were preserved.

The prior authoritative report remains [REPORT-pass2.md](REPORT-pass2.md). Earlier pass-1/pass-2 artifacts and previews are retained. Final previews use `qa-preview/pass3-*`; bulk final output is `/tmp/fn99-fix3-qa`.

## Remaining blockers and next steps

**No make-pr. Keep FN-9.9 blocked.** Numeric gates alone do not satisfy R3–R6. Oak 2, 3 and 2666899686 retain separated masses/open upper windows after the proven upper-shed repair. Spruce curtain mass remains inadequate. Complete tip/socket and rendered peg acceptance remain governed by the per-view findings above; no unobserved pass or owner approval is inferred.

1. Preserve the repaired upper descendants. Trace the remaining oak crown windows to the directions and spacing of the supporting scaffold systems, rather than repeating lower/lateral infill or globally retaining more foliage. Target the retained failed specimens while keeping the newly passing crown views as regressions.
2. For spruce, measure and inspect the foliage-bearing versus bare length and spatial overlap of each descending secondary system at branch scale. Downward local directions and longer axes are now present, but their output cost has not bought substantial S-BRANCH curtains. Revise supporting hierarchy/distribution before increasing it further; do not substitute global needle count or profile relaxation.
3. Resolve surface attachment against actual socket geometry and polygonal radii. Keep exact endpoint/socket IDs and all-angle receipts; refine unobscured contact views where needed. Preserve any observed residual/cap failure, and do not treat mathematical taper or a centreline match as visual acceptance.
4. Retain every seed including 4250668600 and every frozen range. Replay 48 measurements, required mature views, actual exterior/peg views and all nine shared hashes after further production changes. No PR until every required trait passes honest inspection; never merge in this owner session.
