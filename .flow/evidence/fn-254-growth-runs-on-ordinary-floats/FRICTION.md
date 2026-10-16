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

## 2026-10-10, task 1: stills "beside master's" compare two generators
- **Doing:** R5's whole/mid/close stills beside master's.
- **Hindered:**
  - Master (`ebd2e37b`) still draws the pre-engine tree, so a sheet beside master's cannot show what the floats changed. I rendered and viewed a third set from the base (`59e627e7`) to isolate them.
  - Separately, the session's temp directory hit its disk quota (EDQUOT) during a base build, and a command's output was lost.
- **Cost:** about 15 minutes and one extra build of the base outside `/tmp`.
- **Would remove it:** for a spec on an F branch, the R5 wording "beside the base's", and a scratch location outside the tmpfs quota for second builds.

## 2026-10-11, task 1: merging F4 leaves one owner-derived bound on its edge
- **Doing:** the gate on the merge of F4 (`3cb7793a`) into fn-254.
- **Hindered:** `thinning::far_florets_drawn_as_cards_cover_what_the_florets_cover` fails. Its bound, `BLOSSOM_BOUND` = 0.30, is the measured 25.4% plus 5 points under the owner's floret-card decision (fn-246). The F4 tip passes at 29.8% (0.02714 against 0.02091); fn-248's tree change had already moved it from 25.4%. With the floats on top it reads 30.5% (0.02728 against 0.02091). The fn-254 branch passed alone. Re-baselining a bound the owner's decision set is not mine to make.
- **Cost:** about 20 minutes, including one build of the F4 tip to confirm where the failure comes from.
- **Would remove it:** a bound that records its own drift. Alternatively, re-taking it at "measured plus 5 points" whenever the tree moves under it, as AGENTS.md "Generator evolution" allows with evidence.
