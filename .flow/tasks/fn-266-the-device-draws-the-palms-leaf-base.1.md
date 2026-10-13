---
satisfies: [R1, R2, R3]
---
# fn-266-the-device-draws-the-palms-leaf-base.1 Palm boots and cells on the device

## Description
Implement fn-266 R1-R3 as written; design (DESIGN.md 12.8) first.

## Acceptance
The spec's R1-R3.

## Done summary
The device now hangs the date palm's retained leaf bases (fn-256 DESIGN.md 12.8). A stage between the dress and the folds adds two drawn records a base and a cell each. The folds leave the bases out of the envelope and take them into the leaf box. The sweep reshapes a packed base's run onto its cell with squared frames, and the handoff passes the cells as the curve's sections. `executor::bases` replaces `hangs_bases`, and the device's refusal by that capability is gone.

Parity (`boots_tests`, the palm at 12 and 40): 256 of 256 bases and shaped runs, matched by apex key and ordinal. Points are within 9.0e-5 m and directions within 2.2e-4, against bounds of 2e-4 and 5e-4. The missing-base and moved-cell controls are each refused. Stills against F4 (viewed): the trunk carries F4's diamond lattice. Pixels differing: whole 2.5%, mid 12.8%, close 14.0%, the leaflets' own draws. Warm build: 98 ms with the bases, 96 to 99 ms without.

Found by runs and recorded in 12.8.1: the device's storage limit is 8 (the cells ride in the sweep's records binding); the flags take bits 30 and 29, above the leaf count; the oldest base stands on its bearer, so it is merged as `paths.rs` merges it.

stage: impl-review - ran (codex, three draws, SHIP, no findings)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 5c15a9b420f86c7e44b8468d4197e87223695c79, 7cb5f52de6aaf6b40613113d3973086b8ef77588, b48ccf9cde26af354f24e3f3db44cb53cb76cd28, c2a866360f01e5a5972f2f28fe79eddca5470cdb, 51918a2dbb9cd3a255634f85ea27c9a13c1787e4
- Tests: cargo test --profile ci --workspace --exclude telperion-render --no-fail-fast, cargo test --profile ci -p telperion-render --no-fail-fast (GPU lock), cargo test --profile ci -p telperion-jev --test replay, npm test
- PRs:
