---
satisfies: [R1, R2, R3, R4, R5, R6]
---
# fn-202-tree-space-dormant-buds-wake-along-old.1 Dormant buds wake along old branches: engine, closed form, walks, spruce strip

## Description
Build the spec's Design: per-zone dormant table with delay and rate, lineage-keyed draws, waking by degree, only on living wood, closed form; R1 to R6.

## Acceptance
R1 to R6 of the spec.

## Done summary
Built to the Design and host decisions 1 to 3: `dormant` table and release law per zone, lineage-keyed continuous waking, partial first unit running its share of risks, buds aging while asleep, woken size share + (1 - share) p, placed half an internode below its node; closed form follows. Neutral byte-identical. Tests: R2 draws, six release-law walks (red first on dcf6add1), R4 over 4,000 seeds. R5 spruce strip at delay 1, rate 0.3 fills the combs by degree. Gate 1,034 passed; Codex SHIP round 2.

## Evidence
- Commits: dcf6add1, e297fbda, fe019b78, 316e515f, 4e402108
- Tests: cargo test --profile ci -p telperion-space --no-fail-fast, cargo test --profile ci --workspace --no-fail-fast
- PRs:
