---
satisfies: [R1, R2, R3, R4]
---
# fn-50-short-shoots-clothe-the-limbs.1 Implement Short shoots clothe the limbs

## Description
Add short shoots as placements on existing wood (`foliage/short_shoots.rs`) with five canopy rows; set the beech's spurs and, by host direction, its twig rows and leaf lean.

## Acceptance
- [x] R1 five rows railed, wire, blend, harness, metadata; pins held.
- [x] R2 clusters on eligible wood above the crown base; node count identical with short shoots on and off.
- [x] R3 round 12 recorded (`round12-fn50` rows in `.flow/evidence/fn34/rounds.tsv`); 48/48; 4.25 to 5.60 million leaves in band.
- [x] R4 short-shoot tests, split under the 400-line rule.

## Done summary
Short spur shoots with leaf clusters clothe the limbs by rows; the beech's crown is leafed through its inside. The capability is merged; the beech's verdict is fn-62's. The owner accepted the silver birch at fn-34 round 25 (2026-09-16).

## Evidence
- Commits: d0bf6f21 (branch: 57b2401c, 79f8df8a, 0b4c3846, db38df56)
- Tests: cargo test --release -p telperion-core/-p telperion-render, clippy, npm run typecheck, npm test, node tests/species.mjs --measure-only (48 cases)
- PRs:
