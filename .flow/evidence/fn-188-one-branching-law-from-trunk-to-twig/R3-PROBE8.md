# fn-188 R3, round 8: shoot form by vigour, shoot-local two-ranked buds, 2026-10-03

## Setup

- **Code:** the adapter fix (`9e4b8892`: unique identities; clumping 0 by overlay), extended, committed as `85f2b5a8` and reverted in `1220dffe`.
- **Foliage:** the pipeline's own wood and foliage via `executor::expand(..).mesh()`, with the beech's shipped canopy rows and `limbClumping` 0.
- **Runs:** release on the Ryzen 9 5950X, 16 threads.
- **Rows:** `raw/probe8/`: `diag.jsonl`, `bases.jsonl`, `walk.jsonl`, `walk-shoots.jsonl` and `timing.jsonl`.
- **Texture review, section 2:** short shoots with a few short internodes and undeveloped buds against long shoots with functional laterals, and shoot-local two-ranked organisation.

## 1. Diagnosis (seed 1, no renders)

**Method:** a run is a tip back to where its non-structural wood began (a lateral start, a branching node or structural wood); an axis is non-structural wood from its start along its continuations; generations count lateral starts from structural wood; each leaf goes to the class of the nearest segment midpoint within 0.6 m (none unmatched).

| | shipped beech (no overlay) | round 7 + adapter fix |
|---|--:|--:|
| leaves | 5,828,744 | 1,625,505 |
| nodes structural / branch / twig | 3,849 / 117,870 / 91,324 | 318 / 22,124 / 27,069 |
| wood m structural / branch / twig | 2,448 / 25,623 / 22,831 | 69 / 5,531 / 6,767 |
| non-structural axes | 91,324 | 10,307 |
| axis length, median | 0.25 m | 0.75 m |
| terminal run p10 / p50 / p90 / max | 0.25 / 0.25 / 0.50 / 1.39 m | 0.25 / 0.50 / 1.25 / 5.50 m |
| run, nodes p50 / p90 / max | 1 / 2 / 6 | 2 / 5 / 22 |
| laterals per metre of non-structural wood | 1.57 | 0.82 |
| laterals per axis, mean | 0.84 | 0.98 |
| generations p50 / p90 / max | 1 / 2 / 3 | 4 / 6 / 13 |
| leaf share structural / branch / twig | 2.6% / 50.4% / 47.0% | 0.6% / 75.4% / 24.0% |
| leaves per metre, branch / twig wood | 115 / 120 | 222 / 58 |

**Leaves per cluster** (rows, both trees): short shoots carry 8 leaves (`shortShootLeaves`) every 0.03 m, fanned over ±90°. Twig wood carries a leaf every `twig.internodeLength`, 0.05 m.

**The differences that plausibly give masses rather than sleeves:**

1. **About 4x more fine wood:** 48.5 km against 12.3 km.
2. **Shipped fine wood is single 0.25 m twigs,** set at 1.57 per metre and shallow (1 to 3 generations). The probe's is longer runs at 0.82 per metre, 4 to 13 generations deep.
3. **Shipped leaves are split evenly between twig and branch wood** at the same density per metre (115 against 120), so foliage fills the volume around branches. The probe puts 75% of its leaves on branch wood at 222 per metre against 58 on twigs, so foliage sleeves the axes.

## 2–3. The two mechanisms

**Shoot form follows vigour.** A bud given resource v grows as before, with L = 1 / (1 + e^(−(v − s)/0.25)) setting its form.

- Its internodes are scaled by 0.25 + 0.75·L.
- Its metamers' lateral buds receive light × L, so low-vigour shoots develop no laterals.
- s is the vigour scale; s = 0 is dormant (L = 1).

**Two-ranked buds.** The bud bearing blends from the global node-index × 137.5° (blend 0) to shoot-local alternation (blend 1).

- Successive metamers of a shoot take opposite sides (180°) about the shoot's horizontal side vector (shoot × up).
- The weight is D × (1 − 0.5^depth), so leaders (depth 0) stay spiral.
- **One deviation from the brief:** I first weighted it by plagiotropy p as well. That made it inert on base B, where p is 0, so p was dropped. The plane is the shoot axis with its horizontal side vector.

**Dormant parameters reproduce the adapter-fix tree byte for byte** (digest `4d2531e5…`, 49,512 nodes).

**At λ 0.5 vigour cannot vary.** Every extending bud received v = 2.0 exactly (25,773 of 25,773, as the review derived). Shoot form is therefore inert at λ 0.5.

- At λ 0.55 vigour spreads from 1.0 to 6.4 (p10 1.36, p50 1.87, p90 2.82), so **the walk runs at λ 0.55**.
- This changes the base from round 5's λ 0.5.

## 4–5. Bases and walk (seed 1, pinned pose; coverage and width ÷ height as in rounds 4 to 7)

**Base A** is round 7's leaders-only base (p 1, 50°). **Base B** is round 5's full scaffold (c6, ξ 0.2, η 0.1, 40°, p 0). Both have the adapter fix and run at λ 0.5 with the mechanisms dormant.

| base | nodes | leaves | tree / leaf cover | width ÷ height | limbs ≥ 0.4 trunk | leaf share s / b / t | run p50 / p90 / max | lat/m | gens p50 / max | probe + mesh ms | shipped G2 build ms |
|---|--:|--:|---|--:|--:|---|---|--:|---|--:|--:|
| A | 49,512 | 1,625,505 | 0.889 / 0.834 | 0.822 | 3 | 0.6 / 75.4 / 24.0% | 0.50 / 1.25 / 5.5 | 0.82 | 4 / 13 | 67 + 646 | 1,345 |
| B | 52,550 | 1,581,215 | 0.877 / 0.823 | 0.845 | 7 | 12.5 / 58.3 / 29.2% | 0.50 / 1.25 / 4.0 | 0.80 | 2 / 10 | 58 + 635 | 1,371 |

**Base chosen: B,** from one contact sheet of both bases bare and whole beside B-BARE and B-WHOLE.

- B's whole crown is less streaky, with mottled clumps; A's still shows vertical combing and the cleft.
- B's bare form is round 5's.

**The walk on base B at λ 0.55.** Short and long shoots are counted as extension events with v below and above s.

| step | short / long shoots | nodes | leaves | tree / leaf cover | w ÷ h | limbs ≥ 0.4 | leaf share s / b / t | run p10 / p50 / p90 / max m | lat/m grown | gens p50 / max | mesh ms |
|---|---|--:|--:|---|--:|--:|---|---|--:|---|--:|
| s 0, D 0 | 0 / 25,884 | 41,070 | 1,596,830 | 0.880 / 0.820 | 0.838 | 8 | 13.6 / 57.6 / 28.8% | 0.30 / 0.54 / 1.23 / 4.21 | 0.99 | 2 / 11 | 633 |
| s 1.5, D 0 | 4,936 / 23,077 | 44,924 | 1,554,235 | 0.885 / 0.823 | 0.843 | 8 | 12.6 / 56.1 / 31.3% | 0.25 / 0.51 / 1.13 / 3.57 | 1.06 | 2 / 13 | 616 |
| s 1.9, D 0 | 14,778 / 15,368 | 48,683 | 1,431,573 | 0.875 / 0.804 | 0.840 | 11 | 12.7 / 53.8 / 33.5% | 0.17 / 0.48 / 1.15 / 4.74 | 1.08 | 2 / 14 | 566 |
| s 2.5, D 0 | 25,656 / 5,890 | 50,117 | 984,435 | 0.829 / 0.721 | 0.839 | 11 | 16.7 / 41.9 / 41.4% | 0.13 / 0.50 / 1.41 / 4.90 | 1.05 | 2 / 18 | 396 |
| **s 0, D 1** | 0 / 26,080 | 41,345 | 1,635,512 | 0.884 / 0.825 | 0.833 | 8 | 13.2 / 58.8 / 28.1% | 0.30 / 0.53 / 1.22 / 3.94 | 0.97 | 2 / 9 | 697 |
| s 1.9, D 1 | 15,109 / 15,539 | 49,371 | 1,483,355 | 0.882 / 0.807 | 0.843 | 11 | 12.0 / 55.0 / 33.0% | 0.18 / 0.47 / 1.15 / 5.42 | 1.08 | 2 / 17 | 596 |

**Traced at s 0, D 1:** angle to vertical o1–o4 37.7 / 43.2 / 50.2 / 76.1°; tortuosity o0–o3 1.037 / 1.005 / 1.004 / 1.004.

**The shoot-form walk** shifts leaves from branch to twig wood (twig share 28.8 → 41.4%), adds generations (max 11 → 18) and makes very short runs common (p10 0.30 → 0.13 m). It also removes leaves (1.60M → 0.98M) and fine wood, and the median run stays about 0.5 m against the shipped 0.25 m. **Two-ranked bearings** change little at s 0: leaves +2.4%, cover +0.004.

## 6. Judged on bare (owner's change of emphasis)

The bare-measure code was committed as `7a61270d` and reverted in `025740a3`. All measures come from the bare frame at the pinned pose.

**Measures:** width ÷ height is the widest silhouette row ÷ its height; clear bole is the first row up from the base wider than 3× the trunk, as a share of the height; edge share is non-structural wood within 0.2 of the widest radius of the lobed outline (horizontally or from the top); tips per metre counts twig tips projected within 6 px of their row's silhouette edge, per metre of left and right outline at the target's distance.

**B-BARE (`fasy896`), by hand off round 4's gridded view:** trunk base y ≈ 370, first fork y ≈ 280; the crown leaves the frame at the top and left and reaches x ≈ 255. So the clear bole is ≤ 0.24 and width ÷ height ≈ 0.69, both clipped; edge share and tips are not measurable.

| | width ÷ height | clear bole ÷ height | fine wood m | edge share | tips per m outline | limbs ≥ 0.4 trunk |
|---|--:|--:|--:|--:|--:|--:|
| B-BARE (hand, clipped) | ≈ 0.69 | ≤ 0.24 | – | – | – | 4–6 (fn-182 R1) |
| shipped beech | 0.891 | 0.098 | 48,454 | 0.375 | 0.68 | 6 |
| base A | 0.791 | 0.048 | 12,298 | 0.362 | 1.10 | 3 |
| base B | 0.831 | 0.054 | 11,876 | 0.381 | 1.13 | 7 |
| s 0, D 0 | 0.830 | 0.040 | 12,190 | 0.399 | 1.08 | 8 |
| s 1.5, D 0 | 0.824 | 0.036 | 11,433 | 0.429 | 1.30 | 8 |
| s 1.9, D 0 | 0.825 | 0.038 | 10,326 | 0.455 | 1.29 | 11 |
| s 2.5, D 0 | 0.827 | 0.067 | 7,222 | 0.465 | 0.99 | 11 |
| **s 0, D 1** | 0.815 | 0.048 | 12,265 | 0.396 | 1.16 | 8 |
| s 1.9, D 1 | 0.840 | 0.050 | 10,455 | 0.460 | 1.34 | 11 |

The walk steps run at λ 0.55; the bases at λ 0.5.

**One bare contact sheet was viewed:** B-BARE beside the shipped beech, both bases and the six steps. A whole-tree sheet had been viewed before the change of emphasis.

- **No probe step reads like B-BARE.** The shipped beech is the closest: long, straight limbs fanning from a fork, with a dense fine haze.
- **The probe's crowns are rounded and subdivide early,** with no long straight limbs reaching the top. The clear bole is 0.04 to 0.07 of the height, against B-BARE's ≤ 0.24 and the shipped 0.098: markers fill the envelope down to its crown base (0.06 of the height), so branches grow there.
- **Shoot form at s 1.5 to 2.5 adds long looping shoots,** the curving lines seen in leaf.
- **Base A is two-lobed and combed.**
- **Picked on bare: s 0, D 1:** no looping shoots and an even mesh. On the bare measures it differs from base B by little.
- **In leaf** it is the densest and most even of the walk (0.884 / 0.825 cover, 1,635,512 leaves, width ÷ height 0.833). No step reads as B-WHOLE's broad, soft, layered masses.

**Full stills, pinned pose (not viewed):**

- `beech-probe8-baseA-bare.png`, `beech-probe8-baseA-whole.png`
- `beech-probe8-vs0-d1-bare.png`, `beech-probe8-vs0-d1-whole.png`

**Thumbnails:**

- `beech-probe8-thumb-<step>-bare.png`, for the six steps and both bases;
- `beech-probe8-thumb-<step>-whole.png`, for the six steps.

**Rows:** `raw/probe8/bare.jsonl`.

**Chosen values:** base B (round 5's full scaffold, c6, ξ 0.2, η 0.1, 40°), λ 0.55, leaf pipes, no floor, unique identities, clumping 0, shoot form dormant (s 0), two-ranked blend D 1.

**Timing, chosen combination:**

- **Probe:** warm median of 3. **Mesh:** warm median of 2. **Shipped:** `mesh::build` of the same G2 family.
- **Not included:** the scaffold build, about 3 ms.
- The shipped beech preset itself built in 3,221 ms in round 4.

| seed | nodes | leaves | probe ms (cold) | mesh ms | probe + mesh | shipped G2 build | limbs ≥ 0.4 trunk |
|---|--:|--:|---|--:|--:|--:|--:|
| 1 | 41,345 | 1,635,512 | 59.0 (61.8) | 657 | 716 | 1,357 | 8 |
| 7 | 39,250 | 1,586,209 | 54.6 (61.7) | 642 | 696 | 1,368 | 4 |
