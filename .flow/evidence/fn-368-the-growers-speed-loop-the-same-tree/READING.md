# fn-368 R1: the reading, before any change

Base: `8660726b1` (fn-368's branch head). Profile: scoped timers in a probe patch built only into a separate binary (`raw/probe*.patch`, `raw/prof*.rs.src`, built to `target/prof`, never committed), one thread (`telperion-heavy … --threads 1`, so `RAYON_NUM_THREADS=1`), 14 Nov 2026 03:01 to 03:12, no watcher PAUSE inside any run quoted. Each timer costs about 40 ns a call, so the per-node rows below (16.4 M calls on the beech) carry up to about 2 s of their own; the per-cycle rows carry nothing measurable.

Grow seconds at one thread (the tree's own clock, `cycle: … s`): oak 40 1.39 s, spruce 40 6.98 s, beech 80 27.4 s, oak 120 49.6 s. The process's CPU seconds beyond that were the digest I added to `early` (8 s on the beech, FRICTION.md 03:10), now read apart.

## Where the seconds go (one thread, seconds)

| pass, and its parts | oak 40 | spruce 40 | beech 80 | oak 120 |
|---|---|---|---|---|
| **Units** (`grow_live`: every living apex's unit) | 0.30 | 1.13 | 7.36 | 10.83 |
| of which `node_wood`, per zone per unit | 0.03 | 0.15 | 0.53 | 0.93 |
| **Balance** tick (compact, open, retire, the balance) | 0.26 | 1.31 | 4.93 | 14.73 |
| of which `retire` | 0.16 | 0.72 | 2.89 | 9.37 |
| of which the evaluations (`retire::evaluate`) | 0.12 | 0.50 | 2.22 | 6.86 |
| of which `compact` | 0.03 | 0.20 | 0.43 | 1.96 |
| of which `open` | 0.02 | 0.10 | 0.29 | 0.85 |
| of which the balance (`totals`, laterals, walk, apply) | 0.04 | 0.24 | 1.12 | 2.05 |
| **Sketch** (draw the units, frame new axes, gather leaves) | 0.27 | 1.51 | 5.09 | 9.25 |
| of which drawing (`Layer::step` per phytomer) | 0.15 | 0.73 | 3.43 | 5.31 |
| of which framing new axes (`pencil`) | 0.07 | 0.33 | 0.57 | 2.31 |
| of which gathering leaves (`deposit`) | 0.03 | 0.36 | 0.82 | 1.11 |
| **Wood** (`girth::lay`) | 0.11 | 0.64 | 3.31 | 5.28 |
| of which the tree of axes built anew (`Tree::new`) | 0.01 | 0.10 | 0.13 | 0.52 |
| **ShadeField** (deposit quanta, sweep) | 0.08 | 0.47 | 1.79 | 2.69 |
| of which the deposit, 8 corners a leaf | 0.04 | 0.29 | 1.05 | 1.34 |
| **Allot** (inputs, split, order and sizes, surplus) | 0.04 | 0.22 | 1.23 | 2.60 |

The settle's passes (SagPlace, Place, Assign, Scale and the rest) run once, after growth, and are outside this spec (Boundaries).

## The six passes

**Units** (`parallel.rs`, `advance.rs`, `unit.rs`). Per living apex per year: one `Key` chain for the unit and its decisions (4 to 6 `mix` hashes), the survival (`contract::decide::shaded`, a contract power, `survival32`), the window (`Windows::window`, a table read), the relay and viability draws, the abortion (`decide::abortion`) and persistence draws. Per zone of the unit: `node_wood` (the expected wood, a sum over the lateral PAs of `Windows::wood`, an `exp_fixed` each, and over the sleeping PAs of `Windows::dormant`, an `exp_fixed` per year a bud may sleep, and `Windows::share`, one more), the zone's node draws and presences (`node_presences`, one `ln` a node). Per node: one `Growing` pushed (72 bytes) and its presence pushed, a place draw per bud place, and per bud made: shade kind draws, `lived` twice, `shed_certain`, a window, `bud_presence` (two thresholds, four `ln`), `place_presence` (two more `ln` where the place is a share), and a new `Line` (an axis with its own `Vec`s); per node with sleeping wood a dormant draw per place. Beech, one thread: 4.7 M apex-units, 16.4 M phytomers; per unit 0.37 µs before the unit, 0.06 µs after it, and per phytomer about 0.11 µs to push it, 0.04 µs for its buds and 0.03 µs for its sleepers. Allocations: each new axis's `Vec`s; the shares' lists are reused (`Spare`). `node_wood` depends only on the PA, the zone and the cycle, yet is computed for every unit of every apex: 10.5 M times on the oak at 120 for about 10 PA-zone pairs a cycle.

**Balance tick** (`grow.rs::step`, `cycle/passes.rs`, `working.rs`, `retire.rs`, `compact.rs`). Per year: `open` scans every held axis's mark from the lowest open one; `totals` and the laterals' fill visit each open axis (its children through `by_node`, a `Vec` per axis), one keyed `SHED` draw and one window per living lateral; the walk and its application are linear. `retire` evaluates each lateral whose subtree closed since the last year: 155 k evaluations on the oak at 40 and 7.14 M on the oak at 120, for 136 k and 4.65 M axes grown, so most are subtrees of one or a few axes. Each evaluation builds the subtree's index list (sorted), copies every axis in it as a settled `Axis` with every phytomer widened (`Growing::settled`) and a draws list of `Cow`s, runs the settle's `assign_living` on that copy (a pieces list, `cores::map`'s vector of vectors, an ends list, a presences list per axis), then `subtree_living` (the settle's `fades`: about six lists the length of the subtree, and a fade for every axis of which only the root's living is read). Oak 120: 0.89 µs an evaluation, of which the copy 0.49, the living 0.21, the list 0.06. Every input it reads is final (its subtree is closed); nothing is hashed or drawn. `compact` renames every store when a quarter of the axes are loose.

**Sketch** (`draw.rs`, `deposit.rs`). Per phytomer laid that year: its scale and rank, then `Layer::step` (the tropism, the carried rotation, the internode's keyed variation, the wander's two keyed draws, the bend's curvature, the pulls), its path pushed. Per new axis: its frame from its parent (`geometry::frame`, `taken`, `handed`) and its pencil (a `Layer` and a path `Vec`). Per year, where any PA keeps leaf past its year (spruce, beech), `deposit` allocates a list the length of every held axis and scans every one for its older leaf, the closed ones skipped by a test.

**Wood** (`girth.rs::lay`, `upward.rs`). Per year: the origins and parents of every held axis collected and a `Tree` (depths, child starts, children) built anew; then per open axis, deepest first, a node list resized and every held phytomer of it visited, its leaf (`leaf`, three products and a clamp) and its ring (`ring`, f64) each year, also for phytomers long past their leaf years, whose leaf is 0. Allocations: the tree, and one list per piece of 64 axes per depth.

**ShadeField** (`light/field.rs`). Per year: the lattice spanned over every leaf and tip, then per leaf its eight corners' quanta, each an f64 product and a `round`; the sweep over the lattice's nodes is small (93,808 nodes at most on the oak at 120).

**Allot** (`cycle/passes.rs::allot_one`, `working.rs`). Per year over the open axes: three lists sized to the open set, λ per open axis, the split (each axis's children twice, a kids list reused), `order` (a preorder stack), `sizes` (a filtered order list, the blocks' sums, then `sized`, which calls `drawn` again for every bud, two contract `log2`s and an `exp2`), and the surplus per living apex.

## Candidates, with the gain expected on paper (one thread, the tree unchanged)

| # | candidate | oak 40 | beech 80 | spruce 40 | oak 120 |
|---|---|---|---|---|---|
| 1 | The retirement's evaluation builds less: its per-evaluation lists and the copy's widened phytomers trimmed, `assign_living` and `fades` taken without their one-thread scaffolding (vector of vectors, a fade per axis unread) | 4 to 6% | 3 to 5% | 3 to 5% | 6 to 9% |
| 2 | `node_wood` computed once per PA, zone and cycle (a pure function of the three) instead of per unit | 2% | 2% | 2% | 2% |
| 3 | The leaves' gathering visits the open axes from the working set's list instead of every held axis, and allocates its list once | 0% (no lasting leaf) | 2 to 3% | 4 to 5% | 0% |
| 4 | The wood pass keeps its `Tree` and origins across years, extending it as axes are laid and renaming it at compaction | 1% | 0.5% | 1.5% | 1% |
| 5 | The wood pass skips `leaf` for phytomers past their leaf years (their leaf adds an exact 0) | under 1% | under 1% | under 1% | under 1% |
| 6 | `open` scans only from the axes that changed, not every mark | 1% | 1% | 1% | 2% |

No candidate reaches a tenth on the oak at 40 or the beech at 80 on paper: no pass dominates (the split already said so), and the largest bit-exact one is #1. Under R4 the loop ends at the first round that gains under a tenth on both, so the order is by expected gain: #1 first.

## Gains that would change the tree or the design (not built; for the host)

No gain that needs the tree to change was found of any size on these four trees. One needs a design decision:

- The evaluation of a closed subtree is the settle's own code run on a copy (`retire.rs`, by design: "evaluated once by the settle's own code"). A lean evaluation that reads the growing records directly would remove most of the copy (oak 120: up to 3.5 of 49.6 s, 7%; oak 40: about 7%) and need not change the tree, but it is a second implementation of the settle's assignment and fade beside the settle's, which the design rules out ("no copy of a stage outside it"). A question of design, not built.


Smaller exact ones left out of the table: `sizes` computes `drawn` twice per bud (once for the sums, once in `sized`), about 0.5% everywhere.
