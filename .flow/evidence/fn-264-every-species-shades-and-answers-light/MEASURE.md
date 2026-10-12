# fn-264 R3 (worker, 2026-10-12)

The full R3 is the last section. The first sections measured an intermediate tip. What had shipped then: the one site light (k 0.5) for every preset, and the beech's leaf area, which the Two Trees carry. The spruce's, birch's and palm's leaf area and every species' response rows wait on the host (SOURCES.md), so this is the price of the lattice switched on, not yet of the species answering it.

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

## The gate (tip `75de71eea` plus the light test below)

- **Workspace without the renderer** (`cargo test --profile ci --workspace --exclude telperion-render --no-fail-fast`, 4 threads): 1,067 passed, 2 failed, 48 ignored. The two are `telperion-jev`'s replay tests, which need the renderer's examples (FRICTION.md); rerun after the renderer, 3 of 3 passed.
- **Renderer under the GPU lock** (`cargo test --profile ci -p telperion-render --no-fail-fast`): 272 passed, 1 failed. `a_dormant_lattice_reads_as_the_swept_one` took Telperion's site as unlit, which R1a and the beech's leaf area end; its case now takes Telperion's leaves away as it took the oak's, and the light tests pass (3 of 3).
- **`npm test`** (with its pretest's Wasm and render builds): 15 files, 162 tests passed.

## R2a on the populated rows (after the review)

`each_populated_response_moves_the_tree_through_its_consumer` (oak and ordinary at 40, seed 7): each row set to its neutral moves the tree under the site's light, but the ordinary's balance hazard, which its own values leave neutral (upkeep 0.006 never brings a balance below its tolerance 0; stated and tested). Under no shade, the shade hazard is neutral to the bit. Measured, not claimed: on an unshaded tree the leaf girth's term at a light of one is not bit-identical to its dormant path, and the vigour rows (shade size, apical control) move the tree through apical control's split in an even light. Both are the engine's behaviour, outside this spec.

Old family objects: one that states the site's own light (the oak's `extinction 0.5` before R1a) reads as the same family; any other light (the ordinary's old 1/6) is refused by name (`an_old_wire_light_reads_only_as_the_sites`).

# R3 with the size response mapped (superseded below) (worker, 2026-10-12)

**Tip** is `00956e4c7`: the site light, every species' leaf area, and the shade-tolerance mapping (host decisions 1-5). **Base** is `c1aef3f38`, as above. Every heavy run waited for at least 8 GB available and no other telperion binary running (`raw/memgate.sh`, the coordinator's rule), one tree at a time; GPU runs under the lock, with 2.0 GB held by others. Raw output, scripts and sheets are in `raw/r3b/` (ignored).

## Trees (CPU reference, default age, seed 42)

| Preset | Grown | Kept phytomers | Seconds | Peak MB |
|---|---|---|---|---|
| ordinary | 389,743 → 389,443 | 163,214 → 163,121 | 0.41 → 0.38 | 114 → 113 |
| oak | 1,213,968 both | 354,122 both | 0.92 → 1.12 | 271 → 269 |
| spruce | 2,872,640 → 2,754,077 (−4%) | 1,212,283 → 1,111,274 (−8%) | 3.13 → 3.51 | 1,038 → 1,078 |
| beech | 657,516 → 628,131 (−4%) | 332,405 → 312,133 (−6%) | 0.34 → 0.37 | 153 → 153 |
| birch | 1,161,350 → 898,887 (−23%) | 518,131 → 353,722 (−32%) | 0.81 → 0.73 | 249 → 219 |
| date palm | 480 both | 480 both | 0.00 | 11 |
| Laurelin 80 | 16,564,124 → 13,859,318 (−16%) | 3,534,309 → 2,592,009 (−27%) | 10.53 → 10.01 | 2,062 → 2,103 |
| Telperion 82 | 18,239,310 → 15,035,875 (−18%) | 3,773,761 → 2,616,721 (−31%) | 10.91 → 10.61 | 2,232 → 2,213 |

Each leafy species now keeps fewer phytomers. The intolerant birch keeps the fewest, and the Two Trees' grown count falls 16-18%, further from the 20M budget.

## Device (`build_time`, warm build ms, mean of two alternating rounds; device peak and drawn leaves at seed 42)

| Preset | Seed 42 | Seed 1 | Peak MB | Leaves |
|---|---|---|---|---|
| ordinary | 478 → 490 (+3%) | 454 → 458 (+1%) | 257 → 257 | 13,792 → 13,783 |
| oak | 267 → 270 (+1%) | 266 → 260 (−2%) | 1,128 → 1,128 | 350,269 both |
| spruce | 470 → 480 (+2%) | 466 → 460 (−1%) | 1,975 → 2,113 | 4,686,554 → 4,027,939 |
| beech | 190 → 208 (+10%) | 163 → 177 (+9%) | 269 → 292 | 94,633 → 74,431 |
| birch | 236 → 260 (+10%) | 212 → 234 (+10%) | 650 → 719 | 194,530 → 132,789 |
| date palm | 77 → 91 (+18%) | 80 → 86 (+8%) | 31 → 33 | 6,554 both |
| **Laurelin 80** | **1,182 → 1,904 (+61%)** | **858 → 1,372 (+60%)** | 3,022 → 3,639 | 919,388 → 877,325 |
| **Telperion 82** | **1,256 → 2,089 (+66%)** | **921 → 1,498 (+63%)** | 3,301 → 3,968 | 980,108 → 933,363 |

- **The price is the device's light phase.** The beech's leaf area alone took Telperion's light from 15 to 755 ms of 655 → 1,402 ms stamped (above); the smaller trees the responses grow do not pay it back on the device. On the CPU reference every preset costs what it did.
- **Device peak** rises 667 MB on Telperion (to 3,968 MB), 617 MB on Laurelin and 138 MB on the spruce.

## Stills (`resident_stills`, whole, mid and close, each preset's default age and seed)

The oak and the palm are pixel-identical to the base (AE 0). The ordinary is as viewed above. The sheets of the beech, spruce, birch, Laurelin and Telperion (`raw/r3b/stills/<preset>-sheet.png`, base left, tip right) were viewed:

- **Beech:** the same crown, a little sparser inside and below; no perceptible regression.
- **Spruce:** the same cone, a little more open; a few lower branches reach further.
- **Birch, a defect for the owner to judge:** in the whole view a thick white limb now lies along the ground right of the trunk, where the base had a thin dark shoot, and mid and close show more long bare whips through the crown. The likely cause is a low hanging limb thickened by its leaves' light (leaf girth 1.0). Not diagnosed.
- **Laurelin and Telperion:** the same crowns, slightly sparser and more open inside.

## fn-251's frame (oak → birch, t = 0.07504803866484144, age 80, seed 7)

| | Frame grown | `mesh::build` | Oak end | Birch end |
|---|---|---|---|---|
| Base | refused at the 20M budget | refused | 17.2M at k 0.46 (fn-251) | refused at the 20M budget |
| Tip | 15,995,974 | builds (19.5 s, peak 3,972 MB) | 16,625,765 | 11,264,585 |

The frame's grown count now lies between its ends, inside the oracle's band, and it builds. The birch's rows bring its own end under the budget. Kept counts were not compared.

## The gate on the full R3 (tip `176657cac`)

- **Workspace without the renderer:** 1,065 passed, 8 failed. All eight tested the unlit paths (the closed form, the coarse tree, the shed-certain shortcut, the predictor's Wasm ceiling) on the beech or spruce, which now answer light. Each now grows them as they grew before fn-264 (`tolerance::unlit`), and its claim is unchanged; 91 + 18 + 1 pass.
- **Renderer under the GPU lock:** 269 passed, 4 failed.
  - Three resident-budget tests used the spruce and beech for the predicted admission, which only an unlit species has (a lit one is held to the budget as it grows); their families now stand unlit, and 33 of 33 pass.
  - **`the_smooth_barks_hold_the_resolution_and_redraw_contracts` stays red:** the birch's close-up aliases 3.21/255 mean against the 3.0 bound (p95 12.0 against 12.0), on the birch's changed tree. Not weakened; for the owner beside the birch's sheet.
- **`npm test`:** 15 files, 162 tests passed.
- **Consequence to know:** the predictor's early Wasm ceiling (`foreseen`) and the resident budget's predicted peak serve only unlit species, so every leafy preset now takes the held-as-it-grows path, as the oak and ordinary already did.

# The final R3: the size response dormant (worker, 2026-10-12)

**Tip** is `c116854f3`: the host's ruling that ψ is not mapped from tolerance, so the beech, spruce, birch and Two Trees keep ψ dormant while the shade and balance hazards, upkeep and leaf girth stay mapped (BIRCH.md). **Base** is `c1aef3f38`. Every heavy run waited behind the memory gate, one tree at a time, and GPU runs ran under the lock. Raw output and sheets are in `raw/r3c/` (ignored). This section supersedes the two R3 sections above for the owner.

## Trees (CPU reference, default age, seed 42)

| Preset | Grown | Kept phytomers | Seconds | Peak MB |
|---|---|---|---|---|
| ordinary | 389,743 → 389,443 | 163,214 → 163,121 | 0.47 → 0.55 | 112 → 112 |
| oak | 1,213,968 both | 354,122 both | 1.36 → 1.33 | 273 → 270 |
| spruce | 2,872,640 → 2,749,947 (−4%) | 1,212,283 → 1,104,620 (−9%) | 4.08 → 3.96 | 1,028 → 1,087 |
| beech | 657,516 → 627,388 (−5%) | 332,405 → 311,182 (−6%) | 0.35 → 0.37 | 149 → 151 |
| birch | 1,161,350 → 891,867 (−23%) | 518,131 → 347,856 (−33%) | 0.83 → 0.68 | 248 → 217 |
| date palm | 480 both | 480 both | 0.00 | 11 |
| Laurelin 80 | 16,564,124 → 13,488,591 (−19%) | 3,534,309 → 2,380,297 (−33%) | 10.30 → 8.87 | 2,101 → 2,021 |
| Telperion 82 | 18,239,310 → 14,786,033 (−19%) | 3,773,761 → 2,505,972 (−34%) | 10.39 → 9.90 | 2,247 → 2,196 |

## Device (`build_time`, warm build ms, mean of two alternating rounds; device peak and drawn leaves at seed 42)

| Preset | Seed 42 | Seed 1 | Peak MB | Leaves |
|---|---|---|---|---|
| ordinary | 458 → 468 (+2%) | 441 → 444 (+1%) | 257 → 257 | 13,792 → 13,783 |
| oak | 257 → 257 (0%) | 252 → 254 (+1%) | 1,128 → 1,128 | 350,269 both |
| spruce | 436 → 458 (+5%) | 446 → 448 (0%) | 1,975 → 2,113 | 4,686,554 → 4,084,402 |
| beech | 174 → 194 (+11%) | 157 → 172 (+10%) | 269 → 292 | 94,633 → 76,245 |
| birch | 220 → 242 (+10%) | 202 → 221 (+9%) | 650 → 719 | 194,530 → 138,451 |
| date palm | 75 → 84 (+13%) | 74 → 84 (+13%) | 31 → 33 | 6,554 both |
| **Laurelin 80** | **1,108 → 1,808 (+63%)** | **821 → 1,335 (+63%)** | 3,022 → 3,639 | 919,388 → 874,404 |
| **Telperion 82** | **1,201 → 1,980 (+65%)** | **881 → 1,463 (+66%)** | 3,301 → 3,968 | 980,108 → 931,783 |

The price is unchanged by the ruling: the device's light phase, about 0.7 s and 0.6 GB on each of the Two Trees.

## Stills (`resident_stills`, whole, mid and close; sheets `raw/r3c/stills/<preset>-sheet.png`, base left, tip right), viewed

- **Oak, palm:** pixel-identical to the base. **Ordinary:** as viewed above.
- **Birch:** the ground limb is back to the base's thin dark line, and the white whips are gone. The crown is sparser in the mid view, with more of its inner branches showing bare than the base's. At seed 1 the same species (BIRCH.md's variant f) laid its lying limb at 0.0155 m against 0.0132 m before fn-264, viewed clean.
- **Beech:** the same crown, a little sparser inside. **Spruce:** the same cone, a little more open, a few lower branches reaching further.
- **Laurelin, Telperion:** the same crowns, slightly more open inside.

## The smooth-bark contract and fn-251's frame

- `the_smooth_barks_hold_the_resolution_and_redraw_contracts` passes: the birch's close-up 1.81/255 mean, p95 6.0 (bounds 3.0 and 12.0; it was 3.21 with ψ mapped).
- fn-251's frame (oak → birch, t 0.0750, 80, seed 7) grows 15,989,657 and builds (18.4 s, peak 4,056 MB), between its ends: the oak 16,625,765 and the birch 10,628,614.
