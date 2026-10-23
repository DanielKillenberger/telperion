# Friction: fn-310

## 2026-10-23, conductor, run start

- Doing: starting `/flow-next:flow --auto fn-310-a-person-for-scale-and-a-nicer-world` in worktree f4-310.
- Hindered by: the spec has no ready flag, so `pilot snapshot` reports `eligible: false` (route `host`, rule "spec not ready"). `flowctl spec ready` is documented as a person's consent gate, so the conductor cannot set it on an agent's instruction.
- Cost: one round trip to the host before any planning; about 5 minutes and a few thousand tokens.
- Would have removed it: marking the spec ready (`flowctl spec ready fn-310-...`) in the same commit that recorded the owner's pick (201288747), before dispatching the run.

## 2026-10-23, task 2 worker, the person's placement

- Doing: D2's placement against the in-frame acceptance (every shipped preset at the hero shot, 16:9).
- Hindered by: the hero frame puts the ground at the trunk's foot near the frame's bottom edge, so any foot stood toward the camera drops below the frame. With the bounds' box as the footprint no offset the sun rule allows (at most 70 degrees left of the camera, so the foot stays within 90 degrees of the 135 sun) keeps the oak in frame: the first test run failed at y = -1.27, and a projection sweep in Python over offsets 0 to 70 found none. The inscribed ellipsoid (`camera.rs` `reach_of`, the crown the hero pose fits) at offset 68 and margin 0.5 m keeps every tree in, the feet at y = -0.97 at worst.
- Cost: one failed test run and about 15 minutes of geometry sweeps.
- Would have removed it: checking the in-frame AC against the hero frame's geometry before the spec stated an offset rule and a sun condition together; the feasible window is narrow (offset about 64 to 70).

## 2026-10-23, task 3 worker, the base build and the frame-time runs

- Doing: building the base `201288747` for frame time and Wasm bytes in a scratch directory.
- Hindered by: the session's scratchpad is shared with earlier runs, so `base/` and `wasm-base.log` already existed from an earlier export. I had to check that the base's `telperion-render` sources matched `201288747` file by file and that its binary carried no branch-only shader text before I trusted a number. dcg also refused a `>` redirect to a variable path, so the runs went through a script with literal paths.
- Cost: about 5 minutes and two extra tool rounds. The builds were cheap: 13 s for the base `frames` example with the worktree's `ci` deps copied into its own target, and 49 s for each render Wasm.
- Would have removed it: a fresh, task-named scratch subdirectory per worker (for example `scratchpad/<task-id>/`) chosen by the conductor.

## 2026-10-23, task 4 worker, the start-time diagnosis

- Doing: confirming task 3's +1.7 s `fresh_ms` under equal cache state.
- Hindered by: the NVIDIA shader disk cache on this machine (`~/.cache/nvidia/GLCache`) is full at its 1 GiB ceiling and takes no new entries, so every `fresh_ms` taken with the default cache depends on which shaders happened to be cached before it filled. Task 3's +1.7 s was that, not the branch; it cost task 3's NEEDS_HUMAN and this task. dcg also refused a heredoc that wrote a script to a variable path, so the script went through the Write tool.
- Cost: this task, about 25 minutes and five probe rounds (about 3 minutes of GPU time); the code side cost nothing.
- Would have removed it: the `frames` probe (or `docs/gpu-tests.md`) stating the driver cache state for `fresh_ms`, for example running with `__GL_SHADER_DISK_CACHE=0` for a cold number or a fresh `__GL_SHADER_DISK_CACHE_PATH` per run. The full cache itself is a local setup problem on the owner's machine (clearing or raising `__GL_SHADER_DISK_CACHE_SIZE`), reported, not specced.

## 2026-10-23, conductor, the first gate

- Doing: the declared gate at a1b170043, after tasks .1-.4 (void by the host once task .5 was added, but run to its end).
- Hindered by: two failures the tasks' focused runs could not see: `tint_rule_tests::every_shipped_shader_keeps_tints_rules` (a lib test; task .1's WGSL mixes `*` and `^` without parentheses) and `conformance::the_renderer_library_never_names_a_family` (task .2's bounds table names presets in `camera.rs`). Task .1's worker ran conformance before task .2 broke it; neither ran the lib's tint rule test.
- Cost: both fixes and a second gate (about 21 minutes of gate each), plus a worker round.
- Would have removed it: a short, sub-second "static checks" list beside the two pre-gate guards (`generation_limit_guard`, `compile_size`) that every render task runs before review: the lib's `tint_rule_tests` and `tests/conformance.rs`. Both are fast and catch exactly this class.

## 2026-10-23, task 6 (worker): the scratch "base" is not 201288747

Doing: reusing task .3's base export for the film's base-against-HEAD timing, as the dispatch suggested. Hindered: `scratchpad/base` is an older export, not 201288747 (it has no `examples/film*`, and dozens of `telperion-render` and `telperion-core` files differ from `git archive 201288747`). Cost: about 3 minutes to check, and a fresh export and cold build under its own target dir (`scratchpad/t6base`). What would remove it: a base export named by its commit (`scratchpad/base-<sha>`) with a `COMMIT` file written beside it, so a reuse is one `cat` instead of a tree diff.
