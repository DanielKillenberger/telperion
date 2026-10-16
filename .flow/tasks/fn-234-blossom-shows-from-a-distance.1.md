---
satisfies: [R1, R2, R3, R4]
---
# fn-234-blossom-shows-from-a-distance.1 Measure where blossom hides, the host's design, built on both executors

## Description
Measure where blossom hides at the whole-tree view, build the host's design on both executors, and prove it, per the spec.

## Acceptance
- [ ] TBD

## Done summary
R1: florets stand at the leaves' depth, leaves hide 36-54 %; petals at 0.1 drew grey-green through the leaves' transmission colour. R2 as the host designed it: petals pass their own front colour in the organ block; petal rows are albedos (white about 0.8, docs corrected). White blossom now shows across the crown, CPU and device alike; the host accepted it and amended R3 (phenology post-release; Laurelin's gold is fn-218's). Red/green `tests/petal_white.rs`; `blossom_parity` coverage 0.02220/0.02222; flowerless presets byte-identical. Follow-ups: 0.8 % of florets in no depth band; a 219 s device prepare of the shipped beech; `blossom_parity` slow under load. Review: codex 3 draws SHIP.

## Evidence
- Commits: b61eabe3df47e2e03d7face5aab7e5fb9d2edd0b, fd54ea33e31a94c0aa4718750ff3911c694d93a5, a495d61bbb42c8eb8e6715b6784052d65d9ab30f, bca83b6392544f0f591717805b5699910072427f, 72cac1de19a5284c567f14c782c058f6b7f53b58, a968fc65c9f8b27913e2a6195053663f4aa396e4
- Tests: cargo test --profile ci --workspace --no-fail-fast, npm test
- PRs: