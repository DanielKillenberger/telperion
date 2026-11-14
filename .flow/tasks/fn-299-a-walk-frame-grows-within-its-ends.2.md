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
Rule B in `telperion-space/src/blend.rs` (host ruling 2026-10-20). Strictly between the ends, an age one end never reaches takes the other end's settings, every row. Reach is searched from the seed's PA 0 through apexes moving on, through the laterals an axis draws (`PaState::laterals`, zones with nodes) and through sleeping buds. The repro `a_walk_frame_grows_within_its_ends` runs un-ignored at a SLACK of the ends' max + 2 that names the two mechanisms left. `the_floor_counts_the_ages_a_cycle_chains` stays at 4, with the sliver named. New tests in `tests/blend_reach.rs` cover both walk directions, an unread lateral, a mixed age and oak to palm within its ends.
stage: impl-review - ran (codex, NEEDS_WORK then SHIP) (model: gpt-6.1-sol)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 499117b83c380996cca81391111c2b6424e234e4, 0a08e047482f8d6a71eeaa9f9d19b68e158f2b6d
- Tests: telperion-heavy 8 --threads 2 -- cargo test --profile ci -p telperion-space (all green at 499117b83); --test blend_reach --test walks and -p telperion-core --test units_bound (green at 0a08e0474)
- PRs: