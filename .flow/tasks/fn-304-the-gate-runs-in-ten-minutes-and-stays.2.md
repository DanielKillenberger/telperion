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
- ground_reach (`:162-211`): first settle whether 7.8 GB is accumulation. `VmHWM` is a lifetime high-water mark, so it cannot answer this alone: run representative rows (the largest presets at their own age and at 20) each in a fresh process (a temporary row filter, not committed) for each tree's own peak, and record `VmRSS` after each drop in the sequential sweep. Isolated peaks attribute memory to trees; the sweep's peak schedules the test. Retained memory across rows means hold one tree at a time (and find what keeps it); one tree's own peak is its figure. Gate subset: fn-301 R1's touching rows (palm at 6-20, birch, spruce at 40, ordinary and oak at 40 seed 42; spec fn-301 lines 33-39 and 91) plus every preset at its own age, seed 1; full sweep ignored. Red proof: a temporary removal of the surface clearance or the palm's establishment; update the module doc (`:1-27`).
- walk_sizes (`:150-180`): the teaser frame (`:152-156`) stays a gate test; the exceptions loop becomes an ignored test whose reason names fn-273 and fn-304. Keep `grown`/`miss` shared; check no constant goes unused. Red proof: a temporary budget refusal the teaser hits.

### Investigation targets
**Required:**
- `crates/telperion-core/tests/walk_leaves.rs:1-30`, `:200-260`
- `crates/telperion-core/tests/ground_reach.rs`
- `crates/telperion-core/tests/walk_sizes.rs:1-60`, `:119-180`
- `.flow/specs/fn-301-a-tree-rests-on-its-ground.md:30-100`

### Key context
The full sweeps take 280, 160 and 80 s alone; run each proof inside `telperion-heavy 8`, one at a time. fn-299 is open on the same walk tests: keep edits small.
## Acceptance
- [ ] walk_leaves: seed 1 in the gate, three seeds ignored, both red on the rounding patch.
- [ ] ground_reach: accumulation answered with VmHWM figures; one tree at a time; subset as ruled; full sweep ignored; both red on the clearance patch.
- [ ] walk_sizes: teaser in the gate, exceptions ignored naming fn-273; red proof recorded.
- [ ] Gate run once at the end; codex impl review.
## Done summary
Rulings 1, 2 and 5. walk_leaves walks seed 1 in the gate (20, 81, 29 s); the three walks at three seeds are one ignored test. ground_reach reads fn-301 R1's 45 touching rows and each preset at its own age, seed 1 (50 rows, 54-58 s, 4.68 GB); the 168-row sweep is ignored. Its 7.8 GB is Telperion at 82, seed 7, one tree (7.68 GB alone); the test already held one tree at a time. walk_sizes keeps the teaser frame (21 s); fn-273's eight exceptions are ignored. Each subset red where its sweep is (cluster-round, leaflets-round, no-rest, budget-eighth-cycle); fronds-round caught by neither, recorded. Gate: 1,466 passed, 2 failed (fn-302), 119 ignored, 1,995 s.
stage: impl-review - ran (codex, one correctness reviewer, SHIP; two P3 evidence fixes) (model: gpt-6.1-sol)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: ffce4e52471d2d2c71e1d9485425e92f75aed55c, 3fd698485af9c79bf78181cef3a23730d334a088, 6d7a6d7f54b36e045c155267ca3749775be0fe11
- Tests: telperion-heavy 8 --threads 2 -- cargo test --profile ci --workspace --no-fail-fast (1466 passed, 2 failed fn-302, 119 ignored), proofs.sh t2-* (nextest, telperion-heavy 8 --threads 4)
- PRs: