# fn-362-a-runs-rings-have-the-sides-its-widest.1 A run's rings have the sides its widest ring can show

## Description
TBD

## Acceptance
- [ ] TBD

## Done summary
Every ring of a CPU-mesh run now takes the sides its widest drawn ring needs to stay within FINEST of the circle, capped at max(radialSegments, 4·lobes); the GPU's wood, the tree, the curve and unseated leaves are unchanged. The beech at 80 draws 38.2 M wood vertices against 101.8 M, and the memory items it held are within budget except the beech's form test, which builds no wood. Evidence: READING.md, MEASURED.md, PINS.md, SHEET.jpg and FRICTION.md under .flow/evidence/fn-362-a-runs-rings-have-the-sides-its-widest/.

- R1 met: the GPU path is untouched (READING.md).
- R2 met: sides_tests.rs, red on the base, green on the head.
- R3 met: the core surface, suite and foliage tests pass; the render crate passes 215 of 217, and its two failures are the base's with the base's figures. Two tests changed with reasons (PINS.md).
- R4 met: six trees base against head (MEASURED.md). The class under 1 mm is 5.13 M against the paper's 4 M, the host's arithmetic corrected (host ruling, 13 Nov 2026).
- Memory, alone: smooth_bark 3,887 MiB, ground_reach 3,618, the Wasm main entry 2,848 and the Wasm flowers test 2,985, all within budget. The beech's form test is 8,711 against 8,662 on the base and its 4,096 budget; that crate has no diff and does not depend on core.
- R5: SHEET.jpg, ready for the host's verdict.
- R6: the tree, curve and leaves digests are equal on six trees, except the spruce's seated needles. The species digests are read only by tests ignored since fn-198, so they were not repinned.
- R7 met: docs/pipeline.md.

Follow-ups for the host: the dead species digest pins; the beech's form test over its budget in telperion-space.

baseline: green (pipeline::surface, 45 passed)
stage: impl-review - skipped(policy: host-deferred - conductor owns the gate)
Tier: you execute; you do not reason about how the system should work. (model: claude-opus-5-5)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 244100c232300deb7e0306ab89b3a4c0db67a477, 5b9c9fbd51802113085fc302821a5d2e3db64d64, 204a183bd29e85e08fc664a45ecb43b4d9a9799e, 0b4d201c745ad53af16b4c0e410d2bc2a0e12175, 7d6b9eb2a7b0e38018cbdb0ec0e8842b90812acc
- Tests: cargo test --profile ci -p telperion-core --lib -- pipeline::surface suite::surface suite::leaf_base_lattice suite::frond_contact suite::clusters suite::mesh suite::foliage suite::retention suite::organs, cargo test --profile ci -p telperion-core --lib -- suite::drop suite::sweep suite::twig_generations suite::strands suite::sag suite::codominance suite::leaf_bases supernatural capability pipeline::tests pipeline::foliage suite::species::, cargo nextest run --cargo-profile ci -p telperion-render --profile gate (215 passed, 2 failed as on the base), cargo test --profile ci -p telperion-core --test generation_limit_guard, cargo test --profile ci -p telperion-render --lib compile_size, cargo test --profile ci -p telperion-core --test parameter_reference, memory items alone under scripts/gate_test.py: smooth_bark, form, ground_reach, two telperion_wasm entry tests
- PRs: