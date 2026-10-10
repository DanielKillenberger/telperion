# fn-254 friction

## 2026-10-09 22:03, task 1: start
- Doing: the Phase-1 anchor.
- Hindered: `flowctl` is not on PATH in the worker's shell; found it under the plugin cache by hand.
- Cost: about 1 minute.
- Would remove it: `flowctl` on PATH for dispatched workers (local setup, not a repository change).

## 2026-10-09 22:20, task 1: the before timings
- Doing: the cold `grow` compile before any change (`hang_tests::browser_route`, driver cache off), three runs under the GPU lock.
- Hindered: load average 22 to 34 from other sessions; `grow` made in 28 to 33 s (fn-235 measured 13 to 17 s) and `girths` in 2.1 to 7.3 s, so one run fails the existing 20 s and 2.5 s bounds by load alone. `cargo nextest` is not installed here, so the gate runs as `cargo test --profile ci`.
- Cost: about 10 minutes; the before and after are measured again side by side at the end.
- Would remove it: a quiet window for compile timings, or a probe that reports every pipeline's make time without asserting (written here as `cold.sh` in the scratchpad).

## 2026-10-10, task 1: two continuity walks fail at f32's own resolution
- Doing: getting `light_changes_the_tree_by_degree` (sky, seed 3) and `landing_on_the_ground_walks_by_degree` green after the move to f32.
- Hindered: neither failure is a defect a code change resolves. The refinement asks for continuity at 1/262144 of a sky step and 1/32768 of a sag step. Below that scale the f32 tree is flat between some adjacent inputs and jumps between others. On sag, one step of the setting moves a branch by 5.5e-4 of the tree's height, while 12 of the 16 finer samples are identical. On sky, rounding noise is amplified about 300-fold through the light sums and the balance memory, and it flips one subtree's shed decision (axis `760add40`, `t` crossing `d`). It took instrumented tree diffs to tell this apart from a real switch in the code.
- Cost: about 3 hours of probes and roughly 25 compile-and-run cycles.
- Would remove it: a decision from the host on how deep the walk refinement goes, or how much noise it tolerates, under f32 (it is set by host decision 9 of 2026-10-05). Alternatively, a refine that stops once its sub-step reaches the setting's own f32 resolution.
