# Held before the settle against kept after it

Measured 13 Nov 2026 at head `.worktrees/fn-362`, 810e90bf3 (branch fn-362-a-runs-rings-have-the-sides-its-widest), `ci` profile, each tree its own process through `telperion-heavy`, one at a time, no GPU. Temporary instrumentation (saved as `scratchpad/held/held-probe.patch`, runner crate in `scratchpad/held/runner/`, scripts `analyze.py`, `assemble.py`, `runall.sh`, `run2.sh`, logs in `scratchpad/held/logs/`) was applied, built, run and removed again with `git apply -R`; `git status --short` is as found (empty). Nothing was pushed or committed. No timings are reported.

## Two species, one name

The numbers in the brief do not come from one species. Two different beeches were measured:

- The **shipped** European beech is the preset `european-beech` run through the pipeline (`executor::growth`, then `telperion_space::grow`), the tree the scoreboard probe draws. At age 80, seed 7 it holds 7,107,275 phytomers on 636,161 axes just before the settle and settles to 6,170,796 phytomers on 408,085 axes. The 6,170,796 of BEECH-LAI.md is this tree.
- `telperion_space::beech()` is the species the `form` test (`crates/telperion-space/tests/form.rs`) grows and the one BEECH-FORM-MEMORY.md measured. At age 80, seed 7 it holds 27,138,515 phytomers on 1,846,068 axes and settles to 24,178,429 phytomers on 1,469,354 axes. The 27,138,515 of the brief is this tree; the probe reproduces the earlier record exactly (held axes, held phytomers, living apexes, axes by kind, and the footprint's store bytes).

So 27,138,515 held against 6,170,796 settled compares two species. For each species held against kept is within one tenth to one fifth (item 1 of each tree below). Trees 1 to 5 are the shipped presets the brief names; trees 6 to 8 are the `beech()` runs, added so the earlier figures can be read against the same split.

## How it was read

The probe sits in `Grower::settle_shed` (`grow/settle.rs`), after `assign` and before `shed_past`, so it sees the axes exactly as the shed rule does, and again after `shed_past` to count what it kept. It is switched on by an environment variable, runs the settle's own `fades` on the same `given`, `gone` and `stands` closures the settle passes, and replicates the keep/drop loop of `shed_past`. Its kept counts equal the real `shed_past`'s output and the final structure's counts for every tree (the "kept by the real `shed_past`" and "settled structure" rows of item 1).

Words, as the code uses them:

- **Held**: the axes in `Grower::axes` at the pause, and the phytomers in them. A retired root's or an inside-retired axis's record is still held until `compact.rs` lets it go (it runs once the loose axes reach a quarter of those held), but its phytomers were emptied at retirement (`let_go`), so those records are held axes with 0 phytomers. Roots already let go before the pause are not held; they survive only as `Stand`s of 24 bytes, whose count is given per tree ("standing").
- **Kept**: two meanings, both given. (a) The settled structure's: the axes `shed_past` returns, which are what `scale`, `thicken` and `place` then work on; its phytomer count is the structure's final count (no later pass removes or merges phytomers: the `grow()` result has the same count). (b) `Fate::Kept` in `retire.rs`: a closed subtree evaluated and kept, still held and possibly still fading; it is a retirement status, not the settle's verdict, and it shows up in item 3's "retired status" table. `rules.grown` is a third count, the phytomers grown over the whole run including those already let go; it is given for scale. The "taken as drawn or merged" count the brief mentions does not exist at the settle: the drawn tree's node count (BEECH-LAI.md "nodes", 5,953,421) is a later stage, the pipeline's mesh, which this probe does not enter.
- **Fates at the settle** (item 2): from `shed_past`. An axis is lost when its bearer was lost, or when it is a lateral or relay and its own fade is nothing. "Own" below is the second case (the axis is the shed root), "via its bearer" the first. Those are the only two exits; "dropped for another reason" is therefore 0 by construction. Kept axes are classed by their applied fade, the least of their own and their bearers' (`Shed::applied`), which is the share their scale and girth are drawn at.
- **Closed**: `Working::closed(i)`, nothing in the subtree lives or sleeps. "Open" is on the way to a living apex or sleeping bud. "Not yet taken in" would be an axis laid in the last cycle that the working set has not yet extended to; no such axis is dropped in any tree here (all of them are kept), so that row is absent from the dropped tables.
- **Cycles since the apex ended**: the settle's age minus `apex_end`. Living apex means `apex_end` is none.
- **Later years' passes**: not traced at runtime. The column cites the doc that answers it: `compact.rs` ("nothing reads the axes inside it again, for every pass, the settle's included, stops at the root"), `retire.rs` (a retired root is read only through its bearer's carry; a closed subtree is evaluated; a kept lateral that sheds waits in the fading list and is evaluated again once its fade reaches none) and `working.rs` (the balance and the allotment visit only open axes). A paused grower can grow on (`Paused::grow_to`, fn-270), so "a later year" exists for the tree in the grower, not for the tree `grow()` returns, which settles once.
- **Footprint**: the reader of BEECH-FORM-MEMORY.md (`Grower::footprint`, length and capacity times record size), read at the settle's entry. Per-axis stores are split by the axis's own record and Vec capacities; each table row's "slack" is the store's reader bytes not attributable to a held axis (Vec capacity beyond length at the top level). The same axis counts hold: the per-axis split uses each held axis's record and heap bytes, so an axis with an empty phytomer Vec still costs its record.
- The delay of item 5 is `species.states[pa].shedding`. The brief's per-axis "cycles since the apex ended" uses the axis's own `apex_end`; the shed rule itself reads the subtree's last living time, so an axis dropped with its bearer ("below") can have ended later than its delay.


## Summary across the trees

| tree | held axes | held phytomers | kept axes | kept phytomers | dropped axes | dropped phytomers | reader MiB | dropped, attributed MiB | dropped phytomers on axes whose apex ended 2 cycles ago |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| Tree 1 pipe-beech-80-7 | 636,161 | 7,107,275 | 408,085 | 6,170,796 | 228,076 (35.9%) | 936,479 (13.2%) | 1,497 | 288 (19.2%) | 726,248 |
| Tree 2 pipe-beech-80-1 | 427,645 | 5,756,748 | 321,607 | 5,037,729 | 106,038 (24.8%) | 719,019 (12.5%) | 1,175 | 169 (14.4%) | 587,188 |
| Tree 3 pipe-beech-40-7 | 77,365 | 869,659 | 67,311 | 796,872 | 10,054 (13.0%) | 72,787 (8.4%) | 199 | 16 (8.2%) | 64,893 |
| Tree 4 pipe-oak-120-1001 | 577,092 | 3,863,297 | 401,667 | 3,100,796 | 175,425 (30.4%) | 762,501 (19.7%) | 1,189 | 215 (18.1%) | 7,272 |
| Tree 5 pipe-spruce-40-1 | 370,335 | 1,248,873 | 292,083 | 1,115,420 | 78,252 (21.1%) | 133,453 (10.7%) | 600 | 69 (11.5%) | 1,923 |
| Tree 6 space-beech-80-7 | 1,846,068 | 27,138,515 | 1,469,354 | 24,178,429 | 376,714 (20.4%) | 2,960,086 (10.9%) | 5,743 | 652 (11.3%) | 2,497,737 |
| Tree 7 space-beech-80-1 | 1,234,880 | 16,300,325 | 873,989 | 14,514,750 | 360,891 (29.2%) | 1,785,575 (11.0%) | 3,286 | 497 (15.1%) | 1,511,519 |
| Tree 8 space-beech-40-7 | 138,188 | 1,287,768 | 99,309 | 1,203,296 | 38,879 (28.1%) | 84,472 (6.6%) | 329 | 39 (11.8%) | 76,660 |

## Tree 1. Shipped European beech (preset `european-beech`), age 80, seed 7

### Item 1. Held, kept, difference

|  | axes | phytomers |
|---|---:|---:|
| held just before the settle (axes in the grower) | 636,161 | 7,107,275 |
| kept by the shed rule (`shed_past`, probe replica) | 408,085 | 6,170,796 |
| dropped (difference) | 228,076 | 936,479 |
| kept by the real `shed_past` (settle_shed, printed after it) | 408,085 | 6,170,796 |
| settled structure `grow()` returns (after scale, thicken, place) | 408,085 | 6,170,796 |
| `rules.grown` (phytomers grown for the tree, shed ones included; `Cycled.grown`) | - | 24,390,678 |

Living apexes in the grower: 295,914. Retirement table length: 580,620 axes taken in (55,541 laid in the last cycle, not yet taken in). Retired roots whose records were let go earlier and still stand in the tree's fades (`Retirement::standing`, not held as axes): 152,826.

### Item 2. Held split by the axis's fate at the settle

| fate | axes | share | phytomers | share |
|---|---:|---:|---:|---:|
| kept at full scale (applied fade 1) | 399,523 | 62.8% | 5,970,074 | 84.0% |
| kept faded, scale over 0.5 | 4,032 | 0.6% | 94,669 | 1.3% |
| kept faded, 0.1 to 0.5 | 3,638 | 0.6% | 84,654 | 1.2% |
| kept faded, 0.01 to 0.1 | 827 | 0.1% | 19,678 | 0.3% |
| kept faded, 0.01 or under | 65 | 0.0% | 1,721 | 0.0% |
| dropped, shed rule via its bearer (an ancestor was shed) | 62,954 | 9.9% | 285,393 | 4.0% |
| dropped, shed rule at its own fade (axis is the shed root) | 165,122 | 26.0% | 651,086 | 9.2% |
| dropped for another reason | 0 | 0.0% | 0 | 0.0% |

Dropped for another reason: the shed loop has only the two exits (bearer lost, own fade at nothing); none else exists, so 0 by construction.

### Item 3. Dropped split

By PA:

| PA | axes | share | phytomers | share |
|---|---:|---:|---:|---:|
| pa9 | 2,205 | 1.0% | 28,042 | 3.0% |
| pa12 | 26,202 | 11.5% | 83,741 | 8.9% |
| pa14 | 199,669 | 87.5% | 824,696 | 88.1% |

By apex:

| apex | axes | share | phytomers | share |
|---|---:|---:|---:|---:|
| apex ended 0 cycles ago | 284 | 0.1% | 13,651 | 1.5% |
| apex ended 1 cycles ago | 1,024 | 0.4% | 24,719 | 2.6% |
| apex ended 2 cycles ago | 42,247 | 18.5% | 726,248 | 77.6% |
| apex ended 3-5 cycles ago | 120,266 | 52.7% | 36,123 | 3.9% |
| apex ended 6-10 cycles ago | 44,050 | 19.3% | 45,078 | 4.8% |
| apex ended >10 cycles ago | 20,205 | 8.9% | 90,660 | 9.7% |

By working set:

| working set | axes | share | phytomers | share |
|---|---:|---:|---:|---:|
| closed | 226,836 | 99.5% | 899,842 | 96.1% |
| open | 1,240 | 0.5% | 36,637 | 3.9% |

By retired status:

| retired status | axes | share | phytomers | share |
|---|---:|---:|---:|---:|
| a retired root's stand (axis record held) | 133,959 | 58.7% | 0 | 0.0% |
| inside a retired subtree | 40,731 | 17.9% | 0 | 0.0% |
| not retired, fate kept (fading list) | 38,389 | 16.8% | 594,273 | 63.5% |
| not retired, fate open | 14,997 | 6.6% | 342,206 | 36.5% |

By later years' passes:

| later years' passes | axes | share | phytomers | share |
|---|---:|---:|---:|---:|
| no | 174,690 | 76.6% | 0 | 0.0% |
| yes (fate Kept: a lateral that sheds waits in the fading list and is evaluated once its fade is none, retire.rs faded_out; otherwise read inside an ancestor's evaluation) | 38,389 | 16.8% | 594,273 | 63.5% |
| yes (open: balance and allotment visit it, working.rs) | 1,240 | 0.5% | 36,637 | 3.9% |
| yes (retire.rs evaluation of the closed subtree) | 13,757 | 6.0% | 305,569 | 32.6% |

### Item 4. Footprint by store, kept and dropped

| store | reader MiB | kept axes MiB | dropped, retired MiB | dropped, other MiB | slack / unsplit MiB |
|---|---:|---:|---:|---:|---:|
| axes | 166.0 | 90.3 | 38.7 | 11.8 | 25.2 |
| axes.phytomers | 702.5 | 611.3 | 0.0 | 91.2 | 0.0 |
| axes.units | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 |
| draws | 103.0 | 56.0 | 24.0 | 7.3 | 15.7 |
| draws.units | 81.8 | 71.3 | 0.0 | 10.6 | 0.0 |
| draws.nodes | 78.1 | 67.9 | 0.0 | 10.1 | 0.0 |
| draws.shade | 20.5 | 17.8 | 0.0 | 2.6 | 0.0 |
| pencils | 131.9 | 80.9 | 34.7 | 10.6 | 5.7 |
| pencils.inherited | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 |
| pencils.path | 56.0 | 34.0 | 16.9 | 5.1 | 0.0 |
| successor | 17.2 | 9.3 | 4.0 | 1.2 | 2.6 |
| successor.each | 0.2 | 0.2 | 0.0 | 0.0 | 0.0 |
| working.links | 16.8 | 8.1 | 4.0 | 1.2 | 3.5 |
| working.by_node | 16.8 | 8.1 | 4.0 | 1.2 | 3.5 |
| working.by_node.each | 4.2 | 3.3 | 0.7 | 0.2 | 0.0 |
| retire.fate | 7.5 | 4.0 | 2.0 | 0.6 | 0.9 |

Reader total 1,496.9 MiB. Attributed per axis (record and per-axis Vec capacity): kept 1,071.8, dropped retired 133.0, dropped other 155.2 MiB.

Stores not split by axis (global lists or per-leaf; largest first): leaves 32.0, live 8.0, retire.stands 6.0, horizon 5.7, units 5.7, roots 5.7, working.pa 5.6, working.list 4.0, working.at 2.8, working.light 2.8, working.cost 2.8, working.sub 2.8 MiB.

Axes in groups: kept 408,085, dropped retired (root/inside) 174,690, dropped other 53,386.

### Item 5. Dropped axes with no living apex: cycles since the apex ended against the PA's shedding delay

PA delays (cycles, `species.states[pa].shedding`): pa0 = inf, pa1 = inf, pa2 = inf, pa3 = inf, pa4 = inf, pa5 = inf, pa6 = inf, pa7 = inf, pa8 = inf, pa9 = 1.0, pa10 = 1.0, pa11 = 1.0, pa12 = 1.0, pa13 = 1.0, pa14 = 1.0.

| PA | cycles since apex end minus delay | axes | phytomers |
|---|---:|---:|---:|
| pa9 (delay 1.0) | 1 cycle past | 108 | 1,430 |
| pa9 (delay 1.0) | 2 cycles past | 80 | 421 |
| pa9 (delay 1.0) | 3 to 5 past | 264 | 1,983 |
| pa9 (delay 1.0) | 6 to 10 past | 162 | 2,632 |
| pa9 (delay 1.0) | over 10 past | 1,591 | 21,576 |
| pa12 (delay 1.0) | at the delay (0) | 7 | 101 |
| pa12 (delay 1.0) | 1 cycle past | 806 | 3,428 |
| pa12 (delay 1.0) | 2 cycles past | 797 | 960 |
| pa12 (delay 1.0) | 3 to 5 past | 3,371 | 8,452 |
| pa12 (delay 1.0) | 6 to 10 past | 6,903 | 34,027 |
| pa12 (delay 1.0) | over 10 past | 14,318 | 36,773 |
| pa14 (delay 1.0) | before the delay (ago < delay) | 284 | 13,651 |
| pa14 (delay 1.0) | at the delay (0) | 1,017 | 24,618 |
| pa14 (delay 1.0) | 1 cycle past | 41,333 | 721,390 |
| pa14 (delay 1.0) | 2 cycles past | 39,607 | 13,425 |
| pa14 (delay 1.0) | 3 to 5 past | 113,686 | 19,022 |
| pa14 (delay 1.0) | 6 to 10 past | 1,687 | 13,309 |
| pa14 (delay 1.0) | over 10 past | 2,055 | 19,281 |

Same, split by whether the axis is itself the shed root (own) or dropped with its bearer (below):

| class | axes | phytomers |
|---|---:|---:|
| below: 1 cycle past | 7,404 | 202,622 |
| below: 2 cycles past | 8,742 | 10,094 |
| below: 3 to 5 past | 37,690 | 13,556 |
| below: 6 to 10 past | 2,286 | 11,292 |
| below: at the delay (0) | 445 | 10,751 |
| below: before the delay (ago < delay) | 125 | 6,031 |
| below: over 10 past | 6,262 | 31,047 |
| own: 1 cycle past | 34,843 | 523,626 |
| own: 2 cycles past | 31,742 | 4,712 |
| own: 3 to 5 past | 79,631 | 15,901 |
| own: 6 to 10 past | 6,466 | 38,676 |
| own: at the delay (0) | 579 | 13,968 |
| own: before the delay (ago < delay) | 159 | 7,620 |
| own: over 10 past | 11,702 | 46,583 |

Exact cycles since the apex ended, per PA:

| PA | delay | cycles ago | axes | phytomers |
|---|---:|---:|---:|---:|
| pa9 | 1.0 | 2 | 108 | 1,430 |
| pa9 | 1.0 | 3 | 80 | 421 |
| pa9 | 1.0 | 4 | 101 | 831 |
| pa9 | 1.0 | 5 | 75 | 726 |
| pa9 | 1.0 | 6 | 88 | 426 |
| pa9 | 1.0 | 7 | 55 | 895 |
| pa9 | 1.0 | 8 | 37 | 396 |
| pa9 | 1.0 | 9 | 33 | 559 |
| pa9 | 1.0 | 10 | 16 | 302 |
| pa9 | 1.0 | 11 | 21 | 480 |
| pa9 | 1.0 | 12 | 16 | 287 |
| pa9 | 1.0 | 13 | 36 | 602 |
| pa9 | 1.0 | 14 | 32 | 584 |
| pa9 | 1.0 | 15 | 57 | 1,541 |
| pa9 | 1.0 | 16 | 94 | 1,786 |
| pa9 | 1.0 | 17 | 140 | 2,689 |
| pa9 | 1.0 | 18 | 183 | 2,942 |
| pa9 | 1.0 | 19 | 204 | 2,782 |
| pa9 | 1.0 | 20 | 176 | 509 |
| pa9 | 1.0 | 21 | 152 | 678 |
| pa9 | 1.0 | 22 | 135 | 469 |
| pa9 | 1.0 | 23 | 70 | 569 |
| pa9 | 1.0 | 24 | 46 | 1,056 |
| pa9 | 1.0 | 25 | 37 | 699 |
| pa9 | 1.0 | 26 | 26 | 471 |
| pa9 | 1.0 | 27 | 37 | 736 |
| pa9 | 1.0 | 28 | 22 | 387 |
| pa9 | 1.0 | 29 | 20 | 334 |
| pa9 | 1.0 | 30 | 6 | 123 |
| pa9 | 1.0 | 31 | 14 | 112 |
| pa9 | 1.0 | 32 | 13 | 381 |
| pa9 | 1.0 | 33 | 7 | 111 |
| pa9 | 1.0 | 34 | 22 | 448 |
| pa9 | 1.0 | 35 | 7 | 214 |
| pa9 | 1.0 | 36 | 6 | 187 |
| pa9 | 1.0 | 37 | 11 | 379 |
| pa9 | 1.0 | 38 | 3 | 104 |
| pa9 | 1.0 | 39 | 2 | 26 |
| pa9 | 1.0 | 40 | 3 | 74 |
| pa9 | 1.0 | 41 | 5 | 119 |
| pa9 | 1.0 | 42 | 2 | 62 |
| pa9 | 1.0 | 43 | 4 | 38 |
| pa9 | 1.0 | 46 | 1 | 0 |
| pa9 | 1.0 | 47 | 2 | 77 |
| pa12 | 1.0 | 1 | 7 | 101 |
| pa12 | 1.0 | 2 | 806 | 3,428 |
| pa12 | 1.0 | 3 | 797 | 960 |
| pa12 | 1.0 | 4 | 985 | 2,161 |
| pa12 | 1.0 | 5 | 1,099 | 2,546 |
| pa12 | 1.0 | 6 | 1,287 | 3,745 |
| pa12 | 1.0 | 7 | 920 | 4,720 |
| pa12 | 1.0 | 8 | 1,101 | 4,718 |
| pa12 | 1.0 | 9 | 1,279 | 5,619 |
| pa12 | 1.0 | 10 | 1,629 | 8,607 |
| pa12 | 1.0 | 11 | 1,974 | 10,363 |
| pa12 | 1.0 | 12 | 2,219 | 10,075 |
| pa12 | 1.0 | 13 | 2,558 | 10,715 |
| pa12 | 1.0 | 14 | 2,813 | 10,555 |
| pa12 | 1.0 | 15 | 2,329 | 106 |
| pa12 | 1.0 | 16 | 1,875 | 329 |
| pa12 | 1.0 | 17 | 1,292 | 207 |
| pa12 | 1.0 | 18 | 730 | 375 |
| pa12 | 1.0 | 19 | 46 | 298 |
| pa12 | 1.0 | 20 | 44 | 246 |
| pa12 | 1.0 | 21 | 28 | 208 |
| pa12 | 1.0 | 22 | 31 | 217 |
| pa12 | 1.0 | 23 | 26 | 177 |
| pa12 | 1.0 | 24 | 42 | 276 |
| pa12 | 1.0 | 25 | 28 | 266 |
| pa12 | 1.0 | 26 | 20 | 157 |
| pa12 | 1.0 | 27 | 49 | 482 |
| pa12 | 1.0 | 28 | 12 | 173 |
| pa12 | 1.0 | 29 | 15 | 183 |
| pa12 | 1.0 | 30 | 17 | 179 |
| pa12 | 1.0 | 31 | 10 | 113 |
| pa12 | 1.0 | 32 | 12 | 126 |
| pa12 | 1.0 | 33 | 4 | 47 |
| pa12 | 1.0 | 34 | 40 | 341 |
| pa12 | 1.0 | 35 | 6 | 64 |
| pa12 | 1.0 | 36 | 12 | 129 |
| pa12 | 1.0 | 37 | 23 | 207 |
| pa12 | 1.0 | 38 | 12 | 175 |
| pa12 | 1.0 | 39 | 6 | 76 |
| pa12 | 1.0 | 40 | 5 | 60 |
| pa12 | 1.0 | 41 | 5 | 58 |
| pa12 | 1.0 | 42 | 3 | 47 |
| pa12 | 1.0 | 43 | 2 | 36 |
| pa12 | 1.0 | 44 | 2 | 35 |
| pa12 | 1.0 | 45 | 2 | 35 |
| pa14 | 1.0 | 0 | 284 | 13,651 |
| pa14 | 1.0 | 1 | 1,017 | 24,618 |
| pa14 | 1.0 | 2 | 41,333 | 721,390 |
| pa14 | 1.0 | 3 | 39,607 | 13,425 |
| pa14 | 1.0 | 4 | 39,976 | 11,815 |
| pa14 | 1.0 | 5 | 37,546 | 3,238 |
| pa14 | 1.0 | 6 | 36,164 | 3,969 |
| pa14 | 1.0 | 7 | 839 | 4,963 |
| pa14 | 1.0 | 8 | 290 | 2,584 |
| pa14 | 1.0 | 9 | 220 | 2,378 |
| pa14 | 1.0 | 10 | 92 | 1,197 |
| pa14 | 1.0 | 11 | 246 | 2,187 |
| pa14 | 1.0 | 12 | 60 | 840 |
| pa14 | 1.0 | 13 | 60 | 718 |
| pa14 | 1.0 | 14 | 85 | 894 |
| pa14 | 1.0 | 15 | 35 | 310 |
| pa14 | 1.0 | 16 | 198 | 2,082 |
| pa14 | 1.0 | 17 | 37 | 264 |
| pa14 | 1.0 | 18 | 198 | 1,450 |
| pa14 | 1.0 | 19 | 95 | 724 |
| pa14 | 1.0 | 20 | 205 | 2,816 |
| pa14 | 1.0 | 21 | 9 | 238 |
| pa14 | 1.0 | 22 | 6 | 137 |
| pa14 | 1.0 | 23 | 7 | 45 |
| pa14 | 1.0 | 24 | 163 | 1,126 |
| pa14 | 1.0 | 25 | 76 | 524 |
| pa14 | 1.0 | 26 | 3 | 13 |
| pa14 | 1.0 | 27 | 273 | 2,595 |
| pa14 | 1.0 | 28 | 11 | 76 |
| pa14 | 1.0 | 29 | 37 | 325 |
| pa14 | 1.0 | 30 | 1 | 0 |
| pa14 | 1.0 | 32 | 77 | 954 |
| pa14 | 1.0 | 33 | 7 | 39 |
| pa14 | 1.0 | 34 | 256 | 1,851 |
| pa14 | 1.0 | 35 | 3 | 15 |
| pa14 | 1.0 | 36 | 4 | 25 |
| pa14 | 1.0 | 37 | 149 | 1,220 |

Dropped axes with a living apex (apex_end none): 0 axes, 0 phytomers.


## Tree 2. Shipped European beech, age 80, seed 1

### Item 1. Held, kept, difference

|  | axes | phytomers |
|---|---:|---:|
| held just before the settle (axes in the grower) | 427,645 | 5,756,748 |
| kept by the shed rule (`shed_past`, probe replica) | 321,607 | 5,037,729 |
| dropped (difference) | 106,038 | 719,019 |
| kept by the real `shed_past` (settle_shed, printed after it) | 321,607 | 5,037,729 |
| settled structure `grow()` returns (after scale, thicken, place) | 321,607 | 5,037,729 |
| `rules.grown` (phytomers grown for the tree, shed ones included; `Cycled.grown`) | - | 17,583,856 |

Living apexes in the grower: 232,842. Retirement table length: 385,690 axes taken in (41,955 laid in the last cycle, not yet taken in). Retired roots whose records were let go earlier and still stand in the tree's fades (`Retirement::standing`, not held as axes): 170,405.

### Item 2. Held split by the axis's fate at the settle

| fate | axes | share | phytomers | share |
|---|---:|---:|---:|---:|
| kept at full scale (applied fade 1) | 314,801 | 73.6% | 4,870,977 | 84.6% |
| kept faded, scale over 0.5 | 2,718 | 0.6% | 68,473 | 1.2% |
| kept faded, 0.1 to 0.5 | 3,242 | 0.8% | 76,602 | 1.3% |
| kept faded, 0.01 to 0.1 | 770 | 0.2% | 19,492 | 0.3% |
| kept faded, 0.01 or under | 76 | 0.0% | 2,185 | 0.0% |
| dropped, shed rule via its bearer (an ancestor was shed) | 20,203 | 4.7% | 194,225 | 3.4% |
| dropped, shed rule at its own fade (axis is the shed root) | 85,835 | 20.1% | 524,794 | 9.1% |
| dropped for another reason | 0 | 0.0% | 0 | 0.0% |

Dropped for another reason: the shed loop has only the two exits (bearer lost, own fade at nothing); none else exists, so 0 by construction.

### Item 3. Dropped split

By PA:

| PA | axes | share | phytomers | share |
|---|---:|---:|---:|---:|
| pa9 | 764 | 0.7% | 11,561 | 1.6% |
| pa12 | 11,688 | 11.0% | 58,675 | 8.2% |
| pa14 | 93,586 | 88.3% | 648,783 | 90.2% |

By apex:

| apex | axes | share | phytomers | share |
|---|---:|---:|---:|---:|
| apex ended 0 cycles ago | 252 | 0.2% | 12,136 | 1.7% |
| apex ended 1 cycles ago | 806 | 0.8% | 20,283 | 2.8% |
| apex ended 2 cycles ago | 32,260 | 30.4% | 587,188 | 81.7% |
| apex ended 3-5 cycles ago | 61,419 | 57.9% | 20,706 | 2.9% |
| apex ended 6-10 cycles ago | 3,264 | 3.1% | 28,536 | 4.0% |
| apex ended >10 cycles ago | 8,037 | 7.6% | 50,170 | 7.0% |

By working set:

| working set | axes | share | phytomers | share |
|---|---:|---:|---:|---:|
| closed | 105,096 | 99.1% | 689,985 | 96.0% |
| open | 942 | 0.9% | 29,034 | 4.0% |

By retired status:

| retired status | axes | share | phytomers | share |
|---|---:|---:|---:|---:|
| a retired root's stand (axis record held) | 57,240 | 54.0% | 0 | 0.0% |
| inside a retired subtree | 11,309 | 10.7% | 0 | 0.0% |
| not retired, fate kept (fading list) | 30,253 | 28.5% | 485,189 | 67.5% |
| not retired, fate open | 7,236 | 6.8% | 233,830 | 32.5% |

By later years' passes:

| later years' passes | axes | share | phytomers | share |
|---|---:|---:|---:|---:|
| no | 68,549 | 64.6% | 0 | 0.0% |
| yes (fate Kept: a lateral that sheds waits in the fading list and is evaluated once its fade is none, retire.rs faded_out; otherwise read inside an ancestor's evaluation) | 30,253 | 28.5% | 485,189 | 67.5% |
| yes (open: balance and allotment visit it, working.rs) | 942 | 0.9% | 29,034 | 4.0% |
| yes (retire.rs evaluation of the closed subtree) | 6,294 | 5.9% | 204,796 | 28.5% |

### Item 4. Footprint by store, kept and dropped

| store | reader MiB | kept axes MiB | dropped, retired MiB | dropped, other MiB | slack / unsplit MiB |
|---|---:|---:|---:|---:|---:|
| axes | 120.1 | 71.2 | 15.2 | 8.3 | 25.4 |
| axes.phytomers | 569.5 | 499.6 | 0.0 | 69.9 | 0.0 |
| axes.units | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 |
| draws | 74.5 | 44.2 | 9.4 | 5.1 | 15.8 |
| draws.units | 65.9 | 57.9 | 0.0 | 8.0 | 0.0 |
| draws.nodes | 63.3 | 55.5 | 0.0 | 7.8 | 0.0 |
| draws.shade | 16.5 | 14.5 | 0.0 | 2.0 | 0.0 |
| pencils | 107.6 | 63.8 | 13.6 | 7.4 | 22.8 |
| pencils.inherited | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 |
| pencils.path | 38.5 | 27.8 | 6.9 | 3.9 | 0.0 |
| successor | 12.4 | 7.4 | 1.6 | 0.9 | 2.6 |
| successor.each | 0.1 | 0.1 | 0.0 | 0.0 | 0.0 |
| working.links | 10.8 | 6.4 | 1.6 | 0.9 | 2.0 |
| working.by_node | 10.8 | 6.4 | 1.6 | 0.9 | 2.0 |
| working.by_node.each | 3.0 | 2.6 | 0.3 | 0.2 | 0.0 |
| retire.fate | 5.4 | 3.2 | 0.8 | 0.4 | 1.0 |

Reader total 1,175.1 MiB. Attributed per axis (record and per-axis Vec capacity): kept 867.8, dropped retired 52.4, dropped other 116.5 MiB.

Stores not split by axis (global lists or per-leaf; largest first): leaves 32.0, retire.stands 6.0, horizon 4.1, units 4.1, roots 4.1, live 4.0, working.list 4.0, working.pa 3.6, working.at 1.8, working.light 1.8, working.cost 1.8, working.sub 1.8 MiB.

Axes in groups: kept 321,607, dropped retired (root/inside) 68,549, dropped other 37,489.

### Item 5. Dropped axes with no living apex: cycles since the apex ended against the PA's shedding delay

PA delays (cycles, `species.states[pa].shedding`): pa0 = inf, pa1 = inf, pa2 = inf, pa3 = inf, pa4 = inf, pa5 = inf, pa6 = inf, pa7 = inf, pa8 = inf, pa9 = 1.0, pa10 = 1.0, pa11 = 1.0, pa12 = 1.0, pa13 = 1.0, pa14 = 1.0.

| PA | cycles since apex end minus delay | axes | phytomers |
|---|---:|---:|---:|
| pa9 (delay 1.0) | at the delay (0) | 1 | 26 |
| pa9 (delay 1.0) | 1 cycle past | 39 | 429 |
| pa9 (delay 1.0) | 2 cycles past | 32 | 86 |
| pa9 (delay 1.0) | 3 to 5 past | 50 | 328 |
| pa9 (delay 1.0) | 6 to 10 past | 21 | 280 |
| pa9 (delay 1.0) | over 10 past | 621 | 10,412 |
| pa12 (delay 1.0) | at the delay (0) | 5 | 32 |
| pa12 (delay 1.0) | 1 cycle past | 438 | 1,302 |
| pa12 (delay 1.0) | 2 cycles past | 509 | 565 |
| pa12 (delay 1.0) | 3 to 5 past | 1,092 | 4,682 |
| pa12 (delay 1.0) | 6 to 10 past | 3,443 | 24,059 |
| pa12 (delay 1.0) | over 10 past | 6,201 | 28,035 |
| pa14 (delay 1.0) | before the delay (ago < delay) | 252 | 12,136 |
| pa14 (delay 1.0) | at the delay (0) | 800 | 20,225 |
| pa14 (delay 1.0) | 1 cycle past | 31,783 | 585,457 |
| pa14 (delay 1.0) | 2 cycles past | 30,455 | 11,454 |
| pa14 (delay 1.0) | 3 to 5 past | 29,696 | 8,007 |
| pa14 (delay 1.0) | 6 to 10 past | 436 | 7,842 |
| pa14 (delay 1.0) | over 10 past | 164 | 3,662 |

Same, split by whether the axis is itself the shed root (own) or dropped with its bearer (below):

| class | axes | phytomers |
|---|---:|---:|
| below: 1 cycle past | 4,902 | 157,416 |
| below: 2 cycles past | 5,885 | 8,074 |
| below: 3 to 5 past | 7,131 | 1,542 |
| below: 6 to 10 past | 435 | 4,016 |
| below: at the delay (0) | 324 | 8,498 |
| below: before the delay (ago < delay) | 115 | 5,572 |
| below: over 10 past | 1,411 | 9,107 |
| own: 1 cycle past | 27,358 | 429,772 |
| own: 2 cycles past | 25,111 | 4,031 |
| own: 3 to 5 past | 23,707 | 11,475 |
| own: 6 to 10 past | 3,465 | 28,165 |
| own: at the delay (0) | 482 | 11,785 |
| own: before the delay (ago < delay) | 137 | 6,564 |
| own: over 10 past | 5,575 | 33,002 |

Exact cycles since the apex ended, per PA:

| PA | delay | cycles ago | axes | phytomers |
|---|---:|---:|---:|---:|
| pa9 | 1.0 | 1 | 1 | 26 |
| pa9 | 1.0 | 2 | 39 | 429 |
| pa9 | 1.0 | 3 | 32 | 86 |
| pa9 | 1.0 | 4 | 42 | 115 |
| pa9 | 1.0 | 5 | 3 | 77 |
| pa9 | 1.0 | 6 | 5 | 136 |
| pa9 | 1.0 | 7 | 4 | 5 |
| pa9 | 1.0 | 8 | 4 | 7 |
| pa9 | 1.0 | 9 | 3 | 0 |
| pa9 | 1.0 | 10 | 8 | 182 |
| pa9 | 1.0 | 11 | 2 | 86 |
| pa9 | 1.0 | 12 | 6 | 69 |
| pa9 | 1.0 | 13 | 10 | 213 |
| pa9 | 1.0 | 14 | 13 | 205 |
| pa9 | 1.0 | 15 | 20 | 401 |
| pa9 | 1.0 | 16 | 47 | 979 |
| pa9 | 1.0 | 17 | 71 | 1,671 |
| pa9 | 1.0 | 18 | 93 | 1,831 |
| pa9 | 1.0 | 19 | 128 | 2,094 |
| pa9 | 1.0 | 20 | 84 | 129 |
| pa9 | 1.0 | 21 | 40 | 199 |
| pa9 | 1.0 | 22 | 10 | 392 |
| pa9 | 1.0 | 23 | 14 | 422 |
| pa9 | 1.0 | 24 | 10 | 250 |
| pa9 | 1.0 | 25 | 15 | 311 |
| pa9 | 1.0 | 26 | 7 | 131 |
| pa9 | 1.0 | 27 | 8 | 33 |
| pa9 | 1.0 | 28 | 11 | 208 |
| pa9 | 1.0 | 29 | 8 | 127 |
| pa9 | 1.0 | 30 | 6 | 196 |
| pa9 | 1.0 | 31 | 1 | 65 |
| pa9 | 1.0 | 32 | 6 | 91 |
| pa9 | 1.0 | 33 | 5 | 133 |
| pa9 | 1.0 | 34 | 2 | 30 |
| pa9 | 1.0 | 35 | 2 | 53 |
| pa9 | 1.0 | 37 | 1 | 66 |
| pa9 | 1.0 | 39 | 1 | 38 |
| pa9 | 1.0 | 43 | 1 | 12 |
| pa9 | 1.0 | 45 | 1 | 63 |
| pa12 | 1.0 | 1 | 5 | 32 |
| pa12 | 1.0 | 2 | 438 | 1,302 |
| pa12 | 1.0 | 3 | 509 | 565 |
| pa12 | 1.0 | 4 | 554 | 1,039 |
| pa12 | 1.0 | 5 | 214 | 1,453 |
| pa12 | 1.0 | 6 | 324 | 2,190 |
| pa12 | 1.0 | 7 | 414 | 2,687 |
| pa12 | 1.0 | 8 | 577 | 3,609 |
| pa12 | 1.0 | 9 | 616 | 3,908 |
| pa12 | 1.0 | 10 | 820 | 6,472 |
| pa12 | 1.0 | 11 | 1,016 | 7,383 |
| pa12 | 1.0 | 12 | 1,342 | 9,114 |
| pa12 | 1.0 | 13 | 1,538 | 9,458 |
| pa12 | 1.0 | 14 | 1,591 | 8,662 |
| pa12 | 1.0 | 15 | 1,077 | 135 |
| pa12 | 1.0 | 16 | 543 | 58 |
| pa12 | 1.0 | 17 | 20 | 149 |
| pa12 | 1.0 | 18 | 16 | 72 |
| pa12 | 1.0 | 19 | 13 | 60 |
| pa12 | 1.0 | 20 | 10 | 34 |
| pa12 | 1.0 | 21 | 10 | 52 |
| pa12 | 1.0 | 22 | 9 | 10 |
| pa12 | 1.0 | 23 | 4 | 18 |
| pa12 | 1.0 | 24 | 4 | 18 |
| pa12 | 1.0 | 25 | 4 | 20 |
| pa12 | 1.0 | 26 | 5 | 80 |
| pa12 | 1.0 | 27 | 4 | 23 |
| pa12 | 1.0 | 28 | 2 | 34 |
| pa12 | 1.0 | 29 | 4 | 4 |
| pa12 | 1.0 | 30 | 1 | 0 |
| pa12 | 1.0 | 31 | 1 | 0 |
| pa12 | 1.0 | 32 | 1 | 17 |
| pa12 | 1.0 | 35 | 1 | 0 |
| pa12 | 1.0 | 40 | 1 | 17 |
| pa14 | 1.0 | 0 | 252 | 12,136 |
| pa14 | 1.0 | 1 | 800 | 20,225 |
| pa14 | 1.0 | 2 | 31,783 | 585,457 |
| pa14 | 1.0 | 3 | 30,455 | 11,454 |
| pa14 | 1.0 | 4 | 29,306 | 3,117 |
| pa14 | 1.0 | 5 | 304 | 2,800 |
| pa14 | 1.0 | 6 | 86 | 2,090 |
| pa14 | 1.0 | 7 | 108 | 2,570 |
| pa14 | 1.0 | 8 | 55 | 1,192 |
| pa14 | 1.0 | 9 | 38 | 885 |
| pa14 | 1.0 | 10 | 202 | 2,603 |
| pa14 | 1.0 | 11 | 33 | 592 |
| pa14 | 1.0 | 12 | 42 | 728 |
| pa14 | 1.0 | 13 | 28 | 641 |
| pa14 | 1.0 | 14 | 27 | 716 |
| pa14 | 1.0 | 15 | 17 | 403 |
| pa14 | 1.0 | 16 | 15 | 328 |
| pa14 | 1.0 | 17 | 8 | 188 |
| pa14 | 1.0 | 18 | 6 | 159 |
| pa14 | 1.0 | 19 | 5 | 41 |
| pa14 | 1.0 | 20 | 3 | 105 |
| pa14 | 1.0 | 21 | 4 | 43 |
| pa14 | 1.0 | 22 | 1 | 0 |
| pa14 | 1.0 | 23 | 2 | 70 |
| pa14 | 1.0 | 24 | 1 | 24 |
| pa14 | 1.0 | 28 | 2 | 97 |
| pa14 | 1.0 | 29 | 1 | 34 |
| pa14 | 1.0 | 31 | 1 | 49 |
| pa14 | 1.0 | 36 | 1 | 36 |

Dropped axes with a living apex (apex_end none): 0 axes, 0 phytomers.


## Tree 3. Shipped European beech, age 40, seed 7

### Item 1. Held, kept, difference

|  | axes | phytomers |
|---|---:|---:|
| held just before the settle (axes in the grower) | 77,365 | 869,659 |
| kept by the shed rule (`shed_past`, probe replica) | 67,311 | 796,872 |
| dropped (difference) | 10,054 | 72,787 |
| kept by the real `shed_past` (settle_shed, printed after it) | 67,311 | 796,872 |
| settled structure `grow()` returns (after scale, thicken, place) | 67,311 | 796,872 |
| `rules.grown` (phytomers grown for the tree, shed ones included; `Cycled.grown`) | - | 1,273,998 |

Living apexes in the grower: 53,094. Retirement table length: 64,810 axes taken in (12,555 laid in the last cycle, not yet taken in). Retired roots whose records were let go earlier and still stand in the tree's fades (`Retirement::standing`, not held as axes): 17,653.

### Item 2. Held split by the axis's fate at the settle

| fate | axes | share | phytomers | share |
|---|---:|---:|---:|---:|
| kept at full scale (applied fade 1) | 66,522 | 86.0% | 780,088 | 89.7% |
| kept faded, scale over 0.5 | 345 | 0.4% | 6,901 | 0.8% |
| kept faded, 0.1 to 0.5 | 368 | 0.5% | 8,118 | 0.9% |
| kept faded, 0.01 to 0.1 | 68 | 0.1% | 1,619 | 0.2% |
| kept faded, 0.01 or under | 8 | 0.0% | 146 | 0.0% |
| dropped, shed rule via its bearer (an ancestor was shed) | 765 | 1.0% | 12,843 | 1.5% |
| dropped, shed rule at its own fade (axis is the shed root) | 9,289 | 12.0% | 59,944 | 6.9% |
| dropped for another reason | 0 | 0.0% | 0 | 0.0% |

Dropped for another reason: the shed loop has only the two exits (bearer lost, own fade at nothing); none else exists, so 0 by construction.

### Item 3. Dropped split

By PA:

| PA | axes | share | phytomers | share |
|---|---:|---:|---:|---:|
| pa9 | 30 | 0.3% | 204 | 0.3% |
| pa12 | 694 | 6.9% | 4,325 | 5.9% |
| pa14 | 9,330 | 92.8% | 68,258 | 93.8% |

By apex:

| apex | axes | share | phytomers | share |
|---|---:|---:|---:|---:|
| apex ended 0 cycles ago | 19 | 0.2% | 922 | 1.3% |
| apex ended 1 cycles ago | 70 | 0.7% | 1,683 | 2.3% |
| apex ended 2 cycles ago | 5,045 | 50.2% | 64,893 | 89.2% |
| apex ended 3-5 cycles ago | 4,456 | 44.3% | 1,069 | 1.5% |
| apex ended 6-10 cycles ago | 180 | 1.8% | 1,739 | 2.4% |
| apex ended >10 cycles ago | 284 | 2.8% | 2,481 | 3.4% |

By working set:

| working set | axes | share | phytomers | share |
|---|---:|---:|---:|---:|
| closed | 9,960 | 99.1% | 70,135 | 96.4% |
| open | 94 | 0.9% | 2,652 | 3.6% |

By retired status:

| retired status | axes | share | phytomers | share |
|---|---:|---:|---:|---:|
| a retired root's stand (axis record held) | 4,381 | 43.6% | 0 | 0.0% |
| inside a retired subtree | 277 | 2.8% | 0 | 0.0% |
| not retired, fate kept (fading list) | 4,931 | 49.0% | 57,703 | 79.3% |
| not retired, fate open | 465 | 4.6% | 15,084 | 20.7% |

By later years' passes:

| later years' passes | axes | share | phytomers | share |
|---|---:|---:|---:|---:|
| no | 4,658 | 46.3% | 0 | 0.0% |
| yes (fate Kept: a lateral that sheds waits in the fading list and is evaluated once its fade is none, retire.rs faded_out; otherwise read inside an ancestor's evaluation) | 4,931 | 49.0% | 57,703 | 79.3% |
| yes (open: balance and allotment visit it, working.rs) | 94 | 0.9% | 2,652 | 3.6% |
| yes (retire.rs evaluation of the closed subtree) | 371 | 3.7% | 12,432 | 17.1% |

### Item 4. Footprint by store, kept and dropped

| store | reader MiB | kept axes MiB | dropped, retired MiB | dropped, other MiB | slack / unsplit MiB |
|---|---:|---:|---:|---:|---:|
| axes | 22.4 | 14.9 | 1.0 | 1.2 | 5.3 |
| axes.phytomers | 85.0 | 77.9 | 0.0 | 7.0 | 0.0 |
| axes.units | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 |
| draws | 13.9 | 9.2 | 0.6 | 0.7 | 3.3 |
| draws.units | 10.4 | 9.6 | 0.0 | 0.9 | 0.0 |
| draws.nodes | 9.4 | 8.7 | 0.0 | 0.8 | 0.0 |
| draws.shade | 2.6 | 2.4 | 0.0 | 0.2 | 0.0 |
| pencils | 25.7 | 13.4 | 0.9 | 1.1 | 10.4 |
| pencils.inherited | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 |
| pencils.path | 5.0 | 4.3 | 0.3 | 0.4 | 0.0 |
| successor | 2.3 | 1.5 | 0.1 | 0.1 | 0.5 |
| successor.each | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 |
| working.links | 2.4 | 1.3 | 0.1 | 0.1 | 1.0 |
| working.by_node | 2.4 | 1.3 | 0.1 | 0.1 | 1.0 |
| working.by_node.each | 0.4 | 0.4 | 0.0 | 0.0 | 0.0 |
| retire.fate | 0.9 | 0.6 | 0.1 | 0.1 | 0.1 |

Reader total 199.5 MiB. Attributed per axis (record and per-axis Vec capacity): kept 147.1, dropped retired 3.4, dropped other 12.9 MiB.

Stores not split by axis (global lists or per-leaf; largest first): leaves 8.0, live 1.0, working.pa 0.8, horizon 0.8, units 0.8, roots 0.8, retire.stands 0.8, working.list 0.5, working.at 0.4, working.light 0.4, working.cost 0.4, working.sub 0.4 MiB.

Axes in groups: kept 67,311, dropped retired (root/inside) 4,658, dropped other 5,396.

### Item 5. Dropped axes with no living apex: cycles since the apex ended against the PA's shedding delay

PA delays (cycles, `species.states[pa].shedding`): pa0 = inf, pa1 = inf, pa2 = inf, pa3 = inf, pa4 = inf, pa5 = inf, pa6 = inf, pa7 = inf, pa8 = inf, pa9 = 1.0, pa10 = 1.0, pa11 = 1.0, pa12 = 1.0, pa13 = 1.0, pa14 = 1.0.

| PA | cycles since apex end minus delay | axes | phytomers |
|---|---:|---:|---:|
| pa9 (delay 1.0) | 1 cycle past | 9 | 0 |
| pa9 (delay 1.0) | 2 cycles past | 10 | 0 |
| pa9 (delay 1.0) | 6 to 10 past | 1 | 7 |
| pa9 (delay 1.0) | over 10 past | 10 | 197 |
| pa12 (delay 1.0) | 1 cycle past | 94 | 86 |
| pa12 (delay 1.0) | 2 cycles past | 97 | 55 |
| pa12 (delay 1.0) | 3 to 5 past | 79 | 553 |
| pa12 (delay 1.0) | 6 to 10 past | 206 | 1,936 |
| pa12 (delay 1.0) | over 10 past | 218 | 1,695 |
| pa14 (delay 1.0) | before the delay (ago < delay) | 19 | 922 |
| pa14 (delay 1.0) | at the delay (0) | 70 | 1,683 |
| pa14 (delay 1.0) | 1 cycle past | 4,942 | 64,807 |
| pa14 (delay 1.0) | 2 cycles past | 4,283 | 578 |
| pa14 (delay 1.0) | 3 to 5 past | 13 | 129 |
| pa14 (delay 1.0) | 6 to 10 past | 3 | 139 |

Same, split by whether the axis is itself the shed root (own) or dropped with its bearer (below):

| class | axes | phytomers |
|---|---:|---:|
| below: 1 cycle past | 340 | 11,119 |
| below: 2 cycles past | 360 | 451 |
| below: 3 to 5 past | 11 | 60 |
| below: 6 to 10 past | 2 | 34 |
| below: at the delay (0) | 18 | 471 |
| below: before the delay (ago < delay) | 12 | 574 |
| below: over 10 past | 22 | 134 |
| own: 1 cycle past | 4,705 | 53,774 |
| own: 2 cycles past | 4,030 | 182 |
| own: 3 to 5 past | 81 | 622 |
| own: 6 to 10 past | 208 | 2,048 |
| own: at the delay (0) | 52 | 1,212 |
| own: before the delay (ago < delay) | 7 | 348 |
| own: over 10 past | 206 | 1,758 |

Exact cycles since the apex ended, per PA:

| PA | delay | cycles ago | axes | phytomers |
|---|---:|---:|---:|---:|
| pa9 | 1.0 | 2 | 9 | 0 |
| pa9 | 1.0 | 3 | 10 | 0 |
| pa9 | 1.0 | 10 | 1 | 7 |
| pa9 | 1.0 | 13 | 1 | 6 |
| pa9 | 1.0 | 16 | 1 | 39 |
| pa9 | 1.0 | 17 | 1 | 0 |
| pa9 | 1.0 | 18 | 2 | 18 |
| pa9 | 1.0 | 19 | 3 | 134 |
| pa9 | 1.0 | 20 | 2 | 0 |
| pa12 | 1.0 | 2 | 94 | 86 |
| pa12 | 1.0 | 3 | 97 | 55 |
| pa12 | 1.0 | 4 | 22 | 128 |
| pa12 | 1.0 | 5 | 32 | 228 |
| pa12 | 1.0 | 6 | 25 | 197 |
| pa12 | 1.0 | 7 | 38 | 326 |
| pa12 | 1.0 | 8 | 35 | 242 |
| pa12 | 1.0 | 9 | 38 | 345 |
| pa12 | 1.0 | 10 | 39 | 434 |
| pa12 | 1.0 | 11 | 56 | 589 |
| pa12 | 1.0 | 12 | 56 | 592 |
| pa12 | 1.0 | 13 | 58 | 465 |
| pa12 | 1.0 | 14 | 69 | 638 |
| pa12 | 1.0 | 15 | 35 | 0 |
| pa14 | 1.0 | 0 | 19 | 922 |
| pa14 | 1.0 | 1 | 70 | 1,683 |
| pa14 | 1.0 | 2 | 4,942 | 64,807 |
| pa14 | 1.0 | 3 | 4,283 | 578 |
| pa14 | 1.0 | 4 | 12 | 80 |
| pa14 | 1.0 | 6 | 1 | 49 |
| pa14 | 1.0 | 8 | 2 | 97 |
| pa14 | 1.0 | 9 | 1 | 42 |

Dropped axes with a living apex (apex_end none): 0 axes, 0 phytomers.


## Tree 4. Shipped Oregon white oak (preset `oregon-white-oak`), age 120, seed 1001

### Item 1. Held, kept, difference

|  | axes | phytomers |
|---|---:|---:|
| held just before the settle (axes in the grower) | 577,092 | 3,863,297 |
| kept by the shed rule (`shed_past`, probe replica) | 401,667 | 3,100,796 |
| dropped (difference) | 175,425 | 762,501 |
| kept by the real `shed_past` (settle_shed, printed after it) | 401,667 | 3,100,796 |
| settled structure `grow()` returns (after scale, thicken, place) | 401,667 | 3,100,796 |
| `rules.grown` (phytomers grown for the tree, shed ones included; `Cycled.grown`) | - | 30,069,508 |

Living apexes in the grower: 189,672. Retirement table length: 500,273 axes taken in (76,819 laid in the last cycle, not yet taken in). Retired roots whose records were let go earlier and still stand in the tree's fades (`Retirement::standing`, not held as axes): 334,561.

### Item 2. Held split by the axis's fate at the settle

| fate | axes | share | phytomers | share |
|---|---:|---:|---:|---:|
| kept at full scale (applied fade 1) | 341,307 | 59.1% | 2,574,247 | 66.6% |
| kept faded, scale over 0.5 | 21,900 | 3.8% | 189,577 | 4.9% |
| kept faded, 0.1 to 0.5 | 26,488 | 4.6% | 230,662 | 6.0% |
| kept faded, 0.01 to 0.1 | 9,541 | 1.7% | 86,684 | 2.2% |
| kept faded, 0.01 or under | 2,431 | 0.4% | 19,626 | 0.5% |
| dropped, shed rule via its bearer (an ancestor was shed) | 57,123 | 9.9% | 254,863 | 6.6% |
| dropped, shed rule at its own fade (axis is the shed root) | 118,302 | 20.5% | 507,638 | 13.1% |
| dropped for another reason | 0 | 0.0% | 0 | 0.0% |

Dropped for another reason: the shed loop has only the two exits (bearer lost, own fade at nothing); none else exists, so 0 by construction.

### Item 3. Dropped split

By PA:

| PA | axes | share | phytomers | share |
|---|---:|---:|---:|---:|
| pa7 | 48 | 0.0% | 475 | 0.1% |
| pa9 | 2,840 | 1.6% | 53,602 | 7.0% |
| pa12 | 1,511 | 0.9% | 11,795 | 1.5% |
| pa13 | 15,323 | 8.7% | 148,411 | 19.5% |
| pa14 | 155,703 | 88.8% | 548,218 | 71.9% |

By apex:

| apex | axes | share | phytomers | share |
|---|---:|---:|---:|---:|
| apex ended 0 cycles ago | 46 | 0.0% | 561 | 0.1% |
| apex ended 1 cycles ago | 402 | 0.2% | 3,246 | 0.4% |
| apex ended 2 cycles ago | 932 | 0.5% | 7,272 | 1.0% |
| apex ended 3-5 cycles ago | 154,026 | 87.8% | 553,594 | 72.6% |
| apex ended 6-10 cycles ago | 11,266 | 6.4% | 97,964 | 12.8% |
| apex ended >10 cycles ago | 8,753 | 5.0% | 99,864 | 13.1% |

By working set:

| working set | axes | share | phytomers | share |
|---|---:|---:|---:|---:|
| closed | 175,067 | 99.8% | 756,774 | 99.2% |
| open | 358 | 0.2% | 5,727 | 0.8% |

By retired status:

| retired status | axes | share | phytomers | share |
|---|---:|---:|---:|---:|
| a retired root's stand (axis record held) | 59,934 | 34.2% | 0 | 0.0% |
| inside a retired subtree | 17,729 | 10.1% | 0 | 0.0% |
| not retired, fate kept (fading list) | 62,074 | 35.4% | 474,793 | 62.3% |
| not retired, fate open | 35,688 | 20.3% | 287,708 | 37.7% |

By later years' passes:

| later years' passes | axes | share | phytomers | share |
|---|---:|---:|---:|---:|
| no | 77,663 | 44.3% | 0 | 0.0% |
| yes (fate Kept: a lateral that sheds waits in the fading list and is evaluated once its fade is none, retire.rs faded_out; otherwise read inside an ancestor's evaluation) | 62,074 | 35.4% | 474,793 | 62.3% |
| yes (open: balance and allotment visit it, working.rs) | 358 | 0.2% | 5,727 | 0.8% |
| yes (retire.rs evaluation of the closed subtree) | 35,330 | 20.1% | 281,981 | 37.0% |

### Item 4. Footprint by store, kept and dropped

| store | reader MiB | kept axes MiB | dropped, retired MiB | dropped, other MiB | slack / unsplit MiB |
|---|---:|---:|---:|---:|---:|
| axes | 213.1 | 88.9 | 17.2 | 21.6 | 85.5 |
| axes.phytomers | 359.7 | 288.6 | 0.0 | 71.0 | 0.0 |
| axes.units | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 |
| draws | 132.3 | 55.2 | 10.7 | 13.4 | 53.0 |
| draws.units | 49.8 | 40.0 | 0.0 | 9.8 | 0.0 |
| draws.nodes | 40.0 | 32.1 | 0.0 | 7.9 | 0.0 |
| draws.shade | 12.4 | 10.0 | 0.0 | 2.4 | 0.0 |
| pencils | 191.1 | 79.7 | 15.4 | 19.4 | 76.6 |
| pencils.inherited | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 |
| pencils.path | 23.0 | 16.0 | 3.0 | 3.9 | 0.0 |
| successor | 22.0 | 9.2 | 1.8 | 2.2 | 8.8 |
| successor.each | 0.5 | 0.4 | 0.0 | 0.1 | 0.0 |
| working.links | 16.0 | 7.4 | 1.8 | 2.2 | 4.6 |
| working.by_node | 16.0 | 7.4 | 1.8 | 2.2 | 4.6 |
| working.by_node.each | 4.3 | 3.3 | 0.4 | 0.6 | 0.0 |
| retire.fate | 8.0 | 3.7 | 0.9 | 1.1 | 2.3 |

Reader total 1,188.7 MiB. Attributed per axis (record and per-axis Vec capacity): kept 651.1, dropped retired 54.7, dropped other 160.2 MiB.

Stores not split by axis (global lists or per-leaf; largest first): leaves 32.0, retire.stands 12.0, horizon 7.3, units 7.3, roots 7.3, working.pa 5.3, live 4.0, working.at 2.7, working.light 2.7, working.cost 2.7, working.sub 2.7, working.incoming 2.7 MiB.

Axes in groups: kept 401,667, dropped retired (root/inside) 77,663, dropped other 97,762.

### Item 5. Dropped axes with no living apex: cycles since the apex ended against the PA's shedding delay

PA delays (cycles, `species.states[pa].shedding`): pa0 = inf, pa1 = inf, pa2 = inf, pa3 = inf, pa4 = inf, pa5 = inf, pa6 = 5.0, pa7 = 4.0, pa8 = 3.0, pa9 = 3.0, pa10 = 3.0, pa11 = 2.0, pa12 = 2.0, pa13 = 2.0, pa14 = 2.0.

| PA | cycles since apex end minus delay | axes | phytomers |
|---|---:|---:|---:|
| pa7 (delay 4.0) | 3 to 5 past | 4 | 74 |
| pa7 (delay 4.0) | 6 to 10 past | 7 | 36 |
| pa7 (delay 4.0) | over 10 past | 37 | 365 |
| pa9 (delay 3.0) | before the delay (ago < delay) | 6 | 167 |
| pa9 (delay 3.0) | at the delay (0) | 7 | 115 |
| pa9 (delay 3.0) | 1 cycle past | 213 | 5,005 |
| pa9 (delay 3.0) | 2 cycles past | 194 | 928 |
| pa9 (delay 3.0) | 3 to 5 past | 156 | 2,519 |
| pa9 (delay 3.0) | 6 to 10 past | 555 | 12,293 |
| pa9 (delay 3.0) | over 10 past | 1,709 | 32,575 |
| pa12 (delay 2.0) | at the delay (0) | 1 | 14 |
| pa12 (delay 2.0) | 1 cycle past | 2 | 18 |
| pa12 (delay 2.0) | 2 cycles past | 29 | 233 |
| pa12 (delay 2.0) | 3 to 5 past | 128 | 1,166 |
| pa12 (delay 2.0) | 6 to 10 past | 424 | 3,142 |
| pa12 (delay 2.0) | over 10 past | 927 | 7,222 |
| pa13 (delay 2.0) | before the delay (ago < delay) | 25 | 274 |
| pa13 (delay 2.0) | at the delay (0) | 21 | 263 |
| pa13 (delay 2.0) | 1 cycle past | 1,872 | 16,054 |
| pa13 (delay 2.0) | 2 cycles past | 2,081 | 16,340 |
| pa13 (delay 2.0) | 3 to 5 past | 9,777 | 92,419 |
| pa13 (delay 2.0) | 6 to 10 past | 653 | 7,998 |
| pa13 (delay 2.0) | over 10 past | 894 | 15,063 |
| pa14 (delay 2.0) | before the delay (ago < delay) | 420 | 3,449 |
| pa14 (delay 2.0) | at the delay (0) | 907 | 6,912 |
| pa14 (delay 2.0) | 1 cycle past | 70,819 | 453,889 |
| pa14 (delay 2.0) | 2 cycles past | 70,324 | 29,679 |
| pa14 (delay 2.0) | 3 to 5 past | 6,941 | 11,418 |
| pa14 (delay 2.0) | 6 to 10 past | 2,311 | 14,288 |
| pa14 (delay 2.0) | over 10 past | 3,981 | 28,583 |

Same, split by whether the axis is itself the shed root (own) or dropped with its bearer (below):

| class | axes | phytomers |
|---|---:|---:|
| below: 1 cycle past | 14,902 | 110,858 |
| below: 2 cycles past | 24,681 | 35,763 |
| below: 3 to 5 past | 8,222 | 19,377 |
| below: 6 to 10 past | 3,099 | 25,987 |
| below: at the delay (0) | 530 | 3,648 |
| below: before the delay (ago < delay) | 349 | 2,610 |
| below: over 10 past | 5,340 | 56,620 |
| own: 1 cycle past | 58,004 | 364,108 |
| own: 2 cycles past | 47,947 | 11,417 |
| own: 3 to 5 past | 8,784 | 88,219 |
| own: 6 to 10 past | 851 | 11,770 |
| own: at the delay (0) | 406 | 3,656 |
| own: before the delay (ago < delay) | 102 | 1,280 |
| own: over 10 past | 2,208 | 27,188 |

Exact cycles since the apex ended, per PA:

| PA | delay | cycles ago | axes | phytomers |
|---|---:|---:|---:|---:|
| pa7 | 4.0 | 8 | 2 | 7 |
| pa7 | 4.0 | 9 | 2 | 67 |
| pa7 | 4.0 | 10 | 1 | 6 |
| pa7 | 4.0 | 11 | 1 | 5 |
| pa7 | 4.0 | 12 | 1 | 6 |
| pa7 | 4.0 | 13 | 1 | 7 |
| pa7 | 4.0 | 14 | 3 | 12 |
| pa7 | 4.0 | 15 | 2 | 12 |
| pa7 | 4.0 | 16 | 2 | 12 |
| pa7 | 4.0 | 17 | 2 | 11 |
| pa7 | 4.0 | 18 | 2 | 12 |
| pa7 | 4.0 | 19 | 2 | 11 |
| pa7 | 4.0 | 20 | 2 | 13 |
| pa7 | 4.0 | 21 | 2 | 13 |
| pa7 | 4.0 | 22 | 3 | 53 |
| pa7 | 4.0 | 23 | 2 | 11 |
| pa7 | 4.0 | 24 | 2 | 12 |
| pa7 | 4.0 | 25 | 1 | 6 |
| pa7 | 4.0 | 27 | 2 | 7 |
| pa7 | 4.0 | 28 | 1 | 6 |
| pa7 | 4.0 | 29 | 2 | 60 |
| pa7 | 4.0 | 30 | 2 | 48 |
| pa7 | 4.0 | 31 | 1 | 5 |
| pa7 | 4.0 | 32 | 1 | 7 |
| pa7 | 4.0 | 33 | 1 | 6 |
| pa7 | 4.0 | 34 | 1 | 5 |
| pa7 | 4.0 | 35 | 1 | 5 |
| pa7 | 4.0 | 36 | 1 | 5 |
| pa7 | 4.0 | 37 | 1 | 7 |
| pa7 | 4.0 | 38 | 1 | 38 |
| pa9 | 3.0 | 1 | 3 | 84 |
| pa9 | 3.0 | 2 | 3 | 83 |
| pa9 | 3.0 | 3 | 7 | 115 |
| pa9 | 3.0 | 4 | 213 | 5,005 |
| pa9 | 3.0 | 5 | 194 | 928 |
| pa9 | 3.0 | 6 | 34 | 349 |
| pa9 | 3.0 | 7 | 42 | 793 |
| pa9 | 3.0 | 8 | 80 | 1,377 |
| pa9 | 3.0 | 9 | 66 | 1,493 |
| pa9 | 3.0 | 10 | 64 | 1,605 |
| pa9 | 3.0 | 11 | 119 | 3,295 |
| pa9 | 3.0 | 12 | 167 | 4,503 |
| pa9 | 3.0 | 13 | 139 | 1,397 |
| pa9 | 3.0 | 14 | 109 | 2,118 |
| pa9 | 3.0 | 15 | 81 | 1,331 |
| pa9 | 3.0 | 16 | 85 | 1,577 |
| pa9 | 3.0 | 17 | 62 | 1,112 |
| pa9 | 3.0 | 18 | 75 | 1,452 |
| pa9 | 3.0 | 19 | 81 | 1,395 |
| pa9 | 3.0 | 20 | 75 | 1,383 |
| pa9 | 3.0 | 21 | 67 | 1,196 |
| pa9 | 3.0 | 22 | 63 | 1,091 |
| pa9 | 3.0 | 23 | 54 | 1,107 |
| pa9 | 3.0 | 24 | 51 | 952 |
| pa9 | 3.0 | 25 | 45 | 954 |
| pa9 | 3.0 | 26 | 48 | 929 |
| pa9 | 3.0 | 27 | 67 | 1,236 |
| pa9 | 3.0 | 28 | 45 | 845 |
| pa9 | 3.0 | 29 | 45 | 650 |
| pa9 | 3.0 | 30 | 46 | 961 |
| pa9 | 3.0 | 31 | 46 | 1,056 |
| pa9 | 3.0 | 32 | 37 | 839 |
| pa9 | 3.0 | 33 | 26 | 627 |
| pa9 | 3.0 | 34 | 33 | 661 |
| pa9 | 3.0 | 35 | 27 | 469 |
| pa9 | 3.0 | 36 | 29 | 607 |
| pa9 | 3.0 | 37 | 23 | 420 |
| pa9 | 3.0 | 38 | 24 | 447 |
| pa9 | 3.0 | 39 | 27 | 431 |
| pa9 | 3.0 | 40 | 25 | 330 |
| pa9 | 3.0 | 41 | 19 | 394 |
| pa9 | 3.0 | 42 | 14 | 363 |
| pa9 | 3.0 | 43 | 21 | 497 |
| pa9 | 3.0 | 44 | 24 | 506 |
| pa9 | 3.0 | 45 | 12 | 184 |
| pa9 | 3.0 | 46 | 13 | 268 |
| pa9 | 3.0 | 47 | 13 | 251 |
| pa9 | 3.0 | 48 | 10 | 134 |
| pa9 | 3.0 | 49 | 11 | 327 |
| pa9 | 3.0 | 50 | 10 | 164 |
| pa9 | 3.0 | 51 | 8 | 142 |
| pa9 | 3.0 | 52 | 7 | 130 |
| pa9 | 3.0 | 53 | 12 | 403 |
| pa9 | 3.0 | 54 | 18 | 352 |
| pa9 | 3.0 | 55 | 5 | 127 |
| pa9 | 3.0 | 56 | 9 | 150 |
| pa9 | 3.0 | 57 | 7 | 165 |
| pa9 | 3.0 | 58 | 5 | 100 |
| pa9 | 3.0 | 59 | 7 | 106 |
| pa9 | 3.0 | 60 | 5 | 105 |
| pa9 | 3.0 | 61 | 6 | 144 |
| pa9 | 3.0 | 62 | 4 | 90 |
| pa9 | 3.0 | 63 | 1 | 25 |
| pa9 | 3.0 | 64 | 8 | 181 |
| pa9 | 3.0 | 65 | 9 | 127 |
| pa9 | 3.0 | 66 | 6 | 95 |
| pa9 | 3.0 | 67 | 4 | 64 |
| pa9 | 3.0 | 68 | 3 | 43 |
| pa9 | 3.0 | 69 | 6 | 122 |
| pa9 | 3.0 | 70 | 6 | 132 |
| pa9 | 3.0 | 71 | 2 | 27 |
| pa9 | 3.0 | 72 | 3 | 25 |
| pa9 | 3.0 | 73 | 3 | 44 |
| pa9 | 3.0 | 74 | 2 | 38 |
| pa9 | 3.0 | 75 | 1 | 12 |
| pa9 | 3.0 | 77 | 6 | 127 |
| pa9 | 3.0 | 79 | 5 | 88 |
| pa9 | 3.0 | 80 | 2 | 22 |
| pa9 | 3.0 | 81 | 1 | 9 |
| pa9 | 3.0 | 82 | 1 | 37 |
| pa9 | 3.0 | 84 | 2 | 37 |
| pa9 | 3.0 | 86 | 1 | 27 |
| pa9 | 3.0 | 90 | 1 | 15 |
| pa12 | 2.0 | 2 | 1 | 14 |
| pa12 | 2.0 | 3 | 2 | 18 |
| pa12 | 2.0 | 4 | 29 | 233 |
| pa12 | 2.0 | 5 | 38 | 128 |
| pa12 | 2.0 | 6 | 25 | 221 |
| pa12 | 2.0 | 7 | 65 | 817 |
| pa12 | 2.0 | 8 | 106 | 1,127 |
| pa12 | 2.0 | 9 | 99 | 560 |
| pa12 | 2.0 | 10 | 75 | 609 |
| pa12 | 2.0 | 11 | 77 | 431 |
| pa12 | 2.0 | 12 | 67 | 415 |
| pa12 | 2.0 | 13 | 57 | 443 |
| pa12 | 2.0 | 14 | 53 | 370 |
| pa12 | 2.0 | 15 | 56 | 398 |
| pa12 | 2.0 | 16 | 38 | 287 |
| pa12 | 2.0 | 17 | 46 | 524 |
| pa12 | 2.0 | 18 | 42 | 277 |
| pa12 | 2.0 | 19 | 33 | 249 |
| pa12 | 2.0 | 20 | 32 | 256 |
| pa12 | 2.0 | 21 | 31 | 303 |
| pa12 | 2.0 | 22 | 22 | 247 |
| pa12 | 2.0 | 23 | 33 | 190 |
| pa12 | 2.0 | 24 | 28 | 204 |
| pa12 | 2.0 | 25 | 33 | 347 |
| pa12 | 2.0 | 26 | 29 | 150 |
| pa12 | 2.0 | 27 | 28 | 239 |
| pa12 | 2.0 | 28 | 21 | 151 |
| pa12 | 2.0 | 29 | 18 | 135 |
| pa12 | 2.0 | 30 | 12 | 136 |
| pa12 | 2.0 | 31 | 25 | 160 |
| pa12 | 2.0 | 32 | 28 | 211 |
| pa12 | 2.0 | 33 | 17 | 181 |
| pa12 | 2.0 | 34 | 11 | 63 |
| pa12 | 2.0 | 35 | 11 | 74 |
| pa12 | 2.0 | 36 | 9 | 79 |
| pa12 | 2.0 | 37 | 12 | 56 |
| pa12 | 2.0 | 38 | 11 | 109 |
| pa12 | 2.0 | 39 | 13 | 108 |
| pa12 | 2.0 | 40 | 15 | 141 |
| pa12 | 2.0 | 41 | 7 | 52 |
| pa12 | 2.0 | 42 | 9 | 70 |
| pa12 | 2.0 | 43 | 9 | 64 |
| pa12 | 2.0 | 44 | 10 | 55 |
| pa12 | 2.0 | 45 | 9 | 56 |
| pa12 | 2.0 | 46 | 5 | 67 |
| pa12 | 2.0 | 47 | 8 | 55 |
| pa12 | 2.0 | 48 | 5 | 46 |
| pa12 | 2.0 | 49 | 4 | 31 |
| pa12 | 2.0 | 50 | 6 | 49 |
| pa12 | 2.0 | 51 | 6 | 48 |
| pa12 | 2.0 | 52 | 7 | 49 |
| pa12 | 2.0 | 53 | 8 | 37 |
| pa12 | 2.0 | 54 | 8 | 68 |
| pa12 | 2.0 | 55 | 2 | 35 |
| pa12 | 2.0 | 56 | 5 | 31 |
| pa12 | 2.0 | 57 | 4 | 30 |
| pa12 | 2.0 | 58 | 2 | 8 |
| pa12 | 2.0 | 59 | 4 | 4 |
| pa12 | 2.0 | 60 | 3 | 31 |
| pa12 | 2.0 | 61 | 4 | 46 |
| pa12 | 2.0 | 62 | 3 | 23 |
| pa12 | 2.0 | 63 | 2 | 25 |
| pa12 | 2.0 | 64 | 4 | 15 |
| pa12 | 2.0 | 66 | 6 | 37 |
| pa12 | 2.0 | 68 | 3 | 10 |
| pa12 | 2.0 | 69 | 4 | 5 |
| pa12 | 2.0 | 70 | 1 | 5 |
| pa12 | 2.0 | 71 | 3 | 14 |
| pa12 | 2.0 | 72 | 1 | 0 |
| pa12 | 2.0 | 73 | 1 | 4 |
| pa12 | 2.0 | 74 | 2 | 13 |
| pa12 | 2.0 | 75 | 1 | 9 |
| pa12 | 2.0 | 76 | 1 | 5 |
| pa12 | 2.0 | 77 | 2 | 19 |
| pa12 | 2.0 | 79 | 1 | 0 |
| pa12 | 2.0 | 80 | 1 | 0 |
| pa12 | 2.0 | 81 | 1 | 13 |
| pa12 | 2.0 | 85 | 1 | 5 |
| pa13 | 2.0 | 1 | 25 | 274 |
| pa13 | 2.0 | 2 | 21 | 263 |
| pa13 | 2.0 | 3 | 1,872 | 16,054 |
| pa13 | 2.0 | 4 | 2,081 | 16,340 |
| pa13 | 2.0 | 5 | 2,475 | 26,496 |
| pa13 | 2.0 | 6 | 4,224 | 61,619 |
| pa13 | 2.0 | 7 | 3,078 | 4,304 |
| pa13 | 2.0 | 8 | 346 | 2,555 |
| pa13 | 2.0 | 9 | 111 | 1,821 |
| pa13 | 2.0 | 10 | 73 | 1,502 |
| pa13 | 2.0 | 11 | 80 | 1,250 |
| pa13 | 2.0 | 12 | 43 | 870 |
| pa13 | 2.0 | 13 | 50 | 1,000 |
| pa13 | 2.0 | 14 | 79 | 1,005 |
| pa13 | 2.0 | 15 | 47 | 732 |
| pa13 | 2.0 | 16 | 33 | 595 |
| pa13 | 2.0 | 17 | 34 | 512 |
| pa13 | 2.0 | 18 | 34 | 436 |
| pa13 | 2.0 | 19 | 22 | 387 |
| pa13 | 2.0 | 20 | 32 | 660 |
| pa13 | 2.0 | 21 | 31 | 551 |
| pa13 | 2.0 | 22 | 70 | 808 |
| pa13 | 2.0 | 23 | 18 | 362 |
| pa13 | 2.0 | 24 | 40 | 824 |
| pa13 | 2.0 | 25 | 12 | 256 |
| pa13 | 2.0 | 26 | 21 | 421 |
| pa13 | 2.0 | 27 | 56 | 517 |
| pa13 | 2.0 | 28 | 13 | 267 |
| pa13 | 2.0 | 29 | 19 | 367 |
| pa13 | 2.0 | 30 | 44 | 461 |
| pa13 | 2.0 | 31 | 6 | 136 |
| pa13 | 2.0 | 32 | 8 | 160 |
| pa13 | 2.0 | 33 | 11 | 217 |
| pa13 | 2.0 | 34 | 12 | 187 |
| pa13 | 2.0 | 35 | 9 | 203 |
| pa13 | 2.0 | 36 | 12 | 261 |
| pa13 | 2.0 | 37 | 9 | 204 |
| pa13 | 2.0 | 38 | 9 | 201 |
| pa13 | 2.0 | 39 | 5 | 112 |
| pa13 | 2.0 | 40 | 14 | 308 |
| pa13 | 2.0 | 41 | 5 | 110 |
| pa13 | 2.0 | 42 | 9 | 177 |
| pa13 | 2.0 | 43 | 13 | 197 |
| pa13 | 2.0 | 44 | 4 | 90 |
| pa13 | 2.0 | 45 | 12 | 165 |
| pa13 | 2.0 | 46 | 6 | 123 |
| pa13 | 2.0 | 47 | 6 | 123 |
| pa13 | 2.0 | 48 | 5 | 114 |
| pa13 | 2.0 | 49 | 4 | 91 |
| pa13 | 2.0 | 50 | 6 | 115 |
| pa13 | 2.0 | 51 | 7 | 151 |
| pa13 | 2.0 | 52 | 2 | 47 |
| pa13 | 2.0 | 53 | 5 | 117 |
| pa13 | 2.0 | 54 | 2 | 41 |
| pa13 | 2.0 | 55 | 6 | 123 |
| pa13 | 2.0 | 56 | 2 | 42 |
| pa13 | 2.0 | 57 | 4 | 87 |
| pa13 | 2.0 | 58 | 2 | 47 |
| pa13 | 2.0 | 59 | 6 | 140 |
| pa13 | 2.0 | 60 | 4 | 70 |
| pa13 | 2.0 | 61 | 1 | 23 |
| pa13 | 2.0 | 62 | 5 | 105 |
| pa13 | 2.0 | 63 | 1 | 24 |
| pa13 | 2.0 | 64 | 4 | 92 |
| pa13 | 2.0 | 65 | 4 | 88 |
| pa13 | 2.0 | 66 | 1 | 23 |
| pa13 | 2.0 | 67 | 3 | 53 |
| pa13 | 2.0 | 68 | 3 | 65 |
| pa13 | 2.0 | 69 | 1 | 23 |
| pa13 | 2.0 | 72 | 1 | 22 |
| pa13 | 2.0 | 74 | 5 | 113 |
| pa13 | 2.0 | 75 | 2 | 45 |
| pa13 | 2.0 | 76 | 1 | 22 |
| pa13 | 2.0 | 79 | 1 | 23 |
| pa13 | 2.0 | 81 | 1 | 22 |
| pa14 | 2.0 | 0 | 46 | 561 |
| pa14 | 2.0 | 1 | 374 | 2,888 |
| pa14 | 2.0 | 2 | 907 | 6,912 |
| pa14 | 2.0 | 3 | 70,819 | 453,889 |
| pa14 | 2.0 | 4 | 70,324 | 29,679 |
| pa14 | 2.0 | 5 | 5,972 | 4,709 |
| pa14 | 2.0 | 6 | 592 | 3,448 |
| pa14 | 2.0 | 7 | 377 | 3,261 |
| pa14 | 2.0 | 8 | 1,151 | 5,476 |
| pa14 | 2.0 | 9 | 414 | 2,709 |
| pa14 | 2.0 | 10 | 239 | 2,238 |
| pa14 | 2.0 | 11 | 300 | 2,049 |
| pa14 | 2.0 | 12 | 207 | 1,816 |
| pa14 | 2.0 | 13 | 270 | 2,073 |
| pa14 | 2.0 | 14 | 346 | 1,947 |
| pa14 | 2.0 | 15 | 203 | 1,546 |
| pa14 | 2.0 | 16 | 143 | 1,303 |
| pa14 | 2.0 | 17 | 191 | 1,475 |
| pa14 | 2.0 | 18 | 217 | 1,249 |
| pa14 | 2.0 | 19 | 154 | 1,376 |
| pa14 | 2.0 | 20 | 148 | 1,123 |
| pa14 | 2.0 | 21 | 143 | 1,265 |
| pa14 | 2.0 | 22 | 250 | 1,267 |
| pa14 | 2.0 | 23 | 103 | 910 |
| pa14 | 2.0 | 24 | 108 | 795 |
| pa14 | 2.0 | 25 | 94 | 782 |
| pa14 | 2.0 | 26 | 114 | 859 |
| pa14 | 2.0 | 27 | 208 | 864 |
| pa14 | 2.0 | 28 | 101 | 815 |
| pa14 | 2.0 | 29 | 63 | 531 |
| pa14 | 2.0 | 30 | 136 | 653 |
| pa14 | 2.0 | 31 | 62 | 523 |
| pa14 | 2.0 | 32 | 51 | 340 |
| pa14 | 2.0 | 33 | 56 | 458 |
| pa14 | 2.0 | 34 | 40 | 362 |
| pa14 | 2.0 | 35 | 42 | 376 |
| pa14 | 2.0 | 36 | 46 | 350 |
| pa14 | 2.0 | 37 | 61 | 565 |
| pa14 | 2.0 | 38 | 30 | 268 |
| pa14 | 2.0 | 39 | 39 | 311 |
| pa14 | 2.0 | 40 | 35 | 345 |
| pa14 | 2.0 | 41 | 33 | 284 |
| pa14 | 2.0 | 42 | 37 | 246 |
| pa14 | 2.0 | 43 | 47 | 218 |
| pa14 | 2.0 | 44 | 26 | 254 |
| pa14 | 2.0 | 45 | 49 | 276 |
| pa14 | 2.0 | 46 | 13 | 121 |
| pa14 | 2.0 | 47 | 13 | 88 |
| pa14 | 2.0 | 48 | 23 | 222 |
| pa14 | 2.0 | 49 | 15 | 117 |
| pa14 | 2.0 | 50 | 21 | 224 |
| pa14 | 2.0 | 51 | 16 | 98 |
| pa14 | 2.0 | 52 | 15 | 128 |
| pa14 | 2.0 | 53 | 14 | 89 |
| pa14 | 2.0 | 54 | 15 | 86 |
| pa14 | 2.0 | 55 | 12 | 90 |
| pa14 | 2.0 | 56 | 14 | 120 |
| pa14 | 2.0 | 57 | 16 | 107 |
| pa14 | 2.0 | 58 | 6 | 58 |
| pa14 | 2.0 | 59 | 17 | 153 |
| pa14 | 2.0 | 60 | 10 | 92 |
| pa14 | 2.0 | 61 | 9 | 70 |
| pa14 | 2.0 | 62 | 10 | 116 |
| pa14 | 2.0 | 63 | 3 | 17 |
| pa14 | 2.0 | 64 | 7 | 76 |
| pa14 | 2.0 | 65 | 7 | 46 |
| pa14 | 2.0 | 66 | 12 | 40 |
| pa14 | 2.0 | 67 | 5 | 39 |
| pa14 | 2.0 | 68 | 8 | 34 |
| pa14 | 2.0 | 69 | 1 | 11 |
| pa14 | 2.0 | 70 | 3 | 7 |
| pa14 | 2.0 | 71 | 6 | 72 |
| pa14 | 2.0 | 72 | 8 | 37 |
| pa14 | 2.0 | 73 | 12 | 95 |
| pa14 | 2.0 | 74 | 7 | 63 |
| pa14 | 2.0 | 75 | 1 | 3 |
| pa14 | 2.0 | 76 | 3 | 9 |
| pa14 | 2.0 | 78 | 2 | 17 |
| pa14 | 2.0 | 79 | 1 | 13 |
| pa14 | 2.0 | 80 | 4 | 8 |
| pa14 | 2.0 | 81 | 2 | 0 |
| pa14 | 2.0 | 82 | 2 | 8 |
| pa14 | 2.0 | 83 | 1 | 0 |
| pa14 | 2.0 | 85 | 1 | 0 |

Dropped axes with a living apex (apex_end none): 0 axes, 0 phytomers.


## Tree 5. Shipped Norway spruce (preset `norway-spruce`), age 40, seed 1

### Item 1. Held, kept, difference

|  | axes | phytomers |
|---|---:|---:|
| held just before the settle (axes in the grower) | 370,335 | 1,248,873 |
| kept by the shed rule (`shed_past`, probe replica) | 292,083 | 1,115,420 |
| dropped (difference) | 78,252 | 133,453 |
| kept by the real `shed_past` (settle_shed, printed after it) | 292,083 | 1,115,420 |
| settled structure `grow()` returns (after scale, thicken, place) | 292,083 | 1,115,420 |
| `rules.grown` (phytomers grown for the tree, shed ones included; `Cycled.grown`) | - | 2,658,308 |

Living apexes in the grower: 114,279. Retirement table length: 316,171 axes taken in (54,164 laid in the last cycle, not yet taken in). Retired roots whose records were let go earlier and still stand in the tree's fades (`Retirement::standing`, not held as axes): 337,947.

### Item 2. Held split by the axis's fate at the settle

| fate | axes | share | phytomers | share |
|---|---:|---:|---:|---:|
| kept at full scale (applied fade 1) | 290,260 | 78.4% | 1,106,983 | 88.6% |
| kept faded, scale over 0.5 | 768 | 0.2% | 3,637 | 0.3% |
| kept faded, 0.1 to 0.5 | 886 | 0.2% | 4,045 | 0.3% |
| kept faded, 0.01 to 0.1 | 165 | 0.0% | 739 | 0.1% |
| kept faded, 0.01 or under | 4 | 0.0% | 16 | 0.0% |
| dropped, shed rule via its bearer (an ancestor was shed) | 3,271 | 0.9% | 5,908 | 0.5% |
| dropped, shed rule at its own fade (axis is the shed root) | 74,981 | 20.2% | 127,545 | 10.2% |
| dropped for another reason | 0 | 0.0% | 0 | 0.0% |

Dropped for another reason: the shed loop has only the two exits (bearer lost, own fade at nothing); none else exists, so 0 by construction.

### Item 3. Dropped split

By PA:

| PA | axes | share | phytomers | share |
|---|---:|---:|---:|---:|
| pa10 | 369 | 0.5% | 3,795 | 2.8% |
| pa11 | 190 | 0.2% | 722 | 0.5% |
| pa12 | 77,693 | 99.3% | 128,936 | 96.6% |

By apex:

| apex | axes | share | phytomers | share |
|---|---:|---:|---:|---:|
| apex ended 0 cycles ago | 1 | 0.0% | 4 | 0.0% |
| apex ended 1 cycles ago | 22 | 0.0% | 88 | 0.1% |
| apex ended 2 cycles ago | 499 | 0.6% | 1,923 | 1.4% |
| apex ended 3-5 cycles ago | 77,572 | 99.1% | 129,245 | 96.8% |
| apex ended 6-10 cycles ago | 68 | 0.1% | 1,074 | 0.8% |
| apex ended >10 cycles ago | 90 | 0.1% | 1,119 | 0.8% |

By working set:

| working set | axes | share | phytomers | share |
|---|---:|---:|---:|---:|
| closed | 78,249 | 100.0% | 133,413 | 100.0% |
| open | 3 | 0.0% | 40 | 0.0% |

By retired status:

| retired status | axes | share | phytomers | share |
|---|---:|---:|---:|---:|
| a retired root's stand (axis record held) | 36,569 | 46.7% | 0 | 0.0% |
| inside a retired subtree | 1,232 | 1.6% | 0 | 0.0% |
| not retired, fate kept (fading list) | 39,814 | 50.9% | 130,737 | 98.0% |
| not retired, fate open | 637 | 0.8% | 2,716 | 2.0% |

By later years' passes:

| later years' passes | axes | share | phytomers | share |
|---|---:|---:|---:|---:|
| no | 37,801 | 48.3% | 0 | 0.0% |
| yes (fate Kept: a lateral that sheds waits in the fading list and is evaluated once its fade is none, retire.rs faded_out; otherwise read inside an ancestor's evaluation) | 39,814 | 50.9% | 130,737 | 98.0% |
| yes (open: balance and allotment visit it, working.rs) | 3 | 0.0% | 40 | 0.0% |
| yes (retire.rs evaluation of the closed subtree) | 634 | 0.8% | 2,676 | 2.0% |

### Item 4. Footprint by store, kept and dropped

| store | reader MiB | kept axes MiB | dropped, retired MiB | dropped, other MiB | slack / unsplit MiB |
|---|---:|---:|---:|---:|---:|
| axes | 139.9 | 64.6 | 8.4 | 8.9 | 58.0 |
| axes.phytomers | 103.0 | 93.1 | 0.0 | 9.9 | 0.0 |
| axes.units | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 |
| draws | 86.8 | 40.1 | 5.2 | 5.6 | 36.0 |
| draws.units | 37.0 | 32.7 | 0.0 | 4.3 | 0.0 |
| draws.nodes | 11.4 | 10.3 | 0.0 | 1.1 | 0.0 |
| draws.shade | 9.3 | 8.2 | 0.0 | 1.1 | 0.0 |
| pencils | 100.7 | 57.9 | 7.5 | 8.0 | 27.2 |
| pencils.inherited | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 |
| pencils.path | 6.2 | 5.2 | 0.5 | 0.5 | 0.0 |
| successor | 14.5 | 6.7 | 0.9 | 0.9 | 6.0 |
| successor.each | 0.3 | 0.3 | 0.0 | 0.0 | 0.0 |
| working.links | 9.6 | 5.4 | 0.9 | 0.9 | 2.4 |
| working.by_node | 9.6 | 5.4 | 0.9 | 0.9 | 2.4 |
| working.by_node.each | 3.3 | 3.3 | 0.0 | 0.0 | 0.0 |
| retire.fate | 4.8 | 2.7 | 0.4 | 0.5 | 1.2 |

Reader total 600.3 MiB. Attributed per axis (record and per-axis Vec capacity): kept 342.7, dropped retired 25.5, dropped other 43.7 MiB.

Stores not split by axis (global lists or per-leaf; largest first): leaves 16.0, retire.stands 12.0, horizon 4.8, units 4.8, roots 4.8, working.pa 3.2, live 2.0, retire.fading 2.0, working.at 1.6, working.light 1.6, working.cost 1.6, working.sub 1.6 MiB.

Axes in groups: kept 292,083, dropped retired (root/inside) 37,801, dropped other 40,451.

### Item 5. Dropped axes with no living apex: cycles since the apex ended against the PA's shedding delay

PA delays (cycles, `species.states[pa].shedding`): pa0 = inf, pa1 = inf, pa2 = inf, pa3 = inf, pa4 = inf, pa5 = inf, pa6 = inf, pa7 = inf, pa8 = inf, pa9 = 3.0, pa10 = 1.0, pa11 = 1.0, pa12 = 3.0, pa13 = 3.0, pa14 = 3.0.

| PA | cycles since apex end minus delay | axes | phytomers |
|---|---:|---:|---:|
| pa10 (delay 1.0) | 1 cycle past | 12 | 0 |
| pa10 (delay 1.0) | 2 cycles past | 15 | 15 |
| pa10 (delay 1.0) | 3 to 5 past | 200 | 1,833 |
| pa10 (delay 1.0) | 6 to 10 past | 61 | 918 |
| pa10 (delay 1.0) | over 10 past | 81 | 1,029 |
| pa11 (delay 1.0) | 1 cycle past | 10 | 81 |
| pa11 (delay 1.0) | 2 cycles past | 33 | 168 |
| pa11 (delay 1.0) | 3 to 5 past | 147 | 473 |
| pa12 (delay 3.0) | before the delay (ago < delay) | 500 | 1,934 |
| pa12 (delay 3.0) | at the delay (0) | 911 | 2,076 |
| pa12 (delay 3.0) | 1 cycle past | 39,559 | 124,926 |
| pa12 (delay 3.0) | 2 cycles past | 36,723 | 0 |

Same, split by whether the axis is itself the shed root (own) or dropped with its bearer (below):

| class | axes | phytomers |
|---|---:|---:|
| below: 1 cycle past | 898 | 1,685 |
| below: 2 cycles past | 922 | 168 |
| below: 3 to 5 past | 147 | 473 |
| below: at the delay (0) | 804 | 1,648 |
| below: before the delay (ago < delay) | 500 | 1,934 |
| own: 1 cycle past | 38,683 | 123,322 |
| own: 2 cycles past | 35,849 | 15 |
| own: 3 to 5 past | 200 | 1,833 |
| own: 6 to 10 past | 61 | 918 |
| own: at the delay (0) | 107 | 428 |
| own: over 10 past | 81 | 1,029 |

Exact cycles since the apex ended, per PA:

| PA | delay | cycles ago | axes | phytomers |
|---|---:|---:|---:|---:|
| pa10 | 1.0 | 2 | 12 | 0 |
| pa10 | 1.0 | 3 | 15 | 15 |
| pa10 | 1.0 | 4 | 100 | 1,392 |
| pa10 | 1.0 | 5 | 84 | 195 |
| pa10 | 1.0 | 6 | 16 | 246 |
| pa10 | 1.0 | 7 | 11 | 177 |
| pa10 | 1.0 | 8 | 15 | 204 |
| pa10 | 1.0 | 9 | 7 | 114 |
| pa10 | 1.0 | 10 | 19 | 333 |
| pa10 | 1.0 | 11 | 9 | 90 |
| pa10 | 1.0 | 12 | 14 | 204 |
| pa10 | 1.0 | 13 | 13 | 180 |
| pa10 | 1.0 | 14 | 6 | 114 |
| pa10 | 1.0 | 15 | 11 | 165 |
| pa10 | 1.0 | 16 | 8 | 90 |
| pa10 | 1.0 | 17 | 6 | 63 |
| pa10 | 1.0 | 18 | 2 | 0 |
| pa10 | 1.0 | 19 | 5 | 48 |
| pa10 | 1.0 | 20 | 4 | 48 |
| pa10 | 1.0 | 21 | 3 | 30 |
| pa10 | 1.0 | 22 | 3 | 27 |
| pa10 | 1.0 | 23 | 5 | 60 |
| pa10 | 1.0 | 25 | 1 | 0 |
| pa11 | 1.0 | 2 | 10 | 81 |
| pa11 | 1.0 | 3 | 33 | 168 |
| pa11 | 1.0 | 4 | 87 | 473 |
| pa11 | 1.0 | 5 | 60 | 0 |
| pa12 | 3.0 | 0 | 1 | 4 |
| pa12 | 3.0 | 1 | 22 | 88 |
| pa12 | 3.0 | 2 | 477 | 1,842 |
| pa12 | 3.0 | 3 | 911 | 2,076 |
| pa12 | 3.0 | 4 | 39,559 | 124,926 |
| pa12 | 3.0 | 5 | 36,723 | 0 |

Dropped axes with a living apex (apex_end none): 0 axes, 0 phytomers.


## Tree 6. `telperion_space::beech()` (the species the `form` test and BEECH-FORM-MEMORY.md grow), age 80, seed 7, `Light::NEUTRAL`

### Item 1. Held, kept, difference

|  | axes | phytomers |
|---|---:|---:|
| held just before the settle (axes in the grower) | 1,846,068 | 27,138,515 |
| kept by the shed rule (`shed_past`, probe replica) | 1,469,354 | 24,178,429 |
| dropped (difference) | 376,714 | 2,960,086 |
| kept by the real `shed_past` (settle_shed, printed after it) | 1,469,354 | 24,178,429 |
| settled structure `grow()` returns (after scale, thicken, place) | 1,469,354 | 24,178,429 |
| `rules.grown` (phytomers grown for the tree, shed ones included; `Cycled.grown`) | - | 70,431,575 |

Living apexes in the grower: 1,098,002. Retirement table length: 1,659,322 axes taken in (186,746 laid in the last cycle, not yet taken in). Retired roots whose records were let go earlier and still stand in the tree's fades (`Retirement::standing`, not held as axes): 574,621.

### Item 2. Held split by the axis's fate at the settle

| fate | axes | share | phytomers | share |
|---|---:|---:|---:|---:|
| kept at full scale (applied fade 1) | 1,433,619 | 77.7% | 23,325,108 | 85.9% |
| kept faded, scale over 0.5 | 13,836 | 0.7% | 329,828 | 1.2% |
| kept faded, 0.1 to 0.5 | 18,871 | 1.0% | 443,657 | 1.6% |
| kept faded, 0.01 to 0.1 | 2,781 | 0.2% | 73,298 | 0.3% |
| kept faded, 0.01 or under | 247 | 0.0% | 6,538 | 0.0% |
| dropped, shed rule via its bearer (an ancestor was shed) | 66,596 | 3.6% | 751,618 | 2.8% |
| dropped, shed rule at its own fade (axis is the shed root) | 310,118 | 16.8% | 2,208,468 | 8.1% |
| dropped for another reason | 0 | 0.0% | 0 | 0.0% |

Dropped for another reason: the shed loop has only the two exits (bearer lost, own fade at nothing); none else exists, so 0 by construction.

### Item 3. Dropped split

By PA:

| PA | axes | share | phytomers | share |
|---|---:|---:|---:|---:|
| pa9 | 1,654 | 0.4% | 22,094 | 0.7% |
| pa12 | 40,104 | 10.6% | 199,968 | 6.8% |
| pa14 | 334,956 | 88.9% | 2,738,024 | 92.5% |

By apex:

| apex | axes | share | phytomers | share |
|---|---:|---:|---:|---:|
| apex ended 0 cycles ago | 1,197 | 0.3% | 57,417 | 1.9% |
| apex ended 1 cycles ago | 2,826 | 0.8% | 69,645 | 2.4% |
| apex ended 2 cycles ago | 116,693 | 31.0% | 2,497,737 | 84.4% |
| apex ended 3-5 cycles ago | 217,675 | 57.8% | 69,406 | 2.3% |
| apex ended 6-10 cycles ago | 4,595 | 1.2% | 52,809 | 1.8% |
| apex ended >10 cycles ago | 33,728 | 9.0% | 213,072 | 7.2% |

By working set:

| working set | axes | share | phytomers | share |
|---|---:|---:|---:|---:|
| closed | 373,728 | 99.2% | 2,856,505 | 96.5% |
| open | 2,986 | 0.8% | 103,581 | 3.5% |

By retired status:

| retired status | axes | share | phytomers | share |
|---|---:|---:|---:|---:|
| a retired root's stand (axis record held) | 205,811 | 54.6% | 0 | 0.0% |
| inside a retired subtree | 37,414 | 9.9% | 0 | 0.0% |
| not retired, fate kept (fading list) | 111,779 | 29.7% | 2,062,438 | 69.7% |
| not retired, fate open | 21,710 | 5.8% | 897,648 | 30.3% |

By later years' passes:

| later years' passes | axes | share | phytomers | share |
|---|---:|---:|---:|---:|
| no | 243,225 | 64.6% | 0 | 0.0% |
| yes (fate Kept: a lateral that sheds waits in the fading list and is evaluated once its fade is none, retire.rs faded_out; otherwise read inside an ancestor's evaluation) | 111,779 | 29.7% | 2,062,438 | 69.7% |
| yes (open: balance and allotment visit it, working.rs) | 2,986 | 0.8% | 103,581 | 3.5% |
| yes (retire.rs evaluation of the closed subtree) | 18,724 | 5.0% | 794,067 | 26.8% |

### Item 4. Footprint by store, kept and dropped

| store | reader MiB | kept axes MiB | dropped, retired MiB | dropped, other MiB | slack / unsplit MiB |
|---|---:|---:|---:|---:|---:|
| axes | 654.6 | 325.1 | 53.8 | 29.5 | 246.2 |
| axes.phytomers | 2,681.7 | 2,396.5 | 0.0 | 285.2 | 0.0 |
| axes.units | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 |
| draws | 406.3 | 201.8 | 33.4 | 18.3 | 152.8 |
| draws.units | 311.1 | 278.5 | 0.0 | 32.7 | 0.0 |
| draws.nodes | 298.0 | 266.3 | 0.0 | 31.7 | 0.0 |
| draws.shade | 77.8 | 69.6 | 0.0 | 8.2 | 0.0 |
| pencils | 586.9 | 291.5 | 48.2 | 26.5 | 220.7 |
| pencils.inherited | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 |
| pencils.path | 177.1 | 133.1 | 28.1 | 15.8 | 0.0 |
| successor | 67.7 | 33.6 | 5.6 | 3.1 | 25.5 |
| successor.each | 0.4 | 0.4 | 0.0 | 0.0 | 0.0 |
| working.links | 48.3 | 29.4 | 5.6 | 3.1 | 10.3 |
| working.by_node | 48.3 | 29.4 | 5.6 | 3.1 | 10.3 |
| working.by_node.each | 12.1 | 10.7 | 0.9 | 0.5 | 0.0 |
| retire.fate | 24.2 | 14.7 | 2.8 | 1.5 | 5.2 |

Reader total 5,742.7 MiB. Attributed per axis (record and per-axis Vec capacity): kept 4,114.1, dropped retired 189.5, dropped other 462.1 MiB.

Stores not split by axis (global lists or per-leaf; largest first): leaves 128.0, live 32.0, retire.stands 24.0, horizon 22.6, units 22.6, roots 22.6, working.pa 16.1, working.list 16.0, working.at 8.1, working.light 8.1, working.cost 8.1, working.sub 8.1 MiB.

Axes in groups: kept 1,469,354, dropped retired (root/inside) 243,225, dropped other 133,489.

### Item 5. Dropped axes with no living apex: cycles since the apex ended against the PA's shedding delay

PA delays (cycles, `species.states[pa].shedding`): pa0 = inf, pa1 = inf, pa2 = inf, pa3 = inf, pa4 = inf, pa5 = inf, pa6 = inf, pa7 = inf, pa8 = inf, pa9 = 1.0, pa10 = 1.0, pa11 = 1.0, pa12 = 1.0, pa13 = 1.0, pa14 = 1.0.

| PA | cycles since apex end minus delay | axes | phytomers |
|---|---:|---:|---:|
| pa9 (delay 1.0) | 1 cycle past | 109 | 20 |
| pa9 (delay 1.0) | 2 cycles past | 67 | 6 |
| pa9 (delay 1.0) | 3 to 5 past | 81 | 0 |
| pa9 (delay 1.0) | 6 to 10 past | 2 | 0 |
| pa9 (delay 1.0) | over 10 past | 1,395 | 22,068 |
| pa12 (delay 1.0) | 1 cycle past | 1,371 | 1,019 |
| pa12 (delay 1.0) | 2 cycles past | 1,474 | 673 |
| pa12 (delay 1.0) | 3 to 5 past | 2,276 | 4,720 |
| pa12 (delay 1.0) | 6 to 10 past | 5,651 | 45,211 |
| pa12 (delay 1.0) | over 10 past | 29,332 | 148,345 |
| pa14 (delay 1.0) | before the delay (ago < delay) | 1,197 | 57,417 |
| pa14 (delay 1.0) | at the delay (0) | 2,826 | 69,645 |
| pa14 (delay 1.0) | 1 cycle past | 115,213 | 2,496,698 |
| pa14 (delay 1.0) | 2 cycles past | 109,246 | 46,754 |
| pa14 (delay 1.0) | 3 to 5 past | 105,231 | 27,342 |
| pa14 (delay 1.0) | 6 to 10 past | 705 | 22,793 |
| pa14 (delay 1.0) | over 10 past | 538 | 17,375 |

Same, split by whether the axis is itself the shed root (own) or dropped with its bearer (below):

| class | axes | phytomers |
|---|---:|---:|
| below: 1 cycle past | 15,436 | 641,146 |
| below: 2 cycles past | 20,073 | 37,051 |
| below: 3 to 5 past | 25,299 | 2,931 |
| below: 6 to 10 past | 195 | 2,252 |
| below: at the delay (0) | 982 | 26,078 |
| below: before the delay (ago < delay) | 410 | 19,687 |
| below: over 10 past | 4,201 | 22,473 |
| own: 1 cycle past | 101,257 | 1,856,591 |
| own: 2 cycles past | 90,714 | 10,382 |
| own: 3 to 5 past | 82,289 | 29,131 |
| own: 6 to 10 past | 6,163 | 65,752 |
| own: at the delay (0) | 1,844 | 43,567 |
| own: before the delay (ago < delay) | 787 | 37,730 |
| own: over 10 past | 27,064 | 165,315 |

Exact cycles since the apex ended, per PA:

| PA | delay | cycles ago | axes | phytomers |
|---|---:|---:|---:|---:|
| pa9 | 1.0 | 2 | 109 | 20 |
| pa9 | 1.0 | 3 | 67 | 6 |
| pa9 | 1.0 | 4 | 80 | 0 |
| pa9 | 1.0 | 5 | 1 | 0 |
| pa9 | 1.0 | 7 | 1 | 0 |
| pa9 | 1.0 | 8 | 1 | 0 |
| pa9 | 1.0 | 12 | 4 | 7 |
| pa9 | 1.0 | 14 | 1 | 14 |
| pa9 | 1.0 | 15 | 7 | 175 |
| pa9 | 1.0 | 16 | 15 | 188 |
| pa9 | 1.0 | 17 | 67 | 2,263 |
| pa9 | 1.0 | 18 | 183 | 5,306 |
| pa9 | 1.0 | 19 | 339 | 7,891 |
| pa9 | 1.0 | 20 | 282 | 454 |
| pa9 | 1.0 | 21 | 196 | 344 |
| pa9 | 1.0 | 22 | 28 | 500 |
| pa9 | 1.0 | 23 | 36 | 303 |
| pa9 | 1.0 | 24 | 32 | 697 |
| pa9 | 1.0 | 25 | 31 | 917 |
| pa9 | 1.0 | 26 | 26 | 460 |
| pa9 | 1.0 | 27 | 30 | 424 |
| pa9 | 1.0 | 28 | 28 | 619 |
| pa9 | 1.0 | 29 | 19 | 174 |
| pa9 | 1.0 | 30 | 12 | 201 |
| pa9 | 1.0 | 31 | 19 | 251 |
| pa9 | 1.0 | 32 | 9 | 221 |
| pa9 | 1.0 | 33 | 3 | 54 |
| pa9 | 1.0 | 34 | 3 | 130 |
| pa9 | 1.0 | 35 | 5 | 215 |
| pa9 | 1.0 | 36 | 5 | 69 |
| pa9 | 1.0 | 37 | 3 | 0 |
| pa9 | 1.0 | 38 | 1 | 0 |
| pa9 | 1.0 | 39 | 1 | 0 |
| pa9 | 1.0 | 40 | 2 | 0 |
| pa9 | 1.0 | 41 | 1 | 0 |
| pa9 | 1.0 | 42 | 1 | 0 |
| pa9 | 1.0 | 43 | 1 | 0 |
| pa9 | 1.0 | 44 | 1 | 64 |
| pa9 | 1.0 | 45 | 1 | 64 |
| pa9 | 1.0 | 46 | 1 | 0 |
| pa9 | 1.0 | 47 | 1 | 0 |
| pa9 | 1.0 | 50 | 1 | 63 |
| pa12 | 1.0 | 2 | 1,371 | 1,019 |
| pa12 | 1.0 | 3 | 1,474 | 673 |
| pa12 | 1.0 | 4 | 1,515 | 1,251 |
| pa12 | 1.0 | 5 | 333 | 1,345 |
| pa12 | 1.0 | 6 | 428 | 2,124 |
| pa12 | 1.0 | 7 | 544 | 2,709 |
| pa12 | 1.0 | 8 | 630 | 3,555 |
| pa12 | 1.0 | 9 | 738 | 4,276 |
| pa12 | 1.0 | 10 | 1,394 | 12,945 |
| pa12 | 1.0 | 11 | 2,345 | 21,726 |
| pa12 | 1.0 | 12 | 4,143 | 35,515 |
| pa12 | 1.0 | 13 | 6,094 | 47,766 |
| pa12 | 1.0 | 14 | 8,623 | 63,606 |
| pa12 | 1.0 | 15 | 6,436 | 169 |
| pa12 | 1.0 | 16 | 3,751 | 90 |
| pa12 | 1.0 | 17 | 41 | 144 |
| pa12 | 1.0 | 18 | 43 | 57 |
| pa12 | 1.0 | 19 | 40 | 119 |
| pa12 | 1.0 | 20 | 27 | 178 |
| pa12 | 1.0 | 21 | 24 | 89 |
| pa12 | 1.0 | 22 | 19 | 96 |
| pa12 | 1.0 | 23 | 19 | 148 |
| pa12 | 1.0 | 24 | 9 | 51 |
| pa12 | 1.0 | 25 | 13 | 70 |
| pa12 | 1.0 | 26 | 9 | 25 |
| pa12 | 1.0 | 27 | 11 | 66 |
| pa12 | 1.0 | 28 | 4 | 10 |
| pa12 | 1.0 | 29 | 4 | 17 |
| pa12 | 1.0 | 30 | 5 | 52 |
| pa12 | 1.0 | 31 | 5 | 20 |
| pa12 | 1.0 | 32 | 3 | 6 |
| pa12 | 1.0 | 33 | 1 | 0 |
| pa12 | 1.0 | 34 | 1 | 0 |
| pa12 | 1.0 | 35 | 1 | 0 |
| pa12 | 1.0 | 39 | 1 | 16 |
| pa12 | 1.0 | 40 | 2 | 18 |
| pa12 | 1.0 | 42 | 1 | 0 |
| pa12 | 1.0 | 45 | 2 | 17 |
| pa14 | 1.0 | 0 | 1,197 | 57,417 |
| pa14 | 1.0 | 1 | 2,826 | 69,645 |
| pa14 | 1.0 | 2 | 115,213 | 2,496,698 |
| pa14 | 1.0 | 3 | 109,246 | 46,754 |
| pa14 | 1.0 | 4 | 103,866 | 10,988 |
| pa14 | 1.0 | 5 | 1,093 | 8,389 |
| pa14 | 1.0 | 6 | 272 | 7,965 |
| pa14 | 1.0 | 7 | 188 | 5,971 |
| pa14 | 1.0 | 8 | 158 | 5,238 |
| pa14 | 1.0 | 9 | 128 | 4,398 |
| pa14 | 1.0 | 10 | 113 | 3,628 |
| pa14 | 1.0 | 11 | 118 | 3,558 |
| pa14 | 1.0 | 12 | 82 | 2,963 |
| pa14 | 1.0 | 13 | 75 | 2,470 |
| pa14 | 1.0 | 14 | 70 | 1,985 |
| pa14 | 1.0 | 15 | 49 | 1,605 |
| pa14 | 1.0 | 16 | 38 | 1,292 |
| pa14 | 1.0 | 17 | 39 | 1,342 |
| pa14 | 1.0 | 18 | 31 | 885 |
| pa14 | 1.0 | 19 | 41 | 1,148 |
| pa14 | 1.0 | 20 | 15 | 579 |
| pa14 | 1.0 | 21 | 21 | 732 |
| pa14 | 1.0 | 22 | 18 | 659 |
| pa14 | 1.0 | 23 | 19 | 468 |
| pa14 | 1.0 | 24 | 9 | 393 |
| pa14 | 1.0 | 25 | 7 | 185 |
| pa14 | 1.0 | 26 | 6 | 135 |
| pa14 | 1.0 | 27 | 2 | 44 |
| pa14 | 1.0 | 28 | 3 | 95 |
| pa14 | 1.0 | 29 | 1 | 35 |
| pa14 | 1.0 | 30 | 1 | 0 |
| pa14 | 1.0 | 31 | 4 | 163 |
| pa14 | 1.0 | 32 | 2 | 56 |
| pa14 | 1.0 | 33 | 4 | 141 |
| pa14 | 1.0 | 44 | 1 | 0 |

Dropped axes with a living apex (apex_end none): 0 axes, 0 phytomers.


## Tree 7. `telperion_space::beech()`, age 80, seed 1

### Item 1. Held, kept, difference

|  | axes | phytomers |
|---|---:|---:|
| held just before the settle (axes in the grower) | 1,234,880 | 16,300,325 |
| kept by the shed rule (`shed_past`, probe replica) | 873,989 | 14,514,750 |
| dropped (difference) | 360,891 | 1,785,575 |
| kept by the real `shed_past` (settle_shed, printed after it) | 873,989 | 14,514,750 |
| settled structure `grow()` returns (after scale, thicken, place) | 873,989 | 14,514,750 |
| `rules.grown` (phytomers grown for the tree, shed ones included; `Cycled.grown`) | - | 41,950,721 |

Living apexes in the grower: 651,168. Retirement table length: 1,125,523 axes taken in (109,357 laid in the last cycle, not yet taken in). Retired roots whose records were let go earlier and still stand in the tree's fades (`Retirement::standing`, not held as axes): 244,800.

### Item 2. Held split by the axis's fate at the settle

| fate | axes | share | phytomers | share |
|---|---:|---:|---:|---:|
| kept at full scale (applied fade 1) | 851,794 | 69.0% | 13,985,054 | 85.8% |
| kept faded, scale over 0.5 | 10,007 | 0.8% | 235,195 | 1.4% |
| kept faded, 0.1 to 0.5 | 10,312 | 0.8% | 244,820 | 1.5% |
| kept faded, 0.01 to 0.1 | 1,713 | 0.1% | 45,642 | 0.3% |
| kept faded, 0.01 or under | 163 | 0.0% | 4,039 | 0.0% |
| dropped, shed rule via its bearer (an ancestor was shed) | 80,057 | 6.5% | 446,734 | 2.7% |
| dropped, shed rule at its own fade (axis is the shed root) | 280,834 | 22.7% | 1,338,841 | 8.2% |
| dropped for another reason | 0 | 0.0% | 0 | 0.0% |

Dropped for another reason: the shed loop has only the two exits (bearer lost, own fade at nothing); none else exists, so 0 by construction.

### Item 3. Dropped split

By PA:

| PA | axes | share | phytomers | share |
|---|---:|---:|---:|---:|
| pa9 | 1,527 | 0.4% | 14,130 | 0.8% |
| pa12 | 38,282 | 10.6% | 117,580 | 6.6% |
| pa14 | 321,082 | 89.0% | 1,653,865 | 92.6% |

By apex:

| apex | axes | share | phytomers | share |
|---|---:|---:|---:|---:|
| apex ended 0 cycles ago | 733 | 0.2% | 35,258 | 2.0% |
| apex ended 1 cycles ago | 1,646 | 0.5% | 40,072 | 2.2% |
| apex ended 2 cycles ago | 69,998 | 19.4% | 1,511,519 | 84.7% |
| apex ended 3-5 cycles ago | 193,672 | 53.7% | 41,680 | 2.3% |
| apex ended 6-10 cycles ago | 61,983 | 17.2% | 31,440 | 1.8% |
| apex ended >10 cycles ago | 32,859 | 9.1% | 125,606 | 7.0% |

By working set:

| working set | axes | share | phytomers | share |
|---|---:|---:|---:|---:|
| closed | 359,118 | 99.5% | 1,724,302 | 96.6% |
| open | 1,773 | 0.5% | 61,273 | 3.4% |

By retired status:

| retired status | axes | share | phytomers | share |
|---|---:|---:|---:|---:|
| a retired root's stand (axis record held) | 224,839 | 62.3% | 0 | 0.0% |
| inside a retired subtree | 56,132 | 15.6% | 0 | 0.0% |
| not retired, fate kept (fading list) | 67,072 | 18.6% | 1,253,715 | 70.2% |
| not retired, fate open | 12,848 | 3.6% | 531,860 | 29.8% |

By later years' passes:

| later years' passes | axes | share | phytomers | share |
|---|---:|---:|---:|---:|
| no | 280,971 | 77.9% | 0 | 0.0% |
| yes (fate Kept: a lateral that sheds waits in the fading list and is evaluated once its fade is none, retire.rs faded_out; otherwise read inside an ancestor's evaluation) | 67,072 | 18.6% | 1,253,715 | 70.2% |
| yes (open: balance and allotment visit it, working.rs) | 1,773 | 0.5% | 61,273 | 3.4% |
| yes (retire.rs evaluation of the closed subtree) | 11,075 | 3.1% | 470,587 | 26.4% |

### Item 4. Footprint by store, kept and dropped

| store | reader MiB | kept axes MiB | dropped, retired MiB | dropped, other MiB | slack / unsplit MiB |
|---|---:|---:|---:|---:|---:|
| axes | 291.6 | 193.4 | 62.2 | 17.7 | 18.3 |
| axes.phytomers | 1,611.8 | 1,439.9 | 0.0 | 171.9 | 0.0 |
| axes.units | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 |
| draws | 181.0 | 120.0 | 38.6 | 11.0 | 11.4 |
| draws.units | 186.5 | 166.9 | 0.0 | 19.6 | 0.0 |
| draws.nodes | 179.1 | 160.0 | 0.0 | 19.1 | 0.0 |
| draws.shade | 46.6 | 41.7 | 0.0 | 4.9 | 0.0 |
| pencils | 324.8 | 173.4 | 55.7 | 15.9 | 79.8 |
| pencils.inherited | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 |
| pencils.path | 121.9 | 80.0 | 32.3 | 9.5 | 0.0 |
| successor | 30.2 | 20.0 | 6.4 | 1.8 | 1.9 |
| successor.each | 0.3 | 0.2 | 0.0 | 0.0 | 0.0 |
| working.links | 26.5 | 17.5 | 6.4 | 1.8 | 0.7 |
| working.by_node | 26.5 | 17.5 | 6.4 | 1.8 | 0.7 |
| working.by_node.each | 7.8 | 6.5 | 1.0 | 0.3 | 0.0 |
| retire.fate | 13.2 | 8.8 | 3.2 | 0.9 | 0.4 |

Reader total 3,285.7 MiB. Attributed per axis (record and per-axis Vec capacity): kept 2,465.7, dropped retired 218.8, dropped other 278.1 MiB.

Stores not split by axis (global lists or per-leaf; largest first): leaves 128.0, live 16.0, retire.stands 12.0, horizon 10.1, units 10.1, roots 10.1, working.pa 8.8, working.list 8.0, working.at 4.4, working.light 4.4, working.cost 4.4, working.sub 4.4 MiB.

Axes in groups: kept 873,989, dropped retired (root/inside) 280,971, dropped other 79,920.

### Item 5. Dropped axes with no living apex: cycles since the apex ended against the PA's shedding delay

PA delays (cycles, `species.states[pa].shedding`): pa0 = inf, pa1 = inf, pa2 = inf, pa3 = inf, pa4 = inf, pa5 = inf, pa6 = inf, pa7 = inf, pa8 = inf, pa9 = 1.0, pa10 = 1.0, pa11 = 1.0, pa12 = 1.0, pa13 = 1.0, pa14 = 1.0.

| PA | cycles since apex end minus delay | axes | phytomers |
|---|---:|---:|---:|
| pa9 (delay 1.0) | 1 cycle past | 43 | 7 |
| pa9 (delay 1.0) | 2 cycles past | 51 | 0 |
| pa9 (delay 1.0) | 3 to 5 past | 117 | 0 |
| pa9 (delay 1.0) | 6 to 10 past | 3 | 0 |
| pa9 (delay 1.0) | over 10 past | 1,313 | 14,123 |
| pa12 (delay 1.0) | 1 cycle past | 783 | 546 |
| pa12 (delay 1.0) | 2 cycles past | 861 | 479 |
| pa12 (delay 1.0) | 3 to 5 past | 2,868 | 2,630 |
| pa12 (delay 1.0) | 6 to 10 past | 4,299 | 26,203 |
| pa12 (delay 1.0) | over 10 past | 29,471 | 87,722 |
| pa14 (delay 1.0) | before the delay (ago < delay) | 733 | 35,258 |
| pa14 (delay 1.0) | at the delay (0) | 1,646 | 40,072 |
| pa14 (delay 1.0) | 1 cycle past | 69,172 | 1,510,966 |
| pa14 (delay 1.0) | 2 cycles past | 66,578 | 28,404 |
| pa14 (delay 1.0) | 3 to 5 past | 181,545 | 15,370 |
| pa14 (delay 1.0) | 6 to 10 past | 999 | 13,533 |
| pa14 (delay 1.0) | over 10 past | 409 | 10,262 |

Same, split by whether the axis is itself the shed root (own) or dropped with its bearer (below):

| class | axes | phytomers |
|---|---:|---:|
| below: 1 cycle past | 9,153 | 379,764 |
| below: 2 cycles past | 12,327 | 22,042 |
| below: 3 to 5 past | 51,487 | 1,520 |
| below: 6 to 10 past | 705 | 1,545 |
| below: at the delay (0) | 562 | 14,867 |
| below: before the delay (ago < delay) | 261 | 12,527 |
| below: over 10 past | 5,562 | 14,469 |
| own: 1 cycle past | 60,845 | 1,131,755 |
| own: 2 cycles past | 55,163 | 6,841 |
| own: 3 to 5 past | 133,043 | 16,480 |
| own: 6 to 10 past | 4,596 | 38,191 |
| own: at the delay (0) | 1,084 | 25,205 |
| own: before the delay (ago < delay) | 472 | 22,731 |
| own: over 10 past | 25,631 | 97,638 |

Exact cycles since the apex ended, per PA:

| PA | delay | cycles ago | axes | phytomers |
|---|---:|---:|---:|---:|
| pa9 | 1.0 | 2 | 43 | 7 |
| pa9 | 1.0 | 3 | 51 | 0 |
| pa9 | 1.0 | 4 | 36 | 0 |
| pa9 | 1.0 | 5 | 45 | 0 |
| pa9 | 1.0 | 6 | 36 | 0 |
| pa9 | 1.0 | 7 | 1 | 0 |
| pa9 | 1.0 | 9 | 1 | 0 |
| pa9 | 1.0 | 10 | 1 | 0 |
| pa9 | 1.0 | 14 | 4 | 20 |
| pa9 | 1.0 | 15 | 2 | 29 |
| pa9 | 1.0 | 16 | 8 | 260 |
| pa9 | 1.0 | 17 | 41 | 1,183 |
| pa9 | 1.0 | 18 | 100 | 2,851 |
| pa9 | 1.0 | 19 | 209 | 5,222 |
| pa9 | 1.0 | 20 | 203 | 383 |
| pa9 | 1.0 | 21 | 198 | 556 |
| pa9 | 1.0 | 22 | 174 | 285 |
| pa9 | 1.0 | 23 | 112 | 375 |
| pa9 | 1.0 | 24 | 28 | 238 |
| pa9 | 1.0 | 25 | 31 | 499 |
| pa9 | 1.0 | 26 | 19 | 69 |
| pa9 | 1.0 | 27 | 19 | 298 |
| pa9 | 1.0 | 28 | 31 | 483 |
| pa9 | 1.0 | 29 | 26 | 291 |
| pa9 | 1.0 | 30 | 27 | 132 |
| pa9 | 1.0 | 31 | 15 | 144 |
| pa9 | 1.0 | 32 | 15 | 79 |
| pa9 | 1.0 | 33 | 10 | 263 |
| pa9 | 1.0 | 34 | 6 | 122 |
| pa9 | 1.0 | 35 | 7 | 71 |
| pa9 | 1.0 | 36 | 1 | 0 |
| pa9 | 1.0 | 37 | 9 | 130 |
| pa9 | 1.0 | 38 | 6 | 0 |
| pa9 | 1.0 | 39 | 5 | 65 |
| pa9 | 1.0 | 42 | 2 | 0 |
| pa9 | 1.0 | 43 | 2 | 12 |
| pa9 | 1.0 | 45 | 2 | 0 |
| pa9 | 1.0 | 49 | 1 | 63 |
| pa12 | 1.0 | 2 | 783 | 546 |
| pa12 | 1.0 | 3 | 861 | 479 |
| pa12 | 1.0 | 4 | 933 | 545 |
| pa12 | 1.0 | 5 | 920 | 974 |
| pa12 | 1.0 | 6 | 1,015 | 1,111 |
| pa12 | 1.0 | 7 | 403 | 1,454 |
| pa12 | 1.0 | 8 | 583 | 2,449 |
| pa12 | 1.0 | 9 | 673 | 2,710 |
| pa12 | 1.0 | 10 | 1,040 | 7,809 |
| pa12 | 1.0 | 11 | 1,600 | 11,781 |
| pa12 | 1.0 | 12 | 2,754 | 20,683 |
| pa12 | 1.0 | 13 | 4,329 | 29,545 |
| pa12 | 1.0 | 14 | 6,139 | 36,496 |
| pa12 | 1.0 | 15 | 5,618 | 126 |
| pa12 | 1.0 | 16 | 4,769 | 168 |
| pa12 | 1.0 | 17 | 3,579 | 67 |
| pa12 | 1.0 | 18 | 2,039 | 59 |
| pa12 | 1.0 | 19 | 46 | 48 |
| pa12 | 1.0 | 20 | 34 | 71 |
| pa12 | 1.0 | 21 | 24 | 26 |
| pa12 | 1.0 | 22 | 20 | 37 |
| pa12 | 1.0 | 23 | 24 | 89 |
| pa12 | 1.0 | 24 | 11 | 54 |
| pa12 | 1.0 | 25 | 23 | 38 |
| pa12 | 1.0 | 26 | 6 | 0 |
| pa12 | 1.0 | 27 | 9 | 36 |
| pa12 | 1.0 | 28 | 8 | 32 |
| pa12 | 1.0 | 29 | 11 | 45 |
| pa12 | 1.0 | 30 | 5 | 18 |
| pa12 | 1.0 | 31 | 3 | 0 |
| pa12 | 1.0 | 32 | 5 | 51 |
| pa12 | 1.0 | 33 | 5 | 0 |
| pa12 | 1.0 | 34 | 5 | 18 |
| pa12 | 1.0 | 36 | 1 | 0 |
| pa12 | 1.0 | 37 | 1 | 0 |
| pa12 | 1.0 | 38 | 1 | 0 |
| pa12 | 1.0 | 40 | 1 | 0 |
| pa12 | 1.0 | 44 | 1 | 15 |
| pa14 | 1.0 | 0 | 733 | 35,258 |
| pa14 | 1.0 | 1 | 1,646 | 40,072 |
| pa14 | 1.0 | 2 | 69,172 | 1,510,966 |
| pa14 | 1.0 | 3 | 66,578 | 28,404 |
| pa14 | 1.0 | 4 | 63,566 | 6,765 |
| pa14 | 1.0 | 5 | 60,682 | 4,513 |
| pa14 | 1.0 | 6 | 57,297 | 4,092 |
| pa14 | 1.0 | 7 | 604 | 3,875 |
| pa14 | 1.0 | 8 | 134 | 2,876 |
| pa14 | 1.0 | 9 | 97 | 2,382 |
| pa14 | 1.0 | 10 | 98 | 2,682 |
| pa14 | 1.0 | 11 | 66 | 1,718 |
| pa14 | 1.0 | 12 | 65 | 1,979 |
| pa14 | 1.0 | 13 | 64 | 1,416 |
| pa14 | 1.0 | 14 | 48 | 1,128 |
| pa14 | 1.0 | 15 | 40 | 1,211 |
| pa14 | 1.0 | 16 | 37 | 650 |
| pa14 | 1.0 | 17 | 32 | 683 |
| pa14 | 1.0 | 18 | 30 | 916 |
| pa14 | 1.0 | 19 | 16 | 358 |
| pa14 | 1.0 | 20 | 12 | 389 |
| pa14 | 1.0 | 21 | 10 | 206 |
| pa14 | 1.0 | 22 | 13 | 186 |
| pa14 | 1.0 | 23 | 12 | 305 |
| pa14 | 1.0 | 24 | 11 | 266 |
| pa14 | 1.0 | 25 | 3 | 0 |
| pa14 | 1.0 | 26 | 1 | 47 |
| pa14 | 1.0 | 27 | 2 | 50 |
| pa14 | 1.0 | 28 | 7 | 255 |
| pa14 | 1.0 | 29 | 1 | 34 |
| pa14 | 1.0 | 30 | 1 | 54 |
| pa14 | 1.0 | 31 | 1 | 49 |
| pa14 | 1.0 | 32 | 1 | 51 |
| pa14 | 1.0 | 36 | 1 | 0 |
| pa14 | 1.0 | 38 | 1 | 29 |

Dropped axes with a living apex (apex_end none): 0 axes, 0 phytomers.


## Tree 8. `telperion_space::beech()`, age 40, seed 7

### Item 1. Held, kept, difference

|  | axes | phytomers |
|---|---:|---:|
| held just before the settle (axes in the grower) | 138,188 | 1,287,768 |
| kept by the shed rule (`shed_past`, probe replica) | 99,309 | 1,203,296 |
| dropped (difference) | 38,879 | 84,472 |
| kept by the real `shed_past` (settle_shed, printed after it) | 99,309 | 1,203,296 |
| settled structure `grow()` returns (after scale, thicken, place) | 99,309 | 1,203,296 |
| `rules.grown` (phytomers grown for the tree, shed ones included; `Cycled.grown`) | - | 1,728,689 |

Living apexes in the grower: 80,996. Retirement table length: 119,512 axes taken in (18,676 laid in the last cycle, not yet taken in). Retired roots whose records were let go earlier and still stand in the tree's fades (`Retirement::standing`, not held as axes): 0.

### Item 2. Held split by the axis's fate at the settle

| fate | axes | share | phytomers | share |
|---|---:|---:|---:|---:|
| kept at full scale (applied fade 1) | 97,827 | 70.8% | 1,175,839 | 91.3% |
| kept faded, scale over 0.5 | 382 | 0.3% | 8,055 | 0.6% |
| kept faded, 0.1 to 0.5 | 858 | 0.6% | 15,806 | 1.2% |
| kept faded, 0.01 to 0.1 | 234 | 0.2% | 3,416 | 0.3% |
| kept faded, 0.01 or under | 8 | 0.0% | 180 | 0.0% |
| dropped, shed rule via its bearer (an ancestor was shed) | 10,450 | 7.6% | 13,728 | 1.1% |
| dropped, shed rule at its own fade (axis is the shed root) | 28,429 | 20.6% | 70,744 | 5.5% |
| dropped for another reason | 0 | 0.0% | 0 | 0.0% |

Dropped for another reason: the shed loop has only the two exits (bearer lost, own fade at nothing); none else exists, so 0 by construction.

### Item 3. Dropped split

By PA:

| PA | axes | share | phytomers | share |
|---|---:|---:|---:|---:|
| pa9 | 135 | 0.3% | 277 | 0.3% |
| pa12 | 2,682 | 6.9% | 3,797 | 4.5% |
| pa14 | 36,062 | 92.8% | 80,398 | 95.2% |

By apex:

| apex | axes | share | phytomers | share |
|---|---:|---:|---:|---:|
| apex ended 0 cycles ago | 25 | 0.1% | 1,223 | 1.4% |
| apex ended 1 cycles ago | 76 | 0.2% | 1,646 | 1.9% |
| apex ended 2 cycles ago | 5,699 | 14.7% | 76,660 | 90.8% |
| apex ended 3-5 cycles ago | 12,658 | 32.6% | 959 | 1.1% |
| apex ended 6-10 cycles ago | 10,958 | 28.2% | 972 | 1.2% |
| apex ended >10 cycles ago | 9,463 | 24.3% | 3,012 | 3.6% |

By working set:

| working set | axes | share | phytomers | share |
|---|---:|---:|---:|---:|
| closed | 38,794 | 99.8% | 81,835 | 96.9% |
| open | 85 | 0.2% | 2,637 | 3.1% |

By retired status:

| retired status | axes | share | phytomers | share |
|---|---:|---:|---:|---:|
| a retired root's stand (axis record held) | 24,086 | 62.0% | 0 | 0.0% |
| inside a retired subtree | 8,777 | 22.6% | 0 | 0.0% |
| not retired, fate kept (fading list) | 5,591 | 14.4% | 69,020 | 81.7% |
| not retired, fate open | 425 | 1.1% | 15,452 | 18.3% |

By later years' passes:

| later years' passes | axes | share | phytomers | share |
|---|---:|---:|---:|---:|
| no | 32,863 | 84.5% | 0 | 0.0% |
| yes (fate Kept: a lateral that sheds waits in the fading list and is evaluated once its fade is none, retire.rs faded_out; otherwise read inside an ancestor's evaluation) | 5,591 | 14.4% | 69,020 | 81.7% |
| yes (open: balance and allotment visit it, working.rs) | 85 | 0.2% | 2,637 | 3.1% |
| yes (retire.rs evaluation of the closed subtree) | 340 | 0.9% | 12,815 | 15.2% |

### Item 4. Footprint by store, kept and dropped

| store | reader MiB | kept axes MiB | dropped, retired MiB | dropped, other MiB | slack / unsplit MiB |
|---|---:|---:|---:|---:|---:|
| axes | 58.0 | 22.0 | 7.3 | 1.3 | 27.4 |
| axes.phytomers | 125.9 | 117.7 | 0.0 | 8.1 | 0.0 |
| axes.units | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 |
| draws | 36.0 | 13.6 | 4.5 | 0.8 | 17.0 |
| draws.units | 15.5 | 14.5 | 0.0 | 1.0 | 0.0 |
| draws.nodes | 14.0 | 13.1 | 0.0 | 0.9 | 0.0 |
| draws.shade | 3.9 | 3.6 | 0.0 | 0.2 | 0.0 |
| pencils | 30.1 | 19.7 | 6.5 | 1.2 | 2.7 |
| pencils.inherited | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 |
| pencils.path | 9.3 | 6.5 | 2.3 | 0.5 | 0.0 |
| successor | 6.0 | 2.3 | 0.8 | 0.1 | 2.8 |
| successor.each | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 |
| working.links | 3.0 | 1.8 | 0.8 | 0.1 | 0.3 |
| working.by_node | 3.0 | 1.8 | 0.8 | 0.1 | 0.3 |
| working.by_node.each | 0.6 | 0.6 | 0.0 | 0.0 | 0.0 |
| retire.fate | 1.7 | 0.9 | 0.4 | 0.1 | 0.4 |

Reader total 328.9 MiB. Attributed per axis (record and per-axis Vec capacity): kept 220.5, dropped retired 24.1, dropped other 14.7 MiB.

Stores not split by axis (global lists or per-leaf; largest first): leaves 8.0, horizon 2.0, units 2.0, roots 2.0, live 2.0, working.pa 1.0, working.list 1.0, retire.woken 0.6, working.at 0.5, working.light 0.5, working.cost 0.5, working.sub 0.5 MiB.

Axes in groups: kept 99,309, dropped retired (root/inside) 32,863, dropped other 6,016.

### Item 5. Dropped axes with no living apex: cycles since the apex ended against the PA's shedding delay

PA delays (cycles, `species.states[pa].shedding`): pa0 = inf, pa1 = inf, pa2 = inf, pa3 = inf, pa4 = inf, pa5 = inf, pa6 = inf, pa7 = inf, pa8 = inf, pa9 = 1.0, pa10 = 1.0, pa11 = 1.0, pa12 = 1.0, pa13 = 1.0, pa14 = 1.0.

| PA | cycles since apex end minus delay | axes | phytomers |
|---|---:|---:|---:|
| pa9 (delay 1.0) | 1 cycle past | 11 | 0 |
| pa9 (delay 1.0) | 2 cycles past | 16 | 0 |
| pa9 (delay 1.0) | 3 to 5 past | 22 | 0 |
| pa9 (delay 1.0) | 6 to 10 past | 18 | 0 |
| pa9 (delay 1.0) | over 10 past | 68 | 277 |
| pa12 (delay 1.0) | 1 cycle past | 129 | 86 |
| pa12 (delay 1.0) | 2 cycles past | 127 | 37 |
| pa12 (delay 1.0) | 3 to 5 past | 360 | 226 |
| pa12 (delay 1.0) | 6 to 10 past | 527 | 1,187 |
| pa12 (delay 1.0) | over 10 past | 1,539 | 2,261 |
| pa14 (delay 1.0) | before the delay (ago < delay) | 25 | 1,223 |
| pa14 (delay 1.0) | at the delay (0) | 76 | 1,646 |
| pa14 (delay 1.0) | 1 cycle past | 5,559 | 76,574 |
| pa14 (delay 1.0) | 2 cycles past | 4,854 | 651 |
| pa14 (delay 1.0) | 3 to 5 past | 10,245 | 165 |
| pa14 (delay 1.0) | 6 to 10 past | 8,741 | 139 |
| pa14 (delay 1.0) | over 10 past | 6,562 | 0 |

Same, split by whether the axis is itself the shed root (own) or dropped with its bearer (below):

| class | axes | phytomers |
|---|---:|---:|
| below: 1 cycle past | 324 | 11,935 |
| below: 2 cycles past | 392 | 629 |
| below: 3 to 5 past | 1,394 | 77 |
| below: 6 to 10 past | 2,451 | 18 |
| below: at the delay (0) | 21 | 407 |
| below: before the delay (ago < delay) | 9 | 439 |
| below: over 10 past | 5,859 | 223 |
| own: 1 cycle past | 5,375 | 64,725 |
| own: 2 cycles past | 4,605 | 59 |
| own: 3 to 5 past | 9,233 | 314 |
| own: 6 to 10 past | 6,835 | 1,308 |
| own: at the delay (0) | 55 | 1,239 |
| own: before the delay (ago < delay) | 16 | 784 |
| own: over 10 past | 2,310 | 2,315 |

Exact cycles since the apex ended, per PA:

| PA | delay | cycles ago | axes | phytomers |
|---|---:|---:|---:|---:|
| pa9 | 1.0 | 2 | 11 | 0 |
| pa9 | 1.0 | 3 | 16 | 0 |
| pa9 | 1.0 | 4 | 11 | 0 |
| pa9 | 1.0 | 5 | 7 | 0 |
| pa9 | 1.0 | 6 | 4 | 0 |
| pa9 | 1.0 | 7 | 3 | 0 |
| pa9 | 1.0 | 8 | 7 | 0 |
| pa9 | 1.0 | 9 | 2 | 0 |
| pa9 | 1.0 | 10 | 3 | 0 |
| pa9 | 1.0 | 11 | 3 | 0 |
| pa9 | 1.0 | 12 | 8 | 0 |
| pa9 | 1.0 | 13 | 4 | 0 |
| pa9 | 1.0 | 14 | 2 | 0 |
| pa9 | 1.0 | 16 | 4 | 39 |
| pa9 | 1.0 | 17 | 3 | 0 |
| pa9 | 1.0 | 18 | 4 | 18 |
| pa9 | 1.0 | 19 | 8 | 220 |
| pa9 | 1.0 | 20 | 4 | 0 |
| pa9 | 1.0 | 21 | 4 | 0 |
| pa9 | 1.0 | 22 | 8 | 0 |
| pa9 | 1.0 | 23 | 1 | 0 |
| pa9 | 1.0 | 25 | 1 | 0 |
| pa9 | 1.0 | 26 | 2 | 0 |
| pa9 | 1.0 | 27 | 2 | 0 |
| pa9 | 1.0 | 28 | 4 | 0 |
| pa9 | 1.0 | 29 | 3 | 0 |
| pa9 | 1.0 | 31 | 1 | 0 |
| pa9 | 1.0 | 32 | 3 | 0 |
| pa9 | 1.0 | 34 | 1 | 0 |
| pa9 | 1.0 | 36 | 1 | 0 |
| pa12 | 1.0 | 2 | 129 | 86 |
| pa12 | 1.0 | 3 | 127 | 37 |
| pa12 | 1.0 | 4 | 126 | 67 |
| pa12 | 1.0 | 5 | 125 | 124 |
| pa12 | 1.0 | 6 | 109 | 35 |
| pa12 | 1.0 | 7 | 99 | 175 |
| pa12 | 1.0 | 8 | 108 | 139 |
| pa12 | 1.0 | 9 | 109 | 152 |
| pa12 | 1.0 | 10 | 107 | 247 |
| pa12 | 1.0 | 11 | 104 | 474 |
| pa12 | 1.0 | 12 | 137 | 587 |
| pa12 | 1.0 | 13 | 151 | 683 |
| pa12 | 1.0 | 14 | 221 | 991 |
| pa12 | 1.0 | 15 | 174 | 0 |
| pa12 | 1.0 | 16 | 125 | 0 |
| pa12 | 1.0 | 17 | 118 | 0 |
| pa12 | 1.0 | 18 | 86 | 0 |
| pa12 | 1.0 | 19 | 76 | 0 |
| pa12 | 1.0 | 20 | 57 | 0 |
| pa12 | 1.0 | 21 | 44 | 0 |
| pa12 | 1.0 | 22 | 50 | 0 |
| pa12 | 1.0 | 23 | 50 | 0 |
| pa12 | 1.0 | 24 | 33 | 0 |
| pa12 | 1.0 | 25 | 39 | 0 |
| pa12 | 1.0 | 26 | 34 | 0 |
| pa12 | 1.0 | 27 | 37 | 0 |
| pa12 | 1.0 | 28 | 18 | 0 |
| pa12 | 1.0 | 29 | 24 | 0 |
| pa12 | 1.0 | 30 | 21 | 0 |
| pa12 | 1.0 | 31 | 15 | 0 |
| pa12 | 1.0 | 32 | 13 | 0 |
| pa12 | 1.0 | 33 | 10 | 0 |
| pa12 | 1.0 | 34 | 1 | 0 |
| pa12 | 1.0 | 35 | 4 | 0 |
| pa12 | 1.0 | 36 | 1 | 0 |
| pa14 | 1.0 | 0 | 25 | 1,223 |
| pa14 | 1.0 | 1 | 76 | 1,646 |
| pa14 | 1.0 | 2 | 5,559 | 76,574 |
| pa14 | 1.0 | 3 | 4,854 | 651 |
| pa14 | 1.0 | 4 | 4,058 | 80 |
| pa14 | 1.0 | 5 | 3,334 | 0 |
| pa14 | 1.0 | 6 | 2,853 | 85 |
| pa14 | 1.0 | 7 | 2,369 | 0 |
| pa14 | 1.0 | 8 | 2,039 | 97 |
| pa14 | 1.0 | 9 | 1,721 | 42 |
| pa14 | 1.0 | 10 | 1,425 | 0 |
| pa14 | 1.0 | 11 | 1,187 | 0 |
| pa14 | 1.0 | 12 | 963 | 0 |
| pa14 | 1.0 | 13 | 890 | 0 |
| pa14 | 1.0 | 14 | 705 | 0 |
| pa14 | 1.0 | 15 | 680 | 0 |
| pa14 | 1.0 | 16 | 535 | 0 |
| pa14 | 1.0 | 17 | 431 | 0 |
| pa14 | 1.0 | 18 | 344 | 0 |
| pa14 | 1.0 | 19 | 318 | 0 |
| pa14 | 1.0 | 20 | 269 | 0 |
| pa14 | 1.0 | 21 | 204 | 0 |
| pa14 | 1.0 | 22 | 217 | 0 |
| pa14 | 1.0 | 23 | 192 | 0 |
| pa14 | 1.0 | 24 | 138 | 0 |
| pa14 | 1.0 | 25 | 121 | 0 |
| pa14 | 1.0 | 26 | 107 | 0 |
| pa14 | 1.0 | 27 | 101 | 0 |
| pa14 | 1.0 | 28 | 93 | 0 |
| pa14 | 1.0 | 29 | 67 | 0 |
| pa14 | 1.0 | 30 | 61 | 0 |
| pa14 | 1.0 | 31 | 53 | 0 |
| pa14 | 1.0 | 32 | 28 | 0 |
| pa14 | 1.0 | 33 | 18 | 0 |
| pa14 | 1.0 | 34 | 19 | 0 |
| pa14 | 1.0 | 35 | 6 | 0 |
| pa14 | 1.0 | 36 | 2 | 0 |

Dropped axes with a living apex (apex_end none): 0 axes, 0 phytomers.


## Unknown or not measured

- Whether a later year's pass actually reads a given dropped axis was not traced at runtime; item 3's last table is read from the module docs of `compact.rs`, `retire.rs` and `working.rs` only (cited in the table and in the head of this file).
- Retired roots let go before the pause are not axes in the grower, so their phytomers and axes cannot be split here; only their number ("standing") is known.
- The footprint stores that are not per-axis (leaves, live, retire.stands, horizon, units, roots, the working set's lists) cannot be split into kept and dropped; they are listed per tree under item 4. Each split store's last column is Vec capacity beyond length, which belongs to neither part. `axes.units`, `axes.joined`, `pencils.inherited`, `asleep`, `woken` and `marked` read zero or were not split.
- Peak memory over the settle itself was not measured here; the footprint is the grower's held bytes at the settle's entry.
- The per-axis "cycles since the apex ended" is the axis's own `apex_end`; the shed rule reads its subtree's last living time, so the two can differ for an axis dropped with its bearer.
- The oak and spruce were run only as shipped presets, not as `oak()` and `spruce()`; the beech was run both ways.
- Each tree is one run of one seed.

## Closing

Of the shipped European beech at 80 on seed 7, the grower holds 636,161 axes and 7,107,275 phytomers and the settle keeps 408,085 axes and 6,170,796 phytomers, dropping 228,076 axes (35.9%) and 936,479 phytomers (13.2%); 174,690 of the dropped axes are retired roots or inside-retired records holding no phytomers, and 726,248 of the dropped phytomers sit on axes whose apex ended two cycles ago, 288 of the 1,497 MiB footprint (19.2%) belong to dropped axes. The 27,138,515 phytomers on 1,846,068 axes of the brief are `telperion_space::beech()`, not the shipped preset: that species settles to 24,178,429 phytomers on 1,469,354 axes, dropping 2,960,086 phytomers (10.9%) and 376,714 axes, with 652 of its 5,743 MiB attributable to dropped axes. The largest dropped share of phytomers among the shipped trees is the oak at 120 (762,501 of 3,863,297, 19.7%), and the largest dropped share of axes is the shipped beech at 80 on seed 7 (35.9%).
