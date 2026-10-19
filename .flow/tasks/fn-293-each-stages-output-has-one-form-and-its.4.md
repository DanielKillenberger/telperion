---
satisfies: [R1, R3]
---
# fn-293-each-stages-output-has-one-form-and-its.4 One sweep a mesh request, one station count, one Expansion preparation

## Description
Findings 7, 12 and 13 in the core pipeline: a mesh request sweeps the wood once and draws its `SurfaceMesh` from the curve; the leaf plan calls `station_count` instead of restating it; the two `Expansion` preparations share one helper and one `retains` rule.

**Size:** M
**Files:** `crates/telperion-core/src/pipeline/drawn.rs`, `pipeline/surface/curve.rs`, `pipeline/foliage/plan.rs`, `pipeline/foliage/station.rs`, `pipeline/executor.rs`, `pipeline/tests.rs`
**Touches:** [crates/telperion-core/src/pipeline/drawn.rs, crates/telperion-core/src/pipeline/surface/**, crates/telperion-core/src/pipeline/foliage/plan.rs, crates/telperion-core/src/pipeline/foliage/plan/**, crates/telperion-core/src/pipeline/foliage/station.rs, crates/telperion-core/src/pipeline/foliage/placement.rs, crates/telperion-core/src/pipeline/executor.rs, crates/telperion-core/src/pipeline/tests.rs, .flow/evidence/fn-293-each-stages-output-has-one-form-and-its/**]

### Approach
- Finding 7: `drawn.rs:22-33` sweeps rings, `drawn.rs:83-86` traces the curve again (`surface/curve.rs:116` `traced`). When a request asks for both, trace once and draw the rings (and the `SurfaceMesh`, seated leaves) from `Curve::rings` (`curve.rs:348`), which `curve/tests.rs` holds equal to the sweep's rings to the bit. A request without the curve keeps its single sweep. Update `one_sweep_and_one_element_serve_a_request` (`pipeline/tests.rs:57`) to count one sweep for `Request::mesh()`.
- Finding 12: `plan.rs:141-154` and `:210-223` restate `ceil(length/internode)·per` and the budget; call `station::station_count` (`station.rs:178`, make it visible to the plan module if needed). `placement.rs:133-134` restates `per`: use the same helper.
- Finding 13: `executor.rs:137-167` (`resident`) and `:195-216` (`Grown::expansion`) each build element, twig, shell check, reference and bearing. One private helper builds them; `retains` has one rule. Check first whether the two rules ever differ on a shipped preset (species `leaf_years` versus `!tree.leafing.is_empty()`); if they do, keep the digests identical by choosing the rule the resident build uses on the device, and report the difference.
- Measure finding 7's gain: `Request::mesh()` time on the spruce and oak, paired warm over 16 runs, before and after (CPU, `--profile ci`), in `MEASURE.md`.

### Investigation targets
**Required:**
- `crates/telperion-core/src/pipeline/drawn.rs:1-120`
- `crates/telperion-core/src/pipeline/surface/curve.rs:80-160,340-381`
- `crates/telperion-core/src/pipeline/foliage/plan.rs:130-230`
- `crates/telperion-core/src/pipeline/executor.rs:130-220`
**Optional:**
- `crates/telperion-core/src/pipeline/surface/curve/tests.rs`

### Key context
The CPU mesh is the stills' and Jev's reference: its bytes must not move. If `Curve::rings` differs from the sweep's rings for any preset at seeds 1, 7, 42, stop and report.
## Acceptance
- [ ] `Request::mesh()` sweeps the wood once (the counting test says so).
- [ ] The plan and placement call one station count; one `Expansion` preparation helper with one `retains` rule.
- [ ] Digests equal the baseline; the mesh time before and after is in `MEASURE.md`.
- [ ] Gate and `npm test` green.
## Done summary
A request for the wood and its curve (`Request::mesh()`) now traces the curve once and draws the wood's rings from its points (`surface::drawn_from`, `Scratch::read`, on the sweep's workers where it has them); the counting test sees one sweep where the base saw two (red commit `c18c07f74`, fix `1b25167cc`). `station_count`, `spacing` and a new `station_at` live in `foliage/plan/stations.rs`, which the plan's runs, `kept_between`, `leaves_between`, placement's prediction and `station::stations` call instead of restating them. `resident` and `Grown::expansion` share one `prepared` helper, and `retains` follows one rule, the species rule the resident build reads.

- Retains: on every shipped preset the species rule and `!tree.leafing.is_empty()` give the same answer (`raw/t4-retains-rules.txt`: oak, beech, telperion and laurelin true; the rest false). `Grown` now carries `retains` from its family. For `expand` with a hand-built tree, retention comes from the family rather than the tree; no test in the gate depends on the difference.
- Digests: core (24 lines), render pipeline (byte-identical, including the `mesh` line, the only digest that covers `Request::mesh()`) and resident all match the baseline (BASELINE.md "After task 4").
- Mesh time (MEASURE.md "Task 4"), 16 paired warm runs, CPU, `--profile ci`. Spruce: 4166 to 4088 ms median, -82 ms paired (after faster in 14 of 16). Oak: 153 to 139 ms, -15 ms (16 of 16).
- Order change: in a mesh request the curve's allocation now comes before the wood's memory-budget check, and `rings_ms` includes the trace.
- Non-test core lines: 22884 to 22997.
- Outside Touches: `docs/generation-limits-inventory.json`. Three limit sites moved with `station_count`, and the full gate's one failure (`generation_limit_guard`) pointed at them; the guard passes after the fix.
- Gate: `telperion-heavy 12 --threads 2 -- cargo test --profile ci --workspace --no-fail-fast` gave 1402 passed and 1 failed, the guard above, fixed in `2b7c6583e` and re-run alone green. `npm test`: 170 passed. Baseline: green (focused core tests, 85 passed). FRICTION.md has two entries (the core digest never asks for the curve; the inventory is keyed by file).

Tier: not reported (model: claude-opus-5-5)

stage: impl-review - ran (codex, three draws: correctness, contracts and integration, all SHIP with no findings)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: c18c07f7460b081982025ef19cfb8ba26e0da4e0, 1b25167cc1a4ddc0b72e110e265ccc37aeb117b7, 1233de8c88c6c10915f45b672378c3121570ba16, 276e539fc3f651de59b3a79b188216df076e4234, 95643da29b7ba612699f4922e7e45faa37925cfb, df39ca283999234396760f47978e997f22b3ea1e, 2b7c6583e9f70be4238f71a4ababfc0cf0a3ed86, fec80cf6f81d43ef21872ba046bae88c84c5f3a1
- Tests: telperion-heavy 12 --threads 2 -- cargo test --profile ci --workspace --no-fail-fast (1402 passed, 1 failed: generation_limit_guard, fixed in 2b7c6583e), cargo test --profile ci -p telperion-core --test generation_limit_guard (green), npm test (170 passed), artifact_digest, generation_digest, resident digest_tests::digest at seeds 1,7,42 (equal to baseline)
- PRs: