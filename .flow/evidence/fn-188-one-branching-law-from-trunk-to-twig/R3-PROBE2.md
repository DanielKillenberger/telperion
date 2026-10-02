# fn-188 R3, round 2: competition grown from the scaffold, 2026-10-02

## What was built

- **Code:** round 1's probe (`1d5390ee`), changed, committed as `2dd97eb2` and reverted in `fe94621c`. Nothing is wired into the pipeline.
- **Start:** the direct build's scaffold for the preset, every node before `crossover`.
  - A node is a lateral unless it is its parent's terminal, non-codominant continuation, so a fork's siblings are laterals.
  - The beech takes fn-182's G2 overlay (`raw/families/G2.json`); every other preset takes its shipped values.
- **What grows:** every scaffold node bears one axillary bud, and every scaffold tip a terminal bud. Competition grows everything finer: the extended Borchert-Honda model as in round 1, now with any number of laterals per node.
- **Deleted:**
  - the twig layer, the `limbRadius` gate and the tip shoot;
  - the envelope term in light, and the wall stop. The shadow field is the only environment; outside the grid, light is full.
- **Kept:**
  - The scaffold is never shed.
  - Radii come from one pipe pass, with each tip one unit, the preset's `forkExponent`, and the root held at the shipped root radius.
- **Scaled to the preset:** the metamer unit is height ÷ 128 (0.25 m on the beech, 1.16 m on telperion), the voxel is twice the unit, and the bud angle is the preset's `twigs.angle`.
- **Fixed:** λ 0.5, α 2, a 0.03, b 1.8, qmax 6, ξ 0.3, η 0.15, shed threshold 0.01, 12 cycles.
- **Overgrowth guard:** shoots grow bud by bud and each new metamer casts its shadow at once. A metamer whose light would fall under `q_stop` (0.2) is not grown, and the shoot stops there.
- **The measures:**
  - The traced measures and strong limbs are round 1's. Strong limbs are first-order axes off the trunk axis born at 0.4 or more of the trunk.
  - **Fine m/m** is the length of the non-thick laterals born on wood of 0.3 or more of the root radius, with their subtrees, per metre of that wood.
  - **Tone** draws both trees as the probe's tubes from one pose: the scaffold alone minus the whole. The beech uses R1's pinned pose and crown box. Every other preset uses the hero pose of its shipped tubes and the box where the four frames differ.
  - **Escape** is how far grown wood stands outside the envelope.
- **Timing:** Ryzen 9 5950X, release.
  - **Shipped:** `skeleton_ms`, warm median of 3 after a cold build. This covers the scaffold, the twig layer and the budget search.
  - **Probe:** competition alone, warm median of 3 after cold. The scaffold it starts from is the shipped build's. The scaffold's own cost cannot be split out without code: `twigs.generations` has a floor of 1.

## Every preset, seeds 1 and 7

| preset s | shipped nodes / leaves | ship skel ms | probe kept / grown | probe ms (cold) | strong probe (ship) | fine m/m probe (ship) | tone probe (ship) | tip r cm | escape p95 / max m |
|---|---|--:|---|--:|---|---|---|--:|---|
| ordinary 1 | 11,568 / 40,659 | 41 | 28,634 / 44,556 | 22 (23) | 5 (6) | 141 (183) | 0.029 (0.003) | 0.33 | 2.6 / 5.0 |
| ordinary 7 | 11,934 / 41,877 | 43 | 31,100 / 48,281 | 23 (27) | 5 (5) | 210 (186) | 0.030 (0.004) | 0.31 | 3.5 / 6.2 |
| oak 1 | 133,635 / 767,026 | 31 | 159,050 / 275,052 | 147 (142) | 4 (3) | 578 (1,262) | 0.015 (0.006) | 0.12 | 2.1 / 9.3 |
| oak 7 | 143,703 / 903,084 | 37 | 179,820 / 303,952 | 162 (185) | 3 (4) | 558 (3,600) | 0.021 (0.007) | 0.11 | 4.5 / 12.6 |
| spruce 1 | 96,174 / 7,414,510 | 32 | 50,537 / 71,187 | 38 (38) | 12 (5) | 611 (2,765) | 0.009 (0.004) | 0.12 | 1.6 / 2.8 |
| spruce 7 | 91,077 / 7,064,601 | 31 | 52,291 / 74,641 | 39 (42) | 12 (7) | 654 (2,628) | 0.008 (0.003) | 0.12 | 1.4 / 2.6 |
| birch 1 | 86,326 / 258,111 | 25 | 92,453 / 156,133 | 78 (83) | 5 (6) | 278 (990) | 0.027 (0.013) | 0.09 | 1.8 / 4.1 |
| birch 7 | 91,743 / 279,485 | 26 | 87,493 / 143,690 | 73 (84) | 6 (7) | 302 (1,132) | 0.018 (0.013) | 0.09 | 2.0 / 4.4 |
| date palm 1 | 552 / 6,554 | 2 | 9,179 / 11,436 | 6 (7) | 14 (0) | 104 (0) | 0.012 (0.002) | 0.40 | 2.4 / 3.1 |
| date palm 7 | 553 / 6,554 | 2 | 8,682 / 10,802 | 7 (6) | 12 (0) | 88 (0) | 0.011 (0.002) | 0.41 | 2.4 / 3.0 |
| telperion 1 | 77,324 / 550,030 | 350 | 15,812 / 23,897 | 11 (11) | 1 (4) | 99 (335) | 0.059 (0.024) | 9.5 | 15.2 / 24.0 |
| telperion 7 | 18,140 / 128,852 | 51 | 13,826 / 21,968 | 10 (11) | 2 (3) | 81 (74) | 0.055 (0.021) | 9.9 | 16.6 / 24.9 |
| laurelin 1 | 53,861 / 392,944 | 97 | 101,666 / 178,127 | 87 (93) | 6 (7) | 158 (67) | 0.087 (0.028) | 10.8 | 11.6 / 76.3 |
| laurelin 7 | 75,239 / 543,814 | 172 | 81,700 / 145,356 | 82 (82) | 6 (8) | 107 (89) | 0.081 (0.027) | 11.6 | 0.0 / 34.8 |
| beech G2 1 | 82,636 / 1,013,806 | 22 | 66,101 / 110,971 | 54 (60) | 24 (6) | 89 (185) | 0.030 (0.008) | 1.19 | 4.1 / 8.1 |
| beech G2 7 | 92,606 / 1,035,559 | 24 | 66,340 / 110,650 | 53 (60) | 18 (3) | 113 (155) | 0.029 (0.011) | 1.19 | 3.5 / 6.9 |

- **The probe has no leaves.** The whole beech still uses six borrowed leaves at each node that carries two tips or fewer.
- **Telperion:** with no gate, every scaffold station of both seeds grows fine wood. At s7 that is 13,826 nodes; on the shipped build the tip shoots alone carried its twig layer (fn-189 `TRACE.md`).
- **The Two Trees' tips come out at 9.5 to 11.6 cm in radius:** a 1.03 to 1.16 m unit, roots of 7.3 to 7.4 m and exponents of 2.15 and 2.7.
- **The date palm grows branching wood up its trunk.** 98% of it stands outside its envelope.

**Escape.**

- On the forest presets, 19 to 68% of the grown wood stands outside the envelope, at p95 1.4 to 4.5 m.
- Telperion reaches 24 to 25 m out (p95 15 to 17 m). With no envelope term, the outer buds get full light, so growth runs outward along the shadow's gradient.
- Laurelin's 76 m (s1) and 35 m (s7) maxima leave the shadow grid (the envelope's radius plus 0.3 of the height). Outside the grid the shadow is zero, so light is full there.
- The beech still shows tufts above the crown top.

## Continuity, beech G2, seed 1

One parameter moved at a time. "Strong (grown)" counts the strong limbs, with those whose base is grown wood rather than scaffold in brackets.

| step | strong (grown) | o1 / o2 / o3 | mid / end | o3/o1 | tips | tip r cm | fine m/m | tone | kept / grown | ms | esc p95 |
|---|---|---|---|--:|--:|--:|--:|--:|---|--:|--:|
| λ 0.44 | 20 | 0.59 / 0.60 / 0.75 | 0.79 / 1.00 | 0.176 | 21,489 | 1.34 | 118 | 0.018 | 58,115 / 90,503 | 49 | 2.6 |
| λ 0.47 | 23 | 0.51 / 0.45 / 0.66 | 0.79 / 1.00 | 0.141 | 25,879 | 1.59 | 114 | 0.026 | 64,283 / 101,982 | 51 | 3.7 |
| λ 0.50 | 24 (6) | 0.42 / 0.40 / 0.67 | 0.79 / 1.00 | 0.110 | 28,796 | 1.19 | 89 | 0.030 | 66,101 / 110,971 | 54 | 4.1 |
| λ 0.51 | 21 (4) | 0.39 / 0.40 / 0.68 | 0.79 / 1.00 | 0.108 | | | | | | | |
| λ 0.52 | 32 (17) | 0.44 / 0.51 / 0.67 | 0.79 / 0.99 | 0.109 | | | | | | | |
| λ 0.53 | 37 (21) | 0.45 / 0.50 / 0.63 | 0.79 / 0.98 | 0.105 | 22,201 | 1.25 | 92 | 0.037 | 61,018 / 109,108 | 60 | 6.8 |
| λ 0.56 | 48 | 0.39 / 0.51 / 0.57 | 0.68 / 0.91 | 0.137 | 23,914 | 1.30 | 98 | 0.050 | 77,306 / 113,995 | 59 | 14.5 |
| a 0.02 / 0.025 / 0.03 / 0.04 / 0.05 | 23 / 22 / 24 / 21 / 21 | o2 0.31 → 0.52 | 0.79 / 1.00 | 0.108–0.121 | 46,083 → 15,302 | 1.03 → 1.47 | 147 → 52 | 0.035 → 0.024 | 103k / 168k → 36k / 61k | 87 → 31 | 4.1–4.2 |
| cycles 8 / 10 / 12 / 14 / 16 | 23 / 23 / 24 / 25 / 24 | o3 0.60 → 0.69 | 0.79 / 0.96–1.00 | 0.127 → 0.112 | 19,746 → 38,712 | 1.51 → 0.99 | 70 → 129 | 0.017 → 0.045 | 42k / 55k → 93k / 191k | 24 → 102 | 2.5 → 6.0 |
| exponent 2.5 / 2.7 / 2.9 / 3.1 / 3.3 | 19 / 22 / 24 / 25 / 25 | o1 0.37 → 0.47 | 0.76 → 0.81 / 1.00 | 0.077 → 0.144 | 28,796 | 0.65 → 1.88 | 193 → 62 | 0.024 → 0.034 | unchanged | 55 | 4.1 |
| codominance 0.2 / 0.25 / 0.3 / 0.35 / 0.4 | 19 / 22 / 24 / 21 / 21 (ship 10 / 4 / 6 / 6 / 6) | o2 0.28 → 0.40 → 0.38 | 0.79 / 1.00 | 0.110–0.121 | 26.8k–28.8k | 1.2–1.5 | 77–95 | 0.028–0.030 | 62k–66k | 51–53 | 4.1–4.3 |
| q_stop 0 / 0.1 / 0.2 / 0.3 / 0.4 | 24 / 22 / 24 / 20 / 24 | o1 0.40–0.42 | 0.79 / 0.95–1.00 | 0.109–0.112 | 31,427 → 26,891 | 1.15 → 2.01 | 96 → 86 | 0.030 → 0.029 | 71k / 121k → 63k / 100k | 59 → 51 | 4.2 |

**Moving by degree:**

- `a` sets how fine the periphery is: tips 46k to 15k, fine wood 147 to 52 m/m, tone 0.035 to 0.024, time 87 to 31 ms.
- Cycles set its depth: tips 20k to 39k, escape p95 2.5 to 6.0 m, and grown nodes 55k to 191k.
- The exponent changes radii only; the topology is identical.
- The limb count stays within 19 to 25 under `a`, cycles, exponent and `q_stop`.

**Where the limb count jumps:**

- **λ:** 24, 21, 32, 37 at 0.50 to 0.53, with the largest step between 0.51 and 0.52.
  - The strong limbs whose base is grown wood go from 4 to 17.
  - Over the same step, the trunk axis's top rises from 27.9 to 33.7 m, then to 37.0 m at 0.53, against an envelope of 32 m.
  - Once λ favours the main axis, the leader's terminal bud grows up past the envelope, with no wall to stop it. Laterals off that new, thin leader pass the 0.4-of-the-trunk-at-the-junction count.
  - At seed 7, λ 0.50 → 0.53 gives 18 → 27 strong limbs.
- **Codominance:** the shipped scaffold's own limb count jumps (10, 4, 6, 6, 6), and the probe follows it at 19 to 24. 0.35 and 0.4 build the same scaffold at seed 1.
- **Overgrowth:** `q_stop` cuts grown nodes only from 121k to 100k (0 → 0.4). Shed wood is still 37 to 41% of what grows.

## Speed

Beech G2 s1, λ 0.5, by stage (ms):

| Stage | ms |
|---|--:|
| extend, incl. shadow casting | 37.6 |
| shed | 16.5 |
| light | 3.7 |
| basipetal | 3.1 |
| allocate | 1.9 |
| pipe | 1.2 |
| setup | 1.0 |

- **Work done:** 1.9M node visits and 71M voxel updates, against 9.9M and 218M in round 1 at λ 0.5.
- **Competition alone against the shipped skeleton:**

| Presets | Probe against shipped skeleton |
|---|---|
| beech, birch, oak, spruce | 1.2 to 5x slower: oak 147 to 162 ms against 31 to 37 ms; spruce 38 against 31 to 32 ms |
| ordinary | about 0.5x the shipped time |
| telperion | 11 ms against 51 to 350 ms |
| laurelin | 82 to 87 ms against 97 to 172 ms |

- **The scaffold's own build** comes on top of the probe's time and is not measured here.

## The stills (two viewed)

- **`beech-probe2-bare.png`** (pinned pose):
  - A few strong scaffold limbs, forked low, divide into progressively finer branches.
  - Dense fine wood gathers in clumps at limb ends and along the limbs, and the interior limbs stay visible.
  - Tufts stand above the crown top.
- **`telperion-probe2-s7-bare.png`** (hero pose):
  - A massive trunk with short, thick scaffold limbs.
  - Fine wood grows in tufts at stations and tips along the whole height, and the trunk is bare between the tufts.
  - The fine wood is coarse: tips about 10 cm in radius.
  - The tube mesh shows dark seams at segment joins on the trunk.
- **`beech-probe2-whole.png`** was written but not viewed.

## Files

- `raw/probe2.jsonl`: every run, with the shipped and probe traced measures, stage times, escape and parameters.
- `raw/stills/beech-probe2-bare.png`, `beech-probe2-whole.png`, `telperion-probe2-s7-bare.png`.
