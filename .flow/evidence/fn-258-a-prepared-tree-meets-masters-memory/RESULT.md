# fn-258 R2 and R3: the cuts and where F4 stands against master

Measured 2026-10-10 on the fn-258 branch at 2b8615a7 (bebf8a17 changes only a view that sees none of the tree) against master at ebd2e37b, with the probe and protocol of `BYTES.md` (same machine, lock, resolution and MSAA; the probe now draws four frames at the hero pose before it reads memory, as a page shows the tree), seeds 1 and 42. Raw rows: `raw/r3c.jsonl` (sweep: master's rows from the first R3 sweep, F4's after the last cut), `raw/orbit-c.jsonl` (motion), `raw/lod.jsonl` (coarser frames), `raw/cut*.jsonl` (each cut).

**Where it stands.** After the five cuts F4 holds less GPU memory than master for the beech, and its tree buffers are smaller than master's for the beech, the oak and the birch. It holds more for the other presets: the palm and the ordinary by the 102 MB of compiled programs, the rest by up to 3.3×. It draws faster frames than master for the five real species and the palm. The ordinary is 8 % slower at the hero view at seed 1 and 57 % at seed 42; Laurelin and Telperion are 2.4-46× slower. The host's decision (2026-10-10): these remaining losses are the owner's, weighed in fn-259's qualification; fn-258 closes on code and evidence.

## R2: the cuts

Each cut names the consumer it checked. Cuts 1 to 4 change nothing drawn: the palm's and the oak's stills are byte-identical across them, and the spruce's and Telperion's close stills differ from before the cuts in as many pixels as two runs of the same binary differ (any channel changed: spruce 16.7 % against 15.4 %, Telperion 13.8 % against 14.6 %; by more than 8/255: 1.0 % against 0.9 %, 1.8 % against 1.9 %), all on leaves, viewed side by side. Cut 5 is the level-of-detail choice the host made.

1. **Growth scratch and the pool retired after preparation** (1b0e7c66). The grower's regions (`chain *`, 1.2-1.4 GB) and the pool of the last build's geometry scratch (`laid scratch`, `fine laid`, `packed *`, up to 1.7 GB) were kept for the next build. Consumer checked: the next build's grower and fine roots, the only readers. Test: `a_build_keeps_nothing_on_the_device_once_it_is_done` (red with the release removed: 1.15 GiB kept). Cost: a retune allocates them again.
2. **One set of surfacing buffers for both views** (a371641e, fixed in 000b5c75). The sun's records, vertices and indices (48-689 MiB) were kept beside the camera's. Consumer checked: the shadow pass, the only reader of the sun's output, in the frame that surfaces it. A frame that redraws the map surfaces the sun's view, draws the map, then surfaces the camera's into the same buffers; a remade sun view redraws its map (test: `a_remade_sun_view_keeps_the_maps_casters_counted`, red without the redraw).
3. **Coarser scales measured only when the finest overruns** (7e44ba29). Consumer checked: `choose`. Test: `a_view_past_its_finest_scale_is_drawn_at_the_coarser_scale_that_fits` holds a scale-8 view to the CPU reference.
4. **The count pass reuses the measure pass's counts** (d2566b5c). Consumer checked: the scan and rings passes, which read `counts`.
5. **Each view's budget grows from what it asked** (2b8615a7; host decision 2026-10-10). Consumer checked: `choose`, which draws a view past its budget at the finest scale that fits. Each view's budget is held to a capacity set from the demand its passes measured at the finest scale, read back a frame late, a quarter above it, and grown, never shrunk, when a later view asks more; each kind (rings, vertices, tube and ribbon indices) keeps its whole budget until a reading asks for some of it (Astra's review: a tree opened off screen learned a capacity of nothing and drew no wood when brought on; Codex's: a far view of ribbons alone took the tubes' room). Tests: `the_capacity_opens_a_quarter_above_the_view_and_only_grows`, `a_kind_no_view_asked_for_keeps_its_whole_budget`, `a_remade_sun_view_keeps_the_maps_casters_counted`. A resize to a lower resolution also asks more ribbon indices, and draws one coarser frame while the budget grows: the trunk-resolution and grain tests now compare the second frame of each view.

Not taken (host, 2026-10-10): the native allocator hint `MemoryHints::MemoryUsage`, which returned 0.7-1.0 GB of reserved blocks but slowed the spruce's vegetation pass by 24 % (`raw/hints*.jsonl`); native only.

### Frames drawn coarser (cut 5)

Each frame drawn and its report read, seed 1, default age, a fresh tree per preset: a fast zoom from the hero pose to a quarter of its distance in 30 frames, then a full turn of 120 frames at the whole, mid and close distances. Every coarser frame was at scale 2 with no cluster left out (overrun 0); the per-kind capacity (fix after review) left the palm, ordinary, spruce and Telperion counts and memory unchanged (`raw/lod2.jsonl`, `raw/kinds.jsonl`). None was at the hero pose: the zoom's first frame and the whole turn drew none.

| Tree | age | zoom | turn, whole | turn, mid | turn, close |
|---|--:|--:|--:|--:|--:|
| palm | 40 | 0 of 30 | 0 of 120 | 0 of 120 | 0 of 120 |
| ordinary | 40 | 8 of 30 | 0 of 120 | 0 of 120 | 0 of 120 |
| spruce | 40 | 4 of 30 | 0 of 120 | 0 of 120 | 1 of 120 |
| birch | 40 | 9 of 30 | 0 of 120 | 0 of 120 | 4 of 120 |
| oak | 40 | 4 of 30 | 0 of 120 | 0 of 120 | 0 of 120 |
| beech | 40 | 6 of 30 | 0 of 120 | 0 of 120 | 2 of 120 |
| Laurelin | 80 | 7 of 30 | 0 of 120 | 0 of 120 | 1 of 120 |
| Telperion | 82 | 7 of 30 | 0 of 120 | 0 of 120 | 1 of 120 |

Stills viewed: Telperion's zoom frames 7 and 12 at scale 2 beside the next frame at scale 1 of the same camera, whole and in a 640×480 crop of the trunk and limbs at full resolution. They are indistinguishable to the eye. Measured over the whole 1920×1080 frames (two frames of one tree in one process, so no run-to-run leaf variation): 4.7 and 6.3 % of pixels change in any channel, 1.1 and 1.2 % by more than 8/255. This is one sampled zoom and turn per preset, not a bound on every camera history, and the probe's blocking readbacks deliver each reading by the next frame; a browser's arrives when its queue does.

### What the cuts saved

Seed 1, each tree at the same age before the cuts (`BYTES.md`) and after all five:

| Tree, age | GPU process MB before → after | allocated MiB before → after | tree buffers MiB before → after | surfacing ms whole/mid/close before → after | retune build ms before → after |
|---|--:|--:|--:|--:|--:|
| date-palm, 40 | 839 → 583 | 185 → 81 | 106 → 2 | 0.38/0.18/0.25 → 0.26/0.17/0.26 | 6 → 5 |
| date-palm, 100 | 837 → 581 | 247 → 80 | 168 → 2 | 0.36/0.15/0.20 → 0.26/0.15/0.20 | 8 → 7 |
| ordinary, 40 | 1607 → 583 | 754 → 93 | 675 → 14 | 0.43/0.52/0.53 → 0.29/0.37/0.38 | 553 → 482 |
| ordinary, 100 | 1865 → 1097 | 1383 → 238 | 1304 → 159 | 1.29/1.61/2.02 → 0.90/1.16/1.56 | 16471 → 14160 |
| norway-spruce, 40 | 3823 → 1611 | 2929 → 507 | 1445 → 429 | 2.26/1.84/1.20 → 1.78/1.47/0.96 | 398 → 414 |
| silver-birch, 40 | 3171 → 1353 | 2382 → 150 | 850 → 71 | 0.87/0.94/0.96 → 0.67/0.73/0.74 | 767 → 724 |
| silver-birch, 70 | 4847 → 1355 | 3240 → 438 | 1556 → 359 | 2.12/2.48/2.32 → 1.77/2.11/1.98 | 2037 → 1971 |
| oregon-white-oak, 40 | 1609 → 1097 | 838 → 153 | 759 → 75 | 0.81/1.04/1.09 → 0.64/0.85/0.88 | 1527 → 1239 |
| oregon-white-oak, 100 | 2377 → 1353 | 1591 → 497 | 1512 → 418 | 2.79/3.80/2.12 → 2.06/2.84/1.50 | 25401 → 21777 |
| european-beech, 40 | 2799 → 841 | 2200 → 102 | 686 → 24 | 0.57/0.69/0.74 → 0.41/0.50/0.55 | 167 → 301 |
| laurelin, 80 | 4847 → 1099 | 3854 → 348 | 1444 → 234 | 1.70/2.14/2.13 → 1.37/1.82/1.80 | 546 → 620 |
| laurelin, 100 | 5871 → 1355 | 4872 → 557 | 1627 → 415 | 2.61/3.29/3.40 → 2.21/2.90/2.98 | 971 → 967 |
| telperion, 82 | 4849 → 1355 | 3926 → 345 | 1470 → 248 | 1.73/2.22/2.15 → 1.40/1.88/1.82 | 587 → 643 |
| telperion, 100 | 5873 → 1099 | 4840 → 522 | 1627 → 412 | 2.57/3.23/3.24 → 2.15/2.86/2.83 | 897 → 986 |

## R3: against master, at the default age and at master's age

Bold is worse than master. "Frame ms" is wall clock per pipelined frame, camera still (best of three rounds of 120). "Orbit total" is the median GPU time of the three tree passes over a full turn at the hero pose (`measure_orbit`), seed 1. Peak RSS is the CPU high-water mark over five builds. The first block is each version at its own default age; the second is F4 at master's age.

The three refused rows were retried once after cut 5: the spruce at 100 (both seeds) and the beech at 120 (seed 42) still ran out of device memory in F4's preparation, with 1,751 MiB of the 10,240 MiB card held by other processes at the time. They stay refused.

| Tree | seed | ages old/new | GPU process MB old→new | tree buffers MiB old→new | frame ms whole old→new | mid | close | orbit total ms whole old→new | peak RSS MB old→new |
|---|--:|---|--:|--:|--:|--:|--:|--:|--:|
| beech | 1 | 120/40 | 1059→841 | 558→24 | 16.17→1.47 | 68.99→2.31 | 131.94→2.76 | 16.63→1.35 | 912→417 |
| beech | 42 | 120/40 | 1059→841 | 531→66 | 14.63→2.72 | 58.49→4.68 | 123.36→4.60 | - | 883→427 |
| oak | 1 | 100/40 | 739→**1097** | 243→75 | 16.35→4.16 | 29.07→9.95 | 25.38→13.86 | 16.63→3.90 | 347→**911** |
| oak | 42 | 100/40 | 995→**1097** | 271→79 | 16.44→4.62 | 33.11→11.52 | 32.25→15.58 | - | 349→**956** |
| spruce | 1 | 100/40 | 1251→**1611** | 398→**429** | 23.06→6.61 | 22.88→9.08 | 13.20→6.91 | 22.96→6.60 | 348→**607** |
| spruce | 42 | 100/40 | 1251→**1355** | 382→**435** | 18.41→7.85 | 18.46→10.30 | 12.02→7.77 | - | 347→**536** |
| birch | 1 | 70/40 | 739→**1353** | 144→71 | 7.92→1.86 | 24.44→3.01 | 38.45→4.09 | 8.22→1.85 | 352→**464** |
| birch | 42 | 70/40 | 739→**1097** | 147→85 | 8.44→2.28 | 25.58→4.10 | 34.32→6.70 | - | 348→**475** |
| palm | 1 | 100/40 | 483→**583** | 1→**2** | 0.80→0.57 | 1.61→0.58 | 2.20→0.78 | 0.77→0.44 | 337→**358** |
| palm | 42 | 100/40 | 483→**583** | 1→**2** | 0.79→0.53 | 1.64→0.61 | 2.17→0.78 | - | 339→**356** |
| ordinary | 1 | 100/40 | 481→**583** | 19→14 | 0.51→**0.55** | 0.74→0.73 | 0.89→0.86 | 0.39→**0.48** | 329→**399** |
| ordinary | 42 | 100/40 | 481→**583** | 15→**27** | 0.46→**0.72** | 0.62→**0.98** | 0.63→**1.08** | - | 325→**482** |
| Laurelin | 1 | 100/80 | 483→**1099** | 76→**234** | 1.33→**5.20** | 1.63→**11.37** | 1.30→**12.28** | 1.33→**5.03** | 335→**470** |
| Laurelin | 42 | 100/80 | 739→**1355** | 196→**409** | 3.42→**8.38** | 3.85→**21.41** | 2.86→**19.07** | - | 353→**501** |
| Telperion | 1 | 100/82 | 739→**1355** | 175→**248** | 1.91→**5.06** | 1.61→**11.46** | 1.65→**12.21** | 1.79→**5.18** | 344→**480** |
| Telperion | 42 | 100/82 | 481→**1611** | 42→**433** | 0.69→**8.34** | 0.54→**20.98** | 0.41→**18.92** | - | 323→**490** |
| | | | | | | | | | |
| beech | 1 | 120/120 | 1059→**1867** | 558→**732** | 16.17→11.96 | 68.99→19.46 | 131.94→20.70 | - | 912→707 |
| beech | 42 | 120/120 | refused (device memory) | | | | | | |
| oak | 1 | 100/100 | 739→**1353** | 243→**418** | 16.35→11.96 | 29.07→22.24 | 25.38→19.62 | - | 347→**9150** |
| oak | 42 | 100/100 | 995→**1417** | 271→**364** | 16.44→10.22 | 33.11→20.74 | 32.25→21.41 | - | 349→**7668** |
| spruce | 1 | 100/100 | refused (device memory) | | | | | | |
| spruce | 42 | 100/100 | refused (device memory) | | | | | | |
| birch | 1 | 70/70 | 739→**1355** | 144→**359** | 7.92→5.40 | 24.44→8.35 | 38.45→13.48 | - | 352→**871** |
| birch | 42 | 70/70 | 739→**1353** | 147→**416** | 8.44→6.26 | 25.58→9.81 | 34.32→17.38 | - | 348→**987** |
| palm | 1 | 100/100 | 483→**581** | 1→**2** | 0.80→0.47 | 1.61→0.36 | 2.20→0.41 | - | 337→**339** |
| palm | 42 | 100/100 | 483→**581** | 1→**2** | 0.79→0.44 | 1.64→0.37 | 2.17→0.41 | - | 339→335 |
| ordinary | 1 | 100/100 | 481→**1097** | 19→**159** | 0.51→**1.64** | 0.74→**2.31** | 0.89→**3.22** | - | 329→**3579** |
| ordinary | 42 | 100/100 | 481→**1353** | 15→**283** | 0.46→**2.66** | 0.62→**4.12** | 0.63→**5.80** | - | 325→**6255** |
| Laurelin | 1 | 100/100 | 483→**1355** | 76→**415** | 1.33→**8.09** | 1.63→**15.03** | 1.30→**17.14** | - | 335→**563** |
| Laurelin | 42 | 100/100 | 739→**1867** | 196→**667** | 3.42→**13.11** | 3.85→**30.87** | 2.86→**23.00** | - | 353→**571** |
| Telperion | 1 | 100/100 | 739→**1099** | 175→**412** | 1.91→**7.37** | 1.61→**15.31** | 1.65→**17.34** | - | 344→**572** |
| Telperion | 42 | 100/100 | 481→**1867** | 42→**661** | 0.69→**11.10** | 0.54→**27.85** | 0.41→**22.47** | - | 323→**669** |
| | | | | | | | | | |

Orbit, all three views, seed 1 (GPU total ms, master → F4): see `raw/orbit-c.jsonl`. The ordinary, Laurelin and Telperion stay slower in motion; every other preset is faster at every view.

## Remaining losses, for the owner (fn-259)

At the default age, both seeds:

| Loss against master | Size | What it buys |
|---|---|---|
| Surfacing pass every frame | 0.15-3.7 ms over the presets and views | Wood detail chosen per view at half a pixel's error; master tessellated once at a fixed detail |
| Two Trees' vegetation pass | 3.5-17.5 ms against master's 0.23-3.28 | F4's full flowering crowns, where master drew bare contorted limbs (scorecard stills) |
| Compiled programs | about 100 MB of driver memory before any tree; the palm and the ordinary sit 100 MB over master for it | No compile at the first build (fn-243 compiles them at page load) |
| View budgets | 1.5-268 MiB of the views' buffers within 2-435 MiB of tree buffers; the tree buffers are more than master's on the palm, the spruce, Laurelin, Telperion and the ordinary at seed 42 | Up to 9 of 30 frames of a fast zoom and 4 of 120 of a close turn drawn at scale 2 for a frame; none at the hero pose |
| Native allocator blocks | reserved above allocated, which `nvidia-smi` counts: F4 407-1,130 MiB, master 112-819 MiB | Frame time: the `MemoryUsage` hint frees them and costs the spruce 24 % of its vegetation pass (not taken) |
| Peak CPU at master's age | oak at 100: 7.7-9.2 GB; ordinary at 100: 3.6-6.3 GB (master: 0.33-0.35 GB) | Nothing drawn: the preparation's working set (fn-255/fn-256) |
| Preparation at master's age | the spruce at 100 and the beech at 120 (seed 42) refused on a 10 GB card with 1.75 GB held by others | Same as above |
| Retune build | up to +134 ms where the fine executor grows the tree | The 1.2-3.1 GB cut 1 retired |

## Gate

First run, 2026-10-10, at d2566b5c, before the review fix and cut 5:

- `cargo test --profile ci -p telperion-render --no-fail-fast` under the GPU lock (4 test threads): 49 result lines, one red, `every_pipeline_within_2_5_s_but_grow_within_20_s_cold` (grow 25.6 s against 20 s; fn-237 R9). Run alone once: green, 22.8 s.
- `cargo test --profile ci --no-fail-fast` package by package, `telperion-core`, `-space`, `-field`, `-wasm`, `-jev`: all green.
- `npm test`: 14 files, 155 tests, green.
- `cargo check -p telperion-render -p telperion-wasm --target wasm32-unknown-unknown`: clean.

Final run, 2026-10-11, at bebf8a17 (after the review fix and cut 5):

- `cargo test --profile ci -p telperion-render --no-fail-fast` under the GPU lock (4 test threads): 49 result lines, one red, `every_pipeline_within_2_5_s_but_grow_within_20_s_cold` (`girths` 2.68 s against 2.5 s; fn-237 R9). Run alone once, at a load average of 6: red again, `grow` 20.6 s against 20 s. Neither kernel is one fn-258 changed.
- `npm test`: 14 files, 155 tests, green.
- `cargo check -p telperion-render -p telperion-wasm --target wasm32-unknown-unknown`: clean.
- The other packages were not re-run: no fn-258 change since the first run touches them.

## Astra

`gpt-6-astra`, high effort, read-only, on RESULT.md, BYTES.md, the code diff and the six stills in `raw/stills/`. First reading (`raw/astra-result.md`): cuts 1-4 sound with their consumers named, the stills show no perceptible change, cut 5's scale-2 frames indistinguishable, the main table regenerated from the raw rows and matching; DISAGREE on one code blocker (a tree opened off screen learned a capacity of nothing) and four evidence corrections. All five fixed in bebf8a17 and this file. Follow-up (`raw/astra-result-2.md`): blocker and corrections resolved, nothing further to block on: **AGREE WITH NOTES**, the note being this final gate.

