# Tree space: the Oregon white oak's crown reads as an open-grown oak

## Goal

The owner's verdict on 2026-10-09, on the oak's field stills (fn-214 cut sweep): the new oak "look[s] weird". fn-226's R5 sheet showed the same against O-WHOLE (`.refs/fn9/quga788B.jpg`): today's oak at 80 was a smaller, open, irregular crown on a short trunk with long diagonal limbs past the crown's edge, where the reference and the old 0.1.4 oak are a broad, dense, closed dome. The gap predates fn-226's light work; it is the oak's own values and growth. Owner (2026-10-09): "make it all the way. Ship it all together": the release carries the engine, the species and killenberger.com's move off 0.1.4, so the oak must read as an oak in the harness and in the site's grove. The oak's values only, plus host-minted gap specs.

## Decisions

- R1 model (`MODEL-OAK.md`) before any value moved: sourced traits T1-T9 (broad, rounded, dense crown; width about height; short bole; few massive crooked ascending limbs) and derived target ranges (at 80: height 14-24 m, crown 10-20 m, DBH 41-81 cm; width/height 1.0-1.3). The species runner was not used (its Tune moves only catalogue dials, not `/space` rows).
- Host after round 1 (2026-10-09): leaf rows join the brief; size at 80 comes first; the ordinary tree keeps its rows (owner: "an ordinary size whatever"). After the leaf rounds: Astra found values not exhausted (reserve buds, fine-order `shadeSize`, fine-order placement); round 3 ran on those.
- **Owner verdict (2026-10-14), in the harness at the fork (`owner-2026-10-14-fork.jpg`):** "oak has multiple problems.. leaves growing way too numerous and on branches and it also has branches clipping in each other". Traced in `DEFECTS.md`; same on the release branch before fn-275, so not a regression.
- Host rulings (2026-10-14): (a, b) a value round: `leafYears` 1 on every age (Q. garryana is deciduous, FNA), `shootRadius` from sources, acceptance the sourced leaf area index (3.2-3.3 Phillips et al. 2003; mean 4.4 Karlik & McKay 2002). (c) limbs through each other: two host-minted gap specs (the fork's joined surface; limb avoidance). After round 4: `/canopy/size` to a typical blade (72.5 mm, the middle of FNA's 25-120 mm); structural rounds resume, at most three; the refusal at 80, seed 7 (`unsolved radii`) is the host's own spec.
- **Owner's choice of round (2026-10-15):** "round 4 looks ok but nothing like the reference" and "between round 4 + 5 i'd definitely choose 4". Round 4 with the 72.5 mm blade ships (`56c9c8a2d`, `5ae542cd2`); round 5 reverted (`6de1d30bd`).
- Seed 7's long zigzag low limbs (owner, 2026-10-15: "seed 7 specifically doesn't look like a real tree. Lower branches that grow to long and zigzag weirdly"): cause in `CAUSE.md` (young-stem sprigs 3.5 m long, relays hooking 57 deg); fixed by the sourced sapling segment (0.031 m, Hibbs & Yoder 1993) and fn-286's relay take-over. Owner: "definitely better".

## Acceptance Criteria

- **R1:** MODEL-OAK.md written before values moved, sourced traits and a capability assessment — met.
- **R2:** Rounds on the oak's `/space` rows, mirrored into `oak.rs`, stills at 40 and 80 beside O-WHOLE and 0.1.4, viewed, until they read as the reference — partial: round 4 chosen by the owner; the closed dome unmet.
- **R3:** The site's field at n = 36 and 48 beside 0.1.4's — measured at round 3 (three of six grove seeds still irregular at 40).
- **R4:** Build time and nodes measured each round; the coarse-field timing re-measured — met.
- **R5:** The owner's verdict on the final round — round 4 chosen (not the species verdict); seed 7 fix "definitely better".
- **R6:** Gate and npm test green; other species byte-identical — met.

## Result

- Round 3: closed crowns at 80 (width/height 1.09-1.12, gaps 0.14-0.16), but 342,878 leaves at 40 combed along older limbs.
- Round 4 (leaves on the current year's shoots): leaves in tufts at shoot tips, no comb; 47,248-70,247 leaves at 40; gaps 0.31-0.42; with the typical blade the leaf area over hull 1.9-2.8 at 40.
- Round 5 (trunk lives four years) widened toward O-WHOLE at 40 seed 1 but at 80 read bushy with foliage to the ground; reverted.
- Device warm build (round 4 blade): 247 ms at 40, 1,119 ms at 80, device peak 782 / 4,209 MB; the structure, not the leaf count, sets the time.

## Open

- The closed dome of O-WHOLE: fn-285, after the release (round 4 at 80, seed 1: gaps 0.43; age 120 refuses at the phytomer budget).
- Limbs passing through each other and the collar ring: the host's two gap specs.
- Low branches under the crown dying by degree: fn-282 (post-release).
- DBH at 80 runs 1.5-2.7 times the derived range.

## Friction

- Variable-path redirects blocked by the guard -> probes take an output path argument.
- No still tool for the 0.1.4 oak, no value-row writer on the branch -> keep each release's stills once in `.refs/`; merge `space_rows`.
- No measure of a closed crown to steer by -> a silhouette shot (flat background, unlit leaf mask) in the still runner.
- Other sessions' load and probe builds queued behind heavy jobs -> local setup.
- A leaf probe needed a rebuild (first after a merge over ten minutes) -> the still runner reading a value file behind a flag.
- An 80-year probe waited 34 minutes for 12 GB, then met a refusal round 3 already had -> a stored per-preset, per-age, per-seed build status.

## Evidence

- `.flow/evidence/fn-236-tree-space-the-oregon-white-oaks-crown/MODEL-OAK.md` (sources, target ranges, capability assessment)
- `.flow/evidence/fn-236-tree-space-the-oregon-white-oaks-crown/DEFECTS.md`, `defects.sh`, `oak_defects.rs` (the 2026-10-14 defects traced; cited by the preset)
- `.flow/evidence/fn-236-tree-space-the-oregon-white-oaks-crown/SHEET-4.md` (round 4's rows, sources and measures; cited by `radii_tests.rs`)
- `.flow/evidence/fn-236-tree-space-the-oregon-white-oaks-crown/CAUSE.md`, `seed7/` (seed 7's low limbs; cited by tests)
- `.flow/evidence/fn-236-tree-space-the-oregon-white-oaks-crown/oak_measure.rs`, `measure.sh`, `photo_crown.py` (the form measures)
- `.flow/evidence/fn-236-tree-space-the-oregon-white-oaks-crown/owner-2026-10-14-fork.jpg` (the owner's verdict still)
