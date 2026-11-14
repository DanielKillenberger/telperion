# fn-337: the two design questions, read from the code

The worker's findings for the host, 9 Nov 2026, at `fcbb6834f`. Nothing here is decided; each option lists what it costs and what it breaks. Marks: **read** (seen in the code at the named place), **measured** (a run's number, with its source), **inferred** (follows from what was read, not run).

## What the code does today

- **The bend is taken once, at the settle (read).** `Settling::settle_lay` (`grow/settle.rs`) scales the shed tree, thickens it (`girth::thicken`, final radii), lays it unbent (`geometry::place`), gathers each phytomer's lever on that unbent tree (`sag::levers` → `sag::gather`, one upward pass over every axis, base load from children last-first), then lays it again with the levers (`place(.., Some(&levers))`). In the second lay, `Layer::step` (`geometry/lay.rs`) applies tropism, wander and the bend memory first, then turns the running direction down by `sagged::sag_angle`: the lever carried from its unbent frame into the frame now (fn-203), the ground's share (`held`), and `sag::turn`, the closed-form large-deflection beam with curvature `sag * moment / r^4`, capped by the unbent moment.
- **Growth never reads it (read).** The year loop (`Grower::year`, `grow.rs`) is `step` (balance, allotment, growth, waking) then, in every year but the horizon's last (`sketching = cycle < request.age`, `paused.rs`), the rough layout (`sketch.rs`) and the light. The sketch lays each new unit from where its axis's walk stood, with sag left out, and everything it writes into phytomers (tip, frame, scale, rank) is written again at the settle.
- **No phytomer has a radius while the tree grows (read).** `Phytomer::radius` is made 0 (`grow/unit.rs`) and set only by `girth::thicken` at the settle. The radius depends on the age through `girth::own_at` (ripening), on presence at that age (`presence::assign`), on the leaf-girth mean and on retained pipes of shed branches (`girth/disused.rs`). The one exception is retirement: a closed subtree's girth is frozen as it closes (`retire.rs::frozen_at`), from `own_at` at the age it closed.
- **The grower holds wood the settle would shed (read, measured).** Shedding happens at the settle (`shed_past`); the grower's axes hold every unit not retired. fn-330's oak at 300 (seed 1, one thread, `raw/oak-300-after-fn333.log`): 3.89 M phytomers of history at the busiest cycle and 1.47 M at the end, against 1.18 M kept in the settled tree.
- **What the settle costs (measured, fn-330, the same run):** the whole settle at 300 is about 2.3 s of 210 s on one thread: Attachments 0.47, SagPlace 0.40, Place 0.38, Scale 0.27, Thicken 0.25, Levers 0.20, Shed 0.20, Assign 0.13. The year loop: Balance 92.8, Units 53.5, Sketch 35.2, ShadeField 12.0, Allot 9.9.
- **The device runs the same settle (read).** `telperion-render/src/generation/cycle/settle_run.rs` and `settle_place.rs` are production modules: the device grows the whole tree and settles it, with the levers gathered a depth at a time from the deepest and the turn in `settle_sag.wgsl` "in the CPU's f32 operations"; the year loop's layout is `lay_*.wgsl`, the balance `hier_*.wgsl`. A pause keeps the state buffers on the device (`build_pause.rs`).
- **The pins (read).** `paused/tests.rs::a_tree_paused_and_grown_on_is_the_tree_grown_at_once` (one call against splits, settles and empty advances, by `tree_digest`, and `Paused::digest` unmoved by a settle); `the_tree_at_a_year_is_the_horizons_prefix` (a run stopped at a year against one that goes on, and `Retiring::AtAge` against `Never`, the latter allowed to differ only in radii and the pose they bend, `girth_apart`); `a_tree_drawn_within_its_year_unfolds_from_the_last`. `Paused::digest` hashes axes, draws, live, asleep, woken, pencils, leaves and the working set.

## Question 1: how a year's load could be read

The rule needs, for each phytomer and each year it stood, the moment added that year and its radius that year.

**A. The settle's own passes, each year, on a copy (the drawn tree of that year).** Each year, `settled_copy` and the settle up to the levers (assign, attachments, shed, scale, thicken, place, levers), then the year's moment and radius per phytomer.
- Faithful: the load of year y is the tree as drawn at y, shed wood gone, radii as at y.
- Cost (inferred from the measured settle): the 300-year oak would run about 300 settles of a growing tree; at most 300 × 2.3 s on one thread, likely 150 to 400 s added to 210 s, plus a clone of the grower each year (`settled_copy` clones axes and draws; the census counts 12.6 M axes and 1.47 M phytomers held at the end of the run, retired axes' phytomers let go). The device would run its whole settle each year.
- Breaks: nothing pinned by itself; the cost likely breaks R6 and the oak at 300's room.

**B. One extra upward pass in the year loop, over the working set (the grower's own state).** The balance already runs an upward pass each year over the open axes (`Working::totals`), with closed subtrees standing by frozen sums (`Working::closed`) and retired roots by their frozen girth. A load pass could ride beside it: each open axis's phytomers walked as `sag::gather` walks them, on the sketch's positions, closed subtrees standing by a frozen load and moment, as their upkeep does now.
- Needs a radius per phytomer per year, which the year loop does not have. It would be computed in the same pass from `girth::own_at(form, cycle, age, scale, ..)`, as `retire.rs::frozen_at` does, but on presences not yet assigned (the settle's `assign` runs at the settle) and before the settle's shedding: the load would include wood the drawn tree of that year would shed.
- Positions: the sketch's, which leave out sag and take erection at the axis's age when the unit grew; and the horizon's last year has no sketch, so the pass must not hang on the sketch step, or `the_tree_at_a_year_is_the_horizons_prefix` breaks (a run stopped at y has not sketched y; a run going on has).
- Cost (inferred, not measured): a phytomer-level pass over the open set each year. The axis-level balance is 92.8 s of the 210 s; a phytomer walk with a three-vector moment is likely of that order or more, so +40 to +100 % at 300. Needs a measurement before it is chosen.
- Device: the same pass in WGSL in the year's recorded cycle (`hier_*.wgsl`), held to the CPU's to the ruled standard; a new stage on the device.

**C. Nothing read in the year loop; the history rebuilt at the settle from the finished tree.** Each phytomer records the cycle it grew (`Phytomer::cycle`), and `girth::own_at` gives a phytomer's own pipe at any age. At the settle, for each year y (or each year a phytomer stood), the load beyond each phytomer counts only the wood grown by y, and its radius is the pipe sum of what it bore by y.
- Exact sum: one upward pass per year over the settled tree, about 300 × (Levers 0.20 + Thicken 0.25 s) ≈ 135 s at 300 on one thread (inferred upper bound; the tree is smaller before the last years only in what was later shed, which this reading leaves out). A per-phytomer array over years instead would need about 1.2 M × 300 × 4 B ≈ 1.4 GB at 300, which rules that form out.
- What it leaves out: wood shed before the settle never loads anything (a limb that bore a big shed branch for decades bends as if it had not), and the positions are the finished tree's, so a year's lever has the final reach.
- Breaks: `what a year has bent stays` does not hold across ages: the tree at y+1 rebuilds years up to y from the tree at y+1, so a branch shed between them changes the earlier years' bends. The pause and resume pins hold trivially (a pure function of the grower's state). The device settle needs the same per-year passes on its levers.

**D. Which passes the cycle already runs could serve (read):** `Working::totals` (axis-level, scalar light and cost, upward, closed subtrees frozen); retirement's `frozen_at` (radius at closing, from `own_at`); the sketch (positions, no sag, no radius). None carries a per-phytomer moment or radius today.

## Question 2: where a phytomer's standing bend could be kept

**(i) On `Phytomer` (public, `structure.rs`).** A field holding the turn stood so far (a pivot and an angle, or a rotation, 16 bytes; or one angle about a pivot recomputed at the lay, 4 bytes).
- Memory: 1.47 M phytomers held at 300 × 16 B ≈ 24 MB, and the 3.9 M of history at the busiest cycle ≈ 62 MB (inferred).
- Breaks: every `Phytomer` literal (`grow/unit.rs` and tests); `tree_digest` hashes the Debug text, so every pinned digest changes even where sag is 0, which R2's "unchanged to the bit" would then have to read as geometry only; `girth_apart`'s bare comparison must zero it; the device's node table needs a new column (`PH_*` in the cycle WGSL), its readback and its comparisons; `lineage_order` and compaction carry it (a field on the record, so for free).
- The settle already rewrites tip and frame of every phytomer; this field would be the one it reads and does not rewrite.

**(ii) Beside the grower's state, not on the tree.** A per-axis vector of bends in the grower (beside `draws`, or in each `Pencil`), carried between years, copied in `settled_copy`, hashed in `Paused::digest`, read by the settle's lay in place of the levers.
- Breaks: no public type; `tree_digest` changes only where the geometry does, so sag 0 stays the same digest. `Paused::digest` must include it or the pause pins can pass while the state differs. Retirement lets a retired subtree's phytomers go (`retire.rs`), so its bends go with them and its root stands by frozen values; `compact.rs` must carry the vectors when it compacts axes. If kept in `Pencil`, it lives in the sketch, which is not made in the horizon's last year: the update must not ride the sketch step.
- Device: a per-phytomer buffer in the state the pause keeps (`build_pause.rs`), written by the year's new pass.

**(iii) Not kept.** Option C's reading: recomputed at the settle, nothing carried between years, no digest or pause change; the cost is C's.

## What the worker cannot settle from the code

- Whether the year's bend should also reach the sketch (the light and where relays are laid while the tree grows). Today growth never reads sag. Keeping it out keeps the year loop as it is; letting it in changes the light and the relays' angles, which fn-332 named as the path the droop compounds through.
- Whether the shed wood's load belongs in the rule (A and B carry it while it stands, C never does).
