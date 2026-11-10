# fn-345 R2 and R3: the node count falls with the tip's path, dead wood is not charged

Drafted by the worker on 10 Nov 2026 at `f3ba6daca`, for the host, following `R1-ANSWERS.md`. The probe is an instrument, not a candidate. The reads are the worker's eye on the stills, not the owner's.

Marks used below:
- **measured:** printed by a run.
- **eye:** read off a still.
- **read:** taken from the code.
- **inferred:** reasoned, not run.

## Headline

- **The rule bounds the oak hard, and the oak it bounds is worse than today's from 120 on.**
  - **The counts level and then fall** (seed 1001, measured). Held phytomers peak at 784,000 in year 99 and fall to 329,000 by 299. Living tips peak at 54,000 in year 89 and fall to 24,000. Height levels at 27 m (rough layout) and the crown's ground area at about 245 m².
  - **It fits the desk.** The tree grows to 300 in 22 s and 0.7 GiB, against today's 96 s and 2.9 GiB, so the stills were drawn.
  - **The crown empties as it holds.** Current-year leaves at 300 are 14,090 against today's 233,403. Income falls from 19,191 at 109 to 7,704 at 289. Tips per square metre of crown ground fall from about 400 at 89 to about 100 at 289.
- **The wood breaks from 200 on** (eye). The probe's limbs live on and keep making a node or two a year. By 200 and 300 they curl, loop and droop into a tangle of thin, nearly leafless snaking wood, on both seeds.
  - **The silhouette at 200 and 300 is wide and low,** about 22 m tall and 28 to 35 m across, which is an old open-grown oak's proportion.
  - **The wood that makes it reads as a vine or a weeping tree, not an oak.**
- **At 120 it is not an open crown of few heavy limbs** (eye). It is a thin, sparse, radiating crown of many slender limbs. The trunk is half today's girth: 69 against 143 cm on 1001, and 107 against 150 on 1007. The crown carries a fifth of the leaves on 1001 and two fifths on 1007.
- **The young tree is held** (measured). Restating every row's node counts by 1.192, the factor the R1 arithmetic gave, holds the settled height at 40 within 1.4% on seeds 1001, 1007 and 1012. The oak at 40 reads the same as today's, a little thinner.
- **Other species fall below today's by degree** (measured). At 40, beech is 7% lower, birch 23% lower and spruce 10% lower; beech at 80 is 16% lower. Each keeps its form. Palm and ordinary are unchanged (rule dormant).
- **By the host's bar (R4), this is not a candidate.** The oak at 120 does not read as well as today's, and the oak at 200 and 300 does not read as an old oak.

## What was built (the probe)

The probe is `probes/n.patch`. It is fn-344's `oldoak.patch`, the R1 stem dump, and the rule. It is reverted, it re-applies cleanly, and `git status --short crates` is empty. The letters `BN` switch it on; with them unset, the grower is today's.

- **B, dead wood not charged:** as fn-344 (`grow/cycle/working.rs`, `totals`).
- **N, the rule** (`grow/unit.rs`, `grow_unit`).
  - **The count.** After the zone's node rule has drawn its nodes (`lineage::node_presences`), the zone's count n becomes n × s.
  - **The share.** s = `ABLATE_NODE_K` × max(0, 1 − L / `ABLATE_LMAX`) (`probe_ablate::node_share`).
  - **The rounding.** n × s is made a whole count by a keyed draw: the whole part, plus one with the fraction's probability, from the draw `zone_key.child(0xF345_0000_0000)`. Nodes past the count are dropped, from the zone's last. A node added past n stands whole.
  - **What is untouched:** internode length, the buds at each node, girth, light and vigour.
- **L, the tip's path.** L is read from the tip's rough-layout walk at the start of the year: `Pencil.layer.path_end()`. That is H's path, carried from the root collar along the wood (`geometry/lay.rs`). Under N the path is carried but no length is changed.
- **Where the path reads 0** (read): an axis with no pencil, or a grower with no rough layout. The rule is dormant there; every tree here has a rough layout.
- **A unit whose zones all come out 0 nodes** (read). It is pushed as an empty unit: its survival, persistence and shade are recorded, it grows no phytomer, and the apex lives on to the next year. This is the path a zone with no nodes already takes today, for example a Poisson zone drawing 0 (`grow/unit.rs`: the zone's loop does nothing; `presence::assign_one` and `draw_bent` lay no phytomer for that unit). No run failed on it.
- **The values used:**

| species | L_max | source | status |
|---|---|---|---|
| oak (preset `oregon-white-oak`, *Q. garryana* values) | 34.8 m | Gould, Harrington and Devine 2011: "The maximum potential height for OWO on a good site was 34.8 m" | checked by fn-341 against the text; this run did not re-read it |
| beech | 40 m | waldwissen.net portrait: "it may reach over 40 m in height" | CHECKED in the search excerpt; a lower bound, so an estimate |
| birch | 30 m | European Atlas of Forest Tree Species, *Betula* (Beck et al.): "… 30 m, while Betula pubescens … rarely growing beyond 20 m" | the excerpt was cut before the subject; *B. pendula* is inferred |
| spruce | 61 m | USDA FEIS, *Picea abies*: "In central Europe, heights of up to 203 feet (61 m) have been reported" | CHECKED in the search excerpt; a reported maximum, not a site potential |
| palm, ordinary | unset | none read | rule dormant; B alone is on |

- **`ABLATE_NODE_K`** is 1.192 for the oak. It is unset for every other species, whose counts are not restated, as the host ordered.
- **The runs** are `probes/n.sh`, all inside telperion-heavy:
  - the factor: grow-only at 40, `12 --threads 4`;
  - the census: `22 --gpu 2 --threads 4`;
  - the stills: `22 --gpu 2` for the oak and `12 --gpu 2` for the other presets;
  - the one-thread seconds: grow-only, `taskset -c 3`, without the yearly records.

  No run was stopped or killed. Logs are in `raw/stills/`; the tables come from `probes/measures.py`; the sheets from `probes/sheets.sh`.

## 1. The restating factor (measured)

- **By arithmetic.** R1's replay of today's leader under 1 − L/34.8 gives k = 1.192 to hold year 39. Under N the leader's height gain per year is its node count times an unchanged internode, so the same k holds.
- **By run** (settled height at 40):

| seed | today m | probe, k = 1 m | probe, k = 1.192 m | off |
|---|---|---|---|---|
| 1001 | 10.46 | 8.90 | 10.60 | +1.4% |
| 1007 | 10.26 | | 10.38 | +1.2% |
| 1012 | 10.26 | | 10.32 | +0.6% |

- **One factor held all three seeds inside 3% at the first try. It was not moved.**
- **What the factor does near the root.** Near the root the share is above 1: 1.19 × (1 − 0/34.8). So the trunk lays about 9 nodes a year where today it lays 8. The lowest living limb stands at 5.2 m on 1001 against 4.7, and at 5.0 against 4.5 on 1007.

## 2. The census to 300, seed 1001 (measured)

Every 20 years. Today's (A) and H's rows are from fn-344's grows; N is `raw/stills/oak-300-1001-N-census.log`. Height is the rough layout's, before the settle.

| year | A held | N held | A tips | N tips | A height m | N height m | A ground m² | N ground m² | N GiB |
|---|---|---|---|---|---|---|---|---|---|
| 19 | 54,782 | 70,066 | 4,121 | 5,021 | 6.7 | 7.2 | 14 | 18 | 0.2 |
| 39 | 302,606 | 259,067 | 24,622 | 20,419 | 10.6 | 10.6 | 46 | 40 | 0.3 |
| 59 | 839,535 | 534,307 | 61,634 | 38,243 | 14.0 | 13.2 | 106 | 82 | 0.5 |
| 79 | 1,645,899 | 742,430 | 111,239 | 51,942 | 17.4 | 15.7 | 184 | 113 | 0.6 |
| 99 | 2,371,803 | 784,372 | 160,988 | 53,247 | 21.9 | 17.6 | 316 | 154 | 0.7 |
| 119 | 2,506,773 | 685,939 | 158,355 | 48,126 | 25.9 | 19.5 | 375 | 185 | 0.7 |
| 159 | 1,958,386 | 476,633 | 132,432 | 35,070 | 32.8 | 22.3 | 304 | 205 | 0.6 |
| 199 | 2,179,327 | 350,658 | 148,220 | 26,782 | 38.7 | 24.4 | 332 | 222 | 0.6 |
| 239 | 2,225,565 | 345,510 | 169,759 | 25,635 | 44.9 | 25.9 | 239 | 240 | 0.6 |
| 279 | 2,956,941 | 336,603 | 77,909 | 24,536 | 46.9 | 26.8 | 456 | 241 | 0.6 |
| 299 | 1,598,756 | 329,338 | 94,929 | 23,744 | 50.2 | 27.1 | 419 | 246 | 0.6 |

- **Held and living tips level off, then fall.** Both peak around 90 to 100 and settle at about a third of their peak from 240 on. The device fits the tree with room to spare, so the stills went ahead.
- **Height and ground area level:** 27 m and about 245 m² from about 240.
- **The crown thins as it holds** (`BAL` lines).
  - **Income** falls from 19,191 at 109 to 7,704 at 289.
  - **Tips per square metre of ground** fall from about 400 at 89 to about 100 at 289.
  - **The balance sheds almost nothing:** 1 to 123 tips a decade-year.
- **What makes the fall** (inferred). Once most tips stand at a path where 1.19 (1 − L/34.8) is small, units make 0 or 1 nodes. Few nodes make few buds, so the fine orders (twigs, shoots, short shoots, lifespans 3 to 5 years) are not replaced as they die.

## 3. The measures beside today's (measured)

Peak GiB is for the drawn run. "1-thread s" is a grow-only run pinned to one core.

| tree | v | height m | spread m | trunk cm | lowest living limb m | limbs | leaves | held | peak GiB | 1-thread s |
|---|---|---|---|---|---|---|---|---|---|---|
| oak 40 / 1001 | A | 10.6 | 7.6 × 8.0 | 56 | 4.7 | 3 | 47,279 | 325,454 | 0.7 | 1.2 |
| | N | 10.7 | 6.7 × 6.5 | 49 | 5.2 | 3 | 36,013 | 268,419 | 0.6 | 1.2 |
| oak 120 / 1001 | A | 25.1 | 26.6 × 25.5 | 143 | 4.7 | 4 | 304,559 | 2,370,405 | 3.1 | 32.4 |
| | N | 18.2 | 19.5 × 20.3 | 69 | 5.2 | 4 | 64,862 | 638,421 | 1.1 | 13.8 |
| oak 200 / 1001 | A | 34.3 | 22.1 × 22.1 | 129 | 20.0 | 3 | 270,545 | 2,108,291 | 2.8 | 70.6 |
| | N | 22.3 | 24.5 × 31.4 | 69 | 5.2 | 5 | 27,582 | 316,831 | 0.9 | 22.9 |
| oak 300 / 1001 | A | 40.2 | 26.0 × 37.2 | 120 | 25.5 | 1 | 233,403 | 1,537,433 | 2.9 | 133.8 |
| | N | 22.9 | 28.1 × 33.6 | 73 | 5.2 | 5 | 14,090 | 324,070 | 0.8 | 32.1 |
| oak 40 / 1007 | A | 10.4 | 7.8 × 7.7 | 75 | 4.5 | 8 | 76,368 | 467,764 | 0.8 | 1.7 |
| | N | 10.5 | 7.7 × 7.6 | 67 | 5.0 | 8 | 60,830 | 421,029 | 0.8 | 1.8 |
| oak 120 / 1007 | A | 25.5 | 28.0 × 20.5 | 150 | 5.6 | 5 | 386,363 | 2,879,040 | 4.1 | 57.0 |
| | N | 19.3 | 22.1 × 24.9 | 107 | 5.0 | 10 | 144,818 | 1,419,584 | 2.0 | 29.5 |
| oak 200 / 1007 | A | 37.1 | 29.7 × 34.0 | 226 | 14.4 | 2 | 619,213 | 4,121,440 | 5.0 | 129.2 |
| | N | 22.0 | 31.2 × 29.4 | 112 | 5.0 | 10 | 90,220 | 1,083,019 | 1.7 | 56.7 |
| oak 300 / 1007 | A | 31.9 | 34.9 × 24.0 | 106 | none | 0 | 94,815 | 681,125 | 4.0 | 193.6 |
| | N | 22.8 | 35.0 × 33.2 | 118 | 5.0 | 10 | 48,234 | 1,117,811 | 1.6 | 88.6 |
| beech 40 | A | 9.8 | 5.1 × 7.0 | 26 | | | 16,614 | 83,359 | 0.4 | 0.4 |
| | N | 9.1 | 5.8 × 6.2 | 21 | | | 9,802 | 49,220 | 0.3 | 0.3 |
| beech 80 | A | 17.4 | 19.6 × 20.2 | 130 | | | 463,750 | 1,887,629 | 2.6 | 7.6 |
| | N | 14.7 | 14.2 × 13.6 | 76 | | | 130,200 | 471,984 | 0.9 | 2.6 |
| birch 40 | A | 16.7 | 7.7 × 8.5 | 30 | | | 139,929 | 374,880 | 0.7 | 1.4 |
| | N | 12.9 | 6.7 × 6.3 | 19 | | | 46,389 | 137,506 | 0.4 | 0.8 |
| spruce 40 | A | 12.4 | 7.9 × 7.3 | 34 | | | 4,185,635 | 1,137,653 | 2.0 | 5.1 |
| | N | 11.2 | 7.6 × 7.2 | 31 | | | 3,404,826 | 926,728 | 1.7 | 4.7 |
| palm 40 | A, N | 21.8 | 19.3 × 18.7 | 59 | | | 6,554 | 480 | 0.3 | 0.3 |
| ordinary 40 | A, N | 13.3 | 9.1 × 8.5 | 22 | | | 7,957 | 92,345 | 0.4 | 0.6 |

**How far the other species fall below today's at the ages drawn** (their counts not restated):

| species and age | height | leaves |
|---|---|---|
| beech 40 | −7% | −41% |
| beech 80 | −16% | −72% |
| birch 40 | −23% | −67% |
| spruce 40 | −10% | −19% |

**The girth step under the probe** (measured: oak 120, seed 1001, grow-only `PROBE_STEM`, `raw/stills/oak-120-1001-Nstem-grow.log`):
- **The probe:** the bole is a cylinder of radius 0.344 m. It drops to 0.240 m at the node where the fork's limbs leave, at 5.21 m: a ratio of 0.70 and 10 cm of radius.
- **Today's:** 0.711 → 0.546 m, a ratio of 0.77 and 16.5 cm.
- **The step is the same rule's,** smaller in absolute terms on a thinner bole.
- **The open tube does not show** on any probe still. At 120 a small collar is visible at the fork (`oak-120-1001-N-bole.png`).

## 4. The read, per tree (eye)

The sheets are `sheet-oak-1001.jpg`, `sheet-oak-1007.jpg` and `sheet-others.jpg`. Each row is one tree; the columns are A beside N, leafless, leafed, and the lower trunk.

**Oak 40, both seeds: the same as today's.**
- **The same:** the young tree, its leader and its low limbs. Height is within 1 to 2%.
- **A little thinner:** 9 to 24% fewer leaves, a slightly narrower crown, and the lowest limb about half a metre higher.

**Oak 120, seed 1001: worse.**
- **Today's:** a heavy two-limb V on a thick bole with full limb-end tufts.
- **The probe:**
  - a thin bole (69 cm) under a broad, radiating crown of many slender limbs, sparse fine wood and few leaves;
  - not an open crown of few heavy limbs;
  - the crown's edge reads bare and stringy, not stubby;
  - no swelling or knots;
  - a few limbs show black segments.
- **Why** (inferred): units near the crown's edge make one node or none, so the twigs and short shoots that clothe a limb are not renewed.

**Oak 120, seed 1007: about the same in silhouette, worse in mass.**
- **Today's:** a leaning, lopsided tree.
- **The probe:** a rounder, wider, more open-grown dome from a straight bole, which is nearer an open oak's outline. Its wood is thinner and its leafing sparse (145,000 against 386,000 leaves), and some limbs begin to wander in loops near the base.

**Oak 200 and 300, both seeds: worse; not an old oak.**
- **The silhouette** is wide and low, about 22 m by 28 to 35 m, which is the old open-grown proportion. Today's is a bare pole or a looping lean.
- **The wood is broken.** Limbs curl, coil, loop and droop to the ground. The tree is a tangle of thin snaking wood with tufts at the ends and almost no leaves: 14,090 at 300 on 1001.
- **No limb swells or knots;** the wood is thin.
- **It reads as a weeping or climbing plant,** not heavy oak limbs.
- **Why the wood curls** (inferred, not measured):
  - The limbs live for 500 years and now never die back.
  - Each year's few nodes still carry the limb's relays, wander and bends. Those kinks come per unit and per relay, not per metre, so they gather in less wood.
  - The thin limbs then sag under their load.

**Beech 40: slightly worse.** The same form, 7% lower, and fewer leaves (9,802 against 16,614).

**Beech 80: worse.** The same broad dome, but smaller (14.7 against 17.4 m, and 14 m wide against 20). It is less full and much less leafy. The wood does not break.

**Birch 40: worse.** The same weeping birch, 23% lower and narrower, with a third of the leaves. Its L_max (30 m) costs it most at 40, since a 16.7 m birch is already halfway there.

**Spruce 40: about the same.** The same cone, 10% lower, with 19% fewer needles.

**Palm and ordinary 40: the same.** The rule is dormant, and dead wood not being charged changes nothing measurable.

**Two Trees** is a separate example with its own runner, and it was not drawn.

## For the host

1. **The node count alone, by path, bounds the oak but strips its crown.**
   - A tip far from the root makes almost nothing, so the fine wood that clothes the old limbs is not renewed.
   - The long-lived limbs carry on and curl.
   - Whether the decline should spare the fine orders, read something other than the tip's own path, or be read by the limbs' survival as well, is a design question for the host.
2. **The curling wood.** It appears wherever a long-lived axis lays one or two nodes a year for decades. Measuring the turning per metre on the probe's limbs would confirm the inferred cause, and is not done.
3. **Birch and beech fall 16 to 23% below today's** with their counts not restated. If the rule goes further, each needs its own restating, as the oak's did.
