# The oak at 80 builds at every seed

## Goal & Context

fn-236's round 4 (2026-10-14, SHEET-4.md): the Oregon white oak at age 80, seed 7, is refused with "invalid input: unsolved radii"; round 3's rows refuse the same way, so it predates the round. The harness's age dial reaches 80 and the site's grove draws seeded oaks, so a shipped preset refused at a reachable age is a release defect. The radii solver's failure condition and why this tree meets it are unknown.

## Requirements

- **R1 (repro first):** the refusal as a standalone test, red, on the CPU and the device; a sweep of the oak at 60-100 and of every shipped preset at its age and at 80, seeds 1-50, listing every "unsolved radii" refusal.
- **R2 (cause):** the solver's failure condition against this tree's structure, checked in code, written down before any fix and reported to the host.
- **R3 (fix):** every tree the growth law makes has its radii solved, by degree (no fallback to a default radius, no species branch); the sweep has no such refusal; shipped trees' bytes unchanged where they solved before; gate, npm, review.

## Boundaries

Release item (fn-237). The oak's values are fn-236's.
