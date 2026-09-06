# FN-9.9 pass 4 — targeted scaffold and surface repairs; fidelity remains blocked

2026-09-06. **No PR, merge, completion or owner approval.** Production `99112c4` passes **48/48** retained numeric cases. Spruce `4250668600` is **8.685199801 m** wide against the unchanged **9.144 m** maximum. All three accepted oak crowns (`1`, `762807349`, `1444323199`) retain exact pass-3 geometry and whole PNG hashes. The remaining oak crown cases and spruce curtain mass still fail visual fidelity.

## What changed and what the audits establish

The [oak scaffold audit](pass4-oak-audit.json) traces terminal-support distributions before and after correction. A displaced scaffold is redistributed only when its outer terminal mass crowds a neighbour's azimuth; near-central ascending systems are excluded because their azimuth is unstable. An overextended upper scaffold rotates downward about its original trunk socket. Both operations move connected systems without adding nodes, clipping tips or changing parent links. The retained failed specimens change 468 / 488 / 478 structural positions for seeds 2 / 3 / 2666899686, respectively. All structural positions and parents of the three passing specimens remain unchanged, protected by a pass-3 golden regression. [Full output comparison](pass4-protected-oaks.json) also verifies the accepted specimens' final surface, indices, element, placement and whole PNG hashes. This is a geometric defect rule, not a seed-ID switch or a profile revision. Pass-3 upper-descendant retention remains in place.

The [spruce branch audit](pass4-curtain-audit.json) measures retained supporting length, foliage eligibility and overlapping bearing bounds for all seven required spruce specimens. Before this pass, about 99% of each descending system's aggregate supporting length was already needle-bearing. Bare supporting length is unchanged (145–154 m summed over 780–799 systems per specimen). Recomputing the hanging plane from each child's direction turned successive forks across one another. Local descendants now inherit one vertical plane from their structural support and use narrower lateral departures; the primary, leader, secondary lengths, needle stations, prototype dimensions and shell controls are unchanged. Summed bearing length decreases from 12.74–13.05 km to 10.79–11.08 km, and median overlapping bearing AABBs decreases from 20–21 to 13–15. Those are spatial diagnostics, not biological targets or a visual pass. No further global foliage-count or length increase was applied. Final individual needles decrease to 5.21–5.33 million from pass 3's roughly 6 million.

`foliage::place_on_surface` now places needles against the same float32 polygonal sweep used for rendering, including bent rings, socket sinking and swelling. It queries rings without generating mesh indices or normals. A perpendicular ray at a station near a finite bent sweep can miss that sweep; in that case the intended circular origin is projected onto the actual adjacent facets, never silently restored to the circular model or dropped. This correction repairs the retained numeric contact failures. Tests check attachment to emitted facets on three-, seven- and twenty-sided bent/socket fixtures, including a lobed surface. Generic and alternate placement retain their existing API behavior; both the browser engine and native species measurement use the surface-aware needle path.

The 15 mm upper/lower peg frames select an outward-facing unit. Additional 6 mm contact frames use the selected radial/tangent at three angles, at both exterior targets. Every receipt retains the actual endpoint, parent, socket, instance, camera and rendered facet intersection. The branch-scale view similarly frames an actual descending structural secondary. Connected original wood and foliage remain intact at unrestricted depth. Near-zero residuals do not establish visible peg relief or socket continuity; occlusion remains insufficient evidence.

## Numeric replay and provenance

[Numeric evidence](pass4-numeric.json) records every seed and explicit exit 0 for `/tmp/fn99-fix4-final-replay`. Only non-gating per-axis length arrays are omitted from the compact receipt. No frozen seed, range or reference was changed. [Source hashes](pass4-sources.json) bind native/Wasm inputs and the same six locally inspected OSU photographs; no reference pixels are redistributed.

| Species | Height m | Crown width m | Individual foliage units |
|---|---:|---:|---:|
| oregon-white-oak | 16.422–20.437 | 20.414–24.635 | 327,275–541,606 |
| norway-spruce | 15.000–15.006 | 8.345–8.977 | 5,211,227–5,332,038 |

Counts remain contextual. Spruce DBH remains ambiguous/contextual. [Native costs](pass4-costs.json) preserve one warmup and five measured samples for each shared template, plus per-process VmHWM. Concurrent rendering and unrelated host activity prevent a speedup or GPU-throughput claim. Aggregation initially referenced a missing historical machine key; the successful raw samples and native RSS output were preserved and aggregated without rerunning or inventing data.

## Personally inspected final views

[Capture receipts](pass4-captures.json) bind all **75** final PNGs to original output hashes, exact cameras and source versions. [Per-view observations](pass4-visual.json) record personal inspection of every image: 39 required mature whole/bare/detail views, 27 supplemental element/junction/exterior/peg/curtain views, and nine shared-template views. The same six OSU originals were personally reopened. Other 35 numeric specimens remain visually unassessed. These are executor judgments, not owner feedback.

All cameras use neutral materials, 960×720, DPR 1 and one completed frame on Chromium 153.0.8010.12, ANGLE/Vulkan SwiftShader. Whole/bare bounds match. A capture-only conservative frustum optimization skips provably offscreen foliage in detail frames; it preserves original geometry, visible occluders, instance order, full wood, cameras and depth. Three [paired comparisons](pass4-frustum-equivalence.json) have exact PNG and geometry parity. Authentic earlier whole receipts retain their earlier runner hash; later receipts identify `4af35df`. No capture is relabelled as a different source version.

`P` = pass, `F` = fail, `U` = insufficient evidence. Narrow supplemental observations are not transferred to other specimens.

| Species / seed | Silhouette | Habit | Gaps | Taper/socket | Foliage shape/attachment | Observation |
|---|---|---|---|---|---|---|
| oregon-white-oak-1 | P | P | P | U | P | Accepted broad rounded crown and gaps retained; exact pass-3 geometry and whole PNG. |
| oregon-white-oak-2 | F | P | F | U | P | Rotated scaffold bridges part of the V, but tall left and separate right masses still fail healthy rounded distribution. |
| oregon-white-oak-3 | F | P | F | U | P | Upper scaffold lowered, but stacked central/left crown and separate lower right lobe remain. |
| oregon-white-oak-2666899686 | F | P | F | U | P | Redistribution changes the outer mass, but the upper arch still encloses a large open window. |
| oregon-white-oak-762807349 | P | P | P | U | P | Accepted rounded top, internal windows and low spreading limbs retained; exact pass-3 geometry and whole PNG. |
| oregon-white-oak-1444323199 | P | P | P | U | U | Accepted broad connected crown retained exactly; foliage attachment remains obscured in the local detail. |
| norway-spruce-1 | F | F | F | U | U | Leader and vertical pendant systems retained, but transparent fine wood still dominates; substantial hanging curtain mass fails. Alternate-contact left resolves a peg only as a narrow supplemental pass. |
| norway-spruce-2 | F | F | F | U | U | Leader and vertical pendant systems retained, but transparent fine wood still dominates; substantial hanging curtain mass fails. |
| norway-spruce-3 | F | F | F | U | U | Leader and vertical pendant systems retained, but transparent fine wood still dominates; substantial hanging curtain mass fails. |
| norway-spruce-1982700925 | F | F | F | U | U | Leader and vertical pendant systems retained, but transparent fine wood still dominates; substantial hanging curtain mass fails. |
| norway-spruce-281313742 | F | F | F | U | U | Leader and vertical pendant systems retained, but transparent fine wood still dominates; substantial hanging curtain mass fails. |
| norway-spruce-2271779095 | F | F | F | U | U | Leader and vertical pendant systems retained, but transparent fine wood still dominates; substantial hanging curtain mass fails. |
| norway-spruce-4250668600 | F | F | F | U | U | Leader and vertical pendant systems retained, but transparent fine wood still dominates; substantial hanging curtain mass fails. Retained width counterexample passes numerically. |

