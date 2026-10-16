---
satisfies: [R1, R2, R3]
---
# fn-61-trolls-model-limbs-pitch-by-height-and.1 Implement Troll's model limbs: pitch by height and a ragged reach

## Description
Declare `pitchByHeight` and `raggedReach` in the parameter catalogue; per the owner's 2026-09-27 extension, keep a shortened primary's descendants within its share of the shell.

## Acceptance
- [x] R1 rails, neutral digest identical on 8 tables x seeds 1 and 7.
- [x] R2 one test per property; containment red-first checked.
- [x] R3 bare beech rendered with the rows by `--family` overlay; implementer read recorded in the spec.

## Done summary
Both rows neutral at zero; refused by name on build, wire and walk (`suite/troll_limbs.rs`, `harness/rows.test.ts`). Limb bounds in `pipeline/branching/limbs.rs`. Snapshot schema 5. At -20/0.35 the edge breaks in places; at -45/0.6 clearly into limb systems. Codex review NEEDS_WORK (schema) then SHIP. Session model.

## Evidence
- Commits: d72cfdd1f3a27f8053c854929b338dfa618a14ac, bb100b5a2c751aa7edfa39da50b4b8ec2bafa5f4, 708464dc91f860ee40b521225352d5f3792265ed, 8e4e1cf8ca93dfbf808fa1063d37c6237f44919b, 15eb36c0842cdd7224ed055a763a590d7d023e6e, b860dad85026c37bd0eab2eb3df0f3544972e1a9, a1530c4d4cc83bba32711da7e9b9ab92294bc702, 257a6fc259d86ad3faac3aba246afc19ee0c94cd, 624394817717bc541a5f94f6d3d24b04f5028458, ffd03108bcb1b19942896c2a53de8076ac618f62, b49a494d10506c0e65f9dd4c21ed17919a6a0932, 857085fa41d83574f1776687acc9377d6548b592, b5ea090cadc5681bfc5f797824fa12c0c4df4ad6
- Tests: cargo test --profile ci --workspace --no-fail-fast (1062 passed), npm test (112), cargo test --profile ci -p telperion-core --lib limb_tests, cargo run --profile ci -p telperion-render --example generation_digest
- PRs: https://github.com/DanielKillenberger/telperion/pull/130
