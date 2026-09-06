# FN-9.9 pass 5 — local occupancy measured and redistributed; fidelity remains blocked

2026-09-06. **No PR, merge, completion or owner approval.** Production `314efc4` retains every frozen seed and target. Pass 4 is preserved in [REPORT-pass4.md](REPORT-pass4.md). This pass keeps the three protected oak outputs and the shared templates subject to exact geometry/PNG comparison; the numeric and visual results below refer to the final guarded local-only implementation.

## Implementation and diagnosis

The [oak audit](pass5-oak-audit.json) follows the retained terminal descendants of each upper structural support, recording its node/parent, original insertion, unchanged descendant count, before/after centroid and maximum endpoint displacement. Fixed half-metre world voxels show which regions gain or lose those endpoints; they are support diagnostics, not leaf area or botanical gates. Raw endpoint clouds and their hashes remain outside the checkout.

The existing pass-4 geometric scaffold-defect rule determines whether upper local redistribution is considered. No seed-ID switch was added. For those defective crowns, an upper local group pointing away from the upper support centre rotates 35 degrees toward it about its original insertion. A whole-group envelope check rejects rotations that would leave the envelope. Structural geometry, parent/run IDs, node counts and retained descendants remain intact. Lower local groups are unchanged; no lower/lateral infill or foliage-count control is added. Pass-3 upper retention remains in place.

The [spruce audit](pass5-curtain-audit.json) reconstructs the real placement-run order and verifies its complete count before retaining the original matrices and indices for a selected exterior descending system. It compares the same supporting shoot IDs across all seven required seeds. Needle-only triangle projections use a 0.5 mm system raster and 50 mm longitudinal bands; individual shoots additionally use their own tangent frame at 0.25 mm pitch. These are coverage/continuity diagnostics, not AABB-overlap proxies or acceptance thresholds. Isolated diagnostic masks exclude unrelated systems and wood; **all acceptance captures retain connected original geometry**.

Final production narrows only local descendants around their own descending structural support insertion. Their horizontal offsets decrease while the vertical distribution is retained. Every structural axis, including intermediate secondary forks, primary upturns and the leader, remains unchanged. The whole connected local group must fit the existing envelope or the transformation is rejected. No profile, global length or count parameter changes. Needle counts may decrease because supporting lengths decrease; needles remain individual anatomical units at the original station spacing.

The broader pilot compressed structural secondary systems as well as their descendants. It increased projected coverage, but flattened real intermediate forks and failed an existing structural test. That version was replaced, not accepted. An earlier boundary failure likewise led to whole-group rejection rather than clipping or relaxing the envelope. Pilot diagnostics and images retain separate provenance under `pass5-*-pilot-audit.json`, `pass5-pilot.json` and `/tmp/fn99-pass5-pilot*`.


## Final numeric and visual disposition

[Final numeric evidence](pass5-numeric.json) records **48/48 pass with explicit exit 0** at `314efc4`. Spruce `4250668600` measures **8.688966688 m ≤ 9.144 m**. Every one of the 24 oak leaf counts is unchanged from pass 4; no spruce count increases. All seeds, profile ranges and source references remain byte-identical.

| Species | Height m | Crown width m | Individual foliage units |
|---|---:|---:|---:|
| Oregon white oak | 16.422–20.400 | 20.414–24.635 | 327,275–541,606 |
| Norway spruce | 15.000–15.006 | 8.341–8.977 | 5,121,018–5,240,583 |

Counts remain contextual; spruce DBH remains ambiguous/contextual. [Same-host native costs](pass5-costs.json) record one warmup and five measured samples per shared template, counts, buffer sizes and peak RSS after browser completion. Timings are contextual because unrelated host activity is uncontrolled; no native speedup or GPU throughput claim is made. Historical pass-4 cost receipts remain available.

