# The device and the CPU grow the same rounds frames on the release branch

## Goal & Context

fn-298 R4 (host, 2026-10-21) holds the device's node counts exact to the CPU's on 9 `rounds` walk frames. Only spruce to birch seed 42 frame 56 was moved to fn-300. On fn-298's own branch that held.

On F4 after the fn-299, fn-298 and fn-301 merges (bb20f3877), frame 41's refusal hid the test's later frames. fn-302 makes frame 41 build, and then `the_rounds_frames_have_the_cpus_counts` fails on four frames, measured by fn-302's conductor on 2026-10-21:

| Frame | Instances, device / CPU | Nodes, device / CPU |
|---|---|---|
| spruce to birch, seed 7, frame 63 | 187,997 / 187,999 | 382,987 / 382,999 |
| oak to palm, seed 1, frame 36 | 157 / 160 | 20,205 / 20,250 |
| oak to palm, seed 1, frame 37 | within 1 | 19,985 / 20,032 |
| oak to palm, seed 1, frame 48 | within 1 | 4,785 / 4,910 |

None of the four retries under fn-302, and their device digests are identical to bb20f3877's. Which merge moved them is unknown. Candidates:
- fn-299 changed the blended timing rows, and with them the frames' trees.
- fn-301 changed the lay walk and the rest on both executors. Lay positions feed growth.

These are release film frames, so the spec joins the release (fn-237).

## Acceptance Criteria

- **R1 (bisect, measured):** run the four frames' counts at fn-298's tip 55c19adef, at c3ed89f12 (fn-299 + fn-298) and at bb20f3877 (+ fn-301), each in its own target directory. Name the merge that first breaks agreement. Then find the first cycle where the device's node count leaves the CPU's, and the quantity behind it.
  - **Measured (2026-10-21):**
    - The four frames agree at 55c19adef. They part at c3ed89f12, which brings fn-283 (rachis) as well as fn-299. bb20f3877 and 6198fbc9a have c3ed89f12's node counts.
    - Growth never parts. Axes and phytomers grown are equal at every cycle, 1 to 40, on all four frames.
    - Oak to palm 36, 37 and 48: the CPU tree's rachis tail accounts for every node apart. Less those nodes, the CPU's counts are the device's: 20,205, 19,985 and 4,785.
    - Spruce to birch, seed 7, frame 63: a 10-node twig at 0.00022 m on lineage 0xe7a34cf416e9d0b0 falls under `FINEST` on one side, after continuous growth differences (fn-300's kind).
  - **Host rulings (2026-10-21):**
    1. **Oak to palm 36, 37 and 48.** The comparison counts the CPU tree up to `tree.clothing`, because the device draws the rachis separately by design (fn-283), the same reasoning as `leaf_bases` in `nodes_off`.
       - The cut sits in the one place both count helpers read, so `apart` and `nodes_off` agree.
       - A comment names fn-283.
    2. **Spruce to birch, seed 7, frame 63: it moves to fn-300.** It gets its own ignored test, whose reason names fn-300 and states its counts. RESULT.md records three facts for fn-300's R1:
       - the lineage;
       - the `FINEST`-threshold link;
       - the `below` word lead on frame 48.
    3. **The instance gaps are measured before planning:** frame 36's 3 and spruce to birch's 2.
       - Frame 36's gap folds into ruling 1 if it is the same counting difference.
       - Spruce to birch's 2 go to fn-300 if they are the dropped twig's instances.
       - Anything else stops and goes to the host.
       - The instance check stays "within 1".
    - Before the first commit, the probe-only edits are removed, unless the fix needs one.
  - **Measured (2026-10-22): oak to palm frame 36's 3 leaves.**
    - Relay axis 978 continues axis 393 from its node 59. The CPU computes the join's t as 0.9999992; the device writes exactly 1.0.
    - So the CPU draws the join's node (the own wood above is under `FINEST`), and the device draws the phytomer's own node.
    - The node's key differs, and it keys the run's cauline draw: 3 leaflets on the CPU, none on the device.
    - Frames 37 and 48 have no such gap. RESULT.md has the paths with file:line.
  - **Premise check (2026-10-22): the end-join premise fails.** Over the presets at their R1 ages and seeds 1, 7 and 42, 15,167 of 17,962 CPU relay joins sit at t < 1 − 1e-4, and 12,727 join short of the parent's last node. The least t is 0.00037. Relays join inside their parent by `relay_at` and blend, so the first design (every relay at its parent's end) was withdrawn. RESULT.md has the table.
  - **Host ruling, revised (2026-10-22): the design for frame 36.** It replaces the first design.
    - **The defect is in `stand::node_x`** (`telperion-space/src/geometry/stand.rs`).
      - It picks a span by `before + scale >= length`, then measures the share as `(length − before) / scale`. At the span's end the two disagree in floating point: the comparison says the length reaches the end, the division says 1 − δ.
      - For a relay standing at its parent's tip, `attach` passes exactly the tip ((at − tip)·b = 0). `total` is the same left fold the loop runs, so `before + scale == tip` holds exactly, yet the share comes out 1 − δ.
    - **The rule.** The share is exactly 1 when the length reaches the span's summed end, decided by the comparison that chose the span.
      - In `node_x`: if `length >= before + scale`, return `(k, 1.0)`; otherwise `(length − before) / scale`.
      - This is no tolerance: one comparison defines the boundary.
      - Joins truly inside are untouched, because their length is below the span's end.
    - **The device mirrors `node_x` exactly** (`lay_stand.wgsl`, and `settle_thick.wgsl`'s fall-through): the same left-fold order for the summed length, the same comparison, the same share-1 rule. A length at the tip yields the same position on both executors.
      - `dress`'s at-the-end test and the CPU's `space.rs:436` read that position identically: t == 1 at the last node and the past-the-last-node form are one case.
      - One fold order is named in a comment on both sides. Where the device's tip sum is not the same fold, it is made so.
    - **Proof:**
      - Frame 36 agrees: nodes exact, instances within 1.
      - A unit test on each executor: a length equal to the summed tip, over spans whose (tip − before) / scale rounds below 1, gives share 1.
      - Rulings 1 and 2 are applied, and the rounds-frames test is green.
      - The digest table against 6198fbc9a over the presets at seeds 1, 7 and 42, giving which rows change and why. Stills only if a shipped preset's default-age leaves change visibly; otherwise one changed row's numbers.
      - The relay survey's joins at t ≥ 1 − 1e-4 are re-run on the change and read exactly 1.
    - The probes leave before the first commit. The relay survey stays only as an ignored test, if it is cheap to keep.
- **R2 (fix):** if one executor computes a quantity differently, fix it so the two agree. Then all 9 `rounds` frames hold fn-298 R4 again (nodes exact, instances within 1), except frame 56, which stays with fn-300.
- **R3:** if the divergence is fn-300's kind of last-bit amplification rather than a difference in computation, stop and bring the measurement to the host. Do not narrow the test.
- **R4:** output changes only where the fix requires it, and is measured. The gate and `npm test` are green, with a codex review per task.
- **R5:** a span's share is exactly 1 where the length reaches the span's summed end, on both executors, by the comparison that chose the span (host, 2026-10-22). The CPU's span position and the device's mirror name one fold order in a comment, and share one function name.
  - Proof: a unit test on each executor. A length equal to the summed tip, over spans whose (tip − before) / scale rounds below 1, gives (last node, 1).
  - Errors and boundaries:
    - A length 1 ulp under the span's end keeps the computed share.
    - A length past the summed tip keeps today's fall-through, (n, 0), which `split` and the device's `attach_main` read as (n − 1, 1).
    - Zero-scale spans are skipped as today.
    - NaN falls through as today.
    - The rule applies at any node: a length at an interior node's end gives (k, 1), an end join at node k on both executors (`space.rs` `t >= ONE`, `dress` `at_end`).
- **R6:** the node-count comparison counts the CPU tree only up to its clothing tail (`Tree::clothing`, fn-283's rachis and the leaf bases), in the one helper `apart` and `nodes_off` both read (host ruling 1). A comment names fn-283.
  - Errors: a tree with no clothing counts every node, as today. The instance check is unchanged and stays "within 1".
- **R7:** spruce to birch at 40, seed 7, frame 63 leaves the fn-303 test for its own ignored test, whose reason names fn-300 and states its counts as measured on the final code (host ruling 2).
  - `the_rounds_frames_have_the_cpus_counts` then holds all 9 `rounds` frames but fn-300's two (frames 56 and 63), with no fn-303 ignored test left.
  - RESULT.md keeps fn-300's facts: the lineage, the `FINEST` link, the `below` word lead, and the 2 leaves on shared wood.
- **R8:** the output change is measured (host, 2026-10-22):
  - Oak to palm frame 36 agrees: nodes exact, instances within 1.
  - The device digest table against 6198fbc9a (`digest` at seeds 1, 7 and 42, and `palm_digest`), each changed row with its cause.
  - Stills only if a shipped preset's default-age leaves change visibly; otherwise one changed row's numbers.
  - The relay survey re-run on the change: every join whose length reaches its span's end reads exactly 1. Joins with t in [1 − 1e-4, 1) are counted and shown to stand below their span's end.

## Boundaries

- Not fn-300's general agreement past age 12.
- Not fn-302's retry.

- Not the general float agreement of blends, light or positions (fn-300). R5 removes one comparison-against-division mismatch and nothing else.
- Not other span picks with the same shape (`zone_node`, the sag's assignment). They decide no join and stay as they are.

## Edge Cases & Constraints

- The share-1 rule moves a relay's base onto its parent's node exactly where the length reaches the span's end. Relay bases feed the layout and the light, so growth outputs can change by last bits on both executors. Growth agreement is re-checked by the four frames' census (R1's probe) and the gate.
- Frame 63's counts in its ignored test are re-measured after R5, because R5 moves the layout.
- The digest table groups changed rows by cause: a relay at its parent's tip; a join at an interior node's end; layout drift from either.

## Strategy Alignment

Active tracks served by this plan:
- **The core and integration**: the generator owns the CPU reference and the device executor of one contract. This keeps the two drawing the same tree for the release film.

## Decision Context

- **Withdrawn (host, 2026-10-22):** every relay joins at its parent's end. The relay survey refuted it: 15,167 of 17,962 joins sit inside.
- **Rejected:** snapping t within an ε of 1. It moves the knife-edge.
- **Chosen:** the comparison that picks the span also decides the share is 1.
- Maintainability (plan review): duplication - the span-share rule and its fixture inputs are mirrored in Rust and WGSL, as R5 requires; structure - none identified.

## Early proof point

Task fn-303-the-device-and-the-cpu-grow-the-same.1 validates the core approach: the share-1 rule on both executors makes oak to palm frame 36 agree. If frame 36 still parts after it, re-measure where the two executors' lengths for relay 978 differ before continuing with fn-303-the-device-and-the-cpu-grow-the-same.2.

## Memory findings

No relevant entries in project memory.

## Quick commands

```bash
# The share rule's unit tests, CPU and device
~/.local/bin/telperion-heavy 8 --threads 2 -- cargo test --profile ci -p telperion-space --lib geometry::tests
~/.local/bin/telperion-heavy 8 --threads 2 -- cargo test --profile ci -p telperion-render --lib generation::cycle::settle_tests
# The rounds frames' counts
~/.local/bin/telperion-heavy 8 --threads 2 -- cargo test --profile ci -p telperion-render --lib -- --exact generation::film_walk_tests::the_rounds_frames_have_the_cpus_counts --nocapture
```

## Requirement coverage

| Req | Description | Task(s) | Gap justification |
| --- | --- | --- | --- |
| R1 | (bisect, measured): run the four frames' counts at fn-298's tip 55c19adef, at c3ed89f12 (fn-299 + fn-298) and at bb20f3877 (+ fn-301), each in its own target directory. Name the merge that first breaks agreement. Then find the first cycle where the device's node count leaves the CPU's, and the quantity behind it. | — | Measured before planning (2026-10-21/22); RESULT.md holds the bisect, the census and the traces. |
| R2 | (fix): if one executor computes a quantity differently, fix it so the two agree. Then all 9 `rounds` frames hold fn-298 R4 again (nodes exact, instances within 1), except frame 56, which stays with fn-300. | fn-303-the-device-and-the-cpu-grow-the-same.1, fn-303-the-device-and-the-cpu-grow-the-same.2 | — |
| R3 | if the divergence is fn-300's kind of last-bit amplification rather than a difference in computation, stop and bring the measurement to the host. Do not narrow the test. | — | Resolved by host rulings: frame 63 (amplification) moved to fn-300 under R7; frame 36 was a computation difference (R5). |
| R4 | output changes only where the fix requires it, and is measured. The gate and `npm test` are green, with a codex review per task. | fn-303-the-device-and-the-cpu-grow-the-same.2 | — |
| R5 | a span's share is exactly 1 where the length reaches the span's summed end, on both executors, by the comparison that chose the span (host, 2026-10-22). The CPU's span position and the device's mirror name one fold order in a comment, and share one function name. | fn-303-the-device-and-the-cpu-grow-the-same.1 | — |
| R6 | the node-count comparison counts the CPU tree only up to its clothing tail (`Tree::clothing`, fn-283's rachis and the leaf bases), in the one helper `apart` and `nodes_off` both read (host ruling 1). A comment names fn-283. | fn-303-the-device-and-the-cpu-grow-the-same.2 | — |
| R7 | spruce to birch at 40, seed 7, frame 63 leaves the fn-303 test for its own ignored test, whose reason names fn-300 and states its counts as measured on the final code (host ruling 2). | fn-303-the-device-and-the-cpu-grow-the-same.2 | — |
| R8 | the output change is measured (host, 2026-10-22): | fn-303-the-device-and-the-cpu-grow-the-same.2 | — |
