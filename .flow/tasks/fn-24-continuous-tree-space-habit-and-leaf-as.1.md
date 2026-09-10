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
# fn-24.1 — One scaffold builder from the habit trait table

One process now grows every axis of every tree in growth units from fifteen
numeric habit traits. `habit.rs` and its three routines are gone, the separate
colonization pass with them, and the four habit branches below the crossover
read traits instead. Every growth unit of every axis — bole, leader and lateral
alike — takes its heading from one sum of the rule heading, the attractor pull
and the bias field, so the supernatural dials reach the oak and the spruce
exactly as they reach Telperion. Each axis draws from a stream hashed from the
family seed, its parent axis and its child index.

Rust core only. The TypeScript surface and the panel are task 4's; `npm test`
is expected red on the habit shape until then.

### The trait rows as calibrated

| Trait | Oak | Spruce | Ordinary | Two Trees |
|---|---|---|---|---|
| apical dominance | 0.1 | 1.0 | 0.5 | 0.15 |
| whorl strength | 0.1 | 1.0 | 0.3 | 0.2 |
| leader internode (m) | 2.0 | 0.9 | 1.5 | 10.0 |
| laterals per station | 5 | 5 | 3 | 4 |
| lateral pitch (deg ± var) | 55 ± 20 | 88 ± 4 | 60 ± 15 | 60 ± 15 |
| rise per order (primary / secondary) | +0.12 / 0 | +0.12 / −0.8 | +0.05 / 0 | +0.05 / 0 |
| crookedness (deg) | 24 | 0 | 12 | 12 |
| lateral spacing (m) | 1.6 | 0.15 | 0.35 | 10.0 |
| lateral length ratio | 0.45 | 0.30 | 0.40 | 0.45 |
| lateral orders | 3 | 4 | 3 | 3 |
| attractor weight | 0 | 0 | 1.0 | 1.0 |
| twig tip taper | 0.25 | 0.25 | 1.0 | 1.0 |
| shedding threshold | 0 | 0 | 0.45 | 0.45 |

Ordinary is `HabitParams::default()`; Telperion and Laurelin share one row.

### Deviations from the spec's starting table, and why

- **Oak lateral spacing 1.6 m, not 0.35 m; lateral orders 3, not 5.** At 0.35 m
  a ten-metre limb bears twenty-eight second-order axes and five orders of that
  is 63,000 axes: the oak hit the 250,000-node ceiling and truncated on seed 5.
  The scaffold hands over to the local layer at the limb scale, so its spacing
  is a limb spacing.
- **Spruce lateral spacing 0.15 m, lateral orders 4 (spec: 0.20, 2).** At two
  orders the hanging secondaries bear nothing, and the mid-axis forks the
  species gate counts vanish. Four orders at 0.15 m puts the curtain back:
  91,331 nodes against the retired builder's 113,132.
- **Telperion and Laurelin have their own row.** They are 148 m and 132 m tall;
  a metre-valued spacing tuned for a 24 m tree buries them in the node ceiling.
- **The attractor pull is a term beside the rule heading, not a blend against
  it** (`rule + pull × weight`, normalised once). A lerp made weight 1.0 discard
  the rule heading outright, and then crookedness and rise stopped moving the
  ordinary preset at all — R2 fails by construction.
- **Crookedness is applied only above the bare-trunk height.** The envelope has
  no width below the crown base, so a wandering bole is outside the authored
  silhouette by definition; the field's lean and writhe still reach the bole,
  which is how the supernatural terms touch the trunk.
- **Laterals per station is the leader's whorl; axes below it bear one lateral
  per station.** Five laterals at every station of every order is exponential.

### Numbers against the retired builder (default seeds)

