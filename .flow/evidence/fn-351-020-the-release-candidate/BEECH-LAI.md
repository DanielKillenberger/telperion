# European beech: crown, leaf and leaf area index at ages 40, 60, 80

Method. The scoreboard's own probe (`.worktrees/fn-362/.flow/evidence/fn-352-the-scoreboard-a-fixed-panel-of-trees/scoreboard/probe`, with its `peak.py`), command `probe tree european-beech <age> <seed> <prefix> none`. Shots `none`: measures only, no stills, **no GPU used**. Each tree is its own process, one at a time, through `telperion-heavy 8 --threads 4` (16 GB for head at 80), no kills or reruns (every log ends `exit 0`). The probe source is fn-362's copy, built unchanged against each commit's crates into that worktree's own `target/` (`ci` profile, not `run.sh`, so no file under `raw/` or tracked file was touched). Raw logs: `scratchpad/beechlai/head/` and `scratchpad/beechlai/base/`.

Commits
- head: `810e90bf3e33ab0b58e0c46e7245295084d7081e`, branch `fn-362-a-runs-rings-have-the-sides-its-widest`, worktree `.worktrees/fn-362` (status as found: three untracked files, unchanged).
- base: `5ebab2de05cf30cd8a63c7b4c4b082375184e3a3`, the base GATE-FAILURES.md names, `.worktrees/gate-base`, detached and clean.

How each column is measured (probe's own definitions)
- height: drawn mesh bounds, max y minus min y. Crown x / z: the mesh bounds' full width on the x and z axes (the whole drawn tree, not only the leafed crown).
- hull m2: crown projection as the tool gives it: the convex hull on the ground (x, z) of every leaf instance's position. ellipse m2: pi/4 x crown x x crown z, from the two widths (computed by me, shown for comparison).
- dbh: trunk diameter at 1.3 m on the drawn tree (thickest child path from the root; the mesh's root flare is not in it).
- leaf area: one-sided blade area of the drawn leaf instances, every triangle of the blade element counted once. leaves: instance count.
- LAI hull = leaf area / hull (the tool's own `lai`). LAI ellipse = leaf area / ellipse.
- phytomers: phytomers of the engine's structure from a second grow (`probe/src/engine.rs`); nodes: nodes of the drawn tree. "Held" is not a column the tool has; phytomers is the engine's total, nodes the drawn count.
- peak GiB: `peak.py`'s peak resident memory of the whole probe process (`RUSAGE_CHILDREN` max RSS).

## Per tree

| commit | age | seed | height m | crown x m | crown z m | hull m2 | ellipse m2 | dbh m | leaf area m2 | leaves | LAI hull | LAI ellipse | phytomers | nodes | peak GiB |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| head | 40 | 1 | 10.04 | 6.78 | 6.25 | 27.5 | 33.3 | 0.201 | 313 | 92,245 | 11.39 | 9.40 | 485,877 | 442,420 | 0.36 |
| head | 40 | 7 | 10.47 | 7.95 | 8.91 | 49.4 | 55.6 | 0.310 | 608 | 179,354 | 12.31 | 10.93 | 796,872 | 775,112 | 0.66 |
| head | 40 | 1001 | 9.54 | 6.00 | 7.08 | 31.1 | 33.3 | 0.168 | 183 | 54,023 | 5.89 | 5.49 | 325,739 | 260,577 | 0.25 |
| head | 40 | 1002 | 9.09 | 6.67 | 8.35 | 37.7 | 43.7 | 0.222 | 326 | 96,222 | 8.64 | 7.46 | 526,702 | 430,181 | 0.39 |
| head | 40 | 1003 | 10.76 | 8.06 | 8.10 | 42.5 | 51.3 | 0.252 | 426 | 125,677 | 10.03 | 8.31 | 599,102 | 558,253 | 0.48 |
| head | 60 | 1 | 14.25 | 11.82 | 12.03 | 108.9 | 111.6 | 0.480 | 1,408 | 415,316 | 12.93 | 12.61 | 2,273,256 | 2,159,610 | 1.73 |
| head | 60 | 7 | 14.59 | 14.66 | 15.59 | 168.0 | 179.5 | 0.612 | 2,113 | 622,969 | 12.57 | 11.77 | 3,318,366 | 3,214,656 | 2.55 |
| head | 60 | 1001 | 13.38 | 13.15 | 13.87 | 122.3 | 143.3 | 0.453 | 1,316 | 388,008 | 10.75 | 9.18 | 2,068,021 | 1,913,247 | 1.53 |
| head | 60 | 1002 | 13.73 | 13.48 | 13.54 | 134.6 | 143.3 | 0.490 | 1,368 | 403,338 | 10.16 | 9.55 | 2,368,644 | 2,101,726 | 1.72 |
| head | 60 | 1003 | 15.16 | 16.33 | 14.40 | 147.0 | 184.6 | 0.571 | 1,875 | 552,910 | 12.76 | 10.16 | 2,902,689 | 2,793,105 | 2.22 |
| head | 80 | 1 | 17.59 | 18.01 | 18.32 | 251.9 | 259.1 | 0.749 | 2,957 | 871,760 | 11.74 | 11.41 | 5,037,729 | 4,821,905 | 3.81 |
| head | 80 | 7 | 18.55 | 22.23 | 21.26 | 341.2 | 371.1 | 0.875 | 3,741 | 1,103,506 | 10.96 | 10.08 | 6,170,796 | 5,953,421 | 4.73 |
| head | 80 | 1001 | 17.16 | 20.21 | 20.46 | 298.9 | 324.8 | 0.771 | 3,316 | 977,439 | 11.09 | 10.21 | 5,553,773 | 5,268,004 | 4.15 |
| head | 80 | 1002 | 17.57 | 20.18 | 19.36 | 295.9 | 307.0 | 0.856 | 3,835 | 1,131,241 | 12.96 | 12.49 | 6,148,949 | 5,664,089 | 4.58 |
| head | 80 | 1003 | 19.00 | 21.34 | 20.63 | 305.0 | 345.7 | 0.844 | 3,534 | 1,042,235 | 11.59 | 10.22 | 5,799,929 | 5,568,218 | 4.44 |
| base | 40 | 1 | 10.11 | 6.38 | 5.32 | 22.7 | 26.7 | 0.293 | 113 | 33,427 | 4.99 | 4.24 | 134,648 | 114,700 | 0.16 |
| base | 40 | 7 | 10.52 | 6.97 | 8.79 | 42.6 | 48.1 | 0.490 | 227 | 66,854 | 5.32 | 4.71 | 219,833 | 205,400 | 0.28 |
| base | 40 | 1001 | 9.78 | 5.08 | 7.02 | 24.4 | 28.0 | 0.256 | 56 | 16,614 | 2.31 | 2.01 | 83,359 | 57,904 | 0.09 |
| base | 40 | 1002 | 9.16 | 6.17 | 8.20 | 36.4 | 39.7 | 0.356 | 131 | 38,707 | 3.61 | 3.30 | 159,257 | 124,206 | 0.20 |
| base | 40 | 1003 | 10.77 | 7.87 | 8.10 | 38.7 | 50.1 | 0.379 | 158 | 46,714 | 4.09 | 3.16 | 179,060 | 152,558 | 0.22 |
| base | 60 | 1 | 14.30 | 12.42 | 13.59 | 121.9 | 132.6 | 0.786 | 625 | 184,218 | 5.12 | 4.71 | 734,725 | 655,968 | 0.88 |
| base | 60 | 7 | 14.66 | 14.51 | 15.17 | 160.3 | 172.9 | 1.128 | 1,069 | 315,364 | 6.67 | 6.19 | 1,229,702 | 1,100,016 | 1.42 |
| base | 60 | 1001 | 13.45 | 12.96 | 13.55 | 118.0 | 137.9 | 0.696 | 531 | 156,651 | 4.50 | 3.85 | 619,551 | 532,291 | 0.73 |
| base | 60 | 1002 | 13.76 | 13.45 | 13.24 | 131.0 | 139.9 | 0.791 | 578 | 170,351 | 4.41 | 4.13 | 767,186 | 595,095 | 0.84 |
| base | 60 | 1003 | 15.18 | 16.06 | 14.22 | 139.2 | 179.4 | 0.956 | 792 | 233,481 | 5.69 | 4.41 | 866,447 | 805,012 | 1.02 |
| base | 80 | 1 | 17.63 | 17.75 | 19.94 | 257.0 | 277.9 | 1.333 | 1,526 | 449,923 | 5.94 | 5.49 | 1,779,205 | 1,625,652 | 2.07 |
| base | 80 | 7 | 18.62 | 21.77 | 20.70 | 326.5 | 354.0 | 1.686 | 2,148 | 633,405 | 6.58 | 6.07 | 2,491,132 | 2,249,902 | 2.94 |
| base | 80 | 1001 | 17.36 | 19.61 | 20.16 | 288.6 | 310.6 | 1.302 | 1,573 | 463,750 | 5.45 | 5.06 | 1,887,629 | 1,671,245 | 2.14 |
| base | 80 | 1002 | 17.69 | 20.07 | 18.39 | 276.7 | 290.0 | 1.504 | 1,831 | 540,127 | 6.62 | 6.31 | 2,185,243 | 1,877,755 | 2.43 |
| base | 80 | 1003 | 19.14 | 20.64 | 19.35 | 285.2 | 313.8 | 1.562 | 1,801 | 531,293 | 6.32 | 5.74 | 2,024,865 | 1,890,930 | 2.37 |

All 30 trees fit; none was refused or killed. Largest: head age 80 seed 7, 4.73 GiB peak.

## Medians (five seeds per cell)

| commit | age | height | crown x | crown z | hull m2 | ellipse m2 | dbh | leaf area | leaves | LAI hull | LAI ellipse | phytomers | nodes | peak GiB |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| head | 40 | 10.04 | 6.78 | 8.10 | 37.7 | 43.7 | 0.222 | 326 | 96,222 | 10.03 | 8.31 | 526,702 | 442,420 | 0.39 |
| base | 40 | 10.11 | 6.38 | 8.10 | 36.4 | 39.7 | 0.356 | 131 | 38,707 | 4.09 | 3.30 | 159,257 | 124,206 | 0.20 |
| head | 60 | 14.25 | 13.48 | 13.87 | 134.6 | 143.3 | 0.490 | 1,408 | 415,316 | 12.57 | 10.16 | 2,368,644 | 2,159,610 | 1.73 |
| base | 60 | 14.30 | 13.45 | 13.59 | 131.0 | 139.9 | 0.791 | 625 | 184,218 | 5.12 | 4.41 | 767,186 | 655,968 | 0.88 |
| head | 80 | 17.59 | 20.21 | 20.46 | 298.9 | 324.8 | 0.844 | 3,534 | 1,042,235 | 11.59 | 10.22 | 5,799,929 | 5,568,218 | 4.44 |
| base | 80 | 17.69 | 20.07 | 19.94 | 285.2 | 310.6 | 1.504 | 1,801 | 531,293 | 6.32 | 5.74 | 2,024,865 | 1,877,755 | 2.37 |

(Median is taken per column independently, so a row's cells may come from different seeds.)

## Unknown or not exposed

- The grower's own leaf area is not exposed by the tool (the probe reads the drawn leaf instances only). Not measured.
- "Held phytomers" is not a distinct field; phytomers (engine structure, second grow) and nodes (drawn tree) are given instead.
- Crown width is the whole drawn tree's bounds, not a leafed-crown width; the crown projection area is the convex hull of leaf positions (so it ignores concavity and the bare lower trunk's extent). Leaf area counts one side of each blade; the probe does not say whether the blade element is one leaf or a cluster.
- Leaf area index is drawn leaf over projection, a single-sided total, not an allometric or LAI-2000-type measurement. Which published range it is read against is the host's call; ranges.py lists beech "closed mature stands, one-sided, 7.4 (5.6 to 9.5)" (Leuschner et al. 2006) for the panel.
- Peak memory is the whole probe process (draw plus the engine's second grow), not the pipeline alone.
- Seeds 1 and 7 are not among the scoreboard's usual 1001 to 1012; no earlier row exists for them.
