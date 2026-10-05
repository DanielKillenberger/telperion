# fn-197 step 3b: relative allocation and the full lay grown with the tree, 2026-10-05

Host decisions 7 to 10 (DESIGN-OPTIONS.md, section 9) are built.

## What changed

- **Relative allocation (`src/allocation.rs`):**
  - A growth unit's size is light^ψ over the presence-weighted mean of light^ψ among the living apexes on the same bearer that cycle. The bearer is a lateral's parent; an axis carried on takes its first link's bearer.
  - It is computed before any apex grows, so every sibling reads the same mean.
  - The weighted total of sizes equals the total of the weights.
  - ψ = 0 leaves every size exactly 1.
- **The oracle tests:**
  - Two siblings, one shaded: at ψ = 1 the lit share is exactly Pałubicki's Borchert–Honda split at λ = 0.5, Q_lit / (Q_lit + Q_shaded). The share rises with ψ, above 0.98 at ψ 4, and the sum is unchanged at every ψ.
  - Neutral and conservation: sizes are exactly 1 at ψ = 0, a bud of no weight moves its siblings by nothing, and unequal weights keep their total.
- **The full lay grown with the tree (`src/grow/relay.rs`):**
  - Every `RELAY_EVERY` = 10 cycles, the living part of the tree (every axis with a living apex and all that bears it) is sized, thickened and laid with sag, at the tree's age then.
  - Its positions replace the rough layout's. Each axis's rough walk carries on from where the full lay ended it: `place` now returns each axis's `Layer`.
  - The cycle's leaves are re-gathered where the full lay put them.
- **The fade cap:** shedding's fade clamps at most 1 (`shed.rs`). An apex grown past whole would otherwise panic `clamp` (it did, in the walks).
- **Walks:** refine depth 6 (`REFINE`, with the reason in its comment).

## Neutral

- `tests/light.rs` passes: every passed species is unchanged to the bit under three shading lights with φ = ψ = 0. That includes the oak, which now has leaf area and so runs the rough layout and the full lays.
- At the default `Light::NEUTRAL` nothing runs.

## Walks (each run alone, `scratchpad/walk-*.txt`)

| Walk | Result | Time |
|---|---|--:|
| Light: φ, ψ (relative), leaf area, sky, extinction; full lay at cycle 10 | Green. Steepest slopes: sky 16.8, ψ twigs 8.6, leaf area 7.3; no jump at depth 6 | 88 s |
| Every setting (no light) | Green at depth 6 | 91 s |
| Release law | Green | 2 s |
| Every setting in leaf, with the full lay (decision 8) | **Did not finish in 3,000 s.** Committed `#[ignore]`; decision 8's check is unrun | — |

## The full lay's cost and gap (oak at 80 years, φ = ψ = 0, extinction 0.5, sky 0.5)

The machine was loaded (load average 10 to 19 from other sessions). The times are the second of two runs.

| Seed | K | Total | Rough layout | Full lays | Light | Final lay |
|---|--:|--:|--:|--:|--:|--:|
| 1 | none (light off) | 4.0 s | – | – | – | 0.64 |
| 1 | 10 | 6.9 s | 1.23 | 1.39 | 0.44 | 0.68 |
| 1 | 5 | 7.8 s | 1.22 | 2.46 | 0.45 | 0.63 |
| 7 | none (light off) | 5.3 s | – | – | – | 0.97 |
| 7 | 10 | 10.0 s | 2.03 | 1.95 | 0.60 | 0.86 |
| 7 | 5 | 10.4 s | 1.77 | 3.22 | 0.55 | 0.85 |

**Gap** (rough-and-relaid against final, sky 0.5). Age 79 is the worst point for K = 10: nine cycles of rough growth since the full lay at 70.

| Seed, age | Layout | Extent: final / light's (m) | Median / 90th distance (m) | Bud light correlation | Buds off > 0.1 |
|---|---|---|---|--:|--:|
| 1, 80 | Step 2: rough, no sag | 23.5 × 18.6 / 16.9 × 16.5 | 1.53 / 3.81 | 0.848 | 19% |
| 1, 79 | K = 10 | 22.9 × 18.3 / 24.5 × 19.7 | 1.18 / 2.16 | 0.922 | 14% |
| 1, 79 | K = 5 | 22.9 × 18.3 / 25.2 × 20.1 | 1.43 / 2.83 | 0.911 | 15% |
| 7, 80 | Step 2: rough, no sag | 19.9 × 21.5 / 17.6 × 16.1 | 1.80 / 3.74 | 0.796 | 20% |
| 7, 79 | K = 10 | 19.6 × 21.2 / 20.1 × 24.5 | 1.08 / 2.61 | 0.909 | 17% |
| 7, 79 | K = 5 | 19.6 × 21.2 / 20.8 × 24.9 | 1.43 / 3.11 | 0.907 | 18% |

**Reading the gap:**

- **The full lay closes most of the sag gap.** The crown light reads is now as wide as the drawn one, its light correlation rose from 0.80–0.85 to 0.91–0.92, and the 90th-percentile distance halved.
- **What remains is the full lay's own bias, not K:** the crown light reads is 1 to 3 m wider than the drawn one.
  - The full lay leaves dead wood out, so its girth is missing and the living limbs it lays are thinner and sag more.
  - The final lay also applies shedding's fades.
  - K = 5 costs 1 to 1.3 s more and is no closer. K = 10 is kept.
- **Cost of light at K = 10, φ = ψ = 0:** +2.9 s on seed 1 (+72%) and +4.7 s on seed 7 (+89%). This is before shade saves any growth; φ 1 at step 3 cut seed 1's grown tree by about half.

## Renders (oak, 80 years, under the GPU lock; I viewed every still)

Sheets are in `raw/step3b/`:

- **`explore.png`:** seed 1 at sky 0.5, φ 1 with ψ 0, 1, 2 and 4, and φ 0 with ψ 2, bare and in leaf, beside S1, S2 and round 4.
- **`seeds-bare.png`, `seeds-whole.png`:** seeds 1, 7, 2, 3 and 4 at sky 0.5, φ 1, ψ 1 (relative, full lay on), under round 4's same seeds and the references.

| Seed 1 | Height | Width | Leaves | Wood |
|---|--:|---|--:|--:|
| Round 4, no light | 21.9 m | 23.5 × 18.6 | 2.16M | 17.3 km |
| φ 1, ψ 0 | 21.0 m | 25.0 × 20.3 | 1.31M | 10.9 km |
| φ 1, ψ 1 | 26.4 m | 33.3 × 24.7 | 1.86M | 15.7 km |
| φ 1, ψ 2 | 38.8 m | 50.8 × 39.0 | 2.67M | 23.1 km |
| φ 1, ψ 4 | 59.5 m | 99.9 × 84.8 | 4.27M | 37.7 km |

At φ 1, ψ 1 the five seeds stand 23.9 to 31.6 m tall and 32 to 39 m wide, against round 4's 19 to 22 m tall and 20 to 25 m wide.

### Reading

- **ψ 1 broadens and fills the crown.** In leaf the crown is fuller and more continuous than round 4's or φ 1 alone.
  - Seed 3 is the nearest yet to a dome: a broad, rounded, spreading top, close to S2 garryana's habit.
  - Seed 7 spreads broad and low.
- **But the outline does not close evenly.**
  - Seed 1 stays a column.
  - Seed 2 leans hard to one side, with a lopsided mass.
  - Seed 4 sprawls on a few thick, wandering limbs.
  - The bare trees read heavier and coarser than round 4: thicker limbs, a less even web.
- **ψ 2 and 4 are grotesque:** sprawling thickets of heavy, tangled limbs, tens of metres across.
- **Why the trees grow:** sizes above 1 compound down a lineage.
  - A bud's size multiplies its phytomers' scale, and a lateral inherits its node's scale (`geometry::scale`).
  - So a lit lineage is enlarged at every generation. The bearer's total is conserved only among siblings in one cycle, not across generations.
  - Wood thickens with it, because each phytomer's own pipe is scaled by its scale (`girth.rs`).
  - Pałubicki's Borchert–Honda conserves the whole tree's resource from the base down, which this per-bearer form does not.

## For the host before step 4

1. **Should a unit's relative size reach its laterals?**
   - Either a size that scales only the unit's own internodes and girth, not what it bears;
   - or a conservation that runs from the base down, as Borchert–Honda does.
   - Today's form makes ψ a growth dial. At ψ 1 the oak is 25 to 45% taller and 35 to 60% wider than round 4.
2. **The in-leaf walk of every setting (decision 8)** did not finish in 50 minutes. It needs a scope: a sample of settings, or a slower suite outside the gate.
3. **K = 10 is kept.** The remaining 1 m median gap is the full lay leaving dead wood's girth out, not its interval.
4. **The oak at φ 1, ψ 1 is not judged a pass.** The outline moves toward a dome on some seeds, not on all, and the size and coarseness are new regressions against round 4.
