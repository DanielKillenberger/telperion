# fn-158 results (interim, at 69549f15)

The typed stage inputs and the executor interface are in (commits 071169be, e392371c, 69549f15). The visibility step (stages private, tests moved, the compile-fail test) waits on the host decisions in the task's block reason.

## Byte identity (R3)

`crates/telperion-render/examples/generation_digest.rs` digests, per shipped preset at seeds 1 and 7: the pipeline's tree, element, plan, wood, leaves, field and structure for a full request; the field-only request; `mesh::build`; the GPU executor at CPU and resident delivery (mesh, instances, counts, bounds, metrics less timings); the growth path's mesh at age 6.

- Base `eca14aaf`: `digests-base.jsonl` (16 lines). Two base runs are identical.
- `e392371c` and `69549f15`: identical to the base on every line (`cmp`).
- Reproduce: `cargo build --release -p telperion-render --example generation_digest && target/release/examples/generation_digest | cmp - .flow/evidence/fn-158-stages-take-typed-inputs-behind-one/digests-base.jsonl`.

## Timings (R4)

`generation_gpu` (release, fat LTO), seed 1, on the RTX 3080 workstation. Base and head binaries alternate in every run with the order flipped each run, 5 runs of 3 samples, the cold sample dropped: 10 warm samples a cell. A second machine load (another checkout's test run, load average 7) was present during the first runs, which is why the runs interleave. Milliseconds are preparation plus delivery; "GPU, resident" is `Delivery::Resident` drawn once, "GPU, CPU delivery" is `Delivery::Cpu`, "CPU pipeline" is `mesh::build`.

| preset | executor | base median (min-max) ms | head median (min-max) ms | change | n |
|---|---|---|---|---|---|
| ordinary | CPU pipeline | 109.4 (106.2-113.3) | 108.5 (105.8-111.7) | -0.8% | 10/10 |
| ordinary | GPU, CPU delivery | 61.9 (58.6-78.5) | 60.3 (58.4-61.7) | -2.6% | 10/10 |
| ordinary | GPU, resident | 50.9 (48.0-53.0) | 51.0 (48.1-53.8) | +0.2% | 10/10 |
| oregon-white-oak | CPU pipeline | 358.8 (350.7-387.1) | 358.7 (347.6-375.6) | -0.0% | 10/10 |
| oregon-white-oak | GPU, CPU delivery | 119.0 (114.4-132.4) | 122.6 (117.6-184.9) | +3.0% | 10/10 |
| oregon-white-oak | GPU, resident | 119.7 (117.2-179.9) | 119.9 (114.6-142.6) | +0.1% | 10/10 |
| norway-spruce | CPU pipeline | 4760.5 (4657.6-4931.4) | 4773.0 (4656.8-4850.5) | +0.3% | 10/10 |
| norway-spruce | GPU, CPU delivery | 205.9 (199.2-227.2) | 206.8 (202.9-221.2) | +0.4% | 10/10 |
| norway-spruce | GPU, resident | 139.4 (133.4-243.2) | 136.5 (133.3-159.7) | -2.1% | 10/10 |
| european-beech | CPU pipeline | 2868.2 (2809.0-2992.5) | 2914.1 (2831.0-2947.8) | +1.6% | 10/10 |
| european-beech | GPU, CPU delivery | 2909.2 (2824.9-3014.1) | 2881.1 (2843.6-3029.5) | -1.0% | 10/10 |
| european-beech | GPU, resident | 3276.4 (3189.2-6413.4) | 3315.2 (3165.5-4453.5) | +1.2% | 10/10 |
| silver-birch | CPU pipeline | 2085.3 (2059.9-2158.1) | 2076.2 (2050.6-2132.4) | -0.4% | 10/10 |
| silver-birch | GPU, CPU delivery | 166.8 (164.3-178.6) | 170.2 (165.5-180.7) | +2.0% | 10/10 |
| silver-birch | GPU, resident | 163.3 (156.8-173.3) | 165.8 (155.1-187.0) | +1.5% | 10/10 |
| date-palm | CPU pipeline | 4.2 (4.1-4.4) | 4.2 (4.0-4.3) | -0.1% | 10/10 |
| date-palm | GPU, CPU delivery | 4.2 (4.1-4.3) | 4.2 (4.2-4.3) | -0.7% | 10/10 |
| date-palm | GPU, resident | 5.3 (5.0-7.1) | 5.3 (5.1-7.2) | +1.0% | 10/10 |
| telperion | CPU pipeline | 923.0 (903.0-1045.6) | 946.3 (911.0-1000.2) | +2.5% | 10/10 |
| telperion | GPU, CPU delivery | 440.4 (423.2-469.9) | 447.3 (424.5-487.8) | +1.6% | 10/10 |
| telperion | GPU, resident | 526.6 (504.6-570.6) | 515.6 (505.9-553.0) | -2.1% | 10/10 |
| laurelin | CPU pipeline | 384.4 (373.5-397.2) | 377.5 (374.5-415.1) | -1.8% | 10/10 |
| laurelin | GPU, CPU delivery | 130.5 (125.1-133.1) | 128.5 (124.3-135.1) | -1.6% | 10/10 |
| laurelin | GPU, resident | 167.7 (160.8-177.0) | 166.4 (160.0-175.3) | -0.7% | 10/10 |

Every head median lies inside the base's min-max spread and within 3.0% of the base median; no cell regresses beyond its spread. The european-beech runs fall back to the CPU on both binaries (station capability), which is why its GPU cells match its CPU cell.

## Artifact sizes

Built with `scripts/build-wasm.mjs` and `scripts/build-render.mjs`; the base in its own worktree and target.

| artifact | base `eca14aaf` | `e392371c` | `69549f15` | budget |
|---|---|---|---|---|
| dist/telperion-field.wasm | 359,585 | 359,933 | 359,316 (-269) | 362,000 |
| dist/telperion.wasm | 1,346,481 | 1,348,431 | 1,349,088 (+2,607) | 1,397,000 |
| dist/telperion-render.wasm | 1,925,440 | 1,929,759 | 1,929,393 (+3,953) | 1,996,000 |

`node scripts/artifact-budgets.mjs` passes at both commits; the scripts are unchanged. The slim module is 269 bytes below the base: the twig placement is read by reference and the slim build carries no surface, leaf or box input. The full and render modules grow by the executor interface and the owned stage inputs.
