---
satisfies: [R1, R2, R3, R4]
---
# fn-195-tree-space-d-the-oak-as-a-point.1 The oak as a point: values and the five-seed sheet (R1); R2 to R4 after the host's verdict

## Description
The oak (Rauh) as values only (`crates/telperion-space/src/oak.rs`, sources in SOURCES.md), drawn by `examples/space_oak.rs` with the oregon-white-oak preset's rows; sheet at 80 years for seeds 1, 7, 2, 3, 4 and 10/20/40 years beside `.flow/references/oak-quercus-robur/`. Engine gaps go to the host.

## Acceptance
Every R-ID in the parent spec is satisfied; R1 first, R2 to R4 after the host passes the look.

## Done summary
Passed at round 10 on the owner's verdict (2026-10-05). R1 met; R2: gate 1,063 passed, 0 failed, Codex SHIP; R4 met by the beech and spruce neutral hashes and pixel-identical renders across the stem fix (670dacda); R3 split to task 2. Main known gap: density of the fine twig web.

## Evidence
- Commits: 54b9dff8, f3384827, 670dacda
- Tests: cargo test --profile ci --workspace --no-fail-fast, cargo test --profile ci -p telperion-space, cargo test --profile ci -p telperion-render --test space_stems, cargo check -p telperion-render --examples
- PRs:
