---
satisfies: [R1, R2, R3]
---
# fn-210-tree-space-f2-the-exact-speed-levers.1 The exact speed levers: shedding decided before growth, parallel growth, light's wasted work, a compact phytomer

## Description
Build fn-198's exact speed levers, each hash-identical, and measure per-stage cost at 80 years.

## Acceptance
- Hash proof per lever; per-stage cost before/after; gate and Codex review.

## Done summary
Levers 1 (beech, spruce, palm), 2 (growth and lay on every core), 3 and a closed-form cache built, each hash-identical at 80 years, seeds 1 and 7, neutral and lit. The oak's shed subtrees under light not built (host design question). Compact phytomer (4% of peak) and e1 not built. Totals at 80 years: beech 3.6 to 0.85 s, spruce 17 to 3.4 s, oak in light 28.5 to 5.8 s (seed 1). Gate 1,100/0; Codex SHIP (round 2).

## Evidence
- Commits: b23792de, 0258c9a0, ecca0a30, f579008d, f0793ed8, 7676bec198ac46d17230bfdd8baaaff9e1b16a01
- Tests: cargo test --profile ci --workspace --no-fail-fast, measures hash 80 1,7 <species> both
- PRs:
