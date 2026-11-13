# Where the beech's wood vertices sit: base against head

Base: 5ebab2de0 (worktree gate-base). Head: 1958c708c, the head of fn-360-a-runs-girth-eases-wherever-it-drops (worktree gate-bisect). European beech preset, seed 1, ages 80 and 40. Every cell reads base / head. One measurement each.

## Method

- Both worktrees carry the same uncommitted probe (a lib test, `pipeline/surface/probe.rs`, plus a 3-line recorder in `pipeline/space.rs`, since removed). It runs `pipeline::build(family, Request::mesh())` (what `mesh::build` runs), then reads the settled tree, the engine's axes (recorded where the pipeline converts the engine's structure to nodes) and the surface stage's runs: it re-samples every swept run with the surface stage's own functions and counts rings, radii and chords. Its vertex total is checked against `SurfaceMesh::vertices()`: equal at all four trees (35,182,102 / 101,808,366 at 80; 2,500,030 / 9,414,878 at 40).
- Peak memory is the process's VmHWM read straight after the build returned (no /usr/bin/time on this machine; the heavy scope's own peak was not read). Gate item 7 recorded 2,094 / 5,826 MiB at 80 by the same measure; here 2,146 / 5,875. The recorded vertex counts were 35.2 M and 101.7 M; this probe reads 35.18 M and 101.81 M.
- Axes and phytomers are the engine's (`Structure.axes`, phytomers kept); "taken" are phytomers that passed the radius cut (drawn or merged into a node); nodes are the drawn ones. Branching order follows the census rule: a lateral is one deeper than its bearer, a continuation or relay keeps its bearer's order. There are no named kinds (trunk, limb, bough) in the data; the tables use order, the tree's node kind (Structural, Branch, Twig), the stem mark, the short-shoot mark (clustered nodes) and the PA index.
- A ring's vertices (20 radial segments in all four trees) go to the node the ring stands on; a run's two cap vertices go to its last ring. Radius classes use the swept ring radius in millimetres (radius, not diameter). Rings per metre divides ring counts by the length each ring covers (half the chord to each neighbour in its run), so socket rings sunk into a bearer count their short chord.
- Drawn height is the highest node; the wood bounds agree. Trunk diameter at breast height is twice the swept radius of the widest trunk run at y = 1.3 m (with flare), and, beside it, twice the solved node radius at that height.
- Leaf area is the sum over retained leaves of (blade area of the leaf element, 2.83 cm2, triangles facing the blade) x scale^2. A leaf's axis is the axis of the nearest node (grid search); no leaf was left unattributed. This is an approximation of "the axis that carries it".

## Beech at 80, seed 1

Cells read base / head.

### 1. Totals

|  | base | head |
|---|---|---|
| axes (engine) | 181,120 | 321,607 |
| phytomers (engine, kept) | 1,779,205 | 5,037,729 |
| phytomers taken as drawn or merged | 1,634,865 | 4,921,417 |
| nodes | 1,625,652 | 4,821,905 |
| wood vertices | 35,182,102 | 101,808,366 |
| wood triangles | 69,878,920 | 202,640,320 |
| rings | 1,746,973 | 5,066,008 |
| wood length (m, along the swept runs) | 20,809 | 48,233 |
| leaves (retained) | 449,923 | 871,760 |
| leaf area (m2, blade area x scale^2) | 1,548.2 | 3,000.3 |
| drawn height (m, highest node) | 17.49 | 17.45 |
| trunk diameter at 1.3 m, drawn (cm) | 135.6 | 76.2 |
| trunk diameter at 1.3 m, solved radius x 2 (cm) | 133.3 | 74.9 |
| peak memory, VmHWM after the build (MiB) | 2,146 | 5,875 |
| radial segments | 20 | 20 |
| runs (swept paths) | 121,321 | 244,103 |

### 2. By branching order (0 the trunk's axis, 1 its laterals, and so on)

Order counts a lateral as one deeper than its bearer; continuations and relays keep their bearer's order (the census rule). Vertices go to the axis that made the node each ring stands on.

| order | axes | axes with a drawn node | nodes | wood vertices | share of vertices |
|---|---|---|---|---|---|
| 0 | 24 / 24 | 14 / 20 | 176 / 240 | 3,796 / 5,184 | 0.0% / 0.0% |
| 1 | 545 / 554 | 521 / 539 | 5,848 / 6,536 | 128,060 / 142,064 | 0.4% / 0.1% |
| 2 | 1,500 / 1,777 | 1,372 / 1,664 | 33,597 / 45,363 | 697,770 / 937,114 | 2.0% / 0.9% |
| 3 | 5,016 / 8,488 | 4,216 / 7,684 | 117,794 / 201,952 | 2,432,632 / 4,183,426 | 6.9% / 4.1% |
| 4 | 22,160 / 40,409 | 17,552 / 35,323 | 286,585 / 651,810 | 6,081,406 / 13,748,782 | 17.3% / 13.5% |
| 5 | 50,118 / 100,248 | 37,832 / 84,761 | 439,782 / 1,451,917 | 9,564,180 / 30,747,558 | 27.2% / 30.2% |
| 6 | 54,587 / 110,056 | 39,507 / 91,155 | 410,809 / 1,624,537 | 8,998,692 / 34,293,210 | 25.6% / 33.7% |
| 7 | 25,804 / 39,731 | 18,198 / 32,275 | 186,231 / 566,235 | 4,086,026 / 11,962,642 | 11.6% / 11.8% |
| 8 | 12,049 / 13,402 | 8,401 / 10,780 | 85,377 / 186,690 | 1,874,960 / 3,947,250 | 5.3% / 3.9% |
| 9 | 5,788 / 4,974 | 3,992 / 3,972 | 38,875 / 64,555 | 857,402 / 1,370,322 | 2.4% / 1.3% |
| 10 | 2,447 / 1,431 | 1,612 / 1,106 | 14,910 / 17,189 | 330,604 / 365,844 | 0.9% / 0.4% |
| 11 | 752 / 416 | 473 / 298 | 4,282 / 4,244 | 95,222 / 90,750 | 0.3% / 0.1% |
| 12 | 241 / 86 | 150 / 63 | 1,214 / 597 | 27,340 / 13,266 | 0.1% / 0.0% |
| 13 | 89 / 11 | 26 / 7 | 172 / 40 | 4,012 / 954 | 0.0% / 0.0% |

