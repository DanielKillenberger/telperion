---
satisfies: [R2, R3]
---
# fn-270-growth-continues-from-any-age-in-less.2 GrowthState API and retirement without the final age, CPU then device

## Description
R2, R2a, R2a', R2b, R2c, R3.

## Acceptance
Split equality per executor; failure and admission tests; device hashes 0->H == 0->a->H.

## Done summary
Growth continues from any age on both executors (R2, R2a, R2a', R2b, R2c, R3). CPU API `telperion_core::pipeline::growth::{Params, begin, grow_to, settle}`; device `cycle::growth::GrowthState`. Retirement at the age grown is the only rule; device verdicts match the CPU's. Scratch dropped between calls rebuilds in 5-40 ms. Device 0->H == 0->a->H on every preset. Peak growing on beside a drawn tree up to +25 % (Telperion 5,603 MiB), handed to task 3. Gate 1,357 passed; codex SHIP after fixes.

## Evidence
- Commits: bd131606984957d4eeaa527b00fc83be1954c0e1, 5a83fb00aef8a0441aa3435e6c8109a11d432fed, 3f34aaacce751c3fb3fe9872c87dff13a8b0bab3, ee2a4dcfcf71b157794196a4d74dcc01c72e595e, 3be8a7cf838df1aee92e20a5acc499003ec9aa2d, 1c0df651f2209056adfdbb7d4257ec2a4d1516d6, 70ecdac559a125b3ef53a92a878f1fc0a3381906
- Tests: cargo test --profile ci --workspace --no-fail-fast, npm test, npx vitest run harness/parity.test.ts, PRESET=<id> cargo test --profile ci -p telperion-render --lib presets_grow_on_on_the_device -- --ignored
- PRs: