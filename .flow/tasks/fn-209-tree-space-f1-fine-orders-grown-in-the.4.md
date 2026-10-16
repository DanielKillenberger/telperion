---
satisfies: [R1, R2]
---
# fn-209-tree-space-f1-fine-orders-grown-in-the.4 Fine geometry on the GPU executor

## Description
Task 2 built growth and shedding on the GPU (GPU-FINE.md); scale, girth and lay are not ported. Port them (about 1,800 lines of CPU code, in f32) so the executor ends at the botanical tree plus laid geometry in device buffers (host decision 5). Measure geometry against the CPU reference within fn-125's screen, and the whole fine stage's time.

## Acceptance
- Geometry of every fine root within 50 µm max, 10 µm RMS of the CPU reference, for beech and spruce at ages 40 and 80, seeds 1 and 7, in any item order and batch. - Topology still identical by lineage. - Whole fine stage (grow, shed, scale, girth, lay) timed warm and cold, worst item included, against the CPU executor on 1 and all cores. - A CPU-side WGSL parse (naga) in an ordinary test (FRICTION.md). - Workspace gate green.

## Done summary
The GPU executor now scales, girths and lays fine roots in f32. The chain grower takes each element's presence as it decides it, with leads computed from the integer thresholds' differences and ranks held exactly in Q52. A geometry kernel resolves each wave's chains from their packed regions and their bearers in the wave before, into per-wave laid buffers on the device (host decision 5). Positions are each root's own, measured from its bearer's node.

Results for beech and spruce at ages 40 and 80, seeds 1 and 7:
- Topology is identical by lineage.
- Tips are at most 34.2 um from the CPU reference, RMS at most 2.3 um. Radii are within 4.2e-6 relative.
- The laid floats are identical, by digest, in forward 65,536 and reversed 4,096 batches. The age-25 gate test also covers the oak with light off, at most 20.8 um.

Timing of the whole fine stage, warm (cold, worst item and CPU figures are in GPU-GEOMETRY.md):
- Spruce at 80 takes about 1.0 s on the GPU, against 1.0 s for the CPU on 32 cores and 14 s on one core.
- Geometry itself takes 5 to 100 ms. The rest is task 2's growth and its host-driven wave loop, which reads every wave back.

A naga parse and validation of every generation shader now runs in an ordinary test.

Open for the host: growth's per-wave readback. The options are a device-side wave expansion now, or leaving it to F4.

## Evidence
- Commits: 1aa3a5e8, e79d9ba9, 7a9de446
- Tests: cargo test --profile ci --workspace --no-fail-fast, FINE_GPU_LOCK=<lock> cargo test --profile ci -p telperion-render fine_geometry_at_40_and_80 -- --ignored --nocapture
- PRs: