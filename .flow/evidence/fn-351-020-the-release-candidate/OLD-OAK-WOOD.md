# What the leafless wood of the old Oregon white oak is, 13 Nov 2026

Measurement only: no product change, no verdicts, nothing committed or pushed. Branch `fn-351-phase-top` at `d9228bdd8` (detached copy `/home/daniel/Projects/telperion/.worktrees/census-old`, its own `target/`; `p3top` untouched). Preset `oregon-white-oak`, grown through the pipeline's own call (`executor::growth`, then `telperion_space::grow`), which gives the site's light `Light { extinction: 0.5, sky: 0.5 }`. Scratch `$S` = `/home/daniel/.cache/claude-tmp/claude-1000/-home-daniel-Projects-telperion/d6d5cb56-440a-44f4-803a-6a595b8a3532/scratchpad`. Everything below is counted by code over the settled structure `grow()` returns; no estimate.

Trees: seed 1001 at ages 80, 120, 200, 300; seed 1007 at 120 and 200. These are two seeds, not the 12-seed medians of the brief, so the absolute numbers differ from the brief's (seed 1001 at 120 is 4.0 M phytomers, the brief's median 9.5 M); the trend with age is the same and is what is read here.

## How it was read

A temporary reader (`$S/oldoak/oldoak-final.patch`, 654 lines, applied to `census-old`, removed again with `git apply -R`; see the end of this file) sits in the grower's settle (`grow/settle.rs`), switched on by the environment variable `OLDOAK`:

- **Before the settle takes the axes**: each held axis's bytes by store (record, phytomer and unit vectors, draws, rough-layout pencil, successor and working-set lists) and, for each axis with a living apex in the grower's `live` set, the light at its tip (`Pencil::light`, the light the last cycle read at the tip).
- **After `assign`, before `shed_past`**: the reader replicates the shed's keep/drop loop (same `fades`, same retirement closures, same `stands`), records each kept axis's grower state (the grower's `closed(i)` flag), and records each lateral or relay of a kept axis that the shed took on its own fade as a shed lateral at its bearer's node, plus each retired root already let go (`Stand`) whose bearer is kept. Checks: the kept count equals the settled structure's axis count and every kept axis's `lineage` equals the structure's at the same place (lineage mismatches: 0 in all six trees).
- **After the settle has laid the tree** (scale, thicken, place, sag; end of `Settling::settle_lay`): every table below, from the structure, the applied fades (`Settling::shares`) and the records above.

The grower is deterministic: repeated runs of the same tree gave identical totals and tables.

## Definitions

- **Axis, phytomer**: the settled structure's (`Structure.axes`, `Axis.phytomers`), after shedding. Retired subtrees that were shed are not in it; they enter only as shed laterals on their bearers (table E).
- **Class of an axis** (tables B, C, G, H):
  - **L, apex living**: `apex_end` is none.
  - **B, bearer**: `apex_end` is set, and the subtree (laterals, continuations and relays, all depths) holds an axis with a living apex.
  - **D, dead standing wood**: `apex_end` is set and no axis of its subtree has a living apex. The structure holds no sleeping buds; the grower's own flag for an axis is given beside it ("open (grower's flag: not closed)": the axis was on the way to a living apex or sleeping bud at the last cycle's working set, which is computed at the start of the cycle, so an apex that ended in the last cycle is still "open"). I do not split the flag further.
- **Years since the subtree last lived** (D only): the settle's age less the latest `apex_end` in the subtree (the cycle the apex stopped). Table H's "years" column: for D that, for B the axis's own `age - apex_end`, -1 for a living apex.
- **Fade class**: the axis's applied fade (`Shed::applied`, the least of its own and its bearers'): 1, over 0.5, 0.1 to 0.5, 0.01 to 0.1, up to 0.01. Only laterals and relays carry a delay; the trunk, leader and continuations are at 1.
- **Leaf of a phytomer** (settled structure): `leaf_area(PA) x scale x clamp(leaf_years - (age - cycle), 0, 1)` with the phytomer's final `scale` (which already includes its unit's `size`), counted only on an axis whose subtree holds a living apex or that the grower flags not closed (the grower's own rule: closed wood bears none). **Leafed phytomer**: its leaf is above 0. With `leafYears = 1` in every PA row of the oak, a leafed phytomer is one grown in the last cycle. **Leaf m2** is the sum. Cross-check against what the grower itself deposited into the light in its last cycle (`Grower::leaves`): leafed phytomers within 0.3% (355,114 / 355,203 at 80; 741,850 / 742,152 at 120; 1,033,774 / 1,034,359 at 200; 895,705 / 896,136 at 300; 1,516,512 / 1,517,064 and 2,324,420 / 2,325,461 for 1007), leaf m2 1.5 to 3.6% lower than the grower's deposit (column "grower's last deposit" in table A). **The brief's "m2 of leaf" is presumably the drawn mesh's leaf area; that was not read (no drawing), so the ratios here are against the grower's leaf, not the brief's.** "Number of leaves" in table F is the number of leafed phytomers; leaves per phytomer in the drawn tree are not read.
- **Axis length** (D, F, H): the sum of the internodes, each the distance between consecutive laid node tips, the first from the axis's `base`. **Extension in a cycle**: the same sum over phytomers whose `cycle` is that cycle (newest first: `age`, `age-1`, ...). **Height of the tip**: tip z less the seed axis's base z. **Tip from the trunk**: horizontal distance from the vertical through the seed axis's base. "Top tip" in table H is the highest phytomer tip.
- **Bare wood of a living-apex axis** (table E, F): the axis's phytomers that are not leafed and that carry no lateral or relay axis whose subtree holds a living apex. Bare length is the sum of those phytomers' internodes; "longest bare run" is the longest contiguous run of such phytomers. Living-apex axes with no phytomer yet (born in the last cycle) are in table D's first row and carry no bare wood; they are not in table E.
- **Shed laterals on bare wood**: lateral or relay axes the shed took on their own fade whose bearer is this axis at a bare phytomer's node, plus retired roots let go earlier whose bearer is this axis at that node; counted once per root, not per descendant. **Kept dead laterals**: lateral or relay axes standing there with no living apex in their subtree. **Bare phytomers with no lateral axis recorded**: neither of the two. **Never formed is not recorded**: the grower keeps no record of a bud that was not made, so this last column mixes buds never formed with sleepers that never woke, and cannot be split.
- **Table G bytes** are the grower's allocation per held axis before the settle, in the footprint reader's way (record size plus vector capacities), by four groups: axis record with its phytomer and unit vectors; draws; rough-layout pencil (layer heap and path); successor lists, working-set links and children lists, retirement fate. Each is summed over the kept axes of a class (the reader replicates the shed, so the axis is the settled one). "Dropped by the shed" is the same for the axes the shed took (not in any class). The footprint's store-level numbers are in the logs (`OO|store`).

## Commands

```
cd /home/daniel/Projects/telperion/.worktrees/census-old        # git worktree add .worktrees/census-old fn-351-phase-top --detach
git apply $S/oldoak/oldoak-final.patch                          # (the new file is untracked in the worktree; the patch carries it)
bash $S/oldoak/build.sh      # TELPERION_HEAVY_RESERVE=8 telperion-heavy 8 --threads 4 -- cargo build --profile ci   (runner crate $S/oldoak/runner, CARGO_TARGET_DIR=census-old/target)
bash $S/oldoak/runall.sh     # per tree: OLDOAK=1 TELPERION_HEAVY_RESERVE=8 telperion-heavy <8|24> --threads 4 -- python3 $S/peak.py census-old/target/ci/heldrun oregon-white-oak <age> <seed>
python3 -I $S/oldoak/assemble.py                                 # tables from $S/oldoak/logs/<tree>.log
```

Admission: 8 GB for seed 1001 (all four ages), 24 GB for seed 1007 (both ages), each tree its own process, one at a time, no GPU, no drawing. Logs `$S/oldoak/logs/s<seed>-a<age>.{out,log}`.

## Run record (final runs, 13 Nov 2026, Zurich)

The yield watcher's last line before and through these runs is `13:13:42 RESUME`; no PAUSE falls inside any final run, so the walls are timed. Wall includes the reader (a second pass over the axes, about 10 to 25% on top, not measured apart).

| tree | start | end | wall s | peak MiB (peak.py) |
|---|---|---|---|---|
| seed 1001, age 80 | 13:34:12 | 13:34:19 | 7 | 1,051 |
| seed 1001, age 120 | 13:34:19 | 13:34:44 | 25 | 2,299 |
| seed 1001, age 200 | 13:34:44 | 13:36:02 | 78 | 3,174 |
| seed 1001, age 300 | 13:36:02 | 13:38:32 | 150 | 2,783 |
| seed 1007, age 120 | 13:38:32 | 13:39:28 | 56 | 5,205 |
| seed 1007, age 200 | 13:39:28 | 13:42:48 | 200 | 8,329 |

Earlier runs were discarded: the first reader panicked on its first runs (it read the axes after the settle had taken them; fixed), and later runs lacked lines added afterwards (empty axes, tables H and I), two of them started inside the 13:10 to 13:13 PAUSE window. The 1007 at 200 peak here is 8.3 GiB, below the 12 to 16 GiB of earlier runs on other branches.


### Table A. Totals

| tree | axes | phytomers | leafed phytomers | leaf m2 (settled) | phytomers per m2 | wood m (sum of internodes) | held axes before shed | held phytomers before shed | grower's last deposit: leaves / m2 | living axes with no phytomer yet |
|---|---|---|---|---|---|---|---|---|---|---|
| s1001-a80 | 271,690 | 1,779,662 | 355,114 | 346.5 | 5,136 | 9,323 | 476,180 | 2,139,148 | 355,203 / 351.7 | 59,731 |
| s1001-a120 | 578,853 | 3,969,512 | 741,850 | 581.8 | 6,823 | 17,305 | 835,957 | 4,893,129 | 742,152 / 599.1 | 120,474 |
| s1001-a200 | 757,677 | 5,492,748 | 1,033,774 | 553.9 | 9,917 | 16,598 | 1,403,043 | 6,930,389 | 1,034,359 / 570.0 | 152,041 |
| s1001-a300 | 613,174 | 4,699,521 | 895,705 | 331.4 | 14,180 | 9,970 | 909,460 | 6,027,081 | 896,136 / 343.9 | 117,283 |
| s1007-a120 | 1,381,640 | 8,370,953 | 1,516,512 | 959.5 | 8,724 | 29,613 | 2,610,551 | 10,457,252 | 1,517,064 / 980.9 | 309,289 |
| s1007-a200 | 2,079,821 | 13,213,852 | 2,324,420 | 914.7 | 14,446 | 28,649 | 4,027,948 | 17,075,391 | 2,325,461 / 944.3 | 449,206 |

- s1001-a80: check `lineage_mismatch 0`; stands carried `100756/131113`; lost own 136488, lost below 68002.

- s1001-a120: check `lineage_mismatch 0`; stands carried `398239/531999`; lost own 188428, lost below 68676.

- s1001-a200: check `lineage_mismatch 0`; stands carried `385762/543529`; lost own 376888, lost below 268478.

- s1001-a300: check `lineage_mismatch 0`; stands carried `409408/577971`; lost own 178687, lost below 117599.

- s1007-a120: check `lineage_mismatch 0`; stands carried `739966/1052172`; lost own 721876, lost below 507035.

- s1007-a200: check `lineage_mismatch 0`; stands carried `1285885/1947538`; lost own 1074096, lost below 874031.

### Table B. Phytomers by the axis's state


**s1001-a80**

| class | axes | phytomers | share | leafed phytomers | wood m |
|---|---|---|---|---|---|
| apex living | 129,156 | 520,434 | 29.2% | 289,373 | 2,616 |
| apex ended, bears a living descendant | 17,265 | 424,693 | 23.9% | 15,505 | 3,023 |
| apex ended, nothing living below (dead standing wood) | 125,269 | 834,535 | 46.9% | 50,236 | 3,683 |

Dead standing wood by the grower's own flag:

| axes | phytomers | grower flag |
|---|---|---|
| 69,428 | 509,076 | closed |
| 55,841 | 325,459 | open (grower's flag: not closed) |

Dead standing wood by years since its subtree last lived (rows) and applied fade class (columns), phytomers (axes):

| years since lived | full (1) | 0.5-1 | 0.1-0.5 | 0.01-0.1 | <=0.01 | total phytomers |
|---|---|---|---|---|---|---|
| 0 | 177,059 (13,329) | 7,716 (554) | 7,017 (526) | 5,506 (418) | 136 (10) | 197,434 |
| 1 | 280,632 (48,383) | 13,893 (2,547) | 14,450 (2,741) | 9,040 (1,531) | 886 (165) | 318,901 |
| 2 | 259,910 (45,708) | 15,609 (2,663) | 16,194 (3,067) | 9,107 (1,537) | 798 (181) | 301,618 |
| 3-5 | 2,260 (382) | 177 (31) | 209 (46) | 176 (26) | 19 (1) | 2,841 |
| 6-10 | 7,120 (614) | 555 (50) | 818 (71) | 367 (32) | 18 (3) | 8,878 |
| 11-20 | 2,440 (305) | 205 (33) | 315 (45) | 147 (19) | 37 (5) | 3,144 |
| >20 | 1,115 (155) | 224 (31) | 249 (42) | 131 (17) | 0 (1) | 1,719 |

**s1001-a120**

| class | axes | phytomers | share | leafed phytomers | wood m |
|---|---|---|---|---|---|
| apex living | 265,228 | 1,079,568 | 27.2% | 601,644 | 4,208 |
| apex ended, bears a living descendant | 40,922 | 1,013,477 | 25.5% | 32,922 | 6,101 |
| apex ended, nothing living below (dead standing wood) | 272,703 | 1,876,467 | 47.3% | 107,284 | 6,996 |

Dead standing wood by the grower's own flag:

| axes | phytomers | grower flag |
|---|---|---|
| 153,951 | 1,164,734 | closed |
| 118,752 | 711,733 | open (grower's flag: not closed) |

Dead standing wood by years since its subtree last lived (rows) and applied fade class (columns), phytomers (axes):

| years since lived | full (1) | 0.5-1 | 0.1-0.5 | 0.01-0.1 | <=0.01 | total phytomers |
|---|---|---|---|---|---|---|
| 0 | 350,342 (26,301) | 27,640 (2,046) | 29,350 (2,203) | 16,137 (1,194) | 3,967 (291) | 427,436 |
| 1 | 575,078 (97,031) | 45,689 (7,331) | 51,817 (8,562) | 24,853 (4,029) | 6,753 (1,186) | 704,190 |
| 2 | 549,134 (94,356) | 50,418 (7,975) | 53,943 (9,032) | 26,261 (4,193) | 6,416 (1,222) | 686,172 |
| 3-5 | 10,336 (1,055) | 321 (84) | 628 (114) | 273 (42) | 108 (15) | 11,666 |
| 6-10 | 21,961 (1,606) | 2,254 (170) | 2,922 (207) | 1,216 (116) | 189 (21) | 28,542 |
| 11-20 | 7,248 (839) | 1,062 (140) | 1,274 (161) | 537 (79) | 151 (23) | 10,272 |
| >20 | 4,432 (534) | 1,236 (161) | 1,522 (233) | 754 (114) | 245 (37) | 8,189 |

**s1001-a200**

| class | axes | phytomers | share | leafed phytomers | wood m |
|---|---|---|---|---|---|
| apex living | 351,497 | 1,473,428 | 26.8% | 826,353 | 3,995 |
| apex ended, bears a living descendant | 52,871 | 1,303,944 | 23.7% | 40,733 | 5,964 |
| apex ended, nothing living below (dead standing wood) | 353,309 | 2,715,376 | 49.4% | 166,688 | 6,639 |

Dead standing wood by the grower's own flag:

| axes | phytomers | grower flag |
|---|---|---|
| 203,249 | 1,703,219 | closed |
| 150,060 | 1,012,157 | open (grower's flag: not closed) |

Dead standing wood by years since its subtree last lived (rows) and applied fade class (columns), phytomers (axes):

| years since lived | full (1) | 0.5-1 | 0.1-0.5 | 0.01-0.1 | <=0.01 | total phytomers |
|---|---|---|---|---|---|---|
| 0 | 528,696 (39,851) | 34,260 (2,557) | 53,022 (3,971) | 23,883 (1,786) | 10,624 (786) | 650,485 |
| 1 | 819,955 (124,122) | 50,501 (6,953) | 83,396 (11,416) | 36,269 (4,917) | 14,108 (1,888) | 1,004,229 |
| 2 | 777,619 (120,515) | 56,145 (7,665) | 86,102 (12,105) | 36,874 (5,162) | 14,615 (2,001) | 971,355 |
| 3-5 | 16,988 (1,317) | 487 (65) | 913 (116) | 305 (45) | 115 (22) | 18,808 |
| 6-10 | 28,981 (1,919) | 2,170 (171) | 5,195 (334) | 702 (87) | 777 (45) | 37,825 |
| 11-20 | 10,262 (996) | 1,233 (129) | 2,487 (245) | 1,253 (126) | 480 (51) | 15,715 |
| >20 | 7,885 (814) | 1,866 (226) | 3,960 (475) | 1,896 (273) | 1,352 (158) | 16,959 |

**s1001-a300**

| class | axes | phytomers | share | leafed phytomers | wood m |
|---|---|---|---|---|---|
| apex living | 287,182 | 1,246,961 | 26.5% | 701,264 | 2,327 |
| apex ended, bears a living descendant | 43,518 | 1,016,486 | 21.6% | 31,202 | 3,682 |
| apex ended, nothing living below (dead standing wood) | 282,474 | 2,436,074 | 51.8% | 163,239 | 3,960 |

Dead standing wood by the grower's own flag:

| axes | phytomers | grower flag |
|---|---|---|
| 166,162 | 1,551,039 | closed |
| 116,312 | 885,035 | open (grower's flag: not closed) |

Dead standing wood by years since its subtree last lived (rows) and applied fade class (columns), phytomers (axes):

| years since lived | full (1) | 0.5-1 | 0.1-0.5 | 0.01-0.1 | <=0.01 | total phytomers |
|---|---|---|---|---|---|---|
| 0 | 502,243 (37,882) | 25,501 (1,949) | 51,856 (3,886) | 23,634 (1,807) | 20,180 (1,541) | 623,414 |
| 1 | 718,401 (97,292) | 34,890 (4,269) | 67,119 (7,919) | 30,863 (3,441) | 24,474 (2,610) | 875,747 |
| 2 | 684,514 (93,742) | 41,440 (5,081) | 73,848 (8,745) | 30,706 (3,595) | 24,829 (2,674) | 855,337 |
| 3-5 | 18,223 (1,155) | 684 (60) | 629 (63) | 349 (27) | 119 (12) | 20,004 |
| 6-10 | 26,144 (1,651) | 1,647 (94) | 2,650 (190) | 1,304 (74) | 913 (51) | 32,658 |
| 11-20 | 8,865 (765) | 972 (97) | 2,083 (169) | 678 (60) | 406 (39) | 13,004 |
| >20 | 7,003 (658) | 2,103 (198) | 3,808 (371) | 1,358 (145) | 1,638 (162) | 15,910 |

**s1007-a120**

| class | axes | phytomers | share | leafed phytomers | wood m |
|---|---|---|---|---|---|
| apex living | 611,185 | 2,295,657 | 27.4% | 1,266,889 | 7,536 |
| apex ended, bears a living descendant | 115,488 | 2,536,184 | 30.3% | 80,319 | 10,815 |
| apex ended, nothing living below (dead standing wood) | 654,967 | 3,539,112 | 42.3% | 169,304 | 11,261 |

Dead standing wood by the grower's own flag:

| axes | phytomers | grower flag |
|---|---|---|
| 352,159 | 2,118,596 | closed |
| 302,808 | 1,420,516 | open (grower's flag: not closed) |

Dead standing wood by years since its subtree last lived (rows) and applied fade class (columns), phytomers (axes):

| years since lived | full (1) | 0.5-1 | 0.1-0.5 | 0.01-0.1 | <=0.01 | total phytomers |
|---|---|---|---|---|---|---|
| 0 | 569,639 (42,586) | 37,285 (2,772) | 47,241 (3,495) | 29,362 (2,143) | 6,280 (459) | 689,807 |
| 1 | 1,095,796 (226,257) | 88,938 (21,445) | 123,131 (31,006) | 72,431 (17,843) | 18,886 (4,848) | 1,399,182 |
| 2 | 1,016,175 (212,815) | 97,306 (22,002) | 124,609 (30,858) | 73,490 (17,757) | 17,932 (4,623) | 1,329,512 |
| 3-5 | 27,841 (2,996) | 1,764 (383) | 2,634 (534) | 1,141 (265) | 559 (89) | 33,939 |
| 6-10 | 44,037 (3,371) | 4,025 (423) | 5,252 (642) | 3,472 (398) | 1,318 (133) | 58,104 |
| 11-20 | 11,113 (1,499) | 1,431 (291) | 2,478 (467) | 1,433 (269) | 277 (89) | 16,732 |
| >20 | 5,814 (832) | 1,797 (336) | 2,129 (528) | 1,743 (394) | 353 (119) | 11,836 |

**s1007-a200**

| class | axes | phytomers | share | leafed phytomers | wood m |
|---|---|---|---|---|---|
| apex living | 908,398 | 3,469,835 | 26.3% | 1,923,891 | 6,810 |
| apex ended, bears a living descendant | 187,651 | 4,068,236 | 30.8% | 115,215 | 10,625 |
| apex ended, nothing living below (dead standing wood) | 983,772 | 5,675,781 | 43.0% | 285,314 | 11,213 |

Dead standing wood by the grower's own flag:

| axes | phytomers | grower flag |
|---|---|---|
| 542,605 | 3,466,913 | closed |
| 441,167 | 2,208,868 | open (grower's flag: not closed) |

Dead standing wood by years since its subtree last lived (rows) and applied fade class (columns), phytomers (axes):

| years since lived | full (1) | 0.5-1 | 0.1-0.5 | 0.01-0.1 | <=0.01 | total phytomers |
|---|---|---|---|---|---|---|
| 0 | 888,356 (66,521) | 62,775 (4,646) | 112,960 (8,404) | 54,234 (4,060) | 25,224 (1,855) | 1,143,549 |
| 1 | 1,643,171 (318,100) | 129,400 (26,898) | 233,676 (51,389) | 127,362 (28,756) | 62,831 (15,121) | 2,196,440 |
| 2 | 1,548,126 (306,126) | 142,255 (29,035) | 248,534 (54,376) | 129,453 (30,046) | 63,179 (15,315) | 2,131,547 |
| 3-5 | 44,499 (4,046) | 2,427 (422) | 3,882 (794) | 2,703 (510) | 1,156 (237) | 54,667 |
| 6-10 | 60,239 (4,723) | 6,562 (618) | 10,475 (1,168) | 6,527 (658) | 2,999 (340) | 86,802 |
| 11-20 | 18,548 (2,224) | 2,752 (436) | 5,276 (813) | 3,150 (511) | 1,139 (232) | 30,865 |
| >20 | 11,097 (1,492) | 3,950 (610) | 7,489 (1,388) | 5,683 (1,099) | 3,692 (803) | 31,911 |

### Table C. By the axis's PA row, phytomers / leafed phytomers / axes


**s1001-a80**

| PA | apex living | bearer | dead standing |
|---|---|---|---|
| 2 trunk | - | 98 / 0 / 1 | - |
| 3 low_fork | - | 14 / 0 / 1 | - |
| 4 leader | - | 41 / 0 / 1 | - |
| 6 limb | 209 / 36 / 6 | 1,479 / 0 / 54 | - |
| 7 bough | 3,862 / 344 / 70 | 9,202 / 72 / 222 | - |
| 8 sprig | - | 49 / 0 / 1 | - |
| 9 branch | 78,783 / 18,359 / 4,035 | 242,028 / 3,408 / 8,243 | 29,235 / 0 / 923 |
| 12 shoot | 3,341 / 1,830 / 708 | 3,669 / 279 / 268 | 15,310 / 5 / 2,284 |
| 13 twig | 156,160 / 67,050 / 20,028 | 168,113 / 11,746 / 8,474 | 171,720 / 18 / 10,449 |
| 14 short | 278,079 / 201,754 / 104,309 | - | 618,270 / 50,213 / 111,613 |

**s1001-a120**

| PA | apex living | bearer | dead standing |
|---|---|---|---|
| 2 trunk | - | 98 / 0 / 1 | - |
| 3 low_fork | - | 14 / 0 / 1 | - |
| 4 leader | - | 41 / 0 / 1 | - |
| 6 limb | 169 / 49 / 10 | 3,519 / 11 / 105 | - |
| 7 bough | 3,888 / 468 / 137 | 33,929 / 330 / 1,550 | 205 / 0 / 5 |
| 9 branch | 160,966 / 36,551 / 8,085 | 606,204 / 6,784 / 20,513 | 91,211 / 0 / 3,109 |
| 12 shoot | 6,858 / 3,906 / 1,457 | 9,229 / 738 / 658 | 44,713 / 9 / 6,166 |
| 13 twig | 317,984 / 135,516 / 40,499 | 360,443 / 25,059 / 18,093 | 385,061 / 38 / 23,275 |
| 14 short | 589,703 / 425,154 / 215,040 | - | 1,355,277 / 107,237 / 240,148 |

**s1001-a200**

| PA | apex living | bearer | dead standing |
|---|---|---|---|
| 2 trunk | - | 98 / 0 / 1 | - |
| 3 low_fork | - | 14 / 0 / 1 | - |
| 4 leader | - | 41 / 0 / 1 | - |
| 6 limb | 363 / 54 / 11 | 6,660 / 11 / 187 | - |
| 7 bough | 4,093 / 520 / 202 | 93,966 / 690 / 6,801 | 891 / 0 / 40 |
| 9 branch | 210,111 / 45,770 / 10,151 | 723,393 / 7,068 / 22,065 | 140,308 / 0 / 4,556 |
| 12 shoot | 9,428 / 5,191 / 1,859 | 11,774 / 906 / 827 | 67,581 / 14 / 7,973 |
| 13 twig | 404,564 / 171,558 / 50,507 | 467,998 / 32,058 / 22,988 | 509,934 / 16 / 29,895 |
| 14 short | 844,869 / 603,260 / 288,767 | - | 1,996,662 / 166,658 / 310,845 |

**s1001-a300**

| PA | apex living | bearer | dead standing |
|---|---|---|---|
| 2 trunk | - | 98 / 0 / 1 | - |
| 3 low_fork | - | 14 / 0 / 1 | - |
| 4 leader | - | - | 41 / 0 / 1 |
| 6 limb | 357 / 46 / 10 | 10,129 / 12 / 272 | 107 / 0 / 6 |
| 7 bough | 3,741 / 494 / 176 | 123,161 / 551 / 10,986 | 4,456 / 0 / 384 |
| 9 branch | 166,926 / 34,938 / 7,706 | 495,076 / 4,401 / 13,425 | 116,049 / 0 / 3,531 |
| 12 shoot | 7,746 / 4,284 / 1,451 | 10,663 / 811 / 730 | 57,476 / 8 / 5,872 |
| 13 twig | 315,581 / 132,730 / 38,729 | 377,345 / 25,427 / 18,103 | 410,966 / 9 / 23,113 |
| 14 short | 752,610 / 528,772 / 239,110 | - | 1,846,979 / 163,222 / 249,567 |

**s1007-a120**

| PA | apex living | bearer | dead standing |
|---|---|---|---|
| 2 trunk | - | 94 / 0 / 1 | - |
| 3 low_fork | - | 15 / 0 / 1 | - |
| 4 leader | - | 42 / 0 / 1 | - |
| 6 limb | 1,123 / 184 / 36 | 9,413 / 22 / 254 | - |
| 7 bough | 9,483 / 1,457 / 385 | 90,101 / 810 / 3,847 | 1,111 / 0 / 49 |
| 8 sprig | - | 48 / 0 / 1 | - |
| 9 branch | 392,835 / 102,999 / 22,774 | 1,677,425 / 24,497 / 71,186 | 228,937 / 0 / 9,253 |
| 12 shoot | 13,601 / 7,913 / 3,322 | 14,541 / 1,079 / 1,124 | 76,461 / 57 / 14,536 |
| 13 twig | 780,379 / 342,741 / 106,199 | 744,505 / 53,911 / 39,073 | 852,073 / 165 / 60,417 |
| 14 short | 1,098,236 / 811,595 / 478,469 | - | 2,380,530 / 169,082 / 570,712 |

**s1007-a200**

| PA | apex living | bearer | dead standing |
|---|---|---|---|
| 2 trunk | - | 94 / 0 / 1 | - |
| 3 low_fork | - | 15 / 0 / 1 | - |
| 4 leader | - | 42 / 0 / 1 | - |
| 6 limb | 923 / 175 / 36 | 19,968 / 33 / 522 | 335 / 0 / 10 |
| 7 bough | 13,268 / 1,722 / 571 | 244,465 / 1,659 / 17,121 | 2,855 / 0 / 257 |
| 9 branch | 571,788 / 147,328 / 32,529 | 2,675,359 / 33,084 / 111,026 | 370,816 / 0 / 14,351 |
| 12 shoot | 21,035 / 12,259 / 4,973 | 21,618 / 1,629 / 1,618 | 135,685 / 63 / 23,954 |
| 13 twig | 1,142,858 / 500,296 / 153,878 | 1,106,675 / 78,810 / 57,361 | 1,313,856 / 191 / 91,813 |
| 14 short | 1,719,963 / 1,262,111 / 716,411 | - | 3,852,234 / 285,060 / 853,387 |

### Table D. Living-apex axes by their own leafed phytomers


**s1001-a80**

| leafed phytomers | axes | phytomers | mean length m | max length m | mean last-cycle extension m | PA split (axes) |
|---|---|---|---|---|---|---|
| 0 | 59,731 | 0 | 0.00 | 0.00 | 0.000 | bough 14, branch 373, shoot 300, twig 5,152, short 53,892 |
| 1-5 | 68,428 | 496,341 | 0.03 | 4.38 | 0.016 | limb 1, bough 9, branch 2,717, shoot 408, twig 14,876, short 50,417 |
| 6-20 | 997 | 24,093 | 0.26 | 12.21 | 0.053 | limb 5, bough 47, branch 945 |
| >20 | 0 | 0 | - | 0.00 | - |  |

**s1001-a120**

| leafed phytomers | axes | phytomers | mean length m | max length m | mean last-cycle extension m | PA split (axes) |
|---|---|---|---|---|---|---|
| 0 | 120,474 | 0 | 0.00 | 0.00 | 0.000 | limb 2, bough 61, branch 756, shoot 591, twig 10,350, short 108,714 |
| 1-5 | 142,882 | 1,035,252 | 0.03 | 3.48 | 0.013 | limb 1, bough 12, branch 5,528, shoot 866, twig 30,149, short 106,326 |
| 6-20 | 1,872 | 44,316 | 0.18 | 5.50 | 0.040 | limb 7, bough 64, branch 1,801 |
| >20 | 0 | 0 | - | 0.00 | - |  |

**s1001-a200**

| leafed phytomers | axes | phytomers | mean length m | max length m | mean last-cycle extension m | PA split (axes) |
|---|---|---|---|---|---|---|
| 0 | 152,041 | 0 | 0.00 | 0.00 | 0.000 | limb 2, bough 116, branch 993, shoot 708, twig 12,379, short 137,843 |
| 1-5 | 197,083 | 1,413,925 | 0.02 | 2.26 | 0.009 | limb 2, bough 18, branch 6,860, shoot 1,151, twig 38,128, short 150,924 |
| 6-20 | 2,373 | 59,503 | 0.14 | 2.08 | 0.031 | limb 7, bough 68, branch 2,298 |
| >20 | 0 | 0 | - | 0.00 | - |  |

**s1001-a300**

| leafed phytomers | axes | phytomers | mean length m | max length m | mean last-cycle extension m | PA split (axes) |
|---|---|---|---|---|---|---|
| 0 | 117,283 | 0 | 0.00 | 0.00 | 0.000 | limb 2, bough 93, branch 727, shoot 502, twig 9,225, short 106,734 |
| 1-5 | 168,048 | 1,198,723 | 0.01 | 0.90 | 0.006 | limb 3, bough 23, branch 5,193, shoot 949, twig 29,504, short 132,376 |
| 6-20 | 1,851 | 48,238 | 0.11 | 1.77 | 0.023 | limb 5, bough 60, branch 1,786 |
| >20 | 0 | 0 | - | 0.00 | - |  |

**s1007-a120**

| leafed phytomers | axes | phytomers | mean length m | max length m | mean last-cycle extension m | PA split (axes) |
|---|---|---|---|---|---|---|
| 0 | 309,289 | 0 | 0.00 | 0.00 | 0.000 | limb 5, bough 144, branch 2,155, shoot 1,566, twig 30,008, short 275,411 |
| 1-5 | 296,602 | 2,185,318 | 0.02 | 3.47 | 0.011 | limb 7, bough 52, branch 15,538, shoot 1,756, twig 76,191, short 203,058 |
| 6-20 | 5,294 | 110,339 | 0.13 | 5.35 | 0.030 | limb 24, bough 189, branch 5,081 |
| >20 | 0 | 0 | - | 0.00 | - |  |

**s1007-a200**

| leafed phytomers | axes | phytomers | mean length m | max length m | mean last-cycle extension m | PA split (axes) |
|---|---|---|---|---|---|---|
| 0 | 449,206 | 0 | 0.00 | 0.00 | 0.000 | limb 6, bough 286, branch 3,073, shoot 2,248, twig 42,673, short 400,920 |
| 1-5 | 451,469 | 3,304,947 | 0.01 | 2.01 | 0.007 | limb 9, bough 62, branch 21,977, shoot 2,725, twig 111,205, short 315,491 |
| 6-20 | 7,723 | 164,888 | 0.08 | 4.58 | 0.018 | limb 21, bough 223, branch 7,479 |
| >20 | 0 | 0 | - | 0.00 | - |  |

### Table E. Bare wood of the living-apex axes


**s1001-a80**

| bare length per axis | axes | axis phytomers | axis wood m | bare m | bare phytomers | shed laterals on bare wood | kept dead laterals on bare wood | bare phytomers with no lateral axis recorded |
|---|---|---|---|---|---|---|---|---|
| <0.5 m | 69,194 | 508,620 | 2,315 | 894 | 184,014 | 9,466 | 32,765 | 143,544 |
| 0.5-2 m | 226 | 11,100 | 275 | 186 | 7,600 | 1,553 | 1,258 | 4,794 |
| 2-5 m | 4 | 365 | 14 | 10 | 265 | 61 | 22 | 182 |
| >5 m | 1 | 349 | 12 | 10 | 296 | 97 | 11 | 194 |
| all | 69,425 | 520,434 | 2,616 | 1,100 | 192,175 | 11,177 | 34,056 | 148,714 |

**s1001-a120**

| bare length per axis | axes | axis phytomers | axis wood m | bare m | bare phytomers | shed laterals on bare wood | kept dead laterals on bare wood | bare phytomers with no lateral axis recorded |
|---|---|---|---|---|---|---|---|---|
| <0.5 m | 144,429 | 1,064,143 | 3,812 | 1,474 | 388,361 | 21,089 | 66,350 | 304,361 |
| 0.5-2 m | 319 | 14,513 | 374 | 250 | 9,908 | 1,985 | 1,816 | 6,121 |
| 2-5 m | 6 | 912 | 22 | 17 | 727 | 193 | 40 | 497 |
| >5 m | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| all | 144,754 | 1,079,568 | 4,208 | 1,741 | 398,996 | 23,267 | 68,206 | 310,979 |

**s1001-a200**

| bare length per axis | axes | axis phytomers | axis wood m | bare m | bare phytomers | shed laterals on bare wood | kept dead laterals on bare wood | bare phytomers with no lateral axis recorded |
|---|---|---|---|---|---|---|---|---|
| <0.5 m | 199,272 | 1,463,201 | 3,817 | 1,522 | 528,664 | 26,611 | 78,768 | 427,085 |
| 0.5-2 m | 184 | 10,227 | 178 | 128 | 7,410 | 1,786 | 1,115 | 4,536 |
| 2-5 m | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| >5 m | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| all | 199,456 | 1,473,428 | 3,995 | 1,651 | 536,074 | 28,397 | 79,883 | 431,621 |

**s1001-a300**

| bare length per axis | axes | axis phytomers | axis wood m | bare m | bare phytomers | shed laterals on bare wood | kept dead laterals on bare wood | bare phytomers with no lateral axis recorded |
|---|---|---|---|---|---|---|---|---|
| <0.5 m | 169,877 | 1,244,884 | 2,306 | 929 | 447,852 | 19,623 | 57,291 | 373,396 |
| 0.5-2 m | 22 | 2,077 | 21 | 16 | 1,646 | 486 | 145 | 1,052 |
| 2-5 m | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| >5 m | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| all | 169,899 | 1,246,961 | 2,327 | 945 | 449,498 | 20,109 | 57,436 | 374,448 |

**s1007-a120**

| bare length per axis | axes | axis phytomers | axis wood m | bare m | bare phytomers | shed laterals on bare wood | kept dead laterals on bare wood | bare phytomers with no lateral axis recorded |
|---|---|---|---|---|---|---|---|---|
| <0.5 m | 301,336 | 2,268,891 | 6,815 | 2,683 | 850,849 | 55,851 | 184,773 | 621,350 |
| 0.5-2 m | 545 | 25,223 | 670 | 455 | 17,303 | 3,536 | 3,080 | 10,726 |
| 2-5 m | 15 | 1,543 | 51 | 39 | 1,193 | 290 | 104 | 807 |
| >5 m | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| all | 301,896 | 2,295,657 | 7,536 | 3,177 | 869,345 | 59,677 | 187,957 | 632,883 |

**s1007-a200**

| bare length per axis | axes | axis phytomers | axis wood m | bare m | bare phytomers | shed laterals on bare wood | kept dead laterals on bare wood | bare phytomers with no lateral axis recorded |
|---|---|---|---|---|---|---|---|---|
| <0.5 m | 458,969 | 3,456,806 | 6,586 | 2,647 | 1,292,036 | 82,473 | 265,108 | 959,610 |
| 0.5-2 m | 220 | 12,393 | 213 | 152 | 8,939 | 2,082 | 1,383 | 5,498 |
| 2-5 m | 3 | 636 | 11 | 9 | 564 | 223 | 19 | 340 |
| >5 m | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| all | 459,192 | 3,469,835 | 6,810 | 2,809 | 1,301,539 | 84,778 | 266,510 | 965,448 |

### Table F. The 20 living-apex axes with the longest bare wood


**s1001-a80**

| axis | PA | origin | born | phytomers | length m | bare m | longest bare run m | tip height m | tip from trunk m | extension, last 5 cycles m (newest first) | leafed phytomers | tip light |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 38 | pa7 | lateral | 23 | 349 | 12.211 | 10.333 | 1.292 | 14.96 | 9.23 | 0.225,0.225,0.185,0.226,0.259 | 6 | 0.9336696 |
| 2084 | pa7 | lateral | 57 | 135 | 4.384 | 3.209 | 0.387 | 18.45 | 3.14 | 0.165,0.235,0.166,0.197,0.164 | 5 | 0.9326906 |
| 2836 | pa7 | lateral | 60 | 124 | 3.913 | 2.827 | 0.366 | 18.16 | 6.85 | 0.166,0.185,0.187,0.159,0.183 | 6 | 0.90658164 |
| 6838 | pa9 | lateral | 69 | 55 | 2.778 | 2.168 | 0.580 | 13.21 | 6.24 | 0.220,0.178,0.235,0.282,0.301 | 5 | 0.89897764 |
| 8171 | pa9 | lateral | 70 | 51 | 2.950 | 2.040 | 0.752 | 14.99 | 6.46 | 0.269,0.331,0.279,0.277,0.216 | 5 | 0.9642063 |
| 10761 | pa9 | lateral | 71 | 45 | 2.680 | 1.844 | 0.599 | 14.32 | 5.83 | 0.381,0.335,0.337,0.334,0.322 | 6 | 0.9693879 |
| 6341 | pa7 | relay | 68 | 76 | 2.554 | 1.840 | 0.347 | 18.68 | 7.77 | 0.231,0.167,0.200,0.203,0.204 | 7 | 0.9788736 |
| 855 | pa7 | relay | 49 | 189 | 2.415 | 1.807 | 0.192 | 8.64 | 3.05 | 0.083,0.072,0.060,0.084,0.058 | 7 | 0.34563076 |
| 6833 | pa9 | lateral | 69 | 61 | 2.200 | 1.734 | 1.290 | 15.17 | 4.79 | 0.196,0.247,0.234,0.198,0.171 | 5 | 0.7269213 |
| 1494 | pa7 | relay | 54 | 159 | 2.265 | 1.730 | 0.501 | 9.93 | 1.48 | 0.083,0.084,0.074,0.087,0.088 | 7 | 0.28222492 |
| 1790 | pa7 | lateral | 56 | 144 | 2.124 | 1.626 | 0.241 | 10.66 | 0.48 | 0.111,0.097,0.092,0.090,0.088 | 7 | 0.5064617 |
| 1489 | pa7 | relay | 54 | 156 | 2.208 | 1.621 | 0.206 | 9.11 | 2.73 | 0.099,0.083,0.068,0.071,0.078 | 7 | 0.3856222 |
| 7991 | pa9 | lateral | 70 | 54 | 2.107 | 1.619 | 0.611 | 11.93 | 7.28 | 0.145,0.150,0.204,0.171,0.216 | 5 | 0.6144439 |
| 1145 | pa7 | lateral | 51 | 175 | 1.996 | 1.574 | 0.227 | 9.20 | 2.00 | 0.078,0.076,0.086,0.062,0.071 | 6 | 0.44309172 |
| 10326 | pa9 | lateral | 71 | 44 | 2.037 | 1.437 | 0.658 | 13.56 | 3.95 | 0.175,0.230,0.284,0.199,0.212 | 4 | 0.7481784 |
| 10429 | pa9 | lateral | 71 | 48 | 2.065 | 1.402 | 0.432 | 13.19 | 3.04 | 0.191,0.197,0.262,0.223,0.251 | 5 | 0.78357804 |
| 6816 | pa9 | lateral | 69 | 53 | 1.745 | 1.385 | 0.647 | 15.74 | 7.08 | 0.143,0.146,0.152,0.125,0.130 | 5 | 0.65175784 |
| 8475 | pa7 | relay | 70 | 62 | 2.141 | 1.383 | 0.345 | 18.52 | 2.34 | 0.204,0.240,0.206,0.242,0.206 | 6 | 0.9229922 |
| 6851 | pa9 | lateral | 69 | 57 | 1.724 | 1.366 | 0.820 | 14.10 | 6.85 | 0.150,0.150,0.174,0.109,0.166 | 5 | 0.51146597 |
| 6875 | pa9 | lateral | 69 | 58 | 1.871 | 1.313 | 0.652 | 15.77 | 5.06 | 0.201,0.132,0.162,0.165,0.170 | 6 | 0.91005784 |

**s1001-a120**

| axis | PA | origin | born | phytomers | length m | bare m | longest bare run m | tip height m | tip from trunk m | extension, last 5 cycles m (newest first) | leafed phytomers | tip light |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 8048 | pa7 | relay | 97 | 143 | 5.496 | 4.379 | 1.004 | 24.12 | 6.40 | 0.233,0.270,0.192,0.190,0.234 | 6 | 0.9573675 |
| 10252 | pa7 | relay | 100 | 124 | 4.248 | 3.038 | 0.620 | 25.74 | 1.50 | 0.241,0.208,0.244,0.245,0.247 | 7 | 0.9658207 |
| 3004 | pa7 | relay | 84 | 213 | 3.476 | 2.902 | 0.443 | 13.07 | 3.75 | 0.095,0.094,0.096,0.096,0.134 | 5 | 0.66652685 |
| 3197 | pa7 | relay | 85 | 207 | 3.567 | 2.898 | 0.434 | 13.99 | 2.18 | 0.113,0.103,0.091,0.113,0.114 | 7 | 0.57847226 |
| 20309 | pa9 | lateral | 109 | 56 | 2.850 | 2.119 | 1.009 | 22.21 | 6.34 | 0.234,0.230,0.226,0.210,0.262 | 5 | 0.9541172 |
| 5385 | pa7 | relay | 92 | 169 | 2.651 | 2.077 | 0.247 | 14.04 | 2.95 | 0.122,0.087,0.126,0.106,0.120 | 7 | 0.6213498 |
| 28972 | pa9 | lateral | 111 | 44 | 2.889 | 1.985 | 0.586 | 21.39 | 5.84 | 0.215,0.245,0.290,0.310,0.410 | 4 | 0.9513092 |
| 36013 | pa9 | lateral | 112 | 38 | 2.677 | 1.920 | 1.052 | 17.78 | 14.32 | 0.343,0.331,0.264,0.340,0.351 | 5 | 0.97657406 |
| 29311 | pa9 | lateral | 111 | 47 | 2.404 | 1.745 | 1.033 | 17.34 | 8.95 | 0.183,0.272,0.259,0.287,0.267 | 4 | 0.6827154 |
| 28703 | pa9 | lateral | 111 | 42 | 2.312 | 1.695 | 0.895 | 7.81 | 8.74 | 0.223,0.234,0.280,0.216,0.334 | 4 | 0.74463123 |
| 23274 | pa9 | lateral | 110 | 52 | 2.315 | 1.685 | 0.730 | 8.90 | 14.01 | 0.163,0.189,0.189,0.194,0.257 | 4 | 0.8047908 |
| 18911 | pa7 | relay | 108 | 71 | 2.439 | 1.677 | 0.405 | 24.47 | 12.18 | 0.176,0.211,0.210,0.205,0.171 | 5 | 0.9365903 |
| 23365 | pa9 | lateral | 110 | 53 | 2.479 | 1.655 | 0.667 | 9.54 | 9.03 | 0.233,0.207,0.210,0.220,0.276 | 6 | 0.7138841 |
| 23525 | pa9 | lateral | 110 | 52 | 2.252 | 1.652 | 0.807 | 12.18 | 8.05 | 0.201,0.206,0.208,0.272,0.280 | 5 | 0.69446284 |
| 29003 | pa9 | lateral | 111 | 46 | 2.337 | 1.524 | 0.383 | 24.09 | 4.30 | 0.318,0.290,0.269,0.221,0.269 | 6 | 0.96781987 |
| 20269 | pa9 | lateral | 109 | 58 | 1.857 | 1.516 | 0.533 | 19.33 | 8.40 | 0.150,0.153,0.113,0.106,0.139 | 5 | 0.59291494 |
| 23462 | pa9 | lateral | 110 | 47 | 2.086 | 1.506 | 0.630 | 21.14 | 6.39 | 0.173,0.170,0.265,0.178,0.279 | 4 | 0.82559496 |
| 20358 | pa7 | relay | 109 | 71 | 2.407 | 1.452 | 0.212 | 25.56 | 5.39 | 0.237,0.199,0.231,0.234,0.208 | 7 | 0.9621172 |
| 23548 | pa9 | lateral | 110 | 49 | 1.885 | 1.431 | 0.510 | 14.97 | 13.37 | 0.185,0.257,0.199,0.188,0.178 | 4 | 0.7534029 |
| 23852 | pa9 | lateral | 110 | 50 | 1.894 | 1.427 | 0.453 | 20.00 | 9.31 | 0.187,0.178,0.214,0.228,0.193 | 4 | 0.81396514 |

**s1001-a200**

| axis | PA | origin | born | phytomers | length m | bare m | longest bare run m | tip height m | tip from trunk m | extension, last 5 cycles m (newest first) | leafed phytomers | tip light |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 30006 | pa9 | lateral | 190 | 49 | 2.264 | 1.779 | 0.857 | 20.60 | 2.22 | 0.209,0.258,0.173,0.194,0.262 | 5 | 0.97833014 |
| 19752 | pa7 | relay | 182 | 104 | 2.033 | 1.659 | 0.783 | 21.70 | 10.72 | 0.152,0.122,0.103,0.149,0.125 | 7 | 0.7099409 |
| 19394 | pa6 | relay | 182 | 105 | 2.039 | 1.587 | 0.327 | 20.70 | 8.45 | 0.100,0.099,0.116,0.117,0.117 | 5 | 0.85009515 |
| 18340 | pa7 | relay | 181 | 112 | 2.083 | 1.499 | 0.189 | 20.54 | 8.14 | 0.141,0.098,0.114,0.094,0.131 | 7 | 0.95581317 |
| 22059 | pa7 | lateral | 184 | 92 | 1.755 | 1.448 | 0.862 | 21.85 | 10.50 | 0.127,0.128,0.123,0.122,0.140 | 6 | 0.83917785 |
| 28872 | pa9 | lateral | 189 | 54 | 1.894 | 1.418 | 0.621 | 19.94 | 4.97 | 0.139,0.140,0.133,0.203,0.174 | 4 | 0.87704176 |
| 28875 | pa9 | lateral | 189 | 51 | 1.908 | 1.412 | 0.826 | 20.15 | 5.53 | 0.173,0.173,0.132,0.171,0.187 | 5 | 0.8758183 |
| 19377 | pa7 | relay | 182 | 107 | 2.032 | 1.400 | 0.188 | 21.55 | 7.83 | 0.135,0.117,0.097,0.115,0.131 | 7 | 0.86908156 |
| 32768 | pa9 | lateral | 190 | 52 | 1.650 | 1.197 | 0.597 | 20.17 | 3.52 | 0.153,0.173,0.110,0.164,0.119 | 5 | 0.9306041 |
| 24636 | pa7 | relay | 186 | 85 | 1.657 | 1.150 | 0.240 | 22.90 | 7.38 | 0.137,0.117,0.140,0.101,0.144 | 7 | 0.94882923 |
| 28874 | pa9 | lateral | 189 | 52 | 1.554 | 1.147 | 0.550 | 20.83 | 7.22 | 0.148,0.145,0.144,0.144,0.167 | 5 | 0.947983 |
| 38756 | pa9 | lateral | 191 | 46 | 1.549 | 1.062 | 0.454 | 20.88 | 3.67 | 0.109,0.184,0.164,0.219,0.197 | 4 | 0.89970386 |
| 11767 | pa7 | relay | 171 | 173 | 1.255 | 0.993 | 0.285 | 14.49 | 2.19 | 0.054,0.048,0.062,0.054,0.042 | 6 | 0.4191367 |
| 30024 | pa9 | lateral | 190 | 52 | 1.341 | 0.991 | 0.629 | 17.04 | 3.09 | 0.127,0.105,0.125,0.104,0.106 | 6 | 0.6441164 |
| 28054 | pa9 | lateral | 189 | 56 | 1.269 | 0.967 | 0.358 | 22.38 | 2.75 | 0.087,0.072,0.102,0.127,0.109 | 5 | 0.7633007 |
| 48265 | pa9 | lateral | 192 | 42 | 1.401 | 0.961 | 0.197 | 19.10 | 5.17 | 0.201,0.171,0.214,0.225,0.171 | 6 | 0.95122313 |
| 28885 | pa9 | lateral | 189 | 54 | 1.282 | 0.956 | 0.479 | 15.80 | 7.37 | 0.123,0.113,0.089,0.134,0.086 | 6 | 0.5512214 |
| 15773 | pa7 | relay | 178 | 133 | 1.186 | 0.910 | 0.299 | 14.76 | 5.87 | 0.051,0.066,0.055,0.056,0.056 | 6 | 0.4472497 |
| 28978 | pa9 | lateral | 189 | 53 | 1.095 | 0.891 | 0.565 | 17.26 | 5.85 | 0.063,0.091,0.125,0.096,0.107 | 4 | 0.39981425 |
| 28889 | pa9 | lateral | 189 | 49 | 1.240 | 0.888 | 0.471 | 19.70 | 4.63 | 0.101,0.153,0.101,0.116,0.120 | 4 | 0.76019114 |

**s1001-a300**

| axis | PA | origin | born | phytomers | length m | bare m | longest bare run m | tip height m | tip from trunk m | extension, last 5 cycles m (newest first) | leafed phytomers | tip light |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 11182 | pa7 | relay | 260 | 241 | 1.769 | 1.560 | 0.833 | 17.70 | 9.29 | 0.049,0.056,0.048,0.049,0.048 | 6 | 0.54331684 |
| 13714 | pa7 | relay | 270 | 176 | 1.484 | 1.260 | 0.580 | 18.85 | 5.66 | 0.056,0.056,0.046,0.055,0.045 | 6 | 0.5058141 |
| 15545 | pa7 | relay | 275 | 151 | 1.398 | 1.088 | 0.216 | 19.57 | 4.13 | 0.070,0.060,0.050,0.049,0.068 | 7 | 0.74605453 |
| 17017 | pa7 | relay | 278 | 129 | 1.279 | 0.977 | 0.136 | 18.86 | 4.80 | 0.068,0.059,0.058,0.067,0.065 | 6 | 0.6714533 |
| 18398 | pa6 | relay | 280 | 116 | 1.110 | 0.872 | 0.166 | 18.06 | 8.90 | 0.057,0.057,0.056,0.057,0.067 | 6 | 0.71843547 |
| 13121 | pa7 | lateral | 268 | 193 | 0.927 | 0.808 | 0.640 | 18.06 | 2.72 | 0.034,0.039,0.032,0.032,0.026 | 6 | 0.41623342 |
| 14998 | pa7 | lateral | 274 | 155 | 0.879 | 0.775 | 0.170 | 15.62 | 0.98 | 0.039,0.033,0.033,0.031,0.029 | 7 | 0.28614867 |
| 28542 | pa9 | lateral | 290 | 48 | 0.976 | 0.728 | 0.494 | 17.85 | 10.38 | 0.110,0.100,0.083,0.079,0.076 | 6 | 0.88554025 |
| 15932 | pa7 | relay | 276 | 144 | 0.817 | 0.668 | 0.186 | 17.81 | 2.26 | 0.031,0.037,0.043,0.037,0.043 | 5 | 0.50600904 |
| 31684 | pa9 | lateral | 291 | 48 | 0.936 | 0.624 | 0.211 | 14.50 | 3.48 | 0.108,0.108,0.093,0.088,0.091 | 6 | 0.6538569 |
| 34078 | pa9 | lateral | 291 | 46 | 0.898 | 0.613 | 0.204 | 27.29 | 5.67 | 0.102,0.076,0.101,0.078,0.079 | 5 | 0.92929846 |
| 26756 | pa9 | lateral | 289 | 53 | 0.839 | 0.591 | 0.289 | 20.52 | 5.11 | 0.078,0.062,0.094,0.074,0.072 | 5 | 0.88333046 |
| 19923 | pa7 | relay | 282 | 108 | 0.805 | 0.573 | 0.116 | 18.53 | 8.17 | 0.053,0.054,0.046,0.044,0.036 | 7 | 0.5060284 |
| 28533 | pa9 | lateral | 290 | 53 | 0.791 | 0.568 | 0.231 | 20.29 | 3.67 | 0.090,0.080,0.075,0.091,0.061 | 5 | 0.8502387 |
| 28909 | pa9 | relay | 289 | 52 | 0.690 | 0.553 | 0.251 | 15.06 | 2.97 | 0.062,0.061,0.088,0.087,0.055 | 4 | 0.7650269 |
| 45412 | pa9 | lateral | 293 | 39 | 0.900 | 0.553 | 0.203 | 15.20 | 6.44 | 0.140,0.113,0.134,0.114,0.143 | 6 | 0.8856455 |
| 26740 | pa9 | lateral | 289 | 56 | 0.707 | 0.540 | 0.361 | 15.91 | 10.70 | 0.043,0.071,0.075,0.062,0.069 | 4 | 0.74976104 |
| 34055 | pa9 | lateral | 291 | 43 | 0.734 | 0.538 | 0.309 | 20.63 | 3.39 | 0.082,0.108,0.082,0.124,0.058 | 4 | 0.83162695 |
| 38150 | pa9 | lateral | 292 | 40 | 0.783 | 0.531 | 0.203 | 14.88 | 7.13 | 0.097,0.097,0.099,0.100,0.124 | 5 | 0.7891571 |
| 26864 | pa9 | lateral | 290 | 53 | 0.771 | 0.520 | 0.240 | 16.27 | 5.74 | 0.090,0.085,0.068,0.100,0.099 | 6 | 0.5840532 |

**s1007-a120**

| axis | PA | origin | born | phytomers | length m | bare m | longest bare run m | tip height m | tip from trunk m | extension, last 5 cycles m (newest first) | leafed phytomers | tip light |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 25427 | pa7 | relay | 98 | 133 | 5.349 | 4.279 | 0.798 | 23.19 | 15.97 | 0.254,0.257,0.250,0.247,0.207 | 6 | 0.9553427 |
| 32018 | pa7 | lateral | 101 | 117 | 4.563 | 3.629 | 1.552 | 23.44 | 16.68 | 0.228,0.227,0.268,0.227,0.231 | 6 | 0.9564766 |
| 32010 | pa7 | relay | 101 | 112 | 4.696 | 3.564 | 0.662 | 23.15 | 16.83 | 0.289,0.209,0.249,0.247,0.207 | 7 | 0.9239606 |
| 17491 | pa7 | relay | 93 | 159 | 3.560 | 2.723 | 0.389 | 15.85 | 8.54 | 0.132,0.111,0.112,0.134,0.158 | 6 | 0.88684195 |
| 6618 | pa7 | relay | 78 | 250 | 3.106 | 2.683 | 0.640 | 12.14 | 1.55 | 0.084,0.070,0.095,0.089,0.086 | 6 | 0.44040117 |
| 37514 | pa7 | relay | 103 | 101 | 3.472 | 2.553 | 0.378 | 24.51 | 13.08 | 0.179,0.176,0.243,0.209,0.206 | 5 | 0.9432646 |
| 58830 | pa9 | lateral | 109 | 53 | 3.381 | 2.357 | 1.152 | 24.62 | 6.17 | 0.261,0.216,0.222,0.283,0.290 | 5 | 0.8930943 |
| 58933 | pa9 | lateral | 109 | 54 | 3.194 | 2.353 | 0.932 | 21.13 | 18.03 | 0.299,0.272,0.284,0.359,0.294 | 6 | 0.95234466 |
| 10452 | pa7 | lateral | 85 | 206 | 2.941 | 2.267 | 0.307 | 12.40 | 3.34 | 0.085,0.088,0.091,0.094,0.101 | 6 | 0.49223855 |
| 66013 | pa9 | lateral | 110 | 52 | 3.188 | 2.258 | 0.825 | 20.94 | 7.86 | 0.285,0.296,0.262,0.305,0.305 | 5 | 0.95613927 |
| 94056 | pa9 | lateral | 112 | 40 | 3.083 | 2.123 | 0.824 | 25.01 | 6.59 | 0.355,0.377,0.289,0.351,0.446 | 5 | 0.9334179 |
| 58927 | pa9 | lateral | 109 | 56 | 2.501 | 2.090 | 1.150 | 20.26 | 12.65 | 0.245,0.213,0.205,0.217,0.174 | 6 | 0.6405568 |
| 93242 | pa9 | lateral | 112 | 40 | 2.920 | 2.070 | 1.040 | 21.23 | 12.11 | 0.345,0.331,0.340,0.291,0.467 | 5 | 0.97594595 |
| 33469 | pa7 | relay | 102 | 109 | 2.714 | 2.038 | 0.383 | 14.04 | 10.12 | 0.151,0.121,0.170,0.171,0.172 | 6 | 0.8725365 |
| 58831 | pa9 | lateral | 109 | 61 | 2.642 | 2.008 | 1.087 | 21.84 | 3.76 | 0.192,0.211,0.225,0.197,0.170 | 6 | 0.7958423 |
| 58834 | pa9 | lateral | 109 | 56 | 2.580 | 1.958 | 0.559 | 23.36 | 6.97 | 0.152,0.242,0.155,0.187,0.241 | 4 | 0.7631034 |
| 77654 | pa9 | lateral | 111 | 43 | 2.800 | 1.904 | 0.686 | 21.91 | 14.12 | 0.355,0.336,0.422,0.274,0.335 | 5 | 0.96794003 |
| 58918 | pa9 | lateral | 109 | 56 | 2.640 | 1.900 | 0.972 | 14.03 | 7.27 | 0.165,0.205,0.254,0.258,0.221 | 4 | 0.9305635 |
| 58833 | pa9 | lateral | 109 | 59 | 2.415 | 1.849 | 1.333 | 22.37 | 4.58 | 0.188,0.220,0.207,0.155,0.236 | 5 | 0.7655223 |
| 66830 | pa9 | lateral | 110 | 48 | 2.601 | 1.839 | 0.896 | 6.58 | 10.79 | 0.262,0.212,0.268,0.270,0.222 | 5 | 0.76883024 |

**s1007-a200**

| axis | PA | origin | born | phytomers | length m | bare m | longest bare run m | tip height m | tip from trunk m | extension, last 5 cycles m (newest first) | leafed phytomers | tip light |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 26784 | pa7 | lateral | 158 | 252 | 4.575 | 4.116 | 2.734 | 27.19 | 8.48 | 0.110,0.108,0.111,0.131,0.094 | 6 | 0.8828715 |
| 34389 | pa7 | relay | 165 | 207 | 3.121 | 2.802 | 1.439 | 19.57 | 6.13 | 0.089,0.088,0.073,0.089,0.092 | 6 | 0.62640196 |
| 41658 | pa7 | relay | 169 | 177 | 3.038 | 2.557 | 1.567 | 21.42 | 3.87 | 0.129,0.114,0.118,0.095,0.095 | 7 | 0.8171605 |
| 51621 | pa7 | relay | 176 | 147 | 2.392 | 1.877 | 0.218 | 13.28 | 10.11 | 0.114,0.135,0.117,0.112,0.125 | 6 | 0.571766 |
| 55258 | pa7 | relay | 177 | 141 | 2.193 | 1.609 | 0.251 | 18.43 | 6.98 | 0.087,0.101,0.074,0.092,0.093 | 6 | 0.74095243 |
| 50650 | pa7 | relay | 175 | 149 | 1.930 | 1.607 | 0.503 | 19.60 | 5.52 | 0.069,0.069,0.083,0.082,0.078 | 5 | 0.5780214 |
| 80210 | pa7 | relay | 184 | 96 | 2.051 | 1.600 | 0.434 | 23.32 | 11.63 | 0.140,0.117,0.136,0.138,0.140 | 6 | 0.88130516 |
| 107073 | pa9 | lateral | 189 | 55 | 2.006 | 1.594 | 0.799 | 17.41 | 12.47 | 0.099,0.156,0.133,0.186,0.137 | 4 | 0.8418312 |
| 61785 | pa7 | relay | 179 | 125 | 1.803 | 1.481 | 0.495 | 20.35 | 5.11 | 0.091,0.088,0.085,0.086,0.090 | 6 | 0.6946538 |
| 84989 | pa7 | relay | 185 | 91 | 2.019 | 1.479 | 0.235 | 21.78 | 16.25 | 0.136,0.135,0.154,0.129,0.127 | 7 | 0.9184456 |
| 46578 | pa7 | lateral | 173 | 171 | 1.801 | 1.459 | 0.279 | 19.54 | 5.17 | 0.066,0.049,0.071,0.072,0.061 | 7 | 0.5629351 |
| 72809 | pa7 | relay | 182 | 103 | 1.815 | 1.421 | 0.375 | 20.19 | 5.98 | 0.091,0.128,0.111,0.090,0.109 | 5 | 0.8416541 |
| 107081 | pa9 | lateral | 189 | 53 | 1.776 | 1.300 | 0.597 | 20.90 | 11.15 | 0.143,0.137,0.113,0.123,0.185 | 6 | 0.85524786 |
| 86575 | pa7 | relay | 185 | 93 | 1.628 | 1.297 | 0.481 | 25.20 | 13.16 | 0.103,0.104,0.104,0.105,0.105 | 6 | 0.82629013 |
| 103884 | pa9 | lateral | 189 | 50 | 1.625 | 1.285 | 0.850 | 16.26 | 8.76 | 0.108,0.116,0.127,0.141,0.182 | 4 | 0.792985 |
| 108752 | pa9 | lateral | 189 | 55 | 1.513 | 1.232 | 0.767 | 19.25 | 1.05 | 0.108,0.147,0.118,0.150,0.143 | 4 | 0.86853325 |
| 91758 | pa7 | relay | 186 | 88 | 1.588 | 1.215 | 0.662 | 23.49 | 15.04 | 0.122,0.107,0.089,0.126,0.107 | 7 | 0.77283645 |
| 95511 | pa7 | lateral | 187 | 74 | 1.433 | 1.130 | 0.784 | 22.25 | 10.65 | 0.089,0.097,0.124,0.097,0.132 | 5 | 0.59953785 |
| 107072 | pa9 | lateral | 189 | 53 | 1.732 | 1.115 | 0.532 | 16.80 | 11.76 | 0.132,0.145,0.151,0.193,0.177 | 5 | 0.8807669 |
| 71461 | pa7 | relay | 182 | 108 | 1.429 | 1.078 | 0.428 | 19.45 | 7.52 | 0.086,0.076,0.064,0.065,0.078 | 7 | 0.5335625 |

### Table G. Grower bytes of the axes' records on each class of B (MiB)

| tree | class | axis+phytomers+units | draws | rough-layout pencil | successor+working+fate | total |
|---|---|---|---|---|---|---|
| s1001-a80 | L | 78.2 | 34.2 | 28.4 | 7.4 | 148.2 |
| s1001-a80 | B | 43.3 | 11.8 | 5.6 | 2.8 | 63.6 |
| s1001-a80 | D | 105.4 | 39.4 | 29.2 | 10.7 | 184.7 |
| s1001-a80 | dropped by the shed | 78.8 | 38.0 | 46.7 | 17.3 | 180.8 |
| s1001-a120 | L | 161.4 | 70.7 | 58.3 | 15.2 | 305.6 |
| s1001-a120 | B | 103.1 | 28.1 | 13.3 | 6.6 | 151.2 |
| s1001-a120 | D | 235.3 | 87.2 | 63.8 | 23.4 | 409.7 |
| s1001-a120 | dropped by the shed | 143.3 | 60.5 | 59.4 | 22.2 | 285.4 |
| s1001-a200 | L | 217.5 | 95.3 | 77.5 | 20.6 | 410.9 |
| s1001-a200 | B | 132.0 | 36.3 | 17.2 | 8.5 | 194.0 |
| s1001-a200 | D | 331.0 | 119.2 | 84.1 | 30.4 | 564.7 |
| s1001-a200 | dropped by the shed | 277.0 | 127.0 | 151.0 | 55.6 | 610.6 |
| s1001-a300 | L | 181.5 | 79.3 | 63.5 | 17.2 | 341.5 |
| s1001-a300 | B | 102.8 | 29.0 | 13.8 | 6.9 | 152.5 |
| s1001-a300 | D | 288.9 | 100.7 | 68.6 | 24.3 | 482.6 |
| s1001-a300 | dropped by the shed | 189.2 | 75.9 | 70.3 | 25.8 | 361.2 |
| s1007-a120 | L | 354.6 | 156.0 | 133.4 | 33.3 | 677.3 |
| s1007-a120 | B | 264.5 | 72.5 | 36.2 | 18.5 | 391.7 |
| s1007-a120 | D | 475.8 | 187.1 | 148.3 | 56.0 | 867.2 |
| s1007-a120 | dropped by the shed | 468.5 | 226.5 | 275.7 | 106.0 | 1,076.7 |
| s1007-a200 | L | 532.3 | 234.0 | 198.6 | 50.1 | 1,015.0 |
| s1007-a200 | B | 423.6 | 117.0 | 58.4 | 30.1 | 629.0 |
| s1007-a200 | D | 748.1 | 288.7 | 224.6 | 84.3 | 1,345.8 |
| s1007-a200 | dropped by the shed | 795.1 | 372.4 | 441.6 | 169.6 | 1,778.8 |

### Table H (extra). All axes by their own length, by state class: axes / phytomers / leafed phytomers


**s1001-a80** (top tip height above ground 19.25)

| axis length | apex living | bearer | dead standing |
|---|---|---|---|
| <0.1 m | 64,554 / 424,097 / 266,377 | 10,026 / 186,712 / 8,631 | 72,478 / 670,164 / 42,583 |
| 0.1-0.5 | 4,396 / 76,935 / 20,571 | 5,962 / 176,010 / 6,049 | 9,092 / 149,681 / 7,653 |
| 0.5-1 | 321 / 10,977 / 1,618 | 779 / 31,049 / 631 | 343 / 9,674 / 0 |
| 1-2 | 138 / 6,673 / 714 | 434 / 25,925 / 183 | 73 / 4,240 / 0 |
| 2-5 | 15 / 1,403 / 87 | 62 / 4,574 / 11 | 13 / 776 / 0 |
| >=5 m | 1 / 349 / 6 | 2 / 423 / 0 | 0 / 0 / 0 |

**s1001-a120** (top tip height above ground 26.74)

| axis length | apex living | bearer | dead standing |
|---|---|---|---|
| <0.1 m | 137,177 / 926,854 / 565,968 | 26,981 / 534,861 / 21,471 | 164,115 / 1,562,279 / 95,035 |
| 0.1-0.5 | 6,887 / 125,320 / 32,194 | 11,296 / 341,644 / 10,071 | 16,401 / 279,487 / 12,249 |
| 0.5-1 | 478 / 17,223 / 2,400 | 1,639 / 72,336 / 1,064 | 766 / 23,107 / 0 |
| 1-2 | 194 / 8,663 / 986 | 780 / 47,538 / 261 | 164 / 9,354 / 0 |
| 2-5 | 17 / 1,365 / 90 | 215 / 15,760 / 55 | 37 / 2,240 / 0 |
| >=5 m | 1 / 143 / 6 | 5 / 1,338 / 0 | 0 / 0 / 0 |

**s1001-a200** (top tip height above ground 22.93)

| axis length | apex living | bearer | dead standing |
|---|---|---|---|
| <0.1 m | 192,711 / 1,307,181 / 794,047 | 36,863 / 731,985 / 28,584 | 245,270 / 2,465,831 / 164,888 |
| 0.1-0.5 | 6,286 / 144,157 / 29,962 | 13,747 / 428,016 / 11,383 | 9,614 / 223,167 / 1,800 |
| 0.5-1 | 393 / 18,089 / 1,983 | 1,767 / 102,884 / 617 | 396 / 21,708 / 0 |
| 1-2 | 61 / 3,524 / 330 | 426 / 31,535 / 142 | 76 / 4,608 / 0 |
| 2-5 | 5 / 477 / 31 | 65 / 8,905 / 7 | 1 / 62 / 0 |
| >=5 m | 0 / 0 / 0 | 2 / 619 / 0 | 0 / 0 / 0 |

**s1001-a300** (top tip height above ground 29.78)

| axis length | apex living | bearer | dead standing |
|---|---|---|---|
| <0.1 m | 166,884 / 1,145,920 / 686,266 | 34,053 / 612,958 / 25,647 | 216,896 / 2,305,524 / 163,105 |
| 0.1-0.5 | 2,953 / 97,002 / 14,668 | 8,781 / 346,509 / 5,472 | 4,015 / 124,166 / 134 |
| 0.5-1 | 57 / 3,226 / 299 | 530 / 37,837 / 73 | 89 / 5,263 / 0 |
| 1-2 | 5 / 813 / 31 | 129 / 14,553 / 10 | 14 / 1,080 / 0 |
| 2-5 | 0 / 0 / 0 | 23 / 4,629 / 0 | 1 / 41 / 0 |
| >=5 m | 0 / 0 / 0 | 0 / 0 / 0 | 0 / 0 / 0 |

**s1007-a120** (top tip height above ground 25.38)

| axis length | apex living | bearer | dead standing |
|---|---|---|---|
| <0.1 m | 288,365 / 2,016,388 / 1,202,826 | 90,114 / 1,675,601 / 59,261 | 335,618 / 3,043,537 / 154,081 |
| 0.1-0.5 | 12,293 / 230,746 / 57,802 | 21,019 / 628,306 / 18,850 | 24,086 / 436,765 / 15,223 |
| 0.5-1 | 870 / 30,579 / 4,376 | 2,896 / 135,287 / 1,794 | 1,210 / 40,628 / 0 |
| 1-2 | 311 / 14,190 / 1,589 | 1,113 / 69,423 / 344 | 267 / 14,514 / 0 |
| 2-5 | 56 / 3,621 / 290 | 319 / 24,952 / 70 | 61 / 3,668 / 0 |
| >=5 m | 1 / 133 / 6 | 15 / 2,615 / 0 | 0 / 0 / 0 |

**s1007-a200** (top tip height above ground 28.79)

| axis length | apex living | bearer | dead standing |
|---|---|---|---|
| <0.1 m | 447,984 / 3,194,997 / 1,870,087 | 159,307 / 3,076,891 / 93,733 | 547,958 / 5,220,586 / 282,685 |
| 0.1-0.5 | 10,549 / 243,137 / 50,412 | 24,702 / 755,567 / 20,416 | 17,439 / 408,634 / 2,629 |
| 0.5-1 | 591 / 26,874 / 3,029 | 2,715 / 160,010 / 913 | 709 / 38,493 / 0 |
| 1-2 | 60 / 3,661 / 315 | 783 / 57,842 / 148 | 125 / 7,499 / 0 |
| 2-5 | 8 / 1,166 / 48 | 122 / 17,758 / 5 | 8 / 569 / 0 |
| >=5 m | 0 / 0 / 0 | 1 / 168 / 0 | 0 / 0 / 0 |

### Table Ht (extra). The 12 longest axes of each state class


**s1001-a80, class L**

| axis | PA | born | phytomers | length m | tip height m | tip from trunk m | years since apex/subtree ended (-1 living) | applied fade | leafed phytomers | grower open |
|---|---|---|---|---|---|---|---|---|---|---|
| 38 | pa7 | 23 | 349 | 12.211 | 14.96 | 9.23 | -1 | 1.000 | 6 | true |
| 2084 | pa7 | 57 | 135 | 4.384 | 18.45 | 3.14 | -1 | 1.000 | 5 | true |
| 2836 | pa7 | 60 | 124 | 3.913 | 18.16 | 6.85 | -1 | 1.000 | 6 | true |
| 8171 | pa9 | 70 | 51 | 2.950 | 14.99 | 6.46 | -1 | 1.000 | 5 | true |
| 6838 | pa9 | 69 | 55 | 2.778 | 13.21 | 6.24 | -1 | 1.000 | 5 | true |
| 10761 | pa9 | 71 | 45 | 2.680 | 14.32 | 5.83 | -1 | 1.000 | 6 | true |
| 6341 | pa7 | 68 | 76 | 2.554 | 18.68 | 7.77 | -1 | 1.000 | 7 | true |
| 855 | pa7 | 49 | 189 | 2.415 | 8.64 | 3.05 | -1 | 1.000 | 7 | true |
| 1494 | pa7 | 54 | 159 | 2.265 | 9.93 | 1.48 | -1 | 1.000 | 7 | true |
| 1489 | pa7 | 54 | 156 | 2.208 | 9.11 | 2.73 | -1 | 1.000 | 7 | true |
| 6833 | pa9 | 69 | 61 | 2.200 | 15.17 | 4.79 | -1 | 1.000 | 5 | true |
| 8475 | pa7 | 70 | 62 | 2.141 | 18.52 | 2.34 | -1 | 1.000 | 6 | true |

**s1001-a80, class B**

| axis | PA | born | phytomers | length m | tip height m | tip from trunk m | years since apex/subtree ended (-1 living) | applied fade | leafed phytomers | grower open |
|---|---|---|---|---|---|---|---|---|---|---|
| 379 | pa7 | 40 | 225 | 7.908 | 16.79 | 6.71 | 3 | 1.000 | 0 | true |
| 107 | pa7 | 30 | 198 | 7.269 | 13.92 | 5.05 | 17 | 1.000 | 0 | true |
| 569 | pa7 | 44 | 127 | 4.821 | 14.46 | 4.28 | 15 | 1.000 | 0 | true |
| 1706 | pa7 | 55 | 127 | 4.796 | 16.34 | 7.13 | 4 | 1.000 | 0 | true |
| 698 | pa7 | 46 | 144 | 4.596 | 14.31 | 5.62 | 10 | 1.000 | 0 | true |
| 0 | pa2 | 0 | 98 | 4.404 | 4.40 | 0.06 | 68 | 1.000 | 0 | true |
| 451 | pa7 | 42 | 98 | 3.746 | 13.45 | 4.25 | 21 | 1.000 | 0 | true |
| 2530 | pa7 | 59 | 100 | 3.651 | 16.40 | 6.26 | 5 | 1.000 | 0 | true |
| 85 | pa7 | 28 | 104 | 3.448 | 10.82 | 3.04 | 34 | 1.000 | 0 | true |
| 291 | pa6 | 38 | 75 | 3.295 | 13.52 | 3.38 | 30 | 1.000 | 0 | true |
| 2799 | pa9 | 60 | 60 | 3.279 | 14.12 | 5.70 | 8 | 1.000 | 0 | true |
| 39 | pa6 | 23 | 79 | 3.132 | 10.39 | 1.53 | 44 | 1.000 | 0 | true |

**s1001-a80, class D**

| axis | PA | born | phytomers | length m | tip height m | tip from trunk m | years since apex/subtree ended (-1 living) | applied fade | leafed phytomers | grower open |
|---|---|---|---|---|---|---|---|---|---|---|
| 2522 | pa9 | 59 | 58 | 3.280 | 7.12 | 7.42 | 1 | 1.000 | 0 | false |
| 1407 | pa9 | 53 | 60 | 3.016 | 7.68 | 6.15 | 1 | 1.000 | 0 | false |
| 632 | pa9 | 45 | 59 | 2.980 | 9.62 | 5.10 | 1 | 1.000 | 0 | false |
| 3481 | pa9 | 62 | 60 | 2.702 | 13.00 | 1.13 | 0 | 1.000 | 0 | true |
| 256 | pa9 | 37 | 61 | 2.404 | 10.61 | 2.80 | 0 | 1.000 | 0 | true |
| 208 | pa9 | 35 | 60 | 2.214 | 10.76 | 3.41 | 0 | 1.000 | 0 | true |
| 2778 | pa9 | 60 | 56 | 2.202 | 6.56 | 4.59 | 0 | 1.000 | 0 | true |
| 2521 | pa9 | 59 | 61 | 2.174 | 14.32 | 4.83 | 1 | 1.000 | 0 | false |
| 2072 | pa9 | 57 | 62 | 2.170 | 14.39 | 0.54 | 7 | 1.000 | 0 | false |
| 2785 | pa9 | 60 | 59 | 2.103 | 15.02 | 4.40 | 2 | 1.000 | 0 | false |
| 2293 | pa9 | 58 | 60 | 2.102 | 15.28 | 3.92 | 6 | 1.000 | 0 | false |
| 2786 | pa9 | 60 | 61 | 2.041 | 13.52 | 5.81 | 0 | 1.000 | 0 | true |

**s1001-a120, class L**

| axis | PA | born | phytomers | length m | tip height m | tip from trunk m | years since apex/subtree ended (-1 living) | applied fade | leafed phytomers | grower open |
|---|---|---|---|---|---|---|---|---|---|---|
| 8048 | pa7 | 97 | 143 | 5.496 | 24.12 | 6.40 | -1 | 1.000 | 6 | true |
| 10252 | pa7 | 100 | 124 | 4.248 | 25.74 | 1.50 | -1 | 1.000 | 7 | true |
| 3197 | pa7 | 85 | 207 | 3.567 | 13.99 | 2.18 | -1 | 1.000 | 7 | true |
| 3004 | pa7 | 84 | 213 | 3.476 | 13.07 | 3.75 | -1 | 1.000 | 5 | true |
| 28972 | pa9 | 111 | 44 | 2.889 | 21.39 | 5.84 | -1 | 1.000 | 4 | true |
| 20309 | pa9 | 109 | 56 | 2.850 | 22.21 | 6.34 | -1 | 1.000 | 5 | true |
| 36013 | pa9 | 112 | 38 | 2.677 | 17.78 | 14.32 | -1 | 1.000 | 5 | true |
| 5385 | pa7 | 92 | 169 | 2.651 | 14.04 | 2.95 | -1 | 1.000 | 7 | true |
| 23365 | pa9 | 110 | 53 | 2.479 | 9.54 | 9.03 | -1 | 1.000 | 6 | true |
| 18911 | pa7 | 108 | 71 | 2.439 | 24.47 | 12.18 | -1 | 1.000 | 5 | true |
| 20358 | pa7 | 109 | 71 | 2.407 | 25.56 | 5.39 | -1 | 1.000 | 7 | true |
| 29311 | pa9 | 111 | 47 | 2.404 | 17.34 | 8.95 | -1 | 1.000 | 4 | true |

**s1001-a120, class B**

| axis | PA | born | phytomers | length m | tip height m | tip from trunk m | years since apex/subtree ended (-1 living) | applied fade | leafed phytomers | grower open |
|---|---|---|---|---|---|---|---|---|---|---|
| 26 | pa7 | 23 | 367 | 12.883 | 15.28 | 9.81 | 37 | 1.000 | 0 | true |
| 1587 | pa7 | 74 | 234 | 7.984 | 25.72 | 4.78 | 6 | 1.000 | 0 | true |
| 184 | pa7 | 40 | 225 | 7.908 | 16.76 | 6.73 | 43 | 1.000 | 0 | true |
| 70 | pa7 | 30 | 198 | 7.269 | 13.92 | 5.06 | 57 | 1.000 | 0 | true |
| 514 | pa7 | 56 | 314 | 5.163 | 13.15 | 2.05 | 13 | 1.000 | 0 | true |
| 663 | pa7 | 60 | 156 | 4.920 | 18.85 | 7.38 | 35 | 1.000 | 0 | true |
| 246 | pa7 | 44 | 127 | 4.821 | 14.43 | 4.34 | 55 | 1.000 | 0 | true |
| 494 | pa7 | 55 | 127 | 4.796 | 16.32 | 7.14 | 44 | 1.000 | 0 | true |
| 559 | pa7 | 57 | 145 | 4.717 | 18.67 | 3.02 | 38 | 1.000 | 0 | true |
| 277 | pa7 | 46 | 144 | 4.596 | 14.29 | 5.66 | 50 | 1.000 | 0 | true |
| 0 | pa2 | 0 | 98 | 4.404 | 4.40 | 0.06 | 108 | 1.000 | 0 | true |
| 5994 | pa7 | 93 | 118 | 4.299 | 22.99 | 6.28 | 8 | 1.000 | 0 | true |

**s1001-a120, class D**

| axis | PA | born | phytomers | length m | tip height m | tip from trunk m | years since apex/subtree ended (-1 living) | applied fade | leafed phytomers | grower open |
|---|---|---|---|---|---|---|---|---|---|---|
| 9319 | pa9 | 99 | 58 | 3.590 | 18.09 | 12.45 | 2 | 1.000 | 0 | false |
| 5128 | pa9 | 91 | 58 | 3.108 | 9.63 | 13.13 | 1 | 1.000 | 0 | false |
| 9366 | pa9 | 99 | 65 | 3.003 | 20.59 | 11.32 | 1 | 1.000 | 0 | false |
| 10122 | pa9 | 100 | 61 | 2.991 | 7.88 | 12.11 | 1 | 1.000 | 0 | false |
| 9328 | pa9 | 99 | 62 | 2.987 | 12.66 | 6.92 | 2 | 1.000 | 0 | false |
| 7982 | pa9 | 97 | 59 | 2.940 | 16.47 | 10.83 | 7 | 1.000 | 0 | false |
| 5940 | pa9 | 93 | 60 | 2.720 | 16.38 | 8.25 | 1 | 1.000 | 0 | false |
| 10149 | pa9 | 100 | 61 | 2.719 | 21.34 | 9.31 | 2 | 1.000 | 0 | false |
| 9313 | pa9 | 99 | 60 | 2.711 | 17.01 | 6.22 | 1 | 1.000 | 0 | false |
| 9333 | pa9 | 99 | 58 | 2.683 | 18.08 | 7.62 | 1 | 1.000 | 0 | false |
| 7409 | pa9 | 96 | 61 | 2.571 | 20.71 | 10.47 | 1 | 1.000 | 0 | false |
| 8631 | pa9 | 98 | 62 | 2.555 | 16.34 | 7.88 | 2 | 1.000 | 0 | false |

**s1001-a200, class L**

| axis | PA | born | phytomers | length m | tip height m | tip from trunk m | years since apex/subtree ended (-1 living) | applied fade | leafed phytomers | grower open |
|---|---|---|---|---|---|---|---|---|---|---|
| 30006 | pa9 | 190 | 49 | 2.264 | 20.60 | 2.22 | -1 | 1.000 | 5 | true |
| 18340 | pa7 | 181 | 112 | 2.083 | 20.54 | 8.14 | -1 | 1.000 | 7 | true |
| 19394 | pa6 | 182 | 105 | 2.039 | 20.70 | 8.45 | -1 | 1.000 | 5 | true |
| 19752 | pa7 | 182 | 104 | 2.033 | 21.70 | 10.72 | -1 | 1.000 | 7 | true |
| 19377 | pa7 | 182 | 107 | 2.032 | 21.55 | 7.83 | -1 | 1.000 | 7 | true |
| 28875 | pa9 | 189 | 51 | 1.908 | 20.15 | 5.53 | -1 | 1.000 | 5 | true |
| 28872 | pa9 | 189 | 54 | 1.894 | 19.94 | 4.97 | -1 | 1.000 | 4 | true |
| 22059 | pa7 | 184 | 92 | 1.755 | 21.85 | 10.50 | -1 | 1.000 | 6 | true |
| 24636 | pa7 | 186 | 85 | 1.657 | 22.90 | 7.38 | -1 | 1.000 | 7 | true |
| 32768 | pa9 | 190 | 52 | 1.650 | 20.17 | 3.52 | -1 | 1.000 | 5 | true |
| 28874 | pa9 | 189 | 52 | 1.554 | 20.83 | 7.22 | -1 | 1.000 | 5 | true |
| 38756 | pa9 | 191 | 46 | 1.549 | 20.88 | 3.67 | -1 | 1.000 | 4 | true |

**s1001-a200, class B**

| axis | PA | born | phytomers | length m | tip height m | tip from trunk m | years since apex/subtree ended (-1 living) | applied fade | leafed phytomers | grower open |
|---|---|---|---|---|---|---|---|---|---|---|
| 408 | pa7 | 84 | 305 | 5.165 | 14.57 | 4.47 | 65 | 1.000 | 0 | true |
| 145 | pa7 | 56 | 314 | 5.163 | 13.15 | 2.04 | 93 | 1.000 | 0 | true |
| 6084 | pa7 | 152 | 262 | 4.601 | 20.55 | 5.98 | 4 | 1.000 | 0 | true |
| 0 | pa2 | 0 | 98 | 4.404 | 4.40 | 0.07 | 188 | 1.000 | 0 | true |
| 429 | pa7 | 85 | 245 | 4.234 | 14.63 | 2.17 | 74 | 1.000 | 0 | true |
| 1401 | pa7 | 110 | 230 | 4.122 | 15.46 | 3.84 | 52 | 1.000 | 0 | true |
| 1910 | pa7 | 118 | 252 | 3.777 | 15.62 | 6.70 | 40 | 1.000 | 0 | true |
| 7913 | pa7 | 160 | 214 | 3.599 | 20.80 | 7.27 | 5 | 1.000 | 0 | true |
| 120 | pa7 | 51 | 315 | 3.463 | 10.47 | 2.75 | 97 | 1.000 | 0 | true |
| 11823 | pa7 | 171 | 173 | 3.429 | 21.75 | 10.21 | 0 | 1.000 | 7 | true |
| 629 | pa7 | 92 | 202 | 3.236 | 14.55 | 2.99 | 75 | 1.000 | 0 | true |
| 5519 | pa7 | 149 | 184 | 3.210 | 19.72 | 6.50 | 20 | 1.000 | 0 | true |

**s1001-a200, class D**

| axis | PA | born | phytomers | length m | tip height m | tip from trunk m | years since apex/subtree ended (-1 living) | applied fade | leafed phytomers | grower open |
|---|---|---|---|---|---|---|---|---|---|---|
| 17581 | pa9 | 180 | 62 | 2.124 | 14.90 | 10.49 | 0 | 1.000 | 0 | true |
| 15626 | pa9 | 178 | 59 | 1.937 | 11.77 | 6.56 | 2 | 1.000 | 0 | false |
| 12230 | pa9 | 171 | 60 | 1.927 | 18.50 | 6.53 | 2 | 1.000 | 0 | false |
| 16410 | pa9 | 179 | 59 | 1.819 | 16.27 | 7.48 | 1 | 1.000 | 0 | false |
| 14987 | pa9 | 177 | 62 | 1.771 | 17.95 | 9.64 | 7 | 1.000 | 0 | false |
| 13141 | pa9 | 174 | 61 | 1.769 | 10.79 | 6.87 | 2 | 1.000 | 0 | false |
| 15029 | pa9 | 177 | 61 | 1.716 | 17.12 | 3.42 | 7 | 1.000 | 0 | false |
| 10080 | pa9 | 167 | 61 | 1.711 | 11.06 | 5.99 | 2 | 1.000 | 0 | false |
| 7650 | pa9 | 159 | 60 | 1.691 | 10.79 | 4.71 | 2 | 1.000 | 0 | false |
| 15758 | pa9 | 178 | 60 | 1.675 | 18.19 | 6.22 | 2 | 1.000 | 0 | false |
| 9499 | pa9 | 165 | 65 | 1.658 | 16.37 | 3.79 | 8 | 1.000 | 0 | false |
| 17541 | pa9 | 180 | 59 | 1.588 | 18.37 | 4.18 | 1 | 1.000 | 0 | false |

**s1001-a300, class L**

| axis | PA | born | phytomers | length m | tip height m | tip from trunk m | years since apex/subtree ended (-1 living) | applied fade | leafed phytomers | grower open |
|---|---|---|---|---|---|---|---|---|---|---|
| 11182 | pa7 | 260 | 241 | 1.769 | 17.70 | 9.29 | -1 | 1.000 | 6 | true |
| 13714 | pa7 | 270 | 176 | 1.484 | 18.85 | 5.66 | -1 | 1.000 | 6 | true |
| 15545 | pa7 | 275 | 151 | 1.398 | 19.57 | 4.13 | -1 | 1.000 | 7 | true |
| 17017 | pa7 | 278 | 129 | 1.279 | 18.86 | 4.80 | -1 | 1.000 | 6 | true |
| 18398 | pa6 | 280 | 116 | 1.110 | 18.06 | 8.90 | -1 | 1.000 | 6 | true |
| 28542 | pa9 | 290 | 48 | 0.976 | 17.85 | 10.38 | -1 | 1.000 | 6 | true |
| 31684 | pa9 | 291 | 48 | 0.936 | 14.50 | 3.48 | -1 | 1.000 | 6 | true |
| 13121 | pa7 | 268 | 193 | 0.927 | 18.06 | 2.72 | -1 | 1.000 | 6 | true |
| 45412 | pa9 | 293 | 39 | 0.900 | 15.20 | 6.44 | -1 | 1.000 | 6 | true |
| 34078 | pa9 | 291 | 46 | 0.898 | 27.29 | 5.67 | -1 | 1.000 | 5 | true |
| 14998 | pa7 | 274 | 155 | 0.879 | 15.62 | 0.98 | -1 | 1.000 | 7 | true |
| 26756 | pa9 | 289 | 53 | 0.839 | 20.52 | 5.11 | -1 | 1.000 | 5 | true |

**s1001-a300, class B**

| axis | PA | born | phytomers | length m | tip height m | tip from trunk m | years since apex/subtree ended (-1 living) | applied fade | leafed phytomers | grower open |
|---|---|---|---|---|---|---|---|---|---|---|
| 1626 | pa7 | 152 | 262 | 4.601 | 20.58 | 5.97 | 104 | 1.000 | 0 | true |
| 0 | pa2 | 0 | 98 | 4.404 | 4.40 | 0.07 | 288 | 1.000 | 0 | true |
| 3328 | pa7 | 186 | 192 | 3.753 | 24.87 | 7.72 | 81 | 1.000 | 0 | true |
| 1973 | pa7 | 160 | 214 | 3.599 | 20.77 | 7.25 | 105 | 1.000 | 0 | true |
| 1972 | pa7 | 160 | 170 | 3.040 | 20.22 | 7.59 | 112 | 1.000 | 0 | true |
| 145 | pa7 | 80 | 220 | 2.948 | 10.08 | 5.26 | 184 | 1.000 | 0 | true |
| 65 | pa7 | 49 | 224 | 2.808 | 8.88 | 3.34 | 214 | 1.000 | 0 | true |
| 7494 | pa7 | 236 | 176 | 2.771 | 25.07 | 11.31 | 35 | 1.000 | 0 | true |
| 2212 | pa7 | 165 | 149 | 2.761 | 20.53 | 4.25 | 111 | 1.000 | 0 | true |
| 3087 | pa6 | 182 | 129 | 2.533 | 21.02 | 8.48 | 96 | 1.000 | 0 | true |
| 10648 | pa7 | 257 | 243 | 2.391 | 19.86 | 4.68 | 3 | 1.000 | 0 | true |
| 11958 | pa7 | 263 | 223 | 2.380 | 20.72 | 6.17 | 1 | 1.000 | 0 | true |

**s1001-a300, class D**

| axis | PA | born | phytomers | length m | tip height m | tip from trunk m | years since apex/subtree ended (-1 living) | applied fade | leafed phytomers | grower open |
|---|---|---|---|---|---|---|---|---|---|---|
| 7 | pa4 | 14 | 41 | 2.050 | 7.08 | 0.08 | 277 | 1.000 | 0 | false |
| 13538 | pa9 | 270 | 61 | 1.739 | 14.64 | 5.26 | 0 | 1.000 | 0 | true |
| 17889 | pa9 | 280 | 61 | 1.718 | 15.77 | 5.13 | 0 | 1.000 | 0 | true |
| 3486 | pa7 | 189 | 89 | 1.685 | 19.64 | 7.22 | 7 | 1.000 | 0 | false |
| 12203 | pa9 | 265 | 62 | 1.503 | 14.69 | 4.08 | 0 | 1.000 | 0 | true |
| 2271 | pa7 | 166 | 142 | 1.436 | 20.37 | 7.22 | 111 | 1.000 | 0 | false |
| 124 | pa7 | 73 | 164 | 1.325 | 9.71 | 2.73 | 7 | 1.000 | 0 | false |
| 11504 | pa9 | 262 | 59 | 1.324 | 14.62 | 5.37 | 0 | 1.000 | 0 | true |
| 1438 | pa7 | 147 | 73 | 1.305 | 17.64 | 4.95 | 7 | 1.000 | 0 | false |
| 2868 | pa7 | 178 | 67 | 1.269 | 18.95 | 6.28 | 7 | 1.000 | 0 | false |
| 1919 | pa7 | 159 | 67 | 1.244 | 18.39 | 4.99 | 7 | 1.000 | 0 | false |
| 17255 | pa9 | 279 | 55 | 1.196 | 16.02 | 4.31 | 1 | 1.000 | 0 | false |

**s1007-a120, class L**

| axis | PA | born | phytomers | length m | tip height m | tip from trunk m | years since apex/subtree ended (-1 living) | applied fade | leafed phytomers | grower open |
|---|---|---|---|---|---|---|---|---|---|---|
| 25427 | pa7 | 98 | 133 | 5.349 | 23.19 | 15.97 | -1 | 1.000 | 6 | true |
| 32010 | pa7 | 101 | 112 | 4.696 | 23.15 | 16.83 | -1 | 1.000 | 7 | true |
| 32018 | pa7 | 101 | 117 | 4.563 | 23.44 | 16.68 | -1 | 1.000 | 6 | true |
| 17491 | pa7 | 93 | 159 | 3.560 | 15.85 | 8.54 | -1 | 1.000 | 6 | true |
| 37514 | pa7 | 103 | 101 | 3.472 | 24.51 | 13.08 | -1 | 1.000 | 5 | true |
| 58830 | pa9 | 109 | 53 | 3.381 | 24.62 | 6.17 | -1 | 1.000 | 5 | true |
| 58933 | pa9 | 109 | 54 | 3.194 | 21.13 | 18.03 | -1 | 1.000 | 6 | true |
| 66013 | pa9 | 110 | 52 | 3.188 | 20.94 | 7.86 | -1 | 1.000 | 5 | true |
| 6618 | pa7 | 78 | 250 | 3.106 | 12.14 | 1.55 | -1 | 1.000 | 6 | true |
| 94056 | pa9 | 112 | 40 | 3.083 | 25.01 | 6.59 | -1 | 1.000 | 5 | true |
| 10452 | pa7 | 85 | 206 | 2.941 | 12.40 | 3.34 | -1 | 1.000 | 6 | true |
| 93242 | pa9 | 112 | 40 | 2.920 | 21.23 | 12.11 | -1 | 1.000 | 5 | true |

**s1007-a120, class B**

| axis | PA | born | phytomers | length m | tip height m | tip from trunk m | years since apex/subtree ended (-1 living) | applied fade | leafed phytomers | grower open |
|---|---|---|---|---|---|---|---|---|---|---|
| 697 | pa7 | 47 | 284 | 11.405 | 18.28 | 11.96 | 27 | 1.000 | 0 | true |
| 2667 | pa7 | 64 | 217 | 7.959 | 20.38 | 11.58 | 19 | 1.000 | 0 | true |
| 3733 | pa7 | 69 | 185 | 7.003 | 21.23 | 7.56 | 21 | 1.000 | 0 | true |
| 6352 | pa7 | 77 | 154 | 6.611 | 18.84 | 15.13 | 17 | 1.000 | 0 | true |
| 6760 | pa7 | 78 | 187 | 6.572 | 19.43 | 14.19 | 11 | 1.000 | 0 | true |
| 7209 | pa7 | 79 | 184 | 6.460 | 18.29 | 14.47 | 10 | 1.000 | 0 | true |
| 97 | pa6 | 28 | 168 | 6.352 | 13.41 | 5.16 | 64 | 1.000 | 0 | true |
| 765 | pa7 | 48 | 148 | 6.125 | 16.74 | 5.36 | 46 | 1.000 | 0 | true |
| 989 | pa7 | 51 | 276 | 5.991 | 13.05 | 5.90 | 23 | 1.000 | 0 | true |
| 15758 | pa7 | 91 | 163 | 5.927 | 24.62 | 11.90 | 2 | 1.000 | 0 | true |
| 3992 | pa7 | 70 | 181 | 5.750 | 18.82 | 7.45 | 20 | 1.000 | 0 | true |
| 15770 | pa7 | 91 | 136 | 5.633 | 21.73 | 13.11 | 7 | 1.000 | 0 | true |

**s1007-a120, class D**

| axis | PA | born | phytomers | length m | tip height m | tip from trunk m | years since apex/subtree ended (-1 living) | applied fade | leafed phytomers | grower open |
|---|---|---|---|---|---|---|---|---|---|---|
| 29461 | pa9 | 100 | 61 | 3.821 | 16.73 | 9.97 | 0 | 1.000 | 0 | true |
| 34682 | pa9 | 102 | 61 | 3.542 | 19.89 | 6.37 | 0 | 1.000 | 0 | true |
| 23534 | pa9 | 97 | 57 | 3.487 | 14.51 | 10.30 | 7 | 1.000 | 0 | false |
| 25389 | pa9 | 98 | 61 | 3.392 | 21.71 | 4.95 | 2 | 1.000 | 0 | false |
| 23555 | pa9 | 97 | 61 | 3.371 | 11.43 | 5.61 | 10 | 1.000 | 0 | false |
| 27416 | pa9 | 99 | 59 | 3.251 | 19.47 | 7.55 | 1 | 1.000 | 0 | false |
| 29599 | pa9 | 100 | 62 | 2.988 | 21.39 | 11.69 | 0 | 1.000 | 0 | true |
| 23553 | pa9 | 97 | 63 | 2.928 | 9.37 | 9.73 | 7 | 1.000 | 0 | false |
| 27405 | pa9 | 99 | 58 | 2.926 | 17.64 | 8.13 | 1 | 1.000 | 0 | false |
| 27325 | pa9 | 99 | 58 | 2.925 | 17.49 | 11.39 | 0 | 1.000 | 0 | true |
| 28054 | pa9 | 100 | 61 | 2.873 | 13.40 | 5.43 | 0 | 1.000 | 0 | true |
| 23594 | pa9 | 97 | 62 | 2.868 | 22.46 | 6.59 | 10 | 1.000 | 0 | false |

**s1007-a200, class L**

| axis | PA | born | phytomers | length m | tip height m | tip from trunk m | years since apex/subtree ended (-1 living) | applied fade | leafed phytomers | grower open |
|---|---|---|---|---|---|---|---|---|---|---|
| 26784 | pa7 | 158 | 252 | 4.575 | 27.19 | 8.48 | -1 | 1.000 | 6 | true |
| 34389 | pa7 | 165 | 207 | 3.121 | 19.57 | 6.13 | -1 | 1.000 | 6 | true |
| 41658 | pa7 | 169 | 177 | 3.038 | 21.42 | 3.87 | -1 | 1.000 | 7 | true |
| 51621 | pa7 | 176 | 147 | 2.392 | 13.28 | 10.11 | -1 | 1.000 | 6 | true |
| 55258 | pa7 | 177 | 141 | 2.193 | 18.43 | 6.98 | -1 | 1.000 | 6 | true |
| 80210 | pa7 | 184 | 96 | 2.051 | 23.32 | 11.63 | -1 | 1.000 | 6 | true |
| 84989 | pa7 | 185 | 91 | 2.019 | 21.78 | 16.25 | -1 | 1.000 | 7 | true |
| 107073 | pa9 | 189 | 55 | 2.006 | 17.41 | 12.47 | -1 | 1.000 | 4 | true |
| 50650 | pa7 | 175 | 149 | 1.930 | 19.60 | 5.52 | -1 | 1.000 | 5 | true |
| 72809 | pa7 | 182 | 103 | 1.815 | 20.19 | 5.98 | -1 | 1.000 | 5 | true |
| 61785 | pa7 | 179 | 125 | 1.803 | 20.35 | 5.11 | -1 | 1.000 | 6 | true |
| 46578 | pa7 | 173 | 171 | 1.801 | 19.54 | 5.17 | -1 | 1.000 | 7 | true |

**s1007-a200, class B**

| axis | PA | born | phytomers | length m | tip height m | tip from trunk m | years since apex/subtree ended (-1 living) | applied fade | leafed phytomers | grower open |
|---|---|---|---|---|---|---|---|---|---|---|
| 70 | pa6 | 28 | 168 | 6.352 | 13.36 | 5.23 | 144 | 1.000 | 0 | true |
| 1154 | pa7 | 77 | 213 | 4.956 | 19.38 | 5.04 | 87 | 1.000 | 0 | true |
| 2314 | pa7 | 93 | 211 | 4.745 | 16.87 | 9.14 | 71 | 1.000 | 0 | true |
| 14693 | pa7 | 141 | 196 | 4.639 | 19.85 | 12.56 | 27 | 1.000 | 0 | true |
| 6294 | pa7 | 117 | 220 | 4.624 | 18.10 | 10.52 | 47 | 1.000 | 0 | true |
| 3382 | pa7 | 102 | 184 | 4.562 | 15.28 | 11.29 | 68 | 1.000 | 0 | true |
| 14454 | pa7 | 141 | 223 | 4.276 | 15.43 | 9.14 | 21 | 1.000 | 0 | true |
| 0 | pa2 | 0 | 94 | 4.187 | 4.19 | 0.05 | 188 | 1.000 | 0 | true |
| 106 | pa7 | 34 | 266 | 4.126 | 9.64 | 1.06 | 121 | 1.000 | 0 | true |
| 4984 | pa7 | 111 | 183 | 3.781 | 15.21 | 11.08 | 59 | 1.000 | 0 | true |
| 25523 | pa7 | 157 | 170 | 3.675 | 20.45 | 16.01 | 16 | 1.000 | 0 | true |
| 533 | pa7 | 60 | 153 | 3.658 | 16.02 | 1.67 | 115 | 1.000 | 0 | true |

**s1007-a200, class D**

| axis | PA | born | phytomers | length m | tip height m | tip from trunk m | years since apex/subtree ended (-1 living) | applied fade | leafed phytomers | grower open |
|---|---|---|---|---|---|---|---|---|---|---|
| 15996 | pa9 | 143 | 56 | 2.687 | 8.41 | 10.14 | 3 | 1.000 | 0 | false |
| 26770 | pa9 | 158 | 60 | 2.647 | 7.45 | 12.34 | 3 | 1.000 | 0 | false |
| 11663 | pa9 | 134 | 61 | 2.442 | 10.00 | 8.43 | 3 | 1.000 | 0 | false |
| 41661 | pa9 | 170 | 63 | 2.367 | 6.00 | 13.04 | 3 | 1.000 | 0 | false |
| 49137 | pa7 | 174 | 148 | 2.363 | 24.32 | 14.20 | 1 | 1.000 | 0 | true |
| 23291 | pa9 | 154 | 56 | 2.220 | 8.53 | 10.52 | 3 | 1.000 | 0 | false |
| 79 | pa7 | 29 | 68 | 2.061 | 9.68 | 3.30 | 3 | 1.000 | 0 | false |
| 6787 | pa9 | 119 | 57 | 2.029 | 11.22 | 10.24 | 3 | 1.000 | 0 | false |
| 3719 | pa9 | 104 | 58 | 1.989 | 9.63 | 9.77 | 3 | 1.000 | 0 | false |
| 67533 | pa9 | 181 | 62 | 1.982 | 22.70 | 13.42 | 0 | 1.000 | 0 | true |
| 63932 | pa9 | 180 | 62 | 1.942 | 19.57 | 7.36 | 0 | 1.000 | 0 | true |
| 65187 | pa9 | 181 | 67 | 1.866 | 13.36 | 2.75 | 0 | 1.000 | 0 | true |

### Table I (extra). Leaf by PA row: leafed phytomers, leaf m2, mean scale of the leafed, the row's leaf_area (m2 at scale 1)


**s1001-a80**

| PA | leafed phytomers | leaf m2 | mean scale of leafed | PA leaf_area m2 | leaf m2 per leafed phytomer (e-3) |
|---|---|---|---|---|---|
| 6 limb | 36 | 0.0 | 0.5014 | 0.00181 | 0.908 |
| 7 bough | 416 | 0.4 | 0.5468 | 0.00181 | 0.990 |
| 9 branch | 21,767 | 17.5 | 0.4440 | 0.00181 | 0.804 |
| 12 shoot | 2,114 | 2.2 | 0.5846 | 0.00181 | 1.058 |
| 13 twig | 78,814 | 65.9 | 0.4620 | 0.00181 | 0.836 |
| 14 short | 251,967 | 260.4 | 0.5710 | 0.00181 | 1.033 |

**s1001-a120**

| PA | leafed phytomers | leaf m2 | mean scale of leafed | PA leaf_area m2 | leaf m2 per leafed phytomer (e-3) |
|---|---|---|---|---|---|
| 6 limb | 60 | 0.1 | 0.4634 | 0.00181 | 0.838 |
| 7 bough | 798 | 0.8 | 0.5344 | 0.00181 | 0.967 |
| 9 branch | 43,335 | 28.3 | 0.3608 | 0.00181 | 0.653 |
| 12 shoot | 4,653 | 4.0 | 0.4747 | 0.00181 | 0.859 |
| 13 twig | 160,613 | 107.8 | 0.3708 | 0.00181 | 0.671 |
| 14 short | 532,391 | 440.9 | 0.4575 | 0.00181 | 0.828 |

**s1001-a200**

| PA | leafed phytomers | leaf m2 | mean scale of leafed | PA leaf_area m2 | leaf m2 per leafed phytomer (e-3) |
|---|---|---|---|---|---|
| 6 limb | 65 | 0.0 | 0.3252 | 0.00181 | 0.589 |
| 7 bough | 1,210 | 0.8 | 0.3672 | 0.00181 | 0.665 |
| 9 branch | 52,838 | 26.7 | 0.2792 | 0.00181 | 0.505 |
| 12 shoot | 6,111 | 3.5 | 0.3200 | 0.00181 | 0.579 |
| 13 twig | 203,632 | 102.3 | 0.2776 | 0.00181 | 0.502 |
| 14 short | 769,918 | 420.5 | 0.3017 | 0.00181 | 0.546 |

**s1001-a300**

| PA | leafed phytomers | leaf m2 | mean scale of leafed | PA leaf_area m2 | leaf m2 per leafed phytomer (e-3) |
|---|---|---|---|---|---|
| 6 limb | 58 | 0.0 | 0.2389 | 0.00181 | 0.433 |
| 7 bough | 1,045 | 0.5 | 0.2641 | 0.00181 | 0.478 |
| 9 branch | 39,339 | 15.2 | 0.2129 | 0.00181 | 0.385 |
| 12 shoot | 5,103 | 2.0 | 0.2126 | 0.00181 | 0.385 |
| 13 twig | 158,166 | 58.8 | 0.2054 | 0.00181 | 0.372 |
| 14 short | 691,994 | 255.0 | 0.2036 | 0.00181 | 0.368 |

**s1007-a120**

| PA | leafed phytomers | leaf m2 | mean scale of leafed | PA leaf_area m2 | leaf m2 per leafed phytomer (e-3) |
|---|---|---|---|---|---|
| 6 limb | 206 | 0.1 | 0.3137 | 0.00181 | 0.568 |
| 7 bough | 2,267 | 1.7 | 0.4037 | 0.00181 | 0.731 |
| 9 branch | 127,496 | 57.4 | 0.2488 | 0.00181 | 0.450 |
| 12 shoot | 9,049 | 6.5 | 0.3974 | 0.00181 | 0.719 |
| 13 twig | 396,817 | 198.5 | 0.2764 | 0.00181 | 0.500 |
| 14 short | 980,677 | 695.3 | 0.3917 | 0.00181 | 0.709 |

**s1007-a200**

| PA | leafed phytomers | leaf m2 | mean scale of leafed | PA leaf_area m2 | leaf m2 per leafed phytomer (e-3) |
|---|---|---|---|---|---|
| 6 limb | 208 | 0.1 | 0.2062 | 0.00181 | 0.373 |
| 7 bough | 3,381 | 1.6 | 0.2671 | 0.00181 | 0.483 |
| 9 branch | 180,412 | 49.3 | 0.1509 | 0.00181 | 0.273 |
| 12 shoot | 13,951 | 5.9 | 0.2351 | 0.00181 | 0.425 |
| 13 twig | 579,297 | 178.0 | 0.1698 | 0.00181 | 0.307 |
| 14 short | 1,547,171 | 679.8 | 0.2427 | 0.00181 | 0.439 |

### Leafless phytomers (phytomers minus leafed) by class and by class x PA, largest first


- s1001-a80: leafless 1,424,548 of 1,779,662 (80.0%); by class D 784,299, B 409,188, L 231,061; by class x PA D short 568,057, B branch 238,620, D twig 171,702, B twig 156,367, L twig 89,110; leaf m2 per leafed phytomer 0.976 e-3

- s1001-a120: leafless 3,227,662 of 3,969,512 (81.3%); by class D 1,769,183, B 980,555, L 477,924; by class x PA D short 1,248,040, B branch 599,420, D twig 385,023, B twig 335,384, L twig 182,468; leaf m2 per leafed phytomer 0.784 e-3

- s1001-a200: leafless 4,458,974 of 5,492,748 (81.2%); by class D 2,548,688, B 1,263,211, L 647,075; by class x PA D short 1,830,004, B branch 716,325, D twig 509,918, B twig 435,940, L short 241,609; leaf m2 per leafed phytomer 0.536 e-3

- s1001-a300: leafless 3,803,816 of 4,699,521 (80.9%); by class D 2,272,835, B 985,284, L 545,697; by class x PA D short 1,683,757, B branch 490,675, D twig 410,957, B twig 351,918, L short 223,838; leaf m2 per leafed phytomer 0.370 e-3

- s1007-a120: leafless 6,854,441 of 8,370,953 (81.9%); by class D 3,369,808, B 2,455,865, L 1,028,768; by class x PA D short 2,211,448, B branch 1,652,928, D twig 851,908, B twig 690,594, L twig 437,638; leaf m2 per leafed phytomer 0.633 e-3

- s1007-a200: leafless 10,889,432 of 13,213,852 (82.4%); by class D 5,390,467, B 3,953,021, L 1,545,944; by class x PA D short 3,567,174, B branch 2,642,275, D twig 1,313,665, B twig 1,027,865, L twig 642,562; leaf m2 per leafed phytomer 0.394 e-3

## What the numbers say (counted facts; no verdicts)

1. **The leafless share of the stem units does not grow with age; the leaf per leafed unit shrinks.** Leafed phytomers are 20.0%, 18.7%, 18.8%, 19.1% of the phytomers of seed 1001 at 80, 120, 200, 300 (1007: 18.1%, 17.6% at 120, 200); so 80 to 82% of the phytomers are leafless at every age. What rises is phytomers per m2 of leaf, 5,136, 6,823, 9,917, 14,180 (1001) and 8,724, 14,446 (1007), because the leaf per leafed phytomer falls 0.98, 0.78, 0.54, 0.37 e-3 m2 (1001). The PA row's `leaf_area` is one value (0.00181 m2) for every PA that bears leaf, so this is the leafed phytomers' mean `scale` falling with age in every PA (table I: short 0.57, 0.46, 0.30, 0.20; twig 0.46, 0.37, 0.28, 0.21; branch 0.44, 0.36, 0.28, 0.21), not a change of the mix of PAs. Why the scale falls (light, vigour, base scale, unit size) is not read here.
2. **Where the leafless phytomers are** (table B, and the last list): dead standing wood (D) is the largest class at every age: 784,299 of 1,424,548 leafless at 80, 2,548,688 of 4,458,974 at 200, 2,272,835 of 3,803,816 at 300 (1001); 5,390,467 of 10,889,432 at 200 (1007). Bearers (B) are second, living-apex axes (L) third. D is 42 to 52% of all phytomers.
3. **What the dead standing wood is** (tables B, C, H): 96.4 to 98.0% of D phytomers sit on axes whose subtree last lived 0, 1 or 2 years ago, 74 to 88% at applied fade 1 (not yet fading); PA rows 14 (short) and 13 (twig) hold most of it, with 9 (branch) next (at 1001 age 200: short 1,996,662, twig 509,934, branch 140,308 of 2,715,376). The preset's `shedding` delays are short 2, twig 2, shoot 2, branch 3, bough 4, limb 5, so this is wood waiting out its delay. All D axes are short: the longest D axis is 2.1 m at age 200 (1001; 2.7 m in 1007), and D axes of 0.5 m or more are 396 + 76 + 1 at 200 in 1001 (table H), none of them with a leaf. Of the 353,309 D axes at 200 (1001), 97,952 have no phytomer (their apex ended before it grew one) and 245,270 of the other 255,357 are under 0.1 m.
4. **Bearers** (B) hold 22 to 31% of the phytomers and carry 3.5 to 5.3% of the leafed phytomers; by phytomers most are branch, twig and bough axes whose own apex ended and which carry living laterals; the 12 longest B axes of each tree are 2.4 to 12.9 m (table Ht), mostly bough (PA 7), with the trunk (PA 2, 98 phytomers, 4.4 m, apex ended 188 years before the 200-year tree) among them.
5. **Long bare living shoots ("whips") are not in the settled structure's living-apex axes at old ages.** The longest living-apex axis is 2.3 m at 200 and 1.8 m at 300 in seed 1001 (4.6 m at 200 in 1007), against 12.2 m at 80 and 5.5 m at 120; no living-apex axis has more than 1.78 m (1001 at 200) or 4.1 m (1007 at 200) of bare wood (table F), and the longest bare run is 2.7 m (1007 at 200, 1.3 m or less in seed 1001). Living-apex axes with 6 to 20 leafed phytomers are 0.6 to 0.9% of them and none has over 20. The bare wood of the living-apex axes is 41 to 42% of their length (1,651 m of 3,995 m at 200 in 1001). Table H counts all axes by length and class; table Ht lists the 12 longest per class. The stills' whips are therefore either not axes of this settled structure at this length (what the drawing makes of the structure was not read) or are bearer axes (class B, 4 to 13 m, which carry living laterals and so are not bare of laterals) seen without their leaves; which of the two was not measured.
6. **What the shed removes** (table A, G): held axes before the shed are 1.44x the kept axes at 120 and 1.85x at 200 in 1001 (835,957 to 578,853; 1,403,043 to 757,677), held phytomers 1.23x and 1.26x; at 200 in 1001, 376,888 held axes were lost on their own fade and 268,478 with their bearers, and 543,529 retired roots had been let go earlier (385,762 of them bear on a kept axis). The shed laterals on bare wood of living-apex axes are 28,397 at 200 against 79,883 dead kept laterals and 431,621 bare phytomers with no lateral axis recorded.
7. **Bytes** (table G): the grower's allocation for the kept axes at 200 (1001) is 411 MiB on L, 194 MiB on B, 565 MiB on D; D is 48% of the kept axes' bytes (D holds 49% of the phytomers). The shed axes carried 611 MiB more. Totals of the footprint reader's stores are in the logs.

## Unknowns and what is missing

- **Never formed**: not recorded (above). The column "bare phytomers with no lateral axis recorded" is an upper bound for it.
- **Sleeping buds** are not in the settled structure; only the grower's `closed` flag shows them or an apex that ended in the last cycle. D axes the flag marks open: 55,841 of 125,269 (80, 1001), 150,060 of 353,309 (200, 1001), 441,167 of 983,772 (200, 1007). Whether they hold a sleeping bud was not separated.
- **Drawn leaf m2 and the drawn mesh** were not read; the brief's m2 of leaf may differ from the grower's by the mesh's own rules (leaves per phytomer, clusters). The 12-seed medians were not rerun; two seeds only.
- **Light at the tip** is the pencil's value for apexes in the grower's `live` set at the settle (the last cycle's read); it is shown for the living-apex axes only.
- **What the stills draw as whips** is not in these tables; see item 5.
- Phytomers per m2 here use the grower's leaf; with the drawn mesh's leaf the ratio will differ.

## Questions the numbers raise (not answered here; for the host)

1. The leafed phytomers' mean `scale` falls to a third from age 80 to 300 in every PA while 80% of the phytomers stay leafless. Is that fall (what sets `scale x size` of the last cycle's units on old wood) the intended old-tree behaviour?
2. Dead standing wood is 42 to 52% of all phytomers and is nearly all wood within its shedding delay (short, twig, shoot 2 years; branch 3). Whether a delay of 2 to 3 years for the shoots of PA short and twig is meant to leave this share of the tree's stem units standing as dead wood is a design question.
3. The brief's whips are not found as long bare living-apex axes. Which axes the stills draw as whips (tables Ht, class B and L) needs a look at what the mesh draws from them, and a still at a known tree and age, before any statement.


## J. The scale of a leafed phytomer, by factor

Added 13 Nov 2026 on the coordinator's follow-up. Measured on the same six trees, over the leafed phytomers only (each counted once; a leafed phytomer here is one grown in the last cycle, `leafYears` 1, with a leaf above 0), at the reader hook after the settle has laid the tree.

**How the factors were had.** The settled structure does not keep them, so they are recomputed at the reader hook before the shed, from the grower's `Draws` (`birth`, each unit's `survive`, `persist` and `size`, each phytomer's `nodes`) and the same functions the grower uses (`presence::q` and `times` for every f32 product, `geometry::dominance` for the share), along the lineage of every kept axis by the settle's own recursion (`geometry::scale`, `presence::assign_one`, `shed_past`): a phytomer's final `scale` is `running x node x unit_size` (its own unit, `assign_one`) `x base_scale(axis) x size`, with `base_scale = inherited x vigour x share`; a lateral's `inherited` is its bearer phytomer's `running x node x unit_size` times the bearer's `base_scale`; a continuation or relay's is the bearer's `base_scale` and its `vigour` carries the bearer's final `running`; `vigour` is `made` times the shed's fade ratio (`axis.vigour *= own/bearer`, `shed_past`). So the factors are: (a) every `share` on the lineage; (b) every `made` on the lineage (seed, laterals, continuations, relays); (c) every `survive` and `persist` carried into a `running`: the bearer's `running` through the unit the lateral stands in (survive of that unit included), the bearer's final `running` for a continuation or relay, and the phytomer's own `running` through its own unit; (d) its own node draw; (e) its own unit's `size` (the brief's `grown`: `Unit::size`, the share of a node's length); (f) what the brief's list leaves out and the settled scale also contains: (f1) the node draws of the bearer phytomers at each lateral birth, (f2) the `unit.size` of those bearer phytomers' units, (f3) the shed's fade ratios along the lineage (the vigour the shed took from an axis fading toward its shed), (f4) the phytomer's own shade `size` (`Draws::shade`, the light-given unit size), which the settled `scale` includes and the brief's `scale = grew x base_scale` does not name. `survive` and `persist` of the units after the phytomer's own are not in its scale.

**Closeness.** For every leafed phytomer the sum of the logs of (a) to (f) is compared with the log of its final `scale` in the structure: the largest absolute difference and the mean over the leafed phytomers are in the first table. The factors reproduce the scale to f32 rounding. No factor needed for the leafed phytomers is missing; no leafed phytomer was skipped (skipped 0).

### J1. Geometric and arithmetic mean of the scale and of each factor over the leafed phytomers

Each cell: geometric mean / arithmetic mean, phytomers weighted equally. The leaf-area-weighted pair follows in the next table where it differs from the equal-weighted by more than a tenth.

| factor | s1001-a80 | s1001-a120 | s1001-a200 | s1001-a300 | s1007-a120 | s1007-a200 |
|---|---|---|---|---|---|---|
| scale | 0.2402 / 0.5391 | 0.1445 / 0.4333 | 0.1137 / 0.2960 | 0.0847 / 0.2044 | 0.0777 / 0.3496 | 0.0316 / 0.2174 |
| (a) share (dominance) | 0.6647 / 0.6992 | 0.5859 / 0.6225 | 0.4394 / 0.4581 | 0.3154 / 0.3367 | 0.4461 / 0.4942 | 0.2577 / 0.3464 |
| (b) made (birth draws) | 0.7804 / 0.8524 | 0.7313 / 0.8222 | 0.7069 / 0.7983 | 0.7294 / 0.8095 | 0.7532 / 0.8555 | 0.7003 / 0.8175 |
| (c) survive and persist (running) | 0.9728 / 0.9863 | 0.9662 / 0.9842 | 0.9556 / 0.9777 | 0.7989 / 0.9455 | 0.9635 / 0.9817 | 0.9554 / 0.9780 |
| (d) own node | 0.9889 / 0.9944 | 0.9892 / 0.9946 | 0.9892 / 0.9946 | 0.9889 / 0.9944 | 0.9894 / 0.9947 | 0.9896 / 0.9947 |
| (e) own grown (unit size) | 0.9895 / 0.9946 | 0.9899 / 0.9948 | 0.9932 / 0.9966 | 0.9957 / 0.9979 | 0.9778 / 0.9890 | 0.9814 / 0.9906 |
| (f) everything else | 0.4864 / 0.7799 | 0.3565 / 0.7029 | 0.3899 / 0.7474 | 0.4679 / 0.7789 | 0.2480 / 0.6790 | 0.1885 / 0.6209 |
|   (f1) ancestors' node draws at their lateral births | 0.9836 / 0.9920 | 0.9840 / 0.9914 | 0.9858 / 0.9927 | 0.9860 / 0.9927 | 0.9859 / 0.9930 | 0.9848 / 0.9926 |
|   (f2) ancestors' unit sizes at their lateral births | 0.7029 / 0.8465 | 0.5950 / 0.7963 | 0.6248 / 0.8195 | 0.7191 / 0.8593 | 0.4853 / 0.7418 | 0.4337 / 0.7246 |
|   (f3) shed fade ratios along the lineage | 0.8585 / 0.9342 | 0.7650 / 0.8943 | 0.7343 / 0.8884 | 0.6775 / 0.8809 | 0.6905 / 0.8626 | 0.5880 / 0.8272 |
|   (f4) own shade size (`size`) | 0.8196 / 0.9422 | 0.7961 / 0.9257 | 0.8620 / 0.9723 | 0.9742 / 1.0626 | 0.7508 / 0.9109 | 0.7508 / 0.8835 |

Product of the geometric means (a) to (f) against the geometric mean of the scale, as a ratio (1 is exact), and the per-phytomer differences in ln:

| tree | leafed phytomers | geo mean scale / product of geo means | largest |ln scale - sum ln factors| | mean | skipped |
|---|---|---|---|---|---|
| s1001-a80 | 355,114 | 1.00000000 | 2.08e-07 | 2.32e-08 | 0 |
| s1001-a120 | 741,850 | 1.00000000 | 2.33e-07 | 2.83e-08 | 0 |
| s1001-a200 | 1,033,774 | 0.99999999 | 2.39e-07 | 3.54e-08 | 0 |
| s1001-a300 | 895,705 | 0.99999999 | 2.47e-07 | 3.82e-08 | 0 |
| s1007-a120 | 1,516,512 | 1.00000000 | 2.93e-07 | 3.48e-08 | 0 |
| s1007-a200 | 2,324,420 | 1.00000002 | 3.96e-07 | 5.05e-08 | 0 |

Leaf-area-weighted (weight = the phytomer's leaf m2), geometric / arithmetic, shown for the rows that differ from the equal-weighted by more than a tenth in either mean in any tree; every row listed:

| factor | s1001-a80 | s1001-a120 | s1001-a200 | s1001-a300 | s1007-a120 | s1007-a200 |
|---|---|---|---|---|---|---|
| scale | 0.8686 / 1.1188 | 0.7782 / 1.0447 | 0.4702 / 0.6021 | 0.2858 / 0.3439 | 0.7019 / 0.9769 | 0.4397 / 0.5671 |
| (a) share (dominance) | 0.8273 / 0.8588 | 0.7683 / 0.8068 | 0.4989 / 0.5104 | 0.3435 / 0.3628 | 0.6194 / 0.6558 | 0.4416 / 0.4566 |
| (b) made (birth draws) | 0.9373 / 0.9498 | 0.9187 / 0.9339 | 0.8740 / 0.8885 | 0.8454 / 0.8627 | 0.9491 / 0.9626 | 0.9123 / 0.9312 |
| (c) survive and persist (running) | 0.9952 / 0.9967 | 0.9946 / 0.9963 | 0.9927 / 0.9951 | 0.9906 / 0.9936 | 0.9939 / 0.9959 | 0.9924 / 0.9948 |
| (d) own node | 0.9971 / 0.9981 | 0.9971 / 0.9981 | 0.9972 / 0.9981 | 0.9972 / 0.9981 | 0.9972 / 0.9981 | 0.9972 / 0.9981 |
| (e) own grown (unit size) | 0.9983 / 0.9989 | 0.9984 / 0.9989 | 0.9987 / 0.9991 | 0.9989 / 0.9993 | 0.9975 / 0.9983 | 0.9981 / 0.9988 |
| (f) everything else | 1.1306 / 1.2825 | 1.1135 / 1.2758 | 1.0908 / 1.2674 | 0.9975 / 1.1036 | 1.2076 / 1.4373 | 1.1050 / 1.2801 |
|   (f1) ancestors' node draws at their lateral births | 0.9958 / 0.9973 | 0.9946 / 0.9965 | 0.9963 / 0.9975 | 0.9961 / 0.9974 | 0.9963 / 0.9975 | 0.9965 / 0.9976 |
|   (f2) ancestors' unit sizes at their lateral births | 0.9549 / 0.9703 | 0.9452 / 0.9626 | 0.9464 / 0.9637 | 0.9430 / 0.9609 | 0.9411 / 0.9600 | 0.9404 / 0.9597 |
|   (f3) shed fade ratios along the lineage | 0.9908 / 0.9934 | 0.9809 / 0.9856 | 0.9829 / 0.9880 | 0.9779 / 0.9843 | 0.9865 / 0.9902 | 0.9811 / 0.9867 |
|   (f4) own shade size (`size`) | 1.2001 / 1.3286 | 1.2076 / 1.3452 | 1.1769 / 1.3268 | 1.0859 / 1.1763 | 1.3055 / 1.5021 | 1.2018 / 1.3462 |

Rows flagged (weighted differs by more than a tenth): scale, (a) share (dominance), (b) made (birth draws), (c) survive and persist (running), (f) everything else, (f2) ancestors' unit sizes at their lateral births, (f3) shed fade ratios along the lineage, (f4) own shade size (`size`).

### J2. What is on the lineage of a leafed phytomer

Mean / maximum per leafed phytomer. `lateral births` = lineage depth; `hand-overs` = continuations and relays on the lineage; `axis births` = lateral + hand-over + the seed (the number of `made` factors); `growth units` = units grown along the lineage up to the phytomer's own unit; `survive/persist factors` = the count of factors in (c). Last three columns: the geometric mean of one factor: (a) per lateral birth, (b) per axis birth, (c) per survive/persist factor.

| tree | lateral births | hand-overs | axis births | growth units | survive/persist factors | (a) per lateral birth | (b) per axis birth | (c) per factor |
|---|---|---|---|---|---|---|---|---|
| s1001-a80 | 5.78 / 16 | 9.38 / 20 | 16.16 / 28 | 81.16 / 92 | 155.55 / 177 | 0.9318 | 0.9848 | 0.99982 |
| s1001-a120 | 6.74 / 20 | 17.50 / 51 | 25.24 / 54 | 121.77 / 139 | 235.81 / 267 | 0.9238 | 0.9877 | 0.99985 |
| s1001-a200 | 7.44 / 27 | 44.07 / 131 | 52.52 / 134 | 201.83 / 220 | 395.21 / 426 | 0.8954 | 0.9934 | 0.99989 |
| s1001-a300 | 7.98 / 37 | 77.83 / 225 | 86.81 / 229 | 301.44 / 320 | 593.90 / 612 | 0.8653 | 0.9964 | 0.99962 |
| s1007-a120 | 7.06 / 23 | 15.99 / 55 | 24.05 / 59 | 122.78 / 151 | 237.50 / 289 | 0.8920 | 0.9883 | 0.99984 |
| s1007-a200 | 8.59 / 34 | 41.07 / 137 | 50.65 / 140 | 203.46 / 244 | 397.34 / 461 | 0.8539 | 0.9930 | 0.99989 |

### J3. By lineage depth (lateral births) and by growth units passed


**s1001-a80**, by lateral births (geometric means)

| lateral births | leafed phytomers | scale | (a) share | (b) made | (c) survive/persist |
|---|---|---|---|---|---|
| 0 | 6 | 1.0938 | 1.0000 | 1.0000 | 1.0000 |
| 1 | 193 | 0.8514 | 0.9402 | 0.9883 | 1.0000 |
| 2 | 3,243 | 0.7378 | 0.8825 | 0.9465 | 1.0000 |
| 3 | 22,452 | 0.5381 | 0.7966 | 0.8896 | 0.9912 |
| 4 | 76,323 | 0.4346 | 0.7352 | 0.8484 | 0.9783 |
| 5 | 99,432 | 0.2888 | 0.6265 | 0.7957 | 0.9732 |
| 6 | 56,520 | 0.2360 | 0.6280 | 0.7681 | 0.9698 |
| 7 | 35,304 | 0.1837 | 0.6468 | 0.7602 | 0.9629 |
| 8 | 22,338 | 0.1294 | 0.6356 | 0.7437 | 0.9724 |
| 9 | 15,034 | 0.0862 | 0.6313 | 0.7295 | 0.9642 |
| 10 | 8,573 | 0.0716 | 0.6324 | 0.7359 | 0.9550 |
| 11 | 5,940 | 0.0591 | 0.6612 | 0.6721 | 0.9660 |
| 12 | 4,146 | 0.0211 | 0.6500 | 0.4246 | 0.9577 |
| 13 | 3,254 | 0.0117 | 0.6292 | 0.3915 | 0.9560 |
| 14 | 1,651 | 0.0118 | 0.5781 | 0.4535 | 0.9908 |
| 15 | 587 | 0.0112 | 0.6672 | 0.2306 | 0.9784 |
| 16 | 118 | 0.0019 | 0.7600 | 0.1069 | 1.0000 |

**s1001-a80**, by growth units passed on the lineage

| growth units | leafed phytomers | scale | (a) share | (b) made | (c) survive/persist |
|---|---|---|---|---|---|
| 50-99 | 355,114 | 0.2402 | 0.6647 | 0.7804 | 0.9728 |

**s1001-a120**, by lateral births (geometric means)

| lateral births | leafed phytomers | scale | (a) share | (b) made | (c) survive/persist |
|---|---|---|---|---|---|
| 0 | 6 | 1.1444 | 1.0000 | 1.0000 | 1.0000 |
| 1 | 253 | 0.9476 | 0.9280 | 0.9911 | 1.0000 |
| 2 | 3,931 | 0.7482 | 0.8693 | 0.9323 | 0.9947 |
| 3 | 28,283 | 0.5062 | 0.7610 | 0.8897 | 0.9924 |
| 4 | 104,726 | 0.3839 | 0.6857 | 0.8478 | 0.9801 |
| 5 | 164,259 | 0.2465 | 0.5620 | 0.7975 | 0.9715 |
| 6 | 124,179 | 0.1826 | 0.5258 | 0.7749 | 0.9650 |
| 7 | 87,668 | 0.1464 | 0.5458 | 0.7189 | 0.9668 |
| 8 | 67,875 | 0.1109 | 0.5719 | 0.6659 | 0.9658 |
| 9 | 50,207 | 0.0696 | 0.5802 | 0.6149 | 0.9514 |
| 10 | 38,447 | 0.0444 | 0.5999 | 0.5742 | 0.9209 |
| 11 | 24,162 | 0.0419 | 0.6149 | 0.6082 | 0.9525 |
| 12 | 14,759 | 0.0247 | 0.6144 | 0.5908 | 0.9511 |
| 13 | 10,970 | 0.0096 | 0.6005 | 0.5968 | 0.9627 |
| 14 | 7,244 | 0.0035 | 0.5642 | 0.5966 | 0.9619 |
| 15 | 5,414 | 0.0047 | 0.5588 | 0.6392 | 0.9719 |
| 16 | 3,982 | 0.0048 | 0.5733 | 0.4896 | 0.9761 |
| 17 | 2,307 | 0.0051 | 0.6395 | 0.3020 | 0.9714 |
| 18 | 1,306 | 0.0049 | 0.7124 | 0.3154 | 0.9826 |
| 19 | 1,546 | 0.0077 | 0.6663 | 0.4335 | 0.9780 |
| 20 | 326 | 0.0060 | 0.6031 | 0.6184 | 0.9759 |

**s1001-a120**, by growth units passed on the lineage

| growth units | leafed phytomers | scale | (a) share | (b) made | (c) survive/persist |
|---|---|---|---|---|---|
| 100-199 | 741,850 | 0.1445 | 0.5859 | 0.7313 | 0.9662 |

**s1001-a200**, by lateral births (geometric means)

| lateral births | leafed phytomers | scale | (a) share | (b) made | (c) survive/persist |
|---|---|---|---|---|---|
| 1 | 16 | 0.4857 | 0.5375 | 0.8891 | 1.0000 |
| 2 | 852 | 0.3244 | 0.5269 | 0.8346 | 0.9698 |
| 3 | 13,126 | 0.2487 | 0.5166 | 0.8166 | 0.9620 |
| 4 | 77,339 | 0.2278 | 0.4994 | 0.8000 | 0.9539 |
| 5 | 226,182 | 0.2263 | 0.4771 | 0.7885 | 0.9515 |
| 6 | 218,772 | 0.1828 | 0.4069 | 0.7826 | 0.9604 |
| 7 | 156,018 | 0.1503 | 0.3851 | 0.7536 | 0.9526 |
| 8 | 92,546 | 0.1186 | 0.4163 | 0.6825 | 0.9473 |
| 9 | 58,178 | 0.0934 | 0.4324 | 0.5882 | 0.9556 |
| 10 | 43,844 | 0.0806 | 0.4418 | 0.5824 | 0.9540 |
| 11 | 34,191 | 0.0605 | 0.4457 | 0.5719 | 0.9476 |
| 12 | 24,659 | 0.0377 | 0.4560 | 0.5723 | 0.9661 |
| 13 | 19,269 | 0.0218 | 0.4749 | 0.5057 | 0.9585 |
| 14 | 13,194 | 0.0155 | 0.4943 | 0.5107 | 0.9743 |
| 15 | 10,260 | 0.0170 | 0.5098 | 0.5874 | 0.9630 |
| 16 | 9,501 | 0.0143 | 0.5299 | 0.5900 | 0.9740 |
| 17 | 8,969 | 0.0049 | 0.5208 | 0.4878 | 0.9745 |
| 18 | 7,732 | 0.0021 | 0.4904 | 0.3956 | 0.9762 |
| 19 | 4,884 | 0.0009 | 0.4556 | 0.3888 | 0.9802 |
| 20 | 3,993 | 0.0001 | 0.4918 | 0.4470 | 0.9732 |
| 21 | 4,169 | 0.0000 | 0.4952 | 0.3765 | 0.9794 |
| 22 | 3,384 | 0.0000 | 0.5307 | 0.3378 | 0.9715 |
| 23 | 1,341 | 0.0000 | 0.5154 | 0.4062 | 0.9345 |
| 24 | 810 | 0.0000 | 0.5220 | 0.3136 | 0.9678 |
| 25+ | 545 | 0.0000 | 0.5208 | 0.4836 | 0.9848 |

**s1001-a200**, by growth units passed on the lineage

| growth units | leafed phytomers | scale | (a) share | (b) made | (c) survive/persist |
|---|---|---|---|---|---|
| 100-199 | 1,054 | 0.0994 | 0.4059 | 0.8524 | 0.8195 |
| >=200 | 1,032,720 | 0.1137 | 0.4395 | 0.7068 | 0.9557 |

**s1001-a300**, by lateral births (geometric means)

| lateral births | leafed phytomers | scale | (a) share | (b) made | (c) survive/persist |
|---|---|---|---|---|---|
| 1 | 6 | 0.4075 | 0.5074 | 0.7888 | 1.0000 |
| 2 | 317 | 0.2114 | 0.4984 | 0.7751 | 0.7994 |
| 3 | 5,420 | 0.1489 | 0.4812 | 0.7466 | 0.7550 |
| 4 | 35,806 | 0.1323 | 0.4458 | 0.7531 | 0.7367 |
| 5 | 121,266 | 0.1176 | 0.4008 | 0.7589 | 0.7081 |
| 6 | 179,251 | 0.1171 | 0.3084 | 0.8043 | 0.7904 |
| 7 | 182,513 | 0.1081 | 0.2753 | 0.7970 | 0.8097 |
| 8 | 109,248 | 0.0925 | 0.2747 | 0.7431 | 0.8118 |
| 9 | 77,298 | 0.0844 | 0.2832 | 0.7006 | 0.8195 |
| 10 | 53,514 | 0.0648 | 0.2915 | 0.6609 | 0.7707 |
| 11 | 32,316 | 0.0683 | 0.3054 | 0.6549 | 0.8139 |
| 12 | 22,101 | 0.0807 | 0.3213 | 0.6836 | 0.8954 |
| 13 | 14,807 | 0.0480 | 0.3318 | 0.6093 | 0.8460 |
| 14 | 11,450 | 0.0442 | 0.3399 | 0.5661 | 0.9532 |
| 15 | 9,884 | 0.0249 | 0.3635 | 0.5155 | 0.9617 |
| 16 | 8,745 | 0.0237 | 0.3925 | 0.5770 | 0.9721 |
| 17 | 7,219 | 0.0195 | 0.3956 | 0.5177 | 0.9705 |
| 18 | 5,658 | 0.0152 | 0.3925 | 0.4655 | 0.9750 |
| 19 | 3,337 | 0.0045 | 0.4096 | 0.4053 | 0.9620 |
| 20 | 2,920 | 0.0018 | 0.4171 | 0.4138 | 0.9133 |
| 21 | 2,256 | 0.0008 | 0.3941 | 0.2389 | 0.9085 |
| 22 | 2,492 | 0.0008 | 0.4262 | 0.2950 | 0.9834 |
| 23 | 1,160 | 0.0000 | 0.3974 | 0.3708 | 0.9927 |
| 24 | 978 | 0.0009 | 0.4506 | 0.5755 | 0.9951 |
| 25+ | 5,743 | 0.0003 | 0.4267 | 0.4322 | 0.9854 |

**s1001-a300**, by growth units passed on the lineage

| growth units | leafed phytomers | scale | (a) share | (b) made | (c) survive/persist |
|---|---|---|---|---|---|
| >=200 | 895,705 | 0.0847 | 0.3154 | 0.7294 | 0.7989 |

**s1007-a120**, by lateral births (geometric means)

| lateral births | leafed phytomers | scale | (a) share | (b) made | (c) survive/persist |
|---|---|---|---|---|---|
| 0 | 6 | 1.2720 | 1.0000 | 1.0000 | 1.0000 |
| 1 | 252 | 0.8453 | 0.8906 | 0.9041 | 1.0000 |
| 2 | 4,252 | 0.6616 | 0.7515 | 0.9104 | 0.9952 |
| 3 | 38,082 | 0.3903 | 0.6253 | 0.8745 | 0.9918 |
| 4 | 165,363 | 0.2916 | 0.5600 | 0.8556 | 0.9813 |
| 5 | 342,688 | 0.2339 | 0.4993 | 0.8413 | 0.9698 |
| 6 | 270,329 | 0.1642 | 0.4454 | 0.8133 | 0.9598 |
| 7 | 184,138 | 0.0985 | 0.4141 | 0.7533 | 0.9542 |
| 8 | 137,917 | 0.0527 | 0.3730 | 0.7123 | 0.9555 |
| 9 | 108,644 | 0.0304 | 0.3560 | 0.6808 | 0.9525 |
| 10 | 78,669 | 0.0165 | 0.3668 | 0.6398 | 0.9595 |
| 11 | 54,587 | 0.0089 | 0.3801 | 0.6245 | 0.9601 |
| 12 | 40,681 | 0.0051 | 0.4026 | 0.5807 | 0.9594 |
| 13 | 29,641 | 0.0027 | 0.4225 | 0.5385 | 0.9637 |
| 14 | 20,234 | 0.0007 | 0.4340 | 0.4904 | 0.9547 |
| 15 | 14,505 | 0.0005 | 0.4442 | 0.5193 | 0.9471 |
| 16 | 11,242 | 0.0004 | 0.4844 | 0.5684 | 0.9613 |
| 17 | 6,948 | 0.0003 | 0.4825 | 0.6112 | 0.9514 |
| 18 | 3,751 | 0.0001 | 0.4916 | 0.4969 | 0.9582 |
| 19 | 2,350 | 0.0002 | 0.5005 | 0.3998 | 0.9790 |
| 20 | 1,558 | 0.0015 | 0.4916 | 0.2941 | 0.9684 |
| 21 | 553 | 0.0052 | 0.5143 | 0.2566 | 0.9499 |
| 22 | 110 | 0.0035 | 0.5225 | 0.2268 | 0.8973 |
| 23 | 12 | 0.0021 | 0.5225 | 0.1361 | 1.0000 |

**s1007-a120**, by growth units passed on the lineage

| growth units | leafed phytomers | scale | (a) share | (b) made | (c) survive/persist |
|---|---|---|---|---|---|
| 100-199 | 1,516,512 | 0.0777 | 0.4461 | 0.7532 | 0.9635 |

**s1007-a200**, by lateral births (geometric means)

| lateral births | leafed phytomers | scale | (a) share | (b) made | (c) survive/persist |
|---|---|---|---|---|---|
| 1 | 34 | 0.4506 | 0.5112 | 0.8643 | 1.0000 |
| 2 | 1,649 | 0.3059 | 0.5012 | 0.8033 | 1.0000 |
| 3 | 23,830 | 0.2487 | 0.4890 | 0.7800 | 0.9929 |
| 4 | 133,806 | 0.2246 | 0.4624 | 0.7817 | 0.9829 |
| 5 | 376,437 | 0.2129 | 0.4288 | 0.7942 | 0.9670 |
| 6 | 398,760 | 0.1565 | 0.3459 | 0.8095 | 0.9530 |
| 7 | 280,607 | 0.1115 | 0.3091 | 0.7816 | 0.9405 |
| 8 | 206,665 | 0.0652 | 0.2570 | 0.7565 | 0.9492 |
| 9 | 180,343 | 0.0303 | 0.1850 | 0.7097 | 0.9548 |
| 10 | 152,932 | 0.0148 | 0.1383 | 0.6633 | 0.9515 |
| 11 | 122,041 | 0.0085 | 0.1285 | 0.6316 | 0.9453 |
| 12 | 95,185 | 0.0052 | 0.1398 | 0.6000 | 0.9573 |
| 13 | 72,906 | 0.0030 | 0.1482 | 0.5875 | 0.9548 |
| 14 | 56,771 | 0.0015 | 0.1515 | 0.5624 | 0.9462 |
| 15 | 46,349 | 0.0007 | 0.1554 | 0.5112 | 0.9403 |
| 16 | 35,249 | 0.0003 | 0.1513 | 0.4975 | 0.9514 |
| 17 | 29,152 | 0.0001 | 0.1458 | 0.4625 | 0.9604 |
| 18 | 27,033 | 0.0000 | 0.1575 | 0.4247 | 0.9571 |
| 19 | 22,578 | 0.0000 | 0.1689 | 0.4144 | 0.9603 |
| 20 | 17,625 | 0.0000 | 0.1972 | 0.3803 | 0.9590 |
| 21 | 12,561 | 0.0000 | 0.2040 | 0.3435 | 0.9481 |
| 22 | 8,150 | 0.0000 | 0.1881 | 0.3064 | 0.9660 |
| 23 | 5,194 | 0.0000 | 0.1809 | 0.3420 | 0.9606 |
| 24 | 4,414 | 0.0000 | 0.2758 | 0.3336 | 0.9745 |
| 25+ | 14,149 | 0.0000 | 0.4480 | 0.2699 | 0.9732 |

**s1007-a200**, by growth units passed on the lineage

| growth units | leafed phytomers | scale | (a) share | (b) made | (c) survive/persist |
|---|---|---|---|---|---|
| 100-199 | 2,000 | 0.0187 | 0.1034 | 0.8201 | 0.9600 |
| >=200 | 2,322,420 | 0.0316 | 0.2579 | 0.7002 | 0.9554 |

### J4. Dominance by the born axis's PA row

Every lateral birth counted once for each leafed phytomer of its subtree (the lineage view); geometric mean `share` per such birth by the born axis's PA, with the row's `form.dominance` value. Rows with dominance 0 give 1. Axes = lateral axes of that PA with at least one leafed phytomer below them.

| tree | born PA | lateral axes | births x leafed phytomers | births per leafed phytomer | geometric mean share per birth | form.dominance |
|---|---|---|---|---|---|---|
| s1001-a80 | 6 limb | 5 | 235,109 | 0.662 | 0.5396 | 0.500000 |
| s1001-a80 | 7 bough | 68 | 329,289 | 0.927 | 1.0000 | 0.000000 |
| s1001-a80 | 8 sprig | 1 | 1,460 | 0.004 | 1.0000 | 0.000000 |
| s1001-a80 | 9 branch | 7,102 | 940,407 | 2.648 | 1.0000 | 0.000000 |
| s1001-a80 | 13 twig | 25,397 | 295,444 | 0.832 | 1.0000 | 0.000000 |
| s1001-a80 | 14 short | 62,952 | 251,967 | 0.710 | 1.0000 | 0.000000 |
| s1001-a120 | 6 limb | 9 | 641,707 | 0.865 | 0.5390 | 0.500000 |
| s1001-a120 | 7 bough | 133 | 706,528 | 0.952 | 1.0000 | 0.000000 |
| s1001-a120 | 9 branch | 16,328 | 2,500,101 | 3.370 | 1.0000 | 0.000000 |
| s1001-a120 | 13 twig | 52,764 | 620,978 | 0.837 | 1.0000 | 0.000000 |
| s1001-a120 | 14 short | 133,164 | 532,391 | 0.718 | 1.0000 | 0.000000 |
| s1001-a200 | 6 limb | 13 | 1,475,435 | 1.427 | 0.5621 | 0.500000 |
| s1001-a200 | 7 bough | 234 | 1,010,506 | 0.977 | 1.0000 | 0.000000 |
| s1001-a200 | 9 branch | 20,347 | 3,566,140 | 3.450 | 1.0000 | 0.000000 |
| s1001-a200 | 13 twig | 67,566 | 870,255 | 0.842 | 1.0000 | 0.000000 |
| s1001-a200 | 14 short | 192,564 | 769,918 | 0.745 | 1.0000 | 0.000000 |
| s1001-a300 | 6 limb | 19 | 1,901,845 | 2.123 | 0.5807 | 0.500000 |
| s1001-a300 | 7 bough | 242 | 878,093 | 0.980 | 1.0000 | 0.000000 |
| s1001-a300 | 9 branch | 14,878 | 2,914,297 | 3.254 | 1.0000 | 0.000000 |
| s1001-a300 | 13 twig | 53,103 | 759,831 | 0.848 | 1.0000 | 0.000000 |
| s1001-a300 | 14 short | 173,182 | 691,994 | 0.773 | 1.0000 | 0.000000 |
| s1007-a120 | 6 limb | 40 | 2,006,688 | 1.323 | 0.5433 | 0.500000 |
| s1007-a120 | 7 bough | 396 | 1,441,583 | 0.951 | 1.0000 | 0.000000 |
| s1007-a120 | 8 sprig | 1 | 97 | 0.000 | 1.0000 | 0.000000 |
| s1007-a120 | 9 branch | 43,772 | 5,041,413 | 3.324 | 1.0000 | 0.000000 |
| s1007-a120 | 13 twig | 123,237 | 1,236,636 | 0.815 | 1.0000 | 0.000000 |
| s1007-a120 | 14 short | 245,300 | 980,677 | 0.647 | 1.0000 | 0.000000 |
| s1007-a200 | 6 limb | 53 | 5,041,941 | 2.169 | 0.5352 | 0.500000 |
| s1007-a200 | 7 bough | 676 | 2,256,276 | 0.971 | 1.0000 | 0.000000 |
| s1007-a200 | 9 branch | 68,058 | 9,203,380 | 3.959 | 1.0000 | 0.000000 |
| s1007-a200 | 13 twig | 180,715 | 1,907,600 | 0.821 | 1.0000 | 0.000000 |
| s1007-a200 | 14 short | 386,762 | 1,547,171 | 0.666 | 1.0000 | 0.000000 |

### Reading off J (numbers only)

On seed 1001 the geometric mean scale of a leafed phytomer is 0.240, 0.145, 0.114, 0.085 at 80, 120, 200, 300 (arithmetic 0.54, 0.43, 0.30, 0.20); the six factors reproduce it to 4e-7 in ln at worst. Of the 1.04 in ln that the geometric mean falls from 80 to 300, (a) share takes 0.75 (geometric mean 0.665 to 0.315), (c) survive and persist 0.20 (0.973 to 0.799), (b) made 0.07 (0.780 to 0.729) and (f) the rest a net 0.04 (0.486 to 0.468: the shed fade ratios 0.859 to 0.678 and the ancestors' unit sizes 0.703 to 0.719 against the own shade size 0.820 to 0.974); (d) own node and (e) own unit size are 0.989 to 0.996 at every age and move nothing. From 80 to 200 the split is (a) 0.41, (f) 0.22 (f3 0.16 and f2 0.12, against f4 +0.05), (b) 0.10, (c) 0.02. The share (a) comes only from limb-born axes: the limb row's dominance is 0.5 and its geometric mean share per limb birth is 0.54 to 0.58; every other PA row has dominance 0 and gives 1; what changes with age is how many limb births a leafed phytomer has on its lineage, 0.66, 0.87, 1.43, 2.12 on average at 80, 120, 200, 300 (seed 1007: 1.32 at 120, 2.17 at 200), while the mean lateral-birth depth goes 5.8, 6.7, 7.4, 8.0 and the mean hand-overs 9, 18, 44, 78 (growth units on the lineage 81, 122, 202, 301, about the age). At equal depth the share is lower in the older tree (depth 5: 0.627, 0.562, 0.477, 0.401), and the scale falls with depth at every age. Per factor, (a) is 0.93, 0.92, 0.90, 0.87 per lateral birth, (b) 0.985 to 0.996 per axis birth and (c) 0.9996 to 0.9999 per survive/persist factor; the (c) product at 300 is lower than at 200 (0.799 against 0.956) in the lineages of depth 2 to 11 (0.71 to 0.82), (0.92 to 1.00 at the other ages), which the table by lateral births shows and this measurement does not explain. Seed 1007 shows the same order: scale 0.078 to 0.032 from 120 to 200, (a) 0.446 to 0.258, (f) 0.248 to 0.189, (b) 0.753 to 0.700, (c) 0.964 to 0.955. Weighted by leaf area the geometric mean scale is 0.87, 0.78, 0.47, 0.29 and the factors (a), (b), (c) at 300, (f), (f2), (f3), (f4) move by more than a tenth against the equal-weighted ones (leaf area sits on the larger scales, the shade size is 1.09 to 1.31 on that weight against 0.75 to 0.97).

## Reader removed

The reader was removed with `git apply -R` on `oldoak-final.patch`; `git status --short` in `census-old` is empty (see the final line of the run below).
