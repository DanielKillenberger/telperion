# FN-9.9 pass 2 — architectural infill retained, visual fidelity still blocked

2026-09-06. **FN-9.9 remains blocked. No PR, merge, owner approval or completion claim.** All 48 retained numeric cases pass at generation `3f3533c`; spruce `4250668600` measures **8.621604388 m**, below the unchanged **9.144 m** maximum. The architecture changes add intermediate oak lateral systems and spruce pendant subdivisions. They improve some crown mass, but do not meet the frozen healthy-crown/curtain rubric.

## Implementation and scope

- Oak retains its original scaffold random stream, then adds lateral systems at intermediate stations (approximately 40% and 70%) along nonterminal scaffold reaches. Each receives a shorter forked continuation. This supplies actual connected foliage-bearing subdivisions between the existing end forks.
- Spruce retains the existing primary/secondary random stream, then grows alternating pendant axes along descending secondaries. Their lengths decrease toward the secondary tip. This is a change in supporting architecture, not needle spacing or a global foliage-count parameter.
- Leaf/needle geometry, attachment rules, spacing, foliage shell depth, species envelopes, profile ranges and every recorded seed remain unchanged from pass 1. Numeric crown dimensions can still change through the shared radius/local-branch solve after architecture changes.
- A regression checks intermediate forks with both a continuing axis and a departing side axis, including descending secondary forks for spruce. This is an engineering invariant, not a botanical branch-count target. The existing fixed species regressions retain `4250668600`.
- The QA runner selects a real exterior terminal Twig node between 25% and 80% of tree height, follows its parent and grandparent socket, and uses three cameras looking outward through the crown. All original wood and foliage remain; no local near/far clipping or fabricated isolated joint. Spruce upper/lower needle views select a matrix origin matching the actual selected twig's interpolated surface radius within 0.00001 m. Node IDs, positions, attachment residuals, camera parameters and hashes are in receipts. Intact foreground geometry can still occlude a view.

Implementation: `3f3533c`. Exterior protocol: `54702f2`, tightened by `0d608f6`. Browser fixtures: `0d608f6` and `972b3bd`. REVIEW_MODE=none was honored: no review agents, external agents, Claude CLI, approval claim or merge.

The prior report is preserved as [REPORT-pass1.md](REPORT-pass1.md); all original and `corrective-*` evidence and `fix-*.png` previews remain. Final pass-2 evidence uses `pass2-*`. The provisional camera run is retained at `/tmp/fn99-fix2-camera-initial`; pilot images are at `/tmp/fn99-fix2-pilot`. Provisional images do not supply unearned final passes.

## Numeric replay and costs

[Pass-2 numeric evidence](pass2-numeric.json) contains all 48 per-case checks, actual dimensions, counts, completion diagnostics, timings and bytes. Only non-gating per-axis branch-length arrays are omitted. Full JSONL is retained under `/tmp/fn99-fix2-replay/`. No numerical failure was dropped or resampled, and no gating range was relaxed. All cases complete without truncation. Spruce DBH remains contextual and ambiguous, not a botanical pass.

| Species | Height m | Crown width m | Retained individual units |
|---|---:|---:|---:|
| Oregon white oak | 15.455–21.492 | 20.269–24.662 | 152,191–396,747 |
| Norway spruce | 15.000–15.006 | 8.314–8.859 | 3,322,744–3,432,194 |

All blade/needle dimensional gates pass. Counts increased from pass 1 (oak 85,072–180,427; spruce 1,901,963–1,976,748), despite unchanged unit placement controls, because the supporting architecture changed. The extra geometry buys partial distribution improvement, not acceptance. Oak's upper naked sprays are more conspicuous in several specimens after infill; this is not a uniform visual improvement.

[Source and binary hashes](pass2-sources.json) identify the implementation, native/Wasm binaries, raw numeric inputs and the same six OSU source photographs. All six originals were opened again from `/tmp/fn99-refs/`; no source photograph is redistributed. Native timings include measurement overhead and concurrent browser activity; they do not support a speedup or GPU throughput claim.

## Personally inspected visual rubric

