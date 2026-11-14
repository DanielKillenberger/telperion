# fn-368-the-growers-speed-loop-the-same-tree.1 The speed loop on the grower

## Description
TBD

## Acceptance
- [ ] TBD

## Done summary
R1's reading (READING.md) names six bit-exact candidates, none a tenth on paper. Round 1 is kept: a kept subtree that has faded out is retired at the living it was kept at, and is not evaluated again. It costs 3.6 to 5.3% less CPU at one thread, and the digests of all four trees are equal. Under R4 the loop ends there. The oak at 40 grows in 1.44 s on one thread and 0.68 s on four, against an aim of 100 ms (ROUNDS.md). The preset digest pin was already red on the base, and round 1 reads the same as the base, entry for entry (FRICTION.md 03:40).

stage: impl-review - skipped(config: no review dispatch, per the host)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: d59085eccb0a61770431cc40b11916378c8b1e29, 2c23a53c31f3c2c59116b34d5d70a219ebd93841
- Tests: .flow/evidence/fn-368-the-growers-speed-loop-the-same-tree/tests.sh r1 (pin red on base too), .flow/evidence/fn-368-the-growers-speed-loop-the-same-tree/ab.sh r0 r1 5, cargo test --profile ci -p telperion-core --test generation_limit_guard, cargo test --profile ci -p telperion-render --lib compile_size
- PRs: