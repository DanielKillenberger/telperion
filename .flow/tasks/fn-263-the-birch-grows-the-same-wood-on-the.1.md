---
satisfies: [R1, R2, R3]
---
# fn-263-the-birch-grows-the-same-wood-on-the.1 Birch executor agreement

## Description
Implement fn-263 R1-R3 as written; repro first.

## Acceptance
The spec's R1-R3.

## Done summary
The CPU's sag (`geometry/sagged.rs`) turned a phytomer over its internode before its node's own draw. `Layer::step` lays the phytomer, and the device sags it, at the varied length. The birch is the only preset whose internodes vary, so its executors parted by 0.219 m. The sag now takes the length the phytomer is laid at, and the birch's own bound and its leaf exemption are removed.

- **R1:** the first stage apart was the sag, at the root axis with lineage 17581127258315254970 (PA 9), phytomer key 15222163334308185898. BIRCH.md records it.
- **R2:** the birch at 40 now agrees to 1.26e-4 m at most and 3.6e-6 m RMS, and every leaf agrees.
- **R3:** the stills were viewed, and the gate and npm test are green.

baseline: green (preset agreement on F4, the birch held to its own bound)

stage: impl-review - ran (codex, SHIP)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 537c6771926dbb38a42d15072eb52cb95a686198, 8d9d9bcc4d518635ab93e289e8573115506403fb, d859281e9bc6177deba88c22822d4a832c2f7765, ee5989cbdd66eda8b891383982e68cab856114e6
- Tests: cargo test --profile ci -p <space|field|core|render|wasm|jev> --no-fail-fast (render under GPU lock), npm test, cargo test --profile ci -p telperion-render birch_tests
- PRs: