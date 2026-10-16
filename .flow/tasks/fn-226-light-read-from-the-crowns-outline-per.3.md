---
satisfies: [R3, R4]
---
# fn-226-light-read-from-the-crowns-outline-per.3 Local light and local sharing

## Description
The spec's R3 and R4: first the R4 inventory and the written equations (evidence, before code), then the outline history recorded by the structure above the cut, local light and local sharing on the CPU, then the GPU executor inline in the growth step; beside today's model for the head-to-head only.
## Acceptance
- R4's inventory and equations written before code; missing expectations stopped for the host (done; host decisions S1-S8).
- R3 and R4 built per bud on the CPU, with no synchronisation added; the one-way history shown; pass count and sync points per cycle reported. The GPU half is out of scope (S8 (c)).
- Analytic tests of chords and transmission through a cone stack, and every bootstrap and degenerate case of R3.
- Measured for the head-to-head. The gate's one red walk is today's model's, carried into task 4 by the owner's verdict.
## Done summary
R3 and R4 built on the CPU (outline light, local sharing, one-way history) with analytic chord, transmission and degenerate-case tests. Head-to-head on the lit oak: 0 sync points a cycle against 4, but a tree half as tall at 80, clear trunk ~2 m against 4.7 m, 44-70 % slower at 40. Owner verdict (2026-10-08), on the host's recommendation to close with today's model: "ok". Review P1 overridden: it concerned code the verdict removes.

## Evidence
- Commits: 923e927471c16ce9c758524e266d9ec60bc60cd5, 143ad512d8f2e4799fa02fd3f2f1e46fb6e37df8, dda49857aa11ea7a18ae9d85cbfeb81e9bf424cb, e03da4e55b576108e5238645483a69e604ddbee8, 7eb5aa0e6a39979f93332228d05e83fe5bf963a0
- Tests: cargo test --profile ci --workspace --no-fail-fast (two parts)
- PRs: