# fn-340-the-cohort-law-carried-forward-the.1 Implement The cohort law carried forward

## Description
TBD

## Acceptance
- [ ] TBD

## Done summary
The cohort law carried forward was built as a probe and read: R1 to R3 corrected the probe where the cycle's own rules implied it (retirement reading the cohorts, frozen upkeep, the split as S1a) and carried each cohort's state a year at a time, equal to the recomputed law under constant and varying light. R4's read stops the approach by R5's rule as written: the oak is 3.88, 2.49 and 2.63 times cheaper at 40, 80 and 120, and its median trunk is 1.10, 1.35 and 1.21 times the cycle's, with spread 0.88 and leaf 0.86 at 120 (`.flow/evidence/fn-340-the-cohort-law-carried-forward-the/RESULT.md`).

Not isolated: the trunk's bias, the leaner oak at 120, and the law's open-loop leaf rising with the tree's age. The probe stays a patch (`probes/coupled2.patch`); no crate change stays, so no gate ran.

Tier: single minted implicit task, the frontier is exactly one (model: claude-opus-5-5)
stage: impl-review - skipped(config: REVIEW_MODE=none)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 63a98fe592c7855a9cf4071e8ebed849673e065c, c08f74eedc98a33e3cb0626c4532f8c2258d0dec, 8d49fd5855d6e59ad7ec3c5310bc1d2791122d3a, 1d08370959a6ca4ededc27a1cc0fb7a7d63b8745, ffaea6513cb0b1db77e656f0817f4ee1972d1ef7, c163dbccf8e1ce68a5e731f490531779b64faaf5, c1e0a7e124e41d45053cf564bb7a028ddb1d7e99
- Tests: cargo test --profile ci -p telperion-space --lib carried (under probes/coupled2.patch), GATE_SKIPPED:gate:no crate change stays (probe is a patch)
- PRs: