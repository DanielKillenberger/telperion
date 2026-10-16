---
satisfies: [R1, R5, R6, R7]
---
# fn-22-hero-tree-through-a-rust-wgpu-renderer.6 Browser conformance, soak, evidence and docs

## Description
Browser conformance, soak, evidence and docs.

## Acceptance
- Hardware browser run covers presets, dial, views and software-fallback message; browser timing rows and five-minute soak recorded.

## Done summary
`npm run test:render` drives the page on hardware: every preset non-blank and distinct, the height dial regenerates, views change the draw, and `--disable-gpu` shows the fallback sentence. Browser timing rows match the native counts; the five-minute soak held one canvas and one device.

## Evidence
- Commits: 8863c88594b3adec5c7cef5125309920848fbf3c
- Tests: npm run test:render
