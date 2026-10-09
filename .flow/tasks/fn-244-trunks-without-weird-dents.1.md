---
satisfies: [R1, R2, R3]
---
# fn-244-trunks-without-weird-dents.1 Find the dents, fix them, the owner's verdict

## Description
TBD

## Acceptance
- [ ] TBD

## Done summary
The trunk dents are fixed. A relay now joins its parent at a single resolved attachment: its frame is read along the carried chain, and the join along the parent's own units. Girth splits the relay's wood between the two neighbouring nodes, and the drawing inserts a JOIN node there, on both the CPU and the device.

fn-199 R3 and R4 pass unchanged, and the review returned SHIP. The owner's verdict on the beech trunk sheet, 2026-10-09: "much better nice".

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 1f9afd53..2a70a3c7
- Tests: cargo test --profile ci (core, space, render, wasm, field, jev), npm test
- PRs: