# fn-190 R1, probe round 4: light from above, a reserve, a smooth bole, 2026-10-03

## Setup

- **Code:** `crates/telperion-render/examples/growth_law/`, changed per the spec's "Host decisions after R1 round 3".
- **Light from above (`light.rs`, replacing `shadow.rs`):**
  - Every living tip is one unit of leaf area in its voxel (4 units, 1 m on the beech).
  - Once per cycle, leaf area is carried down each column, spread over a 3 × 3 box per layer (a 45° sky cone).
  - A cell's light is exp(−density × leaf area above it). Buds read light and its gradient from the field.
- **Leaves gather the light:** a tip's light is its leaves' income. Round 3 counted buds that grew that year, and a branch that paused one year read as all upkeep, so the seedling was shed.
- **Reserve:** the tree starts with `reserve` (2). Each year the net plus the stock is what it has. A share `store` (0.2) is stored again and the rest is spent. The reserve's cover (stock ÷ the tree's upkeep, at most 1) pays that share of every branch's upkeep before its balance counts. No rule names an age.
- **Short shoots:** they persist on their terminal buds and extend by internodes whose length follows age, with upkeep by their volume. That is the round-3 mechanism; no separate rule was added.
- **Bole:** crown base = the 5th-percentile height of fine-wood length ÷ H. The bole band applies to it, and the old lowest-substantial reading is reported beside it.
- **Votes:** item 5 as before.

## Points (seed 1 tuned, seed 7 held out)

Common to all three: upkeep 0.004, density 0.01, shed 0.3, memory 0.3, budDeath 0.05.

| species | settings off that and neutral |
|---|---|
| beech | cycles 50, lambda 0.55, distich 0.8 |
| oak | cycles 45, lambda 0.54, acrotony 1.5, rhythm 0.3, angle0 40 |
| spruce | cycles 36, lambda 0.6, lean1 1.4, phiStep 0.6, rhythm 0.85, angle0 70, angle1 85 |

- **Density sets the crown's thickness, and the useful range is narrow.** Density 0.1 to 0.4 darkens the 1 m columns at a few dozen tips, and the trees stopped at 500–2,200 nodes. Density 0.005 to 0.02 grows them, and 0.01 was kept.
- **Lambda now sets height steeply.** On the beech, 0.52 / 0.55 / 0.58 grew 12 / 31 / 64 m.

## Results

| tree | crown base (old) | division | trace | fine km (today) | ms (bar) | supplementary | numeric |
|---|---|---|---|---|---|---|---|
| beech s1 | 0.20 ✓ (0.04) | 0.05 ✗ | born 55°, rise 0° ✗ | 13.4 (46.7) ✗ | 231 (79) ✗ | 1/3 ✗ (o1 0.34, sec 30, rest 0.51 ✓) | fail |
| beech s7 | 0.21 ✓ (0.03) | 0.39 ✓ | born 56°, rise 0° ✗ | 11.1 (47.1) ✗ | 201 (79) ✗ | 1/3 ✗ | fail |
| oak s1 | 0.18 ✓ (0.06) | 0.66 ✗ | born 55°, rise 0° ✓ | 2.9 (31.2) ✗ | 86 (50) ✗ | 2/3 ✓ (o1 0.32 ✗) | fail |
| oak s7 | 0.24 ✓ (0.06) | 0.58 ✗ | born 55°, rise 0° ✓ | 2.9 (35.6) ✗ | 100 (55) ✗ | 2/3 ✓ (o1 0.29 ✗) | fail |

| tree | leader | elevation | no division < 0.85 | fine km (today) | ms (bar) | supplementary | numeric |
|---|---|---|---|---|---|---|---|
| spruce s1 | 1.0 ✓ | 28.2° ✓ | 0.0 ✗ | 6.8 (18.8) ✗ | 148 (48) ✗ | 2/2 ✓ | fail |
| spruce s7 | 1.0 ✓ | 28.6° ✓ | 0.0 ✗ | 6.5 (17.9) ✗ | 157 (43) ✗ | 2/2 ✓ | fail |

- **The bole band now passes on the beech and the oak at both seeds.** The crown base is 0.18–0.24 H. The old reading stays at 0.03–0.06, because a few seedling-era limbs with their own crowns survive below the fine wood.
- **The spruce's "division at 0.0" needs a check before it is trusted.** The visible tree has one stem, and `leaders2` probably finds two comparable crossings in the flared base. It is reported as failed, unverified.
- **Fine wood at 0.04 and 0.06** is within 11% of the 0.05 figure everywhere, with the same verdicts.
- **Visual gate (3 images: `raw/stills/sheet-r4-<species>.png`, seed 1):**
  - **Beech, fail.** A full, leafy crown on a clear trunk, but a narrow spindle (a Lombardy poplar's habit), not Entzia's broad dome. No differentiated limbs reach the outer crown.
  - **Oak, fail.** A conical, excurrent tree with a central leader, 10 m tall against the envelope's 24, with a heavy root flare. It is Massart-like, not Rauh.
  - **Spruce, nearest yet.** A full cone with tiers of horizontal branches and foliage filling the outline, close to the photograph's habit, and denser than any earlier round.
  - **All three are columnar:** light from above pulls every shoot up, and nothing in the current points spreads the crown.

## Size at seed 1 against seed 7

| species | nodes s1 / s7 | ratio | height s1 / s7 (m) | reserve at the end s1 / s7 |
|---|---|--:|---|---|
| beech | 144,553 / 120,513 | 1.20 | 30.9 / 30.9 | 900 / 843 |
| oak | 49,241 / 50,107 | 1.02 | 10.4 / 10.1 | 146 / 105 |
| spruce | 126,656 / 124,119 | 1.02 | 15.2 / 15.5 | 569 / 452 |

Heights agree within 0.4 m and node counts within 1.2×. No seedling died at the chosen points, or in any of the 40 walk runs.

## One-parameter walks on the beech (10 points, 9 steps; `raw/r4/walks.json`)

A jump is a step larger than 3 × the walk's median step.

| walk | seed | nodes | crown base | fine wood | height |
|---|---|---|---|---|---|
| upkeep 0.001 → 0.008 | 1 | 234k → 115k, 2.6 ok | 0.15 → 0.21, 9.0 **jump** | 25.0 → 9.8 km, 2.9 ok | 34.5 → 27.4 m, ok |
| upkeep 0.001 → 0.008 | 7 | 182k → 80k, 5.9 **jump** | 0.15 → 0.22, 6.0 **jump** | 18.4 → 6.1 km, 5.3 **jump** | 34.7 → 27.1 m, ok |
| tolerance −0.6 → 0.2 | 1 | 118k → 161k, ok | 0.225 → 0.173, 3.2 **jump** | 11.3 → 14.9 km, ok | 31.2 → 30.9 m, ok |
| tolerance −0.6 → 0.2 | 7 | 102k → 139k, ok | 0.216 → 0.173, 16 **jump** | 9.7 → 12.8 km, ok | 31.3 → 30.8 m, ok |

- **Both dials now move the tree by degree.** Every series is monotonic but for single reversals (upkeep seed 1: nodes 122.8k → 127.6k, crown base 0.203 → 0.199 → 0.202), and height moves smoothly in all four walks.
- **Shade tolerance acts as expected.** Tolerating less deficit sheds more: fewer nodes and a higher crown base.
- **The remaining jumps are single steps.**
  - The crown base steps once between upkeep 0.0026 and 0.0033: 0.15 → 0.19 at seed 1, 0.16 → 0.22 at seed 7.
  - The tolerance walks' last step takes the crown base 0.19–0.20 → 0.173.
  - At seed 7, nodes and fine wood drop at the same upkeep step (170k → 130k).
  - Each is flagged against a small median step: the crown base's median step is about 0.004.

## Where time goes (cold run, ms)

| tree | total | light lookups (7 per bud) | allocate | shed | extend | light field pass (cells) |
|---|--:|--:|--:|--:|--:|--:|
| beech s1 | 235 | 59.7 | 70.7 | 10.5 | 60.0 | 24.8 (0.98M) |
| oak s1 | 90 | 23.9 | 27.2 | 4.2 | 21.8 | 9.4 (0.22M) |
| spruce s1 | 151 | 44.4 | 36.9 | 5.2 | 42.4 | 13.9 (0.68M) |

- **The light field is cheap,** a tenth of the time: 24.8 ms for 145k nodes, against round 3's 9.4 ms of shadow casting for 11k, about 5× cheaper per node.
- **Bud work is the cost.** Light lookups and the gradient (7 grid reads per bud), allocation over a 120–145k-node tree each cycle, and extension are each a quarter to a third of the time.
- **Every tree is 1.8 to 3.5× over its time bar**, at 9% (oak) to 36% (spruce) of today's fine wood.

## Decisions this round needs from the host

1. **The crowns are columnar.** Light from above favours upward growth, so the beech is a spindle and the oak a cone. What spreads a broadleaf crown in the law? Options: the Troll lean and straightening (still dormant at these points), the light cone's width (45° now), a phototropism weighted by the light gradient's horizontal part, or the authored envelope's pull (dormant at 0).
2. **Speed is the binding constraint.** At today's fine wood the probe would need about 3× (beech, spruce) to 11× (oak) its current fine wood, against bars it already misses by 1.8–3.5×. The per-cycle passes over the whole tree (allocation, upkeep, balance) and per-bud lookups scale with nodes × cycles. Should those passes become incremental (only the ancestors of buds that changed), should cycles be fewer and longer, or should the bar be revisited with the owner?
3. **Lambda now sets height steeply** (0.52–0.58 gives 12–64 m on the beech). Should height be calibrated by a different dial, as the host's round 2 note intended (age and upkeep calibrated to the authored height)?
4. **The spruce's division reading of 0.0** needs a look at the measure on a flared base before it decides a vote.

Round 4 of six is spent.
