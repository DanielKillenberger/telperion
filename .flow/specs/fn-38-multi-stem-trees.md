# Multi-stem trees

## Goal

A fn-34 round-3 gap (`.flow/evidence/fn34/GAPS.md`): the birch in S-WHOLE stands on two stems that part at the ground, one leaning out; the generator grew one stem from the origin and every downstream stage assumed it. Birches, hazels, alders and coppiced trees grow this way. This spec added stems as rows (count, divergence, lean), one stem neutral, with the birch as the first specimen.

## Decisions

- > user (2026-09-14, after fn-34 round 3): "so i would like these species to inspire the generator to improve. I guess we can try and get as close as possible with values and then make a gap analysis what the generator needs and then spec that and once implemented do another comparison."
- Stems are order-zero siblings born at the root, not a basal fork born as a lateral: the bole gates forbid crown axes in the bole, and a stem must be a trunk to the surface and the radius solve.
- A count row walked like lobe counts over a boolean; no family field is a switch.
- Boundaries: no root system, coppice regrowth or stem death (fn-16, fn-20).

## Acceptance Criteria

- **R1:** `skeleton.habit.stems` (1 to 6), `stem_divergence` (0 to 120 degrees), `stem_lean` (0 to 45 degrees), railed, blended, in metadata; every preset byte-identical at one stem — met (birch re-pinned once).
- **R2:** Two or more stems grow from the root, spread and leaned, inside the envelope, each a trunk run with buried root and flare, no fork socket at the base — met (refused only when degenerate).
- **R3:** Pipe-model sharing of the base radius; DBH reports the largest stem with the count; twig eligibility keys off the largest stem — met.
- **R4:** Birch declares two stems; pairs re-rendered; owner verdict in fn-34 — met (birch accepted in fn-34).
- **R5:** Tests for neutral identity, rails, stems apart and inside, trunk runs, DBH with count, one-to-two walk, stem birth years — met.

## Result

A table can declare a clump of stems; the birch stands on two. Round 6: S-BARE plainly reads as two stems from one flared foot; S-WHOLE does not because the curtain covers the base. Owner items raised at round 6: both stems lean symmetrically where the photograph has one near-vertical and one leaning far out (a per-stem lean row would be a new spec). The owner accepted the silver birch in fn-34; the beech's verdict moved to fn-62.

## Open

A per-stem lean, if the owner wants the photograph's asymmetric pair, is a new spec.
