---
satisfies: [R2, R4, R5]
---
# fn-91-fast-tree-generation-as-a-core-engine.2 Bound GPU leaf readback memory without changing output bytes

## Description
Bound GPU leaf readback memory without changing output bytes.

## Acceptance
A bounded task under the parent spec's Acceptance; the parent R-IDs it names are judged at the spec.

## Done summary
One fallibly reserved final vector and <=4 MiB staging shared by CPU output and verification readback; exact words and specimen hashes hold. Warm CPU-output p50: spruce 351.73 -> 297.03 ms, oak 259.27 -> 255.65 ms. Report: `.flow/evidence/fn-91-fast-tree-generation-as-a-core-engine/READBACK.md`.

## Evidence
- Commits: 6d5841d78bc39b0b6e7cb34829889f7ec96061ec
- Tests: focused core, renderer and Wasm suites as named in the report
- PRs:
