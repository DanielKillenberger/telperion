---
satisfies: [R1, R2, R3, R4, R5]
---
# fn-37-pendulous-shoots-as-rows.1 Implement Pendulous shoots as rows

## Description

The curtain the spruce and the birch already grew was a hidden mode: a
negative secondary rise flipped it on, its droop was a 0.35 cap and a 0.5
slope written into the twig advance, a pendulous shoot borrowed its length
from the twig anatomy, and neighbouring shoots stood a hard-coded four
degrees apart. This task turns all of that into four twig rows a value table
can set and the blend can walk, widens the canopy's four orientation rails to
their signed ranges so a leaf may hang under its shoot, and tunes the silver
birch's table against its S-WHOLE and S-BARE references with the new rows.

## Acceptance

- [x] **R1** `skeleton.twigs.hang`, 0 to 1, neutral 0. The pendant predicate is
      a function of it; the droop cap and slope scale with it; a family at hang
      0 is byte-identical to the tree it grew before the row existed, and the
      whole departure of a curtain lateral walks in from the one the twig law
      asked for, so no value of the row is the frame where a shoot changes
      kind. Refused by name outside its rail.
- [x] **R2** `skeleton.twigs.pendulousLength` (0.05 to 5 m),
      `pendulousRadius` (0 to 1 of the root radius) and `curtainSeparation`
      (1 to 45 degrees) are rows: railed and refused by name, on the wire, in
      the blend, in the browser metadata (regenerated, never hand-edited) and
      on the harness's own dials. The spruce's table states the values that
      reproduce the constants — hang 1, a pendulous length equal to its twig
      length, every shoot under a descending limb, four degrees apart — and its
      identity pin holds byte for byte.
- [x] **R3** `canopy.forwardLean` and `upward` accept -1 to 1, `leanRise` -2 to
      2 and `outward` -1 to 1; each is refused by its own name outside the
      widened rail. Every shipped preset at its current values is unchanged;
      the oak, the spruce, the beech and the Two Trees are byte-identical.
- [x] **R4** The birch's table uses the rows; its matched pairs are rendered
      again and the crown base and occupied numbers on S-WHOLE and S-BARE are
      recorded beside round 3's in `.flow/evidence/fn34/REPORT.md`. The pairs
      themselves stay on disk under the ignored `measure/` directory and are
      recorded by sha256 in `.flow/evidence/fn34/round4-fn37/stills.json` with
      `visual_status: unassessed`. **The owner's verdict is the open slot.**
- [x] **R5** Tests cover the predicate's continuity across zero
      (`tests/pendulous.rs`), the four rails, the floor under any hang, the
      spruce's stated constants, the oak-to-birch sweep walk (`tests/sweep.rs`)
      and the signed canopy rails on placements
      (`tests/foliage/attachments.rs`).

## Progress

TBD

## Done summary

TBD

## Evidence
- Commits:
- Tests:
- PRs:
