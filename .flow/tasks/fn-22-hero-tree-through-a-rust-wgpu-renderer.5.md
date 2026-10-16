---
satisfies: [R1]
---
# fn-22-hero-tree-through-a-rust-wgpu-renderer.5 Browser target and panel wiring

## Description
Build the browser target and wire the panel.

## Acceptance
- wasm-bindgen renderer draws the harness canvas with every parameter live; errors arrive as the Rust error's words.

## Done summary
`web.rs` exports a `WebRenderer` (setTree, setView, heroCamera, setCamera, resize, frame, stats, timing, dispose); generation runs inside the module; `scripts/build-render.mjs` builds it and refuses with the exact wasm-bindgen install line when the CLI mismatches; the page is cut over.

## Evidence
- Commits: 458fe0157c3eda9df8c9083892d65f174ddab129
- Tests: cargo test --release --workspace; npm test; npm run typecheck; npm run build; npm run render:build
