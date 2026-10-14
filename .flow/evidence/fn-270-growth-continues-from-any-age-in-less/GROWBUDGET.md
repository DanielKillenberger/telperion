# fn-270 task 6: a grow-in past the budget (worker, 2026-10-14)

The owner grew an 80-year Norway spruce in with the harness's "grow in" and got an out-of-memory error. Raw output is in `raw/t6/` (ignored).

## The repro

`grows_in_under_the_page_budget` (`resident/grow_in_peak.rs`, run by name) does what the harness does. It builds the tree at once and puts it up, begins growth with the age as its horizon, then for each whole year calls `grow_to`, `settle` and puts the result up in place of the last. The budget is the page's 6 GiB (`BROWSER`). Each step's peak is a labelled ledger census. The test passes when the grow-in grows whole, or when the budget refuses it by name and the tree drawn before is still up, buffer for buffer. A refusal from the device itself fails the test.

## What the grow-in holds that the build at once does not

On the spruce at 80, the build at once peaks at 6,406 MB (6,109 MiB), while it grows into tables started at the hint. It frees the growth state before it settles and places the tree. The grow-in instead holds four things together through every draw:

- the growth state, about 3.1 GB at year 76: pencils 826, axes 728, units 418, phytomers laid 439 and grown 157, retirement 182, the hierarchy's counts, lists and shape about 340
- last year's settled node table for the year's presentation (`previous`), 444 to 468 MB
- the tree drawn now, about 550 MB of foliage, curve, clusters and the renderer's own 268
- the draw's own scratch: the settle (nodes, work, place, about 1.4 GB) and the dress (700 MB), then the placement (ring positions 946 to 975, station segments 524 to 541, fine rings 250, sweep points 175, the new tree's 300)

Its tables started at half the hint's margin, because the margin-1.0 tables did not fit beside the tree drawn at once. At year 79 the axes outgrow them. The move to 1.5 times holds the old and new axes group together: pencils 1,239 and axes 1,092, plus the hierarchy and retirement sized to match.

## Peaks, year by year (MB, grow / draw; raw/t6/*-spruce80.log)

| Year | Base (`diag2`) | Ledger fix (`l1b`) | Settle scratch shed before the dress (`l2`, dropped) | Ledger fix, no tree drawn (`virt`) |
|--:|--:|--:|--:|--:|
| 70 | 5,342 / 6,196 | 5,342 / 5,726 | 5,342 / 5,594 | 4,803 / 5,187 |
| 73 | 5,269 / 6,386 | 5,269 / 5,875 | 5,269 / 5,747 | 4,713 / 5,320 |
| 76 | refused at 74 | 5,508 / 6,257 | 5,508 / 6,116 | 4,933 / 5,682 |
| 78 | | 5,535 / device OOM | device OOM at 77 | 4,949 / 5,771 |
| 79 | | | | 6,514 (the axes move) / over 6,872 (device OOM) |

The budget is 6,442 MB (6,144 MiB). The `virt` column does not keep the drawn tree (the begin sized its tables beside it, and the budget was opened past 6 GiB afterwards), so the run could reach year 79 on this desk's card. Adding the drawn tree's 550 to 600 MB back gives the faithful grow-in: about 7.07 GB for year 79's growth and at least 7.4 GB for its draw.

## Levers, one at a time

1. **Kept: the ledger lets a destroyed buffer go (a guard fix).** `Owned::destroy` went through `Deref` to `wgpu::Buffer::destroy`. That freed the device's memory, but the ledger kept charging the buffer until its owner dropped. The placement destroys the sweep's 640 to 700 MB of nodes before it makes the rings, so the guard refused the spruce at year 74 for memory the device had already freed. `Owned::destroy` now lets the buffer go through the ledger once, as a drop does, and the placement waits for the device before it makes the rings. Red before and green after: `a_buffer_let_go_early_leaves_the_ledger_once`. The draw peak at year 73 falls 8.0% (6,386 to 5,875), and the refusal moves from year 74 to year 79. Because the guard's charge now matches what the device holds, this is kept as a correctness fix, not judged by the 10% rule.
2. **Dropped: the settle's scratch shed before the dress.** Freeing the settle's work, counts and placing words before the chains and the dress take about 950 MB off the dress stage. The placement then becomes the peak, so the overall draw peak falls 2.2% (5,875 to 5,747 at year 73). That is under the 10% rule.

Neither lever reaches year 80. What is left over budget is the growth state beside a draw: state plus `previous` plus the drawn tree plus the placement, about 6.4 to 6.6 GB from year 77 on even without the move, and over 7 GB with it.

## Every preset, grown in under 6 GiB (MiB, ledger peak; raw/t6/all-l1, all-base)

| Preset | At its age: built at once / grown in | At 80: built at once / grown in |
|---|--:|--:|
| date-palm (40) | 30 / 30 | 30 / 30 |
| ordinary (40) | 161 / 184 | 1,287 / 1,624 |
| european-beech (40) | 269 / 299 | 2,135 / 2,779 |
| silver-birch (40) | 496 / 546 | 2,421 / 2,670 |
| oregon-white-oak (40) | 746 / 808 | 6,088 / 5,933 (year 1: the begin's tables, capped to the budget) |
| norway-spruce (40) | 1,451 / 1,837 (base 2,000) | 6,109 / refused by name at year 79 (the axes move, 6,738 MiB) |
| laurelin (80) | 2,175 / 3,381 | same |
| telperion (82) | 2,369 / 3,589 | 2,175 / 3,328 |

Every preset but the spruce at 80 grows in at its own age and at 80 under 6 GiB. The ledger fix changes only the spruce, the one preset whose peak is a placement after the sweep.

## The device on this desk

On this desk, with other processes holding 2.6 GB of the 10 GiB card, the device itself refused the spruce's draw at years 76 to 78, while the ledger held 5.9 to 6.1 GiB. Natively the page's 6 GiB plus what the ledger does not count does not fit there. Whether the owner's error was the budget's refusal or the device's own is unknown until we have its text.

## Host rulings (2026-10-14) and what they measured

The host ruled C and D, and not B.

**C, kept: a grow-in's draws follow fn-235's rule** (recorded in the spec's R2c). Builds and draws now go through one admission check, `budget::room`. A draw's expected bytes are the last draw's, grown by the ratio of the last draw to the one before, plus a 10% margin (`DRAW_MARGIN`). The draw that went up was at most 0.958 of its expected bytes, on every preset at its age and at 80 (draws of at least 64 MiB, `raw/t6/all-c`). Below that size the extrapolation peaked at 1.09 of the plain estimate (the spruce at year 46), which the margin covers. In the harness, a draw that makes room reports it through `makingRoom`, the hook builds already use. Tests: `a_grow_in_draw_lets_the_drawn_tree_go_only_where_it_fits_alone` (beside, make room, refused with the drawn tree kept, then put up alone), and `grow-in.test.ts` (the hook is told while the tree is let go).

**D, dropped: a finer starting-size ladder.** Each ladder starts at the largest tables that fit beside the tree drawn at once, so a finer step costs memory for the whole run. At ×0.75 the run started at margin 0.75 and held 1.3 GB more from year 10 on (4,168 against 2,869 MB). It was refused at year 50. At ×0.6 it held 411 to 520 MB more all along (3,389 against 2,869 MB at year 10, 4,512 against 4,101 at year 70), which is past the 0.3 GB estimate (`raw/t6/d075-*`, `d06-*`). The halving ladder stays.

**The spruce at 80 under 6,144 MiB, with C (`raw/t6/c2-spruce80.log`).** Draws 1 to 74 fit beside the drawn tree. Draws 75 to 78 fit only alone, so each lets the drawn tree go first and the canvas is blank while it draws. The growth to year 79 is then refused by name, keeping the drawn tree: the axes move's new tables would bring the device to 6,730 MiB. The run's peak is 6,131 MiB, at year 1, where the start-up tables are capped to the budget. With D dropped, the late move remains, so the spruce at 80 stays refused by name at year 79.

**Every other preset** grows in at its age and at 80 with no draw letting the drawn tree go. Their peaks are unchanged from the table above.
