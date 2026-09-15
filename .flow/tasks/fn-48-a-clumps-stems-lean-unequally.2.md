---
satisfies: [R1, R2, R3, R4]
---
# fn-48-a-clumps-stems-lean-unequally.2 The second stem parts at a stated height

## Description
The owner on round 13 (2026-09-15): "Can we also have the second trunk be
moved up/down?" and then "if we can add it with a short task to the
existing spec we can add it. If it's nothing major." Every stem in a clump
is born at the root today (fn-38), so a two-stemmed birch always parts at
the ground. This task adds one habit row, `skeleton.habit.stem_fork_height`
(0 to 0.5 of the crown base's height, neutral 0): stems after the first are
born from the first stem at that height instead of at the root, so the
clump can part at the ground, a metre up, or anywhere below the crown.

The change stays inside the clump: the scaffold's stem frontier
(`branching/scaffold/stems.rs`), the surface's trunk-run marking
(`surface/paths.rs`, `surface/samples.rs`: a stem born on a stem is a
trunk run with a fork socket where it leaves, not a buried root), the
radius solve (unchanged, the pipe model shares the girth at the fork), the
species metrics' DBH (a fork above breast height is one stem there), and
the row's plumbing. If the work turns out to reach beyond these, stop and
report rather than widen it.

## Acceptance
- [ ] **R1** `stem_fork_height` railed and refused by name, on the wire,
      blended, on the harness, in the regenerated metadata; every shipped
      preset byte-identical at 0.
- [ ] **R2** at a positive height the later stems leave the first at that
      height, each swept as a trunk run with a fork socket at its base, and
      the girth below the fork is the pipe model's sum; DBH counts the stems
      at breast height.
- [ ] **R3** the birch states a fork height against S-WHOLE and S-BARE, its
      pairs are rendered again, the implementer does visual QA before
      returning, and the owner judges in fn-34.
- [ ] **R4** tests: neutral byte identity, the rail, the fork's height and
      socket on a synthetic clump, the DBH count below and above breast
      height, and a blend walk from 0.

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
