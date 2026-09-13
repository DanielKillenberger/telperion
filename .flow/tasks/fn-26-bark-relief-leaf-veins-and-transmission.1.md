---
satisfies: [R1, R2, R3, R4, R5]
---
# fn-26-bark-relief-leaf-veins-and-transmission.1 Implement Bark relief, leaf veins and transmission

## Description
TBD

## Acceptance
Every R-ID in the parent spec's ## Acceptance Criteria is satisfied; judge this task against the spec's criteria directly.

## Done summary
Bark relief as a normal perturbation from the distance and arc-length coordinates (vertical grain, shouldered furrows, lifted plates, a fine flake; filtered by pixel footprint on both axes, averaged not switched, removed relief handed to roughness), leaf veins and margin tone from the blade coordinates (absent at section roundness 1), two-sided transmission gated by the shadow map, eight new material-row fields validated and blended; level selection compacted in placement order so a redraw is exact. R5: native oak 3.6879 / 4.0284 ms total p50 / p95 against 3.8 ms; browser orbit at 60 fps. R4: accepted by the owner on 2026-09-13 in the owner's words, recorded in the spec. Report and stills under .flow/evidence/fn26. Six bridged runs; all five gates green at every commit on the host's own runs.
## Evidence
- Commits: dcb2669, 674e2f7, f9721ee, 7eef918, 04d67fd, f1bdd8c
- Tests: cargo fmt --all -- --check, cargo clippy --workspace --all-targets -- -D warnings, cargo test --release --workspace, npm run wasm:build && npm test, npm run typecheck
- PRs: