# fn-190 R1, probe round 5: sky light, hydraulic height, short shoots as records, 2026-10-03

## Setup

- **Code:** `crates/telperion-render/examples/growth_law/`, changed per the spec's "Host decisions after R1 round 4".
- **Sky light (`light.rs`):**
  - Leaf area is carried down five sky directions, the zenith and four at `elevation` 30° (azimuths 0°, 90°, 180°, 270°). Each is bilinear along its ray, one layer at a time.
  - A cell's light is the mean of exp(−density × leaf area) over the five, cached per cell once a cycle.
  - Each pass covers only cells whose ray can meet a leaf at that layer.
- **Phototropism:** every new metamer's direction gains `photo` × the horizontal part of the light gradient.
- **Hydraulic height:** growth efficiency is 1 − (path ÷ (hydraulic × H))², clamped to 0..1. It applies to every bud's vigour and every record's growth; path is length from the root.
- **Short shoots as records:** each long shoot keeps one record (count, total length, remembered balance) of the lateral buds that did not become long shoots, `branching × weight × (1 − development)`.
  - The record reads the shoot's light once. Its leaves gather count × light, its wood costs upkeep × length, and it draws on the laterals' side by count × light × `short`.
  - It grows in length by what it drew. In deficit it loses the same smooth share of its count and length a branch would, keeping their pipes.
  - Geometry is emitted once at the end: keyed-round(count) straight shoots of the mean length, in internodes of a short shoot's length.
  - Its count adds to the tips under it for upkeep and the pipe model, and its leaves are in the light field.
- **Not done this round: incremental whole-tree passes.** Allocation, upkeep and the balances are still full passes each cycle. Their cost is in the time split below.
- **Spruce division checked:**
  - The 0.0 came from the ground, not the flare. Low fine wood dips below y = 0 (lowest fine wood −0.008 to −0.014 H).
  - `score.rs` started counting a bin's crossings strictly above a segment's lower end, so at bin 0 the stem's first segment (lower end exactly 0) was left out. Only the below-ground branches crossed there, and they were comparable to each other.
  - Fix: count a crossing when the bin height is at or above the segment's lower end. The stem is then in bin 0 and the reading on round 4's point is 0.83.
  - The probe still grows a little wood below the ground; nothing stops it.
- **Votes:** item 5 as before.

## Points (seed 1 tuned, seed 7 held out)

| species | settings off neutral |
|---|---|
| beech | cycles 50, lambda 0.55, upkeep 0.006, density 0.02, shed 0.3, memory 0.5, tolerance −0.2, budDeath 0.05, hydraulic 1.2, photo 0.3, lean0 0.5, lean1 0.5, straighten 0.05, angle0 50, angle1 70, distich 0.8, fate 2, vigourFate 1, branching 3, short 0.2 |
| oak | cycles 45, lambda 0.56, upkeep 0.004, density 0.015, shed 0.3, memory 0.3, budDeath 0.05, hydraulic 1.2, photo 0.3, acrotony 1.5, rhythm 0.3, angle0 40 |
| spruce | cycles 36, lambda 0.6, upkeep 0.004, density 0.02, shed 0.3, memory 0.3, budDeath 0.05, hydraulic 1.2, lean1 1.4, phiStep 0.6, rhythm 0.85, angle0 70, angle1 85 |

- **The oak and spruce carry no short-shoot records.**
  - With any lateral fate below 1 (fate with an age that moves, or vigourFate 0.3 to 1), every oak and spruce point tried grew 1 node, or 3–6 m and 5–15k nodes. The whole tree's balance went negative, and the main line was shed with everything on it.
  - Their φ stays 0, so fate alone does nothing. Only vigourFate makes short shoots on them, and it killed them.
- **Sky light brightened every crown.** At round 4's density (0.01) the beech ran to the 600k node cap. Density 0.015–0.02 was needed.

## Results

| tree | crown base (old) | division | trace | fine km (today) | ms (bar) | supplementary | numeric |
|---|---|---|---|---|---|---|---|
| beech s1 | 0.08 ✗ (0.13) | 0.86 ✗ | born 43.6°, rise 11.6° ✓ | 5.9 (46.7) ✗ | 189 (79) ✗ | 2/3 ✓ | fail |
| beech s7 | 0.07 ✗ (0.04) | 0.76 ✗ | born 45.7°, rise 16.0° ✗ | 6.1 (47.1) ✗ | 195 (79) ✗ | 0/3 ✗ | fail |
| oak s1 | 0.10 ✗ (0.26) | 0.69 ✗ | Rauh ✓ | 11.3 (31.2) ✗ | 290 (50) ✗ | 2/3 ✓ | fail |
| oak s7 | 0.12 ✗ (0.34) | 0.70 ✗ | Rauh ✓ | 13.2 (35.6) ✗ | 405 (55) ✗ | 2/3 ✓ | fail |

| tree | leader | elevation | no division < 0.85 (fixed measure) | fine km (today) | ms (bar) | supplementary | numeric |
|---|---|---|---|---|---|---|---|
| spruce s1 | 1.0 ✓ | 30.0° ✓ | 0.77 ✗ | 7.4 (18.8) ✗ | 166 (48) ✗ | 2/2 ✓ | fail |
| spruce s7 | 1.0 ✓ | 28.9° ✓ | 0.84 ✗ | 7.5 (17.9) ✗ | 202 (43) ✗ | 2/2 ✓ | fail |

- **The Troll trace passes for the first time,** at seed 1, with the lean and straightening on. Seed 7 misses by 0.7° at birth.
- **The bole regressed.** The crown base went from 0.20 in round 4 to 0.07–0.12. The low sky lights the lower crown's sides, so low branches keep a positive balance.
- **Fine wood at 0.04 and 0.06** is within 6% of the 0.05 figure everywhere.
- **Visual gate (3 images: `raw/stills/sheet-r5-<species>.png`, seed 1):**
  - **Beech, fail.** A tall, very narrow column, narrower than round 4's spindle.
  - **Oak, fail.** A broad cone with a central leader, a conifer's habit.
  - **Spruce, still the nearest.** A full cone with tiers of horizontal branches and a persistent leader; it fails only on the corrected division (0.77–0.84) and its fine wood and time.
  - **No crown spread from light.** Sky light and phototropism at 0.3–1.0 did not open the beech or oak sideways.

## Size at seed 1 against seed 7

| species | nodes s1 / s7 (with emitted short shoots) | ratio | height s1 / s7 (m) |
|---|---|--:|---|
| beech | 100,786 / 104,666 | 1.04 | 32.0 / 32.2 |
| oak | 184,860 / 224,971 | 1.22 | 16.3 / 17.0 |
| spruce | 149,984 / 160,889 | 1.07 | 13.8 / 14.6 |

At the chosen points the sizes agree, but the walks below find collapses one step away.

## Walks on the beech (10 points, 9 steps; `raw/r5/walks.json`)

| walk | seed | height | nodes | crown base | collapses (under 1,000 nodes or under 5 m) |
|---|---|---|---|---|---|
| lambda 0.50 → 0.60 | 1 | 14.7 → 36.9 m, rising with lambda | 46k → 110k | 0.06–0.11 | lambda 0.589 and 0.600 grew 1 node |
| lambda 0.50 → 0.60 | 7 | 16.0 → 37.4 m, then 25.7 and 32.3 | 42k → 105k | 0.055–0.089 | none, but a drop at 0.589 (96k → 42k nodes) |
| upkeep 0.002 → 0.010 | 1 | 35.5 → 29.4 m, then 15.0 and 12.2 | 190k → 19k | 0.07–0.11 | a step at 0.0091 (66k → 26k nodes, 29 → 15 m) |
| upkeep 0.002 → 0.010 | 7 | 35.5 → 32.1 m, then 11.5, 2.4, 14.4, 16.0 | 190k → 6k | 0.07–0.39 | 0.0082 grew a 2.4 m tree |
| tolerance −0.6 → 0.2 | 1 | 18 m at −0.51 and −0.42, then 31–33 m | 34k → 111k | 0.08–0.09 | −0.6 grew 1 node |
| tolerance −0.6 → 0.2 | 7 | 17, 2.0, 15, 3.0 m, then 31–33 m | 3k → 113k | 0.07–0.41 | −0.51 and −0.33 grew 2–3 m trees |

- **Height does not follow the authored row; lambda still sets it.** From lambda 0.50 to 0.58, height rises steadily from 15 to 37 m. The hydraulic limit (1.2 × 32 m = 38 m on path length) caps the top but does not lift a tree that apical control keeps short.
- **The dials move the tree by degree inside a band, with cliffs at its edges.** Upkeep is smooth from 0.002 to 0.0082, and tolerance from −0.33 to 0.2. Outside, the tree collapses to a 2–18 m tree or one node, seed by seed. This is the bistability of rounds 2 and 3 returning: the records' upkeep and the low sky's light make the whole tree's balance cross zero, and shedding the main line takes everything.

## Where time goes after aggregation (cold run, ms)

| tree | total | light field (5 directions) | light lookups | allocate | shed | extend | straighten | records / short shoots |
|---|--:|--:|--:|--:|--:|--:|--:|--:|
| beech s1 | 196 | 70.7 | 7.0 | 26.5 | 12.6 | 22.0 | 39.5 | 19,026 / 41,536 |
| oak s1 | 307 | 69.5 | 24.8 | 84.5 | 17.5 | 80.6 | 0 | none |
| spruce s1 | 174 | 40.5 | 16.1 | 39.1 | 7.3 | 55.0 | 0 | none |

- **Aggregation carries the beech's short shoots in 19k records during growth** and emits 41.5k short shoots, at least as many nodes, only at the end. The final tree is 101k nodes, against round 4's 145k grown node by node. Light lookups fell from 60 to 7 ms because the field is cached per cell.
- **The five-direction light field is now the largest cost:** 40–71 ms, about a third.
  - Straightening is the beech's second cost (40 ms): a rotation per node per cycle.
  - The full passes (allocate, shed) are a fifth on the beech and a third on the oak, where the passes are not yet incremental and nothing is aggregated.
- **Every tree is 2.4 to 7.4× over its bar.**

## Decisions this round needs from the host

1. **Short shoots kill the oak and spruce.** On their points, any short-shoot fate collapses the tree: the records' upkeep and draw push the whole balance below zero, and the main line is shed. Should a record's draw come only from its own long shoot's surplus? Or should the main line's balance include the reserve's full cover?
2. **Shedding the main line is the cliff behind every collapse** (the lambda, upkeep and tolerance walks, and round 3's seedlings). The host ruled out a special case in round 2. Is a balance measured on the whole tree's reserve, which never lets the main line go negative while the tree has stock, within that rule?
3. **Hydraulic height caps but does not set height.** Lambda still sets height from 15 to 37 m. Should the authored height enter as the hydraulic scale with lambda fixed per architecture, or should a dial other than lambda (alpha, cycles) carry vigour?
4. **The crown does not spread** with sky light and phototropism (0.3–1.0) at these points. The low sky lights the sides but also keeps the lower crown alive, so the bole regressed from 0.20 to 0.07–0.12.
5. **Time:** the light field is the new top cost. Its five directions could run at a coarser voxel than the bud lookups, or the zenith-only pass could serve the bole. Incremental passes and straightening are next. The bars are missed by 2.4 to 7.4×.

Round 5 of six is spent. By the spec's bound, round 6 is the last before `NEEDS_HUMAN`.
