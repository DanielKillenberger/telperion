---
satisfies: [R1, R2, R3, R4]
---
# fn-51-the-curtain-hangs-below-the-crown.1 Implement The curtain hangs below the crown

## Description
Add `curtainDrop` (0 to 1, neutral 0) and `curtainClearance` (0 to 5 m, never above the crown base): a hanging shoot may fall past the shell's lower surface into a band down to the clearance, via `Curtain::admits`. Owner: "makes sense". Birch: drop 1 to a 1.8 m clearance.

## Acceptance
- [x] R1 rows railed, wire, blend, harness, metadata; other presets byte-identical, birch re-pinned.
- [x] R2 only hanging shoots enter the band; containment tests assert the new invariant, the old at drop 0.
- [x] R3 round 9 recorded (`round9-fn51` rows in `.flow/evidence/fn34/rounds.tsv`); 48/48; heaviest birch seed 121,765 nodes.
- [x] R4 `tests/drop.rs` (eight tests), a unit test in pendant.rs, rails in pendulous.rs.

## Done summary
The birch's skirt hangs below its crown with the stems clear; strands still read as one wall rather than per-limb fringes. The owner accepted the silver birch at fn-34 round 25 (2026-09-16).

## Evidence
- Commits: bb2889d4 (branch: c6f34bbf, b88818b6)
- Tests: cargo test --release -p telperion-core/-p telperion-render, clippy, npm run typecheck, npm test, node tests/species.mjs --measure-only (48 cases)
- PRs:
