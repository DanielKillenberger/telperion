# FN-9.9 pass 6 — measured empty reach and longitudinal redistribution; fidelity remains blocked

2026-09-06. **No PR, merge, completion or owner approval.** Production `9e1e889` keeps the frozen seeds and profiles. [Pass 5](REPORT-pass5.md) is preserved verbatim. This report separates software/numeric success from inspected botanical fidelity.

## Production changes and measured limits

Oak uses the existing scaffold-defect trigger, with no seed-ID dispatch. Upper connected local groups now target half-metre empty endpoint cells bracketed by crown mass on two world axes within 3 m. Each candidate must be within the group's maximum insertion-to-endpoint reach. The algorithm scores actual terminal endpoints reaching empty cells, validates the whole rigid group against the envelope and upper region, and reserves filled cells before selecting the next group. No crown mean is used. Structural insertions, run IDs, lengths and descendants remain intact; lower groups are unchanged. The protected specimens do not enter this repair.

The [reach audit](pass6-reach-before.json) is an optimistic per-support bound, not proof a rigid group fits or that a cell contains enough leaf area. The [endpoint comparison](pass6-oak-audit.json) records 10,271 / 10,007 / 9,909 retained upper descendants for 2 / 3 / 2666899686. Relative to pass 5, 8,429 / 7,862 / 7,781 endpoints change and occupied half-metre upper voxels increase 1,793→2,086 / 2,231→2,573 / 2,018→2,429. This includes replacing the previous centre-directed rotations; it does not mean every moved endpoint fills a visible window. The inspected crowns still fail.

[Verified empty-pixel rays](pass6-window-rays.json) distinguish the visible windows from nearby foliage. At the sampled empty region in seed 2, the closest retained upper support falls about 2.960 m short even under an optimistic maximum-radius test. Leaves are excluded from that radius; their allowed extent cannot close a metre-scale shortfall. Seeds 3 and 2666899686 have potentially reachable supports, but feasibility is only necessary and the current cell selection does not close their visible windows. These camera rays are diagnostics, not camera-dependent production tuning or a volumetric proof.

Spruce retains pass-5 transverse narrowing and moves branched local fans to earlier insertions along the same uninterrupted descending structural support. The whole fan translates, preserving edge lengths; movement stops before a structural fork, is limited by the fan's measured reach, and requires envelope containment. Single terminal groups stay attached to their original parents. No structural position, leader or primary upturn is compressed. The final radius solve responds to the new local insertions; the wood mesh is not claimed byte-identical. Reattachment adds roughly 0.5% wood triangles through changed surface paths while anatomical node and individual needle counts remain unchanged.

The [curtain audit](pass6-curtain-audit.json) projects original needle triangles and original wood side triangles separately, in the same local frame. It reports longitudinal bands, overlap and wood-only/needle-only area. Wood end caps are excluded and masks are clipped to needle projection bounds. Original bearing node IDs are pinned across reattachment: selecting only descendants of the old sampled edge would discard some moved shoots and falsely exaggerate the improvement. That unpinned diagnostic remains separate under `/tmp/fn99-pass6-final-audit`; the authoritative comparison uses `/tmp/fn99-pass6-pinned-audit` and retains all sampled needles.

Same-shoot projected coverage rises from about 6.1–10.3% to 9.2–11.7%; longitudinal spans decrease without lengthening or compressing structural axes. The percentages do not supply visual acceptance, and the remaining empty projected area is substantial. No global foliage-count/length control, profile range, source reference or seed was changed.

## Numeric and software evidence

[Final replay](pass6-numeric.json): **48/48 pass, explicit exit 0**. Spruce `4250668600` remains **8.688966688 m ≤ 9.144 m**. [Per-case parity](pass6-count-parity.json) verifies identical individual foliage-unit and anatomical node counts for every one of the 48 cases. Oak mesh counts also remain unchanged. Spruce mesh tessellation changes are retained explicitly.

| Species | Height m | Crown width m | Individual foliage units |
|---|---:|---:|---:|
| Oregon white oak | 16.422–19.976 | 20.414–24.635 | 327,275–541,606 |
| Norway spruce | 15.000–15.006 | 8.341–8.977 | 5,121,018–5,240,583 |

Foliage totals and spruce DBH remain contextual. [Same-host per-case costs](pass6-costs.json) retain native timings and output bytes; host load was uncontrolled and no speedup or software-GPU throughput claim is made.

[Software gates](pass6-gates.json) pass: full native workspace, harness 106, fmt, Clippy `-D warnings`, typecheck, build, clean isolated Wasm/browser with explicit exit 0, diff and Flow validation. The initial native run caught an empty-tree slice panic in the new curtain routine; `9e1e889` fixes it and both the existing growth regression and full suite pass. Earlier audit compile/Clippy/invocation failures are recorded separately. No review matrix or agent/CLI review was invoked.