[Capture receipts](pass2-captures.json) retain all **60** required/supplemental/shared-template PNGs, parameters, geometry/PNG hashes and Chromium 153.0.8010.12 ANGLE/Vulkan SwiftShader identity. Whole/bare bounds are shared. Resolution is 960×720, DPR 1, neutral materials and one completed frame. All 39 required species whole/bare/detail views, 12 supplemental element/junction/exterior/peg views, and nine shared-template views are present.

[Per-view inspection](pass2-visual.json) binds every final PNG hash to executor observations. Five unchanged oak seed-1 views were first inspected during the provisional camera run and verified byte-identical in the final run; the remaining final files were opened directly. The other 35 numeric specimens remain visually unassessed. `P` = pass, `F` = fail, `U` = unassessed. These are executor judgments against the same six source images, not owner feedback.

| Species / seed | Silhouette | Habit | Gaps | Taper/socket | Foliage shape/attachment | Observation |
|---|---|---|---|---|---|---|
| oregon-white-oak-1 | F | P | F | U | P | Centre and lower crown substantially fuller; tall upper-left bare ascending sprays still break healthy rounded crown and leave a sparse upper centre. |
| oregon-white-oak-2 | F | P | F | U | U | Fuller lower and lateral masses but a deep open upper centre separates two foliage groups; tall bare central shoots remain. |
| oregon-white-oak-3 | F | P | F | U | P | More leaf mass below, but isolated top tuft and bare central ascending scaffolds remain; healthy rounded crown still fails. |
| oregon-white-oak-2666899686 | F | P | F | U | P | Dense lower-left and intermediate foliage but tall bare central shoots and isolated right foliage system still fail healthy crown mass distribution. |
| oregon-white-oak-762807349 | F | P | F | U | P | Lower and left crown masses fuller, but central top is a bare scaffold system and separate right foliage masses leave excessive gaps. |
| oregon-white-oak-1444323199 | F | P | F | U | P | Substantial low broad leaf mass, but a distinct bare ascending central spray protrudes above the canopy and leaves upper mass incomplete. |
| norway-spruce-1 | F | F | F | F | U | More foliage-bearing subdivision fills some former transparent space, but the crown remains a fine wood-dominated network, lacking substantial pendant foliage curtains. Exterior-right exposes actual blunt terminal caps. Upper-side forward inclination visible; peg contact still unresolved. |
| norway-spruce-2 | F | F | F | U | U | Leader and conical outline remain, but crown mass is transparent and fine-wood dominated; no substantial leafy curtains. |
| norway-spruce-3 | F | F | F | U | U | Conical leader retained, fuller lower network, but visible trunk and tier framework dominate a transparent crown without substantial pendant foliage curtains. |
| norway-spruce-1982700925 | F | F | F | U | U | Fresh specimen preserves conical leader but remains transparent and wood-dominated, with exposed tiers and insufficient hanging foliage mass. |
| norway-spruce-281313742 | F | F | F | U | U | Lower branchlet network is fuller, but the crown is still translucent with a conspicuous trunk/tier framework and no substantial pendant foliage curtains. |
| norway-spruce-2271779095 | F | F | F | U | U | Conical leader and fuller lower network retained, but central trunk and tier framework remain conspicuous through transparent foliage; curtain mass fails. |
| norway-spruce-4250668600 | F | F | F | U | U | Retained width counterexample remains numerically repaired, but still shares transparent wood-dominated crown and inadequate pendant foliage curtain mass. |

Oak's low crooked spreading habit passes; leaf anatomy passes except in the obscured seed-2 detail. Its lower/lateral mass gains do not repair upper bare scaffolds and crown windows. Spruce retains individual needles and now shows secondary subdivision, but every inspected specimen still fails substantial curtain mass. Spruce seed 1 exterior-right exposes real blunt terminal caps with unrestricted depth, so terminal taper now has an observed failure rather than only missing evidence. Its upper-side forward needle inclination receives a narrow supplemental pass; unresolved peg contact prevents the combined foliage trait passing. Other specimens do not borrow that local result.

## Shared-template regression and verification

All nine Ordinary/Telperion/Laurelin geometry and PNG hashes **exactly match pass 1**; see [shared comparisons](pass2-shared.json). Every new image was also opened. Their sparse/upright or sweeping crowns, supernatural spiral and root flare remain historical relative to this pass, not new pass-2 regressions. Their age before FN9 remains unassessed.

[Native costs](pass2-costs.json) retain one warmup plus five measured samples, with per-process peak RSS from `wait4`. Counts and bytes match pass 1. Concurrent browser work and uncontrolled CPU scheduling/frequency make timings contextual. Wood bytes include positions/normals/indices; matrices use 64 bytes per instance; allocator, prototype, JS/Wasm and GPU memory domains are excluded.

| Template | Median build ms | Nodes | Units | Wood + matrices bytes | Peak RSS KiB |
|---|---:|---:|---:|---:|---:|
| ordinary | 62.92 | 11,766 | 55,438 | 19,212,064 | 27,132 |
| telperion | 1630.37 | 177,543 | 1,360,279 | 403,001,248 | 524,296 |
| laurelin | 818.28 | 105,032 | 802,547 | 158,439,056 | 255,896 |

[Gate evidence](pass2-gates.json) records passing native workspace tests, 106 harness tests, fmt, Clippy `-D warnings`, typecheck, build and isolated Wasm/browser integration with explicit exit **0**. Browser checks retain identity, anatomy, bounds, ownership, malformed/empty output, seed variation, orbit visibility, UI selection and failure/retry assertions. An old capped binding fixture exhausted its structural budget before foliage; the corrected fixture uses an explicitly small oak scaffold and asserts complete growth. Render/UI fixtures use that same small oak architecture. These software fixtures never replace the separate uncapped mature fidelity protocol.

Earlier binding, Vite initialization and execution attempts remain recorded. One slow orbit attempt was deliberately stopped to correct inconsistent small fixtures. A later run reported all assertions passing but its executor shell ended with 143 before collecting the child exit status; the isolated recorder replay completed with exit 0. The capture runner also outlived its executor shell; its completed receipts are verified by a clean replay that returns the expected **1** for unassessed automated visual status, not a rendering failure. No fidelity pass is inferred from screenshots existing or from software checks.

## Remaining blockers and next work

**No make-pr. R3–R6 visual fidelity remains unmet.** Required numeric replay is green, but every inspected species crown still fails the healthy mass/gaps comparison. Spruce still lacks substantial foliage curtains. Spruce seed-1 terminal taper fails; other complete tip/socket judgments and peg contact remain unresolved (upper-side forward bias passes only the seed-1 supplemental view); extra camera angles are evidence, not automatic passes. Owner feedback remains absent.

1. Trace each retained oak counterexample's bare upper scaffold through the radius solve, local handoff and shedding stages. The added lateral systems disproportionately fill lower/lateral regions; several upper sprays are now more conspicuously naked. Preserve fine continuation and place foliage-bearing subdivisions on those specific upper reaches instead of adding another undirected layer of side branches.
2. Diagnose the separate structural `shed(..., 0.45)` in `branching::generate`. Pass 1's foliage shell setting does not disable this earlier structural removal. Pass-2 seed 1 still sheds 22,476 oak and 4,863 spruce local nodes. This is a candidate contributor, not a proven sole cause. Compare retained/lost descendants on exposed supports; do not replace diagnosis with a global shell or foliage-count increase.
3. For spruce, follow the new descending secondary forks into the local branch law. The fans produce extra fine axes but still read as scattered radial sprays rather than substantial pendant foliage systems. Rework their spatial distribution, length and supporting-twig hierarchy against S-BRANCH, while preserving the leader, primary upturn, dimensions and `4250668600`.
4. Keep the actual exterior twig/socket IDs and multi-angle protocol. Select additional unobscured exterior sockets and needle contacts when foreground leaves/wood hide them. Verify surface-radius attachment against the rendered socket, not just the centreline model. The uncropped spruce seed-1 blunt terminal caps require a taper correction; weak peg relief remains unresolved; do not count local clipping artifacts as seams or infer an unobserved pass.
5. Retain every fixed/fresh seed, all frozen targets and the six source photographs. Replay 48 numeric cases, the required mature views (including `4250668600`), exterior detail and the shared-template hashes after the next production change. Keep Flow blocked until every required trait passes honest inspection.
