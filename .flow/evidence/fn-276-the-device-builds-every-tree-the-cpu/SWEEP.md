# fn-276 after the host's ruling on fix 1: the floor, FRAMES, and R4's sweep (worker, 2026-10-14)

Code: d4715567c (the floor without failures, FRAMES 11), 49cdf81e6 and c4bb76107 (the sweep). Raw output is in `raw/` (ignored): `f1c-floors.log`, `f1c-cpu.log`, `f1-presets.log`, `f1-walks-3.log`, `f1c-gpu.log`, `f1c-ab.log`, `r4-*.log`, `r4-given-trace*.log`.

## The floor without the failure term, against the CPU's count (`tests/units_bound.rs`)

Every preset at every whole age from 1 to its own and to 80, and the film's walk frames, at seeds 1, 7 and 42: 2,034 cases. No case passes the floor.

| Cases | Floor at most | Counted at most, living / waking |
|---|--:|---|
| Date palm (240) | 1 | 1 / 0 |
| European beech (240) | 2 | 2 / 0 |
| Laurelin (240) | 2 | 2 / 0 |
| Norway spruce (240) | 2 | 1 / 1 |
| Ordinary (240) | 3 | 3 / 3 |
| Oregon white oak (240) | 2 | 2 / 2 |
| Silver birch (240) | 2 | 2 / 1 |
| Telperion (246) | 2 | 2 / 0 |
| Walk frames (108) | 11 | 4 / 4 |

**FRAMES:** the largest floor in the sweep is 11, at date palm to Laurelin at t = 0.01. `FRAMES` is 11 in `job.wgsl` and `device.rs`, and a mirror test holds the two equal.

## Cost

**Time.** GPU-stamped, Telperion 82 seed 42 (`telperion_is_timed`), the base commit 850eb039a against now, three runs alternated:

| | Stamped total (ms) | Units and wake (ms) | Light (dispatches) | Moves |
|---|--:|--:|--:|--:|
| Base | 696.1 | 231.9 | 1,513 | 1 |
| Now | 701.4 (+0.8%) | 233.0 | 1,649 | 3 |

The floor adds no units rounds on Telperion (9,917 units dispatches either way). The light's extra dispatches and the two extra moves come from the node table growing by its box (fix 3).

Warm wall time (`build_time`, three alternated runs, `f1c-ab.log`) moved +2.0% on Telperion (1,278 to 1,304 ms) and +3.6% on the Laurelin (1,189 to 1,232). A second set of runs (`f1c-ab3.log`) with `FRAMES` at 6, or the old node rule, could not separate either from now within their 3% spread: the machine was shared with other heavy runs. The stamped GPU time is the cleaner reading, at under 1%.

**Memory.** Peak MB, base to now (`f1c-ab.log`); every buffer is charged through the existing ledger, which is not touched here.

| Preset | Peak | Job state ("cycle counts") | Node table ("light depths") |
|---|---|---|---|
| Telperion 82 | 2,484 to 2,631 | 188 to 234 | 66 to 149 |
| Laurelin 80 | 2,281 to 2,420 | 187 to 233 | 61 to 149 |
| Norway spruce 40 | 1,522 to 1,568 | 162 to 208 | small |
| European beech 40 | 282 to 308 | 102 to 129 | small |

## R4's sweep on the device (`every_tree_builds_on_the_device`): a stop

Of 2,034 cases, 9 are refused other than by the budget, all by Given:

| Case | Cycle | Dispatch short |
|---|--:|---|
| Silver birch to oak at 40, t = 0.5, seed 42 | 5 | given 5 workgroups, needed 7 |
| Oak to palm at 40, t = 0.1, seed 42 | 5 | given 5, needed 6 |
| European beech, seed 1, ages 26 to 32 (7 builds) | 26 | given 83, needed 84 |

Every one is the same dispatch: `(MADE+1 + MADE+13) / 64`, the phytomers the units and the wake made in the cycle (`CYCLE_TRACE`). It is a census word that is not a table count, so `Span` projects it from its own last two reads, with no table floor. The beech is under by one workgroup at a mature cycle; the walk frames by one or two at cycle 5. The floor and FRAMES do not move the census, so these refusals most likely predate them. They were not tested at the base.

**Proposed fix, not built (for the host):** give the made-phytomer words a span from the phytomer table's own. The phytomers made in a cycle are its end count less its start count, which `Span` already bounds (`hi[PHYTS] - lo[PHYTS0]`, with the table floor of 4,096). This covers both a burst on a young tree and a mature cycle's step. The cost is wider dispatches over the made phytomers, to be measured.

**Refused by the budget, named:** the Norway spruce at 62 and 69 to 80, every seed ("the build's cycle state units would hold 4,200 MiB" at 62, the pencils from 69). The spruce at 63 to 68 builds. The Two Trees built every age at every seed (486 cases); the ordinary, oak, birch and palm every case.

## After the host's ruling on the made-phytomers dispatch (18e12b167)

**At the base.** All nine refusals are red at the base commit, 850eb039a, with the sweep test cherry-picked onto a detached head (`raw/g-base9.log`). They predate this spec's changes.

**The fix.** The span of the phytomers a cycle's units and wake made (`MADE + 1`, `MADE + 13`) is now the phytomer table's own span, its end less its start (`hi[PHYTS] - lo[PHYTS0]`). The count gets no projection of its own. `a_burst_of_made_phytomers_is_within_its_table` holds a burst past the old projection.

**The nine cases** now build (`raw/g-made.log`):
- the beech at 26 to 32, seed 1: 5,105 to 12,070 instances;
- silver birch to oak at t = 0.5, seed 42: 218,556;
- oak to palm at t = 0.1, seed 42: 72.

**GPU-stamped time.** The commit before (2b7918f97) against now, three runs alternated, median, in ms:

| Build | Before | Now | Units and wake, before / now |
|---|--:|--:|---|
| Telperion 82, seed 42 | 700.5 | 702.3 (+0.3%) | 233.4 / 235.5 |
| European beech 30, seed 42 | 67.9 | 69.6 | 18.47 / 18.49 |

The beech's units phase is unchanged. Its total's spread runs 67.9 to 69.9 on both sides.

**The whole sweep again** (`raw/g-*.log`): 2,034 cases, none refused but by the budget. The walk frames (108), the ordinary, oak, beech, birch and palm (1,200) and the Two Trees (486) all build. The Norway spruce is refused by the budget, by name, at 62 and at 69 to 80, as before.

## The Norway spruce at 60 to 80, and why 62 is refused while 63 to 68 build

**Why.** The build's start margin (`Sizes::margins`, fn-270 R4) starts the tables at margin 1.0 wherever those start tables fit the budget. It halves the margin only where they do not. Nothing checks room for a later move, which holds one group of tables twice. Traced with a throwaway print (`raw/g-spruce62.log`, the patch in `raw/g-spruce-diag.patch`, reverted), seed 1, at the 4 GiB native budget:
- **The spruce at 61 and 62** start at margin 1.0: units 2.95M and 3.04M rows, about 3.9 GB live. At 61 the units move at cycle 54 fits. At 62 the same move at cycle 55 would hold 4,200 MiB and is refused.
- **From 63** the margin-1.0 start does not fit, so the build starts at margin 0.5. Its moves then all fit: five or six of them, the last at about 3.5 GB live.

**At the browser's budget** (`TELPERION_DEVICE_BUDGET_MIB=6144`, `raw/g-spruce-6144.log`). The native build also caps at 85% of what the card leaves, and the desk card was shared, so the cap was 4.6 to 5.6 GiB, not 6 GiB.
- 60 to 70 build.
- 71 is refused: a units move at 5,758 MiB.
- 72 and 73 build.
- From 74 to 80, the pencils' move is refused at some seeds and builds at others (seed 42 at 76 to 80; 74 and 75 at every seed).

Whether the spruce builds at the full 6 GiB in a browser was not measured here.

**Not this spec's sizing rule.** The cause is the start margin, which checks only the start tables against the budget, never their first move. The made-phytomers rule is not involved, so nothing is fixed here.

A proposal for the host: choose the start margin so that the start tables plus their next move fit the budget, with the largest group held twice, as `build_move.rs` holds it.

## Review, the stack fix and the gate

**Review.** Codex's first round was NEEDS_WORK on one finding: with `FRAMES` at 11, the job's stack of moves still held 8. A chain of eleven short ages, whose floor is 11, faulted at its ninth move with UnitFaults (0x10).

**The fix (a552e6cc9).** The stack now sits past the frames, with `FRAMES - 1` entries, and the job words are repacked (575 a job). `a_chain_of_short_ages_fills_a_jobs_frames` builds the chain on the device. It was red with the old stack and is green now (`raw/g-stack.log`), beside R1's repros and the mirror tests. The re-review was SHIP.

**The gate** (`raw/gate.log`) was run once at b7bf127da, before the stack fix: 1,383 passed, 1 failed, 95 ignored. The one failure, `every_shader_file_is_checked`, compares the WGSL files on disk against the sources compiled in. `job.wgsl` was being edited for the stack fix while the gate ran, so the file on disk no longer matched the binary. Re-run on the current source, it passes.

**What was not re-run.** The whole R4 sweep ran before the stack fix. After it, only R1's repros, the chain test and the mirror tests ran. The stack's layout change has not been through the 2,034-case sweep or a gate of its own. The host runs the gate on the merged branch.

## Task 2: a build's start leaves room for its first move (3fc6acf65, 615a153bf)

**The rule.** `Growing::within` takes the largest start margin whose tables the budget holds beside their first move. That move is the largest state group a move holds twice (the phytomers, the axes, or the units and lists), made `GROWTH` times larger (`Sizes::first_move`). It is the same margin ladder, with no species branch.

The room is checked against the cap and charged to nothing (`Ledger::fits`). A first version reserved the room and gave it back, which raised the ledger's peak: Telperion read 2,634 to 3,536 MB and the spruce at 40 read 1,571 to 2,244 (`raw/t2-gpu.log`). With `fits`, the peaks are the base's again: spruce 40 1,571, Telperion 2,634, Laurelin 2,423 (`raw/t2-fits.log`). Both versions make the same comparison against the cap, so they make the same decision.

**Red and green.** `the_spruce_at_62_starts_with_room_to_move` (ignored; it needs 4 GiB free on the card) builds the spruce at 62, seed 1, under the native 4 GiB cap:
- **Red without the room:** "the build's cycle state units would hold 4,203 MiB, past the 4,096 MiB it may hold" (`raw/t2-spruce62-red.log`).
- **Green with it:** 9,065,481 instances (`raw/t2-spruce62-green.log`, `raw/t2-fits.log`).

**The spruce at 60 to 80, seeds 1, 7 and 42, at `TELPERION_DEVICE_BUDGET_MIB=6144`** (`raw/t2-spruce-6144.log`). Every build builds, 63 of 63. The sweep now prints each build's cap: 5,838 MiB for the first build, 5,640 to 5,646 MiB after. That is this desk's card, at 85% of what other processes leave, not the stated 6,144. Every spruce to 80 builds within 5.64 GiB, under the browser's 6 GiB.

**The spruce at the native 4 GiB** (`raw/t2-spruce.log`, cap 4,096 MiB on every build):

| Ages | Outcome |
|---|---|
| 1 to 68, 70 to 72 | build at every seed (62 now among them) |
| 69, 73, 74 | build at seeds 1 and 7; seed 42 refused by the budget |
| 75 to 80 | refused by the budget |

The refusals that remain come at later moves, not the first:
- 69 seed 42: the pencils' move, 4,590 MiB;
- 73 and 74 seed 42: the phytomers' move;
- 75 to 77: the units' move;
- 78 to 80: the pencils' move.

Those later moves are outside this task's rule. All are budget refusals, named.

**The other cases are unchanged.** The walk frames (108), the ordinary, oak, beech, birch and palm (1,200) and the Two Trees (486): all 1,794 have the same outcome and the same instance count as the sweep before task 2.

**Cost.** GPU-stamped, a96f48ef2 against now, three runs alternated, median, in ms:

| Build | Before | Now |
|---|--:|--:|
| Telperion 82 | 693.5 | 693.3 |
| Norway spruce 40 | 232.6 | 232.5 |

The moves are the same: 3 and 2 (`raw/t2-gpu.log`).
