---
satisfies: [R1, R2, R3, R4, R5, R6, R7]
---
# fn-87-the-specimen-says-what-it-will-cost.1 Implement The specimen says what it will cost

## Description
TBD

## Acceptance
Every R-ID in the parent spec's ## Acceptance Criteria is satisfied; judge this task against the spec's criteria directly.

## Done summary
The species suite now sizes each specimen from the tree it just grew and admits
seeds against one memory budget the whole test binary shares, in place of the
`SEEDS_IN_FLIGHT = 4` constant whose comment had drifted three times unnoticed.
On this 32-core desk the suite runs in 26.46 s against the 49.12 s baseline,
peaking at 12,539 MB under a 15,304 MB ceiling it computes from `MemAvailable`;
as free memory fell between runs the budget narrowed on its own, to 11,222 MB
and 35.47 s, without a number changing anywhere.

The counting logic came out of the builders into functions the builder and the
prediction both call, so there is one implementation of each: `surface::extent`
is the ring arithmetic `surface::build` reserves by, `station::walk` and
`station::station_count` are a run's length and the leaves it carries, and
`short_shoots::count` is `clothe`'s own walk counted rather than drawn.
`foliage::place` now reserves the whole crown once, exactly, from that count, so
a finished crown's capacity is the count the prediction reads and the cull and
the limb clumping retain inside it. `footprint::predict` returns nodes, the
wood's four buffers and the crown's leaves, each a count times the size of the
type that holds it; the species harness asserts it equal to
`footprint::measure` of the live objects on every seed of all four species, so
the 15 percent R3 allows is never spent.

No stored byte moved. All 49 committed digests hold.

### Criteria

- **R1** `footprint::predict` (`crates/telperion-core/src/footprint.rs`) builds
  no mesh, no placement buffer and no matrix; a tree with no nodes predicts
  zero, which `a_tree_with_no_nodes_predicts_nothing` covers.
- **R2** One implementation each for stations, runs and vertices; equality is
  asserted per seed per species in `grow_and_check`. A family whose limb
  systems clump places fewer leaves than it reserved for, so its placed count
  is asserted at or under the prediction rather than equal to it, and the
  capacity equality still holds exactly.
- **R3** Predicted and measured are asserted equal, not merely within 15
  percent. Measurement reads the live objects' capacities.
- **R4** No literal byte count for a specimen, a leaf or a vertex exists in
  either. `each_term_is_the_size_of_the_type_that_holds_it` asserts that one
  more leaf moves the answer by `size_of::<Leaf>()` and one more vertex by its
  eight floats, so a stored leaf that changes shape moves the prediction with
  it.
- **R5** One budget, `Budget::of_host`, created once for the binary. The charge
  limit is two thirds of the ceiling, held to its arithmetic across a table of
  free-memory readings. `charges_never_outrun_the_limit_and_a_panicking_seed_releases_its_own`
  drives sixty concurrent admissions of mixed cost, fails every fifth one while
  it holds its charge, and asserts nothing is left charged and admission is
  never none.
- **R6** `SEEDS_IN_FLIGHT` is gone. Each heavy test ends by holding the
  binary's own VmHWM to the ceiling, waived only where a specimen larger than
  the whole charge limit was admitted alone and named.
  `a_saturated_budget_still_stands_under_the_ceiling` saturates an injected
  budget to its whole charge limit with memory it has really written to, in a
  process of its own, and asserts the peak stays under the ceiling.
- **R7** 26.46 s, 39.80 s and 35.47 s against a 49.12 s baseline whose 15
  percent bound is 56.49 s. The narrowest run is the slowest, which is the
  degradation the criterion asks for.

### What the host should weigh

Peak memory is about twice the baseline's 5,695 MB. That figure was a
consequence of the constant this spec removed, and the suite now spends what
the machine can spare and narrows when it cannot; the bound is measured rather
than assumed, and resident stands at 1.25 times what was charged against the
1.5 the headroom allows.

CI runs these tests under `cargo nextest`, which gives each test a process of
its own, so four heavy tests are four budgets reading the same `MemAvailable`.
The seat count bounds each process to the core count, which on the four-core
runner is the sixteen specimens `SEEDS_IN_FLIGHT = 4` already gave, so CI
neither gains nor loses. Whether the bound should be shared across processes is
a design question, and it goes up rather than being answered here.

stage: impl-review - skipped(config: REVIEW_MODE=none)
## Evidence
- Commits: 33429197db67efdee2059f2b724b335c0ab70380, 75153b12f0b31b5124ccf9cbd3330c7a095d9e6d
- Tests: cargo test --release --workspace (685 passed, 94 binaries, exit 0), cargo test --profile ci --workspace (green before the release gate), cargo clippy --profile ci --workspace --all-targets (clean), cargo fmt --all, target/ci/deps/species-* --nocapture with VmHWM polled from /proc/<pid>/status: 12 passed, 26.46 s / 12,539 MB, 39.80 s / 11,750 MB, 35.47 s / 11,222 MB, baseline: green (cargo test --profile ci -p telperion-core, pre-edit, 5m15s)
- PRs: