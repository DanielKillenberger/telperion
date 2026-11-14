# Tree space: Telperion

## Goal

Telperion, the elder of the Two Trees, on the engine path per the host's brief (fn-198 VISION-TWO-TREES.md): the beech's architecture older and loftier; leaves dark green above, silver beneath; countless white-silver blossoms; cool silver light falling as dew; waxing as Laurelin wanes. With the group off, a convincing very old beech. Looks better than today's harness Telperion (owner, 2026-10-07: full creative freedom for the Two Trees).

## Decisions

- Cycle strip and light are after the release (fn-232; host, 2026-10-08, fn-217 DESIGN.md section 6). Scale and judging follow Laurelin (fn-218).
- Values: the `european-beech` rows at `/age` 92, then the group: scale 2.4 (47-50 m against Laurelin's 44.5), dark fronts, silver underside by a warm-tinted strong mirror (the cameras below the crown see mostly undersides, so they set the crown's colour at distance), flowering 1.0 on age 14, upright candles of 9 florets (hanging clusters read as speckle), cool white petals, grey eye.
- **His age is the release's ceiling:** 92 is the oldest the browser grows (`WASM_KEPT` refuses 93-110); 120 runs the device out of memory; 140 exceeds the wood buffer; 160 the engine budget. Round 13 at 110 (scale 2.2) passed with the host and Astra and waits for fn-209's fine path to lift the Wasm bound.
- Astra: DISAGREE at rounds 7 and 10, AGREE at 11, 13 and 14 ("mature, old beeches"); round-14 polish notes optional, not applied.
- Owner's species verdict (2026-10-15, fn-237 R3 table): 'can accept for now'; flowers and magic after the release. fn-286's take-over 0.5 approved.

## Acceptance Criteria

- **R1:** Telperion's values in the `telperion` preset; builds natively and in the harness — met (native and Wasm byte-identical).
- **R2:** Sheets off and on at whole, mid and close views; host confident and Astra agrees — met (round 14 AGREE).
- **R3:** Built from fn-217's group and fn-234's petal albedo: dark above, silver beneath, white-silver blossom distinct at whole-tree view; scale chosen and contact error measured — met.
- **R4:** Gate and npm test green; cost at his scale measured on the native route — met.

## Result

- Off hashes to the beech's mesh at one age (`the_two_trees_with_the_group_off_are_the_beech`).
- Contact (age 92, seed 42, scale 2.4): 16.6M leaves, step 1.50 mm, worst 0.47 mm (0.56 % of the smallest leaf), inside fn-125's screen.
- Cost: CPU route 8.0-14.3 s, 9.1M-16.6M leaves, 10.4M-17.8M florets (~2.3x the beech); device route 4.0-4.5 s, peak RSS 391-479 MB.
- Host reading: off an open-grown old beech; on a silver-green crown with white across it, still a tree; close up florets crowd into papery white masses.
- Gate on `52ab0912`: render 207 passed, workspace 1,067 passed, npm test 143 passed.

## Open

- Age: set `/age` 110 and scale 2.2 when fn-209's fine path lifts the Wasm bound.
- Darker fronts need the leaf's transmission and sheen under the group.
- Astra's optional polish (smaller, less overlapping blossom; brighter undersides).
- The sheet example's close camera can frame a hidden floret.
- Owner: flowers and magic after the release.

## Friction

- Four rounds tuned a leaf face the cameras barely see -> proposed a face mask (front share per view) printed by the sheet example.
- Age ceilings found by failing renders (engine, device, Wasm), the tightest last -> proposed a size estimate and a `foreseen` vs `WASM_KEPT` check in the sheet example, and preparing each sheet tree on the resident route.
- An all-SHIP review fan-out's finalize refused its merge plan (no findings container) -> proposed treating a SHIP draw with no findings as an empty list.

## Evidence

- `.flow/evidence/fn-219-tree-space-telperion/SHEET.md` (values, ceilings, Astra rounds, contact, cost; cited by `telperion.values`)
