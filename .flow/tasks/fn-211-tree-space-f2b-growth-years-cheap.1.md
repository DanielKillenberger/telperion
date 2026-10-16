---
satisfies: [R1]
---
# fn-211-tree-space-f2b-growth-years-cheap.1 Where a growth year's time goes

## Description
R1: a per-year, per-pass profile of today's whole-tree growth (and, once fn-209 task 1 lands, of the structure above its cut): every pass in grow.rs's step and the yearly sketch, shade, re-lay, and the settle (assign, shed, scale, thicken, lay), for beech, spruce, lit oak and palm at ages 40 and 80, seeds 1 and 7, on 1 and all cores, with each pass's share that is serial. No change to any tree; the profile names which passes layer 1 (exact parallelism) and layer 2 (incremental years) address and the expected gain of each, as estimates labelled so.

## Acceptance
- PROFILE.md with the per-pass, per-year table and the serial share of each pass. - Each pass mapped to layer 1 or layer 2 with an estimated gain, labelled as an estimate. - No product behaviour changed; any instrumentation removed or behind the existing stage reports.

## Done summary
PROFILE.md in .flow/evidence/fn-211-tree-space-f2b-growth-years-cheap/ has the per-pass, per-year profile of today's whole-tree growth. It covers beech, spruce, lit oak and palm at ages 40 and 80, seeds 1 and 7, on 1 and 32 cores, with each pass's serial share. Each pass is mapped to layer 1 or layer 2 with an estimated gain, labelled as an estimate, and the file names what each pass reads and writes per year.

- The oak is 91 to 92% serial.
- At 80 years the oak's years cost 3.6 s (seed 1); the re-lay, sketch, allot and balance lead.
- The spruce's and beech's years are only new growth. Their cost is the settle and lay, done once per build.

Measurement only: the timers were a hand patch, kept as profile-instrumentation.patch and reverse-applied. Nothing under crates/ or src changed, so no workspace gate was run.

## Evidence
- Commits: 77d42091
- Tests:
- PRs: