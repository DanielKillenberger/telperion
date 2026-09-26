# fn-167 blend smoothness

R4 lets a walk appear only once it is measured smooth. This file records
every walk measured for the video, at seed 1 on generator revision
`b1d15d93`. The date palm was not measured and does not appear in a walk:
fn-148 owns its jump.

## Method

`video/dump` (`dump blend <from> <to> 1 [overlay]`) builds the family at each
step of t through `blend::families` and `pipeline::build(Request::mesh())`.
It reads the skeleton's height and horizontal spread, the node count, the
wood's vertex count and the kept leaves. With `BLEND_SPAN=a,b` it steps a
narrower span, which tells a steep stretch from a jump.

1. **Coarse:** 21 steps (Δt 0.05), as the brief asked; `video/smoothness.py`.
2. **Fine:** 201 steps (Δt 0.005), ten spans run in parallel;
   `video/finescan.py`. A walk is smooth here when no fine step moves height
   or spread by more than 3%, or nodes or leaves by more than 10%.

The thresholds are this task's working line, not a product definition. What
"measured smooth" should mean for a walk is left to the host (see Open).

## Preset to preset: none measured smooth

Every pair of the seven branching presets, 21 steps. Each cell is the largest
relative change in one 0.05 step and the t it lands at.

| From | To | Nodes | Height | Spread | Nodes step | Leaves step |
|---|---|---|---|---|---|---|
| european-beech | laurelin | 187,968 to 53,155 | 13% at 0.05 | 17% at 0.05 | 90% at 0.05 | 78% at 0.05 |
| european-beech | silver-birch | 187,968 to 68,501 | 7% at 0.95 | 10% at 0.75 | 26% at 0.50 | 34% at 0.95 |
| european-beech | telperion | 187,968 to 75,697 | 15% at 0.05 | 15% at 0.05 | 90% at 0.05 | 79% at 0.05 |
| norway-spruce | european-beech | 95,389 to 187,968 | 6% at 0.20 | 9% at 0.10 | 57% at 0.05 | 76% at 0.05 |
| norway-spruce | laurelin | 95,389 to 53,155 | 26% at 0.05 | 32% at 0.05 | 95% at 0.05 | 99% at 0.05 |
| norway-spruce | silver-birch | 95,389 to 68,501 | 7% at 0.95 | 8% at 0.50 | 53% at 0.50 | 70% at 0.05 |
| norway-spruce | telperion | 95,389 to 75,697 | 29% at 0.05 | 29% at 0.05 | 96% at 0.05 | 99% at 0.05 |
| ordinary | european-beech | 11,567 to 187,968 | 6% at 0.20 | 6% at 0.05 | 90% at 1.00 | 81% at 1.00 |
| ordinary | laurelin | 11,567 to 53,155 | 17% at 0.15 | 26% at 0.10 | 87% at 0.95 | 87% at 0.95 |
| ordinary | norway-spruce | 11,567 to 95,389 | 10% at 0.40 | 14% at 0.05 | 87% at 1.00 | 94% at 1.00 |
| ordinary | oregon-white-oak | 11,567 to 125,087 | 6% at 0.25 | 11% at 0.20 | 94% at 1.00 | 98% at 1.00 |
| ordinary | silver-birch | 11,567 to 68,501 | 7% at 0.35 | 9% at 0.05 | 84% at 1.00 | 88% at 1.00 |
| ordinary | telperion | 11,567 to 75,697 | 20% at 0.15 | 20% at 0.10 | 45% at 0.25 | 54% at 0.25 |
| oregon-white-oak | european-beech | 125,087 to 187,968 | 5% at 0.10 | 3% at 1.00 | 30% at 0.05 | 36% at 0.05 |
| oregon-white-oak | laurelin | 125,087 to 53,155 | 17% at 0.05 | 19% at 0.05 | 92% at 0.05 | 97% at 0.05 |
| oregon-white-oak | norway-spruce | 125,087 to 95,389 | 9% at 0.15 | 8% at 0.85 | 48% at 1.00 | 59% at 1.00 |
| oregon-white-oak | silver-birch | 125,087 to 68,501 | 5% at 0.45 | 6% at 0.40 | 42% at 0.50 | 44% at 0.50 |
| oregon-white-oak | telperion | 125,087 to 75,697 | 19% at 0.05 | 18% at 0.05 | 91% at 0.05 | 96% at 0.05 |
| silver-birch | laurelin | 68,501 to 53,155 | 29% at 0.05 | 27% at 0.05 | 61% at 0.95 | 77% at 0.05 |
| silver-birch | telperion | 68,501 to 75,697 | 28% at 0.05 | 27% at 0.05 | 76% at 1.00 | 78% at 1.00 |
| telperion | laurelin | 75,697 to 53,155 | 4% at 0.25 | 7% at 0.05 | 75% at 0.15 | 74% at 0.15 |

The steps fall into two kinds, told apart at finer steps:

- **At an end of the walk.** oregon-white-oak to Telperion: 125,087 nodes at
  t=0, 4,078 at t=0.005. Four of the five real species (all but the date palm) set
  `attractorWeight = 0`, and Telperion, Laurelin and ordinary pull on
  attractors; ordinary to any real species jumps between t=0.95 and 1.0 the
  same way. Oak to beech, where both sides pull on nothing, also jumps: +30%
  nodes near t=0.0125, and it meets the 250,000-node cap at t=0.5 and 0.8.
- **In the middle, one step at a time.** silver-birch to beech falls from
  241,951 to 168,946 nodes between t=0.5000 and 0.5025, and ordinary to
  Telperion rises from 30,775 to 55,994 between t=0.7475 and 0.75 (and steps
  at 0.25). They sit at t=0.25, 0.5 and 0.75, where a count row one or two
  units apart would round to its next value; which row steps there was not
  checked. STRATEGY.md allows a count to step by one unit; whether a
  whole-crown step of this size reads as smooth is not measured here.

No preset morph is in either cut.

## Parameter sweeps

| Walk | Rows swept | Fine-scan verdict | Largest fine step |
|---|---|---|---|
| Telperion writhe | `/skeleton/bias/supernatural/writheAmplitude` 0 to 0.11 | not smooth | nodes 13.2% at t=0.170, leaves 14.3%, spread 8.2% at t=0.100 |
| Telperion lean | `/skeleton/bias/lean` 0 to 0.04 | not smooth | spread 5.9% at t=0.435 (nodes 4.9%, leaves 5.9%) |
| Telperion twist | `/surface/twistRate` 0 to 2.4 | **smooth** | none: height, spread, nodes, wood vertices and leaves are identical at all 201 steps |

The twist sweep changes the wood surface and nothing upstream of it, so the
whole-tree metrics cannot move. What moves is the surface: the farthest any
wood vertex travels per 0.05 of t is 0.95 to 1.05 m at every one of the
20 steps (`woodShift` in `raw/blend/telperion-twist-shift.jsonl`), on a tree
142 m tall. It is the cut's live parameter sweep (shot 12, V4).

The writhe is shown as the bias field (shot 10), but as two renders at one
camera with a wipe, not a blend, because its walk did not measure smooth.

## Open

- R4's preset-to-preset morph is not in either cut. The host decides whether
  a one-unit count step passes as smooth, whether a sub-span of a walk may
  stand for a morph, or whether a generator spec comes first.
- The end-of-walk cliffs are not at a rounding point: a family at t=0.005
  differs from the one at t=0 by far more than its rows do. Their cause is
  unknown; they are measured here, not diagnosed.
