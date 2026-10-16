# Tree space D: the oak as a point

## Goal

Phase D of the tree-space programme (`docs/tree-space.md`; owner, 2026-10-03): the oak (Rauh's model; Oregon white oak preset, *Q. robur* photographs where *Q. garryana* ones are lacking) as a point in the tree space, values only, from fn-190's MODEL-OAK.md and its sources. Gate as in phase C; wrong-path stop as phase C, or a passed species regresses.

## Decisions

- Values in `crates/telperion-space/src/oak.rs`; sources in SOURCES.md (leaf area 18.1 cm² a node, Visakorpi et al. 2020).
- Round 5 (host decision 29): the oak takes light as its own values on fn-197's engine (φ 1, ψ 1, λ 0.45, upkeep 0.35, balance hazard 2, tolerance 0, retained 0.5; limbs' dead-branch persistence 5 years, estimated).
- Round 6b (host decision 31, `670dacda`): the conversion marks an axis as stem only where it carries the trunk at one of its species' trunk-level ages; a leader turned limb is no longer stem; a codominant trunk fork is both stems. Red-first test `tests/space_stems.rs`. The stills rows carry `shootRadius` 0.06.
- Host and Astra iterated three gates (rounds 5, 6b, 8) without agreeing (Astra: ascending fans and pointed sprays, not heavy tortuous boughs), so the oak went to the owner.
- Owner on round 8 (2026-10-05): "structurally ok but very blob like and unnatural in subtle ways. Branches are a bit chaotic ... the reference looks calmer and more natural." Not a pass. Host diagnosis: `form.wander` was a memoryless per-node random walk; the fix became fn-207 (wander bends over a length), with this oak as its R4 trial.
- Round 10 builds on fn-207's bends (trunk lifespan 12, leader 19, limb insertion 0.5). Round 11 (an arching PA, more division) overshot into umbrellas and lost leaves; reverted to round 10 (`54b9dff8`, restored in `f3384827`).
- R3 split to task 2, blocked by fn-206 (different physiological-age chains).

## Acceptance Criteria

- **R1:** The oak's values from MODEL-OAK.md and its sources, several ages and mature, seeds 1 and 7, beside `.flow/references/oak-quercus-robur/` and today's oak — met (round 10).
- **R2:** The gate as in phase C — met on the owner's verdict; workspace gate 1,063 passed, Codex SHIP.
- **R3:** A spruce-to-oak walk as still strips, no visible pop — met (task 2, PR #154).
- **R4:** The passed beech and spruce sheets unchanged in look — met (fn-197's and fn-207's neutral 80-year hashes; pixel-identical renders across the stem fix). The palm was not on this branch.

## Result

Owner verdict (2026-10-05): "yea i agree the oak needs more fine twigs. If that's a performance thing to be fixed in [phase F] then let's accept it for now." Accepted at round 10 on fn-197's light and carbon engine and fn-207's bends. R3 walk (age 60, seed 1): the spruce cone opens into the oak crown by degree, 16.9 to 19.0 to 15.8 m, no pop. Built interleaved with fn-197 and fn-207, then split into three stacked branches from master.

## Open

- The density of the fine twig web (S1 keeps dividing to the outline; ours carries heavy wood to the edge ending in tufts), bounded by the growth and geometry budget; phase F's levers.
- The trunk-base flare and collar band (`flareRadius` 2.1, `flareFalloff` 0.022 give a bell).
- Bands on the bole, probably the bark's plate texture.
- The 10-year sapling's spire; leaf form.

## Friction

- Measures and stills in different crates left one binary stale -> proposed (as fn-193): the stills runner prints dbh and expected grown count.
- A stem-flag change silently cut leaves from 2.4M to 0.5M through `Tree::stem_radius` -> fixed by decision 31 and `shootRadius` 0.06; proposed: stills print leaves per fine metre.
- One working branch for three specs cost about 45 minutes of cherry-picks -> one spec per branch from the start.

## Evidence

- `.flow/evidence/fn-195-tree-space-d-the-oak-as-a-point/SOURCES.md` (values and sources; cited by `oak.rs`)
