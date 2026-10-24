# fn-255 R3b: what moved to the one cycle, and what is deleted with fn-256 (worker, 2026-10-11)

The host's instruction: run R3b's deletions as far as they do not break the render gate's CPU-against-GPU checks, and list the rest. Those checks (`preset_agree_tests`, `realized_tests`, `route_tests`, `birch_tests` and the others under `generation/fine`) compare the device's fine roots against `telperion_space::grow_roots`, which is the stepped executor. Whatever that reference and the device path read stays until fn-256's executor replaces both.

## Done in fn-255

- **The cycle's f32 deposit option is deleted.** The cycle deposits integer quanta only, as the host adopted. The comparison that decided it was the council's early test 2 (archived; summarised in the spec). The census no longer reads a second lattice, and the early example's deposit mode is gone.
- **The refused light candidate** was already out of the crate (`candidate/`, evidence only).

**Tried and reverted: routing `telperion_space::grow` through the one cycle, and deleting `grow_marked`** (2026-10-11).

- The render gate's CPU-against-GPU checks compare the renderer's resident build with the CPU path. They therefore require the CPU build to grow the same tree as the device's fine roots.
- The routing failed 14 of them (`resident_tests`, `route_tests`, `stations_tests`, `placed_tests`, `ground_tests`, `blossom_parity`). It also failed 6 space and core tests that hold `grow` to the stepped and whole-tree references (`couple`, `oracle`, `angles`, `internodes`, `light`, `folds`).
- So the CPU build moves to the one cycle together with the device path, in fn-256. `grow` is still F4's stepped build.

## Deleted with fn-256 (the device executor replaces what still reads them)

| What | Who still reads it | Why it waits |
|---|---|---|
| `telperion_space::grow` and `grow_marked` through the stepped executor, the fine-root marks in the CPU conversion | The render gate's resident-against-CPU checks, and the space tests holding `grow` to the stepped and whole-tree references | The CPU build and the device's roots must move to the one cycle together |
| `Species::reads_light` | `grow::start`'s rough-layout gate, `grow::apart`, `alone::rules`' refusal, `fine/coarse.rs::grow_coarse`'s lit route, `closed_form::expected_kept` | Each is a consumer listed below |
| `grow::apart`, and `keeps` as a gate | The renderer's executor choice (`pipeline::executor`), `grow_coarse` | It selects the device's fine roots, which fn-256 replaces |
| `lasting`, `Certain` | The stepped executor and `alone` growers | Both are the render gate's CPU references |
| The sketch, balance and allot gates (`start`, `balances()`, `allots()`), and the grower's and rules' `one` flags | The stepped executor, the whole-tree engine (`whole`, test reference), `sketch()` | Shared code; the flags go when the old growers do |
| The f64 survival path in `advance` (`powf_fixed`, `survival`, `neg_log2_of`) | The stepped executor and `alone`, whose survival the device's fine-root tables mirror in F4 | The render gate checks those two against each other |
| `grow/alone.rs`, the stepped executor (handover, per-root settle), `settle_with` against `lit`, the fine-root cut and its 1,024 | `grow_roots` (the render gate's reference), the renderer's resident build | fn-256's executor replaces the device roots and their references together |
| `chain.wgsl`, `contract.wgsl`, `wide.wgsl` | The renderer's fine-root pipelines | fn-256 |
| The f32 lattice deposit `Field::on` | The stepped executor's shade | It goes with the stepped executor |
| `grow_coarse` (the site's coarse field call) through the one cycle | `telperion-field` | The site grows on the device (owner); fn-256 and fn-257 |
| The closed-form predictors | `expected_kept` (core's ceiling foresight, the render budget), the coarse field | Deleted once no consumer remains (COUNCIL.md); fn-257 |
| The fine-root marks in the conversion (`convert`'s `fine` and `bases`) | The renderer's resident build (`pipeline::space::part`) | It goes with the device's roots |

## For fn-256's device code, from fn-255's review (round 1)

- **Zero light:** the spike's `fate.wgsl` shades viability as `exp2(log2 v · exp2(−φ log2 l))`, which takes `log2` of zero light. The CPU contract (`decide::shaded`) now gives `v^∞` explicitly: none, or certain where `v` is one. The device must do the same.
- **Waking and retirement:** a woken bud's kept share is final only three cycles after its waking: its bearer's next unit, then the balance that may shed that bearer one cycle back. Any device retirement must wait as `retire.rs` does.

## Not changed (owner or host decisions)

- **Shed wood's upkeep is charged as today.** That is the model question in CYCLE-COST.md, which the host will spec.
- **Every species' light rows and leaf area** belong to fn-264.
