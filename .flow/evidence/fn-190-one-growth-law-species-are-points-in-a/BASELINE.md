# fn-190 baseline: today's trees at 8e0141dc

Every preset at seeds 1 and 7, measured once on 2026-10-03 between 18:08 and 18:13 (task .18). Later stages and R8/R9 read these numbers and stills; nobody re-runs `today` after production changes.

## Revision, profile, machine

- **Revision.** Built in worktree `.worktrees/fn-190-t18` at `3984bce2`. Its production tree equals origin/master `8e0141dc`: `git diff 8e0141dc 3984bce2` outside `.flow/` adds only the six scratch files of the probe, `crates/telperion-render/examples/growth_law/*.rs`, and changes no file under `crates/*/src`, `crates/telperion-core/presets` or `src`.
- **Profile.** `release` (`lto = true`, `codegen-units = 1`), rustc 1.98.1, this worktree's own `target/`. `growth_law` binary sha256 prefix `6ba8f048aa733e82`.
- **Machine.** AMD Ryzen 9 5950X (16 cores, 32 threads), 31 GiB RAM, RTX 3080 (driver 610.57.04), Linux 7.2.3-arch1-3.
- **Load.** Other sessions were running (Claude sessions, Chrome). The 1-minute load average stayed between 2.0 and 3.1 on 32 threads for the whole run, with about 12 GiB of memory available. Each run's load before and after is in `raw/baseline/load/`. Runs were serial, one process per preset and seed.

## Presets

The eight presets `Preset::from_id` serves at `8e0141dc`: european-beech (in work), oregon-white-oak, norway-spruce, silver-birch, date-palm, telperion, laurelin, ordinary. **The ash and the plane have no preset at this revision**, so there is no baseline for them. The ash has `catalogue/european-ash/` but no value file in `crates/telperion-core/presets/`, and the plane has neither. R8's comparison for these two can only be against their references.

## Measures

- **nodes:** skeleton node count of the direct build. The `today` output and the cost tool agree on every row.
- **fine wood:** the probe's `tree::fine` after `tree::reclass`: wood length on nodes under 0.05 of the root radius, in km. **wood** is all wood length. The palm's stem is structural throughout, so its fine wood is 0.
- **leaves:** leaves the direct build keeps (`leaves.retained`, after the cull). **placed** is the count before the cull.
- **skeleton ms:** the skeleton stage `pipeline::build(Request::default())` reports, warm median of 21 runs after one cold run. The probe's `today` command times the same stage with only 3 warm runs. Its figure is listed beside it, and it is noisier: beech seed 1 gave 80.5 there and 57.3 over 21 runs.
- **full build ms:** `stages.total_ms` of `pipeline::build(Request::mesh())`, which is what `mesh::build` runs (skeleton, rings and wood, placement, cull), warm median of 5 runs after one cold run.
- **peak RSS:** the process's `VmHWM` after one cold and five warm full builds, each build dropped before the next. It includes the process (about 2 MiB before the first build) and whatever the allocator retains.

## Numbers

| preset | seed | nodes | fine wood km | leaves | skeleton ms | full build ms | peak RSS MiB |
|---|---|--:|--:|--:|--:|--:|--:|
| european-beech | 1 | 213,044 | 46.66 | 5,828,744 | 57.3 | 3471 | 727 |
| european-beech | 7 | 215,470 | 47.15 | 5,854,731 | 57.8 | 3474 | 733 |
| oregon-white-oak | 1 | 133,635 | 31.22 | 767,026 | 29.8 | 360 | 251 |
| oregon-white-oak | 7 | 143,703 | 35.63 | 903,084 | 31.7 | 419 | 288 |
| norway-spruce | 1 | 96,174 | 18.79 | 7,414,510 | 33.9 | 4822 | 301 |
| norway-spruce | 7 | 91,077 | 17.91 | 7,064,601 | 32.1 | 4587 | 287 |
| silver-birch | 1 | 86,326 | 35.11 | 258,111 | 21.7 | 2751 | 163 |
| silver-birch | 7 | 91,743 | 37.45 | 279,485 | 23.9 | 2727 | 173 |
| date-palm | 1 | 552 | 0.00 | 6,554 | 1.6 | 4 | 4 |
| date-palm | 7 | 553 | 0.00 | 6,554 | 1.7 | 4 | 4 |
| telperion | 1 | 77,324 | 36.76 | 550,030 | 352.7 | 898 | 202 |
| telperion | 7 | 18,140 | 8.76 | 128,852 | 50.2 | 154 | 45 |
| laurelin | 1 | 53,861 | 36.28 | 392,944 | 95.9 | 417 | 77 |
| laurelin | 7 | 75,239 | 51.71 | 543,814 | 171.9 | 718 | 115 |
| ordinary | 1 | 11,568 | 2.99 | 40,659 | 40.1 | 107 | 30 |
| ordinary | 7 | 11,934 | 3.05 | 41,877 | 43.3 | 113 | 32 |

Supporting figures: wood length, leaves before the cull, the `today` 3-run skeleton figure, cold times, and the full build's warm stage split (surface is rings plus wood).

| preset | seed | wood km | leaves placed | skeleton ms, today (3) | skeleton cold | full cold | surface | placement | cull |
|---|---|--:|--:|--:|--:|--:|--:|--:|--:|
| european-beech | 1 | 50.90 | 5,828,744 | 80.5 | 68.2 | 3483 | 95 | 2598 | 821 |
| european-beech | 7 | 51.45 | 5,854,731 | 55.8 | 65.4 | 3486 | 92 | 2581 | 812 |
| oregon-white-oak | 1 | 31.64 | 767,026 | 35.8 | 40.1 | 370 | 55 | 219 | 107 |
| oregon-white-oak | 7 | 36.14 | 903,084 | 38.8 | 44.7 | 429 | 56 | 260 | 124 |
| norway-spruce | 1 | 18.87 | 7,414,510 | 33.8 | 44.2 | 4758 | 45 | 4283 | 489 |
| norway-spruce | 7 | 18.00 | 7,064,601 | 30.0 | 40.4 | 4556 | 40 | 4066 | 464 |
| silver-birch | 1 | 35.41 | 306,722 | 25.8 | 33.5 | 2749 | 34 | 91 | 2638 |
| silver-birch | 7 | 37.76 | 327,463 | 27.3 | 32.7 | 2768 | 33 | 96 | 2605 |
| date-palm | 1 | 0.21 | 6,554 | 1.6 | 1.8 | 5 | 1 | 2 | 1 |
| date-palm | 7 | 0.21 | 6,554 | 1.7 | 1.8 | 4 | 1 | 2 | 1 |
| telperion | 1 | 38.83 | 569,200 | 341.5 | 371.8 | 917 | 36 | 158 | 391 |
| telperion | 7 | 10.39 | 131,900 | 50.8 | 53.6 | 152 | 35 | 37 | 65 |
| laurelin | 1 | 45.89 | 398,575 | 96.8 | 101.6 | 628 | 25 | 122 | 181 |
| laurelin | 7 | 61.38 | 557,950 | 186.7 | 175.8 | 763 | 27 | 156 | 385 |
| ordinary | 1 | 3.27 | 43,550 | 43.7 | 45.3 | 111 | 13 | 13 | 52 |
| ordinary | 7 | 3.35 | 44,824 | 44.0 | 45.8 | 120 | 18 | 13 | 57 |

Every build reported a complete tree (`diagnostics.complete()`). Structure matches the probe rounds' earlier `today` files for beech, oak and spruce: nodes and fine wood are identical, and only the timings differ.

## Stills

There are 32 stills, at `raw/baseline/stills/<preset>-<seed>-{bare,whole}.png` (960×720). `today` renders the direct build's mesh (`mesh::build`). Each preset also has a 2×2 contact sheet at `raw/baseline/sheets/<preset>.png`: seed 1 on top, seed 7 below, bare on the left and whole on the right. The beech uses the probe rounds' fixed camera, `raw/baseline/pose-beech.json` (sha256 `4817f88b…`), the same pose those rounds' beech stills used. Every other preset uses `hero_pose` fitted to the skeleton's bounds.

The worker viewed all 32 stills through the eight contact sheets. It makes no judgment of them; the host has not viewed them yet (acceptance item 3). What the stills show:

- **Date palm, whole:** the camera fits the skeleton, which is the stem alone, so the crown of fronds runs past the top edge of the frame at both seeds. The bare stills show the plaited stem in full. These stills therefore do not show today's whole palm. Task .4 owns today's palm baseline and needs a camera fitted to the mesh.
- **Telperion and Laurelin** are about 140 to 150 m tall, so the leaves barely show at this framing.
- **Norway spruce, bare:** a few long, low limbs reach out of the cone toward the lower left at both seeds.

## How to reproduce (not to be re-run after production changes)

- `raw/baseline/run.sh` runs `growth_law today <preset> <seed> today/<p>-<s>.json stills/<p>-<s>` from `raw/baseline/cwd/`, which holds the beech pose at the path `today` reads. It then runs `tool/target/release/baseline-cost <p> <s> 5` into `cost/`.
- `baseline-cost <p> <s> 21 skeleton` writes `skel/all.jsonl`.
- `raw/baseline/table.py` prints the rows.
- `baseline-cost` is a scratch standalone crate in `raw/baseline/tool/` (gitignored). It has its own `[workspace]` and target, the workspace's `Cargo.lock` copied in, the same release profile, and a path dependency on this worktree's `telperion-core`. `today` reports neither full-build time, leaf count nor peak memory, and this task's scope excluded editing the probe.

## Friction (for FRICTION.md; this task may touch only this file and raw/baseline/)

- **2026-10-03, task .18, measuring today's full build.** `growth_law today` has no seed-aware full-build timing, leaf count or peak-memory output, and `examples/measure.rs` has no seed argument. Writing and building a scratch crate cost about 10 minutes. A `today --cost` mode in the probe, or a seed argument on `measure`, would remove it.
- **2026-10-03, task .18, the `today` skeleton figure.** Its warm median comes from 3 runs and is noisy under shared load (beech seed 1: 80.5 ms against 57.3 ms over 21 runs). Spotting and re-measuring it cost about 3 minutes. Taking the median of at least 11 runs in `today` would remove it.
- **2026-10-03, task .18, shell guard.** The dcg hook blocks `>` redirects to paths held in shell variables, so two commands were rewritten with literal paths. This cost about 2 minutes.
