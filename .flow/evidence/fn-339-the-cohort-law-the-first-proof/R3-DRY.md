# fn-339 R3 dry run: the cohort law on one tree per species

Drafted by the worker on 10 Nov 2026 at `b86bb7cb4`, under the host's answers (`R1-ANSWERS.md`). This is one tree per species at 40 on seed 1001, before any protocol is frozen; it is not the read that counts. The draft protocol is `PROTOCOL-DRAFT.md`.

## Headline

**R1's speed figure was wrong.** It put the saving at 3.5 to 4.8 times on the oak and 3.8 on the beech, but the arithmetic divided a steady state's yearly sum by ten (the correction is dated in `R1-LAW.md`, section 3, item 1). Measured on these trees, the cohort set at 0.5 m holds:
- 94.4% of the oak's yearly units at 40;
- 93.2% of the beech's;
- 81.9% of the spruce's.

By unit count that bounds the saving at 17.9, 14.7 and 5.5 times. At 1.0 m the spruce's branchlets join and its share is 99.8%. The goal log's entry that rested on R1's figure needs the same correction; that entry is the host's.

## What ran

- **The probe.** `probes/replay.patch` applies at `b86bb7cb4` and leaves the crates as they began. It holds fn-335's recorder (`grow_laterals`, `expected_bud_counts`) and two new parts:
  - `grow/cycle/replay.rs` runs the full cycle as the recorder does, with every axis open and none retired. Each year, after the cycle's own shade pass, it keeps the lattice the cycle's leaves cast.
  - `replay/law.rs` holds the law as answered.
- **The lattice is the cycle's own.** Read back at every living apex's tip, the kept lattice gave the light the cycle read there, to the bit: 288,183 reads on the oak, 1,574,234 on the spruce and 53,611 on the beech, largest difference 0.
- **Checks on the law.** Its unit test reproduces `law.py`'s closed form on R1's oak node, within 0.002 units a year. The space crate's `replay` tests passed: 1 passed, 0 failed.
- **The runs.** `cohort_replay` (ignored, core) grows `oregon-white-oak`, `norway-spruce` and `european-beech` at their preset age, 40, on seed 1001. Each ran through `telperion-heavy 8 --threads 4`, at 0.9, 4.2 and 0.2 s. `probes/replay.py` reads each log. The logs and the read are in `raw/` (ignored).
- **Units: "the cycle's" side.** These are the laterals of the cohort ages whose bearer is not itself inside a cohort, gathered by their bearer's growth unit.
  - The bearing units are every growth unit of an axis that stays structural at 0.5 m and is kept by the settle.
  - Every quantity is read on the settled tree, at the tree's age.
  - Counts are weighted by each member's settled scale (its presence), as the law's are.
- **Units: "the law" side.** It is the law on the same unit, with the light the cycle's lattice gave the unit's place each year, and with births either:
  - **expected,** from the tables: each zone's mean nodes × buds × lateral probability, times the unit's presence; or
  - **drawn,** the cycle's own roots on the unit, each at its made presence.
- **Readings made inside the answers.** These are for the host to check.
  1. A cohort's members other than its root are counted flat. Each stands while it lives and for its own `shedding` after; what it bore stands while the root does. The root's death, by shade or by its balance, sheds the whole cohort after the root's `shedding`.
  2. The balance's `q` is the cohort's living apexes times the cell's light, as `contract/shed.rs` reads a lateral's (not its leaf area). Answer 4 says "its leaf's light"; D16 counts apexes.
  3. A bud made in the tree's last cycle counts as a living member that has grown nothing.
  4. Section is the cohort's own `Σ (pipe · scale)²`. Every cohort age's exponent is 2, so this is its root's radius squared. It is joined to the bearer by `r^e_bearer`. The leaf-light girth term is left out on both sides.

## Share of the cycle's yearly units inside the cohorts

These counts are unweighted: every axis that grew a unit in the cycle counts once, which is what the cycle pays for.

| species | tree age | units grown | in cohorts at 0.25 m | at 0.5 m | at 1.0 m |
|---|---|---|---|---|---|
| oak | 10 | 1,012 | 0.761 | 0.952 | 0.952 |
| oak | 20 | 3,437 | 0.802 | 0.956 | 0.956 |
| oak | 30 | 8,444 | 0.776 | 0.944 | 0.944 |
| oak | 40 | 17,914 | 0.750 | 0.944 | 0.944 |
| spruce | 10 | 3,349 | 0.816 | 0.816 | 0.991 |
| spruce | 20 | 25,909 | 0.847 | 0.847 | 0.997 |
| spruce | 30 | 60,869 | 0.834 | 0.834 | 0.998 |
| spruce | 40 | 101,302 | 0.819 | 0.819 | 0.998 |
| beech | 10 | 276 | 0.775 | 0.946 | 0.946 |
| beech | 20 | 486 | 0.800 | 0.930 | 0.930 |
| beech | 30 | 1,619 | 0.773 | 0.931 | 0.931 |
| beech | 40 | 5,520 | 0.776 | 0.932 | 0.932 |

The cohort sets, computed in the probe from the presets' own species, match R1's table:
- oak: [14] at 0.25 m; [13, 14] at 0.5 and 1.0 m;
- spruce: [12] at 0.25 and 0.5 m; [10, 12] at 1.0 m;
- beech: [14] at 0.25 m; [12, 14] at 0.5 and 1.0 m.

The shares hold steady from 10 to 40, so the 0.5 m bound does not shrink with age on these trees.

## The law beside the cycle on the same wood, at 0.5 m (the tree's age)

The ratio is the law over the cycle.

| quantity | oak: cycle | law, expected | ratio | spruce: cycle | law | ratio | beech: cycle | law | ratio |
|---|---|---|---|---|---|---|---|---|---|
| new shoots in the last year | 15,350 | 15,840 | 1.032 | 55,880 | 56,360 | 1.009 | 2,483 | 2,305 | 0.928 |
| apexes living | 23,700 | 20,470 | 0.864 | 82,580 | 63,000 | 0.763 | 4,296 | 2,996 | 0.697 |
| members standing | 45,300 | 40,910 | 0.903 | 203,200 | 183,000 | 0.901 | 5,587 | 3,909 | 0.700 |
| current-year leaf, m² | 152.3 | 117.5 | 0.771 | 75.0 | 75.7 | 1.009 | 15.26 | 14.15 | 0.927 |
| standing leaf, m² | 152.3 | 117.5 | 0.771 | 307.5 | 310.1 | 1.009 | 15.29 | 14.15 | 0.925 |
| wood, m | 2,635 | 1,636 | 0.621 | 9,160 | 9,240 | 1.009 | 324 | 234 | 0.722 |
| section, Σ r² | 0.1431 | 0.0932 | 0.652 | 0.0518 | 0.0739 | 1.426 | 0.0223 | 0.0205 | 0.921 |
| section joined to the bearer | 0.1425 | 0.0928 | 0.651 | 0.0518 | 0.0739 | 1.426 | 0.0216 | 0.0198 | 0.917 |

- **Births.** Expected births and the cycle's own births give the same law within 3% on every quantity: on the oak, new shoots 15,840 against 15,390. Births are not where the law departs.
- **The balance hazard never fired** on any cohort root of the three trees: 0 roots shed by it. The answer asked for this to be said.
- **The oak and the vigour split.** The cycle's cohorts at size 1 carry 0.747 of their sized current-year leaf (113.7 m²) and 0.759 of their wood (2,001 m).
  - Against those, the law's leaf is 1.033 and its wood 0.818.
  - So the oak's leaf gap is the size the cycle gives its twigs and shorts (answer 6), and so are about two thirds of its wood gap: 634 of the 999 m. What makes up the rest of the wood gap was not isolated.
  - By light, the law's leaf over the cycle's falls from 1.36 in the darkest fifth of nodes to 0.48 in the brightest, while its new shoots stay within 0.95 to 1.10. Size, not survival, moves the leaf.
  - The spruce and beech have `shade_size` 0, and their at-size-1 values equal their sized ones.
  - **Unexplained:** the sizes are normalised to a mean of 1 per age, yet the oak's sized leaf is 1.34 times its unsized. The normalisation weighs buds by the grower's presence; the settled scale weighs them differently. This was not traced further.