## Inspected visual disposition

All **120 final PNGs** were personally inspected: 39 required mature views (six oaks and seven spruces, whole/bare/foliage), 36 original supplemental views, 36 new first-target azimuth views and nine shared-template views. The other 35 numeric specimens remain visually unassessed. Capture and scan recorders both exited 0. See [per-view observations](pass6-visual.json), [capture provenance](pass6-captures.json) and [OSU reference hashes](pass6-sources.json).

Protected oaks **1 / 762807349 / 1444323199** retain exact pass-3 geometry and all nine whole/bare/detail PNGs ([comparison](pass6-protected-oaks.json)). Ordinary/Telperion/Laurelin also retain exact geometry and all nine pass-3 PNGs ([comparison](pass6-shared.json)); their historical sparse foliage and sweeping forms remain, with no new shared regression.

P = inspected trait pass; F = observed failure; U = unresolved. A P in one column does not accept another trait or the entire specimen. Spruce habit includes substantial S-BRANCH curtains; retaining a leader alone cannot pass it.

| Specimen | Silhouette | Habit | Gaps | Taper/socket | Foliage |
|---|---|---|---|---|---|
| oregon-white-oak-1 | P | P | P | U | P |
| oregon-white-oak-2 | F | P | F | U | P |
| oregon-white-oak-3 | F | P | F | U | P |
| oregon-white-oak-2666899686 | F | P | F | U | P |
| oregon-white-oak-762807349 | P | P | P | U | P |
| oregon-white-oak-1444323199 | P | P | P | U | U |
| norway-spruce-1 | F | F | F | U | U |
| norway-spruce-2 | F | F | F | U | U |
| norway-spruce-3 | F | F | F | U | U |
| norway-spruce-1982700925 | F | F | F | U | U |
| norway-spruce-281313742 | F | F | F | U | U |
| norway-spruce-2271779095 | F | F | F | U | U |
| norway-spruce-4250668600 | F | F | F | U | U |

All three targeted oaks still show separated elevated masses and open inter-mass windows. All seven spruces remain transparent and wood-dominated, with localized needle bands and long exposed woody spans. Complete connected socket anatomy remains unresolved. The protected oak foliage-detail for 1444323199 remains screened by crossing wood. No owner feedback or approval is recorded.

Bulk: `/tmp/fn99-pass6-qa` and `/tmp/fn99-pass6-contact-scan-final`. Durable previews: `qa-preview/pass6-*.png`. Captures use repo Playwright, bundled headless Chromium and software SwiftShader; renderer, camera, fixed target and source/Wasm hashes are recorded per image. Geometry and source assertions passed for every final capture.

## Peg/socket identity and visibility

All original exterior, upper/lower, contact and socket angles retain first target **21454 / 8543 / 327** and alternate **48474 / 37208 / 21588**. An additional 36 images sweep 12 azimuths each around the first peg, root junction and distal junction. The original connected scene, neutral materials and unrestricted depth remain; conservative offscreen instance culling does not remove on-screen occluders.

[Contact observations](pass6-contacts.json) are per view. The narrow first-target `peg-clear-front` relief remains local evidence only. Other azimuths hide the selected contact behind wood or needles. [Exact departing-node projections](pass6-target-projections.json) were checked when a neighbouring shoulder could be mistaken for the target. A visible neighbouring branch, a nearly zero residual or a centreline continuation is not promoted to a seam. Complete first-target peg/socket acceptance remains unresolved; no local pass transfers to the alternate target or other angles.

## Remaining blockers and next steps

**Keep fn-9.9 blocked. No make-pr, PR, merge, completion or owner approval.**

- Preserve protected oak hashes, all descendants and counts. Seed 2's sampled main window is outside unchanged upper-group reach; another upper rigid rotation cannot fill it. Any next architecture proposal must address that measured reach limit while retaining the no-lower-infill/count constraints. For 3 and 2666899686, improve the spatial identification and whole-group feasibility of the actually visible inter-mass regions; endpoint-voxel gains alone have failed visual QA.
- Preserve spruce structural forks, leader/upturn and width counterexample. Moving local fans upstream modestly improves overlap but does not establish S-BRANCH curtain mass. Diagnose allocation of the existing needle-bearing runs within each whole secondary system; preserve original node membership when comparing moved shoots. Do not repeat a global count/length bump or compress structural forks.
- Preserve exact first/alternate target identities. Azimuth sweeps still leave the selected surfaces obscured; test visibility through elevation as well as azimuth while retaining connected original polygons. Do not infer a hidden seam or accept a neighbouring junction.
- Repeat frozen numeric, protected/shared hashes and personally inspected mature views after further production changes. Only a complete honest AC pass permits make-pr; merging remains forbidden.
