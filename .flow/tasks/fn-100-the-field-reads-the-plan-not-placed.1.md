---
satisfies: [R1, R2, R3, R4, R5, R6, R7, R8]
---
# fn-100-the-field-reads-the-plan-not-placed.1 Implement The field reads the plan, not placed leaves

## Description
Build the field from the solved tree and a leaf plan instead of placed leaves.

## Acceptance
Spec R1-R8 (R8 is the owner's verdict).

## Done summary
The field answers foliage from a leaf plan (bearing runs, station counts, limb system, conservative reach); station preparation shares the plan; the binding places no leaf for a field request where a plan exists and reports its stages; batch queries add a count estimate and limb id; the core builds without surface and placement behind the default `geometry` feature. Field-only builds 3-40x faster (spruce 9.0 s -> 0.23 s), lower peak memory, wood bits byte-identical on eight trees.

## Evidence
- Commits: 7c90339fd03fadcce964084f1a59b3065eb71f81, bb3c25d715b7727dde5ffe11b750b24954553fef, acd969df024c1b74e59a437f24001b7deac13831
- Tests: cargo test --profile ci --workspace --no-fail-fast (760 passed); cargo test --profile ci -p telperion-core --no-default-features --lib; node scripts/test-wasm.mjs; npx vitest run
- PRs:
