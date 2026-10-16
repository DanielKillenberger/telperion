# fn-209 F1 task 4: fine geometry on the GPU executor (worker, 2026-10-06)

Task 2 grew and shed fine roots on the GPU (GPU-FINE.md). This task ports scale, girth and lay (sag included) to WGSL in f32, ending at the botanical tree plus laid geometry in device buffers (host decision 5).

## Answer in brief

- **Built.** Presences are taken in f32 by the chain grower as it decides each draw (`presence.wgsl`). A geometry kernel (`geometry.wgsl`, `resolve.wgsl`, `walk.wgsl`, `lay.wgsl`; host `laid.rs`) resolves each wave's chains from their packed regions and their bearers in the wave before:
  - scale (`geometry::scale`);
  - girth by the pipe model over the chain's own wood and its borne buds' committed sections, fitted to its committed base;
  - the lay before bending;
  - levers from its own wood and its borne buds' committed mass and moment;
  - the lay under them.
  Each wave writes one laid buffer on the device. Positions are each root's own, metres from its bearer's node, so rounding scales with the root's reach and not with the tree's height.
- **Topology is identical by lineage** in every tree compared (`laid_tests.rs`, the same assertions as task 2).
- **Geometry is inside fn-125's screen** for the beech and spruce at 40 and 80, seeds 1 and 7:
  - tips at most **34.2 µm** (beech 80, seed 7), RMS at most **2.3 µm**;
  - radii within 4.2e-6 relative.
  - The age-25 gate test adds the oak with light off: at most 20.8 µm.
- **Order and batch.** The laid floats are identical, compared by digest, in forward batches of 65,536 and reversed batches of 4,096 (at 40 and 80). At 25 the comparison is forward 4,096 against reversed 777.
- **Timing.** On the GPU, the whole fine stage (grow, shed, scale, girth, lay) is about as fast as the CPU executor on all 32 cores at spruce 80 (about 1.0 s each), and slower for the beech. Geometry itself costs 5 to 100 ms. The rest is task 2's growth and its host-driven wave loop, which reads every wave back. See "Timing".
- **naga.** Every generation shader is parsed and validated on the CPU in an ordinary test (`generation/shader_tests.rs`), and every `.wgsl` under `src/generation` must sit in some checked source. It caught two reserved words before any locked GPU run.

## Two arithmetic choices that the screen needed

The first port met the screen at 25 only by a hair (oak 49.9 µm, spruce 40 µm). Two changes brought the error down by an order of magnitude. Neither changes a decision.

1. **Leads from the integer thresholds.**
   - The first port computed a presence's lead as `logit(bound) - logit(u)` in f32. Each term is about 4.6 with an ulp of 5e-7, and a partial presence divides by a window as small as 0.05. That cost 1.5e-4 of a scale (spruce 25, root 273).
   - Now `below(u, p) = ln1p((T - d) / d) + ln1p((T - d) / (2^53 - T))`, with `T` the decision's own integer threshold and `d` the draw. This has no cancellation near the bound, which is the only place a presence is partial.
   - Spruce tips fell from 40 µm to 1.7 µm at 25.
2. **Exact phyllotactic rank.** A rank (node presence × unit share, summed) is carried in Q52 integers, and its turn is taken as `frac(rank × divergence / τ)` in a 128-bit product. An f32 rank of about 100 times a divergence of 2.4 rad loses about 2e-5 rad of azimuth.

## Geometry and timing (`raw/gpu-geometry/run2.txt`; run1 agrees)

`FINE_GPU_LOCK=<lock> cargo test --profile ci -p telperion-render fine_geometry_at_40_and_80 -- --ignored --nocapture`. The GPU lock is taken inside the test, around the device's work only. Load 1.4 to 3.0. Ryzen 9 5950X (32 threads), RTX 3080.

- GPU columns are wall time from table upload to the last wave laid. Grow, read and lay in brackets are task 2's growth to device finish, task 2's packing and readback, and this task's geometry to device finish.
- The gap to the wall is the test host's wave loop, which assembles topology from the readback.
- "Worst item" is the root slowest on one CPU core, expanded alone on the GPU.
- The CPU columns are `grow_fine`'s items, summed on 1 core and wall on all cores (`Fine::expand_seconds`). Pipelines build in 20 ms.

| Tree | Roots | Chains | Phytomers | Tips most / RMS µm | GPU cold ms (grow / read / lay) | GPU warm ms (grow / read / lay) | Worst item: phytomers, CPU 1 core ms, GPU ms | CPU Σ items 1 core ms | CPU all cores ms |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| beech 40 1 | 553 | 12,405 | 81,297 | 8.1 / 1.57 | 46 (22 / 15 / 5) | 36 (21 / 6 / 5) | 1,160, 1.6, 8.7 | 98 | 24 |
| beech 40 7 | 975 | 22,258 | 141,265 | 26.5 / 2.16 | 49 (29 / 9 / 6) | 49 (29 / 9 / 6) | 2,216, 2.3, 21.3 | 158 | 22 |
| beech 80 1 | 7,865 | 126,350 | 847,545 | 23.0 / 2.29 | 168 (62 / 48 / 13) | 185 (68 / 47 / 14) | 2,098, 3.3, 17.1 | 1,249 | 107 |
| beech 80 7 | 12,829 | 221,386 | 1,485,934 | 34.2 / 2.27 | 470 (230 / 104 / 39) | 304 (95 / 89 / 22) | 1,871, 3.0, 13.8 | 2,121 | 191 |
| spruce 40 1 | 21,307 | 305,626 | 1,171,257 | 2.3 / 0.17 | 566 (232 / 152 / 52) | 300 (77 / 81 / 17) | 962, 2.7, 6.4 | 2,956 | 232 |
| spruce 40 7 | 20,444 | 294,003 | 1,135,099 | 4.5 / 0.17 | 543 (219 / 152 / 46) | 299 (76 / 87 / 16) | 1,197, 3.3, 7.2 | 2,880 | 219 |
| spruce 80 1 | 92,703 | 958,770 | 4,731,672 | 8.9 / 0.35 | 1,191 (312 / 436 / 84) | 1,096 (195 / 390 / 100) | 1,153, 4.0, 11.9 | 15,134 | 1,063 |
| spruce 80 7 | 91,262 | 951,424 | 4,672,415 | 17.1 / 0.35 | 1,176 (367 / 361 / 77) | 987 (183 / 365 / 76) | 768, 3.4, 11.3 | 13,836 | 1,022 |

What the table says (the worker's readings):

- **Geometry is cheap:** 5 to 100 ms, 2 to 9 % of the GPU wall.
- **The GPU stage is bound by task 2's host-driven wave loop.**
  - The readback (36 % of spruce 80's wall) exists because the host builds each next wave's buds from the read-back words.
  - The host's topology assembly accounts for most of the remaining gap.
  - Without both, spruce 80 would be about 0.3 s warm against the CPU's 1.0 s on 32 cores, and 14 s on one.
- **The worst item's latency** is 6 to 21 ms on the GPU against 2 to 4 ms on one CPU core. It is serial waves, each a dispatch and a readback.
- **Radii** within 4.2e-6 relative and 1 µm absolute everywhere. The largest relative errors at 25 sit on sub-micron twig tips.

## Not built, and open for the host

- **Growth still reads every wave back.** Geometry reads only device buffers and writes laid buffers; nothing per fine node is read back except by the tests. Task 2's wave loop, though, still reads each wave's status and packed regions to the host to build the next wave's buds and the per-chain geometry items. Options:
  - **(a)** expand the next wave on the device: compact the borne buds with a prefix sum and dispatch indirectly, with the overflow re-run kept as a device-side queue;
  - **(b)** keep the host loop until F4, which owns the integration (decision 4).
  Either is a design choice for the host. This task did not take it.
- **The ground refusal (D19) is a status flag on the GPU** (bit 2 of each chain's status), not an error. Host decision 3 routes a root that can reach the ground to the CPU, and that routing is not built. No compared tree raised the flag. The landing, resting and `held` terms are ported.
- **Light-reading species stay refused** (the oak lit; fn-211 task 4). Unit size and leaf light are taken as whole, which holds for every species the fine path accepts.
- **The bent lay's frame correction is the CPU's.** In the CPU reference the chain's phytomers are unlaid before its bent pass, so `framed` leaves a lever as it is. The GPU does the same: its first-lay frame is passed as none. This is noted because it reads like an accident of `Bounded::settled` rather than a decision.
