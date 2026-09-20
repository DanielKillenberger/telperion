# Resident wood visual review — 2026-09-20

Exactly four new browser captures compare default CPU and GPU foliage plus resident GPU wood for oak and spruce seed 1. Matching cameras target the lower trunk, with the existing lighting and 1280×720 viewport. `wood-detail.json` records the Wasm hash, camera, device flags and actual GPU wood backend. The renderer queue completed before each screenshot. These images do not change the previously accepted foliage captures.

The host inspected all four images and sees no apparent visual regression. Shaded trunks and dense spruce foliage limit inspection; these are not bright-light, motion, seed-7 or phone qualification. All-field numerical comparisons separately establish exact wood geometry and attributes except normals, whose maximum measured angular difference is below 0.001 radians.

Decoded RGBA comparison (`pixel-diff.json`) finds 921 changed pixels for oak (0.099935%) and 13,222 for spruce (1.434679%) out of 921,600 each. Maximum channel differences are 76 and 39 respectively. These complete CPU/GPU images include the already known foliage differences, so pixel changes cannot be attributed solely to wood normals.

The host opened the side-by-side gallery at http://127.0.0.1:8768/wood.html and requested the owner's scoped verdict. **Owner verdict pending.** Earlier acceptance of foliage images is not counted as acceptance of this new wood calculation.
