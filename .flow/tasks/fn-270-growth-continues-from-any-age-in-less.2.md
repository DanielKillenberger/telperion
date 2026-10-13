---
satisfies: [R2, R3]
---
# fn-270-growth-continues-from-any-age-in-less.2 GrowthState API and retirement without the final age, CPU then device

## Description
R2, R2a, R2a', R2b, R2c, R3.

## Acceptance
Split equality per executor; failure and admission tests; device hashes 0->H == 0->a->H.

## Done summary
Growth continues from any age on both executors (R2, R2a, R2a', R2b, R2c, R3). Retirement at the age grown is the only rule, with a fading set on the CPU and `fading_main` on the device; the device's verdicts match the CPU's word for word. CPU API: `telperion_core::pipeline::growth::{Params, begin, grow_to, settle}`. Device API: `cycle::growth::GrowthState`, crate-internal until task 5. Between calls the device keeps its state, retirement, hierarchy and census; it lets the light lattice, job counts, sums, buckets, the next-written lists, layout lists, compactor scratch, verdicts and the recorded cycle go, and rebuilds them in 5-40 ms. Requests the state cannot meet are refused by name. A failed growth or draw spends the state (`state.failed`). Growth and draws run under the state's budget, and nothing lets the drawn tree go. The 24-year walk premise is out of the gate and runs by name.

Measured: device 0->H == 0->a->H on 8 of 8 young presets (least tables, compacted every cycle, the spruce moving at the pause) and 8 of 8 presets at their ages (a = H/2, H-1). Peak while growing on beside a drawn tree: up to +25% (Telperion 5,603 vs 4,485 MiB; held paused 2,887 MiB). Host ruling (3be8a7cf8): the hierarchy is state; the `ret` split and the continuation peak go to task 3; owned parameters go to task 5, with no `unsafe`. Telperion's Wasm memory now passes 2 GiB, so the browser core reads addresses unsigned (`>>> 0`). Evidence: CONTINUATION.md.

Gate: `cargo test --profile ci --workspace --no-fail-fast` 1,357 passed, 0 failed, 89 ignored over 145 binaries. `npm test` 161 of 162 at first; after the unsigned-address fix, `harness/parity.test.ts` 8 of 8. The review fixes were re-checked by focused tests, not a second gate.

Tier: not stated by the conductor.

stage: impl-review - ran [codex: NEEDS_WORK (3 findings, fixed in 70ecdac55) .. SHIP]

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: bd131606984957d4eeaa527b00fc83be1954c0e1, 5a83fb00aef8a0441aa3435e6c8109a11d432fed, 3f34aaacce751c3fb3fe9872c87dff13a8b0bab3, ee2a4dcfcf71b157794196a4d74dcc01c72e595e, 3be8a7cf838df1aee92e20a5acc499003ec9aa2d, 1c0df651f2209056adfdbb7d4257ec2a4d1516d6, 70ecdac559a125b3ef53a92a878f1fc0a3381906
- Tests: cargo test --profile ci --workspace --no-fail-fast, npm test, npx vitest run harness/parity.test.ts, PRESET=<id> cargo test --profile ci -p telperion-render --lib presets_grow_on_on_the_device -- --ignored
- PRs: