# Tree space: sag measures its lever on the bent branch

## Goal & Context

fn-200's sag takes each phytomer's bending moment on the tree as it stands before bending (small-deflection beam theory). That holds while branches bend a little. The spruce's round 9 (fn-194) gives its old lower boughs the heavy sag S1's downward sweep needs, and there the approximation fails: a bough already hanging near vertical still carries the full horizontal lever of its unbent shape, keeps bending past vertical, and with its tip tropism turns back up into loops of wood around the trunk base (`.flow/evidence/fn-194-…/raw/round9/close-ups.png`). In a real beam the lever shrinks as the beam droops, so a heavy branch hangs and stops; it does not coil (host, 2026-10-05).

## Design (host, 2026-10-05)

- **One march from base to tip.** Along each axis, the moment at a phytomer is taken from the load beyond it, with that load's positions rotated by the bends already applied to the phytomers before it (its bearer's and its own axis's upstream bends). The lever is the horizontal distance in that rotated pose. As an axis droops toward vertical its lever shrinks toward zero, and the bend stops by itself.
- **No iteration and no second rule.** The load's own internal shape is the unbent one, rotated rigidly; this is the first-order large-deflection correction and is deterministic and continuous. The 0.15 rad stop short of straight down stays as a backstop only.
- **Tip tropism unchanged.** Tropism acts after sag as today; an unloaded tip still keeps it.
- **Neutral and evolution.** With sag 0 nothing changes. Trees with small sag change little; the spruce and any tree with heavy sag change by design (AGENTS.md "Generator evolution"); the beech (sag 0) stays byte-identical.

## Requirements

- **R1:** A test that a long, heavily loaded horizontal branch converges to hanging and never bends past vertical along its length, red first on the small-deflection sag.
- **R2:** A test that a lightly loaded branch bends within a small tolerance of the small-deflection result (the correction is first-order).
- **R3:** fn-200's walks (sag, ground landing) and every crate test green; the walk bound of 30 holds on `form.sag` up to the spruce's heaviest value.
- **R4:** The spruce's round-9 values re-rendered at seeds 1 and 4, trunk-base close-ups and whole tree, viewed by the host: no loops.
- **R5:** Workspace gate; Codex review.

## Boundaries

Geometry only, in the sag pass. No growth feedback (reaction wood is phase E or later).