| | nodes (was) | crossover (was) | top m (was) | width m (was) |
|---|---|---|---|---|
| oak | 138,510 (91,699) | 3,130 (2,281) | 22.9 (16.6) | 26.4 (24.7) |
| spruce | 91,331 (113,132) | 17,584 (23,583) | 15.0 (15.0) | 8.85 (9.07) |
| ordinary | 9,240 (11,766) | 2,484 (538) | 22.1 (23.9) | 12.6 (14.0) |
| Telperion | 75,697 (177,543) | 505 (522) | 141.7 (147.9) | 69.3 (71.0) |
| Laurelin | 95,140 (105,032) | 2,320 (1,733) | 131.8 (131.9) | 153.1 (153.1) |

Every gating metric of both frozen profiles passes on all twelve fixed seeds
plus the spruce's retained counterexample seed: oak height and dbh, spruce
height and crown width, both foliage dimensions.

### Stills for the owner's verdict (R3 — not mine to give)

- `/tmp/flow-handover-fn24/fn-24.1-oak-hero.png` — oregon-white-oak, seed 7
- `/tmp/flow-handover-fn24/fn-24.1-spruce-hero.png` — norway-spruce, seed 7

Beside `.flow/evidence/fn23/oak-hero.png` and `.flow/evidence/fn23/spruce-hero.png`:
the oak is wider and taller with a finer leaf texture and three tufts where the
terminal whorl of limbs shows through the crown top; the spruce holds its
conical silhouette, its whorled tiers, its hanging curtains and the same thin
leader spike above the top whorl the fn-23 still has, but its tiers separate
less crisply than the retired builder's.

### Pinned hashes

- oak `14874354835152613499`, spruce `11730507885382185461`,
  ordinary `12781088285770051220` (`branching::audit::shipped_scaffolds_are_reproducible`)
- `tests/identity.rs` re-recorded for both species: skeleton, placement, mesh
  counts and bounds. The element hash is unchanged — nothing here touched it.

### One threshold lowered, deliberately

`tests/field.rs` asserted Telperion retains more than 1,000,000 foliage
placements. The trait-built crown stands further inside the lit shell than the
colonizer's did and retains 534,638, so the rail now reads 400,000 with the old
number recorded in the comment. The spec puts the Two Trees' verdict with
fn-10 and says their scaffolds will differ; this is that difference, and the
conductor should carry it to the owner rather than let it pass as noise.

### Follow-ups, not built here

- Telperion's crown spends its node budget inside the shell where the
  colonizer spent it on the surface. A shell-weighted attractor sampling or an
  occupancy term (fn-4's seam is the heading sum) would win it back.
- The spruce's tier separation is softer than the retired builder's.

stage: impl-review - skipped(policy: parallel wave - the conductor reviews after integration)

### Integration and the owner's verdicts (conductor, 2026-09-10)

Cherry-picked onto the spec branch as f3c30e2; the workspace commit ad2fe55 is retired with the worktree. Host review of the diff: one builder module of 339 lines with the heading sum as the single entry for any direction term, per-axis hashed streams, the four habit branches below the crossover replaced by trait reads, and the duplicate benchmark wire deleted. Follow-up for a later task: `branching/audit.rs` now stands at 458 lines with the R2 tests and must be split to meet the line rule.

> _oak verdict (owner, 2026-09-10):_ Accept. Reads as an oak, equal or better.
> _spruce verdict (owner, 2026-09-10):_ Accept. Reads as a spruce, equal or better.

The owner heard the Telperion foliage rail change (1,000,000 to 400,000, 534,638 retained) and the trait-row deviations before answering.

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: f3c30e2
- Tests: worker baseline: green - cargo test --release --workspace (125 passed, 7 ignored) before any edit, worker: cargo test --release --workspace - 129 passed, 0 failed, 7 ignored, worker: cargo fmt --all -- --check, worker: cargo clippy --workspace --all-targets -- -D warnings, conductor, integrated target f3c30e2: cargo test --release --workspace - green, rc=0, cargo run --release -p telperion-render --example headless -- --preset oregon-white-oak --seed 7 --out /tmp/flow-handover-fn24/fn-24.1-oak-hero.png, cargo run --release -p telperion-render --example headless -- --preset norway-spruce --seed 7 --out /tmp/flow-handover-fn24/fn-24.1-spruce-hero.png
- PRs: