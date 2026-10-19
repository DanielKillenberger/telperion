---
satisfies: [R8]
---
# fn-22-hero-tree-through-a-rust-wgpu-renderer.7 Retire the Three.js stage and move the rigs to the Rust renderer

## Description
Retire the Three.js stage and move the rigs to the Rust renderer.

## Acceptance
- No `three` import remains; species QA and the binding contract run on the Rust renderer; the fn-19 rig is recorded as retired.

## Done summary
Clay stage, tree builder, adapter and `three` dependencies deleted. Species QA renders stills through the headless example (48/48 numeric and 48/48 stills pass); the binding contract runs under `npm run rust:test:wasm`.

## Evidence
- Commits: 2fd76c2fc867f55b156da6ace0d8611a1cc2d25c
- Tests: npm run species:qa; npm run rust:test:wasm