- **The spruce's section is 1.43 times the cycle's.** Its leaf, wood and phytomers agree within 1%. The cycle's section sums `(pipe · scale)²`, so a phytomer at presence `p` adds `p²`, while the law's expectation adds `p`. A section is not linear in presence, and the law as answered carries no per-phytomer presence. This is a definitional gap for the host. It hardly shows on the oak and beech; their section gaps follow their wood.
- **The beech's standing, living and wood run 0.70 to 0.72** while its new shoots and leaf are at 0.93. This is consistent with answer 2's rule, which drops a dead root's whole cohort after its `shedding` of 1, where the cycle keeps a dead shoot standing while its shorts live. Not isolated.
- **Living apexes run 0.70 to 0.86** on all three, beyond what new shoots differ by. Not isolated. One candidate is the presence the cycle's newly made buds carry against the law's whole newborns.

**Per lattice node** (law with expected births, against the cycle, over nodes where either holds leaf):

| species | quantity | nodes | median ratio | p10 | p90 | share within 25% | correlation |
|---|---|---|---|---|---|---|---|
| oak | new shoots | 596 | 1.048 | 0.545 | 1.762 | 0.453 | 0.915 |
| oak | current-year leaf | 596 | 0.891 | 0.304 | 1.937 | 0.289 | 0.726 |
| oak | wood | 631 | 0.731 | 0.218 | 1.527 | 0.303 | 0.749 |
| spruce | new shoots | 967 | 1.008 | 0.882 | 1.195 | 0.912 | 0.998 |
| spruce | current-year leaf | 967 | 1.008 | 0.882 | 1.195 | 0.912 | 0.998 |
| spruce | wood | 969 | 1.006 | 0.914 | 1.140 | 0.957 | 0.999 |
| beech | new shoots | 249 | 0.905 | 0.323 | 1.559 | 0.470 | 0.933 |
| beech | current-year leaf | 249 | 0.904 | 0.317 | 1.613 | 0.458 | 0.931 |
| beech | wood | 259 | 0.851 | 0.000 | 1.565 | 0.425 | 0.826 |

The oak's and beech's nodes hold a few units each (10,171 bearing units in 788 oak nodes; 2,931 in 338 beech nodes), so a node's ratio is noisy where its whole-number cycle is beside an expectation. The spruce's nodes hold about 200 units each.

## The whole tree beside R2

The law's whole tree is the structure's own (the cycle's) plus the law's cohorts. New shoots are weighted by presence; the cycle's unweighted count is in the share table.

| species | measure | cycle | structure + law | R2's checked range |
|---|---|---|---|---|
| oak | new shoots in its 40th year | 16,223 (17,914 unweighted) | 16,715 | no source |
| oak | standing leaf, m² | 162.5 | 127.6 | — |
| oak | leaf area index over its leaf-bearing 0.5 m columns | 4.45 (146 columns) | 3.62 (141) | none checked; 2.4 to 3.3 unchecked; 5 in a mixed floodplain stand |
| spruce | new shoots in its 40th year | 68,464 (101,302) | 68,943 | 691 to 2,372 a tree, 15.5 m trees of about 36 years (Bellan 2019) |
| spruce | standing needle area, m² | 465.4 | 468.0 | — |
| spruce | leaf area index over its columns | 16.5 (113) | 16.6 (113) | 10.5 projected, 28-year plantation (Bolstad and Gower 1990) |
| beech | new shoots in its 40th year | 2,694 (5,520) | 2,516 | no source |
| beech | standing leaf, m² | 17.4 | 16.2 | — |
| beech | leaf area index over its columns | 0.81 (86) | 0.80 (81) | 5.6 to 9.5 (Leuschner 2006); 7.1 and 7.4 optical (Glatthorn 2017) |

- The spruce grows 29 to 99 times the measured tree's new shoots: 68,464 weighted against 691 to 2,372, or 43 to 147 times counted unweighted.
- The beech's index is about a tenth of a beech stand's.
- The oak's leaf comes down 21% under the law, through size alone.

Two caveats on these comparisons:
- An index over a lone tree's own columns is not a stand's index; the protocol draft names that.
- The spruce's needle area is projected (`spruce.rs`), so it compares with a projected index.

As R1 predicted, the law moves these numbers only where the cycle departs from its own expectation, and on these three trees that is the oak's size.

## What this dry run changes in the protocol

- Counts are weighted by presence on both sides, or a count compares tiny made buds with expectations: the unweighted roots read 0.52 to 0.90.
- Per-node ratios are noisy on the oak and beech. The draft reads them in light bins with fixed edges and as totals per bin, not node by node.
- Section is read but flagged: its presence-squared gap is the law's, not noise.
- A lone tree's index is read over its own columns and named as such.
