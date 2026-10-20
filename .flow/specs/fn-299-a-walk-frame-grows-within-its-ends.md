# A walk frame grows within its ends

## Goal & Context

fn-298's survey (host, 2026-10-20) built every frame of the film's walks on the device. On oak to palm at age 40, the frames from t = 0.962 to t = 0.99998 (frames 106–119 of 120) are refused at all three seeds, 1, 7 and 42. Each grows 12 units in one cycle on a single apex, past the 11 a units job holds (`FRAMES`, `build_arena.rs`).

Neither end of the walk comes close:

| t | Most units per cycle on one apex |
|---|---|
| 0 (oak) | 2 |
| 0.5 | 3 |
| 0.9 | 7 |
| 0.95 | 9 |
| 0.962 to 0.99998 | 12 |
| 1 (palm) | 1 |

So a walk frame grows outside the range its two ends span. STRATEGY.md's tree space treats a walk as continuous between its ends; a count shooting past both of them as t approaches the palm breaks that. On the CPU the same frames do build, since it has no `FRAMES` limit. Whether those frames look wrong in the film is unknown.

The suspected cause comes from fn-298's reading of the code (`telperion-space/src/blend.rs`, `species/scale.rs`, and each frame's printed blended state). It has not been traced step by step:
- `lifespan` blends on a log scale toward the palm's 0 for the ages the palm skips (3 to 14). At t = 0.962, those ages last only 0.04 to 0.27 of a cycle.
- Age 2's `continuation` blends linearly, so it is still 0.038 at that t, and the oak's later ages stay reachable.
- One apex then passes through every one of those short ages in a single cycle.

fn-298's two `unit_rounds[0]` misses, oak to palm at seed 42, frames 76 and 77, may share this cause; nobody has checked.

Joins the release (host, 2026-10-20): the film shows this walk, and fn-298's device sweep leaves these frames ignored until this spec lands.

## Requirements

- **R1 (repro):**
  - A test reads the most units per cycle on one apex along oak to palm at age 40, at t = 0, 0.5, 0.9, 0.95, 0.962, 0.99, 0.99998 and 1, at seeds 1, 7 and 42.
  - It fails on the base wherever a frame goes outside the range its ends span.
  - A trace confirms which blended row or rows cause the overshoot, or refutes the suspected cause above.
- **R2 (the rule):**
  - A blended walk frame's per-cycle growth, meaning units per apex in a cycle, stays within the range of its two ends, up to a tolerance stated in the test, at every t.
  - The fix lies in how the rows blend (`blend.rs` or `scale.rs`), as a property of the blend, not as a clamp on the grown count. A row's blend that takes an age the other end does not have must not shorten that age toward zero while the age is still reachable.
  - Question first: whether any blend of `lifespan` and `continuation` is needed at all for ages one end lacks.
- **R3 (the walks):**
  - Every shipped walk is checked by the same test at the film's frames.
  - fn-298's ignored oak-to-palm frames are un-ignored and build on the device.
  - The two `unit_rounds[0]` frames either build, or are named as a different cause.
- **R4:**
  - Shipped presets at their walk ends (t = 0 and t = 1) are byte-identical to the base.
  - Walk frames between the ends may change, under AGENTS.md "Generator evolution". Show the changed frames as stills: oak to palm at t = 0.5, 0.9 and 0.97, base against change.
- **R5:** the gate and `npm test` are green, with a codex review per task.

## Boundaries

- Not the device limits (fn-298).
- Not the film's routing back to the device (fn-272).
