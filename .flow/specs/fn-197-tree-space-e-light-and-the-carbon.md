# Tree space E: light and the carbon balance

## Goal

Phase E of the tree-space programme (`docs/tree-space.md`; owner, 2026-10-03): feedback on top of the passed species, light intercepted from above and a carbon balance that shape density, the clear bole and size. The 2026-10-03 probe (fn-190 R1 round 4) showed this can form a bole by shedding and bound size; here it is first reproduced against a published light model, then added by degree, every new setting neutral so passed species stay byte-identical. Wrong-path stop: a passed species regresses and cannot be restored.

## Decisions

Host design, 2026-10-05; options and reasons in DESIGN-OPTIONS.md (decisions in its section 8 and later sections).

- **Light:** Beer–Lambert through a coarse leaf-area lattice anchored at the world origin, trilinear deposits and reads, swept once per cycle along nine sky directions; buds read the previous cycle's field. Oracles: leaf slab exp(−k·LAI), uniform sphere, GreenLab's production formula.
- **Reading:** a rough layout grown with the tree, and a full lay with sag every K = 10 cycles (decision 8).
- **Sky:** one continuous site setting from overhead-only (0) to uniform overcast (1); every species judged under one value.
- **Stop:** ln(survival) = ln(viability) · I^(−φ), neutral φ = 0; a red-first test showed the multiplicative form viability · I^φ jumps (decision 5).
- **Fill by light (Borchert–Honda):** basipetal light sums, acropetal vigour split by per-PA apical control λ (walked within Pałubicki's 0.45 to 0.55; trunk and leader may sit higher); a bud's unit sized against the tree's mean vigour to ψ, neutral 0; sizes normalised per physiological age (decision 23); no phototropism (decisions 4, 11, 14, 15, 17, 21).
- **Shedding on the balance:** (light − upkeep)/(light + upkeep) remembered with `MEMORY` = 0.5 (estimated); a yearly shed hazard rising continuously below the PA's tolerance; dead limbs dropped after the PA's shedding delay (decisions 5, 18, 20).
- **Girth:** `retained` (Shinozaki's disused pipes) and `leaf_girth` χ, per PA, neutral 0.
- **Walks:** refine depth 6 (decision 9); the gate walks the light settings and a sample of structural ones in leaf; every setting in leaf is a slow `#[ignore]` suite (decision 12).
- **R2 reread (decisions 19 and 26):** light must not widen the seed-to-seed spread in fine wood length; the earlier "seeds within 1.5x in node count" was inferred, and leaf count passes through the pipeline's leaf-bearing marking, phase F's contract.
- Sag's foliage term and light's `leaf_area` are kept apart for neutrality and unified per species later (decision 10). Leaf area and k stay plausible here; the oak's leaf area is sourced on its own branch (decision 16).

## Acceptance Criteria

- **R1:** A light pass reproduced against its published description, then added as a setting whose neutral value leaves every passed species unchanged — met (oracle tests: slab exact, sphere within 0.032 optical depth, GreenLab production within 3.3%/1.5%; neutral 80-year hashes unchanged).
- **R2:** Shedding on each branch's remembered balance, growth bounded by it, light not widening the seed-to-seed spread in fine wood — met (2.1x against round 4's 2.5x).
- **R3:** Every passed species re-rendered and judged, none worse; walks continuous — met (beech, spruce, oak byte-identical at neutral; walks green at depth 6). The palm's neutral hash was not taken (its branch had diverged).

## Result

The oak under light (oak round 5's values) closes into a dome on four of five seeds; with retained 0.3 and χ 1 trunks and limbs read heavier, crowns 2 to 4 m narrower and 1 to 4 m taller than step 4c (heights 21.5 to 23.5 m). Beech under oak-like light (information only): heavier limbs, thinner fine web, broader crowns. Close (host decision 28): workspace gate 1,060 passed, 1 failed (the oak's `material_detail` list, fixed in oak round 5); Codex review SHIP in round 3 after six findings fixed (`bd44799e`, `c101191e`).

Neutral 80-year hashes, beech/spruce/oak seeds 1 and 7, recorded in STEPS-1-2.md.

## Open

- Decision 10: unify sag's foliage term with light's `leaf_area` per species.
- The palm's neutral hash under E.
- The in-leaf walk of every setting (slow suite, hits its 3,000 s timeout).
- Unsourced estimates: `MEMORY` = 0.5; the oak's dead-branch persistence of 5 years.
- Phase F: seed 4's leaves per metre of fine wood (39 against 136) is a leaf-bearing marking item.

## Friction

- No per-stage timing in `grow()` -> done (decision 6: `grow_staged`, `measures -- stages`).
- A loaded machine skewed timings -> check load average before timing.
- Gate failure inherited from fn-195's young-wood row -> fixed on the oak's branch.
- No sourced oak leaf area, balance memory or dead-branch persistence in reach -> leaf area later sourced (Visakorpi et al. 2020); the others stay estimated; proposed a literature task.
- The walks binary outgrew a 30-minute run at depth 6 -> in-leaf every-setting walk made a slow ignored suite.

## Evidence

- `.flow/evidence/fn-197-tree-space-e-light-and-the-carbon/DESIGN-OPTIONS.md` (design options and host decisions; cited by `grow/balance.rs`)
- `.flow/evidence/fn-197-tree-space-e-light-and-the-carbon/STEP3D.md` (cited by the walk tests)
- `.flow/evidence/fn-197-tree-space-e-light-and-the-carbon/STEPS-1-2.md` (neutral hashes and stage timings; cited by fn-198)
