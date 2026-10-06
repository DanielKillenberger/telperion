# fn-209 F1 task 5, item 2: the sag lay's frame correction on the option-4 path (worker, 2026-10-06)

**Finding: an accident, not a decision.** Not fixed, per the host's direction of 2026-10-06 (fn-211, DESIGN-INTERLEAVE.md, "After Astra's review", item 3): option 4's committed boundaries leave production for realized mechanics.

## What happens

- **Whole-tree growth** (`grow/settle.rs`) lays the tree with `place`, gathers the levers, then lays it again with `place`. The second lay starts from the first, so each phytomer's `heading` and `side` are the first lay's. `Layer::step` turns the lever by the rotation from that frame to the frame as it is now. As a branch droops, its lever turns towards the vertical, its torque shrinks, and a heavy branch hangs and stops (fn-203).
- **Option 4** (`bounded/work.rs`) lays each chain twice too.
  - Its bent slice is taken from `Bounded::bent`. The tree reached `Bounded::settled` with `settle: true`, which returns before any lay, so the bent phytomers are unlaid: heading and side are zero.
  - `framed` then finds no frame and returns the lever unturned. The correction never acts, on any chain, above the cut or below it.
  - Nothing in option 4's design or code comments asks for that. The step's own comment says the frame is "still the first lay's".
- The GPU executor mirrors the CPU reference (task 4) by passing no first-lay frame.

## Its size

The one-line fix is in `work`, before the lay under the levers: `bent[first..].clone_from_slice(&unbent[first..])`. It was measured with `contract dump` on this branch: the four species, ages 40 and 80, seeds 1 and 7. Raw output is in `raw/frame/` (gitignored), compared with the example's new `against` mode, which matches each whole tree against the same tree grown alone, by lineage.

- **Topology:** unchanged.
- **The beech and the palm:** unchanged (no PA sags).
- **The oak:** whole tree only, unchanged.
- **The spruce, grown alone, against itself before the fix:** tips move at most 0.06 to 0.13 m at 40 and **4.4 to 5.1 m at 80**, RMS **about 1 m** at 80.
- **The spruce grown alone, against whole-tree growth** (tips of shared lineages, max / RMS):

| Tree | Before the fix | With the fix |
|---|--:|--:|
| spruce 40 1 | 0.36 / 0.031 m | 0.36 / 0.030 m |
| spruce 40 7 | 0.45 / 0.030 m | 0.45 / 0.029 m |
| spruce 80 1 | 5.24 / 1.03 m | 1.43 / 0.22 m |
| spruce 80 7 | 4.38 / 1.01 m | 2.28 / 0.26 m |

So without the correction, the old spruce's limbs droop about a metre further than whole-tree growth draws them. With it, the difference falls to about 0.2 m, which is the size of option 4's other differences (committed rather than realized loads).

## Not done, and why

The fix was reverted. The realized-totals change replaces this path: each fine root reports its realized totals and the CPU settles as today's engine does, so the lay under the levers will be the whole tree's own `place`. When that lands, the GPU executor's bent pass should pass its first lay's frame (`was_heading`, `was_side` in `lay.wgsl`'s `step`, from the unbent fields 12 to 17), as `place` does.
