---
satisfies: [R1, R2, R3]
---
# fn-123-a-familys-rows-can-be-validated-without.1 Implement fn-123-a-familys-rows-can-be-validated-without

## Description
TBD

## Acceptance
Every R-ID in the parent spec's ## Acceptance Criteria is satisfied; judge this task against the spec's criteria directly.

## Done summary
`Family::validate` in `telperion-core` checks every row the build and the growth path check, in the same order, with no tree grown. The host's decisions for R1 are in place, and fn-113's `dial_bounds.rs` now uses `validate` in place of its one-node tree (R2).

- **R1:** `src/family/tests.rs` sets every wire row to each of 9 out-of-range test values on all 8 families; only `clumpSystemOrder`, `seed` and twig `divergence` are never refused. A second test holds `validate` to the build on every family, made small: both must give the same error for each value.
  - Values only the build refuses (overflow at absurd magnitudes) are pinned per family in `BUILD_ONLY`, as the host decided.
  - New floors: `trunkHeight > 0` in `resolved_growth` and `maxInstances >= 1` in the canopy rails. `twigTipTaper` is now in (0, 1]; no shipped preset uses 0, and the smallest is 0.25.
  - `maxNodes >= 1` is refused in `Family::validate` only. A floor in `resolved_growth` broke four tests that pin a zero ceiling as a resumable empty seedling on `generate` and the growth path. The build and `validate` both refuse 0, under different names. This placement is my own choice, not the host's, and is marked [inferred] in the spec.
- **R2:** `crates/telperion-jev/tests/dial_bounds.rs` judges each row with `with(...)?.validate()`; all 5 of its tests pass.
- **R3:** the gate `cargo test --profile ci --workspace --no-fail-fast` passes: 131 test-result lines, 970 passed, 0 failed, 21 ignored.
- The spec's Architecture section records the confirmed validator map and the host decisions. `docs/generation-limits-inventory.json` has one surface height entry and a new `trunkHeight` floor entry.
- **Friction:** 3 entries in `.flow/evidence/fn-123-a-familys-rows-can-be-validated-without/FRICTION.md`. The agreement test adds about 40 s to each gate run.

Tier: session (actual model: claude-opus-5-5)

stage: impl-review - skipped(config: REVIEW_MODE=none)
## Evidence
- Commits: 7c735bb0536439bc28fc2deb6c38865632799a6f, a9187b2328a30237cc78ee31cc3715b145b45b50, 38fc200cd72bc29717e5a7a297c8ce91fb77275d, 5e46fabe22a07c7fa4bbc3adfc2f271b79569d3c, 34cb440386ef3621ecb17be88faab49334244e2c, 19f35ff9108b2ea0025b01d29d644505c7a9ee75
- Tests: cargo test --profile ci -p telperion-core --lib family::tests, cargo test --profile ci -p telperion-jev --test dial_bounds, cargo test --profile ci --workspace --no-fail-fast (suite_rc=0; 131 test-result lines: 970 passed, 0 failed, 21 ignored), baseline: none run pre-edit (project rule: the gate runs once, at the end of a task)
- PRs: