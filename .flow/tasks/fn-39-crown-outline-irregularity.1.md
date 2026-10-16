---
satisfies: [R1, R2, R3, R4, R5]
---
# fn-39-crown-outline-irregularity.1 Implement Crown outline irregularity

## Description
Give the envelope a seeded low-frequency radius perturbation by height and bearing via `skeleton.envelope.irregularity` and `lobe_scale`, used by containment, the scaffold and the twig planner (R1-R5).

## Acceptance
- [x] R1 two railed rows; presets at zero byte-identical
- [x] R2 perturbed shell drives growth and containment
- [x] R3 outline statistic in the compare script
- [x] R4 beech and birch amplitudes; round-5 pairs; verdict in fn-34
- [x] R5 tests

## Done summary
The crown shell takes an irregular outline by amplitude and wavelength rows; neutral is the smooth shell to the byte. The owner accepted the silver birch in fn-34; the beech moved to fn-62.

## Evidence
- Commits: 40e42bb5
- Tests: cargo fmt --all -- --check, cargo clippy --release --workspace --all-targets -- -D warnings, cargo test --release -p telperion-core --no-fail-fast, cargo test --release -p telperion-render --no-fail-fast, ...
- PRs:
