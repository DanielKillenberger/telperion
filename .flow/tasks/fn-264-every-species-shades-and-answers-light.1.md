---
satisfies: [R1, R1a, R2, R2a, R3]
---
# fn-264-every-species-shades-and-answers-light.1 Sourced light rows and leaf area for every species

## Description
Implement fn-264 R1-R3 as written: R1 inventory first, R1a the shared environment, R2 one species at a time (Jev screens, code copies, checked against source), R2a the response tests, R3 stills and costs.

## Acceptance
The spec's R1-R3.

## Done summary
Every shipped preset grows under one site light (k 0.5, Bréda 2003), and every species now shades with sourced or host-ruled leaf area: the beech and the Two Trees measured, the birch, spruce and palm derived. Each species answers light through one shade-tolerance mapping for the hazards, with shared upkeep and leaf girth; the size response stays dormant by host ruling after it ran the birch's long shoots bare. R2a tests prove each response at its consumer, and R3's stills, costs and fn-251's frame are measured. The owner's verdict on the sheets (2026-10-13): "yea looks good the artifact".

- Device cost: Telperion 1.20 → 1.98 s (+65%), Laurelin 1.11 → 1.81 s (+63%), +0.6 GB peak each, all in the light phase; the CPU reference is unchanged.
- Open: retained evergreen foliage (a host spec), and the birch's ground-running wood (fn-252, evidence filed).

stage: impl-review - ran (codex: NEEDS_WORK x2, fixed and host-ruled, then SHIP; fresh round SHIP x3)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 0d6447340f5ef2c7f646d289b2276cfbf0e210ed, d46044868c6f3c870fad195d0d4e12f0b2ded534, c64cc09278f366db962faee3d728d0398c41f56e, ddc1ec3101d8bc6f7170b128e3aacb217ff98be7, 75de71eeae6dca76c3266a873799b389c4f4f42d, a6a0f545868e2a777eef2e3a630b8743e4611c98, ce6a51682267c035b2ad4ee583d9a0679e3cc782, 701bcef7633a688820a1412a5a588642c20e1a44, 8a9e7db8b6f498fb8288dad8f9fbd7f16d78ff7b, 0371ac0328806291c3bedd244b61dcfea7937443, 00956e4c7ed6eda0f6252cf3858b7642e758f608, 28897c6f1d0661bbaa4c60c0bca3b4d0768ea0eb, fc0257862fdc1c4da5dce3b52aa5d589afbaad9d, 176657cacdcd8af68bc78d34b4d1fed464434eb9, b30517796263c11862375ff233b3bf054885d4b2, fc0101347bc30b8b0943cce8b7c7119803038a41, 720ac0f04a994b6cee18f3b9948282152a1b398a, c116854f3a125df3bb06e1516b311b6cbb059a3a, 47a8391dafc50d21542f09e57fc5ccf856b697eb, 08562b51e7c3a4813da47c56248e9187e0c4882f, 8fba993aef78468b6e0336a4a69c18b04c385c7b, a03239abf876b59798ef225559f3fc9e3fa8cfc4, 652b064f88cb3e232535dcd369981d9c80678e64
- Tests: cargo test --profile ci --workspace --exclude telperion-render --no-fail-fast, cargo test --profile ci -p telperion-render --no-fail-fast, npm test
- PRs: