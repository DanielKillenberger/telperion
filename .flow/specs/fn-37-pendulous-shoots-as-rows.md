# Pendulous shoots as rows

## Goal

One of the fn-34 round-3 gaps (`.flow/evidence/fn34/GAPS.md`): the birch was a rounded mop where the photograph is a weeping curtain of long thin shoots almost to the ground, leaves strung along them, crown base 0.20 against the photograph's 0.08. The generator already had a curtain mode, but hidden and fixed: a negative secondary rise flipped it on as a switch, its droop was a constant (0.35 cap, 0.5 slope), its shoot length borrowed from the twig anatomy, its spacing hard-coded at 4 degrees, and no leaf could point down. This spec turned the curtain into rows a value table sets and the blend walks.

## Decisions

- > user (2026-09-14, after fn-34 round 3): "so i would like these species to inspire the generator to improve. I guess we can try and get as close as possible with values and then make a gap analysis what the generator needs and then spec that and once implemented do another comparison."
- Rows on the existing curtain over a gravity term in the bias field (the field is positional and cannot know which shoot is pendulous).
- Widen the canopy orientation rails to signed values rather than adding a separate leaf-hang row.
- The spruce keeps its curtain byte-identical at the values that reproduce the old constants; the birch is the proof.

## Acceptance Criteria

- **R1:** `skeleton.twigs.hang` (0 to 1, neutral 0) makes the pendant mode continuous; zero is byte-identical; no switch frame across any value — met.
- **R2:** Pendulous length, pendulous radius and curtain separation as railed, blended twig rows; spruce pins hold — met.
- **R3:** Signed canopy orientation rails so a leaf may lean down; shipped presets byte-identical — met.
- **R4:** Birch table uses the rows; matched pairs re-rendered beside round 3; owner verdict in fn-34 — met (round 4: five of eight numbers moved toward the photograph, S-BARE crown base within 0.004; birch accepted in fn-34).
- **R5:** Tests for predicate continuity, rails, floor, spruce identity, oak-to-birch sweep, signed rails — met.

## Result

The curtain moved into `branching/local/pendant.rs`; the birch states its curtain as rows (hang, pendulous length, spacing); every other table is byte-identical at neutral. A residual the rows could not close: droop capped at about nineteen degrees below horizontal (the old constant), which hang only scales down; deeper reach came with fn-44. The owner accepted the silver birch in fn-34; the beech's verdict moved to fn-62.

## Open

None in this spec; the curtain's reach and droop continued in fn-44.
