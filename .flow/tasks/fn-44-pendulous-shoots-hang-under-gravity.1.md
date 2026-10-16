---
satisfies: [R1, R2, R3, R4]
---
# fn-44-pendulous-shoots-hang-under-gravity.1 Implement Pendulous shoots hang under gravity

## Description
Add `skeleton.twigs.sag` (0 to 1, neutral 0): a hanging shoot's course turns toward straight down along its run, steepest at the attachment, carried as a heading beside the course. The birch states sag 1 (round 6b).

## Acceptance
- [x] R1 row railed, on the wire, blended, on the harness and in metadata; presets byte-identical at neutral, birch re-pinned with the reason.
- [x] R2 monotone ease-out turn over the pendulous length; droop, floor, separation unchanged.
- [x] R3 birch sag 1, round 6b recorded (`round6b-fn44` rows in `.flow/evidence/fn34/rounds.tsv`); 48/48 protocol.
- [x] R4 `tests/sag.rs`.

## Done summary
Hanging shoots sag toward vertical along their run by a row; neutral is the straight rod. Shoots averaged 1.07 m before floor or shell stopped them, so they turned 19 to 34 degrees. The owner accepted the silver birch at fn-34 round 25 (2026-09-16); the beech's verdict moved to fn-62.

## Evidence
- Commits: e140791b (branch: aa8aaf1, f594569, 23287c4)
- Tests: cargo test --release -p telperion-core/-p telperion-render, clippy, npm run typecheck, npm test, node tests/species.mjs --measure-only (48 cases)
- PRs:
