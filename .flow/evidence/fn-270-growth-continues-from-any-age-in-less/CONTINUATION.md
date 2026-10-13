# fn-270 task 2: growth that continues, CPU then device (worker, 2026-10-13)

R2, R2a, R2a', R2b, R2c, R3. Raw output is in `raw/t2-*.log` (ignored).

## What was built

- **Retirement at the age grown is the only rule (R3).** `Retiring::AtHorizon` and `Working::continuable` are gone; the CPU reads a fade at `cycle - 1` and keeps a fading set (`retire.rs`). The device does the same: `plan_retire.rs` hands the verdict passes `cycle - 1` as their age, and `fading_main` (`retire_axis.wgsl`) retires a kept lateral once its fade reaches none, or marks it inside a subtree retired before or with it, unlogged, as the CPU's `let_go` does.
- **CPU API (R2):** `telperion_core::pipeline::growth::{Params, begin, grow_to, settle}`, through the engine's `Paused` and the one conversion (`space.rs`: `convert`, `scaled`, `foreseen`, `fits`). `telperion_space::{admitted, within_year, presented}` are the shared rules for fractional requests and the task-1 stand-in presentation (task 4 replaces it).
- **Device API (R2):** `generation::cycle::growth::GrowthState` (`begin`, `grow_to`, `settle`), crate-internal until task 5's consumers. The cycle loop is `Growing` (`build_grow.rs`), which carries its census, moves and compactions across calls; a pause readies the arena after its last cycle exactly as the next cycle of a one-call build would. `build_pause.rs` lets go, between calls: the light's lattice and points, the units' job counts, the hierarchy's buckets, sums and the lists its next step writes, the layout's lists, the compactor's scratch, retirement's verdicts (where no test logs them), and the recorded cycle. The settle reads the age grown (`Build::age`), the dress draws a fraction of the last year by the stand-in.
- **Failure (R2b):** non-finite, negative, backward and past-horizon ages, and a draw outside the last year, are refused by name before anything moves. A growth that fails (budget, capacity, device fault or loss) spends the state; every later call is refused as `state.failed`.
- **Admission (R2c):** the device state grows and draws under its own budget through the ledger, which charges every live buffer together (state, the drawn tree, settle scratch, a move's new tables beside its old). Nothing in the path lets the drawn tree go.
- **Friction (task 1):** `every_year_is_the_horizons_prefix` (the 24-year walk) is ignored and runs by name; the gate keeps a prefix check at years 12 and 23 and the horizon.

## Measured

| Check | Result |
|---|---|
| Device verdicts against the CPU's, every young preset (`retire_tests`) | Word for word; 9,310 of the spruce's 9,311 retirements at 18 come through the fading pass |
| Device 0->H == 0->a->H, young presets, least tables compacted every cycle (`a_tree_grown_on_on_the_device_is_the_tree_grown_at_once`) | 8 of 8 presets, a = 2, H/2, H-1, plus the one-call build; the spruce moves 4 times (after cycles 7, 8, 9, 11), one at the pause at 11 |
| Device 0->H == 0->a->H, every preset at its age, hinted tables (`presets_grow_on_on_the_device`) | 8 of 8, a = H/2 and H-1 |
| Zeroing the hierarchy lists the last step wrote, instead of the next's | 7 of 8 young presets differ: the test catches a state the next cycle reads |
| R2c, the lit oak to 12 year by year, its last tree drawn | Peak 57 MiB under a 1 GiB budget; under the live bytes + 1 MiB the next year is refused as "the device budget", the state spent, the drawn tree's words unchanged |

Device, per preset at its age (ledger, MiB; the drawn tree here is the dress only, not the renderer's whole tree):

| Preset (H) | One call: peak | Paused: held (state / retirement / hierarchy) | To a, drawn | Grown on beside it (a = H-1) | Scratch and cycle made again |
|---|--:|--:|--:|--:|--:|
| date-palm (40) | 30 | 4 (3 / 0 / 0) | 30 | 30 | 5 ms |
| european-beech (40) | 290 | 136 (96 / 30 / 8) | 290 | 398 | 7 ms |
| silver-birch (40) | 762 | 455 (324 / 97 / 32) | 762 | 901 | 14 ms |
| ordinary (40) | 261 | 132 (94 / 25 / 11) | 261 | 330 | 5 ms |
| oregon-white-oak (40) | 1,213 | 798 (569 / 170 / 58) | 1,213 | 1,371 | 14 ms |
| norway-spruce (40) | 2,331 | 1,603 (1,154 / 274 / 173) | 2,332 | 2,928 | 22 ms |
| laurelin (80) | 4,127 | 2,633 (1,884 / 523 / 225) | 4,111 | 5,152 | 32 ms |
| telperion (82) | 4,485 | 2,887 (2,066 / 570 / 249) | 4,476 | 5,603 | 40 ms |

"Made again" is host seconds over a run's two resumes (allocation and recording the cycle); the zero-fill runs on the queue inside the next cycle. Growing on beside a drawn tree peaks up to 25% over one call (Telperion 5,603 against 4,485 MiB), past the 4 GiB native default on Laurelin and Telperion: input to R4 (task 3). Tables are sized once from the horizon's hint, so a pause at H/2 holds the full tables.

## Open for the host

The hierarchy's shape, counts and current lists (Telperion 249 MiB) stay resident between calls, as do retirement's per-cycle columns inside `ret`; R2 asks the hierarchy partitions and verdict columns dropped and rebuilt. See the worker's NEEDS_HUMAN.
