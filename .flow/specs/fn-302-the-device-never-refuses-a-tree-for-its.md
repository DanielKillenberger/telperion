# The device never refuses a tree for its hierarchy rounds

## Goal & Context

After fn-299, fn-298 and fn-301 merged into F4 (bb20f3877), the gate fails two tests on one frame. `the_film_walks_build_on_the_device` and `the_rounds_frames_have_the_cpus_counts` both refuse beech to spruce at 40, seed 1, frame 41 of 90 (t 0.4335089163237312): "the build passed its sizes at cycle 12: HierGens (0x1)". The release film's walk passes through this frame, so this spec joins the release (fn-237).

Measured by the host's probe (2026-10-21; uncommitted `zz_hier_probe.rs` in `.worktrees/f4-hier`, logs `bb20.log`, `bb20-cpu.log`, `c3ed.log`, `fn298.log` in the host scratchpad):

- **The tree is sound.** The CPU's hierarchy-generation count for the cycle-12 step is 4 (322 new axes). It is 4 on fn-298's tip too, and every other cycle reads 1 or 2. Nothing passes the plan's bound `b.hier.gens = max(GENS = 8, units)`.
- **Only the device's stepped projection is short.**
  - The projection is `rounds = max(read[GENS] + STEP[0], floors(units)[0])` (`cycle/build_size.rs:269`). `read` is the census two cycles back, which is 1, and `STEP[0]` is 2, so the stepped term is 3. The floor is `2·units − 1` (`cycle/build_arena.rs:127-129`).
  - The check that raises the flag is `GENS > rec::ROUNDS` (`cycle/plan_hier.rs:94`).
- **fn-299 exposed it; it did not cause it.** fn-299's rule (timing rows take the reaching end's values) drops the frame's `most_units` from 3 to 2. The floor therefore falls from 5 to 3. The ends each have `most_units` 2, yet the frame's step needs 4. So `2·units − 1` is not a bound on the generations one step adds. It covered this frame only while the old blend inflated `units`.

## Unknown, check before the plan

- What a hierarchy round costs when it has nothing left to do. Does the pass stop once no depth changes, or does it run every round it is sized for?
- Whether any other stepped limit relies on `most_units` in the same way.

## Design (host, 2026-10-21, after R2's measurement)

The bound is not free. Sizing the rounds at `b.hier.gens` costs +7.8% on beech 40, +12.4% on palm 12 and +1.7% on spruce 40, about 20 to 33 µs per idle round-pass of host recording. Deletion is therefore out.

The projection reads the wrong quantity. A cycle's generations are the layout's depth the cycle before (gens(c) = deep(c−1)). The census the host reads already carries that layout depth (`c::LAY_DEEP`, which `stepped` counts as `deepest` for `laygens`), and it is one cycle fresher than `c::GENS`. On frame 41 it reads 2 where `GENS` reads 1, and the cycle needs 4.

- **The rule.** `rounds` steps from the census's layout depth, not its `GENS`: `deepest + STEP[0]`.
  - `STEP[0]` is the largest rise from that read to the generations the cycle actually needs.
  - That rise is measured over every census of the full film-walk sweep (1,533 frames) and every shipped preset at its R1 ages, at seeds 1, 7 and 42. The measurement is recorded in `RESULT.md` with its maximum and where it occurs.
  - First verify gens(c) = deep(c−1) against the census on those builds. If it does not hold, stop and report.
- **The floor.** `2·units − 1` is no longer called a bound. The comment in `stepped` that claims "never past `2 units - 1`" is corrected, because frame 41 disproved it. The floor stays only if the sweep needs it with the new projection; otherwise it goes from `rounds`. `unit_rounds` keeps its floor.
- **`unit_rounds`.** No change. Report its measured headroom on the same sweep, the largest of `rounds_used(c) − rounds_used(read)` against `ROUND_SLACK`. If the headroom is 0 anywhere, stop and report.
- **Cost.** A paired timing of beech 40, palm 12 and spruce 40, base bb20f3877 against the change, must come out within noise.


## Requirements

- **R1 (repro):** the two failing tests above are red on bb20f3877, and they are the repro. Run them before anything changes.
- **R2 (question, delete, then optimise):**
  - First try deleting the projection: size the hierarchy rounds at their bound, `b.hier.gens`. Measure the cost with paired `build_time` runs on beech 40, palm 12, spruce 40 and this frame. If it is within noise, the stepped projection and its floor for HierGens go.
  - If the bound costs more than noise, the projection must take its step from what can actually happen, never from `most_units`. Then bring the measurement to the host before choosing a rule.
- **R3 (output unchanged):** rounds only size work. Every artifact digest of the shipped presets at seeds 1, 7 and 42 is byte-identical to bb20f3877, and so is the full film-walk sweep (`every_film_walk_frame_builds_on_the_device`, 1,533 frames). Every frame builds, and none is refused.
- **R4:** the gate is green once, with `npm test`, and a codex review on each task.

## Boundaries

- No change to growth, the blend or fn-299's rule.
- Other stepped limits change only if the Unknowns find the same `most_units` reliance. A finding there is brought to the host first.
