# The fidelity dial and the coarse rows are gone

From 2 to 14 Nov 2026, during 0.2.0's development, a family carried a `/fidelity` parameter, and the oak, the spruce and the birch carried six `/space/coarse` rows (`first`, `factor`, `leaf`, `bud`, `hazard`, `shedding`) saying what the dial did to that species at its coarsest (fn-325, fn-326, fn-328). 0.1.4 had neither. The rows were fitted per species, which the owner ruled out (3 Nov 2026: "This shouldn't be per species"), and nothing shipped read them: every preset stated `fidelity` 0 and no entry set another value. fn-367 deleted the rows, the dial and `Species::coarsened`. No shipped tree changed: every preset's digest at `fidelity` 0 is the same before and after.

## What is refused

Every entry reads the family through `params::parse`, so `TreeEngine.build`, `growField`, the renderer's `setTree` and `beginGrowth`, and a native `params::parse` or `params::overlay` refuse alike.

- `/fidelity`, by name:

  ```text
  /fidelity is retired: the per-species coarse rows it dialled were deleted (fn-367), and every tree grows the full cycle
  ```

  `TreeEngine.build` and `growField` throw it as `Tree core: /fidelity is retired: ...`. The `Family` type no longer has the field, so TypeScript refuses it first.
- Any `/space/coarse/<row>`: the space rows' reader knows no `coarse` group, and the family is refused as `space row` (`Tree core: space row` in the browser).

## What to do

Delete `fidelity` from the family and the `coarse` object from its `space` rows. Nothing replaces them: every tree grows the full cycle, which is what `fidelity` 0 grew, so removing them from a family at `fidelity` 0 changes nothing fn-367 did to its tree. A family at any other value grew a coarser tree that no entry can grow now.
