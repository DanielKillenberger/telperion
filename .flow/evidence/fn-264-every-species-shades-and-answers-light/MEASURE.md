# fn-264 R3 so far: what R1a and the beech's leaf area cost (worker, 2026-10-12)

What has shipped: the one site light (k 0.5) for every preset, and the beech's leaf area, which the Two Trees carry. The spruce's, birch's and palm's leaf area and every species' response rows wait on the host (SOURCES.md), so this is the price of the lattice switched on, not yet of the species answering it.

**Base** is `c1aef3f38` (fn-256 task 12's tip), extracted by `git archive` and built in its own target. **Tip** is `c64cc0927` (R1a and the beech; later commits are evidence only). Both were built and measured in this session on the RTX 3080, with 2.0 GB held by desktop processes, 4 threads, one heavy process at a time, and GPU work under the lock. Raw output and scripts are in `raw/` (ignored).

## Trees: unchanged but for the ordinary

- **Stills** (`resident_stills`, every preset at its default age and seed, whole, mid and close): 21 of 24 are pixel-identical to the base (`compare -metric AE`: 0). The ordinary differs (AE 4,331 whole, 17,293 mid, 19,754 close; 13,792 leaves to 13,783). Its extinction moved from 1/6 to 0.5. The sheet was viewed (`raw/r3/stills/ordinary-sheet.png`): the same tree, a few fine shoots of the lower crown placed otherwise, with no perceptible regression.
- **Test:** `leaf_area_alone_changes_no_growth` grows the beech, the Two Trees, the spruce, the birch and the palm under the site light, with their leaf area, to the tree they grew before (no extinction, no leaf area).

## CPU reference (`early three`, default age, seed 42)

Every preset grows what it grew, in the same time and peak memory within run noise: Telperion 18,239,310 grown, 10.02 → 10.05 s, 2,243 → 2,233 MB; Laurelin 9.30 → 9.29 s; the beech 0.34 s both. The ordinary grows 389,743 → 389,443 in 0.35 s both.

## Device (`build_time`, warm build ms, the mean of two alternating rounds; device peak MB)

| Preset | Seed 42 | Seed 1 | Peak MB (seed 42) |
|---|---|---|---|
| ordinary | 458 → 456 (0%) | 438 → 446 (+2%) | 257 → 257 |
| oak | 256 → 254 (−1%) | 251 → 255 (+2%) | 1,128 → 1,128 |
| spruce | 432 → 433 (0%) | 443 → 446 (+1%) | 1,975 → 1,975 |
| beech | 178 → 190 (+6%) | 158 → 172 (+9%) | 269 → 282 |
| birch | 224 → 217 (−3%) | 201 → 200 (0%) | 650 → 650 |
| date palm | 74 → 73 (−1%) | 74 → 74 (0%) | 31 → 31 |
| **Laurelin 80** | **1,104 → 1,772 (+61%)** | **810 → 1,324 (+63%)** | 3,022 → 3,209 |
| **Telperion 82** | **1,198 → 1,949 (+63%)** | **876 → 1,444 (+65%)** | 3,301 → 3,503 |

- **Where it goes** (`the_roofline_is_measured`, stamped GPU ms, seed 42): Telperion's light phase is 14.8 → 755.5 ms (405 → 2,430 dispatches): `path_main` +304, `sweep_main` +256, `combine_main` +120, `deposit_main` +48, `weights_main` +11. Every other phase is within 3 ms. Its total is 655 → 1,402 ms. The beech's light is 1.7 → 17.6 ms of 108.5 → 122.7.
- **The new memory** is the light's scratch, 179 MB on Telperion, which a dormant lattice never allocated.
- **The spruce, birch and palm** now grow at k 0.5, but with no leaf area their lattice stays dormant on the device (fn-256 task 9), so they cost nothing more. The CPU's swept lattice reads them 0.99999982 rather than 1 (f32 rounding of the sky's shares), which nothing reads.
- **CPU against device:** the CPU cycle's light costs Telperion nothing measurable, while the device's costs it 0.75 s. The device's light kernels are the lever for this price, not the species rows.

## fn-251's frame (oak → birch, t = 0.07504803866484144, age 80, seed 7)

| | Frame light | Frame grown | `mesh::build` | Peak MB |
|---|---|---|---|---|
| Base | k 0.4625 (walked) | refused, budget of 20M | refused, engine phytomer budget | 3,801 |
| Tip | k 0.5 (the site's) | 19,376,743 | builds (22.3 s) | 4,737 |

- **It builds now only because the site's light no longer walks** towards the birch's 0, not because the birch has rows; its margin to the budget is 3%.
- **It does not pass fn-251's oracle.** The oak end at the same light, age and seed grows 16,625,765, and the frame is 17% above it, over the oracle's 10% band. The birch end (80, seed 7) outgrows the 20M budget on both base and tip under the one cycle (fn-251 measured 6.40M on its older executor), so the oracle's band has no birch end to stand on at this age.
