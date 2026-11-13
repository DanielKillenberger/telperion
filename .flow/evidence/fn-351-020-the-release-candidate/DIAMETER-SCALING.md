# Diameter scaling of the Oregon white oak, by limb-row dominance, 13 Nov 2026

Measurement only: no verdicts. Statistic after Dassot, Fournier, Deleuze 2019 (Ann. For. Sci. 76:66), measured on three Quercus petraea over 200 years: -2.41, -2.37, -2.44 (living branches); -2.56, -2.62, -2.51 (dead included).

## Method as run

- Checkout `/home/daniel/Projects/telperion/.worktrees/census-p5` (detached, 487266f8f plus its uncommitted trial hooks). Reader: `probe/src/diam.rs` in the scratchpad probe crate (`probe diam <preset> <age> <seed> <dominance>`), plus two uncommitted scratch functions in the worktree: `pipeline/space.rs::grow_living` and `pipeline/executor.rs::grow_living` (the pipeline's own grow and convert, also returning the lineage keys of every phytomer of an axis whose apex lives). No expansion, mesh or GPU.
- The tree is the pipeline's grown tree (`space::grow`: engine structure converted to nodes with radii; wood thinner than 0.2 mm and nodes closer than 1 mm are not drawn, irrelevant above 7 cm). The oak's supernatural scale is 1.
- Dominance: the variant is not made by swapping `oregon-white-oak.values` this time. One binary sets `species.states[6].form.dominance` (the `/space/ages/6/form/dominance` row; the preset has none, default 0.0) at run time to 0, 0.25, 0.5 or 0.75. Check: node counts equal the earlier B6 and B3 drawn trees at seed 1001, age 200 (2,385,355 and 3,826,468). The values file was never touched; `git diff` of it is empty.
- Axis: a branch run (`Node::branch`: a lateral starts a run, continuations and relays carry it on). Trunk: the run with id 1, the seed axis and its continuations (164 to 172 nodes in the trees checked). 25 cm samples at arc length 0, 0.25, 0.5 ... m from each run's start (its attachment point; the first edge of the run includes arc 0), along the polyline of node positions. The diameter is 2 x radius, interpolated linearly along each edge between `start_radius` (proximal) and `radius` (distal). The drawn mesh's root flare is not in it.
- Samples with diameter above 7 cm only; classes are 1 cm wide, [k, k+1) cm, centre k+0.5, so the first class is 7 to 8 cm (centre 7.5). Per tree: log10(count) against log10(centre) over the non-empty classes, ordinary least squares; the slope is the exponent. "Largest class" is the centre of the largest non-empty diameter class. "Slope -3 largest" leaves out the three largest non-empty diameter classes (needs more than three classes).
- Living versus dead: the settled structure tells apex state (`apex_end` none = living). A node is living when it or any node beyond it is a phytomer of an axis with a living apex; otherwise it is dead wood (a branch with no living tip beyond it). Wood that has been shed is not in the tree at all, so "dead" is dead and not yet shed. The classification worked: every measured tree has both living and dead nodes (in the table, living nodes / dead nodes, all sizes). The four fits per tree: all axes or all but the trunk, each with living wood only or with dead included.
- Machine: each tree one process through `telperion-heavy` with `TELPERION_HEAVY_RESERVE=8`, 16 threads, one at a time; asked 10, 12 or 16 GB by age; the killed ones (cap exceeded) were rerun at 24 GB.
- Reading the numbers: dead wood over 7 cm is rare at 120 years, so living and with-dead exponents coincide there. The R2 of the per-tree fits is 0.45 to 0.85: the count-by-class lines are not straight, the largest classes are sparse (counts of 1 to a few) and sit in a long tail.

## Not measurable

- Dominance 0, age 300, seed 1001: the grown tree is drawn at zero size on this commit (187 nodes, 7.7 m, no living node); not measurable. The mean of that cell is over seeds 1007 and 1012 only, marked.
- Dominance 0.75, seed 1007, ages 200 and 300: killed at the 12 GB request, again at 24 GB (the cap) at age 200, and at 24 GB at age 300 (the equally sized 0.5 tree at 300 took 16.4 GB; 0.75 needs more than 24). Not measured; those cells are means of seeds 1001 and 1012 only, marked.
- Dominance 0.25, age 300, seed 1001 has only 252,341 nodes (height 30.3 m, fits measured), a far smaller tree than its neighbours; measured and tabulated as it stands.

## Files

- `/home/daniel/.cache/claude-tmp/claude-1000/-home-daniel-Projects-telperion/d6d5cb56-440a-44f4-803a-6a595b8a3532/scratchpad/DIAMETER-HIST.tsv`: raw class histograms (dominance, age, seed, mode, class from/to/centre cm, count); modes all_live, notrunk_live, all_dead (living + dead), notrunk_dead.
- `/home/daniel/.cache/claude-tmp/claude-1000/-home-daniel-Projects-telperion/d6d5cb56-440a-44f4-803a-6a595b8a3532/scratchpad/DIAMETER-STATS.tsv`: the per-tree, per-mode statistics (samples, classes, largest class, R2, slope, slope without the three largest classes).
- Scripts: `probe/src/diam.rs`, `diam_tables.py`, `drun.sh`, `dall.sh`; logs in `logs/diam/`.

## Table 1: all trees

Slopes are the exponents; "live" is living wood only, "with dead" living plus dead. The left block of statistics is living wood, all axes; the right block is living wood without the trunk.

| dom | age | seed | nodes | height m | live nodes / dead nodes | slope live all | slope live no trunk | slope with dead all | slope with dead no trunk | n>7cm (live all) | classes | largest cm | R2 | slope -3 largest | n (live no trunk) | classes | largest cm | R2 | slope -3 largest |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 0 | 120 | 1001 | 2593693 | 27.1 | 1151197 / 1442495 | -2.12 | -2.78 | -2.12 | -2.78 | 2048 | 51 | 107.5 | 0.608 | -2.57 | 1949 | 41 | 54.5 | 0.699 | -2.97 |
| 0 | 120 | 1007 | 3660852 | 26.9 | 1655773 / 2005078 | -2.28 | -2.70 | -2.28 | -2.70 | 4078 | 59 | 138.5 | 0.713 | -2.67 | 3991 | 51 | 75.5 | 0.816 | -2.85 |
| 0 | 120 | 1012 | 2993794 | 25.4 | 1367508 / 1626285 | -2.07 | -2.60 | -2.08 | -2.60 | 2574 | 55 | 118.5 | 0.611 | -2.54 | 2486 | 46 | 74.5 | 0.771 | -2.83 |
| 0 | 200 | 1001 | 818736 | 32.6 | 341763 / 476972 | -1.85 | -2.36 | -1.91 | -2.42 | 1945 | 61 | 133.5 | 0.581 | -2.18 | 1878 | 53 | 77.5 | 0.718 | -2.53 |
| 0 | 200 | 1007 | 1559056 | 30.9 | 717159 / 841896 | -1.71 | -2.10 | -1.92 | -2.30 | 2077 | 63 | 166.5 | 0.457 | -2.02 | 2038 | 56 | 72.5 | 0.534 | -2.27 |
| 0 | 200 | 1012 | 1908181 | 31.7 | 889949 / 1018231 | -1.69 | -2.14 | -1.60 | -2.04 | 1788 | 62 | 151.5 | 0.492 | -2.11 | 1670 | 53 | 79.5 | 0.619 | -2.31 |
| 0 | 300 | 1001 | 187 | 7.7 | 0 / 186 | not measurable (drawn at zero size) | | | | | | | | | | | | | | | |
| 0 | 300 | 1007 | 1316345 | 22.2 | 570018 / 746326 | -1.65 | -2.24 | -1.55 | -2.18 | 1147 | 49 | 181.5 | 0.584 | -2.13 | 1124 | 45 | 81.5 | 0.803 | -2.39 |
| 0 | 300 | 1012 | 1719125 | 23.7 | 750403 / 968721 | -1.61 | -1.83 | -1.54 | -1.75 | 1096 | 64 | 170.5 | 0.586 | -1.84 | 1074 | 60 | 94.5 | 0.689 | -1.94 |
| 0.25 | 120 | 1001 | 2960832 | 25.7 | 1371458 / 1589373 | -2.17 | -3.11 | -2.17 | -3.11 | 1611 | 47 | 102.5 | 0.712 | -2.58 | 1505 | 34 | 47.5 | 0.847 | -3.24 |
| 0.25 | 120 | 1007 | 4978701 | 24.6 | 2385060 / 2593640 | -2.24 | -2.93 | -2.24 | -2.93 | 3201 | 55 | 133.5 | 0.748 | -2.63 | 3096 | 43 | 61.5 | 0.853 | -3.07 |
| 0.25 | 120 | 1012 | 3113719 | 25.1 | 1453144 / 1660574 | -2.06 | -2.60 | -2.06 | -2.60 | 2374 | 50 | 113.5 | 0.640 | -2.45 | 2278 | 41 | 69.5 | 0.791 | -2.81 |
| 0.25 | 200 | 1001 | 2385355 | 27.8 | 1031061 / 1354293 | -2.16 | -2.45 | -2.08 | -2.45 | 2776 | 63 | 141.5 | 0.636 | -2.45 | 2755 | 60 | 83.5 | 0.714 | -2.61 |
| 0.25 | 200 | 1007 | 4412265 | 30.7 | 2034115 / 2378149 | -1.96 | -2.36 | -2.00 | -2.38 | 3771 | 70 | 183.5 | 0.706 | -2.28 | 3736 | 62 | 88.5 | 0.808 | -2.44 |
| 0.25 | 200 | 1012 | 2584776 | 30.2 | 1182921 / 1401854 | -1.95 | -2.37 | -1.93 | -2.30 | 2174 | 66 | 152.5 | 0.525 | -2.40 | 2137 | 58 | 79.5 | 0.630 | -2.59 |
| 0.25 | 300 | 1001 | 252341 | 30.3 | 110271 / 142069 | -1.38 | -1.63 | -1.33 | -1.64 | 824 | 60 | 150.5 | 0.473 | -1.62 | 803 | 56 | 90.5 | 0.560 | -1.76 |
| 0.25 | 300 | 1007 | 3074844 | 29.2 | 1398222 / 1676621 | -1.42 | -1.67 | -1.54 | -1.70 | 1755 | 73 | 209.5 | 0.588 | -1.64 | 1732 | 69 | 105.5 | 0.675 | -1.74 |
| 0.25 | 300 | 1012 | 1989779 | 28.1 | 881494 / 1108284 | -1.56 | -1.85 | -1.47 | -1.74 | 1560 | 74 | 176.5 | 0.542 | -1.84 | 1538 | 70 | 89.5 | 0.653 | -1.95 |
| 0.5 | 120 | 1001 | 2839584 | 26.7 | 1389840 / 1449743 | -2.00 | -3.22 | -2.00 | -3.22 | 1220 | 43 | 93.5 | 0.653 | -2.36 | 1111 | 28 | 39.5 | 0.807 | -3.17 |
| 0.5 | 120 | 1007 | 5197632 | 25.4 | 2658857 / 2538774 | -2.11 | -3.03 | -2.11 | -3.03 | 2083 | 47 | 116.5 | 0.728 | -2.43 | 1971 | 36 | 56.5 | 0.854 | -3.30 |
| 0.5 | 120 | 1012 | 3291405 | 25.5 | 1559460 / 1731944 | -2.05 | -2.71 | -2.05 | -2.72 | 2130 | 52 | 109.5 | 0.633 | -2.48 | 2032 | 37 | 64.5 | 0.843 | -2.93 |
| 0.5 | 200 | 1001 | 3826468 | 22.9 | 1829493 / 1996974 | -1.93 | -2.34 | -1.85 | -2.35 | 1888 | 62 | 133.5 | 0.591 | -2.31 | 1867 | 58 | 76.5 | 0.715 | -2.43 |
| 0.5 | 200 | 1007 | 6628497 | 28.8 | 3232556 / 3395940 | -1.95 | -2.42 | -1.99 | -2.45 | 3774 | 76 | 173.5 | 0.669 | -2.25 | 3738 | 65 | 82.5 | 0.768 | -2.49 |
| 0.5 | 200 | 1012 | 3382116 | 27.5 | 1585366 / 1796749 | -1.88 | -2.26 | -1.91 | -2.30 | 2625 | 61 | 153.5 | 0.540 | -2.30 | 2597 | 58 | 84.5 | 0.652 | -2.41 |
| 0.5 | 300 | 1001 | 3182287 | 29.8 | 1474293 / 1707993 | -1.80 | -1.96 | -1.81 | -1.98 | 1850 | 79 | 163.5 | 0.687 | -1.96 | 1829 | 76 | 99.5 | 0.764 | -2.00 |
| 0.5 | 300 | 1007 | 5494795 | 28.2 | 2704248 / 2790546 | -1.77 | -2.06 | -1.81 | -2.10 | 3278 | 83 | 213.5 | 0.700 | -2.02 | 3254 | 78 | 101.5 | 0.790 | -2.10 |
| 0.5 | 300 | 1012 | 2552802 | 27.7 | 1165567 / 1387234 | -1.54 | -1.80 | -1.53 | -1.82 | 2103 | 79 | 179.5 | 0.502 | -1.79 | 2081 | 75 | 87.5 | 0.590 | -1.86 |
| 0.75 | 120 | 1001 | 2106878 | 26.7 | 1021377 / 1085500 | -1.88 | -2.63 | -1.88 | -2.63 | 934 | 37 | 81.5 | 0.677 | -2.16 | 824 | 21 | 28.5 | 0.593 | -1.35 |
| 0.75 | 120 | 1007 | 3715440 | 24.8 | 1937017 / 1778422 | -1.86 | -2.90 | -1.86 | -2.90 | 1187 | 41 | 92.5 | 0.716 | -2.14 | 1075 | 28 | 48.5 | 0.794 | -3.20 |
| 0.75 | 120 | 1012 | 3270391 | 25.7 | 1579947 / 1690443 | -2.07 | -2.87 | -2.07 | -2.87 | 1890 | 51 | 104.5 | 0.572 | -2.52 | 1792 | 36 | 59.5 | 0.752 | -3.32 |
| 0.75 | 200 | 1001 | 3232344 | 17.6 | 1642431 / 1589912 | -1.71 | -2.22 | -1.56 | -2.18 | 838 | 44 | 109.5 | 0.545 | -2.16 | 817 | 40 | 66.5 | 0.707 | -2.37 |
| 0.75 | 200 | 1007 | not run | | | | | | | | | | | | | | | | |
| 0.75 | 200 | 1012 | 3357368 | 28.9 | 1618747 / 1738620 | -1.99 | -2.39 | -2.02 | -2.42 | 2732 | 65 | 148.5 | 0.641 | -2.41 | 2704 | 61 | 88.5 | 0.764 | -2.57 |
| 0.75 | 300 | 1001 | 4186814 | 25.6 | 2096088 / 2090725 | -2.11 | -2.29 | -2.07 | -2.29 | 2320 | 70 | 147.5 | 0.707 | -2.32 | 2299 | 68 | 103.5 | 0.791 | -2.35 |
| 0.75 | 300 | 1007 | not run | | | | | | | | | | | | | | | | |
| 0.75 | 300 | 1012 | 3612311 | 25.5 | 1760646 / 1851664 | -1.44 | -1.71 | -1.47 | -1.75 | 1812 | 78 | 176.5 | 0.452 | -1.70 | 1790 | 74 | 92.5 | 0.546 | -1.76 |


## Table 2: mean exponent over the three seeds (per-seed values in brackets)

Paper: living -2.41, -2.37, -2.44; with dead -2.56, -2.62, -2.51.

| dominance | age | live, all axes | live, without trunk | with dead, all axes | with dead, without trunk |
|---|---|---|---|---|---|
| 0 | 120 | -2.16  [-2.12, -2.28, -2.07] | -2.69  [-2.78, -2.70, -2.60] | -2.16  [-2.12, -2.28, -2.08] | -2.69  [-2.78, -2.70, -2.60] |
| 0 | 200 | -1.75  [-1.85, -1.71, -1.69] | -2.20  [-2.36, -2.10, -2.14] | -1.81  [-1.91, -1.92, -1.60] | -2.25  [-2.42, -2.30, -2.04] |
| 0 | 300 | -1.63 (2 of 3 seeds) | -2.04 (2 of 3 seeds) | -1.55 (2 of 3 seeds) | -1.96 (2 of 3 seeds) |
| 0.25 | 120 | -2.16  [-2.17, -2.24, -2.06] | -2.88  [-3.11, -2.93, -2.60] | -2.16  [-2.17, -2.24, -2.06] | -2.88  [-3.11, -2.93, -2.60] |
| 0.25 | 200 | -2.02  [-2.16, -1.96, -1.95] | -2.39  [-2.45, -2.36, -2.37] | -2.01  [-2.08, -2.00, -1.93] | -2.38  [-2.45, -2.38, -2.30] |
| 0.25 | 300 | -1.46  [-1.38, -1.42, -1.56] | -1.72  [-1.63, -1.67, -1.85] | -1.45  [-1.33, -1.54, -1.47] | -1.69  [-1.64, -1.70, -1.74] |
| 0.5 | 120 | -2.05  [-2.00, -2.11, -2.05] | -2.99  [-3.22, -3.03, -2.71] | -2.05  [-2.00, -2.11, -2.05] | -2.99  [-3.22, -3.03, -2.72] |
| 0.5 | 200 | -1.92  [-1.93, -1.95, -1.88] | -2.34  [-2.34, -2.42, -2.26] | -1.91  [-1.85, -1.99, -1.91] | -2.37  [-2.35, -2.45, -2.30] |
| 0.5 | 300 | -1.70  [-1.80, -1.77, -1.54] | -1.94  [-1.96, -2.06, -1.80] | -1.71  [-1.81, -1.81, -1.53] | -1.96  [-1.98, -2.10, -1.82] |
| 0.75 | 120 | -1.94  [-1.88, -1.86, -2.07] | -2.80  [-2.63, -2.90, -2.87] | -1.94  [-1.88, -1.86, -2.07] | -2.80  [-2.63, -2.90, -2.87] |
| 0.75 | 200 | -1.85 (2 of 3 seeds) | -2.30 (2 of 3 seeds) | -1.79 (2 of 3 seeds) | -2.30 (2 of 3 seeds) |
| 0.75 | 300 | -1.77 (2 of 3 seeds) | -2.00 (2 of 3 seeds) | -1.77 (2 of 3 seeds) | -2.02 (2 of 3 seeds) |

