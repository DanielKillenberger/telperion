---
satisfies: [R2]
---
# fn-308-a-walks-midpoint-draws-a-tree-halfway.1 The walk laws weigh a row by the frame's reach

## Description
Host ruling 2 (spec R2): `walk_law` and `walk_leaves` read an engine row only where the frame grows in its age, weighted by the frame's closed-form expected phytomers there. This lands first, on the base, where both laws already pass, so the change is dormant until .2's rule makes unreached ages carry the other end's rows.

**Size:** M
**Files:** `crates/telperion-core/tests/walk_law.rs`, `crates/telperion-core/tests/walk_leaves.rs`
**Touches:** [crates/telperion-core/tests/walk_law.rs, crates/telperion-core/tests/walk_leaves.rs]

### Approach
- **walk_law** (`steepest`, lines 104-135; `law`, lines 56-103):
  - Keep the unweighted series for every form.
  - For a per-age form (`cluster leaves per short shoot, age k`) whose step exceeds `BOUND`, re-read that one step weighted. The value is multiplied by w_k = `expected_counts(species, age).total(k)` over the frame's total, at both frames. The step is normalised by the same series maximum as now.
  - The step is refused only if the weighted step still exceeds `BOUND`. Canopy-wide forms are unchanged.
  - The weight is computed only at refused steps (cost: Decision Context).
- **walk_leaves** (the one-count assertion at lines 155-170):
  - Read only the short-shooting ages the frame grows in, with w_k above 0.
  - Where those ages carry unlike counts, the expectation for leaves per short shoot weighs each age's count by w_k, and the sigma likewise. The tolerances (`SIGMAS`, `ORGANS`) are unchanged.
  - Frames are already built through `blend::families` (lines 218, 299), so take w_k from the frame's own species at `AGE`.
- **A unit check inside `walk_law.rs`** that fails without the weighting: a synthetic step whose per-age row jumps by a whole count at an age the frame barely grows in is passed. The same jump at an age holding most of the frame's phytomers is refused. That is ruling 5a's switch.

### Investigation targets
**Required:**
- `crates/telperion-core/tests/walk_law.rs:41-135`: `walked`, `law`, `steepest`
- `crates/telperion-core/tests/walk_leaves.rs:100-230`: `draws`, the oracle, `jumps`
- `crates/telperion-space/src/closed_form.rs:35`: `expected_counts`; `crates/telperion-space/src/structure.rs:247` `CountTable::total`
- `crates/telperion-core/tests/walk_midpoint.rs`: `the_reach_of_each_age_near_the_ends` (the reach reading) and `the_reach_cost`

### Key context
- R1 measured that all 15 refusals under C5 sit at ages the frame grows in at 0.004 % to 4 % of the reaching end's growth (`R1.md`, C4 guards). A binary entered/not-entered gate would not clear them: the weight must be continuous.
- The closed form costs up to 0.5 s a frame at age 80, so it cannot run at all 1,001 steps per walk.

### Acceptance
- [ ] On the base, `walk_law` and `walk_leaves` pass, with run times within 10 % of today's (walk_law 0.66 s, walk_leaves about 31 s).
- [ ] With `candidates/c5-exits.patch` applied as an uncommitted check, `walk_law` passes and `walk_leaves` passes. Record both results in the task's evidence, then revert the patch.
- [ ] The unit check passes with the weighting and fails with it removed (shown once).
- [ ] The tolerances `BOUND`, `SIGMAS` and `ORGANS` are unchanged.

## Acceptance
- [ ] TBD

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
