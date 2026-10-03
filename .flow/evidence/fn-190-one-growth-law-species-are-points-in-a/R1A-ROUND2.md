# fn-190 R1a, probe round 2: limbs from the spread of birth ages, lifespans by phi, 2026-10-03

## Summary

Limbs now form, but in the wrong place: every major axis on the beech and oak is born in the seedling's first unit.

- **Time and fine wood still pass at both seeds** for all three species: 0.43 to 0.62 of the bar, 1.03 to 1.36 times today's fine wood.
- **Limbs exist now.** The beech has 6 major axes at both seeds (`root04`; today's beech has 5). The oak has 3 and 2.
- **Every major axis is born at 0.05 H.** The first-year laterals live longest and carry the most wood, so the beech is a low fan of arching limbs from the base with a thin leader above it, and the oak is a broom. Both still fail the visual gate.
- **Supplementary bands are 1 of 3** on the beech and oak at both seeds. Junction ratio and secondaries per major miss: majors carry 18 to 68 substantial secondaries, against the band's 4 to 10.
- **The spruce passes every vote numerically.** Visually it is borderline, but better than round 1: tiers show in the upper crown, the lower crown is still a haze, and no wood is below the ground.
- **The straightening pass is exactly the yearly integral.** It differs from a year-by-year run by at most 3 × 10⁻⁶ degrees.

## Setup (changes from round 1, per the host's decisions after R1a round 1)

- **Birth jump (decision 1).** A lateral's birth phi is the parent's phi plus three terms:
  - `phiStep`;
  - `zone` × (1 − u), where u is the lateral's place along the annual unit from the base;
  - `vigourJump` × (1 − the parent shoot's vigour), where vigour is the shoot's metamers ÷ `n0`.

  Distal laterals of young, vigorous shoots jump only `phiStep` (0.05 here); the rest jump far.
- **Lifespan (decision 1).** Each lateral axis gets a lifespan of `life0` to `life1` years, blended by 1 − (1 − phi)^`lifeShape` and keyed-rounded.
  - An axis also dies when the axis carrying it dies.
  - Its buds stop at death, and the wood it grew is pruned at the end.
  - Its pipes stay in the pipe model: tips are counted over all wood grown, the pruned included.
  - The trunk has no lifespan. Its apical persistence still decays with its phi (drift), as in round 1.
  - No limb count is authored.
- **Straightening (decision 3).** Each segment turns toward up at a per-year rate of `straighten` × (1 − its axis's birth phi), over its years, stopping at vertical. Only its own segment turns. The rate depends on nothing that changes during growth, so the one pass at the end is the yearly sum exactly. The diagnostic run replays the turn year by year and records the largest difference (`law.straighten_yearly_gap_deg`): beech 3.1 × 10⁻⁶° and 3.2 × 10⁻⁶°.
- **Wood below the ground (decision 5).** Any node below y = 0 is now a driver error: the tree is refused, never scored or clamped. Two fixes in the law:
  - **A two-ranked defect.** The `distich` blend rotated laterals about the shoot's horizontal side, so "two ranks" put them above and below a slanted shoot instead of to its sides. The spruce's (distich 1) and the beech's (0.8) laterals pointed up and down. The rotation axis is now the shoot's own up, so laterals swing to its sides. This alone lifted the spruce off the ground: crown base −0.02 → +0.06 H.
  - **Gravitropism near the base** (`ground`, reach in internodes). An upward pull of weight exp(−height ÷ (`ground` × internode)), which is 1 at the ground and fades with height. At `ground` 1 the oak's 4 and 17 below-ground nodes are gone. The spruce does not use it.
- **No size cap (decision 2).** Partial reiteration stays (0.7). `maxNodes` was raised to 1.2M grown nodes for the beech and oak because pruned wood counts as grown. No point reached it.
- **Votes (decision 4).** R1a votes on the visual gate, the supplementary bands, fine wood and time. The bole, division, the developmental traces and the oak's "division − clear bole ≤ 0.10" clause are reported.
- **Bars:** as round 1, today's recorded rows × 1.5. They are stricter than this session's re-measure.

## Points (seed 1 tuned, seed 7 held out)

| species | settings (all others neutral) |
|---|---|
| beech | cycles 50, n0 6, n1 1.5, unit 0.009, short 0.85, drift 0.03, persist1 0.85, persistShape 6, reiteration 0.7, branching 2.2, phiStep 0.05, zone 1.5, vigourJump 2, life0 100, life1 42, lifeShape 1.5, acrotony 1, lean0 0.4, lean1 1.0, ground 1, straighten 0.013, angle0 45, angle1 60, distich 0.8 |
| oak | cycles 45, n0 6, n1 1.5, unit 0.012, short 0.85, drift 0.04, persist1 0.92, persistShape 6, reiteration 0.7, branching 2.0, phiStep 0.05, zone 1.5, vigourJump 2, life0 100, life1 38, lifeShape 1.5, acrotony 1, lean1 0.8, eta 0.015, ground 1, angle0 25, angle1 60 |
| spruce | unchanged from round 1 (cycles 36, n0 5, n1 1.5, unit 0.007, short 0.9, drift 0.015, persist1 0.7, persistShape 10, branching 1.6, phiStep 0.3, zone 0.6, rhythm 1.0, lean0 0.3, lean1 1.5, angle0 80, angle1 70, distich 1) |

## Votes, per species and seed

| tree | visual gate | supplementary | fine km (today) | ms warm (bar) |
|---|---|---|---|---|
| beech s1 | ✗ fan of limbs from the base | 1/3 ✗ | 52.4 (46.7) ✓ | 49.6 (79.5) ✓ |
| beech s7 | ✗ as seed 1 | 1/3 ✗ | 52.3 (47.1) ✓ | 47.2 (79.5) ✓ |
| oak s1 | ✗ broom, no trunk | 1/3 ✗ | 36.4 (31.2) ✓ | 23.6 (49.6) ✓ |
| oak s7 | ✗ as seed 1 | 1/3 ✗ | 36.6 (35.6) ✓ | 23.5 (55.3) ✓ |
| spruce s1 | borderline: leader, upper tiers, hazy lower crown | 2/2 ✓ | 24.3 (18.8) ✓ | 24.6 (47.6) ✓ |
| spruce s7 | borderline, as seed 1 | 2/2 ✓ | 23.8 (17.9) ✓ | 23.9 (43.0) ✓ |

## Supplementary bands, major axes, and per-major secondaries

The "per major" column lists the secondaries of each `root04` major. The median in the supplementary column is over every major (`root04` ∪ `local04`), as `score.rs` counts it.

| tree | junction o1 | secondaries per major (median) | length ÷ remaining | root04 (today) | root04 born at (÷ H) | per major (root04) |
|---|---|---|---|---|---|---|
| beech s1 | 0.34 ✗ | 34 ✗ (32 majors with local04) | 0.43 ✓ | 6 (5) | all 0.05 | 40, 60, 60, 68, 63, 59 |
| beech s7 | 0.34 ✗ | 29 ✗ (36 majors) | 0.40 ✓ | 6 (5) | all 0.05 | 64, 60, 60, 63, 61, 41 |
| oak s1 | 0.21 ✗ | 21 ✗ (3 majors) | 0.66 ✓ | 3 (0) | all 0.05 | 21, 20, 27 |
| oak s7 | 0.25 ✗ | 18 ✗ (3 majors) | 0.99 ✓ | 2 (0) | 0.06, 0.06 | 18, 24 |
| spruce s1 | 0.17 ✓ | – | – | 0 (0) | – | – |
| spruce s7 | 0.18 ✓ | – | – | 0 (0) | – | – |

For the spruce, "lowest lateral ≤ 0.10 H" also passes, at 0.04 at both seeds. Today's beech majors carry 4 to 9 secondaries each.

## Reported, not voted

| tree | bole p5 (old) | division | trace | oak clause (division − bole ≤ 0.10) | w/h (authored) | H ÷ authored |
|---|---|---|---|---|---|---|
| beech s1 | 0.21 (0.05) | 0.06 | born 33.7°, rise 10.7° (Troll ✓) | – | 0.87 (0.72) | 1.06 |
| beech s7 | 0.18 (0.05) | 0.05 | born 34.8°, rise 13.3° (Troll ✓) | – | 0.78 (0.72) | 1.18 |
| oak s1 | 0.17 (0.05) | 0.06 | born 38.7° (Rauh ✗) | −0.11 ✓ (division below the bole) | 0.80 (1.10) | 1.36 |
| oak s7 | 0.19 (0.06) | 0.06 | born 40.5° (Rauh ✗) | −0.13 ✓ | 0.89 (1.10) | 1.24 |
| spruce s1 | 0.06 (0.85) | none below 0.92 ✓ | leader 1.0 ✓, elevation 5.5° ✓ | – | 0.78 (0.62) | 1.01 |
| spruce s7 | 0.06 (0.84) | none below 0.96 ✓ | leader 1.0 ✓, elevation 5.6° ✓ | – | 0.80 (0.62) | 0.99 |

- **Fine wood at the 0.04 and 0.06 boundaries:** beech 49.8 / 54.3 km (s1) and 50.7 / 54.3 km (s7), both above today's. Oak and spruce stay within 1% of their 0.05 figure.
- **Nodes kept against grown:**
  - beech 242k of 386k (s1) and 242k of 392k (s7);
  - oak 149k of 257k and 150k of 258k;
  - spruce 257k and 252k, none pruned.

  Lifespan prunes 37 to 42% of the wood the beech and oak grow.

## Visual gate (3 images: `raw/stills/sheet-r1a2-<species>.png`)

Each sheet holds the S1 photograph, round 1's bare still at seed 1, and this round's bare and whole stills at seeds 1 and 7.

- **Beech, fail at both seeds.** Long arching limbs now fan out from a short stub of trunk, with a thin leader standing above them, under a broad, low, dense dome. The limbs are differentiated, which is new, but they start at the ground. The photograph's trunk divides at about a third of the height.
- **Oak, fail at both seeds.** A broom of many thin stems from the base, with sparse foliage and no trunk or limb structure. A few thin shoots stand far above the crown (H 1.24 to 1.36 of authored).
- **Spruce, borderline at both seeds.** It has a persistent leader and a cone. Thin horizontal branches in whorled tiers now read in the upper half, while the lower half is still a haze of fine wood. The lowest branches lie wide at ground level, no longer under it.

## Passes kept and deleted (cold ms, beech s1 / oak s1 / spruce s1)

| pass | what it does | ms |
|---|---|--:|
| yearly sweep | one pass a year over the living buds: grow, set laterals with birth phi and lifespan | 55.1 / 28.2 / 24.2 |
| pipe | one basipetal pass at the end over all wood grown (tips, radii) | 1.1 / 0.5 / 0.6 |
| straighten | one pass at the end, each segment's own yearly turn × its years (beech only) | 7.3 / 0 / 0 |
| prune | one pass at the end: drops the pruned axes and copies the kept tree out | 9.9 / 7.4 / 11.3 |

- **The prune pass is timed for the first time.** Round 1's output copy was not timed. Its cost is the copy itself, as the spruce's 11 ms with nothing pruned shows.
- **Warm totals:** beech 47 to 50 ms for 386k to 392k grown nodes, against today's 53 ms skeleton.
- **State added:** per bud, its death year; per node, its axis's birth phi and death year.
- **Deleted, still absent:** everything round 1 deleted. No pass reads light, balance or the envelope.

## GreenLab-style factorisation with continuous phi

| tree | laterals | distinct (phi, birth year) | laterals per key |
|---|--:|--:|--:|
| beech s1 / s7 | 34,075 / 34,738 | 167 / 174 | 204× / 200× |
| oak s1 / s7 | 13,422 / 13,482 | 104 / 112 | 129× / 120× |
| spruce s1 / s7 | 33,561 / 33,052 | 117 / 117 | 287× / 282× |

- Keys repeat more than in round 1 (34× and 62× on the beech and oak) because more laterals sit at phi 1. The clamp at 1 collapses their keys into one per year.
- The conclusion is unchanged. Expected counts factorise in R1a. Topology per instance (keyed rounding), geometry and pruning (which depends on the carrying axis) do not. The sweep that emits every node is the cost.

## What the round established

- **The spread of birth ages makes limbs.**
  - The beech gets 6 major axes at both seeds. Its junction ratio is 0.34, just under the band and below round 1's 0.39.
  - The Troll trace holds (rise 10.7° and 13.3°).
  - Fine wood and time still pass.
- **The limbs are the seedling's first laterals.** All of the beech's and oak's `root04` majors are born at 0.05 H: the first unit's distal laterals.
  - They are born young with no tree above them and have the most years.
  - Lifespan by phi cannot tell them from a later limb born at 0.3 H with the same phi.
  - That is why projected division sits at 0.05 to 0.06, and why the beech is a fan from the base and the oak a broom.
- **Lifespan buys a bole by pruning, at a cost in fine wood, and steeply.**
  - Shorter lifespans raise the bole and the junction ratio. At life1 40 and branching 2.2 the beech reaches 0.36 to 0.37, inside the band, but fine wood falls to 78%. Raising branching to recover it brings the junction ratio back under 0.35.
  - Many axes sit at phi exactly 1, where the clamp holds them, so they share one lifespan. On an earlier beech point (lifespan blended linearly in phi, before `lifeShape` took its final form), life1 30 against 45 pruned 300k against 38k nodes.
  - The response is continuous in principle and steep in practice where phi saturates.
- **Majors carry far too many secondaries.** A major grows a distal lateral every year all along its length, so it collects 18 to 68 substantial secondaries. The band is 4 to 10, and today's beech carries 4 to 9.
- **The oak's exponent (2.0) makes a major hard.** A major needs 16% of all tips, against 7% on the beech (exponent 2.9). Its 2 to 3 majors are base stems of a broom. Rauh's born-at-45° trace fails (38.7°, 40.5°); it is reported only.
- **The two-ranked defect** affected every probe round that used `distich`: rounds 1 to 5 and R1a round 1 share the same `bud_dir`. It is fixed here for the probe only.

## Decisions this round needs from the host

1. **Where limbs are born.** The spread of birth ages gives limbs, but always the earliest ones, at the base. Without light, nothing in R1a ranks a lateral born at 0.3 H in year 15 above one born at 0.05 H in year 1. Is a term by the tree's age at a lateral's birth wanted in R1a? Examples:
   - a birth jump that falls as the tree establishes (Barthélémy & Caraglio's base effect);
   - a lifespan that falls for laterals born while the tree is young.
   Or are the division and limb height left to R1b's shading and shedding, accepting a fan-from-the-base visual in R1a?
2. **Secondaries per major.** Should a major's laterals jump further with the major's own age or length? This is a gradient along the axis, which the current jump does not have: it depends only on place within one year's unit and that shoot's vigour. Or is the band left to R1b's shading of the limb interior?
3. **Phi saturating at 1.** The clamp gives many axes an identical phi, lifespan and key, which makes lifespan responses steep. Should phi approach 1 smoothly instead of clamping, or is the clamp acceptable as the short-shoot end of the scale?
4. **The oak.** With exponent 2.0, `root04` asks a limb to carry 16% of all tips, and today's oak has 0. Is `root04` the right major-axis diagnostic for the oak, or should `local04` be the oak's count?
5. **The two-ranked fix outside the probe.** The defect sat in the probe's lateral-direction code since round 1. Should R2's production design state the corrected two-ranked rule explicitly?

Round 2 of three for R1a is spent. Per the host's steering, round 3 is not started.