By node kind (the tree's own mark: Structural at or above 5% of the root radius, Branch, Twig = fine wood with nothing beyond it), by the node each ring stands on.

| node kind | nodes | wood vertices | share |
|---|---|---|---|
| Structural | 26,928 / 15,986 | 549,380 / 330,588 | 1.6% / 0.3% |
| Branch | 1,477,403 / 4,562,130 | 31,938,000 / 96,095,320 | 90.8% / 94.4% |
| Twig | 121,321 / 243,789 | 2,694,722 / 5,382,458 | 7.7% / 5.3% |

By stem mark and short shoot (node.stem; clustered = a node of an axis of an age that states a leaf cluster).

| wood | nodes | wood vertices | share |
|---|---|---|---|
| stem (trunk) | 153 / 204 | 3,062 / 4,082 | 0.0% / 0.0% |
| other wood | 702,156 / 1,275,440 | 14,783,392 / 27,010,916 | 42.0% / 26.5% |
| short shoot | 923,343 / 3,546,261 | 20,395,648 / 74,793,368 | 58.0% / 73.5% |

By physiological age (the species' PA index of the axis):

| PA | axes | nodes | wood vertices |
|---|---|---|---|
| 2 | 6 / 6 | 36 / 50 | 828 / 1,130 |
| 4 | 17 / 17 | 129 / 179 | 2,746 / 3,832 |
| 5 | 1 / 1 | 11 / 11 | 222 / 222 |
| 6 | 142 / 143 | 2,281 / 2,454 | 48,504 / 52,032 |
| 7 | 1,329 / 1,317 | 14,078 / 14,750 | 307,908 / 321,366 |
| 8 | 332 / 332 | 28,825 / 28,946 | 580,238 / 582,732 |
| 9 | 8,643 / 9,225 | 317,361 / 367,207 | 6,509,626 / 7,525,712 |
| 12 | 33,113 / 69,387 | 339,588 / 862,047 | 7,336,382 / 18,527,972 |
| 14 | 137,537 / 241,179 | 923,343 / 3,546,261 | 20,395,648 / 74,793,368 |

### 3. Wood vertices and length by drawn radius class

Radius is the swept ring's radius (girth with flare and fork swell). Length: each ring takes half the chord to either neighbour in its run. A run's two cap vertices (and the rings a split run repeats) are counted on its last ring.

| radius class | wood vertices | share | length (m) | rings |
|---|---|---|---|---|
| under 1 mm | 9,709,680 / 18,311,932 | 27.6% / 18.0% | 2,266.2 / 3,039.7 | 476,559 / 905,745 |
| 1 to 2 mm | 7,876,142 / 73,529,562 | 22.4% / 72.2% | 3,289.3 / 32,382.2 | 390,682 / 3,662,066 |
| 2 to 5 mm | 10,561,896 / 6,756,406 | 30.0% / 6.6% | 6,185.5 / 8,034.3 | 528,068 / 337,794 |
| 5 to 10 mm | 3,655,184 / 2,436,622 | 10.4% / 2.4% | 3,915.4 / 3,498.4 | 182,738 / 121,770 |
| 10 to 20 mm | 1,936,552 / 458,138 | 5.5% / 0.5% | 2,743.9 / 685.5 | 96,795 / 22,878 |
| 20 to 50 mm | 1,111,064 / 260,452 | 3.2% / 0.3% | 1,723.2 / 475.3 | 55,552 / 13,000 |
| 50 to 100 mm | 240,022 / 44,404 | 0.7% / 0.0% | 466.6 / 88.2 | 12,001 / 2,214 |
| over 100 mm | 91,562 / 10,850 | 0.3% / 0.0% | 219.4 / 29.3 | 4,578 / 541 |
| all | 35,182,102 / 101,808,366 | 100% | 20,809.4 / 48,233.0 | 1,746,973 / 5,066,008 |

### 4. Rings per metre and vertices per ring

| radius class | rings per metre | vertices per ring |
|---|---|---|
| under 1 mm | 210.3 / 298.0 | 20.37 / 20.22 |
| 1 to 2 mm | 118.8 / 113.1 | 20.16 / 20.08 |
| 2 to 5 mm | 85.4 / 42.0 | 20.00 / 20.00 |
| 5 to 10 mm | 46.7 / 34.8 | 20.00 / 20.01 |
| 10 to 20 mm | 35.3 / 33.4 | 20.01 / 20.03 |
| 20 to 50 mm | 32.2 / 27.4 | 20.00 / 20.03 |
| 50 to 100 mm | 25.7 / 25.1 | 20.00 / 20.06 |
| over 100 mm | 20.9 / 18.5 | 20.00 / 20.06 |
| all | 84.0 / 105.0 | 20.14 / 20.10 |

### 5. Axes by length class

Axis length is the sum of the chords of the nodes the axis made (as drawn). Leaf area goes to the axis of the node nearest each leaf (grid search, 0.9 m limit; unattributed leaves are listed below the table).

| axis length | axes | nodes | wood vertices | share of vertices | leaf area (m2) | share of leaf area |
|---|---|---|---|---|---|---|
| under 2 cm | 24,174 / 27,159 | 114,553 / 133,896 | 2,766,374 / 3,212,690 | 7.9% / 3.2% | 323.6 / 357.8 | 20.9% / 11.9% |
| 2 to 5 cm | 35,559 / 49,514 | 279,020 / 432,587 | 6,291,932 / 9,629,840 | 17.9% / 9.5% | 466.3 / 652.4 | 30.1% / 21.7% |
| 5 to 20 cm | 51,639 / 119,062 | 662,621 / 2,239,095 | 14,287,704 / 47,144,626 | 40.6% / 46.3% | 602.3 / 1,473.2 | 38.9% / 49.1% |
| 20 cm to 1 m | 18,878 / 69,311 | 351,834 / 1,738,880 | 7,411,448 / 36,179,928 | 21.1% / 35.5% | 117.9 / 479.9 | 7.6% / 16.0% |
| over 1 m | 3,616 / 4,601 | 217,624 / 277,447 | 4,424,644 / 5,641,282 | 12.6% / 5.5% | 38.1 / 37.0 | 2.5% / 1.2% |
| no drawn node | 47,254 / 51,960 | 0 / 0 | 0 / 0 | 0.0% / 0.0% | 0.0 / 0.0 | 0.0% / 0.0% |

Leaves not attributed to an axis (no node within 0.9 m): 0 / 0.

## Beech at 40, seed 1

Cells read base / head.

### 1. Totals

|  | base | head |
|---|---|---|
| axes (engine) | 15,171 | 35,541 |
| phytomers (engine, kept) | 134,648 | 485,877 |
| phytomers taken as drawn or merged | 115,483 | 460,132 |
| nodes | 114,700 | 442,420 |
| wood vertices | 2,500,030 | 9,414,878 |
| wood triangles | 4,962,600 | 18,726,760 |
| rings | 124,065 | 468,169 |
| wood length (m, along the swept runs) | 1,049 | 3,385 |
| leaves (retained) | 33,427 | 92,245 |
| leaf area (m2, blade area x scale^2) | 114.9 | 317.3 |
| drawn height (m, highest node) | 9.99 | 9.99 |
| trunk diameter at 1.3 m, drawn (cm) | 29.4 | 20.1 |
| trunk diameter at 1.3 m, solved radius x 2 (cm) | 29.3 | 20.1 |
| peak memory, VmHWM after the build (MiB) | 171 | 576 |
| radial segments | 20 | 20 |
| runs (swept paths) | 9,365 | 25,749 |

### 2. By branching order (0 the trunk's axis, 1 its laterals, and so on)

Order counts a lateral as one deeper than its bearer; continuations and relays keep their bearer's order (the census rule). Vertices go to the axis that made the node each ring stands on.

| order | axes | axes with a drawn node | nodes | wood vertices | share of vertices |
|---|---|---|---|---|---|
| 0 | 21 / 21 | 21 / 21 | 242 / 259 | 5,270 / 5,606 | 0.2% / 0.1% |
| 1 | 120 / 150 | 111 / 140 | 2,189 / 3,355 | 45,796 / 69,560 | 1.8% / 0.7% |
| 2 | 405 / 809 | 348 / 743 | 8,411 / 17,156 | 174,752 / 357,578 | 7.0% / 3.8% |
| 3 | 1,879 / 4,511 | 1,394 / 3,916 | 21,410 / 70,598 | 456,772 / 1,491,988 | 18.3% / 15.8% |
| 4 | 4,634 / 11,387 | 3,122 / 9,285 | 34,444 / 144,775 | 753,124 / 3,083,632 | 30.1% / 32.8% |
| 5 | 5,119 / 12,331 | 3,308 / 9,487 | 31,242 / 142,758 | 691,386 / 3,044,258 | 27.7% / 32.3% |
| 6 | 2,266 / 5,001 | 1,381 / 3,656 | 12,488 / 49,317 | 277,624 / 1,059,524 | 11.1% / 11.3% |
| 7 | 540 / 1,079 | 368 / 778 | 3,300 / 11,974 | 73,492 / 254,884 | 2.9% / 2.7% |
| 8 | 182 / 246 | 115 / 162 | 967 / 2,220 | 21,630 / 47,644 | 0.9% / 0.5% |
| 9 | 5 / 6 | 2 / 2 | 7 / 8 | 184 / 204 | 0.0% / 0.0% |

By node kind (the tree's own mark: Structural at or above 5% of the root radius, Branch, Twig = fine wood with nothing beyond it), by the node each ring stands on.

| node kind | nodes | wood vertices | share |
|---|---|---|---|
| Structural | 7,795 / 8,182 | 163,618 / 170,302 | 6.5% / 1.8% |
| Branch | 97,559 / 408,590 | 2,128,620 / 8,677,560 | 85.1% / 92.2% |
| Twig | 9,346 / 25,648 | 207,792 / 567,016 | 8.3% / 6.0% |

By stem mark and short shoot (node.stem; clustered = a node of an axis of an age that states a leaf cluster).

| wood | nodes | wood vertices | share |
|---|---|---|---|
| stem (trunk) | 189 / 203 | 3,782 / 4,062 | 0.2% / 0.0% |
| other wood | 48,324 / 115,964 | 1,025,842 / 2,476,408 | 41.0% / 26.3% |
| short shoot | 66,187 / 326,253 | 1,470,406 / 6,934,408 | 58.8% / 73.7% |

By physiological age (the species' PA index of the axis):

| PA | axes | nodes | wood vertices |
|---|---|---|---|
| 2 | 6 / 6 | 61 / 66 | 1,350 / 1,446 |
| 4 | 15 / 15 | 181 / 193 | 3,920 / 4,160 |
| 6 | 33 / 34 | 476 / 484 | 10,184 / 10,344 |
| 7 | 122 / 122 | 1,635 / 1,686 | 34,664 / 35,774 |
| 8 | 36 / 36 | 1,421 / 1,515 | 28,942 / 30,848 |
| 9 | 781 / 995 | 21,419 / 29,929 | 442,028 / 616,930 |
| 12 | 2,697 / 7,526 | 23,320 / 82,294 | 508,536 / 1,780,968 |
| 14 | 11,481 / 26,807 | 66,187 / 326,253 | 1,470,406 / 6,934,408 |

### 3. Wood vertices and length by drawn radius class

Radius is the swept ring's radius (girth with flare and fork swell). Length: each ring takes half the chord to either neighbour in its run. A run's two cap vertices (and the rings a split run repeats) are counted on its last ring.

| radius class | wood vertices | share | length (m) | rings |
|---|---|---|---|---|
| under 1 mm | 1,073,510 / 4,293,354 | 42.9% / 45.6% | 184.3 / 743.0 | 52,854 / 212,766 |
| 1 to 2 mm | 584,730 / 4,525,872 | 23.4% / 48.1% | 213.4 / 1,988.8 | 29,127 / 225,636 |
| 2 to 5 mm | 557,166 / 419,990 | 22.3% / 4.5% | 314.7 / 415.2 | 27,856 / 20,994 |
| 5 to 10 mm | 174,594 / 137,614 | 7.0% / 1.5% | 178.1 / 180.1 | 8,728 / 6,874 |
| 10 to 20 mm | 70,510 / 25,498 | 2.8% / 0.3% | 93.6 / 36.1 | 3,524 / 1,273 |
| 20 to 50 mm | 34,120 / 8,204 | 1.4% / 0.1% | 51.8 / 12.7 | 1,706 / 409 |
| 50 to 100 mm | 3,300 / 3,144 | 0.1% / 0.0% | 6.6 / 5.6 | 165 / 157 |
| over 100 mm | 2,100 / 1,202 | 0.1% / 0.0% | 6.5 / 3.6 | 105 / 60 |
| all | 2,500,030 / 9,414,878 | 100% | 1,048.9 / 3,385.0 | 124,065 / 468,169 |

### 4. Rings per metre and vertices per ring

| radius class | rings per metre | vertices per ring |
|---|---|---|
| under 1 mm | 286.8 / 286.4 | 20.31 / 20.18 |
| 1 to 2 mm | 136.5 / 113.5 | 20.08 / 20.06 |
| 2 to 5 mm | 88.5 / 50.6 | 20.00 / 20.01 |
| 5 to 10 mm | 49.0 / 38.2 | 20.00 / 20.02 |
| 10 to 20 mm | 37.6 / 35.3 | 20.01 / 20.03 |
| 20 to 50 mm | 33.0 / 32.3 | 20.00 / 20.06 |
| 50 to 100 mm | 25.0 / 27.9 | 20.00 / 20.03 |
| over 100 mm | 16.2 / 16.8 | 20.00 / 20.03 |
| all | 118.3 / 138.3 | 20.15 / 20.11 |

### 5. Axes by length class

Axis length is the sum of the chords of the nodes the axis made (as drawn). Leaf area goes to the axis of the node nearest each leaf (grid search, 0.9 m limit; unattributed leaves are listed below the table).

| axis length | axes | nodes | wood vertices | share of vertices | leaf area (m2) | share of leaf area |
|---|---|---|---|---|---|---|
| under 2 cm | 2,932 / 5,157 | 14,509 / 26,737 | 348,802 / 636,128 | 14.0% / 6.8% | 38.1 / 67.8 | 33.2% / 21.4% |
| 2 to 5 cm | 3,371 / 6,937 | 34,345 / 75,652 | 754,990 / 1,650,298 | 30.2% / 17.5% | 42.5 / 88.2 | 37.0% / 27.8% |
| 5 to 20 cm | 2,668 / 11,616 | 34,649 / 228,671 | 747,956 / 4,809,104 | 29.9% / 51.1% | 24.8 / 130.0 | 21.6% / 41.0% |
| 20 cm to 1 m | 1,067 / 4,244 | 23,896 / 97,998 | 499,550 / 2,047,348 | 20.0% / 21.7% | 7.7 / 28.4 | 6.7% / 8.9% |
| over 1 m | 132 / 236 | 7,301 / 13,362 | 148,732 / 272,000 | 5.9% / 2.9% | 1.7 / 3.0 | 1.5% / 0.9% |
| no drawn node | 5,001 / 7,351 | 0 / 0 | 0 / 0 | 0.0% / 0.0% | 0.0 / 0.0 | 0.0% / 0.0% |

Leaves not attributed to an axis (no node within 0.9 m): 0 / 0.


## Closing paragraph

At 80 the head holds 101.8 M wood vertices against 35.2 M on the base, 66.6 M more, with 5.07 M rings against 1.75 M (nodes 4.82 M against 1.63 M, engine axes 322 k against 181 k, phytomers kept 5.04 M against 1.78 M). The added vertices sit in rings of drawn radius 1 to 2 mm (73.5 M against 7.9 M, 72% of the head's wood) and under 1 mm (18.3 M against 9.7 M); every class from 2 mm up holds fewer vertices on the head than on the base. By order the added vertices are at orders 4 to 7 (orders 5 and 6 alone 65.0 M against 18.6 M), by node kind in Branch nodes (96.1 M against 31.9 M), by mark in short-shoot nodes (74.8 M against 20.4 M, PA 14) and in PA 12 (18.5 M against 7.3 M); by axis length in axes of 5 to 20 cm (47.1 M against 14.3 M) and 20 cm to 1 m (36.2 M against 7.4 M). Vertices per ring are 20.1 in both trees and in every class (20.0 to 20.4), so the ring is cut the same way; rings per metre are 84 against 105 over all wood, and by class are 298 against 210 under 1 mm, 113 against 119 at 1 to 2 mm, and lower on the head in the classes from 2 mm up (42 against 85 at 2 to 5 mm); the total length of swept wood is 48,233 m against 20,809 m, and the length in the 1 to 2 mm class is 32,382 m against 3,289 m. The trunk at 1.3 m is 76 cm across on the head against 136 cm on the base at the same drawn height (17.5 m). The tree at 40 shows the same pattern at a smaller size: 9.41 M vertices against 2.50 M, 4.53 M of them in the 1 to 2 mm class against 0.58 M and 4.29 M under 1 mm against 1.07 M, vertices per ring 20.1 against 20.15, rings per metre in those two classes 286 against 287 under 1 mm and 114 against 137 at 1 to 2 mm, length in the 1 to 2 mm class 1,989 m against 213 m.
