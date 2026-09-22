---
satisfies: [R1, R2, R3, R4]
---
# fn-108-register-the-date-palm-preset-with-the.1 Implement Register the date palm preset with the values the generator already reaches

## Description
TBD

## Acceptance
Every R-ID in the parent spec's ## Acceptance Criteria is satisfied; judge this task against the spec's criteria directly.

## Done summary
Registered the date palm preset (date-palm, Phoenix dactylifera) with its four reachable value-table rows: skeleton.habit.stems=1, skeleton.habit.lateral_orders=0, skeleton.habit.apical_dominance=1.0, radii.length_taper=0.0. Every other row keeps the family default. Registered in params::IN_WORK (abi id 7, unlisted, not built by name), mirroring european-beech's shape. No palm anatomy was added.

R1: cargo run --release -p telperion-core --example species_measure -- --print-family date-palm exits 0 and the printed family carries the four values (verified: apicalDominance: 1.0, lateralOrders: 0, stems: 1, radii.lengthTaper: 0.0). No other preset branch was touched (diff is additive only), and the full gate's existing pin/wire-roundtrip tests (catalogue_roundtrips_all_controls_and_identities, a_table_in_work_is_reserved_unlisted_and_not_built_by_name, every_pin_matches_its_catalogue_record) stayed green, which is what covers the other presets' byte-identity.

R2: cargo run --release -p telperion-core --example geometry_benchmark -- --support date-palm exits 0 and prints {"capabilities":["woody-axes"],"implemented":true,"profile_id":"date-palm"}.

R3: not run by me - the palm run's gap resume gate rerun is the host's to verify after landing, per the task instructions.

R4: env -u TYPESAFE_API_KEY cargo test --profile ci --workspace --no-fail-fast ran once at the end (gate_rc=0): 118 test suites, all ok, 0 failed. The pin tests for the other presets were not touched and stayed green.

Note: the workspace gate run left .flow/evidence/fn71/resolution/smooth_bark.json modified (a pre-existing fixture a resolution test rewrites with freshly measured numbers each run, for european-beech/silver-birch close-ups; unrelated to this diff, which touches no beech/birch/bark code). It is left uncommitted and unstaged in the worktree; the sandbox's git-safety guard blocks git checkout -- to discard it without explicit approval, so it was left in place rather than force-discarded.

stage: impl-review - skipped(config: REVIEW_MODE=none)
## Evidence
- Commits: 0364499996a52bbbef4b7c703830df52d3a12036
- Tests: cargo run --release -p telperion-core --example species_measure -- --print-family date-palm (exit 0), cargo run --release -p telperion-core --example geometry_benchmark -- --support date-palm (exit 0), env -u TYPESAFE_API_KEY cargo test --profile ci --workspace --no-fail-fast (gate_rc=0, 118 suites ok, 0 failed)
- PRs: