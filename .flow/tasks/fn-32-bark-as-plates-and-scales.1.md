---
satisfies: [R1, R2, R3, R4, R5, R6, R7]
---
# fn-32-bark-as-plates-and-scales.1 Implement Bark as plates and scales

## Description
Implement the plate network, depth, directional occlusion, colour by structure and the measurement receipt (R1-R7).

## Acceptance
Every R-ID in the parent spec's Acceptance is satisfied.

## Done summary
Plate network: the circumference partitioned by the `bark_column` circle embedding, columns cut along the run into ragged plates, girth-scaled, with domed faces, lifted rims and per-plate identity; colour follows structure, affine in filtered height. Parallax chosen over vertex displacement by measurement (see the spec). Fifteen rows default zero. Native oak 4.5192 ms p50 (p95 4.9339), +0.5369 over fn-29. Round-one stills were rejected by the owner as armour plating; task 2 continued.

## Evidence
- Commits: 7fbc411c0c81bf7de59099ba17689734be6066a3, 49ed1084d1116467e478cf55ea13372b70c67af8, 9b4689de6d5297aa8cbf97f234dc6784f4a333e4, f246b3549dd561ec4b5a8c61b736e81fb199378d, bc38f630eb8ad87e5b753f1fd361af9a07490751
- Tests: cargo fmt --all -- --check, cargo clippy --workspace --all-targets -- -D warnings, cargo test --release --workspace, npm run wasm:build, npm test, npm run typecheck, ...
- PRs:
