# fn-125 stage-3 baseline on master (2026-09-27)

Master c71ef3e1, native, `cargo build --profile ci -p telperion-render --example generation_gpu`, mode `gpu-render` (resident delivery, the harness path), RTX 3080, Linux. Six samples per fixture, first (cold) dropped, warm medians in ms. Raw: `raw/baseline-master-c71ef3e1-render.jsonl` (ignored).

| Fixture | Skeleton | Position prepare (CPU) | Descriptors (CPU stations) | Stage-3 CPU prep | Placement wait | Prepare total |
|---|---:|---:|---:|---:|---:|---:|
| Oak 1 | 53.3 | 40.1 | 21.6 | 61.7 | 1.2 | 126.1 |
| Oak 7 | 56.8 | 41.8 | 20.4 | 62.2 | 1.5 | 131.8 |
| Spruce 1 | 42.6 | 33.1 | 25.4 | 58.5 | 12.3 | 126.3 |
| Spruce 7 | 40.7 | 32.1 | 23.8 | 55.9 | 10.7 | 120.7 |

All four ran foliage and wood on the GPU with no fallback reason recorded.

## What this says about R4

On this successful GPU path the renderer compacts the wood once and prepares stations once (`preparation.rs`, the `supports_stations() && round_section()` branch). The duplicate work R4 counts on (up to four surface sweeps, stations prepared twice, masses computed and dropped) runs only on the fallback branches. The oak's and spruce's stage-3 time is the compaction itself (ring sampling, frames, packing, contact ranges) and station preparation, not repetition. fn-91's native attribution names one redundant pass inside compaction, the order-key sampling, at 5.6 to 6.9 ms on the oak and about 4 ms on the spruce.

Removing item G therefore cannot halve oak or spruce stage-3 preparation. Halving it needs the compaction or station work itself to move to the GPU or get cheaper.
