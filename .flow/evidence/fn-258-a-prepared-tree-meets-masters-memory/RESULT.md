# fn-258 R2 and R3: the cuts and where F4 stands against master

Measured 2026-10-10 on the fn-258 branch at d2566b5c against master at ebd2e37b, with the probe and protocol of `BYTES.md` (same machine, lock, resolution and MSAA), seeds 1 and 42. Raw rows: `raw/r3.jsonl` (sweep), `raw/orbit.jsonl` (motion), `raw/cut*.jsonl` (each cut).

**Verdict: R3 is not met.** After the cuts, F4 holds more GPU memory than master on every preset at the product default age (the beech by 4 %, the rest by 21 % to 3.9×), and draws slower frames for the ordinary (whole view, seed 1 by 10 %, seed 42 by 65 %), Laurelin and Telperion (2.4-46×). The rest needs the owner: one level-of-detail choice and the trades in "Remaining losses".

## R2: the cuts

Each cut names the consumer it checked; none changes what is drawn. The palm's and the oak's stills are byte-identical before and after every cut. The spruce's and Telperion's stills differ by as much as two runs of the same binary differ (0.08-0.16 % of pixels, all on leaves, `stills/` in the scratch); they were viewed side by side.

1. **Growth scratch and the pool retired after preparation** (1b0e7c66). The grower's regions (`chain *`, 1.2-1.4 GB) and the pool of the last build's geometry scratch (`laid scratch`, `fine laid`, `packed *`, up to 1.7 GB) were kept for the next build. Consumer checked: the next build's grower and fine roots, the only readers. The regions are released after the build, the pool is deleted, and admission counts the ledger's live bytes. Test: `a_build_keeps_nothing_on_the_device_once_it_is_done` (red with the release removed: 1.15 GiB kept). Cost: a retune allocates them again.
2. **One set of surfacing buffers for both views** (a371641e). The sun's records, vertices and indices (48-689 MiB) were kept beside the camera's. Consumer checked: the shadow pass, the only reader of the sun's output, in the frame that surfaces it. A frame that redraws the map now surfaces the sun's view, draws the map, then surfaces the camera's into the same buffers. The map is drawn again only with its wood; the sun's surfacing is timed inside the shadow pair.
3. **Coarser scales measured only when the finest overruns** (7e44ba29). Consumer checked: `choose`, which read all four scales' totals and picked the finest that fits. The finest is measured first and the coarser scales are dispatched indirectly only where it does not fit. Test: `a_view_past_its_finest_scale_is_drawn_at_the_coarser_scale_that_fits` holds a scale-8 view to the CPU reference.
4. **The count pass reuses the measure pass's counts** (d2566b5c). Consumer checked: the scan and rings passes, which read `counts`. Where the finest scale is chosen, the measure pass already walked every cluster at it and now keeps each cluster's counts.

Not taken: the native allocator hint `MemoryHints::MemoryUsage`, which returned 0.7-1.0 GB of reserved blocks (Telperion 2,123 to 1,384 MB, the palm to 482 MB, under master) but slowed the spruce's vegetation pass by 24 % in two runs (`raw/hints*.jsonl`). It is native only; a browser allocates on its own.

What the cuts saved, seed 1, each tree at the same age before and after:

| Tree, age | GPU process MB before → after | allocated MiB before → after | surfacing ms whole/mid/close before → after | retune build ms before → after |
|---|--:|--:|--:|--:|
| date-palm, 40 | 839 → 583 | 185 → 137 | 0.38/0.18/0.25 → 0.25/0.17/0.24 | 6 → 6 |
| date-palm, 100 | 837 → 581 | 247 → 176 | 0.36/0.15/0.20 → 0.23/0.14/0.20 | 8 → 7 |
| ordinary, 40 | 1607 → 839 | 754 → 444 | 0.43/0.52/0.53 → 0.29/0.37/0.38 | 553 → 536 |
| ordinary, 100 | 1865 → 1353 | 1383 → 876 | 1.29/1.61/2.02 → 0.89/1.14/1.53 | 16471 → 19787 |
| norway-spruce, 40 | 3823 → 2123 | 2929 → 1027 | 2.26/1.84/1.20 → 1.79/1.47/0.97 | 398 → 448 |
| silver-birch, 40 | 3171 → 1609 | 2382 → 571 | 0.87/0.94/0.96 → 0.67/0.74/0.74 | 767 → 741 |
| silver-birch, 70 | 4847 → 1865 | 3240 → 984 | 2.12/2.48/2.32 → 1.86/2.19/2.02 | 2037 → 2044 |
| oregon-white-oak, 40 | 1609 → 1353 | 838 → 500 | 0.81/1.04/1.09 → 0.64/0.85/0.87 | 1527 → 1362 |
| oregon-white-oak, 100 | 2377 → 1609 | 1591 → 1007 | 2.79/3.80/2.12 → 2.05/2.82/1.52 | 25401 → 30677 |
| european-beech, 40 | 2799 → 1097 | 2200 → 450 | 0.57/0.69/0.74 → 0.41/0.51/0.56 | 167 → 300 |
| laurelin, 80 | 4847 → 1611 | 3854 → 976 | 1.70/2.14/2.13 → 1.37/1.82/1.80 | 546 → 632 |
| laurelin, 100 | 5871 → 1867 | 4872 → 1081 | 2.61/3.29/3.40 → 2.21/2.90/2.97 | 971 → 987 |
| telperion, 82 | 4849 → 2121 | 3926 → 966 | 1.73/2.22/2.15 → 1.40/1.89/1.82 | 587 → 652 |
| telperion, 100 | 5873 → 1867 | 4840 → 1049 | 2.57/3.23/3.24 → 2.15/2.86/2.83 | 897 → 1021 |

Retune builds moved by -165 to +133 ms at default age; the ordinary and the oak at 100 were 3-5 s slower, under load averages of 5-8.

## R3: against master, at the default age and at master's age

Bold is worse than master. "Frame ms" is wall clock per pipelined frame, camera still (best of three rounds of 120). "Orbit total" is the median GPU time of the three tree passes over a full turn at the hero pose (the motion protocol, `measure_orbit`), seed 1. Peak RSS is the CPU high-water mark over five builds. The first block is each version at its own default age; the second is F4 at master's age. Rows marked refused: F4's preparation ran out of device memory while other sessions held 1.6-4.5 GB of the 10 GB card.

| Tree | seed | ages old/new | GPU process MB old→new | tree buffers MiB old→new | frame ms whole old→new | mid | close | orbit total ms whole old→new | peak RSS MB old→new |
|---|--:|---|--:|--:|--:|--:|--:|--:|--:|
| beech | 1 | 120/40 | 1059→**1097** | 558→371 | 16.17→1.47 | 68.99→2.34 | 131.94→2.79 | 16.63→1.37 | 912→413 |
| beech | 42 | 120/40 | 1059→**1097** | 531→428 | 14.63→2.72 | 58.49→4.69 | 123.36→4.64 | - | 883→429 |
| oak | 1 | 100/40 | 739→**1353** | 243→**421** | 16.35→4.17 | 29.07→9.93 | 25.38→13.81 | 16.63→3.90 | 347→**906** |
| oak | 42 | 100/40 | 995→**1353** | 271→**423** | 16.44→4.67 | 33.11→11.63 | 32.25→15.68 | - | 349→**942** |
| spruce | 1 | 100/40 | 1251→**2123** | 398→**949** | 23.06→6.60 | 22.88→9.10 | 13.20→6.93 | 22.96→6.61 | 348→**596** |
| spruce | 42 | 100/40 | 1251→**1867** | 382→**955** | 18.41→7.91 | 18.46→10.37 | 12.02→7.84 | - | 347→**529** |
| birch | 1 | 70/40 | 739→**1609** | 144→**493** | 7.92→1.85 | 24.44→3.01 | 38.45→4.11 | 8.22→1.86 | 352→**470** |
| birch | 42 | 70/40 | 739→**1609** | 147→**518** | 8.44→2.20 | 25.58→4.14 | 34.32→6.75 | - | 348→**464** |
| palm | 1 | 100/40 | 483→**583** | 1→**58** | 0.80→0.52 | 1.61→0.59 | 2.20→0.78 | 0.77→0.41 | 337→**345** |
| palm | 42 | 100/40 | 483→**583** | 1→**58** | 0.79→0.55 | 1.64→0.62 | 2.17→0.78 | - | 339→**350** |
| ordinary | 1 | 100/40 | 481→**839** | 19→**366** | 0.51→**0.56** | 0.74→0.73 | 0.89→0.88 | 0.39→**0.48** | 329→**373** |
| ordinary | 42 | 100/40 | 481→**1095** | 15→**390** | 0.46→**0.76** | 0.62→**1.00** | 0.63→**1.10** | - | 325→**467** |
| Laurelin | 1 | 100/80 | 483→**1611** | 76→**862** | 1.33→**5.17** | 1.63→**11.38** | 1.30→**12.26** | 1.33→**5.03** | 335→**474** |
| Laurelin | 42 | 100/80 | 739→**1867** | 196→**930** | 3.42→**8.35** | 3.85→**21.32** | 2.86→**18.91** | - | 353→**503** |
| Telperion | 1 | 100/82 | 739→**2121** | 175→**869** | 1.91→**5.31** | 1.61→**11.07** | 1.65→**12.26** | 1.79→**5.18** | 344→**466** |
| Telperion | 42 | 100/82 | 481→**1867** | 42→**941** | 0.69→**8.29** | 0.54→**20.93** | 0.41→**18.87** | - | 323→**501** |
| | | | | | | | | | |
| beech | 1 | 120/120 | 1059→**1867** | 558→**1010** | 16.17→12.01 | 68.99→19.50 | 131.94→20.71 | - | 912→679 |
| beech | 42 | 120/120 | refused (device memory) | | | | | | |
| oak | 1 | 100/100 | 739→**1609** | 243→**929** | 16.35→11.92 | 29.07→22.20 | 25.38→19.64 | - | 347→**9004** |
| oak | 42 | 100/100 | 995→**1673** | 271→**884** | 16.44→10.24 | 33.11→20.79 | 32.25→21.46 | - | 349→**7646** |
| spruce | 1 | 100/100 | refused (device memory) | | | | | | |
| spruce | 42 | 100/100 | refused (device memory) | | | | | | |
| birch | 1 | 70/70 | 739→**1865** | 144→**906** | 7.92→5.96 | 24.44→8.84 | 38.45→13.59 | - | 352→**933** |
| birch | 42 | 70/70 | 739→**1609** | 147→**925** | 8.44→6.31 | 25.58→9.89 | 34.32→17.49 | - | 348→**1001** |
| palm | 1 | 100/100 | 483→**581** | 1→**97** | 0.80→0.48 | 1.61→0.35 | 2.20→0.41 | - | 337→336 |
| palm | 42 | 100/100 | 483→**581** | 1→**97** | 0.79→0.45 | 1.64→0.36 | 2.17→0.42 | - | 339→334 |
| ordinary | 1 | 100/100 | 481→**1353** | 19→**797** | 0.51→**1.64** | 0.74→**2.31** | 0.89→**3.19** | - | 329→**3688** |
| ordinary | 42 | 100/100 | 481→**1865** | 15→**876** | 0.46→**2.66** | 0.62→**4.13** | 0.63→**5.81** | - | 325→**6502** |
| Laurelin | 1 | 100/100 | 483→**1867** | 76→**939** | 1.33→**8.07** | 1.63→**15.05** | 1.30→**17.04** | - | 335→**516** |
| Laurelin | 42 | 100/100 | 739→**1867** | 196→**1036** | 3.42→**13.01** | 3.85→**30.65** | 2.86→**23.00** | - | 353→**566** |
| Telperion | 1 | 100/100 | 739→**1867** | 175→**939** | 1.91→**7.37** | 1.61→**14.33** | 1.65→**16.28** | - | 344→**535** |
| Telperion | 42 | 100/100 | 481→**1867** | 42→**1036** | 0.69→**10.96** | 0.54→**27.64** | 0.41→**22.40** | - | 323→**573** |
| | | | | | | | | | |

Orbit, all three views, seed 1 (GPU total ms, old → new): ordinary 0.39/0.64/0.64 → 0.48/0.65/0.74; Laurelin 1.33/1.56/1.15 → 5.03/11.47/11.91; Telperion 1.79/1.47/1.37 → 5.18/10.89/11.73. Every other preset is faster in motion than master at every view.

## The level-of-detail choice (needs the owner)

What remains of F4's tree buffers is the camera's surfacing budget: 58-776 MiB, sized for the most any view could ask (every ring at 384 sides, capped by the screen's pixels and two 128 MiB bindings). The hero, mid and close views ask far less. A budget sized from the largest demand those three views measured, with a quarter's headroom, would hold:

| Tree | age | shared now MiB | largest measured demand ×1.25, MiB | share of now |
|---|--:|--:|--:|--:|
| date-palm | 40 | 58 | 2 | 3% |
| date-palm | 100 | 97 | 1 | 1% |
| ordinary | 40 | 362 | 17 | 5% |
| ordinary | 100 | 736 | 137 | 19% |
| norway-spruce | 40 | 746 | 228 | 31% |
| silver-birch | 40 | 469 | 54 | 11% |
| silver-birch | 70 | 768 | 267 | 35% |
| oregon-white-oak | 40 | 394 | 66 | 17% |
| oregon-white-oak | 100 | 737 | 273 | 37% |
| european-beech | 40 | 365 | 24 | 7% |
| european-beech | 120 | 768 | 742 | 97% |
| laurelin | 80 | 768 | 179 | 23% |
| laurelin | 100 | 768 | 293 | 38% |
| telperion | 82 | 768 | 189 | 25% |
| telperion | 100 | 768 | 289 | 38% |

That would cut 62-99 % of the budget at default age. It is a level-of-detail choice: a view that asks more than the budget, nearer or wider than the three measured, is drawn at a coarser error scale, or waits a frame for a larger budget. I have not built it.

## Remaining losses, measured, with what each buys

At the default age, both seeds, unless stated:

| Loss against master | Size | What it buys |
|---|---|---|
| Camera surfacing budget | 58-776 MiB of tree buffers | The wood never drawn coarser at any view, however close |
| Surfacing pass every frame | 0.17-3.71 ms over both seeds and the three views (seed 1 before the cuts: 0.18-2.26) | Wood detail chosen per view at half a pixel's error; master tessellated once at a fixed detail |
| Two Trees' vegetation pass | 3.6-17.5 ms against master's 0.23-3.28 | F4's full flowering crowns, where master drew bare contorted limbs (scorecard stills) |
| Compiled programs | about 100 MB of driver memory before any tree | No compile at the first build (fn-243 compiles them at page load) |
| Native allocator blocks | 0.3-1.1 GB reserved above what is allocated (`nvidia-smi` counts them) | Frame time: the `MemoryUsage` hint frees them and costs the spruce 24 % of its vegetation pass |
| Peak CPU at master's age | oak at 100: 7.6-9.0 GB; ordinary at 100: 3.7-6.5 GB (master: 0.33-0.35 GB) | Nothing drawn: the preparation's working set. In a browser a Wasm memory keeps its peak |
| Peak device memory at master's age | the spruce at 100 (both seeds) and the beech at 120 (seed 42) could not be prepared on a 10 GB card shared with other sessions | Same as above |
| Retune build | +0-140 ms where the fine executor grows the tree | The 1.2-3.1 GB the cut retired |

## Not done

- Astra's agreement (R3) has not been asked.
- Motion was measured, not viewed: the agent rules forbid opening videos and frame sequences.
- `docs/` and the harness are unchanged; no shipped preset or default was touched.
