---
satisfies: [R1, R2, R3, R4]
---
# fn-48-a-clumps-stems-lean-unequally.1 Implement A clump's stems lean unequally

## Description
Add `skeleton.habit.stem_lean_spread` (0 to 1, neutral 0): each stem's lean slides toward its place in stem order, first at `stem_lean * (1 - spread)`, last at the full lean. Birch: two stems, divergence 0, lean 28, spread 1.

## Acceptance
- [x] R1 row railed, wire, blend, harness, metadata, habit audit, sweep; identity pins held at 0.
- [x] R2 leans spread in stem order.
- [x] R3 round 9b recorded (`round9b-fn48` rows in `.flow/evidence/fn34/rounds.tsv`); 48/48; heaviest birch seed 146,028 nodes.
- [x] R4 `tests/stem_lean_spread.rs` and the stems tests.

## Done summary
The birch has one upright and one leaning stem. Remaining faults belonged elsewhere: curtain hides the leaning stem (fn-51), dark leaf mass (fn-52). The owner accepted the silver birch at fn-34 round 25 (2026-09-16).

## Evidence
- Commits: d757f581 (branch: e5a4ca76, 283ec4e6, 50023362, 8c9af243)
- Tests: cargo test --release -p telperion-core/-p telperion-render, clippy, npm run typecheck, npm test, node tests/species.mjs --measure-only (48 cases)
- PRs:
