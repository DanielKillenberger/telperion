---
satisfies: [R1, R2]
---
# fn-140-generate-writes-three-holdout-seeds.1 Implement fn-140-generate-writes-three-holdout-seeds

## Description
Write three deterministic holdout seeds and cases in generate's packet.

## Acceptance
Spec R1-R2.

## Done summary
`write_packet` draws three holdout seeds from the species id (distinct, stable) into `holdout_seeds` and `holdout` cases; the packet-shape test updated for six cases; unit test red on the unfixed body. Gate green (1054 passed).

## Evidence
- Commits: 1bd7dd12426453955adc7cf97a4f1187cb83b4ff
- Tests: cargo test -p telperion-jev --lib pipeline::stages::generate::packet::tests; cargo test --profile ci --workspace --no-fail-fast (1054 passed)
- PRs:
