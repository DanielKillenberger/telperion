# fn-346 R1: what the balance charges today, and what the corrected charge would do

Drafted by the worker on 11 Nov 2026 at `c9e701344`, for the host. No behaviour was changed. The only code is a census probe, kept as `probes/census.patch` and reverted (`git status --short crates` is empty). Nothing here proposes a design. Questions for the host are at the end.

Marks: **read** (from the code, not run), **measured** (printed by a run), **arithmetic** (computed from the run's files by `probes/r1.py` or `probes/lit.py`), **source** (quoted from a text), **unchecked** (named but not read).

## Headline

- **With K = 1/34.8 m, the corrected charge passes the oak's income at age 49 and stands at 2.4 times its income at 119, on today's tree (arithmetic).** On seed 1001 the charge is 0.90 of income at 39, 1.13 at 49, 1.90 at 89 and 2.44 at 119. Today's charge, which includes the dead wood, is 0.31, 0.40, 0.75 and 1.30 of income in the same years.
- **The reason is the tip's light.** A tip of whole presence in full light reads 1 in the code's unit, but the mean living tip reads 0.23 to 0.26 at every age (measured). The mean twig or short tip therefore earns its cost at a path of 8 to 9 m (arithmetic). The tips at the top of the crown, within 1 m of the highest tip, read 0.59 to 0.84 on average (measured).
- **Light rises with path, it does not fall (measured).** In every decade the longest-path band holds the best-lit tips: on A at 119 the mean own light is 0.025 at 10 to 15 m of path and 0.89 at 30 to 35 m. The balance would fall with path only because the cost rises with it.
- **Path runs 12 to 30% longer than height (measured).** At 119 the highest tip stands 25.9 m tall on a 29.0 m path; at 299 it stands 50.2 m on 65.4 m. The 34.8 m in K is a height in its source, so "the path at which a lit tip breaks even" is reached at about 28 m of height on this oak.
- **The two readings of a limb's charge part early (arithmetic).**
  - **Reading (a), the whole path to the root:** puts 3 of the 9 limbs and boughs on the main line below zero at 39, among them two of the three fork limbs (yearly hazard 0.18 and 0.37 under today's rule). At 79 it puts 16 of 16 below zero.
  - **Reading (b), only the path inside the limb's own subtree:** puts none below zero at 39, 4 of 16 at 79 and 3 of 25 at 119 on A. On B at 179 it puts 34 of 47 below zero, the three fork limbs at −0.62 to −0.71 (hazard 0.71 to 0.76 a year).
- **The same K is 13 to 31 times today's living upkeep (arithmetic).** K × ΣL is 12.9 times today's living upkeep at 49, 21.4 times at 89 and 30.7 times at 129. Matching today's living upkeep instead would need a K equal to 1/450 m, 1/745 m and 1/1069 m in those three years.

## How it was made

- **The probe.** `probes/census.patch` is fn-344's `oldoak.patch` (variant B, the yearly census, the memory stop, the `BAL` lines) plus one function in `grow/cycle/probe.rs`, `paths`.
  - **When it runs.** Every 10 years, at the end of `balance_one`, after the walk and the shedding.
  - **What it writes.** It writes `tips-<year>.tsv` and `limbs-<year>.tsv` into `PROBE_PATHS`.
  - **Path.** A tip's path is measured along the rough layout's wood from the root collar. Each axis's base path is its bearer's cumulative path at the node it stands on, or at the bearer's end for a continuation, as H carried it. The probe adds the straight distance from phytomer tip to phytomer tip, starting at the axis's base. It is computed from the layout at census time, so the lay walk is unchanged and nothing is stored per phytomer.
  - **Tips.** For each living tip it writes its path, its height, its own light `base_scale × running × light` (the `own` `balance_one` reads), its presence `base_scale × running`, its raw lattice light, its age row, and the limb it stands under.
  - **Limbs.** For each living lateral standing on the main line it writes its base path and height, its subtree light `w.light`, today's cost `w.cost`, and its memory after this year's walk.
- **Runs.** All runs used `probes/census.sh` under `telperion-heavy 12 --gpu 2 --threads 4`, grow-only, and stopped at 8 GiB resident or at eight times the most A had held (fn-344's `curve-300-1001-peak.txt`).
  - **A, seed 1001, to 300:** 103 s, peak 2.96 GiB, not stopped.
  - **B, seed 1001:** stopped by its census at 8.16 GiB in year 183 (exit 42), holding 12,959,536 phytomers, after 128 s. The last decade it reached is 179. The run was not killed.
  - **A, seed 1007, to 300:** 154 s, peak 4.14 GiB.
- **Files.** Raw files are under the spec's ignored `raw/{A-1001,B-1001,A-1007}/` (795 MB). The tables are `probes/r1.py <dir> whole|bands|ranks|limbs|top` and `probes/lit.py`.
- **Repeatability (measured).** A's held phytomers equal fn-344's at every year compared, for example 302,606 at 39, 1,645,899 at 79 and 1,598,756 at 299. Income and tip counts differ from fn-344's follow-up 2 table by up to 0.3%, for example 10,134 against 10,112 at 49. That difference is not explained.
- **A census caveat.** The tips are listed after this year's shedding, but a limb's Q and today's C are the walk's, taken before it. A limb shed in that very year therefore shows Q with no tips. One such limb is A's axis 759 at 119, and its (a) and (b) rows read +1.00 for that reason.

## 1. Today's charge (read)

**Cost of an axis.**
- `pencils[i].wood` is the axis's own laid length in metres: Σ `internode × scale × size` over its phytomers as the rough layout lays them, summed in f32 (`grow/draw.rs` `draw_bent`, line 385).
- It reads the phytomer's scale, its size relative to siblings. It reads no girth and no pipe count: the trunk under the whole crown costs per metre what a twig of the same scale costs.
- `upkeep(i) = states[pa].upkeep × wood`. The oak's `upkeep` is 0.35 at every age row (`oak.rs`; `tolerance.rs` copies the oak's value to every species). `species.rs` states the unit: "what a metre of this PA's present wood costs, in the units of the light a bud of whole presence collects in full light".

**What a subtree costs** (`Working::totals`, `grow/cycle/working.rs`).
- An open axis is one on the way from the root to a living apex or a sleeping bud. Its cost is its own upkeep, plus each open child's cost, plus each closed child's frozen cost.
- A closed subtree's cost is taken once (`closed`), as the sum of its wood's upkeep, and stays in its bearer's cost for good. fn-344 measured that it stays after the wood is let go.
- `balance.rs` `totals` is the whole-tree reference: every laid axis's upkeep, summed to the root.

**A tip's light.**
- `pencils[i].light` is the lattice light at the apex's tip from last cycle's leaves (`Field::at`), clamped to 0 to 1. A corner outside the lattice reads the open sky's 1.
- The apex's own light in the balance is `base_scale × running × light`, so a tip of whole presence in full light reads 1.
- **Measured.** Tips with light ≥ 0.95 read 0.95 to 0.98, since no tip in the crown sees the whole open sky. Their presence is 0.89 to 1.

**Which laterals are tested.**
- In the walk (`passes.rs`, `contract/shed.rs` `walk`), an axis decides only where it has a bearer, its link's node is not `TIP` and something in its subtree lives. That means a lateral, at any order.
- The seed and every continuation or relay never decide: each inherits its bearer's shedding and kept share. So the main line (seed, trunk, fork and leader rows, and their relays) is never tested, and neither is a lateral's own continuation. That continuation is shed or kept with its lateral.
- **Measured.** One living main-line tip stands in every decade. Its own light is 0.94 to 0.99 except 0.62 at 199, and its path reaches 63.9 m at 299.

**What the hazard does** (`balance` in `contract/shed.rs`).
- `memory ← (memory + (Q − C)/(Q + C)) / 2`.
- `hazard = balance_hazard × max(0, tolerance − memory)`. For the oak, `balance_hazard` is 2.0 and `tolerance` is 0.0, so hazard = 2 × max(0, −memory).
- The lateral is shed when its draw lies below `1 − e^−hazard`. Otherwise it keeps a share `kept`, the bearer's share times the presence of the draw's lead over its window, which falls toward 0 as the draw nears the threshold.
- `kept` multiplies the apex's presence in the allotment (`allot_one`: presence `= base_scale × running × kept`). It also multiplies the unit's survival (`advance.rs` line 173, `stop.rs` line 106). So a lateral with a negative memory grows less before it is shed.
- In the tables below, "hazard" is the yearly chance `1 − e^(−2 × max(0, −b))` for a steady balance b. A balance held steady is also the memory's fixed point.

**What else reads a tip's light** (read).
- **Survival.** `advance.rs` gives ln survival = ln viability × light^−shade_hazard, with the oak's `shade_hazard` 1.0.
- **The vigour split.** `allot_one` and `Working::split` divide the root's vigour at each branching point by the subtrees' lit presence with apical control λ (oak 0.45). Each bud's size then comes from its share against its PA's sums with ψ = `shade_size` (oak 1.5).
- **Girth.** `leaf_girth` (oak 1.0) scales each phytomer's own pipe by its leaves' light against the tree's mean (`retire.rs`, `girth.rs`).
- **The balance reaches growth twice:** through shedding, and through `kept`.

**The girth rule's pipes** (read; `girth.rs`).
- Every phytomer adds its own pipe, `pipe × scale × ripened × leaf factor` as a radius (oak `pipe` 0.009 m, exponent 2.6).
- A section is the sum of the sections it carries, so a node's girth counts the pipes of every phytomer above it, living or not.
- A shed branch's pipe stays in its bearer by `retained`, which is 0 for the oak. Standing dead wood that has not been shed still carries its pipes into the girth below it.

**Is a count of living pipes held anywhere?** (read)
- No. `Working::living` is one bool per axis, and `Working::light` is a sum of light.
- A count of living tips per subtree would be one more number summed in the same reverse loop of `Working::totals`, with no new pass.
- Σ path over a subtree's living tips needs each tip's path, which nothing holds today. H carried it as one f32 per laid phytomer on the lay's walk. This probe computed it from the rough layout at census time.

## 2. The published basis (30 minutes)

- **Maintenance respiration against living sapwood (source).** Ryan (1990, Can. J. For. Res. 20: 48–57), read in the full-text PDF through a search excerpt, not the whole paper:
  - "Stem maintenance respiration was linearly related to live-cell volume for lodgepole pine (Pinus contorta var. latifolia Engelm.) from 4 to 36 cm dbh and for Engelmann spruce (Picea engelmannii Parry) from 0 to 20 cm dbh. Sapwood contained greater than 80% of the total live-cell volume in stems."
  - From its conclusions: "Sapwood or live-cell volume, not bole surface area, should be used to estimate stem maintenance respiration."
  - It gives 6.6 × 10⁻⁵ kg C per kg C of sapwood per day at 15 °C.
  - These are conifers. No broadleaf or oak figure was read.
- **Heartwood is dead tissue (source, secondary).** Lehnebach et al. (2018, Ann. Bot. 121: 773, "The pipe model theory half a century on"), full text read: "this distinction is made on the assumption that SW is physiologically active whereas the HW is the dead part of the wood that no longer contains living cells". The same paragraph adds that sapwood "remains physiologically active until it transforms into HW, particularly in terms of water and carbohydrate storage", while water conduction is restricted to the outermost sapwood. No source read states in so many words that heartwood costs no maintenance respiration. That follows from "no longer contains living cells" together with Ryan's respiration by live-cell volume, which is an inference.
- **Disused pipes (source, secondary; Shinozaki et al. 1964 unchecked).** Lehnebach et al. quote Shinozaki: "the successive accumulation of disused pipes in the trunk is associated with the progress of tree growth", and "the pipes which make up the stem and branches of a tree are renewed every year". Their reading: "when a new leaf is produced, a corresponding new functional pipe is also produced, and is automatically disabled when the leaf is shed", so that in temperate deciduous trees "only the last wood growth ring is conductive". Shinozaki's own papers were not read.
- **Pipe length and height growth (source, abstract and secondary).**
  - **Lehnebach et al., full text:** "A consequence of the PMT theoretically derived by Valentine (1985) and Mäkelä (1986) is the limitation of growth in height as the result of increasing maintenance requirements and respiration of longer pipes."
  - **Valentine 1985, as Lehnebach et al. quote it:** "tall trees with long pipes should be more susceptible to defoliation-induced dieback than short trees because the respirational demand for substrate by a tree increases with pipe length". Valentine (1985, J. Theor. Biol. 117: 579) was read at abstract level only: "The model of height growth rate is derived considering only those pipes extending to the apex of the crown of the tree." The respiration-per-pipe-length sentence is read only through Lehnebach's quotation.
  - **Mäkelä 1986, abstract only** (J. Theor. Biol. 123: 103–120; IIASA record): "It is shown that there is a species-specific, environment-dependent upper limit for a sustainable length of the woody organs, and that the diameter of a tree at a certain height depends upon the rate at which that height has been achieved." The model's equations, and whether its upkeep is per unit sapwood mass along the pipe, are unchecked.
  - **Gerrish (1988), as Lehnebach et al. report it:** "the so-called specific leaf burden, the SW mass associated with a unit leaf mass, increases with stem elongation independently of stem diameter". Gerrish's paper is unchecked.
- **Unchecked:** Shinozaki et al. 1964a/b; the full text of Valentine 1985 and Mäkelä 1986; Valentine 1988; any oak or broadleaf measurement of sapwood respiration per unit length or volume; any source that ties the oak's 34.8 m potential height to a maintenance-per-pipe constant.

## 3 and 4. The census and the arithmetic

The corrected charge as written in the dispatch:
- **Each tip's cost is K × L**, where L is the tip's path from the root along the wood, and nothing is charged for dead wood.
- **K = 1 / 34.8 m = 0.02874 per metre of path per tip.** This makes a tip of whole presence in full light, with own light 1, earn exactly its cost at L = 34.8 m.
- **Where 1 comes from.** It is the unit `species.rs` states for `upkeep`, the light a bud of whole presence collects in full light. `Field::at` clamps light to 1 and the open sky reads 1. A tip's own light is `presence × light` (`passes.rs` line 41).

**What the lit tips at the top of the crown read** (measured, `probes/lit.py`). "Top 1 m" is the tips within 1 m of the highest tip's height.

| tree | year | tips with light ≥ 0.95 | their own, median | top 1 m tips | their own, mean | their own, median | their presence, median |
|---|---|---|---|---|---|---|---|
| A 1001 | 39 | 9 | 0.955 | 333 | 0.692 | 0.737 | 1.000 |
| A 1001 | 79 | 32 | 0.955 | 711 | 0.743 | 0.764 | 1.000 |
| A 1001 | 119 | 80 | 0.955 | 537 | 0.588 | 0.581 | 0.889 |
| A 1001 | 199 | 49 | 0.960 | 883 | 0.618 | 0.620 | 1.000 |
| A 1001 | 299 | 42 | 0.966 | 525 | 0.767 | 0.783 | 1.000 |
| B 1001 | 179 | 252 | 0.867 | 98 | 0.839 | 0.833 | 0.889 |

Over all tips, presence has a median of 0.72 to 1 and a 10th percentile of 0.07 to 0.29 from 39 on (measured).

### The tree's whole charge (arithmetic from measured files)

- **K·ΣL** is the corrected charge.
- **Income** is the root's subtree light, `w.light[0]` (measured).
- **Today's charges** are the balance pass's own sums, fn-344's `BAL` line (measured). "Today living" is the open axes' own upkeep and "today dead" the closed children's frozen upkeep. Under B the dead upkeep is computed for the record but not charged.

**A, seed 1001**

| year | living tips | income | Σ L km | mean L m | K·ΣL | K·ΣL / income | today living | today dead | today charged | today / income | K·ΣL / today living |
|---|---|---|---|---|---|---|---|---|---|---|---|
| 9 | 1,191 | 607 | 2 | 1.6 | 56 | 0.09 | 20 | 6 | 26 | 0.04 | 2.7 |
| 19 | 4,121 | 1,985 | 14 | 3.3 | 390 | 0.20 | 94 | 155 | 249 | 0.13 | 4.2 |
| 29 | 10,750 | 3,956 | 66 | 6.1 | 1,884 | 0.48 | 227 | 554 | 781 | 0.20 | 8.3 |
| 39 | 24,622 | 6,382 | 200 | 8.1 | 5,744 | 0.90 | 522 | 1,465 | 1,987 | 0.31 | 11.0 |
| 49 | 40,939 | 10,134 | 399 | 9.8 | 11,473 | 1.13 | 886 | 3,199 | 4,085 | 0.40 | 12.9 |
| 59 | 61,634 | 15,122 | 689 | 11.2 | 19,793 | 1.31 | 1,333 | 5,906 | 7,239 | 0.48 | 14.9 |
| 69 | 85,551 | 21,004 | 1,100 | 12.9 | 31,596 | 1.50 | 1,813 | 9,874 | 11,686 | 0.56 | 17.4 |
| 79 | 111,239 | 26,907 | 1,612 | 14.5 | 46,313 | 1.72 | 2,367 | 15,408 | 17,775 | 0.66 | 19.6 |
| 89 | 137,939 | 33,671 | 2,230 | 16.2 | 64,084 | 1.90 | 2,992 | 22,207 | 25,199 | 0.75 | 21.4 |
| 99 | 160,988 | 38,036 | 2,862 | 17.8 | 82,253 | 2.16 | 3,263 | 30,593 | 33,857 | 0.89 | 25.2 |
| 109 | 190,427 | 44,588 | 3,679 | 19.3 | 105,731 | 2.37 | 3,889 | 39,897 | 43,786 | 0.98 | 27.2 |
| 119 | 158,355 | 40,960 | 3,481 | 22.0 | 100,018 | 2.44 | 3,351 | 49,894 | 53,245 | 1.30 | 29.8 |
| 129 | 61,680 | 22,993 | 1,724 | 28.0 | 49,544 | 2.15 | 1,612 | 58,482 | 60,094 | 2.61 | 30.7 |
| 139 | 84,904 | 28,330 | 2,552 | 30.1 | 73,334 | 2.59 | 2,195 | 63,314 | 65,510 | 2.31 | 33.4 |
| 149 | 102,623 | 28,679 | 3,307 | 32.2 | 95,043 | 3.31 | 2,506 | 69,676 | 72,182 | 2.52 | 37.9 |
| 159 | 132,432 | 37,752 | 4,516 | 34.1 | 129,757 | 3.44 | 3,292 | 77,091 | 80,383 | 2.13 | 39.4 |
| 169 | 151,822 | 44,945 | 5,481 | 36.1 | 157,506 | 3.50 | 3,734 | 86,210 | 89,943 | 2.00 | 42.2 |
| 179 | 173,430 | 49,409 | 6,548 | 37.8 | 188,152 | 3.81 | 3,956 | 96,709 | 100,665 | 2.04 | 47.6 |
| 189 | 202,927 | 56,254 | 7,991 | 39.4 | 229,617 | 4.08 | 4,650 | 108,045 | 112,695 | 2.00 | 49.4 |
| 199 | 148,220 | 35,012 | 5,761 | 38.9 | 165,544 | 4.73 | 3,073 | 120,512 | 123,586 | 3.53 | 53.9 |
| 219 | 205,858 | 41,294 | 8,605 | 41.8 | 247,272 | 5.99 | 3,906 | 139,262 | 143,168 | 3.47 | 63.3 |
| 239 | 169,759 | 23,841 | 6,702 | 39.5 | 192,585 | 8.08 | 2,532 | 160,360 | 162,892 | 6.83 | 76.1 |
| 259 | 222,546 | 31,684 | 9,144 | 41.1 | 262,750 | 8.29 | 3,366 | 175,815 | 179,181 | 5.66 | 78.0 |
| 279 | 77,909 | 35,665 | 4,404 | 56.5 | 126,545 | 3.55 | 2,398 | 195,327 | 197,724 | 5.54 | 52.8 |
| 299 | 94,929 | 37,230 | 5,732 | 60.4 | 164,708 | 4.42 | 2,340 | 208,383 | 210,724 | 5.66 | 70.4 |

The rows for 209, 229, 249, 269 and 289 are in `python3 probes/r1.py raw/A-1001 whole`.

**B, seed 1001** (stopped in year 183)

| year | living tips | income | Σ L km | mean L m | K·ΣL | K·ΣL / income | today living | today dead (not charged) | today / income | K·ΣL / today living |
|---|---|---|---|---|---|---|---|---|---|---|
| 39 | 24,702 | 6,400 | 200 | 8.1 | 5,757 | 0.90 | 527 | 1,462 | 0.08 | 10.9 |
| 49 | 42,686 | 10,335 | 412 | 9.7 | 11,844 | 1.15 | 932 | 3,191 | 0.09 | 12.7 |
| 59 | 64,651 | 15,493 | 715 | 11.1 | 20,549 | 1.33 | 1,424 | 5,955 | 0.09 | 14.4 |
| 69 | 93,834 | 21,366 | 1,183 | 12.6 | 33,991 | 1.59 | 2,004 | 9,996 | 0.09 | 17.0 |
| 79 | 130,741 | 28,682 | 1,856 | 14.2 | 53,324 | 1.86 | 2,828 | 15,637 | 0.10 | 18.9 |
| 89 | 166,143 | 36,398 | 2,618 | 15.8 | 75,218 | 2.07 | 3,592 | 23,176 | 0.10 | 20.9 |
| 99 | 212,390 | 43,805 | 3,673 | 17.3 | 105,547 | 2.41 | 4,490 | 32,552 | 0.10 | 23.5 |
| 109 | 256,185 | 52,797 | 4,819 | 18.8 | 138,465 | 2.62 | 5,549 | 44,062 | 0.11 | 25.0 |
| 119 | 307,400 | 61,484 | 6,302 | 20.5 | 181,088 | 2.95 | 6,574 | 57,879 | 0.11 | 27.5 |
| 129 | 364,937 | 72,021 | 8,103 | 22.2 | 232,836 | 3.23 | 7,851 | 74,139 | 0.11 | 29.7 |
| 139 | 426,991 | 82,493 | 10,305 | 24.1 | 296,127 | 3.59 | 9,344 | 93,408 | 0.11 | 31.7 |
| 149 | 500,841 | 94,132 | 13,006 | 26.0 | 373,744 | 3.97 | 11,038 | 116,061 | 0.12 | 33.9 |
| 159 | 590,873 | 108,874 | 16,441 | 27.8 | 472,454 | 4.34 | 13,087 | 142,638 | 0.12 | 36.1 |
| 169 | 684,776 | 125,838 | 20,347 | 29.7 | 584,685 | 4.65 | 15,171 | 173,598 | 0.12 | 38.5 |
| 179 | 803,025 | 142,382 | 25,260 | 31.5 | 725,866 | 5.10 | 17,764 | 209,311 | 0.12 | 40.9 |

**A, seed 1007** (excerpt; full table from `python3 probes/r1.py raw/A-1007 whole`)

| year | living tips | income | mean L m | K·ΣL / income | today / income | K·ΣL / today living |
|---|---|---|---|---|---|---|
| 39 | 39,413 | 9,026 | 8.4 | 1.05 | 0.31 | 10.7 |
| 49 | 78,156 | 15,250 | 10.1 | 1.49 | 0.44 | 12.7 |
| 89 | 296,983 | 59,142 | 17.3 | 2.49 | 0.88 | 22.0 |
| 119 | 281,608 | 65,707 | 22.4 | 2.76 | 1.58 | 29.7 |
| 129 | 372,183 | 78,936 | 24.8 | 3.36 | 1.56 | 33.4 |
| 199 | 313,102 | 60,275 | 39.9 | 5.96 | 4.06 | 47.9 |
| 299 | 41,739 | 15,874 | 55.4 | 4.19 | 21.09 | 63.7 |

### Light by path (arithmetic from measured files)

"Below K·L" is the share of living tips whose own light is below their cost K × L.

| tree | year | L m | tips | below K·L | mean own | median own | K·L at band middle |
|---|---|---|---|---|---|---|---|
| A 1001 | 39 | 0 to 5 | 752 | 0.19 | 0.377 | 0.402 | 0.072 |
| | | 5 to 10 | 22,951 | 0.62 | 0.242 | 0.187 | 0.216 |
| | | 10 to 15 | 919 | 0.08 | 0.592 | 0.614 | 0.359 |
| A 1001 | 79 | 5 to 10 | 2,296 | 1.00 | 0.091 | 0.093 | 0.216 |
| | | 10 to 15 | 64,317 | 0.97 | 0.152 | 0.139 | 0.359 |
| | | 15 to 20 | 44,626 | 0.75 | 0.367 | 0.346 | 0.503 |
| A 1001 | 119 | 10 to 15 | 24,589 | 1.00 | 0.025 | 0.024 | 0.359 |
| | | 15 to 20 | 34,880 | 1.00 | 0.157 | 0.158 | 0.503 |
| | | 20 to 25 | 38,998 | 0.99 | 0.243 | 0.222 | 0.647 |
| | | 25 to 30 | 59,843 | 0.97 | 0.386 | 0.362 | 0.790 |
| | | 30 to 35 | 45 | 0.24 | 0.891 | 0.895 | 0.934 |
| A 1001 | 199 | 30 to 35 | 40,529 | 1.00 | 0.037 | 0.033 | 0.934 |
| | | 35 to 40 | 39,627 | 1.00 | 0.194 | 0.197 | 1.078 |
| | | 40 to 45 | 63,181 | 1.00 | 0.358 | 0.341 | 1.221 |
| | | 45 to 50 | 4,883 | 1.00 | 0.519 | 0.514 | 1.365 |
| A 1001 | 299 | 50 to 55 | 5,460 | 1.00 | 0.199 | 0.145 | 1.509 |
| | | 55 to 60 | 25,187 | 1.00 | 0.336 | 0.334 | 1.652 |
| | | 60 to 65 | 64,219 | 1.00 | 0.429 | 0.431 | 1.796 |
| | | 65 to 70 | 39 | 1.00 | 0.897 | 0.920 | 1.940 |
| B 1001 | 119 | 10 to 15 | 46,896 | 1.00 | 0.044 | 0.025 | 0.359 |
| | | 15 to 20 | 106,654 | 1.00 | 0.124 | 0.115 | 0.503 |
| | | 20 to 25 | 81,393 | 1.00 | 0.222 | 0.206 | 0.647 |
| | | 25 to 30 | 71,288 | 0.97 | 0.390 | 0.363 | 0.790 |
| | | 30 to 35 | 233 | 0.75 | 0.809 | 0.823 | 0.934 |
| B 1001 | 179 | 15 to 20 | 75,285 | 1.00 | 0.022 | 0.019 | 0.503 |
| | | 25 to 30 | 137,101 | 1.00 | 0.131 | 0.125 | 0.790 |
| | | 35 to 40 | 166,037 | 1.00 | 0.216 | 0.205 | 1.078 |
| | | 45 to 50 | 24,938 | 1.00 | 0.541 | 0.556 | 1.365 |

Bands with fewer than 50 tips are left out (A at 299, 45 to 50 m, 24 tips; B at 179, 50 to 55 m, 96 tips, mean own 0.568). The full tables are in `r1.py <dir> bands`.

**What the bands show (arithmetic).**
- **Light rises with path inside each year.** The tips with the shortest paths are the inner, shaded ones.
- **The tree's paths shift outward as it grows.** At 39 most tips stand at 5 to 10 m of path; at 119 at 15 to 30 m; at 299 at 55 to 65 m.
- **The share below K·L passes 0.97 in every band from 79 on, but for the top band.** It reaches 1.00 in every band from 199 on, including the best-lit 39 tips at 65 to 70 m (mean own 0.90 against a cost of 1.94).

### The path at which the mean tip of each rank earns its cost (arithmetic: mean own / K)

| tree | year | limb 6 | bough 7 | branch 9 | shoot 12 | twig 13 | short 14 | mean L of all tips |
|---|---|---|---|---|---|---|---|---|
| A 1001 | 39 | 23.6 (4 tips) | 20.1 | 11.7 | 10.2 | 9.1 | 8.9 | 8.1 |
| A 1001 | 79 | 20.4 (6) | 18.2 | 10.1 | 9.0 | 8.2 | 8.2 | 14.5 |
| A 1001 | 119 | 22.6 (10) | 19.1 | 9.7 | 9.8 | 8.2 | 8.5 | 22.0 |
| A 1001 | 199 | 18.4 (12) | 15.3 | 8.5 | 9.7 | 7.4 | 8.2 | 38.9 |
| B 1001 | 179 | 13.6 (26) | 12.4 | 6.7 | 7.0 | 5.8 | 6.2 | 31.5 |

The short and twig rows hold 95 to 97% of the living tips. The mean tip of those rows earns its cost at a path of 6 to 9 m in every decade, which the tree's mean path passes between 39 and 49. The rows below the limb row (trunk, forks, leader) hold no living tip from 29 on. The one main-line tip is the trunk row's at 9, the leader's at 19 and the limb row's from 29 on, and it is counted in the limb column.

### Each limb on the main line (arithmetic from measured files)

- **Q** is the limb's subtree light `w.light`. **Today C** is today's cost `w.cost`.
- **Today now** is (Q − C)/(Q + C), and **today memory** is the remembered balance after this year's walk.
- **(a) C** is K × Σ L over the limb's living tips, each tip's path whole to the root.
- **(b) C** is K × Σ (L − the limb's base path), the path inside the limb's own subtree.
- **Hazard** is the yearly chance of shedding `1 − e^(−2·max(0, −b))` that today's rule gives a balance b held steady.
- Rows show the limb, fork and bough rows and any lateral with at least 50 tips. Smaller laterals (sprigs, branches) are counted below each table. Full tables come from `python3 probes/r1.py raw/<dir> limbsall <year>`.

**A, seed 1001, year 39** (48 living laterals on the main line)

| axis | rank | base path m | tips | Q | today C | today now | today memory | (a) balance | hazard (a) | (b) balance | hazard (b) |
|---|---|---|---|---|---|---|---|---|---|---|---|
| 749 | limb | 4.7 | 4,996 | 966.5 | 208.1 | +0.65 | +0.67 | −0.10 | 0.18 | +0.32 | 0 |
| 748 | limb | 4.7 | 3,450 | 888.0 | 185.1 | +0.66 | +0.67 | +0.04 | 0 | +0.43 | 0 |
| 903 | limb | 5.0 | 5,558 | 748.6 | 139.1 | +0.69 | +0.70 | −0.23 | 0.37 | +0.32 | 0 |
| 1791 | bough | 6.3 | 1,488 | 541.0 | 94.9 | +0.70 | +0.72 | +0.18 | 0 | +0.67 | 0 |
| 1524 | bough | 6.0 | 1,650 | 483.4 | 112.3 | +0.62 | +0.65 | +0.07 | 0 | +0.56 | 0 |
| 2458 | bough | 7.1 | 1,178 | 459.8 | 72.4 | +0.73 | +0.74 | +0.17 | 0 | +0.68 | 0 |
| 3968 | bough | 7.7 | 1,054 | 365.3 | 46.2 | +0.78 | +0.79 | +0.12 | 0 | +0.75 | 0 |
| 2889 | bough | 7.3 | 1,295 | 322.5 | 58.0 | +0.70 | +0.72 | −0.02 | 0.05 | +0.65 | 0 |
| 6366 | bough | 8.1 | 574 | 248.6 | 25.2 | +0.82 | +0.83 | +0.22 | 0 | +0.83 | 0 |

- **All 48 laterals.** Below zero today (memory) 3, under (a) 12, under (b) 0.
- **The sprigs at 0.3 to 4 m.** Their balance rises under both readings, because today's per-metre cost charges their own long wood: axis 3 goes from +0.23 today to +0.62 under (a).

**A, seed 1001, year 79** (45 living laterals)

| axis | rank | base path m | tips | Q | today C | today now | today memory | (a) balance | hazard (a) | (b) balance | hazard (b) |
|---|---|---|---|---|---|---|---|---|---|---|---|
| 29 | limb | 4.7 | 31,963 | 5,980.2 | 4,032.1 | +0.19 | +0.23 | −0.35 | 0.50 | −0.14 | 0.25 |
| 35 | limb | 5.0 | 32,672 | 4,787.8 | 2,931.2 | +0.24 | +0.26 | −0.42 | 0.57 | −0.19 | 0.31 |
| 28 | limb | 4.7 | 16,243 | 4,687.7 | 3,264.5 | +0.18 | +0.19 | −0.24 | 0.38 | −0.07 | 0.13 |
| 60 | bough | 6.0 | 3,715 | 1,332.7 | 1,038.6 | +0.12 | +0.12 | −0.12 | 0.22 | +0.11 | 0 |
| 2118 | bough | 11.2 | 2,858 | 1,242.6 | 451.5 | +0.47 | +0.48 | −0.07 | 0.13 | +0.42 | 0 |
| 115 | bough | 7.1 | 2,425 | 1,136.4 | 912.4 | +0.11 | +0.12 | −0.03 | 0.06 | +0.23 | 0 |
| 313 | bough | 8.1 | 3,191 | 889.2 | 561.6 | +0.23 | +0.24 | −0.24 | 0.38 | +0.12 | 0 |
| 149 | bough | 7.3 | 2,913 | 732.7 | 666.1 | +0.05 | +0.08 | −0.30 | 0.45 | −0.01 | 0.01 |
| 8322 | bough | 13.6 | 1,938 | 682.8 | 155.5 | +0.63 | +0.64 | −0.17 | 0.28 | +0.55 | 0 |
| 53997 | bough | 16.9 | 682 | 326.3 | 20.4 | +0.88 | +0.89 | −0.05 | 0.10 | +0.82 | 0 |

- **All 45 laterals.** Below zero today 1, under (a) 37, under (b) 4: the three fork limbs and bough 149.
- **The other boughs of the limb row** (11 more, all below zero under (a) at −0.03 to −0.20) are in the full table.

**A, seed 1001, year 119** (56 living laterals)

| axis | rank | base path m | tips | Q | today C | today now | today memory | hazard today | (a) balance | hazard (a) | (b) balance | hazard (b) |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 28 | limb | 4.7 | 37,715 | 12,064.1 | 12,710.2 | −0.03 | −0.01 | 0.02 | −0.40 | 0.55 | −0.31 | 0.46 |
| 29 | limb | 4.7 | 69,339 | 9,655.4 | 11,584.1 | −0.09 | −0.08 | 0.15 | −0.56 | 0.68 | −0.45 | 0.59 |
| 9699 | limb | 20.8 | 10,792 | 4,019.9 | 674.0 | +0.71 | +0.72 | 0 | −0.36 | 0.51 | +0.32 | 0 |
| 398 | bough | 11.2 | 4,278 | 1,783.5 | 2,027.4 | −0.06 | −0.04 | 0.08 | −0.26 | 0.41 | +0.03 | 0 |
| 1782 | bough | 16.9 | 3,500 | 1,642.2 | 841.0 | +0.32 | +0.34 | 0 | −0.20 | 0.33 | +0.36 | 0 |
| 504 | bough | 12.1 | 3,466 | 1,124.2 | 1,676.5 | −0.20 | −0.16 | 0.28 | −0.35 | 0.51 | −0.01 | 0.02 |
| 12111 | bough | 21.3 | 3,108 | 1,108.6 | 256.9 | +0.62 | +0.64 | 0 | −0.34 | 0.49 | +0.53 | 0 |
| 1262 | bough | 15.5 | 2,875 | 860.6 | 836.6 | +0.01 | +0.06 | 0 | −0.38 | 0.53 | +0.15 | 0 |

- **All 56 laterals.** Below zero today (memory) 9, under (a) 47, under (b) 3: fork limbs 28 and 29 and bough 504.
- **The fork limbs.** Today they stand at −0.01 and −0.08, and A sheds them by year 132 (the spec, from fn-344). Under (b) they would stand at −0.31 and −0.45, a yearly hazard of 0.46 and 0.59.

**B, seed 1001, year 179, the last decade reached** (82 living laterals; rank ≤ 7 rows with the most light)

| axis | rank | base path m | tips | Q | today C | today now | (a) balance | hazard (a) | (b) balance | hazard (b) |
|---|---|---|---|---|---|---|---|---|---|---|
| 28 | limb (fork) | 4.7 | 160,656 | 36,002 | 4,904 | +0.76 | −0.66 | 0.73 | −0.62 | 0.71 |
| 9750 | limb | 20.8 | 100,234 | 33,433 | 3,445 | +0.81 | −0.56 | 0.68 | −0.28 | 0.43 |
| 29 | limb (fork) | 4.7 | 242,608 | 23,768 | 3,565 | +0.74 | −0.76 | 0.78 | −0.71 | 0.76 |
| 35 | limb (fork) | 5.0 | 129,435 | 13,997 | 2,033 | +0.75 | −0.72 | 0.76 | −0.66 | 0.73 |
| 46758 | limb | 27.9 | 45,833 | 10,413 | 887 | +0.84 | −0.65 | 0.73 | −0.11 | 0.20 |
| 3077 | bough | 15.0 | 4,416 | 2,056 | 240 | +0.79 | −0.43 | 0.57 | −0.22 | 0.35 |
| 39939 | bough | 27.2 | 2,304 | 910 | 90 | +0.82 | −0.48 | 0.62 | +0.08 | 0 |
| 141479 | bough | 32.0 | 3,432 | 598 | 42 | +0.87 | −0.71 | 0.76 | +0.24 | 0 |
| 232 | bough | 7.7 | 3,087 | 934 | 132 | +0.75 | −0.55 | 0.67 | −0.46 | 0.60 |
| 60 | bough | 6.0 | 3,031 | 508 | 94 | +0.69 | −0.70 | 0.75 | −0.64 | 0.72 |

- **All 82 laterals.** Below zero today (memory) 0, under (a) 82, under (b) 34.
- **Under (b) the low limbs and boughs fall and the high ones stand.** Every limb and bough based below 26 m of path is below zero; 10 of the 12 based above 30 m stay above it.

**Laterals of rank ≤ 7 below zero, every decade** (arithmetic; `r1.py <dir> top`)

| year | A count | A today | A (a) | A (b) | B count | B today | B (a) | B (b) |
|---|---|---|---|---|---|---|---|---|
| 39 | 9 | 0 | 3 | 0 | 9 | 0 | 3 | 0 |
| 79 | 16 | 0 | 16 | 4 | 17 | 0 | 17 | 5 |
| 119 | 25 | 5 | 21 | 3 | 32 | 0 | 29 | 10 |
| 149 | 26 | 0 | 26 | 2 | 43 | 0 | 43 | 20 |
| 179 | 23 | 0 | 23 | 2 | 47 | 0 | 47 | 34 |
| 199 | 24 | 2 | 24 | 7 | | | | |
| 299 | 24 | 3 | 24 | 3 | | | | |

### On A at 40, 80 and 120: how far the corrected charge moves a limb's balance from today's

Mean over the laterals of rank ≤ 7 on the main line (arithmetic), with today's balance taken as this year's (Q − C)/(Q + C):

| year | mean shift, (a) − today | mean shift, (b) − today | below zero today, (a), (b) |
|---|---|---|---|
| 39 | −0.65 | −0.12 | 0, 3, 0 of 9 |
| 79 | −0.57 | −0.07 | 0, 16, 4 of 16 |
| 119 | −0.76 | −0.03 | 5, 21, 3 of 25 |

**Per limb (from the tables above).**
- **At 39.** The three fork limbs move from about +0.67 to −0.10, +0.04 and −0.23 under (a), and to +0.32, +0.43 and +0.32 under (b).
- **At 79.** They move from about +0.2 to −0.24 to −0.42 under (a), and to −0.07 to −0.19 under (b).
- **At 119.** They move from −0.03 and −0.09 to −0.40 and −0.56 under (a), and to −0.31 and −0.45 under (b).
- **Small sprigs low on the stem.** Their balance rises under both readings.

### The same K against today's per-metre living charge (arithmetic)

K × ΣL against today's living upkeep (the open wood at 0.35 a metre):

| tree | year | Σ L over living tips | today living upkeep | K·ΣL / today living | K that would equal today's living upkeep |
|---|---|---|---|---|---|
| A 1001 | 49 | 399 km | 886 | 12.9 | 0.00222 = 1/450 m |
| A 1001 | 89 | 2,230 km | 2,992 | 21.4 | 0.00134 = 1/745 m |
| A 1001 | 129 | 1,724 km | 1,612 | 30.7 | 0.00094 = 1/1,069 m |
| B 1001 | 49 | 412 km | 932 | 12.7 | 1/442 m |
| B 1001 | 89 | 2,618 km | 3,592 | 20.9 | 1/729 m |
| B 1001 | 129 | 8,103 km | 7,851 | 29.7 | 1/1,032 m |

**Why the ratio grows (arithmetic).** It grows with age because ΣL counts the trunk once per tip above it, while today's per-metre charge counts it once.

## 5. Questions for the host

1. **Calibrating K.** Against a tip of whole presence in full light, K = 1/34.8 m puts the whole charge above income from about age 45 on seed 1001 and age 39 on seed 1007, and at 2.4 to 2.8 times income at 119. Is K meant to be calibrated against a fully lit tip, as written, or against something the census reads? The lit tips at the crown's top read about 0.6 to 0.8. The mean tip reads about 0.24.
2. **Reading (a) or (b).** The young oak stays near today only under reading (b). Under (a) the two lower fork limbs stand at −0.10 and −0.23 at 39 against +0.67 today. Is a lateral's charge (a) or (b)? And if (b), what pays for the path below the limb's base, which is the trunk?
3. **Path or height for 34.8 m.** The source is a height, while path runs 1.12 to 1.30 times the height of the tallest tip. Should K be read against path, or against the height a lit tip stands at?
4. **The untested main line.** One main-line tip lives in every decade, at own 0.94 to 0.99 and 63.9 m of path at 299. Under K it costs 1.84 against its 0.97 but is never tested. Should the main line's tips be charged, and should their charge fall on anything? Today they carry the trunk's and fork rows' wood, and the tree's whole charge has no test of its own.
5. **Light rises with path here.** The balance falls with path only by its cost; the best-lit tips are the longest-path ones. Does that match the design's premise that "a tip far from the root earns what a near one earns"?
6. **Shedding alone, or growth too.** Under either reading most limbs would carry a yearly hazard of 0.2 to 0.8 from the decade they go negative. Through `kept`, a negative memory already fades a lateral's growth before it is shed. Is that the only path from the balance into growth?
7. **Dead wood in the girth.** Closed, unshed subtrees still carry their pipes into the girth below them (`retained` applies only to shed branches). The spec keeps girth out of scope. Should a closed subtree's pipes still count toward the living pipes K × L reads? As written, K × L counts only living tips, so they do not.
8. **Sapwood respiration figures.** The sources read give per-volume rates for conifers only (Ryan 1990). No source checked gives a per-pipe-length constant or ties one to potential height. Is K to be listed as an estimate with that reason, under the spec's boundary on species values?
