# Branch until the tips bear leaves: one recursion, trunk to twig

## Goal

The tree stopped branching about halfway down: finest wood 0.79 m, leaves 0.12 m, 2,904 leaves where a tree this size carries millions. The owner's framing: branches branch until the last depths are twigs that leaves attach to. This spec (superseding fn-2 and fn-3, whose seam was where the defect would have lived) grows one structure from trunk to leaf-bearing tips: Valinor-sized trees at realistic depth, performant, near reality-level fidelity, keeping the supernatural parameters.

## Decisions

Owner (verbatim):
> "can you check the spec 5 and review it. Goal is to build trees of valinor sized trees at "realistic" branching depth and structure. I want this to be performant and close to reality level fidelity. But keep the parameters that make the trees appear "magical/supernatural"."
> "ok you wanna /capture --rewrite the spec then?"
> "but definitionally branches should branch until the last depths are twigs that attach to leaves no?"
> "just to be clear this will allow for basically "realistic" 150m trees that'll fill up with whatever a tree that size amount of branches would have?"
> "so fn-3 and fn-2 will allow for a realistically constructed tree at that size with right amount of branches and twigs and leaves?"
> "how would a 150m tall tree work in real life? would it have giant leaves? or would there be millions of leaves?"

- One structure in two passes: space colonization while attractors are meaningful, then local rules continue from its tips into the same skeleton (one parent-index invariant). Colonization all the way down was rejected (~23 fork generations, ~8 M tips; each step halving costs ~2.3x nodes).
- The bias field (gravitropism, lean, writhe, spiral, torsion) shapes twigs exactly as it shapes limbs (R9); letting local rules ignore it was rejected.
- Local rules rest on sourced botanical defaults (children 2, angle 45, divergence 137.508, taper 0.6/order), each a dial. Fine orders get their own taper law anchored to the parent's radius at the handoff.
- Interior twigs are shed by the canopy's shell rule (`shedTwigs`, `DEFAULT_SHED` = `DEFAULT_CULL`).
- The attractor search radius is the wider of nine steps and 2.0 attractor spacings (1.2 still starved 3 of 108 draws); the mechanism fix in colonize was rejected because Laurelin's shipped tree exits through that branch.
- Rejected: the attractor dial, separate step/kill/influence knobs, larger leaves, "realism" by count and ratio alone.

## Acceptance Criteria

- **R1:** Trunk to leaf-bearing tips as one structure in two passes, depth a named parameter with a rail, sourced botanical defaults as dials — met (`SkeletonParams.step`, `twigs.levels`).
- **R2:** Radius, direction and taper continuous across the crossover, asserted at every junction-ring vertex, never vacuously — met (90th-percentile taper assertion; the tail is fn-4's blunt-tip defect).
- **R3:** Terminal wood reaches leaf scale (leaf a multiple of its wood's diameter) on both presets — met at eight orders: 1.64x Telperion, 1.16x Laurelin (from 0.15/0.16).
- **R4:** No panel-reachable depth/attractor combination starves growth — met (radius floor at 2.0 spacings; 0 of 108 draws starve).
- **R5:** Leaf count reaches 10^5 to 10^7 after shedding, ceiling reported — met: 136,686 Telperion / 766,390 Laurelin at eight orders; ceilings ~0.8 M / ~1.1 M before the 250k node stop.
- **R6:** Cost curve measured; shipping depth renders inside 16.7 ms on the RTX 3080; `maxNodes` reported; dragging stays usable — met (see Result; builds settle 250 ms after the last notch when dearer than 80 ms).
- **R7:** Deterministic; depth default reproduces both presets as shipped byte-identically — met (Telperion 808 / Laurelin 2442 node hashes unchanged).
- **R8:** Owner confirms in clay and states the shipping depth — superseded by fn-6 R3 and R7 (generation count follows radius; owner judges anatomy in clay).
- **R9:** The bias field shapes growth below the crossover as above; zero terms reproduce the unbiased recursion byte-identically — met (lean test moves twigs +0.65 of a step).

## Result

GPU sweep on the RTX 3080 (vsync off, panel default 24 m tree): 0 orders 58,880 triangles / 0.20 ms; 8 orders 3.63 M triangles, 189,843 leaves, 2.12 s CPU build, 1.28 ms GPU; 10 orders 212,430 nodes (ceiling), 4.06 s build. The frame is not the constraint (vertex-bound, flat across resolution); the CPU build is. No impostor demanded. The swept surface is most of the triangles (3.1 M of 3.6 M at eight orders). A spatial grid made colonization association 1.5-1.6x faster.

Closed 2026-09-07 as a completed historical implementation superseded by fn-6 and the fn-8 Rust migration. The closure does not convert the missing owner sign-off or preset GPU timing into a pass; rendering performance belongs to fn-13.

## Open

- Shipping depth for the presets (R8) passed to fn-6.
