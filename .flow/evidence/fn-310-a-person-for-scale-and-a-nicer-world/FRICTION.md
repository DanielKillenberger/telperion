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
