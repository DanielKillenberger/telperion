# fn-158 results

Base `d8368b0b` (master with fn-152, #122). The branch was rebased onto it after fn-152 merged; every number below is taken against it.

## Byte identity (R3)

`crates/telperion-render/examples/generation_digest.rs` digests, per shipped preset at seeds 1 and 7: the pipeline's tree, element, plan, wood, leaves, field and structure for a full request; the field-only request; `mesh::build`; the GPU executor at CPU and resident delivery (mesh, instances, counts, bounds, metrics less timings); the growth path's mesh at age 6.

- `digests-base.jsonl` (16 lines) is the base's output; `eca14aaf` (the pre-rebase base) prints the same bytes, and two base runs are identical.
- Head: identical to the base on every line (`cmp`), before and after the rebase.
- Reproduce: `cargo build --release -p telperion-render --example generation_digest && target/release/examples/generation_digest | cmp - .flow/evidence/fn-158-stages-take-typed-inputs-behind-one/digests-base.jsonl`.

Parameterised cases: `pipeline::executor::tests` builds four families (leaves free and seated on the wood; `maxTurnPerStep` stated and unset) and holds the expansion (grown and handed back) to the build's tree, wood, element, leaves and box, and each output asked for alone (wood, leaves, field, structure) to the whole request's.

## The eight core examples (host decision 3)

Each ran on the base and the head with the same inputs: `curtain_audit`; `field_still` for the date palm and the oak (now the ignored suite test `field_still`); `generation_limits`; `geometry_benchmark --worker`, `--support` and `--vocabulary` for all eight presets; `measure` and `measure --field` for all eight; `node_buffer` direct for all eight and grown to age 6 for the ordinary tree and the oak; `occupancy_audit`; `species_measure` for five profiled cases and the date palm (fn80 profile). Outputs were compared with timings, load and timestamps removed.

- Every output of every preset but the date palm is unchanged, including both field stills (byte-identical PPMs), every occupancy audit and every node buffer.
- The date palm changes wherever the example grew its own skeleton, because the pipeline's skeleton hangs the palm's 256 shed leaf bases (clearing the apical twigs is a no-op for it: its tree had no twig wood): `geometry_benchmark`, `measure`, `measure --field`, `node_buffer` and `species_measure`.
- `measure`'s output schema drops `boundsMs` (the pipeline times the leaves' bounds inside the cull) and gains `cullMs`.

`species_measure`, date palm at seed 1, profile `date-palm` (`.flow/evidence/fn80/profiles-date-palm.json`); numeric status `pass` before and after:

| field | before | after |
|---|---|---|
| nodes | 40 | 552 |
| branch_count | 0 | 256 (order 1) |
| stored_branch_runs | 0 | 256 |
| wood_vertices | 822 | 16,674 |
| wood_triangles | 1,640 | 32,320 |
| wood_height_m | 19.451 | 19.706 |
| height, crown, dbh, leaves, leaf area | unchanged | unchanged |

The species runner's pins (`telperion-jev/src/runner/pins.rs`) hashed the raw scaffold solve and now hash the pipeline's skeleton: the palm's recorded skeleton pin moves from 17313072172866236736 to 10058597765394239704 (`catalogue/date-palm/pins.json` and its README); every other pin is unchanged.

## Timings (R4)

`generation_gpu` (release, fat LTO), seed 1, on the RTX 3080 workstation. Base and head binaries alternate in every run with the order flipped each run, 5 runs of 3 samples, the cold sample dropped: 10 warm samples a cell. Milliseconds are preparation plus delivery; "GPU, resident" is `Delivery::Resident` drawn once, "GPU, CPU delivery" is `Delivery::Cpu`, "CPU pipeline" is `mesh::build`.

| preset | executor | base median (min-max) ms | head median (min-max) ms | change | n |
|---|---|---|---|---|---|
| ordinary | CPU pipeline | 109.8 (106.0-113.7) | 110.4 (106.4-115.8) | +0.5% | 10/10 |
| ordinary | GPU, CPU delivery | 60.4 (58.0-62.6) | 60.9 (58.3-63.4) | +0.8% | 10/10 |
| ordinary | GPU, resident | 51.7 (48.5-55.9) | 50.8 (48.0-52.8) | -1.8% | 10/10 |
| oregon-white-oak | CPU pipeline | 355.8 (349.4-365.0) | 350.9 (345.7-363.5) | -1.4% | 10/10 |
| oregon-white-oak | GPU, CPU delivery | 116.2 (114.5-130.5) | 118.4 (116.2-129.4) | +1.9% | 10/10 |
| oregon-white-oak | GPU, resident | 119.4 (114.9-130.6) | 120.7 (117.5-138.5) | +1.1% | 10/10 |
| norway-spruce | CPU pipeline | 4688.1 (4606.2-4774.2) | 4621.0 (4569.8-4710.1) | -1.4% | 10/10 |
| norway-spruce | GPU, CPU delivery | 209.5 (201.8-215.9) | 205.4 (199.1-212.0) | -1.9% | 10/10 |
| norway-spruce | GPU, resident | 143.5 (135.2-164.0) | 137.2 (132.0-147.0) | -4.4% | 10/10 |
| european-beech | CPU pipeline | 2872.8 (2806.0-3056.6) | 2839.7 (2795.5-2944.9) | -1.2% | 10/10 |
| european-beech | GPU, CPU delivery | 2857.4 (2832.1-3014.4) | 2872.3 (2816.8-3051.3) | +0.5% | 10/10 |
| european-beech | GPU, resident | 3285.0 (3165.3-3416.1) | 3294.1 (3194.7-3404.3) | +0.3% | 10/10 |
| silver-birch | CPU pipeline | 2076.7 (2052.7-2171.2) | 2095.5 (2080.0-2142.9) | +0.9% | 10/10 |
| silver-birch | GPU, CPU delivery | 171.3 (166.1-182.6) | 168.3 (164.1-171.0) | -1.8% | 10/10 |
| silver-birch | GPU, resident | 164.0 (158.2-170.8) | 163.6 (158.2-172.5) | -0.2% | 10/10 |
| date-palm | CPU pipeline | 4.2 (4.1-4.5) | 4.2 (4.1-4.5) | -1.2% | 10/10 |
| date-palm | GPU, CPU delivery | 4.2 (4.2-4.3) | 4.2 (4.1-4.2) | -1.0% | 10/10 |
| date-palm | GPU, resident | 5.2 (5.1-5.3) | 5.2 (5.1-5.5) | +0.0% | 10/10 |
| telperion | CPU pipeline | 920.2 (900.3-958.5) | 919.9 (898.6-937.6) | -0.0% | 10/10 |
| telperion | GPU, CPU delivery | 432.6 (417.3-450.2) | 440.0 (417.3-459.9) | +1.7% | 10/10 |
| telperion | GPU, resident | 530.0 (507.0-804.1) | 524.8 (503.8-1488.7) | -1.0% | 10/10 |
| laurelin | CPU pipeline | 382.2 (374.0-421.9) | 385.7 (371.4-486.1) | +0.9% | 10/10 |
| laurelin | GPU, CPU delivery | 129.3 (123.4-158.1) | 133.1 (125.2-141.0) | +2.9% | 10/10 |
| laurelin | GPU, resident | 166.1 (159.4-216.2) | 167.6 (160.3-276.1) | +0.9% | 10/10 |

Every head median lies inside the base's min-max spread; the largest moves are +2.9% (laurelin, GPU CPU delivery) and -4.4% (spruce, resident). No cell regresses beyond its spread. The european-beech falls back to the CPU on both binaries (station capability), which is why its GPU cells match its CPU cell.

## Artifact sizes

`npm run build` on the head and `scripts/build-wasm.mjs` plus `scripts/build-render.mjs` on the base, each in its own target; `node scripts/artifact-budgets.mjs` passes.

| artifact | base | head | change | budget |
|---|---|---|---|---|
| dist/telperion-field.wasm | 359,585 | 359,177 | -408 | 362,000 |
| dist/telperion.wasm | 1,370,440 | 1,380,610 | +10,170 | 1,397,000 |
| dist/telperion-render.wasm | 1,946,986 | 1,959,990 | +13,004 | 1,996,000 |

The slim field module shrinks: its build carries no surface, leaf or box input and reads the twig placement by reference. The two unstripped modules grow mostly by their function-name sections, where every stage's symbol gains `pipeline::` (+6,783 and +8,643 bytes by `twiggy diff`); their code grows by about 3.4 KB and 4.4 KB, the owned stage inputs and the executor interface.

## Gate (R5)

- `cargo test --profile ci --workspace --no-fail-fast` at `19c3b5a0`: 1,042 passed, 4 failed, 22 ignored. The four were the species suite's process ceiling (`suite::species::fixed_{oaks,spruces,beeches,birches}_...`): the whole core suite shared one test process under `cargo test`, and the ceiling read that process's high-water mark. The review's fix runs each of those four tests in a child process of its own (one at a time), as the saturation test already did; the rerun of the gate is in the done summary.
- `npm test`: 12 files, 130 tests passed.
