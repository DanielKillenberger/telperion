# fn-190 R1, probe round 3: one carbon balance, 2026-10-03

## Setup

- **Code:** `crates/telperion-render/examples/growth_law/`, changed per the spec's "Host decisions after R1 round 2".
- **Net resource:**
  - A shoot's **light** is gathered by the buds of shoots that grew; a dormant bud gathers nothing.
  - Its **upkeep** is `upkeep × (length ÷ unit) × tips^(2/e)`, its pipe volume in tip-pipe units.
  - Both are summed basipetally. The base resource is `alpha × max(0, net)`.
- **Allocation follows demand:** a bud draws in proportion to its light times its internode share by age. A resource v buys v units of wood, cut into internodes as long as the age allows.
- **Dormant buds** die each year with chance `budDeath × dormant years`.
- **Shedding:** every branch remembers its relative balance, (light − upkeep) ÷ (light + upkeep), and is shed with chance `shed × smoothstep` as that falls from −tolerance to −(tolerance + shedWidth). No branch is spared.
- **Votes:** the host's item 5, with the three probe definitions confirmed. A tree under 1,000 nodes is a driver error (exit 2).

## Points (seed 1 tuned, seed 7 held out)

| species | settings off neutral |
|---|---|
| beech | cycles 50, upkeep 0.002, shed 0.3, memory 0.3, budDeath 0.05, distich 0.8 |
| oak | cycles 45, upkeep 0.0005, shed 0.3, memory 0.3, budDeath 0.05, acrotony 1.5, rhythm 0.3, angle0 40, lambda 0.5 |
| spruce | cycles 36, upkeep 0.001, shed 0.3, memory 0.3, budDeath 0.05, lambda 0.58, alpha 1.2, lean1 1.4, phiStep 0.6, rhythm 0.85, angle0 70, angle1 85 |

## Results

| tree | bole | division | trace | fine km (today) | ms (bar) | supplementary | numeric |
|---|---|---|---|---|---|---|---|
| beech s1 | 0.02 ✗ | 0.06 ✗ | born 55°, rise 0° ✗ | 1.7 (46.7) ✗ | 38.3 (79) ✓ | 3/3 ✓ | fail |
| beech s7 | 0.02 ✗ | 0.13 ✗ | born 55°, rise 0° ✗ | 2.2 (47.1) ✗ | 40.2 (79) ✓ | 2/3 ✓ (rest 0.82 ✗) | fail |
| oak s1 | 0.03 ✗ | 0.03 ✗ | born 55°, rise 0° ✓ | 6.5 (31.2) ✗ | 119 (50) ✗ | 3/3 ✓ | fail |
| oak s7 | 0.03 ✗ | 0.03 ✗ | born 55°, rise 0° ✓ | 5.8 (35.6) ✗ | 108 (55) ✗ | 3/3 ✓ | fail |

| tree | leader | elevation | no division < 0.85 | fine km (today) | ms (bar) | supplementary | numeric |
|---|---|---|---|---|---|---|---|
| spruce s1 | 1.0 ✓ | 27.4° ✓ | 0.87 ✓ | 0.22 (18.8) ✗ | 7.8 (48) ✓ | 2/2 ✓ | fail (fine wood) |
| spruce s7 | 1.0 ✓ | 27.3° ✓ | 0.85 ✓ | 0.37 (17.9) ✗ | 10.8 (43) ✓ | 1/2 ✓ | fail (fine wood) |

- **Boundary sensitivity:** fine wood at 0.04 and 0.06 of the root is within 20% of the 0.05 figure on every tree, with the same verdicts.
- **The spruce fails only on fine wood:** 0.2–0.4 km against 18 km.
- **Visual gate (3 images: `raw/stills/sheet-r3-<species>.png`, seed 1):**
  - **Beech, fail.** Several thick limbs rise from the ground: a multi-stemmed tree, no bole. The crown is sparse, with tufts at the limb ends.
  - **Oak, fail.** Dozens of stems fan from the base into a flat-topped vase, with no trunk. The leader is lost (stem top 0.7 H at seed 1, 0.1 H at seed 7).
  - **Spruce, fail on density.** The structure is right: a persistent leader with tiers of horizontal branches. But it is a skeleton, with almost no fine wood or foliage.

## Size at seed 1 against seed 7

| species | nodes s1 / s7 | ratio | height s1 / s7 (m) |
|---|---|--:|---|
| beech | 11,093 / 13,937 | 1.26 | 31.3 / 31.5 |
| oak | 40,118 / 35,905 | 1.12 | 13.4 / 13.5 |
| spruce | 4,111 / 6,106 | 1.49 | 11.0 / 11.9 |

- **The balance bounds size at these points.** Seed 7 is within 1.5× of seed 1 on every species, against 12× on round 2's spruce.
- **One spruce point still ran away.** At alpha 1.5 and upkeep 0.001, seed 1 grew 572k nodes, its net rising past 500k, against 20k at seed 7. Light gathered still outgrows upkeep in a dense crown, so a large enough alpha escapes the bound.

## One-parameter walks on the beech (10 points, 9 steps; `raw/r3/walks.json`)

A jump is a step larger than 3 × the walk's median step.

| walk | seed | nodes (range, max ÷ median step) | bole (range, max ÷ median step) | fine wood |
|---|---|---|---|---|
| upkeep 0.0005 → 0.004 | 1 | 22.7k → 8.0k, 4.2 **jump** | 0.014–0.018, flat but 0-median **jump** | 3.7 **jump** |
| upkeep 0.0005 → 0.004 | 7 | 23.5k → 7.6k, 5.3 **jump** | 0.014–0.078, 61 **jump** | 7.1 **jump** |
| tolerance −0.6 → 0.2 | 1 | 9.9k → 15.7k, 2.2 ok | 0.015–0.049, **jump** | 2.1 ok |
| tolerance −0.6 → 0.2 | 7 | 10.5k → 16.9k, 2.1 ok | 0.015–0.068, **jump** | 2.3 ok |

- **Node count and fine wood jump on the upkeep walk** (about 4–7 × the median step), but not on the tolerance walk.
- **The bole does not respond to either setting.** It stays at 0.014–0.018 H.
- **Its jumps are the measure's.** "Lowest substantial lateral" flips to a different branch at 0.05–0.08 H at single steps. A bole measure that is continuous in the tree is needed before the bole can be walked.
- **Shade tolerance barely moves the tree.** It changes node count by 1.6× across its whole range.

## Where time goes (cold run, ms)

| tree | light lookups | allocate | shed | extend: bud work | extend: shadow casts (voxel updates) |
|---|--:|--:|--:|--:|--:|
| beech s1 | 4.3 | 6.6 | 1.3 | 17.1 | 9.4 (25.1M) |
| oak s1 | 12.8 | 19.4 | 4.0 | 52.4 | 28.8 (81.3M) |
| spruce s1 | 0.9 | 1.5 | 0.3 | 3.2 | 1.7 (4.5M) |

- **Light lookups fell** against round 2 (beech 11.6 → 4.3 ms), because dormant buds now die.
- **Bud work still dominates.** Shadow casts are a quarter to a third of the time.
- **The oak is over its bar** (108–119 ms against 50–55) because it sheds 30k nodes it grew: growth spent and then thrown away.

## Decisions this round needs from the host

1. **The bole does not emerge from shedding.**
   - On an open-grown tree the lowest limbs grown in the seedling years stay lit. The shadow reaches 6 layers, 3 m on the beech, so their own balance stays positive and they become the tree's main stems (the beech's ground limbs, the oak's vase).
   - Deeper shadow (depth 10–20) darkens the whole tree into collapse: 750–3,900 nodes.
   - What should shade the lower crown? The shadow's reach and strength over the full height, a light from above that the crown intercepts, or crown competition with neighbours, which the authored bole would stand in for?
2. **The seedling is a cliff.** The main line's balance is the seedling's, so a slightly worse early balance sheds the whole tree.
   - Oak at upkeep 0.001, or lambda 0.52, grows 1–2 nodes.
   - Beech with Troll lean grows 810 nodes.
   - Should the seedling's early balance be buffered (a seed reserve, or no shedding before an age), or is that a threshold the spec forbids?
3. **Upkeep's scale is not yet a smooth dial.** Node count jumps 4–5× its median step across the upkeep walk, and a large alpha still escapes the bound (the spruce at 572k). Upkeep grows with tips^(2/e), slower than light grows with buds in a crown that does not shade itself enough.
4. **Fine wood fell further** (0.2–6.5 km against 18–47). With upkeep on, the balance affords few shoots, so "fine wood is what the balance affords" puts R1's fine-wood bar out of reach unless light per shoot or the shoot's cost changes.
5. **The bole measure needs a continuous form** before the walks can test the bole (see the walks).

Round 3 of six is spent.
