# Fast, high-quality rendering of parameter-driven trees

## North star — owner locked

A renderer that makes procedurally generated trees look convincing from individual leaves to a thousand-tree forest, while staying fast enough for real-time use.

Generator parameters define trees and leaves; templates are reusable presets. Supported parameter changes require no renderer-specific code. Preserve convincing anatomy, silhouettes, gaps and shading without distracting noise, popping or loading holes. Target 2ms GPU time for a hero tree and 4ms for 1,000 distinct trees on RTX 3080 at 1600×1000 physical pixels, DPR1. Automatically derive and share detail, draw what the view needs, and retain exact inspection. Even a hero view aggregates screen-unresolved crown interior and distant foliage; preserve source-like anatomy where it is actually resolved. Exact inspection remains a separate explicit mode.

Choose the highest expected-value path we can identify now: maximize the likelihood of reaching this outcome per unit of development time and compute. Nanite is inspiration, not a required implementation. Representations, algorithms, experiments and task structure are expendable; the north star is not.

## Current state

Completed 1/7/8/12/13 remain historical completed work: baseline, shared depth/submission, canonical assemblies, census and bounded reference methodology. Task18 has committed parameter/prototype support, numeric scatter, bounded exact-source paging and focused parity/ownership checks. A spruce crown source/canonical reference screen converges, but representative oak fit still needs inspection. Prior captures and failed/incomplete verdicts remain intact; the 272-frame draft is not a required prerequisite or a passed result.

The existing interactive assembly viewer is fast but visibly noisy and has loading issues. Reuse it, its camera controls, GPU timers, shared part library and source-reference tools. Deliver visible improvements there before duplicating the renderer in the product route.

## Highest-EV approach

1. Finish the small cross-shape compatibility check using existing evidence and bounded oak comparisons.
2. First build one cross-part coarse level and measure it at the demanding hero view; part-level decimation is optional only if measured part triangles warrant it. Reduce actual rendering work using geometry-driven detail. Start with the existing triangle/shared-part path because it is implemented and measurable; this is a starting hypothesis, not a triangles-only mandate. Measure at a representative demanding view immediately. If that path cannot plausibly reach the target, test the smallest alternative on the failed case before expanding it.
3. Build cross-part hierarchy, scene-wide selection and shared residency into the interactive renderer. Near views retain plausible anatomy; distant views must reduce work above individual-part roots. Allow aggregate geometry, filtered coverage or other approximations when they preserve visible appearance and measured cost. Check a short motion clip during representation selection so unresolved shimmer/popping cannot be deferred until integration. If triangle aggregation fails, a likely next candidate is placement-derived cluster points/cards with filtered view-dependent coverage; test only when the observed bottleneck supports it.
4. Write reusable compiler/hierarchy/residency TypeScript under src/browser from the outset and import it in the interactive viewer. Wire that working pipeline into the normal browser build and parameter controls, fix motion/loading/depth defects in that actual path, then scale to the distinct forest and perform final acceptance once.

Approximation may merge, replace or omit individually unresolved source elements when aggregate appearance is preserved; retaining every original needle/leaf at every distance is not a requirement. No hand-authored foliage clusters or species-name/needle-topology renderer branches. Derive choices from geometry, placement and projected error. New unsupported botanical rules are generator work; current spruce/oak support is mandatory.

## Development rules

- Build the smallest working increment that answers the next consequential question. Prefer an interactive before/after and a focused measurement over a new qualification subsystem.
- Use cheap rejection checks before expensive captures. Stop a sweep on a decisive failure, fix the cause, and rerun only affected cases. Choose further experiments by expected information/value, not fixed quotas or mandatory method sequences.
- Use representative spruce/oak views and parameter changes early. Add cases for a demonstrated risk or changed behavior; no automatic Cartesian product of species, seeds, distances, motion and channels.
- Reuse valid source/camera/parameter-matched evidence. Use expensive converged references only when needed to resolve sampling ambiguity or a disputed visual result, not as a universal gate on each implementation step. Label scoped/invalid/uninspected results honestly and preserve historical failures.
- Astra low owns implementation and harness work. No Grok. No per-task implementation review per owner policy; focused tests during implementation and the relevant integrated suite once at integration. Review this plan with Fable for development speed and outcome quality.
- Keep resource ownership, cancellation, stale-result rejection and useful errors where the real path needs them. Avoid speculative public APIs, exhaustive capability matrices or separate infrastructure projects.

## Acceptance criteria

- **R1 — Appearance:** Convincing resolved anatomy and attachment near the tree; preserved silhouette, crown gaps, foliage volume and shading farther away. Approximate geometry/pixels need not match exactly. Inspect representative spruce/oak stills and movement, using matched references where needed; visible damage cannot be hidden by a global average.
- **R2 — Stability:** Normal slow/fast navigation, reversal, approach into the crown and detail loading do not show distracting stepping, popping, shimmer, ghosting or missing regions. Exercise cold approach and parameter replacement; a missing resource keeps valid coverage or reports unavailable preparation rather than silently disappearing.
- **R3 — Performance:** Measure total vegetation GPU p95 against 2ms for mature spruce and oak hero views and 4ms for at least 1,000 structurally distinct specimens with the pinned species/size mix. Use RTX 3080,1600×1000 physical pixels,DPR1; include all selection/render/filter passes, report whole-stage timing separately, and report preparation/CPU/upload/memory costs. Hero misses block final acceptance; forest target misses remain explicit as in the original contract. Invalid or contended GPU measurements cannot claim a pass.
- **R4 — Scene correctness:** Fix foliage/ground noise and flicker while preserving shared depth, occlusion and contact in still/moving/grazing views. Hiding ground, forcing foliage on top or silently thinning the scene is not an acceptable fix.
- **R5 — Generalized data/lifetimes:** Existing spruce/oak, supported parameter changes and a different-topology data fixture use the same renderer contract. Preserve independent specimen identity, conservative bounds and shared-resource survival across replacement, cancellation and disposal. Keep the data boundary portable without requiring another engine or a universal adapter.
- **R6 — Inspectability/evidence:** Keep explicit deterministic exact inspection and reproducible identities, relevant images/clips, raw timings and honest verdicts. Missing, stale or uninspected evidence is not a pass. Separate visual quality from performance acceptance.

## Remaining tasks and milestones

| Order | Task | Deliverable |
|---|---|---|
| 1 | 18 | Finish the small input/visual compatibility gate; reuse committed work |
| 2 | 10 | Hero-view cross-part coarse detail, motion check, then two trees with bounded shared residency |
| 3 | 3 | Production Wasm/browser data preparation and ownership for that working renderer |
| 4 | 4 | Normal viewer integration, stable filtering/loading, correct shared scene depth and parameter updates |
| 5 | 19, 20, 21 | Task 5 was blocked on 2026-09-08 after a failed 1,024-tree admission run. 19 bounds retained CPU memory (one forest capture), 20 fixes close motion on the 8-tree forest, 21 runs final acceptance and docs once |

Task10 may iterate on a measured representation failure without reopening a separate feasibility bureaucracy. Task10 must show an interactive result before production promotion. Task4 owns noise diagnosis and fixes in the actual renderer, not a mandatory standalone diagnostic/filter qualification sequence. Task5 uses a small population first when useful, then the actual forest; it does not blindly repeat a 1/8/32 protocol.

Removed from the active graph:2(feasibility ceremony),6(separate finalization),9(prescribed union/representation probe),11(duplicate experimental integration),15(diagnosis-only),16(separate filter qualification),17(separate residency qualification),14(per-part reduction separated from hierarchy). Their necessary behavior is folded into10/3/4/5; obsolete method restrictions and evidence ceremonies are dropped. Archived task definitions are history, not completed work.

## Boundaries and strategy alignment

Web-first TypeScript/WebGPU/WGSL renderer with the existing Rust generator. Preserve exact output defaults and current owner viewer. No native rewrite, external-engine adapter, new species catalogue/editor, wind/material authoring or GPU botanical generation. Fn9 stays a recorded dependency under the existing owner authorization to use current species references.

Serves STRATEGY.md tracks **Surface and rendering at scale**, **The core and integration**, and **Growth and botanical fidelity**. Generalization is parameter-driven; templates are presets. No strategy drift.

## Verification and reuse points

Use `npm run typecheck`, focused tests for changed code and `node_modules/.bin/vitest run` at integration. Run browser/hardware checks on the paths actually changed. Final normal-build/browser checks occur in tasks3–5. Capture only what helps decide correctness, quality or cost.

Reuse `experiments/fn13-rendering/canonical-library.js`, `assembly-engine.js`, `webgpu-budget.js`, `live-assembly.js`, existing page/selection modules, `compact-wood.ts`, task18 bounded source tools and `tests/browser/fixtures/rendering.json`. Source placement is in `crates/telperion-core/src/foliage/placement.rs`; production boundary is `src/browser/core.ts` and `scripts/build-wasm.mjs`.

## Requirement coverage

| Requirement | Remaining owners |
|---|---|
| R1 | 18,10,4,5 |
| R2 | 10,3,4,5 |
| R3 | 10,4,5 |
| R4 | 10,4,5 |
| R5 | 18,10,3,4,5 |
| R6 | 18,10,3,4,5 |

## Owner decision

The owner locked the north star, requested removal of unnecessary requirements/restrictions/tasks, and asked Fable to review the highest-EV plan with explicit emphasis on development speed toward a fast, high-quality renderer. This plan supersedes earlier sequencing/method restrictions; prior evidence remains historical and its verdicts are not changed.

## Closure (2026-09-08)

Closed as a prototype, partially delivered. Three days, 230 commits, about 10,700 code lines and 2.6 million evidence lines. Working and reusable: a WebGPU hero path with compact wood, canonical assemblies, cross-part detail, worker preparation and streaming; GPU timestamp measurement with contention detection; ownership-based memory accounting with a memory bound and farthest-first eviction; parameter-driven prototype support; two real bug fixes in the transfer collector and short-span precision. Not delivered: a usable forest, acceptable appearance, a stable viewer, the default preset, or a qualified performance number. The branch is not merged; it is a reference prototype whose ideas are ported by rewriting under fn-22 and its successors. Tasks .20, .21 and .22 are superseded by that series.
