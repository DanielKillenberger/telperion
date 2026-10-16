---
satisfies: [R1]
---
# fn-270-growth-continues-from-any-age-in-less.1 Premises on the CPU (fail early)

## Description
R1 (a)-(d) and R2a/R2a' tests on the CPU cycle, before any API or device work. Stop on any failure and report.

## Acceptance
R1's tests run with their verdicts; a stop condition that fires ends the spec's build and goes to the host.

## Done summary
Premises held on the CPU cycle (R1 a-d, R2a, R2a'): every final-age read listed; a horizon fixes all but retirement's (`Retiring::AtAge`); horizon prefix, kept tree byte-equal 24/24, 0->H == 0->a->H on 8 presets (a = 10, 40, H-1), no snapshot mutation. Live-set peak +11.6 to +18.1 %. Host rulings after NEEDS_HUMAN: the stop reads the peak; R5a blends old organs between years.

## Evidence
- Commits: 3718708a8, c9bbfcf9d, 0ded361ca
- Tests: cargo test --profile ci --workspace --no-fail-fast, npm test, premises kept, split and walk tests
- PRs: