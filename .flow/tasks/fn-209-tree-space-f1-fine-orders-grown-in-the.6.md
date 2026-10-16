---
satisfies: [R1]
---
# fn-209-tree-space-f1-fine-orders-grown-in-the.6 Realized mechanics: the fine stage equals today's engine

## Description
fn-211's host decision 3 (DESIGN-INTERLEAVE.md on branch fn-211-tree-space-f2b-growth-years-cheap, "After Astra's review"): production uses realized mechanics for every species; option 4's committed boundaries leave production.

For species that read no light (beech, spruce, palm), both executors:
- grow each fine root whole by the engine's own rules and shed it by its realized subtree, as the engine sheds;
- sum its girth, retained pipes, mass and moment from the deepest wave up;
- report one record of realized totals per root to the CPU (named under Astra change 3: one record per root, not per node);
- the CPU settles the structure above the cut exactly as today's whole-tree engine;
- then fine roots are laid from the settled bearers' frames, the bent pass using the first-lay frame (resolving FRAME-CORRECTION.md).

The target is today's engine: the whole tree assembled from the CPU structure plus the fine roots equals today's whole-tree grower (fixed-point contract included) by lineage. Light-reading species follow fn-211's stepped executor, not this task.

## Acceptance
- Beech and spruce at ages 40 and 80, seeds 1 and 7: topology identical by lineage to today's whole-tree engine, with bit-identical geometry on the CPU reference executor and geometry within 50 µm max, 10 µm RMS on the GPU.
- The same at cuts 256, 1,024 and 4,096, in any order or batch.
- The committed-boundary path removed from production code (kept only where a test or probe still needs it, named).
- Timing warm and cold against the CPU on 1 and all cores; round trips counted.
- Workspace gate green.


## Done summary
Realized mechanics on both executors (REALIZED.md, "The GPU mirror built").

- `telperion_space::settle` settles the structure above the cut through the four phases with any roots executor; `grow_fine` is that driver with the CPU reference and is today's engine to the bit. Host decision 3's ground bound is in the driver (`fine/landing.rs`).
- The GPU executor (`telperion-render` `generation/fine/device.rs`) grows every root whole with every axis kept, takes every presence a fade reads as the CPU's Q32 integer, carries living up and sheds down across waves (`fades.wgsl`), then scales, thickens, lays, gathers levers and bends between the CPU's settles (`settle.wgsl`); one record per root per phase crosses.
- Beech and spruce at 40 and 80, seeds 1 and 7, cuts 256, 1,024 and 4,096: topology identical by lineage; GPU tips at most 25.3 µm, RMS at most 1.44 µm; CPU-laid ground roots at most 44.5 µm; the same floats in two orders and batches; 14 to 40 round trips a tree. At 25 (beech, spruce, unlit oak) presences and living equal the CPU's to the bit, in three orders and batches.
- Timing (cut 1,024, warm): spruce 80 1.08 to 1.12 s on the GPU against 1.09 to 1.13 s on 32 cores and 8.5 to 8.6 s on one; the beech 2 to 7 times slower than 32 cores. Phase 1's growth is most of the GPU's time; proposals for the host in REALIZED.md.
- Option 4 removed: the GPU's committed kernels and tests, `grow_committed` and its support; P2 stays as a named probe.

## Evidence
- Commits: 837edc7c, 75b553ca, ab1d0dcb, 19d8ad2e, cae01463, 7baafe8c, aa747fdb, d00eb019, 8d6b2fac, 78c21bb7, 689ede1c, beabe892
- Tests: cargo test --profile ci --workspace --no-fail-fast, cargo test --profile ci -p telperion-render generation::fine (GPU lock), FINE_CUT={256,1024,4096} cargo test --profile ci -p telperion-render realized_at_40_and_80 -- --ignored --nocapture
- PRs: