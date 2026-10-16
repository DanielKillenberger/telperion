---
satisfies: [R9]
---
# fn-14-bark-and-foliage-appearance.3 The sun and one shadow map, timed

## Description
Add one sun with a single shadow map, timed on its own.

## Acceptance
- Self-shadow on wood and leaves and a ground shadow; leaves at their coarsest level in the pass; own timestamp pair.

## Done summary
The sun writes one single-sample depth map over the wood and every leaf at the coarsest level in one instanced draw; wood, leaf and room sample it. The pass has its own timestamp pair.

## Evidence
- Commits: 426b31d191ba837802cbf25e5a8438ff2905f0dc
- Tests: cargo test --release --workspace; cargo check -p telperion-render --target wasm32-unknown-unknown
