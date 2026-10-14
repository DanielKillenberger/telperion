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
