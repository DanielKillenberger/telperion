---
satisfies: [R4]
---
# fn-283-a-frond-draws-its-rachis.2 The device draws its fronds once, in the leaf-base stage

## Description
Behaviour-preserving move: the frond clock and the segment draw leave `stations.wgsl` for the boots stage, which writes one frond table a dispatch per apex; the stations' frond segments read the table instead of recomputing. Split from .3 so the move is pinned by unchanged device output before the axis records depend on it.

**Size:** M
**Files:** `crates/telperion-render/src/generation/resident/boots.wgsl`, `boots.rs`, `stations.wgsl`, `stations.rs`, `build.rs`, their tests
**Touches:** [crates/telperion-render/src/generation/resident/**]

### Approach
- The frond functions `unfolded_frond`, `born_frond`, `established_frond`, `pinnate_frond`, `frond_leaves`, `segments`, `toward` (`stations.wgsl:221-330`) and the per-frond spiral of `rosette()` (`stations.wgsl:798-875`) move to `boots.wgsl` (or a shared WGSL include both stages compose, as `boots.rs::source` composes `KEYS` and `ORGANS`).
- A frond table, one record per frond slot per clothed apex: what `rosette()` writes per segment today (insertion, radial, heading, axis, share, split, leaflets, spines, key, withered), sized from the crown's slots (`organ_rows`/`rosette_rows` in `stations.rs:391-500`).
- `boots.wgsl`'s `apices` already computes per-apex frond counts (`boots.wgsl:164-188`); a family with fronds but no leaf bases must still run the frond pass (today the boots wave is skipped when no base hangs: check `build.rs:395-402`).
- `stations.wgsl`'s `rosette()` copies segment words from the table.
- Inputs that reach only the stations stage today move earlier: the requested `Year` (`build::placed` passes it to `station_rows`; thread it through `readied -> folded -> Hanger::hang` for the mature, growing, field and fractional-year builds) and the blade (`crown::blade` of the element; build the element from the family's element rows before the folds, `executor::element`, rather than from the `Expansion` that `folded` creates after the boots). Never rebuild the year from the family's terminal age.

### Investigation targets
**Required:**
- `crates/telperion-render/src/generation/resident/build.rs:380-420` - stage order hang, folds, sweep, stations
- `crates/telperion-render/src/generation/resident/boots.rs` and `boots.wgsl:1-200`
- `crates/telperion-render/src/generation/resident/stations.wgsl:160-340, 790-880`
- `crates/telperion-render/src/generation/resident/agree_tests.rs:251` - `every_rosette_is_placed_as_the_cpu_places_it`
**Optional:**
- `crates/telperion-render/src/generation/resident/pack.wgsl` - reads the segment words
- `.flow/specs/fn-275-a-young-palms-fronds-grow-to-their-size.md` - friction on this move

### Key context
- Equivalence harness: the device's leaf instances for the date palm at 40 and at the young age, and for every shipped preset, byte-identical before and after the move (record the base's bytes or digests before editing; `digest_tests.rs` may already hold this).
- Run GPU tests natively (`crate::generation::test_gpu()`); never WebGPU in a browser.

## Acceptance
- [ ] The frond clock and segment draw are computed once per apex in the boots stage; `stations.wgsl` holds no copy.
- [ ] Device leaf instances of every shipped preset (the palm at 40 and young included) are byte-identical to the base before the move.
- [ ] `every_rosette_is_placed_as_the_cpu_places_it` passes unchanged.
- [ ] A family with fronds and no leaf bases still builds on the device.
- [ ] The equivalence covers a growing build (`settle`) and a fractional-year build of the palm, and the device field of the palm, all unchanged against the base.

## Done summary
The device draws each crown's fronds once, in the leaf-base stage. The frond clock and the segment draw left `stations.wgsl` for `fronds.wgsl`, composed into the boots stage. Its `crowns` pass lists the crowned apices and `crown_fronds` writes one frond table record per frond, one invocation per crown. The stations' `measure` and `rosette` copy from the table and hold no copy of the draw. `counts.wgsl` holds the organ-count helpers both stages read. The boots stage now runs for a rosette with no leaf bases. The requested year and the element's blade reach it through `readied`/`folded` (mature, growing, field and fractional-year builds). The frond rows read `Family::applied()`, as the stations did.

Equivalence (measured): `palm_digest` and `digest`, base fbee5b3ba against a06639cd5, gave 34 of 34 lines identical. That covers every shipped preset at seeds 1, 7 and 42; the palm at 40, 12 and 4; the palm within its year at 40 and 4 (0.3, 0.8); settled at 5.4 and 6; and its device field (raw/digest-base.lines, raw/digest-after.lines). The applied-canopy fix after it changes only builds with a supernatural size override, which no shipped preset enables. `every_rosette_is_placed_as_the_cpu_places_it` passes unchanged. The new test `every_crown_the_leaf_base_stage_draws_is_placed_as_the_cpu_places_it` covers a palm with no leaf bases and a palm whose override lifts a natural size of 0.

Gate: `telperion-heavy 8 --threads 2 -- cargo test --profile ci --workspace --no-fail-fast`, gate_rc=0, 1,409 passed, 0 failed, 112 ignored (raw/gate-2.log). baseline: green via handoff (9839dc3d9, task .1's gate).

Decision: the element is built before the boots stage when a rosette stands, as the spec directs, so an invalid element now answers ahead of a bases/folds ResourceLimit when both fail.
Follow-ups: boots.wgsl (~590 lines) was over the ~400-line rule before this change; `Hanger::hang` grew (review FYI, Long Method). `executor::bases` reads the raw canopy, not `applied()` (pre-existing, not touched).

Tier: not given by the conductor (model: claude-opus-5-5)

stage: impl-review - ran (codex: round 1 NEEDS_WORK, 1 finding from all three lenses: applied canopy, fixed; round 2 SHIP)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: a06639cd576a0c782dfd9c7151e33f32e739aa56, 2c75515caef5cdb06c74bfdf1a9f98a923b6d9a9
- Tests: telperion-heavy 8 --threads 2 -- cargo test --profile ci --workspace --no-fail-fast, cargo test --profile ci -p telperion-render resident::digest_tests -- --ignored
- PRs: