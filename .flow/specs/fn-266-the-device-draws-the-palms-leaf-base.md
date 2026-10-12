# The device draws the palm's leaf-base lattice

## Goal & Context

fn-256 task 3 moves every resident build onto the device's one cycle (host decision B, 2026-10-12), and the device has to draw the date palm, a shipped species. Its rosette fits the stations stage and is built in fn-256 task 3; its trunk does not (fn-256 DESIGN.md 12.7). On the CPU the palm's leaf-base boots (fn-110, fn-144) are wood added in expansion's first step (`leaf_bases.rs`, `lattice.rs`): two nodes per boot at exact heights along the stem on the crown's spiral, with a lattice cell for its shape, which the sweep reshapes onto its cell (`section::reshape`) and the handoff passes to the renderer as cells. Without them the palm would draw with a bare trunk, a visible loss against F4. One generator gap per spec (AGENTS.md).

## Requirements

- **R1 (design first, in fn-256's DESIGN.md as 12.8):** a device stage between the dress and the folds that walks each stem and adds two records per boot plus the cell table, by degree (zero boots, no change); the folds and sweep taking the extra records; the sweep's reshape onto each cell with squared frames; the handoff passing the device's cells as a buffer. Direct dispatches, at most 16 storage buffers a stage, Tint's rules (COUNCIL.md).
- **R2:** the palm builds on the device and draws in the harness; CPU and device agree by lineage on boots (heights, spiral positions, cells) within the lineage tolerance; the refusal by leaf capability is removed.
- **R3:** palm stills whole, mid and close beside F4's, viewed, the diamond lattice visible; build time; gate and npm test green; codex review.

## Boundaries

The palm's boots and cells on the device. fn-256 depends on it; both land with fn-255 on fn-256's branch line. Release item.
