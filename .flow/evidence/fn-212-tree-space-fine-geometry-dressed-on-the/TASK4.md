# fn-212 task 4: fine nodes in the tree's frame, and the whole-tree folds, on the device (worker, 2026-10-07)

The first part of CONTRACT.md's probe is built and measured: steps 2 and 5. D3(a) is not built. It is a design question for the host (below).

## Answer in brief

- **The device draws every fine root as stage 2's conversion draws it, in the tree's frame.**
  - `dress.wgsl` runs once per chain, wave by wave, after phase 4.
  - It applies the conversion's rules: `FINEST`, `SHORTEST`, the relay's joint, its behind and inside merges and its aside, the whole-axis rule, and runs and base radii.
  - It reads each root's laid buffer and its bearer's drawn node.
  - Wave 1 starts from each root's attachment as the pipeline drew its bearer (`Tree::attachments`, new) and from the bearer's node as laid under sag (the root's origin).
- **The folds combine exactly.** The core's folds are now combinable:
  - `executor::Folds`: the height, the crown's lowest node and the widest;
  - the shoulder's bins, combined by their maximum;
  - the leaf box, combined by its corners.
  - Over any split of a tree's nodes they reach the whole tree's to the bit (`parts_of_a_tree_combine_to_the_whole`). `grown_envelope` and `Reference::grown` are those folds, and every digest is unchanged (gate green).
- **Device against the CPU reference** (`dressed_at_40_and_80`, `raw/task4/dressed-2.txt`): beech and spruce at 40 and 80, seeds 1 and 7, against `executor::grow(&family)`'s tree.
  - **Nodes:** every device node has a reference node within fn-125's 50 µm. Each tree has 0 or 1 reference node without a device node, and 0 or 1 device node with no reference node anywhere. These are D1''s known residual: decisions near a threshold, which contract A removes.
  - **Folds:** combined from the device's nodes and the pipeline's own, they lie within 4.6 µm of the whole tree's.
  - **Envelope:** height and widest reach within 4.6 µm, shares within 3e-7, shoulder within 2.6e-6 relative.
  - **Leaf box:** within 2.7 µm.
- **The shoulder's front is a small summary.** The device hands on 11 to 78 crown nodes, read off the whole tree's bins. CONTRACT.md counted 3,567 to 3,988 on the spruce with one side's bins alone.
- **Cost on the device:** 44 to 45 ms for the spruce at 80, and 13 to 24 ms for the beech at 80. That covers drawing, all four fold passes and their three round trips, with buffers allocated per call. Today the CPU spends 1.0 s converting the spruce at 80 and 0.25 s on its envelope and inputs (TASK2.md baseline B).
- **D3(a) is not built.** A ground bound that is conservative before upload needs a bound on the trunk's own bend, and none exists. See "For the host".

## How it works

- **Drawn records** (`dress.wgsl`, `dress.rs`).
  - **One record per laid phytomer.** Each record holds:
    - the node it is drawn as: its own node, or the one it merges into;
    - that node's position (y up, tree frame), radius, start radius and base radius;
    - its run;
    - its laid axis;
    - its parent's position.
  - **One record per laid axis.** It holds how many phytomers the conversion took, the node the axis ends on, whether it took them all, the root's flags and its origin.
  - **Within a chain the conversion is sequential**, as on the CPU. A relay's joint, girth and aside change only nodes of their own wave. A merged record copies its node's final values for the next wave.
- **Folds** (`fold.wgsl`). Each fold is an atomic on a float's ordered bits.
  - `extent` and `widest` fold the height, the crown's lowest node and the widest reach, then the widest node's position. The CPU combines these with its own folds into the crown's shape.
  - `crowns` folds the shoulder's bins and the leaf box under that shape.
  - `sift` keeps the crown nodes that no bin beyond may pass.
    - The bins are the whole tree's: the device's and the pipeline's, combined.
    - A node is compared with the bin beyond its own, with 1e-5 slack. A float32 way may bin one off, and the bin beyond is still a valid witness.
    - The pipeline reads the shoulder off its own nodes and the sifted ones. The front is the nodes no other passes, so that is the whole tree's front.
- **Roots laid on the CPU** (host decision 3) carry a flag. The device never folds them; the pipeline's own nodes include them.
- **Readback:** four reads of summaries only (the sums, at most `CAP` crown nodes). The test reads the drawn records back for its comparison alone.
- **The leaf box's reach terms** come from the reference build's envelope in the probe. In production they come from the combined envelope, which agrees within the figures above.

## Measured (`raw/task4/dressed-2.txt`; seed per row)

| Tree | Device nodes | Reference fine nodes (seam subtrees) | Device nodes off every reference node | Reference fine nodes off the device's | Roots laid on the CPU | Folds (m) | Shoulder (rel.) | Box (m) | Kept | ms |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| beech 40 s1 | 96,612 | 96,613 | 0 | 1 | 4 | 2.7e-7 | 1.8e-6 | 1.5e-6 | 12 | 4.0 |
| beech 40 s7 | 169,332 | 169,332 | 0 | 0 | 3 | 1.5e-7 | 5.7e-7 | 1.4e-6 | 11 | 4.3 |
| beech 80 s1 | 1,468,344 | 1,468,344 | 0 | 0 | 16 | 4.6e-6 | 3.7e-7 | 2.7e-6 | 33 | 12.5 |
| beech 80 s7 | 2,434,977 | 2,434,978 | 0 | 1 | 29 | 9.0e-7 | 2.6e-6 | 1.4e-6 | 62 | 24.3 |
| spruce 40 s1 | 1,054,399 | 1,050,604 | 0 | 1 | 1,337 | 3.2e-6 | 4.5e-7 | 1.0e-6 | 78 | 10.1 |
| spruce 40 s7 | 1,033,791 | 1,029,757 | 0 | 0 | 1,021 | 3.3e-6 | 2.6e-7 | 1.3e-6 | 75 | 8.9 |
| spruce 80 s1 | 4,213,499 | 4,175,203 | 0 | 0 | 5,623 | 3.9e-6 | 3.2e-7 | 1.9e-6 | 60 | 43.7 |
| spruce 80 s7 | 4,235,815 | 4,196,975 | 1 | 1 | 4,843 | 4.2e-6 | 6.9e-7 | 1.7e-6 | 66 | 45.5 |

- **Why the spruce's device count exceeds its seam subtrees** (by 0.9 %): a fine root whose first phytomers merge into its bearer's node draws laterals from that node, outside the seam's subtree. The column "off every reference node" shows that each of those nodes is a reference node. The test's partition counts them on both sides, which a fold does not mind.
- **The fast test** (`fine_roots_are_drawn_and_folded_on_the_device`, beech and spruce at 30, in the gate) holds 0 strays both ways and every fold within the screen.

## Gate

- **CPU half:** 987 passed. The one red was the limit guard: the folds' sites had moved. They are now classified in `docs/generation-limits-inventory.json`, and the guard passes.
- **Render half,** under the GPU lock: 159 passed, 0 failed.
- `RAYON_NUM_THREADS`, `RUST_TEST_THREADS` and `CARGO_BUILD_JOBS` were 8.

## For the host: D3(a) needs a bound that does not exist

D3(a) asks for a conservative ground bound computed before upload, without laid geometry. Before upload (phase 1, `sow` and `grow`) none of the following exists yet:
- a root's scale (phase 2);
- its base radius (phase 2);
- its bearer's laid node (phase 3).

Scale and radius can be bounded from above:
- every scale is a product of shares of 1;
- the radius needs a bound not written yet.

The bearer node's height needs a bound **from below**. It depends on how the trunk leans and sags, and nothing bounds that today. fn-209 REALIZED.md (task 9, "Design questions") measured the nearest rule, `likely.rs`:
- **The rule:** the trunk stands as long as it is, and each branch hangs straight down from it.
- **Result:** it covers 4,904 of the spruce at 80's 5,623 landed roots (seed 1), even with scales and the laid trunk known.
- **It misses 719,** because the trunk sags too. It is a guess, not a bound.
- **With a bound on the trunk's sag,** the same path rule becomes a superset: about 19,000 of 74,000 roots at the spruce at 80, against 5,623 that land.

Options:

1. **(a) as asked, with a trunk-sag backstop.**
   - **What it needs:** a designed bound on the trunk's bend (a sag backstop) and an upper bound on a root's base radius.
   - **Effect:** it routes about 19,000 roots (estimated from REALIZED.md's count). That is about a quarter of the spruce's live roots, so the device's share falls from 91 % to an estimated 65 to 70 % (estimated by arithmetic; nothing was run).
   - **Its CPU work** is about 3.4 times today's landing.
2. **The guess before upload, with the landing kept as the check.**
   - **What it does:** route by `likely.rs`'s rule before upload. Any root the phase-3 or phase-4 bound then finds is laid on the CPU as today, and its device copy is masked (the `LANDED` flag this task built).
   - **Cost:** at least 719 roots on the spruce at 80 seed 1 still reach the device (fewer than today's 5,623). "Never reach the device" holds only for the roots the guess catches.
3. **Exact routing where it is decided today, masked on the device.**
   - **What it does:** every root still grows on the device in phase 1. A ground-routed root's device copy is never drawn, folded or surfaced: it carries this task's `LANDED` flag, as the probe already does.
   - **Cost:** the device grows and lays about 7 % of the spruce's phytomers for nothing (CONTRACT.md §2). No bound is needed, and the routing is exact.
   - **Optional:** it could also skip the lay and bend passes for those roots, since phase 3's routing is known before them, though phase 4's is not.

**The worker's reading:**
- Option 3 is what the readers already need, and it is built.
- Option 1 is the only one that keeps D3's "never reach the device" literally, and it moves the most work back to the CPU.

The choice is the host's.

## Files

| File | What |
|---|---|
| `crates/telperion-render/src/generation/fine/dress.wgsl`, `dress.rs` | the device's drawing and its buffers |
| `crates/telperion-render/src/generation/fine/fold.wgsl` | the folds, bins, box and sift |
| `crates/telperion-render/src/generation/fine/dress_tests.rs` | the probe against the pipeline |
| `crates/telperion-core/src/pipeline/input/folds.rs` | the combinable folds and their test |
| `raw/task4/` (ignored) | `dressed-1.txt`, `dressed-2.txt`, the gate logs |

## Host decision on D3 (2026-10-07)

- **D3 amended from (a) to (c)** (host, 2026-10-07), after task 4 showed that phase 1 has no floor for a bearer's height. Routing stays exact where it is decided today, in phases 3 and 4. The device masks a landed root with the `LANDED` flag: it is never drawn, folded or surfaced there.
- **Measured cost:** about 7 % of the spruce's phytomers are masked on the device: 6.9 to 7.9 % at 80, and 5.5 to 7.0 % at 40 (CONTRACT.md §2).
- **Option 1's estimate stands beside it:** about 19,000 of 74,000 roots routed, and the device's share down from 91 % to 65 to 70 % (estimated). Together with the 7 %, this satisfies the acceptance's "routed share measured".
- **Carried to task 7's handoff:** in production, the leaf box's reach terms come from the combined envelope, not the reference build's.
