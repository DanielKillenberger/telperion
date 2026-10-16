## Goal

All wood took the trunk's bark colour, so the silver birch's hanging curtain drew in white and the crown read as frost, where the photograph's shoots are dark red-brown. This spec gives the material a young-wood colour and a radius below which wood takes it, blending continuously to the bark colour as the wood thickens, neutral at today's single colour, judged on the birch's pairs.

## Decisions

- user (2026-09-15): "Please do visual QA on these before submitting another review from me. If you can see it's clearly not there and you have a way to fix it you should do that."
- worker read, fn-44 round 6b (2026-09-15): "thinning made the white shoots more prominent, not less — with fewer leaves covering them the crown reads frosted where the photograph's twigs are dark ... now the single loudest fault in the image."
- worker read, round 7 (2026-09-15): "The white fine shoots, which make the S-BARE winter crown a white fog."
- Keyed on radius (already a continuous wood varying), not branch order or age. Rows, not species; no lichen, lenticels or peel (fn-40); no geometry change.

## Acceptance Criteria

- **R1:** `material.shootRed/Green/Blue` (0 to 1) and `shootRadius` (0 to 0.1 m), neutral radius 0, refused by name, wire, blend, metadata, harness; neutral stills byte-identical — met.
- **R2:** wood thinner than the radius takes the shoot colour and blends to bark by twice it, native and browser, with mottle, cavity and occlusion applied — met.
- **R3:** birch dark red-brown and beech olive-brown stated; pairs re-rendered; implementer visual QA; owner judges in fn-34 — met (round 8, `round8-fn46` rows in `.flow/evidence/fn34/rounds.tsv`; beech stated, not rendered).
- **R4:** tests for neutral identity, rails, blend ends and continuity, native/browser agreement — met (`crates/telperion-render/tests/shoot_colour.rs`).

## Result

Birch: dark red-brown under 1 cm (VT Dendrology, Atkinson 1992); beech olive grey-brown under 4 mm. A host-authorised value pass matched the birch's leaf faces to S-WHOLE's leaf pixels. Host read of round 8: the first birch that reads as a weeping birch, dark strands under green leaves on two white stems; the leaf mass far too dark (35 against 83), and no leaf row reached it (handed to fn-52). Oak hero frame 3.59 ms median against 3.60 ms. The owner accepted the silver birch at fn-34 round 25 (2026-09-16); the beech's verdict moved to fn-62.

## Open

None.
