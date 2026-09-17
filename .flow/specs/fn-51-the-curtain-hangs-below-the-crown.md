## Conversation Evidence

> host, on fn-47's result (2026-09-15): "The fix is to let the curtain hang below the outline, down to a set height above the ground, as the photograph's left side does to about a metre. That reverses a fn-44 decision ... I'd do it: weeping shoots really do hang below the crown's main mass."
> user (2026-09-15): "makes sense"
> worker, fn-47 (2026-09-15): "While fn-44 keeps the curtain inside the shell, every column's longest strands end on its lower surface. Letting the curtain hang past the shell's lower surface to a ground clearance, or bunching it per limb, is a spec decision."

## Goal & Context
<!-- scope: business -->
<!-- Goal & Context: 25% [user], 45% [paraphrase], 30% [inferred] -->

A weeping birch's curtain hangs below its crown: the limbs make the crown's shape and the strands fall from them past it, some nearly to the ground. Telperion keeps every node inside the envelope's shell, so the curtain ends where the shell's lower surface is, and after fn-47 varied the strands' lengths and a lumpier shell roughened that surface, the birch still reads as a round crown with a lumpy lower edge. The owner agreed the curtain should hang below the crown. [paraphrase]

This spec lets hanging shoots fall past the shell's lower surface, as a row, down to a clearance above the ground, with zero reproducing today's containment. [inferred]

## Architecture & Data Models
<!-- scope: technical -->

- **A drop row.** `skeleton.twigs.curtain_drop`, 0 to 1, neutral 0: how far below the shell's lower surface a hanging shoot may fall, as a share of the distance from that surface to `curtain_clearance` (metres above the ground, 0 to 5, default 0.5). At 0 the shell binds a hanging shoot as today; at 1 a hanging shoot may fall to the clearance. Continuous in the row, so no value is a frame where a shoot changes kind. [inferred]
- **Only hanging shoots.** The exemption applies to shoots whose curtain hangs (`Curtain::hangs`) and only below the shell's lower surface: above it and outside the crown's footprint the shell binds as before. The floor (`pendant.rs`, walked to the crown base by the sag) walks on down to the clearance by the same row. [inferred]
- **Containment says what it means.** The repo-wide invariant that every node past the crossover lies inside the envelope (`tests/species.rs`, `tests/outline.rs` and others) becomes: inside the envelope, or a hanging shoot's node within the drop band under the crown's footprint. Every test that asserted containment asserts the new form, and at drop 0 the old one holds exactly. [inferred]
- **The birch is the proof.** Its table states a drop against S-WHOLE and S-BARE: the photograph's left-hand curtain reaches about a metre off the ground while the stems stand clear in the middle. [user]

## Edge Cases & Constraints
<!-- scope: technical -->

- **Neutral is inert.** Every shipped preset at drop 0 is byte-identical, asserted by the pins. [paraphrase]
- **Ground.** No node falls below the clearance; the clearance never exceeds the crown base. [inferred]
- **Stems stay visible.** A strand's band is under its own limb's footprint, so the bole below the crown base is not curtained unless the limbs above it hang there. [inferred]
- **File sizes.** `advance.rs` stays under about 400 lines; the band test lands beside the curtain in `pendant.rs`. [paraphrase]

## Acceptance Criteria
<!-- scope: both -->

- **R1:** `curtain_drop` and `curtain_clearance` are twig rows with rails, neutral at drop 0, refused by name outside them, on the wire, blended, in the regenerated browser metadata and on the harness; every shipped preset byte-identical at neutral. Errors: a value outside its rail is refused naming the field. [inferred]
- **R2:** With a positive drop, hanging shoots fall past the shell's lower surface into the band down to the clearance, and nothing else leaves the shell; every containment test asserts the new invariant and the old one at drop 0. Errors: a node outside the envelope and the band fails the test naming the seed. [paraphrase]
- **R3:** The silver birch's table states a drop against S-WHOLE and S-BARE, its pairs are rendered again with the numbers and fn-47's hem statistics beside round 8b's in `.flow/evidence/fn34/REPORT.md`, the implementer does visual QA before returning (a curtain falling below the crown to a ragged fringe, the two stems visible in the middle), and the owner judges in fn-34. Errors: a rejecting verdict stops the spec with the owner's words and a one-paragraph blocker. [user]
- **R4:** The tests cover: neutral byte identity, the rails, the band on a synthetic curtain at drop 0.5 and 1, nothing but hanging shoots in the band, the clearance, determinism and a blend walk. Errors: a missing case is a review finding, not implementer discretion. [inferred]

## Boundaries
<!-- scope: business -->

- No change to the sag law, strand variation or separation; fn-44's, fn-47's and fn-37's. [paraphrase]
- No shoot colour (fn-46), no per-stem lean (fn-48), no leaf lighting (its own spec). [paraphrase]

## Resolved via Codebase

- The shell rejection of a candidate: `crates/telperion-core/src/branching/local/advance.rs:199` (`rejected(config, p) || s.curtain.below(p.y)`), the deferral at `:211`.
- The floor: `crates/telperion-core/src/branching/local/pendant.rs:53-60` (the sag walks the first descending ancestor's tip down to the crown base), `below` at about `:68`.
- The containment invariant: `crates/telperion-core/tests/species.rs:207` and the outline tests.
- The birch's curtain and outline: `crates/telperion-core/src/presets/species.rs` (hang 2.4, sag 1, pendulous length 3, variation 0.95, irregularity 0.35 at 0.25).
- The round-8b hem statistics: `.flow/evidence/fn34/REPORT.md`, round 8b.
