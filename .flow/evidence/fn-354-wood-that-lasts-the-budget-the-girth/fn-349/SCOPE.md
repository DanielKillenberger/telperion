# fn-349 R1: the scope on paper

Drafted by the builder on 11 Nov 2026 at `f88a21c31`, from reading the code and `fn-346/probes/square.patch`. No code was written and the patch was not applied. Nothing here is a design. Section 7 holds the questions for the host.

Marks: **read** (seen in the code at this commit), **unchecked** (not run or not read through).

## 1. The balance

**Where it lives (read).** The CPU has two copies of the balance today:

- **The one cycle:** `grow/cycle/passes.rs::balance_one` builds `own(i)` (light, upkeep × `pencil.wood`, lit) and calls `Working::totals` (`grow/cycle/working.rs`). That sums each open axis's children in `by_node` order, and charges a closed child its frozen upkeep through `Working::closed` and `frozen`. The result goes to `contract/shed.rs::walk`.
- **The test-only whole-tree grower:** `grow/balance.rs` (`totals`, `laterals`, `shed_by`), reached through `grow::whole` and `Grower { one: false }`.

**What changes.**

- **`passes.rs`.** `own(i)` charges `size × (path / H)²` for a lit tip and 0 for anything else, where `size = base_scale × running` (the factor the light already uses). The `upkeep` closure goes. The `decides` test drops `link.node != TIP`, so only the seed's axis (no parent) is never tested. Letter M's second half is a separate question (Q6).
- **`working.rs::totals`.**
  - A closed child adds nothing.
  - An open child that is a lateral (`links[c].node != TIP`) adds its light and charge only where it pays (`light[c] >= cost[c]`, its sums already filtered).
  - A continuation or relay is always summed.
  - `living` is ORed in as today, whether the child pays or not, as the probe did.
- **`closed()`, `frozen` and the `upkeep` parameter of `totals` are deleted.**
- **`contract/shed.rs` is unchanged.** `balance`, `walk` and `Bud` read q and c as they do today.
- **`grow/cycle/compact.rs::gone`.** It lets an axis go only when its retired root has `!w.frozen[i].is_nan()`, which means "its upkeep has been taken into its bearer's sum, so nothing reads inside it again". With nothing charged for dead wood, nothing has to be taken first: the condition becomes `Fate::Root(_)` alone. `rename_working` drops `r.keep(&mut w.frozen)`. Doc comments to rewrite: `compact.rs` lines 1 to 12, `working.rs` lines 1 to 15, `retire.rs` lines 19 to 21.
- **`grow/cycle/retire.rs`.** Only its doc text changes. `Retirement::frozen` is the girth frozen as a subtree closes (fn-270 R4). It is a separate store and stays.
- **`footprint.rs`.** The `"working.frozen"` entry goes.

**Can the frozen sum and its bookkeeping be deleted?** Yes, on the CPU: `Working::frozen`, `closed()` and the frozen condition in `compact.rs`. Two things depend on Q1:

- `Pencil::wood` (`grow/sketch.rs`), written in `grow/draw.rs:385`, has no reader except the upkeep. It goes with it, along with its capture columns.
- That holds only if a species without H is charged nothing. If such a species keeps today's per-metre charge, `wood`, and the frozen sum for that species, have to stay.

**At risk.**

- **The test-only whole-tree reference.** It must carry the same rule (a second CPU copy of the rule, against R2's "no second path"), or be deleted with the tests that read it: `grow/tests.rs:233` and the ignored `cycle/tests.rs::agreement_with_todays_tree` (Q4).
- **Compaction timing.** It moves, because roots are let go a cycle sooner or later. The tree is held to be the same to the bit by `cycle/tests.rs::letting_go_of_retired_records_leaves_the_tree_as_it_was`, which should still hold.
- **The order of the remain rule.** It filters on the child's already-filtered sums, children before bearer, in the existing reverse-list order. That order is what the device must reproduce (section 4).

## 2. The path from the root

**Where the probe carries it (read).**

- **The store.** `geometry/lay.rs::Layer` gains `base_path` and a `cum: Vec<f32>` that holds one f32 per phytomer laid: the path to the end of that phytomer, pushed in `Layer::step` as `path_end + length`, where `length` is the internode length before any rest on the ground.
- **A new axis's start.** A new axis's `base_path` is taken where its pencil is made (`grow/draw.rs::pencil`, `.from_path(base_path(axis, ..))`):
  - for a lateral or relay, its bearer's `path_at(node)`;
  - for a continuation, its bearer's `path_end()`;
  - for the seed, 0.
- **The charge** reads `pencil.layer.path_end()`.
- **The final lay.** `Layer` is shared with the final lay (`geometry/place.rs`), so the probe also threaded a path argument through `lay()` and `place.rs`'s `Start`. SR does not need that: the charge reads only the rough layout's pencils.

**Does the product already hold anything to read it from (read)?**

- **No path is stored.** Neither `structure::Phytomer` nor `Pencil` nor `Layer` holds one.
- **Phytomer tips are held.** The sketch writes `Phytomer::tip` for every laid phytomer, and the grower keeps those tips across years. The settle works on a copy (`Paused`).
- **The path could be summed from the tips.** It would be the sum of `|tip_j − tip_{j−1}|` along each axis from its bearer's node, but it would not equal the probe's value wherever the lay eases or rests wood on the ground (`lay.rs` around lines 357 to 380). The probe sums the internode length it computed, not the distance between the tips.
- **The device reads nothing like it.** `dress.wgsl` names a "distance along the wood from the root" on the settled drawing's nodes, but that is the drawn tree after the settle, not the grower's state.

**Cost (arithmetic from SQUARE-RESULT.md).**

- **Memory.** One f32 per held phytomer: 1.74 to 2.48 M held at 300 under SR, about 7 to 10 MB, against a peak of 2.2 to 2.7 GiB. One f32 per pencil for the base.
- **Time.** One push per phytomer laid, and one lookup per new axis. The probe's measured cost pair (SR at +0 to +17% wall) includes the remain rule and the larger crowns at 80 and 120. The cost of the path alone is not separated (unchecked).
- **Where it should sit.** The cleaner place for the store is `Pencil`, not `Layer`. Then the final lay carries nothing, and `place.rs` and `lay()` stay untouched (Q3).

**Compaction and pause (read).**

- **Compaction.** `compact.rs::rename_axes` keeps `sketch.pencils` by axis with `r.keep`, so a held axis's pencil and its per-node path move with it. Bearers are renamed in `axes[].origin`, so `path_at(node)` through the bearer stays valid. `cum` is indexed by the phytomer's place within its own axis, which compaction does not change. Every ancestor of a living tip is open and never retired, so every path a charge or a new axis reads is held.
- **Pause.** `Paused` holds the `Grower` in memory between years and settles from a copy. A path in the pencil is therefore the grower's state at year y, and a paused-and-resumed grow carries it unchanged.
- **Tests that hold this.** `paused/tests.rs` and `cycle/tests.rs::the_working_set_and_retirement_leave_the_tree_as_it_was` should hold it with no new test (unchecked until run).
- **The device.** The device grower has its own pause and compaction (`build_pause.rs`, `compact.rs`/`compact.wgsl`), so a device path column must be added to both (section 4).

## 3. The species value

**Today (read).**

- **The field.** `PaState::upkeep` (`species.rs:234`) is per PA, and its neutral 0 means free.
- **The templates.** Oak 0.35 (`oak.rs:116`); beech, birch, palm, spruce and chain 0.
- **`tolerance.rs::respond`.** It copies the oak's upkeep, tolerance and leaf girth to every lit species as "shared physiology". It asserts that the oak states the row alike at every age, and its test (lines 77 and 81) pins `upkeep == 0.35`.
- **The shipped presets** are `crates/telperion-core/presets/*.values`:
  - `upkeep = 0.35` at every age in oregon-white-oak (10 rows), european-beech, laurelin, norway-spruce, silver-birch and telperion (9 rows each), and ordinary (15 rows);
  - none in date-palm.
- **The catalogue row.** `telperion-core/src/catalogue/group.rs:119` declares the row. It is generated into `src/browser/parameters.generated.ts:1270` (log scale, floor 0.001, range 0 to 10, neutral 0, unit "light per metre") and into `presets.generated.ts` (70 mentions).
- **The space-side scale.** `species/scale.rs` (the blend, lines 129 and 189), `species/scale/table.rs:135` (`Log { floor: 1e-3 }`), `species/scale/edges.rs:53` (range [0, 10]) and `species/validate.rs:63` (the range check).
- **`balance_hazard`'s meaning text** says "Dormant without upkeep" and has to be reworded.

**What changes.**

- A potential-height value replaces `upkeep` in every file above: its catalogue row, its scale-table row, its range in `edges.rs`, its validation, the seven preset files (with the source in a comment) and the regenerated TS.
- `tolerance.rs` stops copying `upkeep`. A potential height is a species trait, not shared physiology, so each species states its own. That is a change to the doc's stated premise, and the test at lines 77 and 81 changes.
- 21 hand-built `PaState` literals in tests (`upkeep: 0.0`) are renamed.

**Values available (fn-345 RESULT.md).**

| species | H | status |
|---|---|---|
| oak | 34.8 m | source checked against its text |
| beech | 40 m | a lower bound |
| birch | 30 m | the species is inferred |
| spruce | 61 m | a reported maximum, not a site potential |
| palm, ordinary, laurelin, telperion | none | none sourced |

**By degree, and blends between species (read; at risk).**

- **The problem.** The probe's form is `(L / H)²`, with H unset meaning dormant. Stored as H with 0 meaning unset, the value is not by degree: as H falls toward 0 the charge rises without bound, and at 0 it drops to nothing.
- **Where it shows.**
  - A blend from a species with H = 34.8 toward one without H passes through small H. The log-scale blend with a floor would carry it down to the floor, a charge in the hundreds, before the switch to free.
  - `tests/walks.rs`/`walk/mod.rs` walk `states[1].upkeep` and `states[2].upkeep` over 0 to 0.6 (lines 842 and 843) and hold the walk to change the tree by degree. A walk over H from 0 would break that.
- **Other forms.** Storing `1/H²` (or `1/H`) as the row keeps "0 is free" continuous and maps linearly onto the charge. A measured H would then not be what sits in the preset, which is what the spec's "a measured trait in the preset" asks for. This is the host's call (Q2).
- **Per PA or per species?** The charge reads `species.states[pa]`, so a per-PA row like `upkeep` stays the least change. A species-level scalar would need a new home in `Species` and in the scale table (Q2).

## 4. Outside the cycle

**Is there a second implementation of the balance?** Yes. The device grower in `crates/telperion-render/src/generation/cycle/` is a full second implementation in WGSL. The resident build draws from it (`resident.rs`, `resident/build.rs`, `growing.rs`), and the gate holds it to the CPU.

| place | what it holds today | what changes |
|---|---|---|
| `hier_sweep.wgsl::totals` | `Working::totals` and the frozen upkeep (`T_FROZEN`), own cost `row(pa, P_UPKEEP) × T_WOOD` | the charge per lit tip from a path column; the remain filter on child laterals; the frozen branch removed |
| `hier_sweep.wgsl::walk` | `decides` needs `T_NODE != NONE` | drop that condition (the seed only) |
| `hier_cycle.wgsl` | PA row slot `P_UPKEEP = 0` | the new value in that slot |
| `compact.rs`/`compact.wgsl` (`freeze_main`, `A_FROZEN`) | each retired root's upkeep frozen before its subtree goes | deleted, along with the flag's frozen condition |
| `glue.wgsl` (`P_WOOD` 54 into `T_WOOD`), `hier_cols.wgsl` (`T_WOOD`, `T_FROZEN`), `lay_draw.wgsl` (accumulates `P_WOOD`) | the wood in metres, read only by the upkeep | the slot repurposed or removed; the path end per pencil |
| `lay_walk.wgsl`/`lay_pencil.wgsl`/`lay_draw.wgsl` | the device's own rough layout | a per-phytomer path column, written as each phytomer is laid, and a base path read at a new pencil's framing |
| `state.rs` (`PENCILS` = 63 words), the phytomer tables, `build_pause.rs`, `compact.rs` column lists | layouts and the remapped columns | the new column in each |
| `hier_compare.rs:123` ("upkeep at the open axes", `S_COST`), `hier_columns.rs:83` | the device's cost held to the CPU's | renamed; follows the change |
| `examples/gpucycle/{check.rs,bufs.rs,sum.wgsl,flat.wgsl}` | a spike that reads `pa_upkeep` and `wood` from a capture | updated or deleted (it builds under `cargo test`) |

**The CPU's captures for the device (read).**

- `capture.rs:169` (`wood`) and `:176` (`pa_upkeep`);
- `capture_grown.rs:53` and `:69` (the PA constants);
- `capture_hierarchy.rs:118` (`wood` bits) and `:144` to `:151`;
- `capture_layout.rs:77` (`wood` bits).

Each of these gains the path and the new value, and loses the wood and the upkeep.

**The tests that hold the CPU's tree to the device's (read; 69 `#[test]` across `cycle/*tests.rs`).**

- `grown_tests.rs`: every shipped preset (8), young (8 to 12 years), device against CPU by lineage, to the ruled standard;
- `hier_tests.rs`: the hierarchy passes, including the cost;
- `retire_tests.rs`, `settle_*_tests.rs`, `mirror_tests.rs`, `grown_det_tests.rs`, `grown_life_tests.rs`;
- `resident/agree_tests.rs`.

All of these run in the gate on the card (`.config/nextest.toml`, test group `device`). A CPU-only change fails them, so the device is inside this change.

**Everything else.**

- **The Wasm bindings** (`crates/telperion-wasm`) do not name `upkeep` (grep). They pass the family through, so the row follows the catalogue.
- **The browser source under `src/`** carries it only through the two generated TS files.
- **`closed_form`** names `upkeep` only in test literals (`closed_form/tests.rs:72`, `kept/tests.rs:54`). It does not compute a balance (grep).
- **The Jev crate** does not name it.
- **`examples/space/tuning.rs:78`** (render) sets `(upkeep, balance_hazard, tolerance)` from tuning arguments and changes with the row.
- **The docs.** `docs/tree-space.md` names the balance hazard; no doc names `upkeep` (grep). The fn-197 and fn-255 design notes under `.flow/` describe today's rule and are history.

## 5. The tests and baselines that will move

**The literal pins (read).** Few were found. No test in the space or render crates pins a grown tree's hash or count as a literal: the 16-digit hex in `dormant_tests.rs:21` is an FNV seed. The tests are mostly relational.

**Pins that move by rename or by rerunning:**

- `tolerance.rs:77` and `:81`;
- the catalogue pins (`telperion-core/src/suite/catalogue/pins.rs`) and the generated tables, together (memory: "A new catalogue row moves the wire tests and the generated tables together");
- the 21 `upkeep: 0.0` literals;
- `walk/mod.rs:779` (`leafy` sets `upkeep = 0.3`) and `:842`/`:843`, the walk rows.

**Relational tests that hold if both sides change alike:**

- one cycle against the whole-tree reference (`grow/tests.rs`, `grow/cycle/tests.rs`);
- paused against one call (`paused/tests.rs`);
- compaction leaving the tree to the bit;
- device against CPU (69).

These need thought, not rerunning: they pass only if the whole-tree reference and the device carry the same rule bit for bit (Q4, section 4).

**Banded measures on a grown tree (unchecked).** Height, leaf and counts on the oak, beech, birch and spruce:

- `tests/light.rs`, `death.rs`, `form.rs`, `sag.rs`, `dormant.rs`;
- the core's `cycle_cost_tests.rs`;
- `species:qa` and the numeric protocol.

SR changes the oak by +1% leaf at 40 and +8% at 80. It changes beech, birch and spruce from free (0 upkeep in their templates, but 0.35 in the `.values`) to the square charge, "within 0 to 9%" (SQUARE-RESULT.md). Whether any band is crossed is found by the first gate, not by reading. My guess is 0 to 5 such tests, each needing a look at the measure.

**New tests (R4), three:**

- dead twigs charged nothing;
- the charge's form on a hand-built tree;
- a paying and a failing child.

These are about 120 lines in `grow/cycle/tests.rs` or a new `cycle/balance_tests.rs`.

## 6. The size of it

**Files and lines (estimate):**

- CPU balance and path: `passes.rs`, `working.rs`, `compact.rs`, `sketch.rs`, `draw.rs`, `footprint.rs`, `balance.rs` (or its deletion). About 150 to 250 lines changed, and about 60 lines deleted (`closed`, `frozen`, `wood`).
- The species value: `species.rs`, `scale.rs`, `table.rs`, `edges.rs`, `validate.rs`, `tolerance.rs`, five templates, the catalogue `group.rs`, seven `.values` files, two generated TS files and 21 test literals. About 150 lines, mostly mechanical.
- Captures: about 40 lines.
- Device: `hier_sweep.wgsl`, `hier_cycle.wgsl`, `glue.wgsl`, `hier_cols.wgsl`, `lay_*.wgsl`, `compact.{rs,wgsl}`, `state.rs`, `build_pause.rs`, `hier_compare.rs`, `hier_columns.rs` and the gpucycle example. About 200 to 400 lines, and the hardest part, because it must agree with the CPU to the ruled standard.
- Tests: about 150 lines new, plus the moved ones.

**Build order:**

1. The species value and its catalogue row, with H read but no charge yet. This is a compile-only step, and every relational test should still hold.
2. The path in `Pencil`, carried and unused. Test: it survives compaction and a pause.
3. The CPU rule in `passes.rs` and `working.rs`, with the frozen sum deleted, the whole-tree reference matched or deleted, and the R4 tests.
4. The captures.
5. The device, in four pieces: the path column, `totals`, `walk`, then dropping the freeze.
6. R5: the census at 40, 120 and 300 on seeds 1001 and 1007 against SR's table, under `telperion-heavy`.
7. The gate, then the codex review in the queue.

**Times.**

- **Narrow tests.** The first build of the space crate's tests in this empty `target/` costs a few minutes (unchecked). Later narrow runs take under a minute each. Device narrow tests need the card and the heavy queue.
- **R5.** SR's grow to 300 took 131 s on one thread per seed. With the census, two seeds take about 10 to 15 minutes in the queue.
- **The gate.** The first run in this worktree runs every suite: about 30 minutes as fn-348 measured, plus about 3 minutes of build. A rerun after a fix runs the failed tests first.

**What could not be finished inside three hours.** Steps 1 to 4, the R4 tests and a CPU-only R5 fit in about 2 to 2.5 hours. The device implementation held to the CPU (step 5) is about 2 to 3 hours more on its own. The gate and review then add about 1 hour. The whole is about 5 to 6.5 hours. If the device is in scope, it is the part that would not finish in three hours.

## 7. Questions for the host

1. **A species without a potential height.** Spec R3 says such a species "is not charged". R1-ANSWERS.md said it "runs today's charge with dead wood not charged", and the probe ran that (`None => upkeep(i)`). The two differ for ordinary, laurelin and telperion, which carry `upkeep = 0.35` in their `.values` today and have no sourced H. They also differ for whether `Pencil::wood` and the frozen sum can be deleted. Which holds?
2. **The stored form and its home.** H itself, with a separate neutral, is not by degree near 0, and a blend toward an unset species passes through small H. Is the row H, `1/H`, or `1/H²`? Is it a per-PA row like `upkeep`, or a species-level value? And what is its range for validation?
3. **Where the path is carried.**
   - As the probe did, one f32 per laid phytomer, but in the rough layout's `Pencil` rather than in `geometry::Layer`, so the final lay carries nothing.
   - Or summed from phytomer tips, which differs where wood rests on the ground.
   - On the device, a new phytomer column either way?
4. **The test-only whole-tree grower** (`grow/balance.rs`, `grow::whole`). Does it carry the same rule, or is it deleted with the tests that read it?
5. **The device grower.** It is a second implementation of the balance, held to the CPU by the gate's device tests on all eight presets. Is it in this spec, and in this three-hour build?
6. **Letter M's other half.** SR ran with M, which also sets a balance of 0 (today's is 1) where a living lateral's light and charge are both nothing. R2 does not name it. Is it part of the rule?
7. **Ages that differ.** `tolerance.rs` asserts that the oak states its shared rows alike at every age. Does the potential height leave `respond` entirely, so each species states its own, or is it copied from the oak where a species has none?

## Friction

None slowed this R1. The probe's `probe_ablate.rs` is not in this worktree; it was read from the patch.
