# fn-259 task 3: validating the scorecard tooling

Measured 2026-10-19 on the fn-259 branch (base `1a0844c60`), RTX 3080, under `telperion-heavy 6 --gpu 3 --threads 4`, `frames` built with `--profile ci`. Raw rows: `raw/t3-ordinary40-*.json` (ignored).

## The probe on one small preset: the ordinary at 40, seed 1

`AGE=40 SEED=1 frames ordinary full whole mid close`, one run:

| Column | Value |
|---|--:|
| age, seed (asked) | 40, 1 |
| fresh: `Renderer::new` to a fenced first frame, ms | 2,585.5 |
| warm rebuild to a fenced frame, ms | 366.0 |
| frame ms, whole / mid / close | 0.555 / 0.733 / 0.881 |
| peak RSS, MB | 797 |
| device memory (wgpu allocator report), allocated / of it wgpu-internal / reserved, MiB | 291.8 / 196.5 / 1,024.0 |

## One-build mode against the multi-build run

`ONE_BUILD=1 AGE=40 SEED=1 frames ordinary`, one run: one build, four hero frames, timing keys null.

| | multi-build (above) | one-build |
|---|--:|--:|
| allocated MiB | 291.8 | 97.1 |
| of it wgpu-internal MiB | 196.5 | 6.5 |
| allocated less wgpu-internal MiB | 95.3 | 90.6 |
| reserved MiB | 1,024.0 | 768.0 |
| peak RSS MB | 797 | 691 |

What the gap is (measured with a temporary per-label dump, not committed, at the whole view only): every labelled buffer over 0.1 MiB had the same size in both modes; the difference was two wgpu-internal buffers, "Indirect draw validation destination buffer" and "... metadata buffer", 3.0 MiB each after one build and four frames and 60.0 MiB each after the rebuild and the timing loop's 120 pipelined frames. They grow with the frames the timing loop pipelines, so the probe now reports them as `wgpu_internal_mib`. With all three views drawn, the multi-build run's tree-and-target bytes are 4.7 MiB above the one-build run's. Peak RSS ranged 666-691 MB over two one-build runs and 748-797 MB over two multi-build runs of the same build. So one-build mode reproduces the multi-build run's memory only for the bytes the tree and its targets hold. It does not reproduce the reserved blocks, the wgpu-internal buffers or the peak RSS, which a second build and a timing loop raise. Which of these figures a memory-only row reports is task 4's protocol to state.

## Pixel helper

`npx vitest run scripts/pixel-diff.test.mjs`: green, 2 tests. With the count replaced by an intensity-weighted sum (`differing += most / 255`, reverted), the exact-count test is red: `expected [ 2048, 5.556862745098039, 12 ] to deeply equal [ 2048, 37, 12 ]`.

On two ImageMagick PNGs of 64×32 differing in 21 pixels (20 by 1/255, one by 155/255), the helper prints 21 differing and 1 over 8/255, and `compare -metric AE` prints 0.475. The helper reads 8-bit RGB and RGBA only. ImageMagick's default grey or palette PNGs are refused by name, so write `PNG32:` or `PNG24:`.

## Row runner

`scripts/gpu-row.mjs`, under the host's rule of 2026-10-19 ("no other session holding the GPU" means other sessions' jobs, not the owner's desktop clients). A process that appears on the card during the row, was not there at the start and is not in the probe's process tree is foreign. The row starts only when no other `telperion-heavy-*` unit runs and no process naming telperion is on the card. The processes present at the start are written into the row with their memory. The residual gap: a compute job outside Telperion already running at the start is recorded, not invalidating. The rows below had one, a GNO embedding worker (`bun ... gno/src/llm/native-worker`) holding 1,150 MiB, recorded in `gpu_start.apps`.

Each row below wraps `AGE=40 SEED=1 frames ordinary full whole mid close` under `telperion-heavy 6 --gpu 3 --threads 4 -- node scripts/gpu-row.mjs raw/t3-rows.jsonl -- ...`:

- **The lock holds for the whole row.** Row admitted 04:00:59. A second `telperion-heavy 1 -- date` was started at 04:01:03 and admitted at 04:01:07, the moment the row ended (`gpu-row: valid true, exit 0, 0 foreign`).
- **A foreign GPU job invalidates the row.** Row admitted 04:01:44. A short foreign job, `ONE_BUILD=1 AGE=40 nice -n 10 frames date-palm` outside `telperion-heavy`, ran 04:01:47-04:01:50. The row was written `valid: false` with one foreign process: that pid, 428 MiB, its command line. The card read 7,289 MiB free at the start and 7,305 at the end.
