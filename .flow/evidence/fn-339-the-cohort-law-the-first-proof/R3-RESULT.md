# fn-339 R3 result: the cohort law replayed on full trees

The worker read this on 10 Nov 2026 under `PROTOCOL.md`, frozen by the host at `2f1c5ed20`. The trees are the oak, spruce and beech at 40, seeds 1001 to 1012, with the boundary at 0.5 m.

- The full tables are `R3-TABLES.md`.
- The probe is `probes/replay.patch` (corrected below) and the reader is `probes/r3.py`.
- Each species ran as one `telperion-heavy 8 --threads 4` process.
- The kept lattice gave the cycle's own light at all 23,970,174 apex reads, with a largest difference of 0.

## Corrections made after the first read, and the read repeated

1. **Size was counted twice on the cycle's side** (also in the dry run). The settle's scale already holds the unit's size (`geometry.rs:97`, `phytomer.scale *= phytomer.size`). The probe multiplied by size again. Now the cycle's leaf, wood and section use the settled scale as it is. Its counts and "size 1" values use scale over size, which is the phytomer's presence.
2. **The bearing-unit size form scaled leaf and wood only.** Size scales a phytomer's section too, so it now scales section (as size²) and join (as size^e) as well.

The first read is `raw/r3-*.log` (ignored); the repeat is `raw/r3b-*.log`. Every number below is from the repeat.

## The size trace (PROTOCOL.md, size)

**Found.** The dry run's "sized leaf 1.34 times the leaf at size 1" was the probe's double count above. It was not the cycle.

| oak age | units in the last cycle | mean size: unweighted | survival-weighted | pencil-weighted | settled-scale-weighted |
|---|---|---|---|---|---|
| 13 (twig) | 76,109 | 0.931 | 0.931 | 1.036 | 1.498 |
| 14 (short) | 223,045 | 1.070 | 1.072 | 1.141 | 1.519 |

- Sizes average near 1 under the grower's own weights.
- Weighted by a scale that already contains them, they read 1.5, which is E[s²]/E[s].
- The spruce's and beech's sizes are 1 throughout (`shade_size` 0).
- Took about 15 minutes.

## Shares of the cycle's units inside the cohorts

Median over the twelve seeds, with the range. These are unweighted: what the cycle pays for.

| species | age 10 | age 20 | age 30 | age 40 |
|---|---|---|---|---|
| oak at 0.5 m (= 1.0 m) | 0.952 (0.944–0.959) | 0.948 | 0.931 | 0.925 (0.900–0.944) |
| oak at 0.25 m | 0.755 | 0.779 | 0.727 | 0.694 (0.628–0.750) |
| spruce at 0.5 m (= 0.25 m) | 0.809 | 0.849 | 0.835 | 0.820 (0.818–0.823) |
| spruce at 1.0 m | 0.989 | 0.997 | 0.998 | 0.998 |
| beech at 0.5 m (= 1.0 m) | 0.932 | 0.932 | 0.922 | 0.937 (0.915–0.941) |
| beech at 0.25 m | 0.753 | 0.801 | 0.767 | 0.782 |

**The unit-count bound at 0.5 m, at 40:**

| species | median | range over seeds |
|---|---|---|
| oak | 13.3 times | 10.0 to 17.9 |
| spruce | 5.6 times | 5.5 to 5.6 |
| beech | 15.9 times | 11.8 to 16.9 |

The oak's share falls slowly with age, from 0.952 to 0.925.

## The law beside the cycle on the same wood

The table gives the law over the cycle per tree (median over seeds, with the range), sorted by the protocol's rule into agrees or differs. It reads the law two ways:
- **subtree-size:** the species' shedding rule at the bearing unit's size;
- **root-1:** R1-ANSWERS.md's root rule at size 1.

The other readings are in `R3-TABLES.md`.

| quantity | oak, subtree-size | oak, root-1 | spruce (all readings alike) | beech, subtree-size | beech, root-1 |
|---|---|---|---|---|---|
| new shoots | 1.006, agrees | 0.995, agrees | 1.012, agrees | 1.007, agrees | 0.911, agrees |
| apexes living | 0.735, differs | 0.735, differs | 0.771, differs | 0.792, differs | 0.753, differs |
| members standing | 0.839, differs | 0.808, differs | 0.906, agrees | 0.854, differs | 0.745, differs |
| current-year leaf | 1.143, differs | 0.896, differs | 1.012, agrees | 1.012, agrees | 0.910, agrees |
| standing leaf | 1.143, differs | 0.896, differs | 1.011, agrees | 1.008, agrees | 0.909, agrees |
| wood | 0.991, agrees | 0.724, differs | 1.011, agrees | 0.956, agrees | 0.711, differs |
| section (second moment) | 0.912, differs | 0.404, differs | 0.948, agrees | 0.930, agrees | 0.703, differs |
| section joined to the bearer | 0.914, differs | 0.403, differs | 0.948, agrees | 0.932, agrees | 0.702, differs |

**The oak's leaf by light**, the bins of 0 to 0.2, 0.2 to 0.4, 0.4 to 0.6 and 0.6 to 0.8:

| reading | 0–0.2 | 0.2–0.4 | 0.4–0.6 | 0.6–0.8 | per tree |
|---|---|---|---|---|---|
| size 1 | 1.60 | 0.80 | 0.59 | 0.52 | 0.91 |
| bearing unit's size | 1.20 | 1.14 | 1.08 | 1.11 | 1.14 |

The bearing unit's size takes out the light trend. What is left is a level 14% over the cycle.

**Shedding.** The species' rule closes the beech's wood from 0.711 to 0.956 and its standing members from 0.745 to 0.854. On the oak it raises standing from 0.808 to 0.839 and wood from 0.724 to 0.824 at size 1. So the beech's gap in the dry run was the root rule.

**Section.** The second moment, taken where the tables give it (the count's fraction and the bearer's presence), brings the spruce from the dry run's 1.43 to 0.948.
- What is left is the grower's per-phytomer presences (leads, windows), which the tables do not give.
- The law's second moment cannot carry these, so the gap is named and not closed.
- With the cycle's own births, the spruce's section reads 1.006.

**Living apexes run 0.73 to 0.79** on all three species, in every reading and every light bin. The cause is not isolated. Births, new shoots and leaf agree, so the gap is in which apexes the two count as alive at the tree's age, not in growth.

**The balance hazard** shed no cohort root on the spruce or beech. On the oak it shed a median of 0 roots a tree, and at most 0.34 of a root, of about 23,000.

**Per lattice node** (subtree-1 against the cycle, median over seeds):

| species | quantity | nodes within 25% | median ratio |
|---|---|---|---|
| spruce | new shoots | 0.906 | — |
| spruce | wood | 0.952 | — |
| oak | new shoots | 0.504 | 1.009 |
| oak | leaf | 0.240 | — |
| beech | new shoots | 0.511 | 0.998 |
| beech | leaf | 0.485 | — |

The oak's and beech's nodes hold a few units each, so a node's whole-number cycle is noisy beside an expectation. As frozen, this decides nothing.

## The whole tree beside R2

The law's tree is the structure's own plus the law's cohorts (subtree-1). The leaf area index is over the tree's own leaf-bearing 0.5 m columns: a lone tree's, not a stand's.

| species | measure | cycle | law | R2 checked range | cycle's distance | law's distance | nearer |
|---|---|---|---|---|---|---|---|
| spruce | new shoots a year, weighted | 64,650 (60,550–72,830) | 65,310 | 691 to 2,372 (Bellan 2019) | 27.3× over | 27.5× over | cycle, by 0.7% |
| spruce | new shoots, unweighted | 94,720 | — | the same | — | — | — |
| spruce | leaf area index | 16.65 | 16.77 | 10.5 projected (Bolstad and Gower 1990) | 1.59× | 1.60× | cycle, by 0.7% |
| beech | leaf area index | 1.12 (0.79–1.80) | 1.15 | 5.6 to 9.5 (Leuschner 2006) | 0.200 | 0.205 | law, by 2% |
| oak | leaf area index | 3.96 | 3.65 | none checked | — | — | — |
| oak | new shoots a year, weighted | 21,780 | 21,940 | none checked | — | — | — |

On every checked measure the law and the cycle stand within 2% of each other. Both are far outside the published ranges: the spruce grows about 27 times the measured shoots, and the beech carries a fifth of a beech stand's index. The law is not closer to real trees than the cycle on any checked measure by more than 2%. This is the R3 preview of R5's first half; R4 decides it.

## For R4

The law R4 uses is **subtree-size**: the species' shedding rule at the bearing unit's size. It agrees with the cycle on new shoots, wood and (bar the oak) leaf and section. On the oak it holds wood at 0.991 and leaf at 1.143, flat across light, where size 1 swings from 1.60 to 0.52.
