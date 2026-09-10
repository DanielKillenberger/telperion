---
satisfies: [R1, R2]
---
# fn-24-continuous-tree-space-habit-and-leaf-as.1 One scaffold builder from the habit trait table

## Description
Replace the three habit builders and the separate colonization pass with one scaffold builder driven by the numeric habit traits (R1), apply the bias field at every growth unit, seed every axis from its own stream, and prove R2 with the cross-trait hash test. This is the early proof point: the owner must read the oak row as an oak and the spruce row as a spruce on the 8-tree harness before any leaf task starts. Rust core only; the TypeScript surface is restored in task 4 and is not this task's gate.

**Size:** M
**Files:** `crates/telperion-core/src/branching/scaffold.rs` (new, under 400 lines), `crates/telperion-core/src/branching/habit.rs` (deleted), `crates/telperion-core/src/branching/colonization.rs` (attractor pull reduced to a direction helper or deleted), `crates/telperion-core/src/branching.rs`, `crates/telperion-core/src/branching/local.rs`, `crates/telperion-core/src/branching/audit.rs`, `crates/telperion-core/src/params.rs`, `crates/telperion-core/src/presets.rs`, `crates/telperion-core/examples/geometry_benchmark/params.rs` (deleted, the example uses the core's wire), `crates/telperion-core/tests/growth.rs`, `crates/telperion-render/tests/conformance.rs`
**Touches:** [crates/telperion-core/src/branching/**, crates/telperion-core/src/branching.rs, crates/telperion-core/src/params.rs, crates/telperion-core/src/presets.rs, crates/telperion-core/examples/geometry_benchmark/**, crates/telperion-core/tests/**, crates/telperion-render/tests/conformance.rs]

### Approach
- Define the habit trait struct with the twelve fields of the spec's first trait table plus the attractor count the skeleton already carries; validate ranges with errors naming the trait, in the style of `crates/telperion-core/src/branching/habit.rs:50-77`.
- Grow axes in growth units. One heading-composition function sums rule heading, attractor pull scaled by attractor weight, and the bias field via `GrowthBias::apply` (`crates/telperion-core/src/bias.rs:91-151`), normalised once; every unit of every axis, trunk and leader included, goes through it. Today's trunk at `habit.rs:125-143` walks straight up with no bias call; that is the spruce's missing supernatural reach.
- Derive each axis's random stream from the family seed, the parent axis id and the child index (hash, not sequential draws), so a step on one trait perturbs only its own subtree. The sequential `Rng::new(params.seed ^ ...)` at `habit.rs:506` is the pattern being retired.
- Attractor sampling from `inner.sample(params.attractors, ...)` (`branching.rs:262`) runs only when attractor weight is positive; weight positive with zero attractors is an invalid input naming both fields.
- Local rules read the traits: crookedness for every tree (`local.rs:254`), hanging secondaries from rise per order rather than a habit match (`local.rs:193`), twig tip taper from the trait (`local.rs:440`), and shedding runs when the threshold is positive (`branching.rs:290-292`, `audit.rs:29`).
- Wire: replace the tagged `Habit` shadow enum and its `Wire` impl (`params.rs:198-266`) with flat rows in the `fields!` table (`params.rs:9-11`); the closed schema then rejects a kind tag naming the field. Delete the duplicate wire under `examples/geometry_benchmark/params.rs:198-239` and point the example at the core's json feature.
- Presets become rows: replace the `BranchHabit::Spreading`/`Tiered` construction at `presets.rs:70,102` with trait values from the spec table; calibrate against the frozen profiles on the 8-tree harness until the owner reads oak and spruce; record the final rows in the task done summary.
- Audit: re-record the oak hash and add spruce and ordinary hashes to `audit.rs:106-132`; add the cross-preset test that steps each habit trait and each bias term by one step on oak, spruce and ordinary and asserts the skeleton hash changes (R2). Rewrite the habit-parametrised tests at `tests/growth.rs:368-530` against the trait struct. Update the fixture mutation at `crates/telperion-render/tests/conformance.rs:26-35` to flat fields.
- Keep the new builder under 400 lines; split heading composition or the lateral placement into a sibling module if it grows.

### Investigation targets
**Required** (read before coding):
- `crates/telperion-core/src/branching/habit.rs:107-330,314-470` — the spreading and tiered builders being replaced; the magnitudes to reproduce
- `crates/telperion-core/src/branching.rs:250-300` — generate(): the colonize-or-habit fork, shedding gate, radius solve order
- `crates/telperion-core/src/branching/local.rs:170-262,430-450` — the four habit branches below the crossover
- `crates/telperion-core/src/params.rs:9-11,181-266` — the fields! table and the tagged habit wire
- `crates/telperion-core/src/branching/audit.rs:1-132` — pinned oak hashes and the diagnostic tests

**Optional** (reference as needed):
- `crates/telperion-core/src/branching/colonization.rs` — the attractor association loop and its bias call at line 188
- `crates/telperion-core/tests/growth.rs:368-530` — the habit test surface to rewrite

### Key context
- Never lerp or perturb angles as raw scalars across the wrap; keep headings as vectors and angles as shortest-path deltas.
- No special case at trait extremes: apical dominance 0 and 1, whorl strength 0 and 1, attractor weight 0 and 1 run the same code.
- Budget rules bind: 8-tree harness only, at most four images viewed per capture, 5 pilot ticks or 10 commits, then NEEDS_HUMAN.

## Acceptance
- [ ] One builder module under 400 lines; `habit.rs` and the duplicate benchmark wire are deleted; no `BranchHabit`, `Spreading`, `Tiered` or `Colonizing` identifier remains in the workspace
- [ ] Habit traits are numeric fields validated by range with errors naming the trait; a JSON habit with a `kind` tag is rejected naming the field
- [ ] Every growth unit of every axis, trunk and leader included, passes through one heading sum that includes the bias field
- [ ] Each axis draws from a stream derived from seed, parent axis and child index
- [ ] Cross-preset test: stepping each habit trait and each bias term on oak, spruce and ordinary changes the skeleton hash (R2)
- [ ] Oak, spruce and ordinary scaffold hashes pinned; same seed and parameters reproduce the same hash across runs
- [ ] Extremes test: apical dominance, whorl strength, rise per order and attractor weight at their bounds generate without error or non-finite position
- [ ] Owner reads the oak row as an oak and the spruce row as a spruce on the 8-tree harness; the final trait rows are recorded in the done summary
- [ ] `cargo test --release --workspace`, `cargo fmt --all -- --check` and `cargo clippy --workspace --all-targets -- -D warnings` pass

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
