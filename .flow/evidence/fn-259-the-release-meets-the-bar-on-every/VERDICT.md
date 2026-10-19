# fn-259 R3: old-versus-new stills and the regression ledger, for the owner

R3 is met only by the owner's recorded words. This packet holds the stills, the ledger and the questions. No verdict below is the agent's. The owner's species verdicts of 2026-10-15 (fn-237, review page `Kfsy2fgr5B2tJ4uiE1qxae`) are quoted as context. They were taken on F4's stills alone, not beside 0.1.4, so they do not answer the questions here.

The site's grove stills and the grove verdict are not in this packet. They are killenberger.com's work (owner, 2026-10-15, fn-237 "Scope: the site is the site's repository").

## How the stills were made

Rendered 2026-10-19, 07:19-07:21, on the RTX 3080, native wgpu under `telperion-heavy --gpu 3`, one job at a time. No browser and no WebGPU were used. Each still is 960×720.

- **F4:** the resident device path, `crates/telperion-render/examples/resident_stills.rs` (`Generator::prepare`, `submit_prepared`). This task added its `SEED` and `AGE` overrides.
- **0.1.4:** tag `v0.1.4` (commit `c79bb590e`), in a detached scratch worktree with its own target directory. `v014-probe.patch` was applied, plus a stills adapter (`raw/t7/v014-resident_stills.rs`, ignored) on the `frames` probe's route: `Generator::prepare(&family, Delivery::Resident)`, `submit_prepared`, and the family's material.
- **Pose:** both versions use one rule. The whole view is `hero_pose` from the drawn tree's bounds. The mid and close views are F4's `nearer` at 0.4 and 0.12 of the tree's height. Both versions compute the views from their own tree's bounds, so the cameras sit in the same place relative to each tree, not in the same world position.
- **Seeds and ages:**
  - Seeds 1, 7 and 42, the scorecard's.
  - F4 at its product default age: 40, Laurelin 80, Telperion 82.
  - 0.1.4's direct build reads no age. It draws its mature tree, labelled "mature direct build; reads no age".
- **0.1.4's palm** took its CPU fallback on this route (`backend CpuFallback` at every seed), as in the scorecard. Every other 0.1.4 build reported `Gpu/Gpu`.
- **Sheets:**
  - Each sheet is one compressed JPEG in `sheets/`, with one row per seed and the columns 0.1.4 whole, F4 whole, 0.1.4 mid, F4 mid, 0.1.4 close, F4 close.
  - Every tile is labelled with version, route, age, seed and view. The full-size PNGs are in `raw/t7/` (ignored), and the scripts are `raw/t7/t7-stills.sh` and `raw/t7/t7-sheets.sh`.
- **Matched age:**
  - F4 at the age the scorecard's matched-age rows carry. The palm and the ordinary drew at 100 and the birch at 70.
  - The oak and the spruce at 100 were refused: "the cycle state axes buffer needs 2558845952 bytes" (the oak) and "2513984512 bytes" (the spruce), each "above the device's limit of 2147483644".
  - Laurelin and Telperion at 100 were refused: "the tree grows past the engine's budget of 20000000 phytomers".
  - The scorecard records the same refusals.

### What the pixel helper states

`node scripts/pixel-diff.mjs <0.1.4> <F4>` was run on every pair; the output is in `raw/t7/diff.txt`. The table gives the share of pixels where a channel differs by more than 8/255, over seeds 1 / 7 / 42.

Almost every pixel differs by some amount, 75.4-83.3 % even at the whole view. One cause is that the ground is drawn 1-3 levels apart between the versions: the bottom-left pixel reads 114,121,88 on 0.1.4 and 115,121,85 on F4, while the sky's corner pixel is identical (measured on the ordinary's and the spruce's whole views). So the over-8/255 count is the one that tracks the trees. A count is not a judgment of which still looks better.

| Sheet | F4 age | Whole | Mid | Close |
|---|--:|--:|--:|--:|
| `sheets/ordinary.jpg` | 40 | 17.4 / 20.0 / 22.0 % | 56.8 / 57.1 / 64.5 % | 62.9 / 72.2 / 77.2 % |
| `sheets/date-palm.jpg` | 40 | 19.6 / 20.4 / 20.2 % | 66.2 / 66.7 / 67.8 % | 82.5 / 85.2 / 79.7 % |
| `sheets/oregon-white-oak.jpg` | 40 | 39.3 / 38.7 / 41.8 % | 96.0 / 97.5 / 98.2 % | 94.5 / 96.6 / 98.7 % |
| `sheets/silver-birch.jpg` | 40 | 28.4 / 33.0 / 24.3 % | 95.6 / 98.9 / 88.1 % | 98.6 / 98.1 / 97.0 % |
| `sheets/norway-spruce.jpg` | 40 | 15.4 / 15.5 / 15.4 % | 44.8 / 43.8 / 45.8 % | 68.7 / 69.3 / 68.4 % |
| `sheets/laurelin.jpg` | 80 | 39.3 / 36.4 / 37.3 % | 91.3 / 90.7 / 97.9 % | 98.3 / 98.3 / 97.1 % |
| `sheets/telperion.jpg` | 82 | 36.2 / 34.3 / 36.3 % | 87.8 / 77.4 / 87.6 % | 98.6 / 85.9 / 98.1 % |
| `sheets/date-palm-matched-age-100.jpg` | 100 | 17.6 / 16.7 / 17.3 % | 48.9 / 52.1 / 51.8 % | 72.1 / 77.0 / 64.8 % |
| `sheets/ordinary-matched-age-100.jpg` | 100 | 21.2 / 22.2 / 24.3 % | 68.4 / 77.6 / 86.6 % | 79.3 / 96.8 / 97.3 % |
| `sheets/silver-birch-matched-age-70.jpg` | 70 | 29.6 / 34.7 / 26.4 % | 95.9 / 98.9 / 95.3 % | 99.0 / 98.5 / 99.2 % |
| `sheets/european-beech.jpg` | 40 | F4 only: 0.1.4 did not ship the beech (host, 2026-10-19) | | |

Two pairs are not the same kind of tree (measured, `SCORECARD.md`):

- **Telperion:** 0.1.4's Telperion at seeds 7 and 42 is small. Its field marks 1,219-1,611 wood cells against F4's 5,976-7,364.
- **The palm:** 0.1.4's palm is a CPU mesh.

## The owner's species verdicts of 2026-10-15 (context, F4 only)

| Species | Verdict | The owner's note |
|---|---|---|
| Ordinary | approve | |
| Date palm | approve | |
| Oregon white oak | needs work | "seed 7 specifically doesn't look like a real tree. Lower branches that grow to long and zigzag weirdly." Later: "Oak seed 7 does look like it needs fixing": a release item. |
| European beech | needs work, after the release | "they really aren't full enough.. … But i think we can move that work to after release." |
| Silver birch | needs work | "… But it's the best species." Ground branches accepted for the release, fn-282 after it. |
| Norway spruce | needs work | "Looks really good but still needs fuller curtains and larger species." |
| Telperion | needs work, accepted for now | "Doesn't look magical enough and flowers look too much like copy pasted. Not natural. But can accept for now" |
| Laurelin | needs work | "Looks better than telperion but still flowers are too samey all over. and doesn't look magical enough." |

## The regression ledger

Every row where F4 is worse than 0.1.4, by any amount, at the product default ages, from `SCORECARD.md` (R1, tasks 4 and 5) and `AUDIT.md` (R2, task 6). Sizes are medians over seeds 1 / 7 / 42, or their range. **Fidelity gain offered** names only recorded evidence:

- the owner's 2026-10-15 verdict on the species, quoted above, which was taken on F4 alone;
- the product rule that "Telperion grows trees botanically, year by year, and that growth is part of the product, locked in" (owner, 2026-10-13, AGENTS.md).

No row has a measured fidelity comparison against 0.1.4. That comparison is the stills question asked below.

Rows the owner's tolerances already cover are listed apart at the end and carry no question.

| # | Item | Measured size (F4 − 0.1.4) | Ratio | Owner tolerance | Fidelity gain offered (recorded) |
|---|---|---|---|---|---|
| L1 | Fresh build (renderer start to a fenced first frame), every preset | palm +1,754-1,839; ordinary +2,117-2,144; oak +1,834-1,950; birch +1,677-1,819; spruce +1,984-2,093; Laurelin +2,416-2,591; Telperion +2,186-2,845 ms. About 4 s of F4's start is shader compilation. | ×1.60-2.10 | none (R0a names warm builds only) | botanical growth (2026-10-13); per species, the verdicts above |
| L2 | Laurelin warm build, seed 7 | 1,272 ms against 308 ms, +964 ms | ×4.13 | open: R0a's "about 1.25 s" | Laurelin: "Looks better than telperion" |
| L3 | Laurelin warm build, seed 42 | 1,346 ms against 565 ms, +782 ms | ×2.38 | none: over 1.25 s and over the 1.33 s accepted for Telperion | as L2 |
| L4 | Telperion warm build, seeds 7 and 42 | 1,356 ms against 97 ms (+1,259); 1,413 ms against 83 ms (+1,330) | ×13.95 / ×17.02 | none: over the 1.33 s the owner accepted at seed 42 (2026-10-16). 0.1.4's Telperion at these seeds is small. | Telperion: "accepted for now" |
| L5 | Ordinary frames | whole +0.05 / +0.16 / +0.27; mid +0.23 / +0.36 and close +0.29 / +0.48 ms at seeds 7 / 42 | ×1.10-1.77 | none | Ordinary: approve |
| L6 | Laurelin frames | whole +4.26-6.66; mid +13.01-26.63; close +12.20-21.18 ms (F4 5.70-28.68 ms) | ×2.50-13.96 | none | as L2 |
| L7 | Telperion frames | whole +2.71-7.97; mid +10.28-21.74; close +9.90-17.10 ms (F4 5.40-22.28 ms) | ×2.00-45.38 | none | as L4 |
| L8 | CPU peak (one build), every preset | palm +527-533; ordinary +510-517; oak +462-484; birch +494-530; spruce +467-491; Laurelin +481-526; Telperion +428-543 MB (F4 770-826 MB) | ×2.22-3.16 | none | botanical growth (2026-10-13) |
| L9 | Palm device memory | +2.0 / +1.9 / +2.0 MiB | ×1.02-1.03 | none | Palm: approve. 0.1.4's palm is a CPU mesh. |
| L10 | Ordinary device memory, seeds 7 and 42 | +2.4 / +11.8 MiB | ×1.03 / ×1.13 | none | Ordinary: approve |
| L11 | Laurelin device memory | +162.4 / +288.5 / +213.9 MiB (F4 315-487 MiB) | ×1.78-2.57 | none | as L2 |
| L12 | Telperion device memory | +65.5 / +360.0 / +369.1 MiB (F4 317-488 MiB) | ×1.26-4.13 | none | as L4 |
| L13 | Palm field call (warm) | +79 / +76 / +72 ms (91 / 87 / 83 against 12 / 11 / 11) | ×7.52-8.03 | none | Palm: approve |
| L14 | Ordinary field call | +314 / +342 / +343 ms (371-397 against 54-57) | ×6.50-7.35 | none | Ordinary: approve |
| L15 | Oak field call | +82 / +64 / +57 ms (233 / 253 / 230 against 151 / 189 / 173) | ×1.32-1.55 | none | Oak: needs work (seed 7 a release item) |
| L16 | Birch field call, seed 42 | +15 ms (215 against 200) | ×1.08 | none | Birch: "it's the best species" |
| L17 | Spruce field call | +274 / +273 / +274 ms (426 / 416 / 421 against 152 / 143 / 147) | ×2.81-2.91 | open: the spruce tolerance (2026-10-16) accepted "about 0.35 s against 0.24 s" | Spruce: "Looks really good" |
| L18 | Laurelin field call | +683 / +839 / +736 ms (825-1,134 against 142-398) | ×2.85-5.81 | none | as L2 |
| L19 | Telperion field call | +427 / +1,119 / +1,204 ms (876-1,248 against 45-448) | ×1.95-28.02 | none | as L4 |
| L20 | Download, `telperion.wasm` | gzip +57,753, brotli +44,333 B (raw +132,512, under its ceiling) | gzip ×1.14, brotli ×1.14 | open: do the raw ceilings cover compressed growth? | the engine for every species (botanical growth) |
| L21 | Download, `telperion-render.wasm` | gzip +472,729, brotli +353,793 B (raw +1,499,523, under its ceiling) | ×1.78 / ×1.79 | open, as L20 | as L20 |
| L22 | Download, `wasm-source.js` | gzip +15,356, brotli +13,040 B (raw +64,202, under its ceiling) | ×77.78 / ×85.13 | open, as L20 | as L20 |
| L23 | Download, all artifacts and the tarball | total raw +1,425,947 / gzip +409,828 / brotli +296,158 B; tarball +422,936 B | ×1.39 / ×1.35 / ×1.33; ×1.35 | none | as L20. `AUDIT.md`: the name sections are 15.8 % and 15.3 % of the two modules, a size question for fn-242. |
| L24 | Matched age: F4 refuses the oak, spruce, Laurelin and Telperion at 100 | the oak and the spruce: "cycle state axes" buffer 2,558,845,952 and 2,513,984,512 bytes over the 2,147,483,644 limit; Laurelin and Telperion: over the 20,000,000-phytomer budget. 0.1.4 draws one mature tree at any age. | – | none | the owner on size (2026-10-15): "we're not efficient enough with the generator to make large species at full fidelity", plan fn-285 after the release |

Covered by an owner tolerance, no question:

- warm builds under 0.5 s on the palm, ordinary, oak, birch and spruce;
- Laurelin's 942 ms and Telperion's 988 ms at seed 1, under about 1.25 s;
- every artifact's raw bytes, under the ceilings raised on 2026-10-16.

`telperion.js` grew +75,475 B raw but is smaller gzip (−4,632) and brotli (−7,095).

## Questions for the owner

Each question has a place for the answer. The answers are the owner's words, recorded when they come, with the date.

### Tolerances (raised by tasks 4 and 5)

1. **T1.** Does R0a's "about 1.25 s" for Laurelin cover Laurelin's 1,272 ms warm build at seed 7? And does anything cover Laurelin's 1,346 ms at seed 42, and Telperion's 1,356 ms and 1,413 ms at seeds 7 and 42 (the owner accepted 1.33 s at seed 42 on 2026-10-16)?
   Answer: accepted (owner, 2026-10-19, review page https://claude.ai/artifact/APHF4DkshoeREwbGMKbe6s).
2. **T2.** Does the spruce field tolerance ("about 0.35 s against 0.24 s", 2026-10-16) reach F4's 0.42 s against 0.1.4's 0.15 s measured natively (426 / 416 / 421 ms against 152 / 143 / 147 ms)?
   Answer: accepted (owner, 2026-10-19, review page https://claude.ai/artifact/APHF4DkshoeREwbGMKbe6s).
3. **T3.** Do the raw-byte artifact ceilings raised on 2026-10-16 also cover the gzip and brotli growth: `telperion.wasm` +57,753 / +44,333 B, `telperion-render.wasm` +472,729 / +353,793 B, `wasm-source.js` +15,356 / +13,040 B, total +409,828 / +296,158 B?
   Answer: accepted (owner, 2026-10-19, review page https://claude.ai/artifact/APHF4DkshoeREwbGMKbe6s).

### Old versus new, per species

Each species question asks whether F4 at its product default age is at least as good as 0.1.4 on the sheet, and whether that holds whatever its performance rows say. The matched-age sheet is a diagnostic beside the default-age one.

4. **S1, ordinary** (`sheets/ordinary.jpg`, `sheets/ordinary-matched-age-100.jpg`). Is F4 at 40 at least as good as 0.1.4?
   Answer: accepted (owner, 2026-10-19, review page https://claude.ai/artifact/APHF4DkshoeREwbGMKbe6s).
5. **S2, date palm** (`sheets/date-palm.jpg`, `sheets/date-palm-matched-age-100.jpg`; 0.1.4's palm is a CPU mesh). Is F4 at 40 at least as good as 0.1.4?
   Answer: accepted (owner, 2026-10-19, review page https://claude.ai/artifact/APHF4DkshoeREwbGMKbe6s).
6. **S3, Oregon white oak** (`sheets/oregon-white-oak.jpg`; F4 at 100 refused). Is F4 at 40 at least as good as 0.1.4? Seed 7 is the release item of 2026-10-15.
   Answer: accepted (owner, 2026-10-19, review page https://claude.ai/artifact/APHF4DkshoeREwbGMKbe6s).
7. **S4, silver birch** (`sheets/silver-birch.jpg`, `sheets/silver-birch-matched-age-70.jpg`). Is F4 at 40 at least as good as 0.1.4?
   Answer: accepted (owner, 2026-10-19, review page https://claude.ai/artifact/APHF4DkshoeREwbGMKbe6s).
8. **S5, Norway spruce** (`sheets/norway-spruce.jpg`; F4 at 100 refused). Is F4 at 40 at least as good as 0.1.4?
   Answer: accepted (owner, 2026-10-19, review page https://claude.ai/artifact/APHF4DkshoeREwbGMKbe6s).
9. **S6, Laurelin** (`sheets/laurelin.jpg`; F4 at 100 refused). Is F4 at 80 at least as good as 0.1.4?
   Answer: accepted (owner, 2026-10-19, review page https://claude.ai/artifact/APHF4DkshoeREwbGMKbe6s).
10. **S7, Telperion** (`sheets/telperion.jpg`; F4 at 100 refused; 0.1.4's Telperion at seeds 7 and 42 is small). Is F4 at 82 at least as good as 0.1.4?
    Answer: accepted (owner, 2026-10-19, review page https://claude.ai/artifact/APHF4DkshoeREwbGMKbe6s).
11. **S8, European beech** (`sheets/european-beech.jpg`, F4 only). Does F4's beech at 40 ship in 0.2.0 with no 0.1.4 to compare against (2026-10-15: "needs work, after the release")?
    Answer: accepted (owner, 2026-10-19, review page https://claude.ai/artifact/APHF4DkshoeREwbGMKbe6s).

### The ledger, per item

Each ledger question asks whether the item ships in 0.2.0 at its measured size for the fidelity gain named, or holds the species or the artifact back.

12. **L1.** Fresh build +1.7-2.8 s on every preset: accepted?
    Answer: accepted (owner, 2026-10-19, review page https://claude.ai/artifact/APHF4DkshoeREwbGMKbe6s).
13. **L2.** Laurelin warm build 1,272 ms at seed 7 (see T1): accepted?
    Answer: accepted (owner, 2026-10-19, review page https://claude.ai/artifact/APHF4DkshoeREwbGMKbe6s).
14. **L3.** Laurelin warm build 1,346 ms at seed 42: accepted?
    Answer: accepted (owner, 2026-10-19, review page https://claude.ai/artifact/APHF4DkshoeREwbGMKbe6s).
15. **L4.** Telperion warm build 1,356 ms and 1,413 ms at seeds 7 and 42: accepted?
    Answer: accepted (owner, 2026-10-19, review page https://claude.ai/artifact/APHF4DkshoeREwbGMKbe6s).
16. **L5.** Ordinary frames +0.05-0.48 ms (×1.10-1.77): accepted?
    Answer: accepted (owner, 2026-10-19, in chat after discussing: "4 ok").
17. **L6.** Laurelin frames +4.3-26.6 ms (×2.50-13.96): accepted?
    Answer: accepted for now (owner, 2026-10-19, in chat after discussing: "ok we can accept for now").
18. **L7.** Telperion frames +2.7-21.7 ms (×2.00-45.38): accepted?
    Answer: accepted for now (owner, 2026-10-19, in chat after discussing: "ok we can accept for now").
19. **L8.** CPU peak +428-543 MB on every preset: accepted?
    Answer: accepted (owner, 2026-10-19, "ok"), after `CPU-PEAK.md` found the excess is the NVIDIA driver compiling 206 pipelines on one thread each, not tree-sized and not paid by a page in a browser; capping `Program::make_many` to a small pool (about −300 MB natively, start time to be measured) goes after the release.
20. **L9.** Palm device memory +2 MiB: accepted?
    Answer: accepted (owner, 2026-10-19, review page https://claude.ai/artifact/APHF4DkshoeREwbGMKbe6s).
21. **L10.** Ordinary device memory +2.4 / +11.8 MiB at seeds 7 / 42: accepted?
    Answer: accepted (owner, 2026-10-19, review page https://claude.ai/artifact/APHF4DkshoeREwbGMKbe6s).
22. **L11.** Laurelin device memory +162-289 MiB: accepted?
    Answer: accepted for now (owner, 2026-10-19, in chat after discussing: "ok we can accept for now").
23. **L12.** Telperion device memory +66-369 MiB: accepted?
    Answer: accepted for now (owner, 2026-10-19, in chat after discussing: "ok we can accept for now").
24. **L13.** Palm field call +72-79 ms (×7.5-8.0): accepted?
    Answer: open (owner, 2026-10-19: "discuss"). A field call now grows the tree first (fn-257 R8), so its cost is growth speed; the owner asked whether fn-285 should come into 0.2.0. Pending.
25. **L14.** Ordinary field call +314-343 ms (×6.5-7.4): accepted?
    Answer: open (owner, 2026-10-19: "discuss"). A field call now grows the tree first (fn-257 R8), so its cost is growth speed; the owner asked whether fn-285 should come into 0.2.0. Pending.
26. **L15.** Oak field call +57-82 ms (×1.32-1.55): accepted?
    Answer: open (owner, 2026-10-19: "discuss"). A field call now grows the tree first (fn-257 R8), so its cost is growth speed; the owner asked whether fn-285 should come into 0.2.0. Pending.
27. **L16.** Birch field call +15 ms at seed 42: accepted?
    Answer: open (owner, 2026-10-19: "discuss"). A field call now grows the tree first (fn-257 R8), so its cost is growth speed; the owner asked whether fn-285 should come into 0.2.0. Pending.
28. **L17.** Spruce field call +273-274 ms (see T2): accepted?
    Answer: accepted (owner, 2026-10-19, "ok accept it"), as T2. The 2026-10-18 acceptance was the spruce at about 0.44 s on the site-field measurement, with the site keeping the spruce on 0.1.4 until fn-285.
29. **L18.** Laurelin field call +683-839 ms: accepted?
    Answer: open (owner, 2026-10-19: "discuss"). A field call now grows the tree first (fn-257 R8), so its cost is growth speed; the owner asked whether fn-285 should come into 0.2.0. Pending.
30. **L19.** Telperion field call +427-1,204 ms: accepted?
    Answer: open (owner, 2026-10-19: "discuss"). A field call now grows the tree first (fn-257 R8), so its cost is growth speed; the owner asked whether fn-285 should come into 0.2.0. Pending.
31. **L20.** `telperion.wasm` gzip +57,753 / brotli +44,333 B (see T3): accepted?
    Answer: accepted (owner, 2026-10-19, review page https://claude.ai/artifact/APHF4DkshoeREwbGMKbe6s).
32. **L21.** `telperion-render.wasm` gzip +472,729 / brotli +353,793 B: accepted?
    Answer: accepted (owner, 2026-10-19, review page https://claude.ai/artifact/APHF4DkshoeREwbGMKbe6s).
33. **L22.** `wasm-source.js` gzip +15,356 / brotli +13,040 B: accepted?
    Answer: accepted (owner, 2026-10-19, review page https://claude.ai/artifact/APHF4DkshoeREwbGMKbe6s).
34. **L23.** Package total +409,828 B gzip / +296,158 B brotli, tarball +422,936 B: accepted?
    Answer: accepted (owner, 2026-10-19, review page https://claude.ai/artifact/APHF4DkshoeREwbGMKbe6s).
35. **L24.** F4 refuses the oak, spruce, Laurelin and Telperion at age 100, where 0.1.4 draws a mature tree: accepted for 0.2.0, with fn-285 after it?
    Answer: accepted (owner, 2026-10-19, review page https://claude.ai/artifact/APHF4DkshoeREwbGMKbe6s).
