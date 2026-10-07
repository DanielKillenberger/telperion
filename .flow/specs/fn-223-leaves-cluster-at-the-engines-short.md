# Leaves cluster at the engine's short shoots

## Goal & Context

The silver birch (fn-213) reached its second wrong-path stop on Astra's traits (j) and (h) on sheet 5: 'repeated feather-like leafy sprays attached to long exposed axes' and foliage that follows thin sprays. The cause, found by the run (SHEET-5.md): leaves are seated evenly along every thin shoot, one every 12 mm, so a long hanging shoot reads as a two-sided feather; a birch carries most of its leaves two or three together on short shoots, in clumps along the strand. The engine grows short shoots, but nothing seats a leaf cluster at them; the canopy's `/canopy/shortShoot*` rows scatter clusters by their own random placement, not at the engine's short shoots (claim from the run, to be checked against the code before this spec is marked ready). Host decision (2026-10-07): a capability, not more values. Many broadleaves (beech, oak, apple) bear leaves this way.

## Requirements

- **R1:** A short shoot the engine grows carries its leaves as one cluster at its tip (count and spread as rows), and a long shoot's evenly spaced leaves thin by a row, by degree; at the rows' neutral values every species byte-identical.
- **R2:** The cluster's draws keyed by station identity (fn-212's host decision, task 6), so both executors seat the same leaves.
- **R3:** One existing mechanism, not two: the canopy's random short-shoot clusters are either expressed through R1 or kept with a stated reason.
- **R4:** npm test and the gate green; stills of the passed species unchanged where the rows are neutral.

## Boundaries

Leaf seating only. Builds after fn-212's station work (tasks 6-7) lands on F4. The birch's values are fn-213's.