# fn-335 result: what a fine lateral really grows, by its light

Drafted by the worker on 9 Nov 2026 under the protocol the host froze (`PROTOCOL.md`). The next step is the host's and is not written here.

## Verdict (R5)

**Near.** A stand-in read from local state, with span and light as its inputs, predicts the standing leaf, section and wood of every preset's fine laterals within 10% for the whole tree and within 20% for the worst bearer-age group. It does not reach go on any preset but the spruce. No model was trained, and no product row or behaviour changed.

Beside the verdict, by the same thresholds:
- **Span alone: no-go.** The beech misses by 18% on the whole tree, and the oak's leaf misses by 11%.
- **One curve: no-go.** The other three presets' shares miss by 15 to 69% on the whole tree.

## What ran (R2, R3)

- The recorder `grow_laterals` grows the tree as the reference working set does: every axis open and none retired. It reads each lateral twice, before and after the settle.
  - **Grown** is read before the settle's shed: phytomers counted by age, including those of the last cycle.
  - **Standing** is read after the settle's shed and scale: phytomers summed by their settled scale, including those of the last cycle.
  - Its test `the_kept_laterals_are_the_full_cycles` holds the standing sums of every kept lateral equal to the subtree sums of the tree `grow_cycle` returns (oak, age 12, seed 1001, matched by lineage). So the standing quantities are read from the settle, and the grower is not changed.
- `lateral_records` (core, ignored) grows the oak, spruce, birch and beech at their preset age of 40, full cycle, on seeds 1001 to 1012. Each preset ran one thread through `telperion-heavy`, pinned to its own core.
- `probes/read.py` reads the output as a pipe. Pass 1 grows the fit seeds and cuts the light bins. Pass 2 grows all 12 seeds and reduces the lines to cell sums. No record line reached the disk. The four reads took under 3 minutes together.
- `probes/verdict.py` applies the rule. Its output is `raw/read/verdict.txt`, and each preset's cell sums are `raw/read/<preset>.json` (ignored).
- `cells.csv` holds every cell (preset, lateral age, span bin, light bin) of the fit seeds. For each cell it gives the laterals and the mean, 10th, 50th and 90th percentiles of seven values: grown leaf share, standing leaf, standing section, standing wood, and the three standing shares of the potential.

**Fine ages,** by fn-334's reach rule at 1.0 m (s above 1). The listed ages are those with laterals; the other coarsened ages grow nothing.
- Oak: 7, 8, 9, 13, 14
- Spruce: 8, 10, 12
- Birch: 9, 14
- Beech: 9, 12, 14

The beech set is computed here for the first time.

**Laterals left out or read at 0,** over all 12 seeds:

| Preset | Fine laterals | Span 0 (left out) | Grew nothing, read at share 0 |
| --- | --- | --- | --- |
| oak | 2,821,014 | 268,411 | 892,250 |
| spruce | 9,039,577 | 610,405 | 959,487 |
| birch | 1,355,992 | 112,827 | 154,540 |
| beech | 375,433 | 44,214 | 32,686 |

No fine lateral with a span was refused by the tables.

## The read (R4): median error over the six test trees, whole tree / worst bearer-age group

**Span and light, the best predictor:**

| Preset | Leaf | Section | Wood |
| --- | --- | --- | --- |
| oak | 0.060 / 0.104 | 0.054 / 0.110 | 0.053 / 0.111 |
| spruce | 0.009 / 0.109 | 0.012 / 0.053 | 0.009 / 0.053 |
| birch | 0.064 / 0.118 | 0.054 / 0.114 | 0.047 / 0.125 |
| beech | **0.099** / 0.173 | 0.079 / 0.161 | 0.075 / 0.161 |

**Span alone:**

| Preset | Leaf | Section | Wood |
| --- | --- | --- | --- |
| oak | 0.112 / 0.146 | 0.078 / 0.121 | 0.072 / 0.117 |
| spruce | 0.007 / 0.086 | 0.005 / 0.026 | 0.005 / 0.026 |
| birch | 0.097 / 0.116 | 0.101 / 0.107 | 0.108 / 0.101 |
| beech | 0.176 / 0.187 | 0.181 / 0.183 | 0.181 / 0.181 |

**Span, light and branching order:**

| Preset | Leaf | Section | Wood |
| --- | --- | --- | --- |
| oak | 0.077 / 0.089 | 0.063 / 0.063 | 0.062 / 0.062 |
| spruce | 0.009 / 0.109 | 0.012 / 0.053 | 0.009 / 0.053 |
| birch | 0.059 / 0.107 | 0.046 / 0.107 | 0.038 / 0.128 |
| beech | 0.112 / 0.175 | 0.094 / 0.157 | 0.089 / 0.159 |

This reads no-go because of the beech's leaf at 0.112. 96 oak, 58 birch and 30 beech test laterals fall in an order cell that no fit lateral holds (see the readings below).

**One curve (span and light, the other three presets' shares):**

| Preset | Leaf | Section | Wood |
| --- | --- | --- | --- |
| oak | 0.153 / 1.167 | 0.493 / 0.817 | 0.524 / 0.816 |
| spruce | 0.221 / 0.890 | 0.690 / 0.493 | 0.619 / 0.488 |
| birch | 0.046 / 0.208 | 0.373 / 0.451 | 0.553 / 0.532 |
| beech | 0.336 / 0.268 | 0.435 / 0.385 | 0.561 / 0.493 |

All six test trees grew on every preset, and every quantity was read.

**What the numbers show:**
- Adding light lowers the whole-tree error on the oak, birch and beech. The largest drop is the beech's, from 0.176 to 0.099 for leaf.
- On the spruce, span alone is already the closest.
- The near verdict rests on the beech's leaf at 0.099, against a threshold of 0.10.
- The worst group is above 10% on every preset under every predictor except the spruce's section and wood, and the oak's under span, light and order.

## Readings made inside the protocol, for the host to check

1. **Standing scale** is the scale of the settled tree: after the shed and the settle's scale pass, which carries the fade and the inherited node scale. It is the scale on the tree `grow_cycle` returns, and the test holds them equal. The scale before that pass would leave out the fade.
2. **The worst-group error** is taken per test tree as the worst group of at least 5% of that tree's fine laterals. Its median over the six trees is reported.
3. **An empty order cell.** A test lateral whose order cell no fit lateral holds is given nothing and counted (above). This happens only under the third predictor, which is not the best, so it cannot move the verdict: the beech misses go on every predictor.
4. **One curve** pools the other presets' cells by span bin and light-bin index. Each preset's bins are its own equal-count quintiles per age, so the same index covers different light ranges on different presets.
5. **Light bins** are cut at the 20th, 40th, 60th and 80th percentiles of the fit seeds' lights per preset and age. Light values at a cut fall into the upper bin. Where lights tie, bins are unequal; the spruce's age 10 has its top cut at 1.0. Raw ranges and cuts are in `raw/read/<preset>.json`.
6. **Laterals that grew nothing** read at their bearer's light, with all values 0, and enter every cell and total.

## What surprised the worker

1. **Order sharpens position but not the whole tree.** On the oak, adding the branching order lowers the worst group from 0.104 to 0.089 for leaf, while the whole-tree error rises from 0.060 to 0.077.
2. **The tip ages hold nearly all fine laterals,** and over a third of the oak's grew nothing: 867,690 of its 2,506,768 age-14 laterals over 12 seeds. The oak's age-14 laterals per seed average 208,897, against 121,857 on seed 1001 in the dry run, so the count varies widely by seed. Inferred from the totals; the per-seed counts were not kept.
3. **The one curve's shares carry each species' scale.** Leaf misses least (the birch's whole tree at 0.046), and section and wood miss by 37 to 69%. Inferred: pipe and internode differ by species, while leaf area per node differs less. This was not tested.

## Verification (R6)

The code that measures stays out of the crates. It is kept as `probes/recorder.patch`, which applies at `03fae16f4`, so the crates end as they began. Under R6 no gate or review is owed for code that does not stay in the tree; the recorder's own test passed with the patch applied.
