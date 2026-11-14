# Fast, high-quality rendering of parameter-driven trees

## Goal

The owner locked a north star for the renderer and asked for the highest expected-value path to it, with unnecessary requirements, restrictions and tasks removed and the plan reviewed for development speed:

> A renderer that makes procedurally generated trees look convincing from individual leaves to a thousand-tree forest, while staying fast enough for real-time use.
>
> Generator parameters define trees and leaves; templates are reusable presets. Supported parameter changes require no renderer-specific code. Preserve convincing anatomy, silhouettes, gaps and shading without distracting noise, popping or loading holes. Target 2ms GPU time for a hero tree and 4ms for 1,000 distinct trees on RTX 3080 at 1600×1000 physical pixels, DPR1.

## Decisions

- Representations, algorithms and task structure are expendable; the north star is not. Nanite is inspiration, not a required implementation.
- Start from the existing triangle/shared-part path and measure at a demanding hero view; aggregate unresolved crown interior and distant foliage; exact inspection stays a separate explicit mode.
- No hand-authored foliage clusters and no species-name or needle-topology renderer branches; choices derive from geometry, placement and projected error.
- Development rules: smallest increment that answers the next question, cheap rejection checks before captures, honest labels on failed/uninspected evidence; no per-task implementation review (owner policy).
- Removed from the active graph: tasks 2, 6, 9, 11, 14, 15, 16, 17 (ceremony folded into 10/3/4/5).

## Acceptance Criteria

- **R1:** Appearance — convincing near anatomy, preserved far silhouette, gaps, volume and shading — not delivered.
- **R2:** Stability — no stepping, popping, shimmer or holes in navigation and loading — not delivered.
- **R3:** Performance — vegetation GPU p95 within 2 ms hero and 4 ms for 1,000 distinct trees — not qualified.
- **R4:** Scene correctness — foliage/ground noise and flicker fixed with shared depth intact — partial (shared depth works).
- **R5:** Generalized data and lifetimes across species, parameters and topologies — partial (parameter-driven prototype support).
- **R6:** Inspectability and honest evidence — met for the work done.

## Result

Closed 2026-09-08 as a prototype, partially delivered: three days, 230 commits, about 10,700 code lines and 2.6 million evidence lines. Working and reusable: a WebGPU hero path with compact wood, canonical assemblies, cross-part detail, worker preparation and streaming; GPU timestamp measurement with contention detection; ownership-based memory accounting with farthest-first eviction (the 1,024-tree capture peaked at 1.73 GB retained, from 4.28 GB); parameter-driven prototype support; two real bug fixes in the transfer collector and short-span precision. Not delivered: a usable forest, acceptable appearance, a stable viewer, the default preset, or a qualified performance number. The branch was not merged; its ideas are ported by rewriting under fn-22 and its successors. Tasks .5, .20, .21 and .22 are superseded by that series.

The fn-13 task 5 forest captures consumed a full weekly quota and led to the token and evidence budget rules in AGENTS.md (owner, 2026-09-08).

## Open

- None here; rendering continues under fn-22 and successors.
