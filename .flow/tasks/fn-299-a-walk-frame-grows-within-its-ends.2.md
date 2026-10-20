---
satisfies: [R2]
---
# fn-299-a-walk-frame-grows-within-its-ends.2 The rule: a walk frame's ages blend within its ends

## Description
R2. Build the blend rule the host rules on after task .1, in `blend.rs` or `species/scale.rs`, as a property of the blend. Un-ignore task .1's repro, add a unit test of the rule at the blend level (both walk directions, a pass-through age and an age behind a stop), and rewrite `the_floor_counts_the_ages_a_cycle_chains` if the rule covers pass-through ages.

**Size:** M
**Files:** `crates/telperion-space/src/blend.rs`, `crates/telperion-space/src/species/scale.rs`, `crates/telperion-core/tests/units_bound.rs`, `crates/telperion-space/tests/walks.rs`
**Touches:** [crates/telperion-space/src/blend.rs, crates/telperion-space/src/species/scale.rs, crates/telperion-space/src/species/scale/**, crates/telperion-core/tests/units_bound.rs, crates/telperion-space/tests/walks.rs, docs/tree-space.md]

### Approach
- The rule's shape comes from the host's ruling recorded in `R1.md`; do not choose one here.
- Keep `Scale::mix`'s exact ends (`scale.rs:65-71`) and `outcomes`' early return (`blend.rs:71-73`) so t = 0 and t = 1 stay byte-identical.
- Follow `a_walk_between_cluster_counts_moves_by_degree` in `species/scale.rs` tests for a blend-property test.
- Update the module docs at `blend.rs:1-7` and `species/scale.rs:1-12`, and `docs/tree-space.md` where it states how ages blend.

### Investigation targets
**Required**:
- `.flow/evidence/fn-299-a-walk-frame-grows-within-its-ends/R1.md` - the trace and the ruling
- `crates/telperion-space/src/blend.rs`
- `crates/telperion-space/src/species/scale.rs:57-95`

**Optional**:
- `crates/telperion-space/tests/walks.rs:43-62` - walk-by-degree test
- `.flow/memory` entry `bug/runtime-errors/a-whole-number-engine-row-on-a-linear-2026-10-08` - walk tests across differing values, not only neutral ends

## Acceptance
- [ ] The repro test runs un-ignored and passes at every t and seed, at the tolerance the host set.
- [ ] A blend-level test covers both walk directions, an age passed through on the way to a later age, and an age behind a stop.
- [ ] `the_floor_counts_the_ages_a_cycle_chains` asserts what the rule now gives, or the reason it is unchanged is stated in the test.
- [ ] Focused tests green: `telperion-heavy 8 --threads 2 -- cargo test --profile ci -p telperion-space` and `-p telperion-core --test units_bound`.

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
