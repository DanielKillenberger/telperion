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
