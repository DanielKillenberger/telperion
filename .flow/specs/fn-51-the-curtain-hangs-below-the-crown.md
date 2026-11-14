## Goal

A weeping birch's strands fall past its crown, some nearly to the ground. Telperion kept every node inside the envelope's shell, so after fn-47 the birch still read as a round crown with a lumpy lower edge. This spec lets hanging shoots fall past the shell's lower surface, by a row, down to a clearance above the ground, neutral at today's containment.

## Decisions

- host, on fn-47's result (2026-09-15): "The fix is to let the curtain hang below the outline, down to a set height above the ground, as the photograph's left side does to about a metre. That reverses a fn-44 decision ... I'd do it: weeping shoots really do hang below the crown's main mass."
- user (2026-09-15): "makes sense"
- worker, fn-47 (2026-09-15): "While fn-44 keeps the curtain inside the shell, every column's longest strands end on its lower surface. Letting the curtain hang past the shell's lower surface to a ground clearance, or bunching it per limb, is a spec decision."
- Only hanging shoots, only below the shell's lower surface and under the crown's footprint; the repo-wide containment invariant becomes "inside the envelope, or a hanging shoot within the drop band", exactly the old one at drop 0.

## Acceptance Criteria

- **R1:** `curtainDrop` (0 to 1, neutral 0) and `curtainClearance` (0 to 5 m, never above the crown base) railed, refused by name, wire, blend, metadata, harness; presets byte-identical at neutral — met (birch re-pinned with the reason).
- **R2:** hanging shoots fall into the band, nothing else leaves the shell; containment tests assert the new invariant and the old at drop 0 — met (species.rs, outline.rs, growth.rs, monthly boundary test).
- **R3:** birch states a drop; pairs rendered beside round 8b; visual QA; owner judges in fn-34 — met (drop 1 to a 1.8 m clearance, its crown base; round 9, `round9-fn51` rows in `.flow/evidence/fn34/rounds.tsv`; 48/48, heaviest birch seed 121,765 nodes).
- **R4:** tests for neutral identity, rails, band at 0.5 and 1, only hanging shoots, clearance, determinism, blend walk — met (`tests/drop.rs`, eight tests).

## Result

Implemented through `Curtain::admits` in place of the shell check in advance.rs and planner.rs. A 1.0 m clearance hid the stems behind one wall of curtain, so 1.8 m. Host read of S-WHOLE: the curtain falls past the old round bottom into a skirt with the two stems clear below; the ball on a stick is gone. Still wrong: symmetric silhouette, and the strands fill the width as one wall where the photograph hangs in separate fringes per limb; no row reached the fringes. The owner accepted the silver birch at fn-34 round 25 (2026-09-16).

## Open

None.