All **84** final PNGs were personally inspected: 39 required mature species whole/bare/detail views, 36 supplemental anatomy/peg/socket/curtain views and nine shared-template views. The other 35 numeric specimens remain visually unassessed. [Protected oaks](pass5-protected-oaks.json) retain exact pass-3 geometry and whole/bare/detail PNGs for `1`, `762807349`, `1444323199`. [All nine shared views](pass5-shared.json) retain exact pass-3 geometry and PNG hashes. Their sparse/upright or sweeping crowns, spiral and flare remain historical relative to this pass, not new regressions; their pre-FN9 age remains unassessed.

`P` = pass, `F` = fail, `U` = insufficient evidence. These are executor judgments, not owner feedback.

| Species / seed | Silhouette | Habit | Gaps | Taper/socket | Foliage shape/attachment | Observation |
|---|---|---|---|---|---|---|
| oregon-white-oak-1 | P | P | P | U | P | Protected accepted crown retained: exact pass-3 geometry and whole PNG hashes. |
| oregon-white-oak-2 | F | P | F | U | P | Upper descendants move without repairing the elevated left mass and large right window. |
| oregon-white-oak-3 | F | P | F | U | P | Upper cap remains stacked above the lower left mass, with a distinct right low lobe. |
| oregon-white-oak-2666899686 | F | P | F | U | P | Upper arch still encloses a large open window above the right crown. |
| oregon-white-oak-762807349 | P | P | P | U | P | Protected accepted crown retained: exact pass-3 geometry and whole PNG hashes. |
| oregon-white-oak-1444323199 | P | P | P | U | U | Protected accepted crown retained: exact pass-3 geometry and whole PNG hashes. |
| norway-spruce-1 | F | F | F | U | U | Leader/upturn and narrower pendants retained; transparent wood-dominated crown still fails curtains. First-target peg-clear-front is a narrow local pass; full peg/socket remains unresolved. |
| norway-spruce-2 | F | F | F | U | U | Leader and hanging systems retained; crown remains a transparent fine network without substantial curtain mass. |
| norway-spruce-3 | F | F | F | U | U | Narrow hanging strands and leader retained; exposed primary tiers and transparent fine wood still fail curtain mass. |
| norway-spruce-1982700925 | F | F | F | U | U | Lower branches and leader retained; thin hanging strands, exposed tiers and transparent crown still fail curtains. |
| norway-spruce-281313742 | F | F | F | U | U | Lower tiers and hanging systems retained; transparent fine-wood crown still lacks needle-clothed curtain mass. |
| norway-spruce-2271779095 | F | F | F | U | U | Leader and tiers retained; transparent fine-wood hanging strands still fail curtain mass. |
| norway-spruce-4250668600 | F | F | F | U | U | Retained width counterexample passes at 8.688967 m; transparent woody crown still lacks substantial curtains. |

The oak trace retains 10,271 / 10,007 / 9,909 upper terminal descendants in the failed cases. Of those, 8,182 / 7,459 / 7,377 endpoints move. Occupied half-metre upper voxels decrease from 2,026→1,793 / 2,466→2,231 / 2,238→2,018. Entering some new cells while compacting existing masses does not bridge the inspected windows. The protected cases have zero changed upper twig endpoints. **Oak occupancy repair remains incomplete.**

Final same-shoot spruce projected coverage rises from roughly **2.4–3.1% to 6.1–10.3%**. Individual bearing runs still show sparse transverse needle coverage. The final whole and branch images remain transparent and wood-dominated; **all seven required spruce crowns still fail curtain fidelity**. The larger structural-compression pilot values are not substituted for this final result.

The first target remains endpoint **21454**, parent **8543**, socket **327**; its selected final instance is **1853593** (pass 4: 1884715). The alternate remains **48474 / 37208 / 21588**, with its actual new instance recorded in every receipt. The first `peg-clear-front` resolves a short projecting connector locally. Other angles still obscure its base. The root/tip socket front views show near-side connected wood without a visible empty gap, but overlapping needles and supporting faces prevent complete socket acceptance. These narrow observations do not establish the complete first target. No hidden seam is inferred and no occluded angle acquires a pass.

## Captures and target identity

