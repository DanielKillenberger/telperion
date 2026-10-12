# fn-256 task 4: speed, memory, stills, Chrome (worker, 2026-10-12)

Branch `fn-256-the-whole-grower-runs-on-the-gpu`, after merging F4 (fn-253's flowers) and fn-266 (the palm's boots). Raw output is in `raw/`: `t4-build-time-final.txt`, `t4-compile-cold-{before,after,parts}.txt`, `t4-rss.txt`, `t4-stills/`. Measured on the RTX 3080 (10 GB, about 2 GB held by desktop applications), one heavy process at a time under the GPU lock.

## The merges

- **F4 (fn-253).** This branch had deleted `generation/fine/`, so fn-253's device side is ported onto the cycle's drawing path:
  - the cycle's dress seats a bloom at its axis's own tip alone;
  - the stations count a bloom's rachis after its florets;
  - the florets pack against the one bloom box both executors share.
  `stalk_tests.rs` runs on the resident build and gives F4's numbers: 12,294 florets and 2,072 rachises on both executors, at most 0.45 px off the drawn rachis. fn-253's `preset_florets` probe is kept there; the floret word matcher it called went with `fine/` in task 3.
- **fn-266.** Merged whole. The palm now draws on the device, its boots included.

## Warm build per preset (R4), ms

The second build in the process, family to draw-ready (`build_time`). F4 and task 3 are task 3's table (DESIGN.md 12.5b).

| Preset | F4 | Task 3 | Now | Restarts / moves | Dispatches a cycle | Device peak MB |
|---|--:|--:|--:|---|--:|--:|
| ordinary 40 | 523 | 518 | 604 | 2 / 30 | 835 | 458 |
| oak 40 | 1159 | 515 | 525 | 5 / 23 | 572 | 562 |
| spruce 40 | 403 | 657 | 613 | 4 / 33 | 694 | 1911 |
| beech 40 | 210 | 287 | 295 | 1 / 16 | 485 | 470 |
| birch 40 | 716 | 739 | 782 | 3 / 30 | 560 | 783 |
| palm 40 | 4 | refused | 98 | 0 / 1 | 355 | 163 |
| Telperion 82 | 671 | 5929 | **1909** | 1 / 58 | 690 | 6122 |
| Laurelin 80 | 627 | 5834 | **1782** | 1 / 55 | 673 | 5526 |

Every remaining restart is at cycle 4 to 18, where a restart costs a few small cycles. The exception is the birch, which has two late ones (axes at 36, units and phytomers at 38), worth about 0.4 s.

**What changed**, each measured on the presets:

1. **The light's sweep limit** projects the grid two cycles on. Its count steps with the layer's area: Telperion's went from 29 to 54 in one cycle. This was 7 of Telperion's 10 late restarts.
2. **Units rounds are gated.**
   - Each round marks itself used, the living apexes' and the wake's apart.
   - A cycle records the rounds the last one used plus two, up to eight.
   - A round short still raises `UnitRounds`, and the build begins again.
   - Rounds used are 1 or 2 in most cycles.
3. **Slack and plan bounds.** The slacks start at two for the hierarchy's generations and the layout's and units' rounds. Plan bounds rise by half again, so the plan is re-recorded a few times rather than every cycle.
4. **Table growth.**
   - The small tables (sleeping, living and woken buds) double when they grow.
   - The large tables keep a quarter over their projection. No phytomer table grows past the engine's budget, and a build past it is refused by name (`no_phytomer_table_grows_past_the_budget`, `a_build_past_the_budget_is_refused_by_name`, after Astra's round 1).
   - Wider steps (half again under 2^20 rows, then under 2^17) put the Two Trees out of device memory in the repeat test. So the moves stay: 58 on Telperion, about 0.6 s of host time.

**Per phase, Telperion 82** (`CYCLE_TRACE=1`):

| Stage | ms |
|---|--:|
| Species tables on the CPU | 40 |
| Device grow (recording 213, waiting 407, moving 652) | 1,395 |
| Settle | 199 |
| Chains, dress, trunk | 145 |
| Folds, sweep, stations, place, handoff | 92 |

**Dispatches by phase, Telperion 82** (before is EXECUTOR.md's run at the tree's own sizes):

| Phase | Before | Now |
|---|--:|--:|
| Units and wake | 34,742 | 26,178 |
| Balance and allotment | 19,471 | 19,331 |
| Light | 1,131 | 3,734 |
| Layout | 2,782 | 2,772 |
| Retirement | 3,027 | 2,981 |
| Sizing | 1,463 | 1,463 |
| **Build** | **62,700 (765 a cycle)** | **56,543 (690 a cycle)** |

At Chrome's 7 µs a direct dispatch, 690 a cycle is about 0.40 s of dispatch overhead on Telperion, against DESIGN.md 7a's target of about 340 a cycle. Two things hold the rest:
- **The hierarchy's sums** sweep about 28 block levels four times a cycle. A larger block cap saves only 4 levels (cap 256: 25 levels, HIERARCHY.md) and costs GPU time.
- **The units' phases** are an entry each, so they stay inside the 2 s cold compile.

Cutting further means changing the hierarchy's schedule, which is a design call.

## Cold compile (R4): native, driver cache off

Every program a page's builds dispatch, made as a page makes them (`the_cycle_is_made_cold`, `CYCLE_COMPILE=generator`):

| | Seconds |
|---|--:|
| Before: modules one after another, each module's entries side by side | 3.33 |
| **Now: modules side by side (`io::pair`)** | **1.45 (1.54 with every worker scoped)** |
| Task 2's cycle modules alone, side by side / in turn | 0.94 / 1.67 |
| By part, in turn: build kernels | 1.29 |
| By part, in turn: settle | 0.34 |
| By part, in turn: drawn | 0.55 |
| By part, in turn: the curve's passes | 0.58 |

A browser compiles every pipeline async already, so `io::pair` makes in turn there. Chrome's figure is the owner's probe.

## CPU peak during preparation (R3a)

Peak resident memory (VmHWM) of one resident build a process (`prepare_timing`, 4 threads):

| Tree | Peak MB |
|---|--:|
| palm 40 (the process's floor: device, every program compiled) | 772 |
| beech / oak / spruce / birch / ordinary at 40 | 827 / 852 / 845 / 862 / 873 |
| Laurelin 80 / Telperion 82 / oak 80 | 946 / 938 / 928 |
| ordinary 100, seeds 1 and 42 | 1,009 / 1,013 |
| **oak 100, seeds 1 and 42** | **refused: device out of memory** |

- **Against F4.** F4 peaked at 7.7 to 9.2 GB on the oak at 100 and 3.6 to 6.3 GB on the ordinary at 100. A tree now adds 0.05 to 0.24 GB over the process's floor.
- **Against master.** Master's 0.33 to 0.35 GB is under the floor here. fn-258's probe took that figure in its own binary, so the floors are not the same measure.
- **The oak at 100** does not build on the device on this card (see Open).

## Stills (R4)

Every preset's default age and seed, whole, mid and close, F4's tip against this branch (`raw/t4-stills/sheets/final-*.png`, every sheet viewed). The figure is the share of pixels differing past 2%.

| Preset | Whole | Mid | Close | Seen |
|---|--:|--:|--:|---|
| ordinary | 1.3% | 6.2% | 7.5% | same form; twigs and leaves placed differently |
| oak | 2.3% | 7.3% | 8.8% | same crown; the close view darker |
| spruce | 0.5% | 2.8% | 4.1% | near identical |
| beech | 0.2% | 0.6% | 0.5% | identical to the eye |
| birch | 1.0% | 5.5% | 11.3% | same habit; hanging twigs placed differently |
| palm | 2.5% | 12.8% | 14.0% | F4's crown and boots; leaflets and frond draws its own |
| Telperion | 1.0% | 1.5% | 1.8% | identical to the eye; candles at the tips at share 0.3, as approved |
| Laurelin | 1.7% | 8.2% | 6.8% | same crown; racemes hang from the tips |

The five species read exactly as in task 3. Task 4's changes size how the tree is grown, never the tree.

## The owner's Chrome probe

`.worktrees/probe` (branch `probe-v2`, commit `ff224a610`), `probe.html?tree=<preset>`. It builds that preset alone on a fresh page through this branch's `createRenderer` and `setTreeGpu`, the harness's own calls, and reports:
- module and device;
- programs compiled;
- the first build (cold);
- three warm builds and their median;
- errors, with a JSON block to copy.

To run it:
1. In this worktree, build the package's Wasm: `npm run wasm:build && npm run render:build`. Done at `2d8ddf38b`.
2. In the probe worktree, serve it with this worktree named: `TELPERION_CYCLE=/home/daniel/Projects/telperion/.worktrees/gpucycle npx vite --port 5190`.
3. Open `http://localhost:5190/probe.html?tree=telperion`, or any preset id.

Checked without a browser: the page's modules resolve to this worktree's `src/browser` and its Wasm, and the probe typechecks against both the stub and this branch's module. Not opened here; the owner runs it.

## Astra, round 1 (DISAGREE), fixed

1. **The round markers** were plain stores shared by many jobs, a WGSL data race even at one value. The units module's `lists` is now `array<atomic<u32>>` behind `ls` and `put_ls`, which also covers task 2's `T_ROUNDS` and `T_COLLAPSED` markers. The determinism, agreement and repeat tests pass, and the timings are unchanged (beech 287 ms, Telperion 1,926, Laurelin 1,785).
2. **Error scopes are a thread's own.**
   - Natively, each pipeline is now made under scopes of its own on the thread that makes it.
   - A refusal is kept by name on the `Gpu`, and native `programs::ready` reports the first, as the browser path does.
   - Test: `a_program_refused_on_a_worker_is_reported_by_name`.
   - Round 2 (still NOT FIXED then): a `pair` worker also makes shader modules and layouts. `io::pair` now runs its worker under scopes of its own and keeps what they catch for `ready` (`a_shader_refused_on_a_side_by_side_worker_is_reported`).
3. **The phytomer clamp** let a projection past the budget through. Fixed as above.

**Astra, round 3: AGREE** (`raw/t4-astra/astra-3.log`). Every native worker is scoped, the earlier two fixes stand, and no must-fix remains.

## The gate and npm test

- **The workspace gate** (`cargo test --profile ci --workspace --no-fail-fast`, `raw/t4-gate-workspace.txt`) was green but for one test. The repeat test ran out of device memory on Telperion while other whole-tree tests ran beside it. The Two Trees' peak equals the base's (Telperion 6,114 MB on the base against 6,122; Laurelin 5,619 against 5,526), so the sizing did not raise it. The repeat test now holds `whole_tree()`, as the other whole-tree tests do, and the render library is green (161 passed, `raw/t4-gate-render.txt`).
- **`npm test`:** 15 files, 162 tests, green (`raw/t4-gate-npm.txt`).

## impl-review (codex, against F4's tip `9e4f2c276`)

1. **Round 1, three reviewers** (one failed on the model's capacity): NEEDS_WORK. Two reviewers found one P1. A species that reads light has no expected peak, and the browser's admission took that as "fits beside the drawn tree", while its build now runs on the device.
   - **Admission:** with no expectation, a build makes room first wherever a tree is drawn, and is refused where the page already holds its budget (`a_tree_with_no_expectation_makes_room_first`).
   - **The build:** it is held to the budget as it grows. The ledger takes a build's cap, and `io::buffer` refuses by name any buffer that would pass it beside every byte held (`a_build_with_no_expectation_is_held_to_the_budget_as_it_grows`).
2. **Round 2:** a first version checked after each move and missed the move's peak. That check was replaced by the one before allocation.
3. **Round 3** (after a round-cap reset, per the owner's standing rule): a buffer let go while the copy encoder was open left the ledger at once. It now stays charged until it is destroyed (`a_buffer_let_go_counts_against_the_cap_until_it_is_destroyed`).
4. **Round 4: SHIP.**

After the fixes:
- the render library: 164 passed (`raw/t4-gate-render-final.txt`);
- `npm test`: 162 (`raw/t4-gate-npm-final.txt`).

**The device-memory guard** in Open item 2 is now built for the browser: a build there is refused by name before it passes the page's 6 GiB. Natively no budget is stated, so a native build is still bounded only by the card.

## Open, for the owner and the host

1. **Warm build, missed for six presets:**
   - Telperion 1,909 ms against F4's 671;
   - Laurelin 1,782 against 627;
   - spruce 613 against 403;
   - beech 295 against 210;
   - birch 782 against 716;
   - ordinary 604 against 523.

   The palm (98 against 4) was refused in task 3 and now draws with its boots.

   On the Two Trees, even with no restarts or moves, the device grow at the tree's own sizes is about 1.05 s of GPU work (EXECUTOR.md) plus the settle and drawing. Reaching F4's figure needs the hierarchy's dispatch schedule changed (a design call) and the 58 moves removed, which device memory does not allow (item 2).
2. **Device memory is at the card's edge.**
   - The Two Trees peak at 5.5 to 6.1 GB, the page's 6 GiB.
   - With 2 GB held by the desktop, a build that runs short hangs the GPU (NVRM Xid 109, four times today) more often than it is refused.
   - The oak at 100 is refused outright.
   - The browser's budget check admits every build, because `expected_kept` is none for species that read light.
   - A device-memory guard, and a budget bound that is not none, are the host's call (FRICTION.md).
3. **Dispatches:** 690 a cycle on Telperion against about 340, for the reason above.
4. **Chrome:** the cold compile and the warm build in Chrome wait on the owner's probe.
# fn-256 task 6: fewer dispatches a cycle (worker, 2026-10-12)

COUNCIL.md round 4, step 4, and step 5's re-measure. DESIGN.md section 14 has the design. Raw output is in `raw/t6-*`. Measured on the RTX 3080 with 1.6 GB held by desktop processes, one GPU process at a time under the GPU lock. "Base" is task 5's tip (`19e112eb1`), built and timed in this session beside this branch.

## The gate: at most 340 dispatches a cycle on Telperion — missed

Telperion 82, seed 42, a build's dispatches by phase (`CYCLE_TRACE`), a cycle's average in brackets:

| Phase | Base | Now |
|---|--:|--:|
| Units and wake | 43,631 (532) | 11,874 (145) |
| Balance and allotment | 22,032 (269) | 13,373 (163) |
| Light | 4,479 (55) | 2,430 (30) |
| Layout | 3,292 (40) | 2,621 (32) |
| Retirement | 3,251 (40) | 2,353 (29) |
| Sizing | 1,463 (18) | 1,463 (18) |
| Start | 84 (1) | 84 (1) |
| **A build** | **78,232 (954)** | **34,198 (417)** |
| Compaction, a build's | 1,428 (7 compactions) | 612 (3) |

At Chrome's 7 µs a direct dispatch, the build's dispatch overhead falls from about 0.56 s to 0.24 s, against DESIGN.md 7a's 0.2 s. Laurelin is 934 to 410 a cycle.

**What holds the rest, measured:**
- **Units, 145.** Five rounds every cycle (task 5: four refused the ordinary at cycle 18), each 3 dispatches counted and 5 emitted, a pass each range of 65,536 living, and the wake's two passes; scans 10. The range stays at 65,536: at 262,144 the passes halved, but Telperion's `written_main` took 5.9 ms a dispatch, past the 2 ms bound.
- **Balance and allotment, 163.** The placing passes are about 85: 9.5 round-passes recorded a cycle at 9 dispatches each. Their slack is needed: over every preset at seeds 1, 7 and 42 the generations rose by 2 over two cycles and the passes by 1 (`raw/t6-gate-arena.txt`), each at its `STEP`. The sums' four sweeps are about 70: the wide levels a dispatch each, the narrow ones folded.
- **Light, 30, and layout, 32,** are below.

**Design calls, for the host (NEEDS_HUMAN):**
1. Record cycle c after cycle c-1's census rather than c-2's. Every projection's slack shrinks to one cycle's growth (the placing passes, the layout's generations, the units' ranges). It costs the overlap of recording with the device, about 90 ms of recording on Telperion, and a readback each cycle.
2. Units rounds from the rounds the last census used plus a slack, refused by name past it (task 4's form, without its restart).
3. A placing rule of one pass a generation (DESIGN.md section 3 restated). The partition is speed only; the words are unchanged.

## Light's dispatches a cycle, explained

Telperion at cycle 80, before: points 1, span 1, grid 1, deposits 6, densities 19, weights 1, paths 22, sweeps 65, combine 20, read 1, lit 1.

- **Cut:**
  - The densities take the whole lattice in one dispatch. They cost 0.13 ms over the oak's whole build.
  - The first two cycles read no census and recorded the bound's 224 sweep dispatches each. They now record 8.
  - The sweep's work bound goes from 64 to 256 nodes a thread a dispatch. At 64 the sweep took a dispatch a layer on Telperion's lattice: 1,654 of 1,660 were under 10 µs. At 256 a lit oak's sweep dispatch is about 1 ms.
- **Kept, at their bound a dispatch:** the paths (0.36 ms a dispatch on Telperion), the combine and the deposits.

Telperion's light is unlit (fn-264 owns its leaf area), yet `path_main` costs 220 ms of its 1,334 ms of stamped GPU time. That is outside this task.

## The layout, the scans and the folds

- **The layout** records a generation's flag, scan, gather, frame, carried-on frame and draw. The carried-on frame is new: it keeps the cold compile under the bound (below).
- **The scan** is at most 2 dispatches, from up to 5.
- **The sums' and retirement's narrow levels** are folded into one workgroup at each end of a sweep. Telperion's late cycles fold the top 3 levels and the deepest 3 to 8.

## Cold compile, native, driver cache off (`CYCLE_COMPILE=generator`)

| | Seconds |
|---|--:|
| Base (`raw/t6-cold-base.txt`), two runs | 2.19, 2.25 |
| Now, three runs (`raw/t6-cold-{1,2,3}.txt`) | 2.00, 2.04, 1.99 |

The base was over 2 s before this task. Its pole was the layout's `frame_main`: 1.28 s alone and 1.97 s beside every other pipeline. Each fused units entry is under 0.9 s alone (DESIGN.md 14). The frame's carried-on path (`stand`) moved into the gather, at 0.19 s alone. `every_pipeline_within_2_5_s_cold` passes.

## Compaction's cadence, measured

Warm build in ms, and compactions, by the projected share of a table that compacts it. Every preset grew with no move (`raw/t6-cadence-*.txt`).

| Threshold | Telperion | Laurelin | Spruce | Ordinary |
|--:|--:|--:|--:|--:|
| 0.5 (task 5) | 2,028 (7) | 1,895 (7) | 556 (3) | 543 (3) |
| 0.7 | 1,973 (4) | 1,825 (4) | 529 (1) | 559 (1) |
| **0.85** | **1,932 (3)** | **1,790 (3)** | 531 (1) | 529 (1) |

A compaction costs Telperion about 24 ms. The threshold is now 0.85, below the seven eighths at which a table would double. The gate passes: every preset at seeds 1, 7 and 42 makes no move, and the fullest table is the beech's phytomers at 0.85.

## The axes' own hint, and the oak at 80 — gate missed

- **The hint.** The axis tables are sized from `telperion_space::expected_axes` (test `the_expected_axes_count_the_buds_that_live`). Against it, the axes' most runs 0.35 (Telperion, seed 1) to 1.38 (the beech, seed 42; `raw/t6-ratios.txt`). Telperion's axis tables fall from 2.90M to 2.27M rows.
- **The oak at 80 still does not fit 4 GiB.** It is refused by name before cycle 1: its phytomers grown would take 4,424 MiB (`raw/t6-gate-oak80.txt`).
  - Grown from the least tables, by moves (`raw/t6-ratios-oak80.txt`, seed 1), it held at most:
    - axes 1.53M, 0.41 of its expected 3.77M;
    - phytomers 1.94M, 0.24 of its expected kept 8.04M;
    - records 8.12M, 1.01 of kept.
  - The shipped presets' most ratios size it at 5.65M axes and 22.96M phytomers.
  - The cause is the light. The closed form is light-neutral, and the lit oak keeps a quarter of what the form expects; the beech keeps 1.9 times.
  - One ratio a table cannot fit both. A hint that reads light, or a factor per species, is a design call (NEEDS_HUMAN).

## Warm build and peak per preset

Warm build in ms, the second build in the process (`build_time`); peak is the build's owned bytes in MB. F4, task 4 and task 5 are this file's earlier tables. Base and now were timed in this session (`raw/t6-build-time-final.txt`).

| Preset | F4 | Task 4 | Task 5 | Base | **Now** | Base peak | **Now peak** | Dispatches a cycle, base / now |
|---|--:|--:|--:|--:|--:|--:|--:|--:|
| ordinary 40 | 523 | 604 | 522 | 513 | **541** | 366 | **255** | 811 / 399 |
| oak 40 | 1159 | 525 | 405 | 385 | **336** | 1467 | **1120** | 755 / 362 |
| spruce 40 | 403 | 613 | 554 | 594 | **531** | 1645 | **2028** | 873 / 394 |
| beech 40 | 210 | 295 | 326 | 328 | **254** | 424 | **281** | 708 / 324 |
| birch 40 | 716 | 782 | 351 | 361 | **313** | 894 | **684** | 742 / 355 |
| palm 40 | 4 | 98 | 136 | 134 | **93** | 146 | **33** | 607 / 268 |
| Telperion 82 | 671 | 1909 | 1958 | 2034 | **2009** | 3990 | **3466** | 954 / 417 |
| Laurelin 80 | 627 | 1782 | 1869 | 1905 | **1881** | 3707 | **3176** | 934 / 410 |

- **Faster than the base:** the oak, spruce, beech, birch and palm, by 10 to 30%.
- **No faster:** Telperion and Laurelin are 1 to 2% faster, the ordinary 5% slower (noise).
- **Why the Two Trees did not speed up:** their build is GPU-bound. On Telperion:
  - the device's stamped time is 1,334 ms of a 1,709 ms span;
  - the light's paths take 220 ms, retirement's `living_main` 173 ms, and the units' `written_main` 137 ms;
  - the dispatches cut were mostly empty, and natively cost little;
  - the gain from them is Chrome's.
- **Against F4:** the Two Trees take three times F4's time.
- **Peaks** fall for every preset but the spruce. The spruce's axis tables now follow the beech's ratio.
- **The longest dispatch** is 1.98 ms (`written_main`, Telperion), under the 2 ms bound.

## Stills

Every preset's default age and seed, whole, mid and close, beside F4's (`raw/t6-stills/sheets`, every sheet viewed).
- Against task 4's renders, every image is pixel-identical, except the ordinary's: 8 to 50 pixels of 691,200 differ.
- So the differences from F4 are task 4's table to the tenth of a percent (ordinary 1.3/6.2/7.5%, ..., Laurelin 1.7/8.2/6.8%).
- Nothing moved visibly.

# fn-256 tasks 8 and 9: the two races, and the lattice dormant where nothing shades (worker, 2026-10-12)

COUNCIL.md round 5, steps 8 and 9. Measured on the RTX 3080 with 1.9 GB held by desktop processes, one GPU process at a time under the GPU lock. Raw output is in `raw/t8-*` and `raw/t9/`.

## Task 8: both races reproduced, red then green

Each test drives the real kernel's function through a probe entry added to its module (`race_tests.rs`).

| Race | Test | Red (`564be9296`) | Green (`8f2fe513c`) |
|---|---|---|---|
| `fault_out` ORs into the tally with a plain load and store | `every_fault_of_a_dispatch_reaches_the_tally`: seven axes of each workgroup fault at once, each its own bit | the tally kept one bit (`0x1`) in 16 of 16 dispatches | every bit, 16 of 16 |
| `node_laterals` takes an index and writes in the counted pass | `a_counted_lateral_writes_no_axis`: a beech's zone rows counted as the emitted round's `zones_main` counts them, axes filled with a sentinel | 2 axis words written, 2,189 laterals counted | none written |

- **The fix:** the layout's `lists` is `array<atomic<u32>>`, and `fault_out` uses `atomicOr`. A counted lateral (`what` 0) counts and goes on: it takes no index, reads no window and writes nothing.
- **What the race cost before:** the lost bits were fault bits only, read as a flag. The words the counted pass wrote were overwritten by `written_main`, so no node table moved (below).

## Task 9: the lattice dormant

The lattice sleeps where nothing shades: the extinction is zero, or no PA has leaf area under a sky whose open light is one (`build_light.rs`, `Light::dormant`).
- **Not recorded:** the densities, weights, paths, sweeps and combine (`plan_lay.rs`, `plan_lattice`).
- **Recorded as before:** the box, the grid, the deposit and the read, with every flag and refusal. The deposit stays for its refusals (a leaf's area, a node's quanta).
- **The read:** `read_main` sums each tip's eight rounded weights against a literal 1.0, the light the combine wrote on an unlit lattice. `a_dormant_lattice_reads_as_the_swept_one` holds every read word, and the refusals, to the swept lattice's: Telperion 14 (unlit, 316 tips) and the lit oak 8 with its leaf area zeroed (266 tips), 0 differ.
- **Which presets sleep:** every one but the oak and the ordinary, the two lit sites.

## The gate

**Node tables, byte-identical on every preset (met).** Each preset at its default age and seed, grown as a build grows it and laid, hashed (`dormant_tests.rs`, `node_tables_are_hashed`; `raw/t8-nodes-base.txt` before task 8, `raw/t8-nodes-after.txt` after both tasks). All 8 hashes agree, Telperion's 99.2M words and Laurelin's 93.0M included.

**GPU time (met).** Every dispatch stamped, a build warm, seed 42 (`the_roofline_is_measured`, against task 7's run in `raw/t7`):

| Preset | Before ms (m) | After ms (m) | Light before | Light after | Dispatches |
|---|--:|--:|--:|--:|---|
| Telperion 82 | 1,313.5 | **1,060.9** (−252.6) | 315.6 | 62.4 | 34,810 → 32,989 |
| Oak 40 (lit) | 222.0 | **222.9** (+0.4%, noise) | 23.4 | 23.6 | 14,492 → 14,492 |
| Beech 40 | 159.3 | 154.4 (−4.9) | 9.1 | 4.1 | 13,160 → 12,875 |

Every other phase is within 1% of task 7's figure; run-to-run variation is about 5%.
