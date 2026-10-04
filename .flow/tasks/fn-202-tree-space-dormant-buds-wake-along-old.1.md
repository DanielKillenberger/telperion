---
satisfies: [R1, R2, R3, R4, R5, R6]
---
# fn-202-tree-space-dormant-buds-wake-along-old.1 Dormant buds wake along old branches: engine, closed form, walks, spruce strip

## Description
Build the spec's Design as written: per-zone dormant table with delay and rate, lineage-keyed draws, waking by degree, only on living wood, closed form; R1 to R6.

## Acceptance
R1 to R6 of the spec.

## Done summary
Dormant buds wake along old branches, built to the spec's Design and host decisions 1 to 3.

- **Settings:** each zone has a `dormant` table and a release law (`delay`, `rate`).
- **Draws:** keyed by lineage. The waking time is continuous.
- **Waking:**
  - The partial first unit grows its share of the waking cycle and runs that share of the year's risks.
  - A bud ages through its stages while it sleeps.
  - It wakes only if its bearer is carried on through the waking cycle. It then keeps share + (1 - share) p of its size, where p is its bearer's presence through the next cycle.
  - The abortion hazard and secondary erection count the time it slept.
  - It stands on its slot's side, half an internode below its node.
- **Closed form:** follows all of the above.
- **Neutral:** byte-identical (beech stills, oracle).
- **Tests:**
  - R2: lineage-keyed draws.
  - R3: six release-law walks (red first on dcf6add1), plus the sleeping-probability, survival and abortion walks.
  - R4: 4,000 seeds against the closed form.
- **R5:** spruce strip at delay 1, rate 0.3. It changes the tree by degree and fills the combs in the spray plane.
- **Gate:** 1,034 passed.
- **Codex:** SHIP in round 2.

Evidence: `.flow/evidence/fn-202-tree-space-dormant-buds-wake-along-old/RESULT.md`.

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: dcf6add1, e297fbda, fe019b78, 316e515f, 4e402108
- Tests: cargo test --profile ci -p telperion-space --no-fail-fast, cargo test --profile ci --workspace --no-fail-fast
- PRs: