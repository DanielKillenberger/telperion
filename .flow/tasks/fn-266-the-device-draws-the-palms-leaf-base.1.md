---
satisfies: [R1, R2, R3]
---
# fn-266-the-device-draws-the-palms-leaf-base.1 Palm boots and cells on the device

## Description
Implement fn-266 R1-R3; design (DESIGN.md 12.8) first.

## Acceptance
The spec's R1-R3.

## Done summary
The device hangs the date palm's retained leaf bases: a stage between dress and folds adds two drawn records and a cell per base; the sweep reshapes each base onto its cell; the handoff passes cells as sections. `executor::bases` replaces `hangs_bases`. Parity (`boots_tests`, ages 12 and 40): 256/256 matched, points within 9.0e-5 m, directions within 2.2e-4; controls refused. Stills show F4's diamond lattice; warm build 98 ms. Codex SHIP.

## Evidence
- Commits: 5c15a9b420f86c7e44b8468d4197e87223695c79, 7cb5f52de6aaf6b40613113d3973086b8ef77588, b48ccf9cde26af354f24e3f3db44cb53cb76cd28, c2a866360f01e5a5972f2f28fe79eddca5470cdb, 51918a2dbb9cd3a255634f85ea27c9a13c1787e4
- Tests: cargo test --profile ci --workspace --exclude telperion-render --no-fail-fast, cargo test --profile ci -p telperion-render --no-fail-fast (GPU lock), cargo test --profile ci -p telperion-jev --test replay, npm test
- PRs:
