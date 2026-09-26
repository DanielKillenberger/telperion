---
satisfies: [R1, R2, R3, R4]
---
# fn-121-the-conductor-carries-a-tuning-pause-as.1 Implement fn-121-the-conductor-carries-a-tuning-pause-as

## Description
TBD

## Acceptance
Every R-ID in the parent spec's ## Acceptance Criteria is satisfied; judge this task against the spec's criteria directly.

## Done summary
The conductor now carries a tuning pause as its own. After `tuning-loop run` exits, `conductor/tuning.rs` reads the tuning run's `run.json`. A `pause` there becomes the conductor's pause under the same id, identity and action, with the tuning run's reason and decision request and a handoff whose signals name the tuning run. `species-conductor resume --decision FILE` on that pause runs `tuning-loop run --resume FILE` in the same directory. A revision that ends is recorded, and its gaps are checked on the next step.

- R1: `a_tuning_pause_becomes_the_conductors_pause_with_its_handoff` (tests/conductor_tuning_pause.rs).
- R2: `resume_runs_the_tuning_run_in_its_directory_then_every_gap_is_checked`. If either side refuses the decision, the run is left as it was: `a_decision_the_tuning_loop_refuses_leaves_the_run_paused`.
- R3: the same resume test checks every non-passing gap through the scripted transport and gets reachable, covered and new.
- R4: the gate is green, 973 passed, 0 failed, 21 ignored, summed over 132 `test result:` lines.

A revision counts as ended only when its record holds neither a pause nor an attempt in flight. `result.json` alone does not show that, because the tuning loop rewrites it on every save. The run record gains an optional `tuning_pause` (the revision). `Executor::tune` takes `resume: Option<&Path>`. `step.rs` fell from 403 to 299 lines. The conductor tests' scaffolding moved to `tests/conductor_support/mod.rs`, which both test files share. `docs/species-conductor.md` describes the behavior. Friction: the gate's cold build took 521 s (FRICTION.md).

Tier: session

stage: impl-review - skipped(config: REVIEW_MODE=none)
## Evidence
- Commits: 47da598b6f73253617e54f2b5bcce681cad3dd16, b5dd103d4f3ddb2f453927178978663110087375
- Tests: cargo test --profile ci --workspace --no-fail-fast (rc 0; 132 test result lines: 973 passed, 0 failed, 21 ignored; log .flow/evidence/fn-121-the-conductor-carries-a-tuning-pause-as/raw/gate.log), cargo test --profile ci -p telperion-jev --test conductor_tuning_pause --test conductor --test owner_first
- PRs: