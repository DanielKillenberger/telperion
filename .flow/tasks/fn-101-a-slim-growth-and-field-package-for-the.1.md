---
satisfies: [R1, R2, R3, R4, R5, R6, R7]
---
# fn-101-a-slim-growth-and-field-package-for-the.1 Implement A slim growth-and-field package for the homepage

## Description
Ship a slim growth-and-field Wasm package with a typed entry point and an example voxelizer.

## Acceptance
Spec R1-R7.

## Done summary
`crates/telperion-field` (101,417 brotli bytes vs 172,127 target), entry `telperion/field` (1,067 brotli bytes JS), example voxelizer with fn-100's rules tested on a fixture grid, `docs/field-package.md`, the voxel experiment reproducing both accepted sheets byte for byte. R3 first missed the unmeasured 250 ms ceiling (273-649 ms) at parity with the full binding; the host amended R3 to parity.

## Evidence
- Commits: 044074525cb833c48055dcd9d694f2b28532386a, 217c502ee08b04d76f1972c50ec79fd2cb14ceb9
- Tests: cargo test --profile ci --workspace --no-fail-fast (891 passed); cargo test --profile ci -p telperion-field; npx vitest run (123 passed); node scripts/test-wasm.mjs; npx tsc --noEmit
- PRs:
