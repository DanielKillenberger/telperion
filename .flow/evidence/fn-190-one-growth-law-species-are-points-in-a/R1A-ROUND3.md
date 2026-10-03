# fn-190 R1a, probe round 3 (last): establishment curve, vigour by phi, smooth phi, 2026-10-03

## Summary

R1a is not met. Every numeric vote passes on all three species, but the beech and oak fail the visual gate and the spruce is borderline.

- **Numeric votes:**
  - Time and fine wood pass on all three species at both seeds: 0.37 to 0.97 of the bar, 1.07 to 1.26 times today's fine wood.
  - Supplementary bands pass on all three at both seeds: beech and oak 2 of 3, spruce 2 of 2.
  - Beech and oak time is close to the bar: beech 0.84 to 0.86, oak s1 0.97.
- **The establishment curve gave both broadleaves a bole and lifted the limbs.**
  - The beech's crown base is now 0.27 to 0.28 H (round 2: 0.18 to 0.21), and its projected division is 0.51 to 0.56, inside the round-1 band.
  - The oak's crown base is 0.28 to 0.29 H.
  - Major limbs (`local04`) are now born at 0.25 to 0.41 H on the beech and 0.61 to 0.72 H on the oak, instead of at 0.05 H.
- **Visual gate:**
  - **Beech, fail.** A clean bole under a dense, broad cone with a single leader: an excurrent conifer's habit, not the photograph's divided crown of ascending limbs.
  - **Oak, fail.** A clean bole and a narrow upright ovoid crown of thin branches around a leader (w/h 0.69 to 0.71, authored 1.10). It has no spreading limbs.
  - **Spruce, borderline.** It has a leader and whorled tiers in the upper half, but its lower crown is a wide, sparse haze. It is wider than the photograph (w/h 0.97 to 0.98, authored 0.62).
- **Smooth phi ends GreenLab-style factorisation, as predicted.** On the beech and oak, almost every lateral's (phi, birth year) key is distinct (1.14 to 1.15 laterals per key).
- **Every law change resets the tuning** (`FRICTION.md`). Round 2's points exploded past 1.2M grown nodes on this law.

## R1a verdict per species and seed

R1a passes when the visual gate, the supplementary bands, fine wood and time all pass (host decisions after R1a round 1, item 4).

| tree | visual gate | supplementary | fine wood | time | R1a |
|---|---|---|---|---|---|
| beech s1 | ✗ | 2/3 ✓ | ✓ | ✓ | **fail** (visual gate) |
| beech s7 | ✗ | 2/3 ✓ | ✓ | ✓ | **fail** (visual gate) |
| oak s1 | ✗ | 2/3 ✓ | ✓ | ✓ | **fail** (visual gate) |
| oak s7 | ✗ | 2/3 ✓ | ✓ | ✓ | **fail** (visual gate) |
| spruce s1 | borderline | 2/2 ✓ | ✓ | ✓ | **not passed**: every numeric vote passes; the visual gate is borderline and needs the host's or Astra's reading |
| spruce s7 | borderline | 2/2 ✓ | ✓ | ✓ | **not passed**, as seed 1 |

This was R1a's third and last round. Under the spec's bound, R1a unmet after its third round stops the run with `NEEDS_HUMAN`.

## Setup (changes from round 2, per the host's decisions after R1a round 2)

- **Establishment curve (decision 1).** Every shoot's vigour is scaled by E(t) = 1 − (1 − `est0`) × exp(−t ÷ `estYears`), where t is the tree's age. `est0` 1 is flat (neutral).
  - Vigour sets the unit's metamers: n = `n0` × vigour.
  - Vigour enters the birth jump through `vigourJump` × (1 − vigour), so the seedling's laterals jump far, live short and are pruned.
- **Vigour by phi (decision 2).** An axis's own vigour is lerp(1, `n1` ÷ `n0`, phi). One curve applies to every axis, and phi rises with the axis's years, so a major's later units are weaker. There is no per-order rule.
- **Smooth phi (decision 3).** Each year an axis moves the fraction `drift` of the way to 1. A lateral's birth phi moves toward 1 by the fraction 1 − exp(−jump). Nothing is clamped.
  - **Consequence:** phi never reaches 1, so (1 − phi)^`fate` never reaches 0, and short shoots keep a small branching chance. Round 1 and 2 relied on the clamp for unbranched short shoots.
  - All three points needed `fate` raised (beech and oak 1.5, spruce 3) to keep high-phi shoots nearly unbranched. The jump settings needed rescaling for the exponential form.
- **Major count stays a diagnostic (decision 4).** No setting was tuned to `root04`.
- **Unchanged from round 2:**
  - lifespans with pruning, keeping the pruned wood's pipes;
  - per-year straightening, whose end pass matches the yearly replay to 3.0 × 10⁻⁶° on the beech;
  - the two-ranked fix;
  - gravitropism near the base and the below-ground driver error. No tree had below-ground wood.

## Points (seed 1 tuned, seed 7 held out)

| species | settings (all others neutral) |
|---|---|
| beech | cycles 50, n0 6, n1 1.5, unit 0.0065, short 0.85, drift 0.02, est0 0.3, estYears 8, persist1 0.85, persistShape 6, reiteration 0.7, branching 1.75, fate 1.5, phiStep 0.05, zone 2.5, vigourJump 1, life0 100, life1 33, lifeShape 3, acrotony 1, lean0 0.4, lean1 1.0, ground 1, straighten 0.018, angle0 45, angle1 60, distich 0.8 |
| oak | cycles 45, n0 6, n1 1.5, unit 0.008, short 0.85, drift 0.025, est0 0.3, estYears 8, persist1 0.95, persistShape 6, reiteration 0.7, branching 1.85, fate 1.5, phiStep 0.05, zone 2.5, vigourJump 1, life0 100, life1 30, lifeShape 3, acrotony 1, lean1 0.8, eta 0.015, ground 1, angle0 25, angle1 60 |
| spruce | round 2's point with fate 3 and phiStep 0.42 (cycles 36, n0 5, n1 1.5, unit 0.007, short 0.9, drift 0.015, persist1 0.7, persistShape 10, branching 1.6, zone 0.6, rhythm 1.0, lean0 0.3, lean1 1.5, angle0 80, angle1 70, distich 1) |

## Votes, per species and seed

| tree | visual gate | supplementary | fine km (today) | ms warm (bar) |
|---|---|---|---|---|
| beech s1 | ✗ cone with a leader | 2/3 ✓ | 50.6 (46.7) ✓ | 68.4 (79.5) ✓ |
| beech s7 | ✗ as seed 1 | 2/3 ✓ | 50.4 (47.1) ✓ | 66.4 (79.5) ✓ |
| oak s1 | ✗ narrow upright ovoid | 2/3 ✓ | 39.3 (31.2) ✓ | 48.3 (49.6) ✓ |
| oak s7 | ✗ as seed 1 | 2/3 ✓ | 41.3 (35.6) ✓ | 46.0 (55.3) ✓ |
| spruce s1 | borderline | 2/2 ✓ | 20.5 (18.8) ✓ | 17.5 (47.6) ✓ |
| spruce s7 | borderline | 2/2 ✓ | 20.2 (17.9) ✓ | 17.4 (43.0) ✓ |

## Supplementary bands, major axes, and per-major secondaries

`root04` is 0 on every tree, so per-major figures are over `local04` majors: laterals off the trunk at 0.4 or more of the local trunk radius whose axis reaches 0.2 H.

| tree | junction o1 | secondaries per major (median) | length ÷ remaining | root04 (today) | local04 | local04 born at (÷ H) | per major (local04) |
|---|---|---|---|---|---|---|---|
| beech s1 | 0.36 ✓ | 19 ✗ | 0.76 ✓ | 0 (5) | 29 | 0.25 to 0.41 (first six) | 32, 29, 30, 36, 23, 27, … 5, 6, 7 |
| beech s7 | 0.36 ✓ | 20 ✗ | 0.74 ✓ | 0 (5) | 30 | 0.30 to 0.41 (first six) | 31, 30, 27, 23, 29, 23, … 7, 9, 6 |
| oak s1 | 0.24 ✗ | 9 ✓ | 0.78 ✓ | 0 (0) | 3 | 0.61, 0.63, 0.72 | 9, 12, 6 |
| oak s7 | 0.25 ✗ | 9 ✓ | 0.78 ✓ | 0 (0) | 4 | 0.64, 0.66, 0.66, 0.71 | 12, 9, 8, 5 |
| spruce s1 | 0.19 ✓ | – | – | 0 (0) | 1 | 0.71 | 4 |
| spruce s7 | 0.18 ✓ | – | – | 0 (0) | 0 | – | – |

For the spruce, "lowest lateral ≤ 0.10 H" also passes, at 0.03 at both seeds.

- **Beech secondaries per major** fell from 29 to 34 (round 2) to 19 to 20. They range from 5 to 36 along the majors and still sit above the band of 4 to 10.
- **The oak's majors are now in the band** (5 to 12).

## Bole and division (reported)

| tree | crown base p5 (lowest substantial lateral) | division (projected) | developmental trace | oak clause (division − bole ≤ 0.10) | w/h (authored) | H ÷ authored |
|---|---|---|---|---|---|---|
| beech s1 | 0.28 (0.12) | 0.56 | born 34.9°, rise 11.0° (Troll ✓) | – | 0.83 (0.72) | 1.13 |
| beech s7 | 0.27 (0.15) | 0.51 | born 34.7°, rise 10.3° (Troll ✓) | – | 0.82 (0.72) | 1.13 |
| oak s1 | 0.28 (0.41) | 0.64 | born 42.6° (Rauh ✗) | 0.36 ✗ | 0.69 (1.10) | 1.23 |
| oak s7 | 0.29 (0.42) | 0.67 | born 43.9° (Rauh ✗) | 0.38 ✗ | 0.71 (1.10) | 1.19 |
| spruce s1 | 0.05 (0.63) | no division below 0.85: 0.79 ✗ | leader 1.0 ✓, elevation 4.5° ✓ | – | 0.98 (0.62) | 1.05 |
| spruce s7 | 0.05 (0.64) | 0.85 ✓ | leader 1.0 ✓, elevation 4.6° ✓ | – | 0.97 (0.62) | 1.06 |

- **Against round 1's bands:** the beech's bole (0.18 to 0.30) and division (0.24 to 0.56) both fall inside, at both seeds. The oak's bole (0.14 to 0.32) is inside, but its division (0.15 to 0.35) and its clause miss. These vote in R1b.
- **Fine wood at the 0.04 and 0.06 boundaries:** beech 43.2 / 52.9 km (s1) and 43.2 / 52.5 km (s7). The beech is below today's at 0.04 at both seeds. Oak and spruce stay within 1% of their 0.05 figure.
- **Nodes kept against grown:**
  - beech 314k of 511k (s1) and 314k of 497k (s7);
  - oak 239k of 454k and 251k of 437k;
  - spruce 214k and 211k, none pruned.

  The broadleaves prune 37 to 47% of what they grow.

## Visual gate (3 images: `raw/stills/sheet-r1a3-<species>.png`)

Each sheet holds the S1 photograph, round 2's bare still at seed 1, and this round's bare and whole stills at seeds 1 and 7.

- **Beech, fail at both seeds.** For the first time a clean bole of about a quarter of the height, with the crown above it. The crown is a broad, dense cone around one persistent leader, with branches layered along it, which reads as a fir. It has no divided limbs and is not the photograph's dome of ascending reiterated limbs.
- **Oak, fail at both seeds.** A clean bole and a narrow, upright, ovoid crown of thin branches around a central leader, with sparse foliage. It reads as a young tree. The photograph's broad crown of spreading, forking limbs is missing.
- **Spruce, borderline at both seeds.** It has a persistent leader, and the whorled tiers read clearly in the upper half. The lower half is a wide, sparse haze, and the outline is broader than the photograph's narrow cone.

## Passes kept and deleted (cold ms, beech s1 / oak s1 / spruce s1)

| pass | what it does | ms |
|---|---|--:|
| yearly sweep | one pass a year over the living buds: vigour, unit, laterals with birth phi and lifespan | 58.9 / 49.4 / 14.9 |
| pipe | one basipetal pass over all wood grown (tips, radii) | 1.0 / 0.8 / 0.3 |
| straighten | one pass at the end, each segment's yearly turn × its years (beech only) | 12.7 / 0 / 0 |
| prune | one pass at the end: drops the pruned axes and copies the kept tree out | 18.3 / 7.5 / 8.0 |

- **Warm totals:** beech 66 to 68 ms for about 500k grown nodes; oak 46 to 48 ms for about 450k. Pruned wood is now about 40% of the work on the broadleaves.
- **Deleted, still absent:** everything round 1 deleted.

## GreenLab-style factorisation with continuous phi

| tree | laterals | distinct (phi, birth year) | laterals per key |
|---|--:|--:|--:|
| beech s1 / s7 | 33,940 / 33,191 | 29,731 / 29,122 | 1.14× / 1.14× |
| oak s1 / s7 | 23,264 / 22,485 | 20,219 / 19,640 | 1.15× / 1.14× |
| spruce s1 / s7 | 7,623 / 7,546 | 262 / 265 | 29× / 28× |

- With smooth phi, phi depends on continuous terms: the exponential jump, the establishment curve and vigour. Its keys stop repeating, and expected counts no longer factorise on the broadleaves.
- The spruce keeps 28 to 29× because its jumps add no position or vigour term (rhythm 1, no establishment). Its keys come from drift × years and one step size.
- Rounds 1 and 2 keyed repeats came from the lattice and the clamp at 1, both now gone. Answering the spec's hypothesis: GreenLab-style factorisation does not hold with continuous phi, as soon as phi takes continuous inputs.

## What R1a established over three rounds

- **Cost:** one open-loop organogenesis pass grows today's fine wood or more within the bar on all three species, in every round.
  - The bar is nearly used up only on the beech and oak in round 3, where 37 to 47% of grown wood is pruned.
  - Pruning by lifespan costs growth that is later discarded. A light-driven law in R1b would shed the same wood, but would not have to grow it as far.
- **Architecture:** each mechanism moved a measure as the host predicted:
  - birth-age spread → limbs (round 2);
  - establishment → a bole and limbs born higher (round 3);
  - vigour by phi → fewer secondaries per major: beech 34 → 19, oak into its band.

  None made the broadleaf crown read as its photograph. The trunk persists as a leader through the crown on both in every round (stem top 1.0 on every broadleaf tree but round 1's oak at seed 7): an excurrent habit. Neither the trunk's apical persistence decaying with phi nor partial reiteration handed the crown over to limbs.
- **Size has no bound without feedback.** Each law change moved the trees between explosion and starvation, and every round started by refinding a viable size (`FRICTION.md`, three entries).

## Decisions this round needs from the host

1. **R1a is unmet after its third round.** By the spec's bound this stops the run with `NEEDS_HUMAN`. The numeric votes all pass. The visual gate fails on the beech and oak, and the spruce's is borderline. Should the spruce's gate be read by Astra or the owner before the report goes to the owner?
2. **The leader never yields.** In all three R1a rounds the trunk is the tallest axis through the crown. Its persistence decays with phi, but phi rises slowly at drift 0.02 to 0.025, and relays inherit a young phi through partial reiteration. That is a question about how Troll's and Rauh's apical take-over is meant to happen in the law, and it is the host's.
3. **Factorisation.** It fails with continuous phi. Should R2's production design drop GreenLab-style factorisation as a speed lever?
4. **Pruned-growth cost.** About 40% of the broadleaves' growth time goes to wood that is later pruned. Should R2 record this as the cost R1b's shedding must beat?

Round 3 of three for R1a is spent. No further round is started.
