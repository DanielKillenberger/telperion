# fn-256 task 5: the device guard, the census, and the arena (worker, 2026-10-12)

COUNCIL.md round 4, steps 1 to 3 of task 5. The guard is built and tested, and the census and the live set are measured; the live set's stop condition did not fire. The census showed the arena's sizing needed calls COUNCIL did not make ("Open", kept as asked), and the host ruled on them the same day: the arena as built is the last section. Raw output is in `raw/` (`t5-*`). Measured on the RTX 3080 (10 GiB), with 1.8 to 3.0 GiB held by desktop processes, one GPU process at a time under the GPU lock.

## The guard (step 1)

- **A native budget.** A native build is held to `TELPERION_DEVICE_BUDGET_MIB`, else 4 GiB (`budget::NATIVE`), and to no more than the bytes it holds plus 85% of the device memory the driver says the process may still take (`Gpu::free_device_memory`, VK_EXT_memory_budget through wgpu's Vulkan device). That figure falls as other processes fill the card: at 2.0 GiB held by others, the driver's budget was 8.24 GB, the card less what they held.
- **Reserve before create.** `io::buffer` charges a buffer's bytes in one atomic step before it makes it (`Ledger::reserve`), and natively makes it under scopes of its own, so a device that cannot hold it refuses it by name there.
- **Charges held until completion.** A destroyed buffer stays charged until the device has finished the work submitted before its destroy (`on_submitted_work_done`). A move's old tables therefore count beside its new, and a dropped build's tables count beside the next build's.
- **Polled free.** A resident build waits for the device before its budget check and again before it returns; the grow loop waits before each begin.
- **Counts clamped to capacity** in the cycle's kernels:
  - the hierarchy's block runs and child ranges (`within`, `kids_within`), its scanned runs and merges, the levels' and blocks' counts, and the growing buds;
  - the layout's phytomers, units and lifts per axis, and its unit-list walks;
  - retirement's parent walk and unit walks, and the glue's tip walk.

  The light's lattice was already held to its capacity by `grid_main`. The settle's and the dress's kernels run only on a build whose census raised no flag, and are not clamped.
- **Tests:**
  - `a_halted_cycle_leaves_later_dispatches_empty` raises a flag as the beech's sixth cycle begins and holds every buffer a cycle carries (state, the hierarchy's shape and lists, retirement's columns) to its words before. It was red: retirement's `end_main` wrote the laid count, none in a halted cycle, over the last cycle's. That kernel now leaves it.
  - `a_buffer_let_go_counts_against_the_cap_until_it_is_destroyed` now also holds the buffer charged after its encoder closes, until the device is polled.
  - "A build past the budget submits nothing" is not written. Today's builds grow by moves, so a build past the budget has submitted cycles before a move is refused. It becomes true only once a build reserves its tables before cycle 1 (step 4).

## Ten builds on a two-thirds-full card

`full-card.sh` holds the card at two thirds with a ballast process (3,855 MiB beside the desktop's 2,971), then builds Telperion 82, the oak 80, Laurelin 80, the birch 40 and the ordinary 40 in turn, ten in all (`raw/t5-full-card-1.txt`).

| Build | Ended |
|---|---|
| Telperion 82, twice | refused by name at 1,916 MiB of a 1,766-1,775 MiB cap |
| oak 80, twice | refused by name at 1,780 MiB of a 1,766-1,769 MiB cap |
| Laurelin 80, twice | refused by name at 1,903 MiB of a 1,766-1,767 MiB cap |
| birch 40, twice | built, 194,530 instances, 0.8 s |
| ordinary 40, twice | built, 13,792 instances, 0.5 s |

No device was lost and no reset happened; the card was back at 2,971 MiB once the ballast stopped. Earlier, with only the desktop's 2,970 MiB held, Telperion was refused by name at 5,455 MiB of a 5,443 MiB cap, near where task 4's builds hung (NVRM Xid 109).

## Warm build and peak memory per preset, under the guard

Warm build in ms, the second build in the process (`raw/t5-build-time-guard.txt`). Peak is the build's owned bytes in MB. F4 and task 4 are PROOF.md's table.

| Preset | F4 | Task 4 | Now | Task 4 peak | Now peak |
|---|--:|--:|--:|--:|--:|
| ordinary 40 | 523 | 604 | 553 | 458 | 483 |
| oak 40 | 1159 | 525 | 514 | 562 | 687 |
| spruce 40 | 403 | 613 | 610 | 1911 | 2210 |
| beech 40 | 210 | 295 | 282 | 470 | 576 |
| birch 40 | 716 | 782 | 784 | 783 | 787 |
| palm 40 | 4 | 98 | 98 | 163 | 163 |
| Telperion 82 | 671 | 1909 | refused | 6122 | refused at 4,113 MiB |
| Laurelin 80 | 627 | 1782 | refused | 5526 | refused at 4,913 MiB |

The guard costs no build time. The peaks read 0 to 23% higher because a destroyed buffer now stays charged until the device is done with it, which is when the device frees it. The Two Trees cross the 4 GiB default in the light's scratch, so `every_device_preset_builds_the_same_leaves_each_time` fails on Telperion. That is the render library's one failure (164 passed, `raw/t5-render-lib-guard.txt`).

## The labelled peak census (step 2)

Each build's owned bytes by label at its peak (`Ledger::take_peak_census`, `build_time` prints it). The oak at 80 built (`raw/t5-census-oak80.txt`). Telperion is its census when refused, mid-move, at 5,681 MB (`raw/t5-census-telperion.txt`).

| Tables, MB | oak 80 | Telperion 82 |
|---|--:|--:|
| Indexed by axis: pencils, axes, retirement's verdicts and columns, the hierarchy's sums, tree, counts, shape and lists, the cycle's inputs (Telperion's inputs were being made when it was refused) | 1,957 | 2,108 |
| Phytomers: laid and grown | 574 | 1,094 |
| Light scratch, and Telperion's points | 87 | 1,857 |
| Units | 95 | 186 |
| The units' job counts | 165 | 192 |
| Peak | 3,083 | 5,681 |

Per axis row, the axis-indexed tables hold 854 bytes; a phytomer row holds 76.

## The live set per cycle on the CPU cycle (step 3)

`early live <preset> <age> 42`: each cycle's phytomers not retired against those grown by its end (`raw/t5-live-*.txt`).

| Tree | Grown | Live at the end | Share | Axes at the end |
|---|--:|--:|--:|--:|
| Telperion 82 | 18,239,310 | 3,831,128 | 0.210 | 1,532,978 |
| Laurelin 80 | 16,564,124 | 3,582,961 | 0.216 | 1,396,481 |
| oak 80 | 8,499,924 | 1,511,312 | 0.178 | 1,244,690 |

The live set is largest in the last cycle on all three. Its share falls from 1.0 in the first cycles to about 0.4 by cycle 22 and about 0.2 at the end. Past cycle 10 it passes 0.6 only in cycles 11 to 13, which hold 3,500 to 19,000 live phytomers. **The stop condition (live over 60% of grown) does not fire:** an arena at the live set holds a fifth of Telperion's grown rows.

## Open, for the host: the arena's sizing (step 4)

The plan sizes one arena once from the live-set bound. The census shows the phytomer rows the live set bounds are 19% of the oak's peak and 19% of Telperion's census. Zero table moves needs every table sized once, and these calls are open:

1. **The axis-indexed tables have no live set.** They are 63% of the oak's peak, 854 bytes an axis. An axis keeps its CPU index (DESIGN.md section 0), so these rows never compact, and they grow with every axis ever grown: 1.53M on Telperion, 1.31 GB at its final count. Sizing them once needs one of:
   - one bound measured on the shipped presets, which every build allocates, the beech at 5 included;
   - axes retired and renumbered as phytomers are, which changes the identity rule;
   - their moves kept, which fails "zero table moves".
2. **One arena for every build, or one per build.** Telperion's live set is 3.83M rows, 291 MB, and a column of scratch 15 MB more. A per-build size needs a count before cycle 1, which "never an expectation" (DESIGN.md section 5) forbids. One bound for all makes the beech hold Telperion's arena.
3. **The light's scratch** is the largest label at Telperion's census, 1,802 MB. It is sized at twice the lattice's nodes (`build_size::ahead`), outside any arena.
4. **The native default.** 4 GiB refuses the Two Trees today, so the render library's repeat test fails on Telperion. Raising the default, or keeping it until the arena brings their peak under it, is the host's call.
5. **Restarts.** Most of task 4's remaining restarts come from the cycle's limits (units rounds, the sweep's layers), not from tables. Whether "moves and restarts deleted" covers those is open.

At their final counts with no moves, Telperion's tables would hold about 1.3 GB by axis, 0.3 GB of phytomers, 0.2 GB of units and 0.2 GB of job counts, plus the light's scratch (0.9 to 1.8 GB). That is 2.9 to 3.8 GB, an estimate from the counts above that no build has measured.

## The host's rulings (2026-10-12), as built

DESIGN.md section 13 has the design. What each ruling became:

1. **Axes compact too** (`compact.rs`). Between cycles, the arena compacts in place, a column at a time through one column of scratch. Phytomers of retired subtrees go. Where no PA keeps records, so do the axes inside retired subtrees and their units, each root carrying its frozen upkeep. Every index into them is remapped first. Tests compare by lineage key, and DESIGN.md section 0's index rule is marked retired. With compaction after every cycle, all eight young presets settle as the CPU's laid tree, and their node tables agree over ten contexts and three block caps.
2. **The arena per build from the hint** (`build_arena.rs`). The hint is the closed form's expected kept count, its light terms let go for a species that reads light, computed in under 0.1 s. Each table is sized from its measured most against the hint, over every preset at seeds 1, 7, 42 and its own (`arena_ratios`), past a margin of 1.5. The ratios spread widely: phytomers run 0.37 (oak) to 1.90 (beech 42), axes 0.13 to 0.65 (spruce). So one ratio a table over-sizes some trees: Telperion's axis tables are sized about four times what it uses. **The gate** (`every_preset_grows_in_its_hinted_arena`, `raw/t5-gate.txt`): all eight presets at seeds 1, 7 and 42 grow with 0 moves and are never refused. At most, a table is 0.73 full (the birch's phytomers at seed 7).
3. **The light's scratch.** The 61,600 nodes came from the CPU census of the preset's raw species. The device grows `executor::growth`'s species, which the supernatural rows scale, and Telperion's lattice reaches 822,000 nodes. The scratch held ten words of column records a direction for every node, 432 bytes a node, where the records are needed a column of a layer. It now holds two words a direction a node and the records a column (`columns`, checked against capacity in `grid_main`). Telperion's census went from 1,802 MB to 179 MB. The retirement verdict log (204 MB, read only by a test) is now kept only where a test asks.
4. **The native default stays 4 GiB.** The Two Trees fit once the build lets go of its cycle-only buffers before the settle (`Build::settling`, about 1.3 GB on Telperion): Telperion peaks at 3,990 MB and Laurelin at 3,707 MB.
5. **No restarts.**
   - Units rounds are stated at 5 and recorded every cycle; 4 refused the ordinary at cycle 18.
   - Every list's ranges, the hierarchy's levels, generations and passes, the layout's generations and the sweep's layer ranges are bounded from the tables (`Sizes::stated`). Ranges, levels, generations and the sweep stay projections past their counts. The sweep's bound comes from the lattice's nodes stood as a cube twice over, a stated bound the species' rows do not give: no height or spread row exists in the species.
   - A count past any limit or table is refused by name (`build_size::refusal`); the restart path, `raised`, the slacks and the widths are deleted.
   - A dispatch's workgroups come from projections that read each table's growth within its own cycle, which compaction does not hide. They are capped at the table's rows and floored at 4,096 items, and the census read after a compaction sets their floors.
   - The units' round markers that fed the deleted round projection are deleted.

### Warm build and peak per preset, the arena built

Warm build in ms, the second build in the process; peak is the build's owned bytes in MB, the settle included (`raw/t5-build-time-arena.txt`). F4 and task 4 are PROOF.md's.

| Preset | F4 | Task 4 | Now | Task 4 peak | Now peak |
|---|--:|--:|--:|--:|--:|
| ordinary 40 | 523 | 604 | 514 | 458 | 370 |
| oak 40 | 1159 | 525 | 380 | 562 | 1416 |
| spruce 40 | 403 | 613 | 563 | 1911 | 1645 |
| beech 40 | 210 | 295 | 306 | 470 | 424 |
| birch 40 | 716 | 782 | 356 | 783 | 894 |
| palm 40 | 4 | 98 | 132 | 163 | 146 |
| Telperion 82 | 671 | 1909 | 2010 | 6122 | 3990 |
| Laurelin 80 | 627 | 1782 | 1897 | 5526 | 3707 |

- **Faster than task 4:** the ordinary, oak, spruce and birch. They no longer begin again or move; the birch had begun again twice late.
- **Slower:** Telperion by 5%, Laurelin by 6%, the beech by 4%, the palm by 35 ms. On Telperion, 5 to 7 compactions cost 15 to 25 ms each, and all five units rounds are recorded every cycle: about 1,000 dispatches a cycle against task 4's 690.
- **Peaks:** the Two Trees peak at two thirds of task 4's and fit the 4 GiB default. The oak at 40 and the birch hold more than in task 4, because their tables are sized by the ratios the spruce and the beech set.
- **Against F4,** every preset but the ordinary, the oak and the birch is slower. The Two Trees take three times as long. Reaching F4 is task 6's dispatch work.

### Ten builds on a two-thirds-full card, the arena built

`full-card.sh`, a ballast of 5,006 MiB beside the desktop's 1,820 (`raw/t5-full-card-arena.txt`). The birch and the ordinary built twice each. Telperion, Laurelin and the oak at 80 were refused by name twice each, at their first tables, before any cycle was submitted. No reset.

### Open, for the host

- **The oak at 80 no longer fits 4 GiB even on a free card.** Its hint (8.0M expected kept) is five times its live set (1.5M), and the spruce's axes ratio sizes its pencils alone at 3.3 GB. In task 4 it built at 3.1 GB. One ratio a table can't fit both; a hint for the axes from the closed form would.
- **The sweep's bound** is stated from the lattice's nodes, because the species carry no height or spread rows.
- **Speed:** the Two Trees are 5 to 6% slower than task 4 (above).
