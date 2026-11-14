# fn-335 result: what a fine lateral really grows, by its light

Drafted by the worker on 9 Nov 2026 under the protocol the host froze (`PROTOCOL.md`). The next step, in `NEXT.md` beside this file, is the host's.

## Verdict (R5)

**No-go.** No predictor passes the rule on every preset, and the beech fails all three. Its whole-tree error is above 10% on every quantity under every predictor. Its best is 0.104 to 0.110 with span and light.

The oak, spruce and birch would read near with span and light: whole tree within 5.4%, worst bearer-age group within 12.2%. The spruce alone would read go.

No model was trained, and no product row or behaviour changed.

Beside the verdict, by the same thresholds:
- **Span alone: no-go.** The beech misses by 18 to 19% on the whole tree, the oak's leaf by 11%, and the birch's wood by 11%.
- **One curve: no-go.** The other three presets' shares miss by 54 to 75% on the whole tree.

## A correction to the first read

The first read cut span into three bins (1 to 8, 9 to 16, 17 and over). The protocol gives ten: spans 1 to 8 each as its own bin, then 9 to 16, then 17 and over. The first read's verdict was near, resting on the beech's leaf at 0.099 against a threshold of 0.10, under span and light; span alone and one curve were no-go there too. That was a mistake in the probe. Under the protocol, the reader was fixed and the whole read repeated, with the same seeds, fit and test split, predictors and one-curve test.

The recorder and the probe did not change, only the reader's span bins. The repeat read the same trees: every preset's lateral counts and light cuts are identical. The first read's output is kept in `raw/read-three-bins/` (ignored).

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

**Span alone:**

| Preset | Leaf | Section | Wood |
| --- | --- | --- | --- |
| oak | 0.114 / 0.150 | 0.077 / 0.124 | 0.071 / 0.121 |
| spruce | 0.004 / 0.015 | 0.006 / 0.024 | 0.004 / 0.024 |
| birch | 0.094 / 0.101 | 0.100 / 0.106 | 0.108 / 0.099 |
| beech | 0.180 / 0.177 | 0.189 / 0.186 | 0.188 / 0.183 |

**Span and light:**

| Preset | Leaf | Section | Wood |
| --- | --- | --- | --- |
| oak | 0.049 / 0.107 | 0.044 / 0.108 | 0.044 / 0.109 |
| spruce | 0.002 / 0.024 | 0.010 / 0.017 | 0.006 / 0.017 |
| birch | 0.054 / 0.104 | 0.049 / 0.112 | 0.043 / 0.122 |
| beech | **0.105** / 0.154 | **0.110** / 0.163 | **0.104** / 0.163 |

10 oak and 20 spruce test laterals fall in a cell that no fit lateral holds (see the readings below).

**Span, light and branching order:**

| Preset | Leaf | Section | Wood |
| --- | --- | --- | --- |
| oak | 0.072 / 0.084 | 0.065 / 0.053 | 0.063 / 0.053 |
| spruce | 0.002 / 0.024 | 0.010 / 0.017 | 0.006 / 0.017 |
| birch | 0.052 / 0.100 | 0.043 / 0.102 | 0.036 / 0.126 |
| beech | 0.113 / 0.148 | 0.121 / 0.160 | 0.119 / 0.160 |

258 oak, 20 spruce, 104 birch and 123 beech test laterals fall in a cell that no fit lateral holds.

**One curve (span and light, the other three presets' shares):**

| Preset | Leaf | Section | Wood |
| --- | --- | --- | --- |
| oak | 0.671 / 0.710 | 0.615 / 0.730 | 0.639 / 0.729 |
| spruce | 0.541 / 0.433 | 0.746 / 0.508 | 0.683 / 0.503 |
| birch | 0.671 / 0.822 | 0.556 / 0.649 | 0.669 / 0.654 |
| beech | 0.741 / 0.767 | 0.579 / 0.563 | 0.670 / 0.617 |

All six test trees grew on every preset, and every quantity was read.

**What the numbers show:**
- Adding light lowers the whole-tree error on the oak (leaf 0.114 to 0.049), the birch (0.094 to 0.054) and the beech (0.180 to 0.105). It does not bring the beech within 10%.
- On the spruce, span alone already reads go.
- Adding branching order lowers the worst group on the oak, and raises the whole-tree error on the oak and the beech.

## Readings made inside the protocol, for the host to check

1. **Standing scale** is the scale of the settled tree: after the shed and the settle's scale pass, which carries the fade and the inherited node scale. It is the scale on the tree `grow_cycle` returns, and the test holds them equal. The scale before that pass would leave out the fade.
2. **The worst-group error** is taken per test tree as the worst group of at least 5% of that tree's fine laterals. Its median over the six trees is reported.
3. **An empty fit cell.** A test lateral whose cell no fit lateral holds is given nothing and counted (above). The beech has none under span and light, and its whole-tree errors there are above 10% on every quantity. So this cannot move the verdict.
4. **One curve** pools the other presets' cells by span bin and light-bin index. Each preset's bins are its own equal-count quintiles per age, so the same index covers different light ranges on different presets.
5. **Light bins** are cut at the 20th, 40th, 60th and 80th percentiles of the fit seeds' lights per preset and age. Light values at a cut fall into the upper bin. Where lights tie, bins are unequal; the spruce's age 10 has its top cut at 1.0. Raw ranges and cuts are in `raw/read/<preset>.json`.
6. **Laterals that grew nothing** read at their bearer's light, with all values 0, and enter every cell and total.

## What surprised the worker

1. **The finer span bins moved the presets in different directions.** The oak, spruce and birch read closer than under three bins. The beech's span-and-light errors rose from 0.075 to 0.099 to 0.104 to 0.110.
2. **One curve fails by more under the finer bins:** 54 to 75% on the whole tree, against 5 to 69% under three bins.
3. **The tip ages hold nearly all fine laterals,** and over a third of the oak's grew nothing: 867,690 of its 2,506,768 age-14 laterals over 12 seeds. The oak's age-14 laterals per seed average 208,897, against 121,857 on seed 1001 in the dry run, so the count varies widely by seed. Inferred from the totals; the per-seed counts were not kept.

## Verification (R6)

The code that measures stays out of the crates. It is kept as `probes/recorder.patch`, which applies at `03fae16f4`, so the crates end as they began. Under R6 no gate or review is owed for code that does not stay in the tree; the recorder's own test passed with the patch applied.
