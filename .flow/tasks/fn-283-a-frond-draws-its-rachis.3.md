---
satisfies: [R1, R5]
---
# fn-283-a-frond-draws-its-rachis.3 The device hangs the frond axes as the CPU hangs them

## Description
The axis records on the device: written from the frond table in the boots stage, in room the dress leaves past the bases', in the drawn-record layout the folds, sweep and field read, so the device draws, bounds, shadows and fields the axes through its existing path.

**Size:** M
**Files:** `crates/telperion-render/src/generation/resident/boots.wgsl`, `boots.rs`, `build.rs`, `fold.rs` (room), `crates/telperion-render/src/generation/cycle/dress.wgsl` if the room is reserved there, tests
**Touches:** [crates/telperion-render/src/generation/**]

### Approach
- Room: extend `boots::most` (`boots.rs:68-75`) with the most axis records a stem can hang: slots times (the most leaflets a frond draws + 1), from the rows' whole counts; refuse past the device's capacity by name.
- Records: mirror the CPU nodes of .1 (`hang` in `boots.wgsl` writes a base's two records; an axis writes one per node), with the same parent rule, radius and stations, reading the table .2 wrote.
- Agreement: extend `every_leaf_base_is_hung_as_the_cpu_hangs_it` (`boots_tests.rs:317`) or add its sibling for the axes, by lineage tolerance; the curve/mesh agreement tests already cover what the sweep makes of them.

### Investigation targets
**Required:**
- `crates/telperion-render/src/generation/resident/boots_tests.rs`
- `crates/telperion-render/src/generation/resident/fold.rs` - `Dressed`, `record`, the room past the dress
- `docs/pipeline.md` (the one cycle and its executors) - the drawn-record buffer contract
**Optional:**
- `crates/telperion-render/src/generation/resident/field_tests.rs` - the device field against the CPU's

### Key context
- CPU = device agree by lineage, not to the bit (`telperion_space::agree`).
- The field reads the drawn records; `the_device_field_answers_as_the_cpu_field` must hold for the palm.

## Acceptance
- [ ] The device hangs every frond axis the CPU hangs: same count, nodes within lineage tolerance (palm at 40 and young).
- [ ] The device's drawn curve, mesh and field agree with the CPU's for every shipped preset (existing agreement tests green, palm included).
- [ ] A capacity overflow is a named error.
- [ ] The date palm's `/canopy/rachisWidth = 0.25` (FAO 156 ch. I citation in the values comment) lands here, with the generated presets, HASHES.md's palm line and every core, render and jev test green at the palm's real width (moved from .1, conductor decision 2026-10-20).

## Done summary
The device hangs each frond's axis from the frond table, as the CPU hangs it. Its records belong to the clothing (BOOT), so no leaf is borne on them, and each is borne on its stem node as a base is. Overflowing the room is ResourceLimit("rachis nodes"). The date palm is at rachisWidth 0.25 (FAO 156 ch. I), and its pins are re-pinned.

Measured:
- rachis_tests: the axes match the CPU's at ages 4, 12 and 40 (count, nodes, bearers; nodes at most 1.15e-3 m apart, radii 1.9e-8 m), and the overflow is refused by name.
- agree_tests, field_tests, boots_tests and digest_tests: green (raw/focused-3g.log).
- palm_digest at width 0: identical to the base.
- CPU hashes: only the palm moves (HASHES.md).

Decision (conductor): the frond's wood is held to an angular lineage bound, 6e-4 rad times reach (shared cause: the ill-conditioned spiral reference). A frond-axis run's point in the sweep test is held to 1e-3 m + 6e-4 rad x its reach; every other run keeps 1e-3 m. A frond crown's field wood is held to 0.995. The cause was measured (raw/frond-azimuth.log, raw/axis-angle.log, raw/frond-clock.log): the spiral's phase is 1.2e-4 to 3.0e-4 rad apart between the executors. rosette::frame squares vertical off a crown axis that stands 4e-3 to 7e-3 rad off vertical, which magnifies the crown axes' 1.2e-6 rad lineage gap. The leaflets and bases share it.

Follow-ups:
- rosette::frame takes the spiral's reference from vertical squared off a near-vertical crown axis; a better-conditioned reference would tighten leaflets, bases and axes alike (core frame change, own spec).
- The palm's wood grows from 49,960 to 316,760 triangles at 40.

Gate: `telperion-heavy 8 --threads 2 -- cargo test --profile ci --workspace --no-fail-fast` (raw/gate-3.log, admitted 14:05): 1,408 passed, 3 failed, 112 ignored. The 3 failures were the catalogue_identity digests, which move with the palm's 0.25. They are re-pinned to .1's 0.25 values, and the rerun `cargo test --profile ci -p telperion-core --test catalogue_identity` passes 4 of 4 (raw/catalogue-3.log).

baseline: green via handoff (2c75515ca, task .2's gate)
stage: impl-review - ran (codex, three lenses: correctness, contracts, integration; all SHIP, no findings)
Tier: not given by the conductor (model: claude-opus-5-5)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: e74aee9880634b3de86e33741233ffc4eda8041b, 7dc6a4119a46347f94612208776233c7b1c17797, 5b3864d120efea333550c0f20dfee2ccc6d4d16a, 21da162e9fb9bb2d20cbe8afb42063d2c5cb4dda, 2cf1ea852048e3a97a6140e4cf90ad3152768d74
- Tests: telperion-heavy 8 --threads 2 -- cargo test --profile ci --workspace --no-fail-fast (raw/gate-3.log: 1408 passed, 3 failed catalogue_identity digests, re-pinned), cargo test --profile ci -p telperion-core --test catalogue_identity (raw/catalogue-3.log: 4 passed), cargo test --profile ci -p telperion-render --lib -- agree_tests field_tests rachis_tests boots_tests digest_tests (raw/focused-3g.log: 15 passed)
- PRs: