---
satisfies: [R1, R2, R3, R4]
---
# fn-193-tree-space-c-the-beech-as-the-first.1 Implement Tree space C: the beech as the first point

## Description
The beech as a point in the tree space, values only (`crates/telperion-space/src/beech.rs`), drawn through the pipeline's expansion by `crates/telperion-render/examples/space_beech.rs`.

## Acceptance
Every R-ID in the parent spec's Acceptance is satisfied.

## Done summary
Passed on the owner's verdict (2026-10-04: "i mean to me they look really good structurally") after 22 rounds: round 20's values with a 9-cycle clear bole (round 22). The engine gained per-PA `Form` (tropism, elevation, wander, plane, pipe girth, roll, exponent, ripening, dominance at neutral 0) and Troll's relay modules for trunk, limbs and boughs. Rounds 3 and 4 stopped on the wrong-path rule (relay presences collapsed the tree); round 10 made relays continue their axis (worst walk slope 2.82 against 30). Astra failed rounds 11 to 21 on crown architecture and branch hierarchy, so the gate went to the owner. Codex review NEEDS_WORK; dbh on relays, SOURCES.md and R4 fixed; four P1 relay-continuity findings moved to fn-199 (host override). Known gaps: Astra's hierarchy critique, tube-like branch surface, young ages. Gate: 1018 passed, 0 failed.

## Evidence
- Commits: 648d6405, 068d1ded, 02a9aa61, 57ab643f, fbcf6c3d, 71dbaa3b, 9acc6dda, f85225c1, 5696674e, 75f733ba, 3c89f5a4, a666161a, 6bb530aa, 39ab1ba7, 52c14a78, aae9a76e, a5d6ec51, eaedb3d3, c08bb389, 903ade4f, 90cc1a6f, e179c8dd, a8daf966, 3bd91bc7
- Tests: cargo test --profile ci -p telperion-space --no-fail-fast, cargo test --profile ci --workspace --no-fail-fast
- PRs:
