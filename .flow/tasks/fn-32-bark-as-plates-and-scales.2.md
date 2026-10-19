---
satisfies: [R1, R4, R5, R6]
---
# fn-32-bark-as-plates-and-scales.2 Organicness harness and hill climb to the owner's acceptance

## Description
Round one was rejected as "armor plating". Reach acceptance by a measured hill climb: a structure score against the reference photographs judges every iteration, no model in the loop; the owner judges at most four images per round.

## Acceptance
- A structure score separates bark from brickwork
- Preset rows hill-climbed against it with no model in the loop; the primitive changes only when the row plateau is far from the reference
- Owner verdict recorded in the spec

## Done summary
Six-component score guarded by `bark_score.rs`; 1,510 trials. The column-and-cut network plateaued at 64% of start, so the primitive changed once to a weighted cellular partition with two warps. Round two: oak 1.2284, spruce 0.1657. Round three added `plateFurrowWidth`, crop-colour and dark-fraction constraints and a level pass (oak trunk 1.3293, spruce trunk 0.1870 under re-measured pins). Native 5.2142 ms p50. The owner accepted on 2026-09-15 ("ok do it") and the remaining gaps went to fn-42.

## Evidence
- Commits: bd2f1938b2054b986a66ecfee8b61ecaa71950b6, 438aac2c51962edb89f4c7ededd01fb481bc486e, cb86074288bcf6f9936003d60f44af23e01d7141, 2a23a59534b7bf35c580dcfa476fca6be0dcc723, 9abe16ac1898e6e109d439403cb9effb4765c4a6, 526da27ebb289ebd52e1e89be62a43242eea3aae, 5f75ec1d52aeed18356936634c4025a09cd5da1b
- Tests: cargo fmt --all -- --check, cargo clippy --workspace --all-targets -- -D warnings, cargo test --release --workspace, npm run wasm:build, npm test, npm run typecheck, ...
- PRs:
