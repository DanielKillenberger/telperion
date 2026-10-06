# fn-209 F1 task 6: realized mechanics, the CPU reference executor (worker, 2026-10-07)

fn-211's host decision 3 (DESIGN-INTERLEAVE.md, "After Astra's review"): production uses realized mechanics, and option 4's committed boundaries leave production. This file covers the CPU reference executor (built, equal to today's engine to the bit), the committed path's removal, timing, and the one design question that stops the GPU mirror.

## Answer in brief

- **Built on the CPU and equal to today's whole-tree engine to the bit.**
  - The trees: beech and spruce at 40 and 80, seeds 1 and 7.
  - The cuts: 64, 256, 1,024, 4,096 and ∞.
  - Five runs per cut, in four order and chunking settings: forward in pieces of 16 (cold, then warm), reverse in pieces of 1, shuffled in pieces of 7, and forward in pieces of 64.
  - 40 configurations, 200 builds. Every digest equals `grow`'s, over every field in canonical (lineage) order.
- **No new arithmetic.** Every pass is today's own, run on a root's axes, and every sum a root's subtree enters is taken in the whole tree's order. So equality to the bit follows from the structure; it is not a tolerance.
- **The committed-boundary path is out of production.** It is kept as a named probe, `grow_committed` (`fine/committed.rs`), only because the GPU executor's tests still compare against it.
- **Speed: the partition beats today's engine.** It is bit-identical and 3 to 7 times faster on all cores (proposal below).
- **GPU mirror: not built. One design question first (D17, below).** Whole-tree shedding decides topology (`fade <= 0`) from presence-weighted f64 sums. On the fine path that decision now belongs to the executors, and the contract does not yet represent it.

## How it works (`grow/realized.rs`, `fine/realized.rs`, `fine.rs`)

1. **Above the cut.** The structure is grown by the engine's own `Grower` with a hand table: a lateral, or a sleeping bud as it wakes, whose PA and birth are fine is made where it stands and never grown. Its stand-in keeps its index among its bearer's children, so every sum at a bearer's node meets its children in today's order.
2. **Each fine root, phase 1.** The root is grown whole from its bud alone by the same grower (P2's `start`, with `alone` off), then marked and assigned. It is shed by its own realized subtree (`shed_given`). Up goes its `Living`: the subtree's last presence-weighted living time and its most present living apex.
3. **The structure above the cut** is shed on those values (`shed_given`), then scaled. Each root receives its bearer's base scale and its node's drawn scale.
4. **Phase 2.** The root is scaled and thickened; up goes its base radius. The structure above is then thickened on those radii (`thicken_given`) and laid.
5. **Phase 3.** Down comes the bearer's laid phytomer at the node. The root is laid (`place_from`, the stub fixed) and its levers gathered; up goes its base `Load`. The structure above gathers levers on those loads (`levers_given`) and lays again.
6. **Phase 4.** Down comes the bearer's bent phytomer. The root is laid again under its own levers, from its own first lay's frames. This resolves FRAME-CORRECTION.md: the bent pass uses the first-lay frame, as `place` does.
7. **Assembly.** Each kept root's first axis takes its stand-in's index; its other axes are appended.

**Round trips:** three (four item phases) for species that sag, two (three phases) for those that do not. Each trip is one record per root up and one bearer record down, never anything per node.

**Refused here:**
- species that read light (fn-211's stepped executor);
- species whose girth reads the tree as grown (`retained`, `leaf_girth`), whose disused pipes and leaf mean are tree-wide sums that fn-211 made order-free.

**Tests** (`fine/tests/realized.rs`, in the gate, about 35 s):
- every cut at age 30, and every order and chunking;
- fractional lifespans, with woken roots present;
- relays, dormancy, and roots shed by what they realized, on the unlit oak (`retained` 0) and the spruce;
- a budget refusal taken with the roots' growth counted.

**The 40/80 sweep:** `cargo run --profile ci -p telperion-space --example realized -- 40,80 1,7 64,256,1024,4096,inf`. Raw output is in `raw/realized/` (gitignored).

## Timing (the CPU executor against today's engine; Ryzen 9 5950X, 32 threads)

All-cores run: load 12 at the start, 9 at the end (`raw/realized/sweep2-all-cores.md`). The one-core run used `RAYON_NUM_THREADS=1` at load 9 to 6 (`sweep2-one-core.md`).
- "Rows" is the closed form's table for the predictor, which the cut reads. A cut of ∞ needs no table.
- Cold is the first build in the process; warm is the second.
- The fine figures exclude "Rows".

| Tree | Whole, all cores, cold / warm ms | Fine, all cores, cut 1,024, cold / warm ms | Fine, all cores, best cut | Rows, all cores, ms | Whole, 1 core, cold / warm ms | Fine, 1 core, cut 1,024, cold / warm ms | Fine, 1 core, cut ∞ |
|---|--:|--:|--:|--:|--:|--:|--:|
| beech 80 1 | 690 / 643 | 170 / 147 | 145 / 162 (4,096) | 68 | 1,437 / 1,285 | 821 / 937 | 883 / 872 |
| beech 80 7 | 1,524 / 1,492 | 203 / 205 | 190 / 192 (4,096) | 188 | – | – | – |
| spruce 80 1 | 3,221 / 3,202 | 957 / 881 | 764 / 773 (∞) | 142 | 7,379 / 7,090 | 6,162 / 6,808 | 5,710 / 5,987 |
| spruce 80 7 | 3,124 / 2,969 | 790 / 869 | 781 / 812 (∞) | 139 | – | – | – |
| spruce 40 1 | 1,268 / 1,066 | 237 / 233 | 209 / 229 (4,096) | 64 | 1,750 / 1,809 | 1,895 / 1,456 | 1,165 / 1,137 |

The phases' split, warm, spruce 80 at 1,024: growth 200 to 210 ms, girth 50, lay 113 to 116, bend 108 to 110. The rest is the structure above the cut's own settle.

### Proposal (owner asked for speed): today's `grow` through the partition

For every species that neither reads light nor keeps shed pipes, `grow` can run as this partition. The tree is unchanged to the bit, and the build is 3.4 to 7.3 times faster on all cores at age 80 (warm, cut 1,024):
- beech: 0.64 to 1.49 s down to 0.15 to 0.21 s, plus 0.07 to 0.19 s of rows;
- spruce: 3.0 to 3.2 s down to 0.77 to 0.88 s.

The gain is parallelism. Today's engine pays a fork and a merge per cycle and runs serial whole-tree passes (fades, shed, the vigour and base-scale loops); the partition grows and settles each root independently. On one core the partition is 10 to 35 % faster.

The best cut differs by species:
- the spruce is fastest at ∞, which needs no closed-form table;
- the beech is slowest at ∞ (1.1 to 2.1 s): its few huge roots serialise.

So the cut that minimises time is a tuning value per species, and it is a work placement only. Not built; for the host.

## Design question (host): D17, whole-tree shedding, on the GPU

**What stops the GPU mirror.** Realized mechanics makes shedding by fade (FINE-CONTRACT.md D17, `shed.rs`) a decision both executors take inside fine roots. An axis is dropped where `fade <= 0`. With no living apex in its subtree, that is `until <= age - delay - 1`, where `until` is the subtree's presence-weighted living time. It is f64 sums and products of presences: unit survival × share, node presences, birth presences and vigour. The contract makes none of them exact on the GPU, which takes presences in f32 (task 4).

Under option 4 this decision was D15, P2's committed extinction, which is integer. So it is new on the fine path.

**Measured** on today's engine: the decisions taken on the time term (`raw/realized/fade-probe.txt`; probe patch alongside, not committed).

| Tree | Taken | Exactly on the bound (whole presences, fade = 0, dropped) | With partial presences | Within 1e-4 of the bound | Nearest partial |
|---|--:|--:|--:|--:|--:|
| beech 40 1 / 7 | 5,497 / 8,823 | 602 / 1,061 | 56 / 95 | 0 / 0 | 2.2e-2 / 1.4e-2 |
| beech 80 1 / 7 | 81,575 / 137,981 | 8,390 / 14,512 | 1,068 / 1,834 | 0 / 0 | 2.1e-3 / 9.5e-4 |
| spruce 40 1 / 7 | 188,213 / 181,048 | 8,367 / 8,037 | 1,639 / 1,528 | 0 / 0 | 5.0e-3 / 2.2e-4 |
| spruce 80 1 / 7 | 606,027 / 602,123 | 27,707 / 27,203 | 5,406 / 5,450 | 0 / 1 | 1.3e-3 / 2.1e-5 |

What the table shows:
- About 5 % of the decisions sit exactly on the bound. That is whole presences, integer sums, exact in f32 too.
- About 1 % read partial presences.
- One decision (spruce 80, seed 7) sits 2.1e-5 from its bound. That is within reach of f32 presences' error (task 4 measured radii within 4.2e-6 relative).

**Options:**
- **A. Fixed-point living (extends host decision 1's contract).**
  - Every presence that enters `until`, `alive` or the child weight is computed as a Q32 integer on both executors: unit survival and persistence leads from the decision's own integer thresholds, through `fixed`'s `log2`; the node and D10 presences are already Q32.
  - The living sums and products are taken in a specified fixed point, and the comparison is exact.
  - It moves today's trees at rounding level, as decision 1 did, measured on the 28-tree dump and stills. The geometry may keep reading the f64 or f32 presences.
  - The largest option: every presence on that path is ported, which FINE-CONTRACT.md avoided for D18.
- **B. An integer shedding rule.** Drop where the subtree's whole-presence idle time has passed `delay + 1` (as `fade <= 0` reads it with every presence whole). The fade then only sizes what is kept, continuously, clamped at zero.
  - Exact on both executors from integer data, with no new arithmetic.
  - It changes today's topology: the axes now dropped only because partial presences pulled their living time under the bound (at most the "partial" column, 0.1 to 1 % of decisions) are kept at size zero or near it.
  - Host decision 2 (identities botanical; omitting thin wood is a draw-time level of detail) would then drop them from the drawing.
- **C. Emulated binary64 for the shedding chain.** Add, multiply and max are cheap to emulate, but the presences feeding them would also have to be bit-exact f64. That is option A's port plus the f64 transcendentals decision 1 rejected. Not recommended.
- **D. f32 shedding on the GPU with a measured flip rate.** That breaks R1's "identical topology", so it is listed only for completeness.

**Also for the GPU mirror, implementation rather than design:**
- The device pipeline is top-down by wave today. Realized mechanics adds three bottom-up passes across a root's waves (living, girth, load) and the three per-root exchanges with the CPU's settle.
- The ground routing (host decision 3) must be decided before the bearer's realized scale exists. An upper bound that ignores the bearer's fade (fade ≤ 1) keeps the rule conservative.

## Acceptance against the task

| Criterion | State |
|---|---|
| Beech and spruce, 40 and 80, seeds 1 and 7: topology by lineage and geometry bit-identical on the CPU reference | **met** (digest over every field) |
| The same at cuts 256, 1,024, 4,096, any order or batch | **met** on the CPU (and at 64 and ∞) |
| GPU geometry within 50 µm max, 10 µm RMS | **not met**: GPU mirror not built; stops at D17 |
| Committed-boundary path out of production, kept only where named | **met**: `grow_committed`, kept for the GPU tests |
| Timing warm and cold against the CPU on 1 and all cores; round trips counted | **met for the CPU executor** (`Fine::phases`, `Fine::trips`); the GPU's waits on its mirror |
| Workspace gate green | see the task's done summary |