Final bulk captures are `/tmp/fn99-pass5-qa`; durable previews use `qa-preview/pass5-*`. [Capture receipts](pass5-captures.json) retain original geometry hashes, PNG hashes, cameras, source/binary versions, browser/backend and independent numeric/visual/owner fields. The same six OSU originals in `/tmp/fn99-refs` were reopened personally; their source hashes are in [pass5-sources.json](pass5-sources.json). No reference photograph is redistributed.

`--targets` pins the actual pass-4 endpoint, parent and socket IDs and fails on a topology mismatch. Both targets retain their exterior and upper/lower/contact angles. Three additional 3 mm half-width `peg-clear-*` views attempt tangential profiles of the first connector. Six 10 mm half-width `socket-root-*` / `socket-tip-*` views inspect both actual junctions with explicit incoming/departing node IDs. The view names do not certify visibility. Each receipt records the actual new instance index and polygonal contact; a changed placement index is not silently relabelled as the old needle. [Contact observations](pass5-contacts.json) govern visual acceptance. A near-zero residual, mathematical taper, overlapping foliage or a hidden socket does not establish a visible seam or a pass.

The capture-only `--batch-instances` path submits every original retained matrix exactly once in ordered draws of at most 100,000 instances, with the same prototype, neutral material and depth. [Paired parity](pass5-batch-parity.json) verifies exact geometry and PNG equality for oak whole and spruce branch captures. Conservative offscreen detail culling retains the pass-4 protocol. Full/bare bounds match; mature dimensions and counts are not capped. Captures use 960×720, DPR 1 and one completed neutral frame on headless Chromium/SwiftShader. No GPU speed or throughput claim follows.

## Attempts and verification limits

Two original spruce whole-view pilot workers crashed while other heavy work was active. Their logs and process receipts remain; failed or missing views never supply a pass. Final heavy work is serialized and uses ordered draw batches. The first 48-case recorder terminated with exit 143 after writing its per-case results but before saving its child's final exit; those raw results remain pilot evidence. A subsequent detached replay has explicit exit 0, but still belongs to the structural-compression version rejected by the fork test. Only the final `314efc4` replay supplies the final numeric claim.

The reused Vite integration attempt exited 1 with `Tree core is not initialized`: the served skeleton module referenced a timestamped core module while the test initialized an unversioned import. A clean standard `npm run rust:test:wasm` replay builds and starts its own fresh server, avoiding this duplicate-module state; both process receipts remain in the gates evidence. No production or integration-test code was changed for the retry.

[Software receipts](pass5-gates.json) distinguish both observed structural failures, their fixes, the successful final native workspace run, 106 harness tests, fmt, Clippy `-D warnings`, typecheck, build, isolated Wasm/browser, diff and Flow validation. Rendered mature fidelity is independent of the smaller integration fixtures.

## Remaining blockers and next steps

**Keep FN-9.9 blocked. No make-pr, PR, merge, completion or owner approval.**

- Preserve the exact protected oak outputs, all upper descendants and all frozen cases. The three failed oak crowns still have inter-mass windows. Centre-directed rotations compacted their existing upper groups; next target measured empty inter-mass regions and check each connected group's feasible reach before proposing another redistribution. Do not repeat a centre-mean turn or add lower infill/count.
- Preserve the spruce structural forks and the width counterexample. Narrowing supporting distributions must yield substantial S-BRANCH needle-clothed curtains in intact views; a better projected-coverage percentage alone is insufficient. Diagnose the remaining longitudinal extent, overlap and needle-versus-wood projected coverage on the same connected local shoots before changing any count or length allocation. Preserve structural forks; compressing those already failed an existing invariant.
- Preserve both endpoint/socket identities and all-angle receipts. Continue unresolved peg relief/socket continuity from a demonstrably unobscured connected view; never transfer a narrow local pass to the complete target or other angles.
- Replay numeric, mature visual and protected/shared hashes after any further production change. Open a PR only after a complete honest visual pass; never merge under this owner instruction.
