# fn-190 R1, probe round 1: the law from the root, 2026-10-03

## Setup

- **Code:** `crates/telperion-render/examples/growth_law/` (scratch, never merged). One law grows the whole tree from the root: space markers in the preset's envelope, light weighted onto a bud's resource, Borchert-Honda allocation, a continuous physiological age φ per bud, straightening of thick wood, and shedding on remembered resource per tip. Every setting is continuous with a stated neutral value (`params.rs`); counts are rounded in expectation with keyed randomness.
- **Ontogeny:** each marker is a point of the mature crown scaled to the crown of the age at which it joins (`young` spreads them over the ages), so the crown grows from a seedling's.
- **Scorecard:** fn-188's `score.rs`, with the spec's rules: structural wood is radius ≥ 0.05 of the root's, majors are `root04`, generations are taken once per terminal. The bands are `bands.rs`, as fixed in the spec before this round.
- **Runs:** `growth_law today|grow PRESET SEED …`, release, Ryzen 9 5950X, 16 threads. Times are warm medians of 3 after a cold run. Rows are in `raw/today/` and `raw/r1/`.

## Today's trees (the baseline to beat)

| tree | bands met | skeleton ms | nodes | fine wood km | 1.5 × skeleton ms |
|---|--:|--:|--:|--:|--:|
| beech s1 / s7 | 4 / 5 of 10 | 52.7 / 52.8 | 213k / 215k | 46.7 / 47.1 | 79 |
| oak s1 / s7 | 4 / 4 of 10 | 33.0 / 36.7 | 134k / 144k | 31.2 / 35.6 | 50–55 |
| spruce s1 / s7 | 5 / 5 of 9 | 32.7 / 30.4 | 96k / 91k | 18.8 / 17.9 | 46–49 |

Today's beech fails the bole (0.07), division (0.08–0.15), length ÷ remaining (1.4–1.5), shell ÷ interior (0.19) and the junction ratio (0.15). The oak fails majors (0), secondaries (11–18, a comb), the density ratios and the junction ratio. The spruce fails division (0.68), the density ratios and the junction ratio (0.28).

## Round 1 species points

| species | settings off neutral |
|---|---|
| beech (Troll) | light 0.5, lean0 0.15, lean1 0.9, straighten 0.02, acrotony 0.8, branching 1.3, fate 1.5, short 0.3, phiStep 0.35, drift 0.2, reiteration 0.15, angle0 35, angle1 65, distich 0.8, shed 0.12, young 2 |
| oak (Rauh) | light 0.5, lean1 0.35, acrotony 1.5, rhythm 0.35, branching 1.2, fate 1.5, short 0.35, phiStep 0.25, drift 0.2, reiteration 0.2, angle0 40, angle1 55, shed 0.12, young 2 |
| spruce (Massart) | lambda 0.6, light 0.4, lean1 1.4, rhythm 0.85, branching 1.0, fate 1.0, short 0.4, phiStep 0.6, drift 0.15, angle0 70, angle1 85, distich 0.6, shed 0.1, young 2 |

| tree | bands met | probe ms | nodes | fine wood km |
|---|--:|--:|--:|--:|
| beech s1 / s7 | 2 / 2 of 10 | 43.7 / 43.4 | 7.2k / 7.1k | 0 / 0 |
| oak s1 / s7 | 5 / 4 of 10 | 94.1 / 83.8 | 35.8k / 33.3k | 3.4 / 3.1 |
| spruce s1 / s7 | 7 / 7 of 9 | 27.5 / 29.7 | 12.4k / 14.0k | 0.5 / 0.5 |

**Viewed (2 images):** `raw/stills/sheet-r1-european-beech.png` and `sheet-r1-norway-spruce.png`, each the S1 reference, today's bare still and the probe's bare and whole stills.

- **The beech grows a pole.** One leader with short side shoots hugging it, no crown. The crown of the young ages lies close to the axis, and the laterals never reach the mature crown's width.
- **The spruce's 7 of 9 is a false pass.** A leader with sparse tiers of thick horizontal branches, the right model, but nearly no fine wood and no leaves: every tip is thicker than the structural boundary, so the pipeline's foliage finds no twig wood. It fails Astra's visual gate.

## What the round shows

1. **Fine wood is two orders short.** The law makes 7k to 36k nodes and a few thousand tips. Pipe radii from the root leave every tip at or above 0.05 of the root's radius, so the scorecard counts 0 to 3.4 km of fine wood against today's 18 to 47 km. A real crown's fine wood is short shoots of a few centimetres, far below the marker scale (a metamer is H ÷ 128, 0.25 m on the beech).
2. **Speed and fine wood pull against each other.** Competition costs per bud and per metamer: 44 to 94 ms already, against budgets of 46 to 79 ms, at a tenth of today's node count. Growing today's fine-wood length as competing metamers would cost several times the budget (fn-188's R3-SPEED: time follows nodes and bud evaluations about linearly).
3. **The clear bole and the crown's spread depend on how the crown's ontogeny is modelled.** Scaling markers per joining age makes a pole on the beech; a fixed envelope with a raised crown base (fn-188 round 9) gives no trunk for the law to grow up through.

## Decisions this round needs from the host

These are design judgments (AGENTS.md, "Dispatch and escalation"), so the run stops here instead of tuning past them.

1. **Short shoots.** Whether they are grown wood (cheap shoots below the marker scale, with occupancy shrinking with shoot length) or foliage stations on grown wood. The spec puts this choice in R2 (the fn-125 contract); R1's fine-wood and speed bounds cannot be met or judged without it.
2. **The fine-wood comparison.** Whether R1's "equal or greater fine-wood length" counts short shoots that are stations, and whether the 0.05-root structural boundary stays the measure (Astra: a convention only).
3. **Ontogeny and the bole.** Scaled crowns per marker, a fixed envelope with a trunk the law is given, or the crown base as an authored row.
4. **Astra's REJECT of the bands** (`R1-BANDS-ASTRA.md`): which bands vote and which are diagnostics. The spec currently keeps every band and adds Astra's visual and developmental gates.

Round 1 of the six the spec allows is spent.
