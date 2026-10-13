---
satisfies: [R1]
---
# fn-270-growth-continues-from-any-age-in-less.1 Premises on the CPU (fail early)

## Description
R1 (a)-(d) and R2a/R2a' tests on the CPU cycle, before any API or device work. Stop on any failure and report.

## Acceptance
R1's tests run with their verdicts; a stop condition that fires ends the spec's build and goes to the host.

## Done summary
Premises on the CPU cycle (R1 a-d, R2a, R2a'). `Paused` splits the existing cycle loop between years; `grow_cycle` runs through it. Every final-age read is listed against code; a horizon fixes all but retirement's, which `Retiring::AtAge` replaces (test hook; task 2 makes it the rule).

Held: the horizon prefix (every preset, every year), the kept tree byte-equal (24/24), 0->H == 0->a->H (8 presets, a = 10, 40, H-1), no snapshot mutation, R2a/R2a' with a stand-in presentation. Live-set peak +11.6% to +18.1%.

Host rulings after the worker's NEEDS_HUMAN (spec R1, R2a, R5a): the stop reads the peak (it sizes the arena); per-cycle rises up to +91% feed R4 lever 4. The horizon adds no pop; shedding's whole-year steps and old organs' yearly moves are smoothed by R5a's blend between the previous and the committed year's drawing, keyed by identity. Evidence: PREMISES.md.

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 3718708a8, c9bbfcf9d, 0ded361ca
- Tests: cargo test --profile ci --workspace --no-fail-fast, npm test, premises kept/split/walk (raw/)
- PRs: