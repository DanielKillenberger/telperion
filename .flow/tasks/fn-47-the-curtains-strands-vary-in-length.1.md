---
satisfies: [R1, R2, R3, R4]
---
# fn-47-the-curtains-strands-vary-in-length.1 Implement The curtain's strands vary in length

## Description
Add `skeleton.twigs.pendulousVariation` (0 to 1, neutral 0): each hanging shoot runs its own share of the pendulous length, keyed on station birth identity and bud place xor the seed. Birch: 0.95 over 3 m plus a lumpier shell.

## Acceptance
- [x] R1 row railed, wire, blend, harness, metadata; other presets byte-identical, birch re-pinned.
- [x] R2 runs spread to a twentieth of the length, independent of growth order; short strands end vertical.
- [x] R3 round 8b recorded (`round8b-fn47` rows in `.flow/evidence/fn34/rounds.tsv`); 48/48; heaviest birch seed 107,569 nodes.
- [x] R4 `tests/strands.rs` (six cases) and the rail in `pendulous.rs`.

## Done summary
Each hanging shoot draws its own share of the pendulous length, so the hem is ragged (18% of columns clear of the bottom vs 7%); the curtain still ended on the shell's lower surface. The owner accepted the silver birch at fn-34 round 25 (2026-09-16); the beech's verdict moved to fn-62.

## Evidence
- Commits: e5636535 (branch: 27e02b0, 49bd8e0, 0944780)
- Tests: cargo test --release -p telperion-core/-p telperion-render, clippy, npm run typecheck, npm test, node tests/species.mjs --measure-only (48 cases)
- PRs:
