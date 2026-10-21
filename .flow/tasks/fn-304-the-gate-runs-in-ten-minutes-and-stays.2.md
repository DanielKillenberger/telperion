---
satisfies: [R6]
---
# fn-304-the-gate-runs-in-ten-minutes-and-stays.2 Core sweeps: walk_leaves at seed 1, ground_reach subset, walk_sizes teaser

## Description
Rulings 1, 2 and 5: the three core integration binaries added since fn-271 (fn-277, fn-301, fn-251), 439 + 159 + 80 test-seconds.

**Size:** M
**Files:** `crates/telperion-core/tests/walk_leaves.rs`, `crates/telperion-core/tests/ground_reach.rs`, `crates/telperion-core/tests/walk_sizes.rs`, RESULT.md
**Touches:** [crates/telperion-core/tests/walk_leaves.rs, crates/telperion-core/tests/ground_reach.rs, crates/telperion-core/tests/walk_sizes.rs, .flow/evidence/fn-304-the-gate-runs-in-ten-minutes-and-stays/**]

### Approach
- walk_leaves (`:21`, `:207-251`): `jumps` takes its seeds; the three gate tests run seed 1, an ignored test runs all three walks at seeds 1, 7 and 42. Red proof: a temporary rounding of a count across a whole number (the fn-277 defect, CAUSE.md) that the three-seed sweep catches and seed 1 catches.
- ground_reach (`:162-211`): first measure whether 7.8 GB is accumulation: print `VmHWM` per row (`suite/species/budget.rs:235-238` reads it) on the full sweep, inside the heavy wrapper. Accumulation means hold one tree at a time (drop before the next build); one tree's own peak is reported as its figure. Gate subset: fn-301 R1's touching rows (palm at 6-20, birch, spruce at 40, ordinary and oak at 40 seed 42; spec fn-301 lines 33-39 and 91) plus every preset at its own age, seed 1; full sweep ignored. Red proof: a temporary removal of the surface clearance or the palm's establishment; update the module doc (`:1-27`).
- walk_sizes (`:150-180`): the teaser frame (`:152-156`) stays a gate test; the exceptions loop becomes an ignored test whose reason names fn-273 and fn-304. Keep `grown`/`miss` shared; check no constant goes unused. Red proof: a temporary budget refusal the teaser hits.

### Investigation targets
**Required:**
- `crates/telperion-core/tests/walk_leaves.rs:1-30`, `:200-260`
- `crates/telperion-core/tests/ground_reach.rs`
- `crates/telperion-core/tests/walk_sizes.rs:1-60`, `:119-180`
- `.flow/specs/fn-301-a-tree-rests-on-its-ground.md:30-100`

### Key context
The full sweeps take 280, 160 and 80 s alone; run each proof inside `telperion-heavy 8`, one at a time. fn-299 is open on the same walk tests: keep edits small.

### Acceptance
- [ ] walk_leaves: seed 1 in the gate, three seeds ignored, both red on the rounding patch.
- [ ] ground_reach: accumulation answered with VmHWM figures; one tree at a time; subset as ruled; full sweep ignored; both red on the clearance patch.
- [ ] walk_sizes: teaser in the gate, exceptions ignored naming fn-273; red proof recorded.
- [ ] Gate run once at the end; codex impl review.

## Acceptance
- [ ] TBD

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
