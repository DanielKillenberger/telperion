---
satisfies: [R1, R2, R3, R4]
---
# fn-193-tree-space-c-the-beech-as-the-first.1 Implement Tree space C: the beech as the first point

## Description
TBD

## Acceptance
Every R-ID in the parent spec's ## Acceptance Criteria is satisfied; judge this task against the spec's criteria directly.

## Done summary
The beech is a point in the tree space, values only (crates/telperion-space/src/beech.rs). It is drawn through the pipeline's own expansion and the headless renderer (crates/telperion-render/examples/space_beech.rs). The engine gained per-PA Form (tropism, elevation, wander, plane, pipe girth). Stopped before review, as the host directed: the sheet goes to the host and Astra.

Sheet: .flow/evidence/fn-193-tree-space-c-the-beech-as-the-first/raw/final2/sheet-beech.png (round 2; ignored, on disk). Round 2 reading in RESULT.md.

stage: impl-review - skipped(policy: host directive - stop before review with a candidate sheet)
Tier: session

Round 3: BLOCKED (DESIGN_CONFLICT). Troll's relay mechanism is built and green (02a9aa61). The module beech collapses because relay presences multiply up the stem (RESULT.md, round 3; raw/v3b/chain.log). The committed beech is round 2's.

Round 4: BLOCKED. The relay stake is built and green (see the commit list). The module beech no longer collapses, but 20 years still fails, the bole stacks module rings and the in-leaf crown is sparse (raw/final3/sheet-beech.png; RESULT.md, round 4). The committed beech is round 2's.

Round 5: sheet raw/final4/sheet-beech.png, close-ups raw/final4/close-ups.png; reading in RESULT.md round 5.

Round 6: raw/final5/sheet-beech.png and close-ups.png; reading in RESULT.md, round 6.

Round 7: raw/final6/sheet-beech.png and close-ups.png; reading in RESULT.md, round 7.

Round 8: raw/final7/five-seeds.png, sheet-beech.png and close-ups.png. Four of five seeds read as an open-grown beech (RESULT.md, round 8).

Round 9: raw/final8/five-seeds.png. Five of five seeds read as an open-grown beech at 80 cycles, seed 2 borderline. The relay = 1 walk jumps (RESULT.md, round 9).

Round 10: the relay = 1 walk passes (slope 2.82). raw/final9/five-seeds.png: two of five seeds read as a beech, two borderline (RESULT.md, round 10).

Round 11: values only plus the conversion's joints (3c89f5a4). raw/final10/five-seeds.png: four of five seeds read as an open-grown beech (1, 2, 3, 4), seed 7 borderline; no collars at the trunk bases (raw/final10/bases.png). Reading in RESULT.md, round 11.

Round 12: a666161a. raw/final11/five-seeds.png: seeds 2 and 4 read as a beech, 1 and 7 borderline, 3 no; the large limbs read at every seed. Reading in RESULT.md, round 12.

Round 13: 6bb530aa. form.roll built and walked; the sibling vigour share not built (seed 3's leader died by viability, not a near-bound birth draw; fixed by leader viability 1). raw/final12/five-seeds.png: seeds 2, 3 and 4 read as a beech, 1 and 7 borderline. Reading in RESULT.md, round 13.

Round 14: values only. raw/final13/five-seeds.png: all five read as a connected crown; seed 3 yes/borderline. Reading in RESULT.md, round 14.

Round 15: values only. raw/final14/five-seeds.png: deeper crowns (depth/width 0.72-0.87, target >= 1 not met); 7 and 2 yes, 1 yes/borderline, 3 and 4 borderline. RESULT.md, round 15.

Round 16: limbs and boughs as Troll module stacks, values only. raw/final15/five-seeds.png: five of five integrated domes (4 yes/borderline). RESULT.md, round 16.

Round 17: values only. raw/final16/five-seeds.png: rounded tops and continuous outlines at all five seeds (3 and 4 yes/borderline). RESULT.md, round 17.

Round 18: form.exponent built and walked; bark plates 1 m (preset, identity re-pinned). raw/final17/five-seeds.png: five of five show substantial tapering limbs under a dome. RESULT.md, round 18.

Round 19: form.ripening built and walked. raw/final18/: fine limb tips and a twig haze at the crown edge (seed4-final17-vs-final18.png). RESULT.md, round 19.

Round 20: values only; the trunk persists as the leader and forks late. raw/final19/: deep rounded crowns; limbs at several heights at four of five seeds (seed 1 still low). RESULT.md, round 20.

Round 21: form.dominance built and walked; limbs 0.3. raw/final20/: 3 to 4 major limbs per seed. RESULT.md, round 21.

Finish (round 22, a8daf966 and 3bd91bc7): round 20's values with a 9-cycle clear bole for foliage lower on the tree (owner's note); raw/final21/five-seeds.png. Crate tests 43 passed; workspace gate `cargo test --profile ci --workspace --no-fail-fast` 1018 passed, 0 failed, 21 ignored.
stage: impl-review - ran (codex fan-out, three draws, then one re-review): NEEDS_WORK. Fixed: dbh on the trunk's relays, SOURCES.md final values, R4 for the final beech. Surviving: four P1 engine continuity findings (viability crossing with relay 1; relay position by integer node counts; blend-zero relay straightening on a lateral; relay section on a discrete parent node), declined as engine design changes to the owner-passed tree, for the host.
Known gaps: Astra's branch-hierarchy critique, the tube-like branch surface, the young ages (10 and 20 years are poles with rods).
Tier: session
stage: impl-review - OVERRIDDEN (host, 2026-10-04): the four surviving P1 findings are relay continuity on walked settings, not on the passed beech; they move to fn-199 (relays change the tree by degree), which fn-198 now depends on, so production does not ship them.
Passed on the owner's verdict ("to me they look really good structurally"), round 20's values with foliage lower on the tree.

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 648d6405, 068d1ded, 02a9aa61, 57ab643f, fbcf6c3d, 71dbaa3b, 9acc6dda, f85225c1, 5696674e, 75f733ba, 3c89f5a4, a666161a, 6bb530aa, 39ab1ba7, 52c14a78, aae9a76e, a5d6ec51, eaedb3d3, c08bb389, 903ade4f, 90cc1a6f, e179c8dd, a8daf966, 3bd91bc7
- Tests: cargo test --profile ci -p telperion-space --no-fail-fast, cargo test --profile ci --workspace --no-fail-fast
- PRs: