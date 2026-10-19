# Flowers hang from the tree

## Goal

The owner, looking at Laurelin and Telperion in the harness after fn-248 (2026-10-09): "flowers seem placed randomly and attached to the air. The shape looks good." Two causes were on record: the inflorescence's axis and stalks placed florets but were never drawn (fn-217's review), and every node of a flowering shoot became a flowering site (fn-234), not the shoot tips where racemes and clusters hang. Host (2026-10-09): release item for fn-237 (the Two Trees). Scope: flower seating and stalks.

## Decisions

- R1 measured: nine in ten inflorescences stood along a short shoot's four years of wood, often two or three to a shoot; every floret hung 5 to 39 cm from its wood (median 17 cm Laurelin, 12 cm Telperion) with nothing drawn between. CPU and device agreed.
- R2: tip seating with no new row; `Bloom.side` deleted; share 1 on the Two Trees to hold the count near the owner-approved build (tips cap it at one per short shoot).
- Host decisions on R3 (2026-10-10): the pedicel is part of the floret at a fixed width and length that never scales with it, its ends the rachis point and the floret's base after every transform; the rachis is one instance per inflorescence along the curved axis, in the existing flower draw; no new rows (rachis tapers from the twig radius to the stamen width, in the shoot's colour); a terminal raceme continues the shoot's direction and the droop bends it. The rachis was first exempt from thinning; inflorescences then thinned and culled as whole units.
- Host decision (2026-10-10): the rachis's cost accepted (+2.9 % Laurelin, +4.1 % Telperion at the whole view; ~2 % was the host's bound, not the owner's, within the comparison's ±3 %). No sub-pixel rachis cutoff.
- Host decision (2026-10-10): fn-246's per-floret card stays (the owner accepted it by eye, "almost no difference from reference"); its test fixture moved to the beech at 40, bound at the measured +30.1 % plus 10. The per-inflorescence card (+87 to +131 %) stays on branch `fn-253-unit-card-trial`.
- Owner verdicts (2026-10-11, in the harness): stalks "yes the flowers lok like they are attached"; "laurelin looks pretty good"; Telperion "i think the density is a little much can barely make out individual flowers. Also leaves seem to be missing? but it's pretty good."
- Telperion's leaves were hidden, not dropped (980,082 leaves either way; florets at share 1 covered twice the frame, showing leaves fell ~40 %). Host (2026-10-11) set Telperion's flowering share to 0.3 from the share sheet; Laurelin stays at 1.
- Owner verdict (2026-10-11, judging page `GJV93Q6kLQhAgJ4UVWmYVp`): Telperion at share 0.3 "30% is good"; flowers hang from wood on both trees: "yes".

## Acceptance Criteria

- **R1:** where each inflorescence is seated and what connects it to the wood, both executors, against the code — met.
- **R2:** inflorescences seated at shoot and short-shoot tips by degree; neutral values byte-identical — met.
- **R3:** rachis and pedicels drawn as thin stalks in the existing organ path, endpoints computed after every floret transform; dormant without a rachis — met.
- **R4:** close and mid stills viewed, every floret hung from wood; a connectivity test on both executors with non-unit scales, active thinning and partial survival; counts and timing; CPU and device agree; frame rate held; Astra's agreement; gate and npm test green; the owner's verdict — met (Astra round 2 AGREE, codex SHIP).

## Result

- Tests: tip seating, stalks in their levels, every floret hung on both executors through selection with thinning, units kept or dropped whole, ribbons by chord error, forced levels; one packing box on both executors (`foliage::bloom_box`) fixed fn-234's parity test.
- Telperion at share 0.3, seed 42: 420,016 florets and anchors; 115.1 fps whole, 49.2 mid, 54.5 close (mid under 60, for fn-259). Laurelin 173.4 / 76.7 fps whole / close, level with fn-248.
- npm test 155. Catalogue family and walk digests re-pinned for Telperion's one row.

## Open

- Laurelin's live gold reads denser than the 4x reference at the whole view.
- Astra's polish: more curve in Laurelin's long rachises.

## Friction

- `flowctl` not on the worker's PATH -> dispatch names the `flowctl` path (proposed).
- Fresh worktree builds cold (27 s core, 82 s render) -> accepted.
- Organ-mask coverage does not follow floret count, and fn-248's coverage measured the picture not the mask -> share set by inflorescence count; the coverage measure needs fixing (reported).
- The approved look sat in another worktree's ignored `raw/` -> keep verdict sheets as compressed JPEGs beside the spec (now the retention rule).
- fn-246's card level hid every rachis -> found by looking; fixed.
- Two along-shoot fixtures went red only at the gate -> run affected fixtures when seating changes (proposed).
- Shell hook blocks common edit idioms -> local setup, reported.

## Evidence

- `.flow/evidence/fn-253-flowers-hang-from-the-tree/STALKS-laurelin.jpg` (Laurelin's sheet: whole, mid, close and raceme views)
- `.flow/evidence/fn-253-flowers-hang-from-the-tree/STALKS-telperion-share.jpg` (Telperion at shares 0.3, 0.5, 1; the 0.3 verdict)
- `.flow/evidence/fn-253-flowers-hang-from-the-tree/STALKS-telperion-merged.jpg` (Telperion at 0.3 on the merged build, as approved)
- `.flow/evidence/fn-253-flowers-hang-from-the-tree/SEATS.md` (the full record, kept in line numbering because fn-237's species audit cites its lines)
- `.flow/evidence/fn-253-flowers-hang-from-the-tree/FRICTION.md` (cited by fn-237)
