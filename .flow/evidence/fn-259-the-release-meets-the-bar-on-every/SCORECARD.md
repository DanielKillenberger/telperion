# fn-259 R1: the scorecard, F4 beside 0.1.4

Measured 2026-10-19, 04:55-05:45, under `PROTOCOL.md`. It was committed in `dd09b47b9` before the first row, amended in `196062203` before the matched-age rows, and amended in `e27b07876` after review, before the field rows were rerun. The setup is the identities, hardware and route stated there. Raw rows: `raw/t4-oak-rows.jsonl`. Progress: `raw/t4-oak-progress.txt`. Byte sizes: `raw/t4-oak-bytes.txt`. Protocol check: `raw/t4-checks.jsonl`. All raw files are ignored. Task 5 measured the other seven presets on the same protocol (amendment 5) and set them below the oak. Their raw rows, progress files, runner and logs are `raw/t4-<preset>-*` and `raw/t5-*`. The European beech is reported F4-only, because 0.1.4 did not ship it in its package catalogue (host decision, 2026-10-19).

Each cell is the median of the valid samples, [min, max] (n), F4 against 0.1.4. **Bold** marks F4 worse than 0.1.4.

## Oregon white oak at the product default (F4 age 40; 0.1.4 the mature direct build)

| Column | Seed | F4 | 0.1.4 | Verdict |
|---|--:|--:|--:|---|
| Build, fresh ms | 1 | **4,501** [4,354, 4,628] (5) | 2,599 [2,568, 2,641] (5) | regression, +1,902 |
| | 7 | **4,451** [4,375, 4,559] (5) | 2,617 [2,586, 2,910] (5) | regression, +1,834 |
| | 42 | **4,574** [4,409, 4,636] (5) | 2,624 [2,520, 2,739] (5) | regression, +1,950 |
| Build, warm ms | 1 | **243** [242, 247] (5) | 129 [128, 142] (5) | covered: under the 0.5 s budget |
| | 7 | **259** [259, 269] (5) | 158 [148, 191] (5) | covered |
| | 42 | **243** [241, 245] (5) | 164 [151, 170] (5) | covered |
| Frame ms, whole | 1 / 7 / 42 | 2.39 / 3.30 / 2.79 | 16.4 / 15.6 / 16.3 | no worse |
| Frame ms, mid | 1 / 7 / 42 | 5.68 / 8.03 / 6.66 | 29.0 / 33.7 / 33.0 | no worse |
| Frame ms, close | 1 / 7 / 42 | 8.01 / 10.4 / 10.1 | 25.3 / 28.2 / 32.0 | no worse |
| CPU peak MB (one build) | 1 | **783** [746, 794] (3) | 299 [298, 300] (3) | regression, +484 |
| | 7 | **775** [760, 798] (3) | 313 [313, 314] (3) | regression, +462 |
| | 42 | **773** [750, 802] (3) | 305 [305, 307] (3) | regression, +468 |
| Device MiB, allocated less wgpu-internal (one build) | 1 / 7 / 42 | 137 / 163 / 141 (3 each, no spread) | 319 / 356 / 347 | no worse |
| Field call, warm ms | 1 | **233** [229, 246] (5) | 151 [148, 164] (5) | regression, +82 |
| | 7 | **253** [251, 265] (5) | 189 [187, 214] (5) | regression, +64 |
| | 42 | **230** [229, 236] (5) | 173 [167, 180] (5) | regression, +57 |

Frame-time ranges are within 4 % of each median on both sides (`raw/`). Reported beside the counted columns, not counted:

- **Device reserved:** 512 MiB on both sides at every seed (one build), and 768 on both after the multi-build run.
- **wgpu-internal:** 6.5 MiB on both after one build.
- **Multi-build peak RSS:** F4 871-881 MB, 0.1.4 362-374.
- **Multi-build allocated less internal:** F4 146-182 MiB, 0.1.4 319-356.
- **The field's first call in a process:** F4 231-258 ms, 0.1.4 165-190 (medians).
- **F4's field start:** the device, the generator and `prepare_programs`, once a page, 2,285 [2,202, 2,307] ms (5).
- **Superseded field rows:** the first field rows (`f-` jobs) are not counted (PROTOCOL.md amendment 2). 0.1.4's probe had been built with the core's `geometry` feature, and F4's start had left out `prepare_programs`. Their warm medians were within 6 ms of the rerun's at every seed.
- **0.1.4's route:** every 0.1.4 build reported backend `Gpu` for foliage and wood, so nothing fell back to the CPU.

The two oaks are different trees. 0.1.4's is the mature specimen, and its 36-cell field marks 10,592-11,333 wood cells and 11,707-12,393 foliage cells. F4's oak at 40 marks 3,814-4,540 and 3,836-4,613.

## Date palm at the product default (F4 age 40; 0.1.4 the mature direct build)

| Column | Seed | F4 | 0.1.4 | Verdict |
|---|--:|--:|--:|---|
| Build, fresh ms | 1 | **4,190 [4,152, 4,297] (5)** | 2,436 [2,374, 2,607] (5) | regression, +1,754 (×1.72) |
|  | 7 | **4,254 [4,189, 4,273] (5)** | 2,414 [2,348, 2,434] (5) | regression, +1,839 (×1.76) |
|  | 42 | **4,259 [4,161, 4,949] (5)** | 2,431 [2,371, 2,821] (5) | regression, +1,828 (×1.75) |
| Build, warm ms | 1 | **90 [88, 105] (5)** | 6 [6, 6] (5) | covered: under the 0.5 s budget, +84 (×15.43) |
|  | 7 | **94 [89, 137] (5)** | 6 [6, 6] (5) | covered: under the 0.5 s budget, +89 (×17.11) |
|  | 42 | **93 [89, 122] (5)** | 6 [6, 6] (5) | covered: under the 0.5 s budget, +87 (×15.98) |
| Frame ms, whole | 1 / 7 / 42 | 0.61 / 0.58 / 0.57 | 0.85 / 0.82 / 0.85 | no worse |
| Frame ms, mid | 1 / 7 / 42 | 0.59 / 0.60 / 0.61 | 1.62 / 1.55 / 1.61 | no worse |
| Frame ms, close | 1 / 7 / 42 | 0.79 / 0.78 / 0.78 | 2.19 / 2.11 / 2.12 | no worse |
| CPU peak MB (one build) | 1 | **780 [777, 791] (3)** | 248 [248, 250] (3) | regression, +532 (×3.15) |
|  | 7 | **776 [773, 778] (3)** | 249 [247, 249] (3) | regression, +527 (×3.12) |
|  | 42 | **780 [766, 784] (3)** | 247 [247, 249] (3) | regression, +533 (×3.16) |
| Device MiB, allocated less wgpu-internal (one build) | 1 | **79.6 [79.6, 79.6] (3)** | 77.6 [77.6, 77.6] (3) | regression, +2.0 (×1.03) |
|  | 7 | **79.6 [79.6, 79.6] (3)** | 77.7 [77.7, 77.7] (3) | regression, +1.9 (×1.02) |
|  | 42 | **79.6 [79.6, 79.6] (3)** | 77.6 [77.6, 77.6] (3) | regression, +2.0 (×1.03) |
| Field call, warm ms | 1 | **91 [86, 96] (5)** | 12 [12, 13] (5) | regression, +79 (×7.52) |
|  | 7 | **87 [86, 88] (5)** | 11 [11, 11] (5) | regression, +76 (×8.03) |
|  | 42 | **83 [81, 84] (5)** | 11 [11, 11] (5) | regression, +72 (×7.59) |

Beside the counted columns (medians, seeds 1 / 7 / 42): multi-build peak RSS F4 848 / 854 / 849, 0.1.4 357 / 358 / 357 MB; field's first call F4 99 / 90 / 82, 0.1.4 13 / 11 / 11 ms; F4's field start 2,326 [2,247, 2,398] (5) ms.

## Ordinary at the product default (F4 age 40; 0.1.4 the mature direct build)

| Column | Seed | F4 | 0.1.4 | Verdict |
|---|--:|--:|--:|---|
| Build, fresh ms | 1 | **4,606 [4,510, 4,772] (5)** | 2,462 [2,407, 2,502] (5) | regression, +2,144 (×1.87) |
|  | 7 | **4,587 [4,494, 4,693] (5)** | 2,461 [2,416, 2,480] (5) | regression, +2,127 (×1.86) |
|  | 42 | **4,598 [4,492, 4,677] (5)** | 2,482 [2,412, 2,628] (5) | regression, +2,117 (×1.85) |
| Build, warm ms | 1 | **371 [367, 380] (5)** | 50 [49, 51] (5) | covered: under the 0.5 s budget, +321 (×7.40) |
|  | 7 | **394 [391, 400] (5)** | 51 [50, 51] (5) | covered: under the 0.5 s budget, +343 (×7.79) |
|  | 42 | **402 [399, 406] (5)** | 50 [49, 52] (5) | covered: under the 0.5 s budget, +352 (×8.05) |
| Frame ms, whole | 1 | **0.56 [0.56, 0.57] (5)** | 0.51 [0.51, 0.51] (5) | regression, +0.05 (×1.10) |
|  | 7 | **0.66 [0.66, 0.67] (5)** | 0.50 [0.50, 0.54] (5) | regression, +0.16 (×1.32) |
|  | 42 | **0.73 [0.73, 0.74] (5)** | 0.46 [0.46, 0.46] (5) | regression, +0.27 (×1.58) |
| Frame ms, mid | 1 | 0.74 [0.74, 0.77] (5) | 0.78 [0.75, 0.82] (5) | no worse |
|  | 7 | **0.88 [0.87, 0.89] (5)** | 0.65 [0.62, 0.69] (5) | regression, +0.23 (×1.35) |
|  | 42 | **0.99 [0.99, 1.00] (5)** | 0.63 [0.62, 0.65] (5) | regression, +0.36 (×1.57) |
| Frame ms, close | 1 | 0.89 [0.88, 0.93] (5) | 0.89 [0.88, 0.93] (5) | no worse |
|  | 7 | **0.95 [0.94, 0.96] (5)** | 0.66 [0.66, 0.69] (5) | regression, +0.29 (×1.43) |
|  | 42 | **1.10 [1.09, 1.10] (5)** | 0.62 [0.61, 0.62] (5) | regression, +0.48 (×1.77) |
| CPU peak MB (one build) | 1 | **778 [757, 783] (3)** | 261 [255, 262] (3) | regression, +517 (×2.98) |
|  | 7 | **771 [743, 782] (3)** | 261 [255, 263] (3) | regression, +510 (×2.95) |
|  | 42 | **771 [721, 787] (3)** | 255 [254, 261] (3) | regression, +516 (×3.02) |
| Device MiB, allocated less wgpu-internal (one build) | 1 | 90.6 [90.6, 90.6] (3) | 95.0 [95.0, 95.0] (3) | no worse |
|  | 7 | **98.0 [98.0, 98.0] (3)** | 95.6 [95.6, 95.6] (3) | regression, +2.4 (×1.03) |
|  | 42 | **102.7 [102.7, 102.7] (3)** | 90.9 [90.9, 90.9] (3) | regression, +11.8 (×1.13) |
| Field call, warm ms | 1 | **371 [369, 373] (5)** | 57 [56, 58] (5) | regression, +314 (×6.50) |
|  | 7 | **396 [393, 418] (5)** | 55 [54, 56] (5) | regression, +342 (×7.23) |
|  | 42 | **397 [393, 405] (5)** | 54 [54, 57] (5) | regression, +343 (×7.35) |

Beside the counted columns (medians, seeds 1 / 7 / 42): multi-build peak RSS F4 860 / 889 / 930, 0.1.4 345 / 338 / 336 MB; field's first call F4 393 / 396 / 398, 0.1.4 61 / 55 / 55 ms; F4's field start 2,263 [2,244, 2,358] (5) ms.

## European beech at the product default (F4 age 40), F4 only

| Column | Seed | F4 |
|---|--:|--:|
| Build, fresh ms | 1 | 4,388 [4,366, 5,070] (5) |
|  | 7 | 4,427 [4,369, 4,444] (5) |
|  | 42 | 4,360 [4,258, 4,402] (5) |
| Build, warm ms | 1 | 174 [173, 177] (5) |
|  | 7 | 191 [189, 192] (5) |
|  | 42 | 219 [211, 231] (5) |
| Frame ms, whole | 1 | 1.43 [1.42, 1.43] (5) |
|  | 7 | 2.44 [2.44, 2.45] (5) |
|  | 42 | 2.68 [2.63, 2.73] (5) |
| Frame ms, mid | 1 | 2.28 [2.23, 2.33] (5) |
|  | 7 | 3.42 [3.41, 3.43] (5) |
|  | 42 | 4.14 [4.13, 4.24] (5) |
| Frame ms, close | 1 | 2.53 [2.45, 2.56] (5) |
|  | 7 | 2.48 [2.47, 2.49] (5) |
|  | 42 | 3.57 [3.56, 3.57] (5) |
| CPU peak MB (one build) | 1 | 774 [773, 777] (3) |
|  | 7 | 786 [758, 792] (3) |
|  | 42 | 801 [776, 826] (3) |
| Device MiB, allocated less wgpu-internal (one build) | 1 | 97.6 [97.6, 97.6] (3) |
|  | 7 | 125.4 [125.4, 125.4] (3) |
|  | 42 | 133.4 [133.4, 133.4] (3) |
| Field call, warm ms | 1 | 165 [158, 166] (5) |
|  | 7 | 178 [177, 186] (5) |
|  | 42 | 192 [190, 202] (5) |

Beside the counted columns (medians, seeds 1 / 7 / 42): multi-build peak RSS F4 863 / 884 / 931 MB; field's first call F4 181 / 180 / 201 ms; F4's field start 2,271 [2,179, 2,425] (5) ms.

## Silver birch at the product default (F4 age 40; 0.1.4 the mature direct build)

| Column | Seed | F4 | 0.1.4 | Verdict |
|---|--:|--:|--:|---|
| Build, fresh ms | 1 | **4,434 [4,411, 4,464] (5)** | 2,617 [2,560, 2,641] (5) | regression, +1,817 (×1.69) |
|  | 7 | **4,474 [4,442, 4,678] (5)** | 2,797 [2,748, 2,957] (5) | regression, +1,677 (×1.60) |
|  | 42 | **4,454 [4,428, 4,476] (5)** | 2,635 [2,626, 2,775] (5) | regression, +1,819 (×1.69) |
| Build, warm ms | 1 | **210 [209, 217] (5)** | 170 [165, 174] (5) | covered: under the 0.5 s budget, +40 (×1.24) |
|  | 7 | 243 [241, 253] (5) | 337 [316, 359] (5) | no worse |
|  | 42 | **228 [226, 231] (5)** | 167 [157, 172] (5) | covered: under the 0.5 s budget, +62 (×1.37) |
| Frame ms, whole | 1 / 7 / 42 | 1.45 / 1.93 / 1.60 | 7.11 / 8.83 / 6.88 | no worse |
| Frame ms, mid | 1 / 7 / 42 | 2.60 / 3.34 / 3.04 | 21.63 / 30.17 / 20.85 | no worse |
| Frame ms, close | 1 / 7 / 42 | 3.85 / 5.19 / 4.24 | 36.06 / 37.87 / 36.24 | no worse |
| CPU peak MB (one build) | 1 | **783 [781, 802] (3)** | 267 [266, 267] (3) | regression, +516 (×2.93) |
|  | 7 | **812 [766, 819] (3)** | 282 [281, 282] (3) | regression, +530 (×2.88) |
|  | 42 | **772 [733, 793] (3)** | 278 [278, 280] (3) | regression, +494 (×2.78) |
| Device MiB, allocated less wgpu-internal (one build) | 1 / 7 / 42 | 123.9 / 144.0 / 133.9 | 204.5 / 230.4 / 212.8 | no worse |
| Field call, warm ms | 1 | 200 [197, 203] (5) | 208 [207, 210] (5) | no worse |
|  | 7 | 244 [243, 248] (5) | 365 [361, 370] (5) | no worse |
|  | 42 | **215 [213, 222] (5)** | 200 [198, 205] (5) | regression, +15 (×1.08) |

Beside the counted columns (medians, seeds 1 / 7 / 42): multi-build peak RSS F4 901 / 919 / 887, 0.1.4 368 / 361 / 362 MB; field's first call F4 223 / 248 / 219, 0.1.4 216 / 364 / 206 ms; F4's field start 2,270 [2,216, 2,288] (5) ms.

## Norway spruce at the product default (F4 age 40; 0.1.4 the mature direct build)

| Column | Seed | F4 | 0.1.4 | Verdict |
|---|--:|--:|--:|---|
| Build, fresh ms | 1 | **4,675 [4,650, 4,784] (5)** | 2,617 [2,592, 2,657] (5) | regression, +2,058 (×1.79) |
|  | 7 | **4,663 [4,639, 4,828] (5)** | 2,679 [2,569, 2,702] (5) | regression, +1,984 (×1.74) |
|  | 42 | **4,667 [4,649, 4,835] (5)** | 2,574 [2,520, 2,694] (5) | regression, +2,093 (×1.81) |
| Build, warm ms | 1 | **461 [460, 478] (5)** | 161 [141, 169] (5) | covered: under the 0.5 s budget, +300 (×2.86) |
|  | 7 | **454 [451, 468] (5)** | 146 [134, 151] (5) | covered: under the 0.5 s budget, +308 (×3.11) |
|  | 42 | **459 [455, 469] (5)** | 140 [132, 160] (5) | covered: under the 0.5 s budget, +318 (×3.27) |
| Frame ms, whole | 1 / 7 / 42 | 7.08 / 6.78 / 6.50 | 23.29 / 21.88 / 18.42 | no worse |
| Frame ms, mid | 1 / 7 / 42 | 9.79 / 8.94 / 8.75 | 22.98 / 22.16 / 18.51 | no worse |
| Frame ms, close | 1 / 7 / 42 | 7.65 / 7.74 / 7.42 | 13.31 / 13.37 / 12.04 | no worse |
| CPU peak MB (one build) | 1 | **797 [792, 801] (3)** | 306 [303, 307] (3) | regression, +491 (×2.60) |
|  | 7 | **772 [748, 805] (3)** | 303 [302, 308] (3) | regression, +469 (×2.55) |
|  | 42 | **770 [763, 799] (3)** | 303 [302, 305] (3) | regression, +467 (×2.54) |
| Device MiB, allocated less wgpu-internal (one build) | 1 / 7 / 42 | 424.6 / 413.2 / 427.7 | 473.7 / 454.1 / 458.4 | no worse |
| Field call, warm ms | 1 | **426 [423, 435] (5)** | 152 [150, 154] (5) | regression, +274 (×2.81): the spruce tolerance accepted about 0.35 s; F4 is over it |
|  | 7 | **416 [413, 421] (5)** | 143 [139, 147] (5) | regression, +273 (×2.91): the spruce tolerance accepted about 0.35 s; F4 is over it |
|  | 42 | **421 [420, 423] (5)** | 147 [142, 151] (5) | regression, +274 (×2.86): the spruce tolerance accepted about 0.35 s; F4 is over it |

Beside the counted columns (medians, seeds 1 / 7 / 42): multi-build peak RSS F4 939 / 940 / 928, 0.1.4 364 / 364 / 364 MB; field's first call F4 434 / 417 / 423, 0.1.4 170 / 145 / 147 ms; F4's field start 2,355 [2,211, 2,368] (5) ms.

## Laurelin at the product default (F4 age 80; 0.1.4 the mature direct build)

| Column | Seed | F4 | 0.1.4 | Verdict |
|---|--:|--:|--:|---|
| Build, fresh ms | 1 | **5,099 [5,043, 5,251] (5)** | 2,620 [2,587, 2,644] (5) | regression, +2,479 (×1.95) |
|  | 7 | **5,332 [5,219, 5,413] (5)** | 2,741 [2,721, 2,925] (5) | regression, +2,591 (×1.95) |
|  | 42 | **5,413 [5,374, 5,652] (5)** | 2,997 [2,932, 3,018] (5) | regression, +2,416 (×1.81) |
| Build, warm ms | 1 | **942 [932, 950] (5)** | 186 [182, 192] (5) | covered: under the 1.25 s budget, +757 (×5.07) |
|  | 7 | **1,272 [1,264, 1,273] (5)** | 308 [305, 340] (5) | regression, +964 (×4.13): over 1.25 s; whether "about 1.25 s" reaches 1.27 s is the owner's call |
|  | 42 | **1,346 [1,328, 1,387] (5)** | 565 [537, 573] (5) | regression, +782 (×2.38): over 1.25 s and over the 1.33 s accepted for Telperion |
| Frame ms, whole | 1 | **5.70 [5.67, 5.93] (5)** | 1.44 [1.41, 1.44] (5) | regression, +4.26 (×3.96) |
|  | 7 | **8.55 [8.34, 8.80] (5)** | 1.89 [1.85, 2.38] (5) | regression, +6.66 (×4.54) |
|  | 42 | **8.86 [8.83, 8.89] (5)** | 3.54 [3.49, 4.11] (5) | regression, +5.32 (×2.50) |
| Frame ms, mid | 1 | **14.64 [14.58, 15.08] (5)** | 1.63 [1.62, 1.64] (5) | regression, +13.01 (×8.99) |
|  | 7 | **28.68 [25.79, 30.42] (5)** | 2.05 [2.04, 2.15] (5) | regression, +26.63 (×13.96) |
|  | 42 | **26.45 [26.14, 26.50] (5)** | 3.88 [3.87, 3.93] (5) | regression, +22.57 (×6.82) |
| Frame ms, close | 1 | **13.50 [13.46, 13.76] (5)** | 1.30 [1.30, 1.37] (5) | regression, +12.20 (×10.37) |
|  | 7 | **22.88 [20.90, 23.91] (5)** | 1.71 [1.70, 1.75] (5) | regression, +21.18 (×13.40) |
|  | 42 | **20.17 [19.97, 20.30] (5)** | 2.88 [2.86, 2.91] (5) | regression, +17.29 (×7.00) |
| CPU peak MB (one build) | 1 | **795 [795, 796] (3)** | 273 [272, 275] (3) | regression, +522 (×2.91) |
|  | 7 | **819 [791, 839] (3)** | 293 [292, 293] (3) | regression, +526 (×2.80) |
|  | 42 | **826 [824, 831] (3)** | 345 [345, 345] (3) | regression, +481 (×2.39) |
| Device MiB, allocated less wgpu-internal (one build) | 1 | **314.7 [314.7, 314.7] (3)** | 152.3 [152.3, 152.3] (3) | regression, +162.4 (×2.07) |
|  | 7 | **472.1 [472.1, 472.1] (3)** | 183.6 [183.6, 183.6] (3) | regression, +288.5 (×2.57) |
|  | 42 | **486.5 [486.5, 486.5] (3)** | 272.6 [272.6, 272.6] (3) | regression, +213.9 (×1.78) |
| Field call, warm ms | 1 | **825 [815, 833] (5)** | 142 [140, 155] (5) | regression, +683 (×5.81) |
|  | 7 | **1,071 [1,067, 1,113] (5)** | 232 [228, 249] (5) | regression, +839 (×4.61) |
|  | 42 | **1,134 [1,131, 1,160] (5)** | 398 [385, 425] (5) | regression, +736 (×2.85) |

Beside the counted columns (medians, seeds 1 / 7 / 42): multi-build peak RSS F4 973 / 1,030 / 1,001, 0.1.4 349 / 350 / 363 MB; field's first call F4 832 / 1,081 / 1,136, 0.1.4 149 / 234 / 402 ms; F4's field start 2,280 [2,207, 2,385] (5) ms.

## Telperion at the product default (F4 age 82; 0.1.4 the mature direct build)

| Column | Seed | F4 | 0.1.4 | Verdict |
|---|--:|--:|--:|---|
| Build, fresh ms | 1 | **5,201 [5,084, 5,860] (5)** | 3,014 [2,969, 3,220] (5) | regression, +2,186 (×1.73) |
|  | 7 | **5,356 [5,326, 5,590] (5)** | 2,580 [2,492, 2,730] (5) | regression, +2,776 (×2.08) |
|  | 42 | **5,424 [5,351, 5,602] (5)** | 2,579 [2,478, 2,707] (5) | regression, +2,845 (×2.10) |
| Build, warm ms | 1 | **988 [980, 1,021] (5)** | 613 [589, 622] (5) | covered: under the 1.25 s budget, +375 (×1.61) |
|  | 7 | **1,356 [1,347, 1,391] (5)** | 97 [95, 101] (5) | regression, +1,259 (×13.95): over 1.33 s |
|  | 42 | **1,413 [1,391, 1,435] (5)** | 83 [80, 84] (5) | regression, +1,330 (×17.02): over the 1.33 s the owner accepted at this seed |
| Frame ms, whole | 1 | **5.40 [5.38, 5.49] (5)** | 2.70 [2.33, 3.10] (5) | regression, +2.71 (×2.00) |
|  | 7 | **7.85 [7.84, 8.01] (5)** | 0.71 [0.71, 0.71] (5) | regression, +7.14 (×11.08) |
|  | 42 | **8.71 [8.54, 8.85] (5)** | 0.74 [0.74, 0.77] (5) | regression, +7.97 (×11.74) |
| Frame ms, mid | 1 | **11.87 [11.82, 12.29] (5)** | 1.60 [1.59, 1.60] (5) | regression, +10.28 (×7.44) |
|  | 7 | **19.14 [19.03, 19.62] (5)** | 0.48 [0.48, 0.53] (5) | regression, +18.65 (×39.62) |
|  | 42 | **22.28 [21.47, 22.39] (5)** | 0.55 [0.54, 0.58] (5) | regression, +21.74 (×40.88) |
| Frame ms, close | 1 | **11.52 [11.48, 11.68] (5)** | 1.62 [1.62, 1.63] (5) | regression, +9.90 (×7.10) |
|  | 7 | **16.34 [16.14, 16.83] (5)** | 0.36 [0.36, 0.39] (5) | regression, +15.98 (×45.38) |
|  | 42 | **17.51 [17.10, 17.53] (5)** | 0.42 [0.41, 0.44] (5) | regression, +17.10 (×42.20) |
| CPU peak MB (one build) | 1 | **779 [779, 792] (3)** | 351 [348, 351] (3) | regression, +428 (×2.22) |
|  | 7 | **792 [774, 838] (3)** | 257 [256, 258] (3) | regression, +535 (×3.08) |
|  | 42 | **801 [794, 825] (3)** | 258 [258, 259] (3) | regression, +543 (×3.10) |
| Device MiB, allocated less wgpu-internal (one build) | 1 | **316.6 [316.6, 316.6] (3)** | 251.1 [251.1, 251.1] (3) | regression, +65.5 (×1.26) |
|  | 7 | **475.2 [475.2, 475.2] (3)** | 115.2 [115.2, 115.2] (3) | regression, +360.0 (×4.13) |
|  | 42 | **487.5 [487.5, 487.5] (3)** | 118.4 [118.4, 118.4] (3) | regression, +369.1 (×4.12) |
| Field call, warm ms | 1 | **876 [869, 877] (5)** | 448 [435, 467] (5) | regression, +427 (×1.95) |
|  | 7 | **1,183 [1,178, 1,211] (5)** | 64 [64, 65] (5) | regression, +1,119 (×18.52) |
|  | 42 | **1,248 [1,224, 1,283] (5)** | 45 [44, 46] (5) | regression, +1,204 (×28.02) |

Beside the counted columns (medians, seeds 1 / 7 / 42): multi-build peak RSS F4 960 / 1,019 / 1,016, 0.1.4 368 / 331 / 333 MB; field's first call F4 890 / 1,208 / 1,244, 0.1.4 451 / 65 / 45 ms; F4's field start 2,300 [2,234, 2,406] (5) ms.

The download bytes below belong to the package and are the same for every preset.

## Download bytes (each shipped artifact; raw / gzip -9 / brotli -q 11)

| Artifact | F4 | 0.1.4 | Ceiling (raw) | Verdict |
|---|--:|--:|--:|---|
| `telperion.wasm` | **1,463,135 / 480,779 / 370,337** | 1,330,623 / 423,026 / 326,004 | 1,600,000 | raw covered; gzip +57,753 and brotli +44,333 regressions |
| `telperion-render.wasm` | **3,399,922 / 1,076,219 / 803,265** | 1,900,399 / 603,490 / 449,472 | 3,700,000 | raw covered; gzip +472,729 and brotli +353,793 regressions |
| `telperion-field.wasm` | not shipped | 344,798 / 131,050 / 107,663 | none (F4) | no worse |
| `telperion.js` | **193,862** / 17,821 / 11,240 | 118,387 / 22,453 / 18,335 | 200,000 | raw covered by the ceiling; gzip and brotli smaller |
| `wasm-source.js` | **64,437 / 15,556 / 13,195** | 235 / 200 / 155 | 70,000 | raw covered; gzip +15,356 and brotli +13,040 regressions |
| `field.js` | 1,917 / 963 / 881 | 2,720 / 1,247 / 1,096 | 16,000 | no worse |
| `voxelize.js` | 4,362 / 1,837 / 1,635 | 4,526 / 1,881 / 1,670 | 16,000 | no worse |
| all of the above | **5,127,635 / 1,593,175 / 1,200,553** | 3,701,688 / 1,183,347 / 904,395 | none | regression |
| packed tarball | **1,645,249** | 1,222,313 | none | regression, +422,936 |

## Matched-age diagnostic: F4's oak at 100

The diagnostic was **refused at every seed**. Every timing, memory and field job (seeds 1, 7 and 42, each run twice) exited 1 with `Oversize { buffer: "cycle state axes", bytes: 2558845952, limit: 2147483644 }`. That is 14 rows, all marked invalid, so no matched-age row exists. The field job's two rows ran the first F4 field probe, before amendment 2. The refusal comes from growth, which that amendment does not touch, so the job was not rerun (inferred). fn-258 drew F4's oak at 100 on 2026-10-10 (`RESULT.md`), so this refusal is newer than that branch (inferred; not traced). It is a diagnostic and carries no verdict, but it is a fact the owner's ledger should hold.

## Matched-age diagnostics: F4 at the age 0.1.4's row carries (task 5)

One timing, one memory and one field process per seed (PROTOCOL.md, Samples). 0.1.4 draws one tree at any age, so its side is the default-age column above. No verdict. Cells are seeds 1 / 7 / 42, one sample each.

| Preset at age | Fresh ms | Warm ms | Frame ms whole / mid / close | CPU peak MB | Device MiB | Field warm ms |
|---|--:|--:|--:|--:|--:|--:|
| Date palm at 100 | 4,460 / 4,412 / 4,367 | 191 / 188 / 193 | 0.46-0.49 / 0.36-0.38 / 0.44-0.46 | 796 / 759 / 761 | 79.6 each | 184 / 184 / 184 |
| Ordinary at 100 | 8,775 / 9,265 / 9,192 | 4,574 / 5,173 / 5,164 | 1.63-2.62 / 2.31-4.07 / 3.19-5.81 | 838 / 849 / 857 | 233.0 / 350.8 / 353.8 | 4,780 / 5,172 / 5,171 |
| Silver birch at 70 | 4,929 / 4,994 / 4,997 | 660 / 768 / 743 | 3.70-4.12 / 5.52-6.05 / 8.65-10.54 | 791 / 795 / 804 | 250.8 / 274.3 / 281.4 | 662 / 758 / 724 |
| Norway spruce at 100 | refused | refused | refused | refused | refused | refused |
| Laurelin at 100 | refused | refused | refused | refused | refused | refused |
| Telperion at 100 | refused | refused | refused | refused | refused | refused |

Each refused job ran twice and exited 1 with no row counted (`raw/t5-matched.log`, in job order):

- **Norway spruce at 100:** `Oversize { buffer: "cycle state axes", bytes: 2513984512, limit: 2147483644 }`, all 7 jobs, the oak's refusal at 100 with a different size.
- **Laurelin and Telperion at 100:** `DeviceRefused { requirement: "experimental generation", detail: "the tree grows past the engine's budget of 20000000 phytomers" }`, all 7 jobs each.

The ordinary at 100 builds warm in 4.6-5.2 s and answers its field in 4.8-5.2 s. Both are over 10 times its default-age figure. fn-258 measured its peak at 3.6-6.3 GB, and here it peaks at 838-857 MB.

## Validity and load

- **Rows:** 83 in all. 58 are counted: 48 timing and memory rows and 10 rerun field rows. 10 are superseded field rows. 15 are invalid.
  - **Default-age jobs:** every row valid but one, `t-v014-42-3`. That row was invalid because a GNO embedding worker (`bun … gno … native-worker`, 1,143 MiB) appeared on the card during it. Its rerun was valid. So every default-age job has its full sample count.
  - **Matched-age jobs:** the 14 refused rows above.
- **Free device memory at the start of a row:** 7,287-8,478 MiB, median 8,462. The low end is rows with the GNO worker resident from the start, which rule (a) records and does not invalidate.
- **GPU utilisation at the start:** 0-17 %.
- **One-minute load over the counted rows:** 0.37-3.83, median 1.37.

### Task 5's rows

Measured 2026-10-19, 05:31-06:44 (default ages) and 06:44-06:56 (matched ages), under PROTOCOL.md with amendment 5 (`35b2d3a2b`, committed before the first row). The runner is `raw/t5-run.sh`, the tables come from `raw/t5-table.mjs`, and the logs are `raw/t5-default.log` and `raw/t5-matched.log`. All of these are ignored.

- **Rows:** 441 for the seven presets. 398 are valid and counted, and 43 are invalid.
  - **Default-age jobs:** every job has its full sample count. One row was invalid: the first try of the birch's memory job `m-f4-42-1`, during which a GNO process (`bun … gno`, 10 MiB) appeared on the card. Its rerun was valid.
  - **Matched-age jobs:** 42 rows are the refusals above, each job run twice.
- **Free device memory at the start of a row:** 7,282-8,477 MiB, median 8,461.
- **Processes on the card at the start, recorded and not invalidating (rule (a)):** the desktop's clients (Chrome 240 MiB, Battle.net 217, Grok 63, 1Password 21, voxtype 9). 30 rows also had the GNO embedding worker at up to 1,153 MiB from their start; these are the low end of the free-memory range.
- **GPU utilisation at the start:** 0-35 %, median 0.
- **One-minute load:** 0.36-8.18, median 1.87.
- **No stops.** No probe reached its 10 minutes, and no job waited 30 minutes to start.

## Protocol check (not samples)

F4's oak at seed 1, run four ways (`raw/t4-checks.jsonl`):

| Run | fresh ms |
|---|--:|
| The driver's default cache | 4,609 |
| `__GL_SHADER_DISK_CACHE=0` | 4,290 |
| An empty cache directory | 4,397 |
| That same directory, now holding the cache | 362 |

So the variable does take the cache out: cache-off is a cold compile. The protocol's stated test (cache-off above cache-on) could not show it, because the default cache missed: its 1.1 GB directory evidently returns nothing for these pipelines (inferred). Shader compilation is about 4 s of F4's 4.5 s fresh start. 0.1.4's fresh rows ran under the same variable.

## Regression list (task 7's ledger)

These are every default-age row where F4's median is worse than 0.1.4's, by any amount, on every preset 0.1.4 shipped. Each gives the absolute gap and the ratio, per seed 1 / 7 / 42, or a range over the three seeds. It names the owner tolerance that covers the row, or "none". The only tolerances counted are the three in PROTOCOL.md (Verdict per column). The beech has no 0.1.4 row and is not on the list (PROTOCOL.md amendment 5).

| Preset | Column | Gap (F4 − 0.1.4) | Ratio | Owner tolerance |
|---|---|---|---|---|
| every preset | Build, fresh | palm +1,754-1,839; ordinary +2,117-2,144; oak +1,834-1,950; birch +1,677-1,819; spruce +1,984-2,093; Laurelin +2,416-2,591; Telperion +2,186-2,845 ms | ×1.60-2.10 | none. R0a's budget names warm builds only. |
| palm, ordinary, oak, birch (seeds 1, 42), spruce | Build, warm | palm +84-89; ordinary +321-352; oak +80-114; birch +40 / +62; spruce +300-318 ms | ×1.24-17.11 | covered: fn-237 R0a, under 0.5 s at every seed |
| Laurelin | Build, warm | seed 1 +757 (942 ms) | ×5.07 | covered: R0a, under about 1.25 s |
| Laurelin | Build, warm | seed 7 +964 (1,272 ms) | ×4.13 | none as written: 1,272 ms is over 1.25 s. Open: does "about 1.25 s" reach it? |
| Laurelin | Build, warm | seed 42 +782 (1,346 ms) | ×2.38 | none: over 1.25 s, and over the 1.33 s the owner accepted for Telperion |
| Telperion | Build, warm | seed 1 +375 (988 ms) | ×1.61 | covered: R0a, under about 1.25 s |
| Telperion | Build, warm | seed 7 +1,259 (1,356 ms); seed 42 +1,330 (1,413 ms) | ×13.95 / ×17.02 | none: over the 1.33 s the owner accepted at seed 42 (2026-10-16) |
| ordinary | Frame, whole | +0.05 / +0.16 / +0.27 ms | ×1.10 / ×1.32 / ×1.58 | none |
| ordinary | Frame, mid and close | mid +0.23 / +0.36, close +0.29 / +0.48 ms (seeds 7, 42; seed 1 no worse) | ×1.35-1.77 | none |
| Laurelin | Frame, whole / mid / close | +4.26-6.66 / +13.01-26.63 / +12.20-21.18 ms | ×2.50-13.96 | none |
| Telperion | Frame, whole / mid / close | +2.71-7.97 / +10.28-21.74 / +9.90-17.10 ms | ×2.00-45.38 | none |
| every preset | CPU peak (one build) | palm +527-533; ordinary +510-517; oak +462-484; birch +494-530; spruce +467-491; Laurelin +481-526; Telperion +428-543 MB | ×2.22-3.16 | none |
| palm | Device memory | +2.0 / +1.9 / +2.0 MiB | ×1.02-1.03 | none |
| ordinary | Device memory | seeds 7, 42: +2.4 / +11.8 MiB | ×1.03 / ×1.13 | none |
| Laurelin | Device memory | +162.4 / +288.5 / +213.9 MiB | ×1.78-2.57 | none |
| Telperion | Device memory | +65.5 / +360.0 / +369.1 MiB | ×1.26-4.13 | none |
| palm | Field call | +79 / +76 / +72 ms | ×7.52-8.03 | none |
| ordinary | Field call | +314 / +342 / +343 ms | ×6.50-7.35 | none |
| oak | Field call | +82 / +64 / +57 ms | ×1.32-1.55 | none |
| birch | Field call | seed 42 +15 ms (seeds 1, 7 no worse) | ×1.08 | none |
| spruce | Field call | +274 / +273 / +274 ms (426 / 416 / 421 against 152 / 143 / 147) | ×2.81-2.91 | none as measured. The spruce tolerance (2026-10-16) accepted about 0.35 s against 0.24 s, and F4 measures 0.42 s. |
| Laurelin | Field call | +683 / +839 / +736 ms | ×2.85-5.81 | none |
| Telperion | Field call | +427 / +1,119 / +1,204 ms | ×1.95-28.02 | none |
| package | Download, gzip and brotli | `telperion.wasm` +57,753 / +44,333; `telperion-render.wasm` +472,729 / +353,793; `wasm-source.js` +15,356 / +13,040 B | see Download bytes | none. The raw ceilings cover raw bytes only (amendment 4). |
| package | Download, total and tarball | total +1,425,947 raw / +409,828 gzip / +296,158 brotli; tarball +422,936 B | ×1.39 / ×1.35 / ×1.33; ×1.35 | none |
| package | Download, raw per artifact | `telperion.wasm`, `telperion-render.wasm`, `telperion.js`, `wasm-source.js` over 0.1.4 raw | see Download bytes | covered: the artifact ceilings (2026-10-16) |

What is no worse at every seed (no list entry):
- frames on the palm, oak, birch and spruce;
- device memory on the oak, birch and spruce;
- the birch's field at seeds 1 and 7;
- the birch's warm build at seed 7.

Two pairs are not the same tree, and the ledger should read them that way:
- **Telperion:** 0.1.4's Telperion at seeds 7 and 42 is small. Its 36-cell field marks 1,219-1,611 wood and 691-949 foliage cells, against F4's 5,976-7,364 and 5,908-7,300, and it builds warm in 83-97 ms. fn-258 saw 0.1.4's Two Trees draw bare contorted limbs.
- **The palm:** 0.1.4's palm took its CPU fallback on the harness route (`backend` `CpuFallback` in every 0.1.4 palm row), so its 6 ms warm build is a CPU mesh, not a GPU build.

These are facts about what each version draws (measured). Whether either changes the verdict is the owner's call.

### Open questions for the owner
1. Does R0a's "about 1.25 s" for Laurelin cover 1.27 s at seed 7?
2. Does the spruce field tolerance ("about 0.35 s against 0.24 s") reach F4's 0.42 s against 0.15 s natively?
3. From task 4: do the raw artifact ceilings also cover compressed growth?