## Actual contact and all-angle limits

[Contact receipts](pass4-contacts.json) preserve exact node, parent, socket and instance IDs at both selected targets. Each target has upper/lower views and three local contact angles, in addition to its complete exterior terminal/socket views. The first contact's local signed origin gap is about **−0.000092 mm**; the alternate is about **+0.000045 mm**, at float32 precision. These replace pass-3 residuals of roughly 0.011–0.064 mm. They establish improved surface placement, not visual peg acceptance.

The alternate contact's left angle resolves a small projecting connector between twig and needle, a narrow supplemental peg pass. The first target and other obscured angles do not acquire that pass. Full branch/socket continuity remains governed by each exterior observation; no seam is inferred from overlapping foliage, and a camera-clipped foreground object does not diagnose a real terminal cut. The pass-3 terminal narrowing law is retained.

## Shared regression and software gates

[Shared comparisons](pass4-shared.json) retain all nine Ordinary/Telperion/Laurelin geometry and PNG comparisons with pass 3. Their sparse/upright or sweeping crowns, supernatural spiral and root flare remain historical relative to this pass; their age before FN9 remains unassessed. They were personally inspected again.

[Gate evidence](pass4-gates.json) records native workspace tests, all 106 harness tests, fmt, Clippy `-D warnings`, typecheck, build, diff checks and Flow validation. The isolated Wasm/browser run now has explicit **exit 0** and separate clean logs, resolving the earlier recorder-143 caveat. Recorder and child use separate sessions and file-backed output, so the child's actual exit survives the invoking tool shell. Software fixtures remain separate from uncapped mature fidelity captures.

The first numeric attempt exposed finite bent-sweep ray misses; its failures remain in `/tmp/fn99-fix4-replay`, and the final corrected replay passes all 48. Initial concurrent browser page crashes remain in previous receipts/logs. The concurrent scheduler was deliberately stopped, active detached workers allowed to finish, and missing views retried serially. The serial scheduler was then stopped between workers to add the verified offscreen optimization; both deliberate termination receipts remain. Final worker exits and all PNG hashes are checked independently. Rendering success never awards automated visual approval.

The first broad upper-fork steering pilot is retained under `/tmp/fn99-fix4-pilot`; it did not sufficiently repair the failed crowns and was replaced by the targeted scaffold rule. `/tmp/fn99-fix4-pilot2` retains subsequent spacing/contact pilots and the early interior branch camera. Final bulk output is `/tmp/fn99-fix4-qa`; durable previews use `qa-preview/pass4-*`. [REPORT-pass3.md](REPORT-pass3.md) preserves the previous authoritative report.

## Remaining blockers and next steps

**Keep FN-9.9 blocked. No make-pr, PR, merge, completion or owner approval.** Numeric success and the software gates do not satisfy R3–R6 while crown and attachment fidelity remain incomplete.

1. Preserve all three exact accepted oak outputs and upper-descendant retention. The targeted scaffold rotations alone do not repair the stacked masses/open windows of 2, 3 and 2666899686. Next trace how each upper scaffold's connected local descendants occupy the space between those masses, before changing any foliage count or adding lower infill.
2. Preserve spruce leader/upturn, dimensions, every seed and all frozen ranges. Stable pendant planes reveal long, mostly needle-bearing supports but still lack substantial curtain mass. Measure transverse needle coverage and continuity along individual supporting shoots at S-BRANCH scale; AABB overlap and aggregate supporting length already fail to explain visual mass by themselves. Rework the hierarchy/distribution on that evidence, without a further global length/count increase.
3. Preserve both exact contact targets and all-angle receipts. The polygonal origin correction and one visible alternate peg are partial results. Complete first-target peg relief and branch/socket continuity still need unobscured connected views; occlusion and zero residual must not be promoted to acceptance.
4. Replay all 48 retained cases, required mature images and shared hashes after any further production changes. Retain 4250668600, source references and existing counterexamples. Open a PR only after an honest complete visual pass; never merge under this owner instruction.
