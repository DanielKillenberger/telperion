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
- [x] **R1** `stem_fork_height` railed and refused by name, on the wire,
      blended, on the harness, in the regenerated metadata; every shipped
      preset byte-identical at 0.
- [x] **R2** at a positive height the later stems leave the first at that
      height, each swept as a trunk run with a fork socket at its base, and
      the girth below the fork is the pipe model's sum; DBH counts the stems
      at breast height.
- [x] **R3** the birch states a fork height against S-WHOLE and S-BARE, its
      pairs are rendered again, the implementer does visual QA before
      returning, and the owner judges in fn-34.
- [x] **R4** tests: neutral byte identity, the rail, the fork's height and
      socket on a synthetic clump, the DBH count below and above breast
      height, and a blend walk from 0.

## NEEDS_HUMAN - the ledge at the fork and the owner's verdict

Commits e06083da, b42f2ea9, 682077f0, 4ad41eda. Gates green. The birch
states 0.5, the rail's top (the fork node at 0.96 m): S-WHOLE's pair parts
at the ground, S-BARE's one trunk forks a third of its visible height up,
and the owner asked for the stem to move up. The worker's read: S-BARE now
stands on one trunk that forks, S-WHOLE no longer parts at the ground as
its photograph does, and the fork is not clean. The trunk run follows the
wider, leaning stem, and the socketed upright stem leaves the rest of the
trunk's girth as a ledge. A fix rendered clean (the trunk runs on into the
straighter stem and eases its girth into it). It needs a stem told from a
limb in the specimen's shoot-less trees (`specimen/history.rs`,
`specimen/interval.rs`, `specimen/view.rs` via `RunNode`) and the contact
query's run choice mirrored, which is past this task's surfaces: a
follow-up task. One commit is outside the named surfaces: b42f2ea9 has
`Tree::stem_radius` measure a clump's stems at the fork, where it fell back
to the combined trunk (+17 to 21% nodes on the birch); its callers all read
trees that carry bud fates, and aged packed reads are byte-identical with
and without it.

Host decision (2026-09-15): the owner allowed this task only "If it's
nothing major", and the ledge fix is not. a3a72a13 reverts the birch's fork
height to 0, so it parts at the ground as S-WHOLE's photograph does, with
no ledge and round 13's pins; the row stays for the owner to move. The
ledge fix is a follow-up task if the owner wants the stem moved up.

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
