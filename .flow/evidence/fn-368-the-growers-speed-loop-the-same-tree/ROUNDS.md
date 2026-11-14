# fn-368 R2: the rounds

CPU seconds of the process up to the grown tree (`early`'s `cpu:` line, read before the digest), one thread (`telperion-heavy … --threads 1`), the round's binary and its base's run in turn, five pairs a tree (`ab.sh`, `ab.py`), median of each side; the spread is in `raw/<round>/ab/`. The spec's three plain runs a side (`measure.sh`) spread up to a quarter on the loaded desk (FRICTION.md 03:18), so a round is read from the interleaved pairs. Digests: `tree_digest` of the grown tree, every run of both sides.

| round | change | oak 40 (1001) | beech 80 (1) | spruce 40 (1) | oak 120 (1001) | peak MiB (oak 40 / beech / spruce / oak 120) | digests equal | kept |
|---|---|---|---|---|---|---|---|---|
| 0 | base, `d59085ecc` | 1.50 | 21.12 | 6.22 | 48.06 | 148 / 1371 / 722 / 1586 | (base) | |
| 1 | A kept subtree that faded out is retired at the living it was kept at, which is final, instead of being evaluated again (`retire.rs`) | 1.42 (−5.3%) | 20.37 (−3.6%) | 6.21 (−0.2%; pairs −4.8%) | 45.75 (−4.8%) | 149 / 1371 / 725 / 1587 | yes, all four | yes |

Round 1 gains under a tenth on both the oak at 40 and the beech at 80, so the loop ends there (R4). No round was reverted.

Digests: oak 40 10956267507761938372, beech 80 10028435592196877230, spruce 40 11334586712057171561, oak 120 15754524856295882608, the same on both sides of every pair.

R3 after round 1 (`tests.sh`, `raw/r1/*.log`): all ten tests of `grow/cycle/paused/tests.rs` (one more ignored), `many_cores_grow_the_tree_one_does`, `a_growing_phytomer_holds_what_growth_reads`, `an_axis_and_its_draws_stay_compact`, and the retirement's three equality tests against the reference retiring nothing (`the_working_set_and_retirement_leave_the_tree_as_it_was`, `woken_buds_that_stop_at_once_retire_as_the_reference_has_them`, `letting_go_of_retired_records_leaves_the_tree_as_it_was`) pass; `every_preset_grows_the_same_tree_again_and_on_any_cores` passes (258 s). The preset digest pin (`every_preset_grows_the_tree_it_grew_at_both_ends_of_the_dial`) is red on the base and red on round 1 with the same read, entry for entry (`raw/r0/pin.log`, `raw/r1/pin.log`; FRICTION.md 03:40): the pin was stale before this spec. No test named as a retired root's size exists; the nearest, `a_retired_root_let_go_is_read_as_it_was`, passes.

R5: `generation_limit_guard` (2 passed) and the render crate's `compile_size` (2 passed).

## The oak at 40 against 100 ms

The grown tree's own clock (`cycle: … s`, growth and its settle), no PAUSE inside:

- one thread, round 1: 1.44 s median (1.28 to 1.63), 14 times the aim;
- four threads, round 1: 0.68 s median (0.67, 0.68, 0.81; `raw/r1-t4`), about 7 times the aim.
