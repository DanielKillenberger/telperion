# The site's field grows only the detail it can show

## Goal & Context

fn-245's fourth stage. killenberger.com stays on 0.1.4 until the new engine meets its speed (fn-237 R0). fn-245 `PROFILE.md` found every measured internode shorter than a field cell and 88-97% of converted nodes below the site's wood cutoff: their geometry is unnecessary, their light, mass and allocation effects are not. Astra's review (fn-245 `ASTRA-ARCH.md` §4-5) derives grow budgets at 36³ of roughly oak 185-210 ms, birch 210-275 ms, spruce 195-200 ms of 300/350/240 ms end to end, and says caching alone cannot reach them cold. Supersedes fn-214's remaining field work and fn-227's size question where they overlap. Release item.

## Requirements

- **R1 (aggregates in the one cycle):** growth below the requested resolution is carried as coarse records (attachment and lineage, presence and cohorts, leaf area, wood mass and spatial moments, pipe, load and carbon state, enough to split finer) that take part in the same cycle's light, balance and allocation as stage 2's grower, never through a second grower (no `lit` versus `settle_with`). Which records stay coarse is decided before growth from the stated resolution, so aggregate feedback is consistent through every cycle; there is no local refinement after growth.
- **R1a (query contract):** the field build takes the resolution (world-space cell size) through TypeScript and Wasm; the `cut` argument it replaces is removed and its old meaning stated in the migration note. A query with a non-zero half-extent finer than the build resolution, or a mixed-resolution query, returns a named error; a point query (zero half-extent, `src/field/index.ts:19`) is exempt and answers the build-resolution cell that contains the point. Tests cover each case. The fixed 1,024-expected-phytomer cut is replaced by a world-space cell size or footprint the call states, or a cheap conservative plan step whose cost is measured inside the call.
- **R2 (convergence, calibrated first):** the reference is the fully expanded tree at the same seed through the same grower. Compared at 24³, 36³ and 48³: bounds, per-cell wood occupancy and maximum radius, per-cell foliage density, and limb ownership of each cell. The first task measures these on every supported species and proposes thresholds; the owner approves them before implementation proceeds. The check runs in tests, never in a production call; a resolution whose aggregates miss a threshold aggregates less at that resolution, and a request over the resource limit returns a named error. A feedback-sensitive tree (the oak) is tested at tightening resolutions, and any repeated work counts in the cold timing. The existing closed-form rows serve as the tested aggregate first; replacing their full-history recursion with incremental cohort updates is measured, not assumed (nonlinear survival does not commute with expectation).
- **R3 (proved):** cold single-thread Wasm 36³ field calls at or under 300/350/240 ms for oak/birch/spruce, and every supported species at or under its 0.1.4 time (the ordinary tree's 11.2 s included), with cold table construction and the first tree measured separately; 24³ and 48³ checked; field fidelity against the expanded tree; Astra's agreement; gate and npm test green.

## Council plan (host, 2026-10-11)

The owner asked for a council of Fable and Astra on the attack plan; both signed `.flow/evidence/fn-245-a-breakthrough-in-generation-time/COUNCIL.md`, which governs this spec where it differs from the requirements above. For this spec: `couple::Aggregate` is the first record candidate, cut by world cell size, the lattice fixed at 0.5 m; early test 4 comes first and its stop condition goes to the owner.

## Boundaries

The field call. Depends on stage 2; integrates after stage 3. Release item for fn-237.
