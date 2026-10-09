---
satisfies: [R1, R2, R3, R4]
---
# fn-234-blossom-shows-from-a-distance.1 Measure where blossom hides, the host's design, built on both executors

## Description
TBD

## Acceptance
- [ ] TBD

## Done summary
R1 measured where blossom hides (MEASURE.md). The florets stand at the leaves' own depth, and the leaves hide 36 to 54 % of them. Petals at the old 0.1 values drew grey, tinted by the leaves' transmission. R2, as the host designed it: petals pass their own front colour in the organ block, and petal rows are albedos (white about 0.8, docs corrected). Re-measured on both executors beside the cherry photograph: white blossom shows across the crown, CPU and device alike. The host accepted it and amended R3 (phenology is a post-release spec; Laurelin's gold hue belongs to fn-218).

- Red/green: `tests/petal_white.rs` (backlit petal (126, 193, 139) on the base, (188, 190, 188) after).
- R3 fixtures:
  - `blossom_parity::the_flowering_tree_draws_alike_on_both_executors`: coverage 0.02220 / 0.02222, petals within 2 sRGB.
  - `florets_tests::florets_on_the_device_match_the_whole_tree`.
  - Shipped presets without flowers render byte-identical to the base: beech, oak and spruce on the CPU path, oak on the device path.
- Cost: no WGSL, draw or buffer added; the organ uniform takes another source for one colour.
- Follow-ups:
  - The measurement's depth quartiles include florets in sparse direction bins (NaN depth) in the boundaries, so about 0.8 % of florets fall in no band (review P3).
  - The device prepare of the shipped beech took 219 s in `generation_gpu` (FRICTION.md).
  - `blossom_parity` ran over 60 s under the gate's parallel load (about 5 s alone).

stage: impl-review - ran (codex fan-out, three draws SHIP)
Tier: none given

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: b61eabe3df47e2e03d7face5aab7e5fb9d2edd0b, fd54ea33e31a94c0aa4718750ff3911c694d93a5, a495d61bbb42c8eb8e6715b6784052d65d9ab30f, bca83b6392544f0f591717805b5699910072427f, 72cac1de19a5284c567f14c782c058f6b7f53b58, a968fc65c9f8b27913e2a6195053663f4aa396e4
- Tests: cargo test --profile ci --workspace --no-fail-fast, npm test
- PRs: