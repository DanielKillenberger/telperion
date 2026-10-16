---
satisfies: [R1]
---
# fn-252-a-shoot-that-reaches-the-ground-stops.1 Measure: crown lift and strands at the ground, young and old

## Description
R1 with the 2026-10-14 additions; stop and report to the host.

## Acceptance
R1's numbers and stills, the two causes told apart.

## Done summary
R1 measured the birch's crown base and the wood on the ground and whether growth's rough lay sees contact; R2a was scoped, recounted and withdrawn. R2 built as ruled: a per-age `ground_hazard` row (silver birch 1, a declared stand-in on the owner's verdict, sourcing in R2-PLAN.md; every other species 0), a survival factor while the rough layout rests an apex's tip on the ground, on both executors. The owner accepted it for the release on 2026-10-14; crown lift goes to fn-282. Impl-review SHIP after one fix; the full gate is the host's on the merged branch.

## Evidence
- Commits: 4c6524b316d16c4d0a94a7badf897558c3ee2fbe, 51dd04ba09dae12aff03946aa21acb21d38d6f66, 387164ae196ba2cb8a2e43950de929ea34fc06fb, 71bfd866932ce67c0dfa91ad26877bf02c86199a, 1af3e94fbba09e8c6fb83bf684dc8c12ab2d9bf4, 190a3778e02b96a6b42a7b39c820e76db438f040, ea2812326fae0a8d8ec63b58aae5a5bdf0998f1b, 9a9a734a4be0e922ada172711154235f415c4f91, e78477bb268a5a80b87e8a04c906be66cbb427fc, 43a2b9f280db7381defb395c766cd539540a8da6, e011227e645aea4becf2f7e1f9a50efa1f9c8a47
- Tests: cargo test --profile ci -p telperion-space (green; raw/space-tests.log), cargo test --profile ci -p telperion-space --test ground --test sag --test light, cargo test --profile ci -p telperion-core --test catalogue_identity --test space_presets, CYCLE_ONLY=silver-birch cargo test --profile ci -p telperion-render --lib -- young_trees_grow_on_the_device_as_on_the_cpu compile, GATE_SKIPPED:workspace:host - the full gate runs on the merged branch (host)
- PRs:
