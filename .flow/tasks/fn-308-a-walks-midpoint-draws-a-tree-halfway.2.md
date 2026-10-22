---
satisfies: [R2, R3, R5]
---
# fn-308-a-walks-midpoint-draws-a-tree-halfway.2 The rule: every row of an unreached age, the lifespan clamped at the age, an unreached end's exits

## Description
Land C5 (spec R2) through one aged path (R5), with host rulings 3 and 4 applied to the tests that read the old rows.

**Size:** M
**Files:** `crates/telperion-space/src/blend.rs`, `crates/telperion-space/src/lib.rs`, `crates/telperion-core/src/blend.rs`, `crates/telperion-space/tests/{blend_reach,walks,chain}.rs`, `crates/telperion-core/tests/{space_walks,catalogue_identity,walk_law,grown_total}.rs`, `crates/telperion-core/examples/{walk_size,walk_ends}.rs`
**Touches:** [crates/telperion-render/src/generation/film_walk_tests.rs, .flow/specs/fn-300-*.md, .flow/tasks/fn-308-a-walks-midpoint-draws-a-tree-halfway.2.md, crates/telperion-space/src/blend.rs, crates/telperion-space/src/lib.rs, crates/telperion-space/src/species/scale.rs, crates/telperion-space/tests/**, crates/telperion-core/src/blend.rs, crates/telperion-core/tests/space_walks.rs, crates/telperion-core/tests/catalogue_identity.rs, crates/telperion-core/tests/walk_law.rs, crates/telperion-core/tests/grown_total.rs, crates/telperion-core/examples/**]

### Approach
- **The rule.** Start from `.flow/evidence/fn-308-a-walks-midpoint-draws-a-tree-halfway/candidates/c5-exits.patch`. Rewrite it as product code, not a copy:
  - `telperion_space::blend(a, b, t, age)` takes the age as a required argument (R5); there is no `blend_at` beside it.
  - Rule B copies the whole `PaState`. Delete `TIMING`, `timed` and the `every_timing_row_is_a_setting` unit test, which become dead.
  - Name the two meanings of reach apart: graph entry (`reached`) and ending within the age (the exit rule).
  - Rewrite the module, `blend`, `reaching` and the exit-rule doc comments for the new rule.
- **The age.** `crates/telperion-core/src/blend.rs` `space()` takes the frame's age from `mixed`: `linear(a.age, b.age, t)`, the value `sized` reads as `f.age as u32`. Clamp and compare against that same whole age, so the rule and the grower agree. Update the `families`, `centroid` and `space` doc comments.
- **Callers.** Every caller of the species blend passes an age: `crates/telperion-space/tests/{blend_reach,walks,chain}.rs`, `crates/telperion-space/src/species/scale.rs:287` if it calls it, `crates/telperion-core/examples/{walk_size,walk_ends}.rs`, and `crates/telperion-core/tests/{space_walks,grown_total,walk_law}.rs`. `walk_law::walked` passes the walk's age.
- **Host ruling 3** (`space_walks.rs`):
  - Remove the walked-equals-raw-mix clause in `a_walk_to_the_palm_is_never_grown_to_its_ends_size` (line 236) and its doc sentence. Keep the within-ends clause; R1 measured 180,095.85, 7,783.30 and 480 on C5.
  - In `a_walk_between_branching_species_keeps_its_ends_size_at_40_and_80`, the bare-equals-mix clause (line 110) holds against the aged blend; keep it.
- **Host ruling 4:**
  - Re-pin `catalogue_identity::every_walk_is_the_walk_it_was` (line 94) with a comment citing fn-308.
  - Move `the_grown_count_binds_where_a_blend_sheds_more_than_its_ends` to a point where the grown count binds. Scan as fn-299 did, in steps of 0.05, using predictions only, and record the scan in its comment.
  - Rewrite `blend_reach::the_rest_of_an_unreached_age_still_mixes` to assert that every row of an unreached age, the zones included, equals the reaching end's state exactly. Widen `an_age_one_end_never_reaches_keeps_the_others_timing` the same way, or fold it in.
  - Add the exit rule's own case: the oak to palm age-2 continuation equals the oak's at 0 < t < 1, at age 40. At an age neither end ends within, the rows mix.

### Investigation targets
**Required:**
- `.flow/evidence/fn-308-a-walks-midpoint-draws-a-tree-halfway/candidates/c5-exits.patch`
- `crates/telperion-space/src/blend.rs`: the whole file, 244 lines
- `crates/telperion-core/src/blend.rs:28-160`
- `crates/telperion-space/tests/blend_reach.rs`
- `crates/telperion-core/tests/space_walks.rs:85-250`

**Optional:**
- `crates/telperion-core/tests/units_bound.rs:44-107`: fn-299's bound, green on C5 (R1)
- `crates/telperion-render/src/generation/{cpu_builds_tests,film_walk_tests}.rs`: the device frames, re-run in the gate

### Key context
- R1's C5 guard run (`raw/c5-guards.log`) lists exactly which tests move, and why.
- `the_floor_counts_the_ages_a_cycle_chains` (units_bound) stayed at 4 under C5. It used the unaged blend there; with the age it must be re-run.
- Memory: a whole-number engine row on a linear scale breaks walks (`.flow/memory/bug/runtime-errors/a-whole-number-engine-row-on-a-linear-2026-10-08.md`). Rule B copies such rows whole at an unreached age, and reached ages still mix them as now.

### Acceptance
- [ ] `blend` takes the age, and no unaged species blend remains in `crates/`. A test holds `families` equal to the blend at the walk's age, for all six film walks at t = 0.25, 0.5 and 0.75, in every row but the lateral probabilities. The size solve scales those by one factor of at most 1 (R5).
- [ ] The `blend_reach` rewrite and the exit-rule case pass, and each fails on the base (shown once).
- [ ] The re-pins and the moved example point each carry a comment citing fn-308 and the measured value.
- [ ] `units_bound`, `walk_law`, `walk_leaves`, `walk_sizes`, `space_walks`, `catalogue_identity`, `blend_reach`, `walks` and `chain` pass.

### Scope widened (host, 2026-10-22)
- **Ruling 1:** the lifespan clamp is at `max(age, 1)`, the smallest valid lifespan at or past the age, so a walk under one year is never refused. Its proof is open with the host: the beech to spruce tree at age 0.5 is not the base's (probe in `raw/t2-age-under-one-*.log`).
- **Ruling 2:** the aged blend exposed fn-300's CPU/device node divergence on two more spruce to birch rounds frames at 40: seed 1, frame 63 (387,693 / 387,695) and seed 42, frame 59 (393,600 / 393,601). They join the ignored fn-300 node test in `film_walk_tests.rs`; `the_rounds_frames_have_the_cpus_counts` stays green over the rest; fn-300's R1 records both.
## Acceptance
- [ ] TBD

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:

