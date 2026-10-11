# fn-256 task 2: the device cycle executor, its design (worker, 2026-10-11)

This design ports fn-255's one cycle (`grow/cycle.rs`, `Grower::year`, then `Grower::settle`) to the device. The COUNCIL's decisions hold throughout: bounded subtree blocks, one reduction order, the integer deposit, the one-workgroup-per-direction sweep, the f32 viability, and agreement measured by lineage. This file adds what COUNCIL.md and SPIKE.md left open: the buffers, each cycle's dispatch graph, the block bookkeeping as the tree grows, and how ordered child lists are kept on the device.

Task 2 builds the executor test-only. Task 3 routes production through it.

## 0. The governing rule: every append is a scan, never an atomic

The node table must be byte-identical over fresh contexts and block sizes (R2, R2a). Every record the executor creates therefore gets its index from an exclusive prefix sum over its producers, taken in the CPU's order. No index ever comes from an `atomicAdd` slot.

- **Atomics are allowed only where their result is order-free.** That means the integer deposit, integer counts, and min/max folds.
- **The CPU's order** is the order `Grower` appends in:
  - Units go in live order. Each apex's records follow in its emission order (`grow_live`, `absorb`, which is equivalent to one sequential pass).
  - Wake goes in `asleep[cycle]` order, after the merge.
  - New axes are numbered after the cycle's existing ones, as on the CPU, so an axis index on the device equals the CPU's axis index. Agreement can then be checked by index as well as by lineage.

## 1. Buffers

Everything is resident, one storage buffer per column (SoA).

- **Grouped, as built (2026-10-11, CYCLE.md).** WebGPU allows 8 storage bindings a stage, and this adapter's largest binding is 2 GiB. So the columns are grouped into a few buffers, each column contiguous within its buffer, and the phytomer columns are split across two buffers. The rule that matters holds: no record-per-phytomer buffer.
- **Why one column per field:** a column of 18.2M phytomers × 4 B is 73 MB, under WebGPU's default 128 MB binding limit. One record-per-phytomer buffer would not be.
- **Capacity:** each arena has a capacity and a count. Section 5 says how the capacities are set and grown.

### 1.1 Axes (one row per axis, CPU index)

| Column | Type | From |
|---|---|---|
| `lineage` | 2×u32 | `Axis.lineage` |
| `pa`, `birth` | u32 | `Axis` |
| `origin` | u32 | Packed: kind 2 bits, slot 8 bits, woken 1 bit. |
| `parent` | u32 | Parent axis. NONE for the seed. |
| `node` | u32 | Ordinal node on the parent; `TIP` for a continuation; the relay's node. |
| `node_phyt` | u32 | Global phytomer index of `node`. It is set at sprout, so a lateral's frame never searches for it. |
| `whorl` | f32 | |
| `apex_end` | u32 | `+1`, 0 = none |
| `root`, `horizon` | 2×u32, f32 | `Grower.roots`, `.horizon` |
| `draws` | 16×u32 | The scalar `Draws`: `birth[2]` f32, flags, `below` (2×u32), `ended_relays`, `aged`, `sleep`, `outlived`, `living`. The same word map as `chain.wgsl`'s AXW where the fields coincide. |
| `units_head`, `units_n` | u32 | Last unit record (list linked backwards) and count |
| `phyt_n` | u32 | Phytomers on the axis (the next node ordinal) |
| `spent` | f64 as 2×u32 ticks | `units[a]`, held as integer ticks. It is on the tick grid, so this is exact. |
| `succ_head`, `succ_tail`, `succ_next` | u32 | `successor[a]` as a linked list in push order |
| `depth`, `block` | u32 | Section 3 |

### 1.2 Phytomers (an append arena, in creation order)

| Column | Type | Note |
|---|---|---|
| `key` | 2×u32 | `Phytomer.key` |
| `axis`, `cycle` | u32 | |
| `node_presence` | f32 | `Draws.nodes[j]` |
| `tip`, `heading`, `side` | 3×f32 each | Written by the layout |
| `scale`, `grown`, `size`, `rank` | f32 | The sketch's bare values. The settle rewrites them. |

**An axis's phytomers are not contiguous across cycles.** Each unit's phytomers are contiguous, and the unit record holds `(first, count)`. The settle compacts each kept axis's phytomers into one run (section 2.4).

### 1.3 Units (an append arena)

`survive`, `persist`, `share`, `size` (`presence::Unit`), `shade[2]` (`Draws.shade`), `first_phyt`, `count`, `prev` (the axis's previous unit).

### 1.4 Pencils (one row per laid axis)

`Layer` (its f32 fields; `inherited: Vec<Lift>` as a bounded inline array, bound checked and refused by name), `base_scale`, `running`, `rank`, `units`, `laid` (the next unit to lay), `trunk`, `light`, `size`, `wood`, `memory`, `kept`.

### 1.5 Sleepers, woken buds, live list

- **Sleepers:** `asleep` as one arena with a `wake` cycle column. Each cycle's own list is the stable compaction, in creation order, of the rows whose `wake` equals the cycle: a flag, a scan, a scatter.
- **`Sleeper` and `Woken`:** their fields as in `dormant.rs`.
- **Live list:** double-buffered `(axis, spent ticks)`.

### 1.6 Hierarchy and light scratch

- **Hierarchy:** the CSR of children (section 4), block lists (section 3), and the spike's per-axis sums: `light`, `cost`, `living`, `sub`, `incoming`, `bud`.
- **Light:** the spike's grid box, quanta, density, lattice and reads (`light.wgsl`).

### 1.7 Tables (uploaded once per build)

- **Species:** `chains::packed`'s unit tables (the zone and node-law tables `zones.wgsl` reads), the PA rows, and the `Windows` (`window`, `wood`, `sleeping`) tabulated per (pa, zone, cycle) on the host.
- **Why on the host:** these are the f64 static-per-species values the CPU takes from `exp_fixed` sums. They are computed once by the same Rust code, so the device reads the CPU's own numbers.

## 2. One cycle on the device

The order is fn-255's, as COUNCIL.md has it. Each box is one dispatch or a short fixed chain of them. `[n]` marks an indirect dispatch whose count a scan on the device wrote.

```
cycle c:
  1. balance        own values [axes] -> totals by block level (B) -> walk (shed) by block level
                    -> memory, kept, shed apexes leave the live list (flag, scan, compact)
  2. allot          lit presence [live] -> split by block level (top-down) -> normaliser blocks
                    (1,024 growing buds in subtree order, summed in order, blocks in order) -> sizes
  3. units          count [live] -> scan -> capacity check -> emit [live]
  4. wake           compact asleep[c] -> count [sleepers] -> scan -> emit [sleepers]
  5. children       new axes: depth, block, CSR merge (section 4)
  6. retire         open/closed marks; evaluate closed subtrees (section 6)
  7. layout         round 0: draw_bent [grown axes]; then per generation g = 0..G:
                    frame [new axes of g] -> draw_bent [new axes of g]
  8. light          (c < age) box -> deposit [phytomers laid in c] -> densities
                    -> sweep (one workgroup a direction) -> combine -> read [live]
then the settle (section 2.4).
```

### 2.1 Units (step 3)

- **One invocation per living apex**, running `advance` as `grow_live` does. That includes `carry_on`'s recursion inside the cycle and a woken bud's first unit.
- **The function is one WGSL body in two modes:**
  - **Count** writes the apex's output counts: axes, phytomers, units, sleepers, next-live entries and grown nodes.
  - **Emit** recomputes the same draws and writes the records at the scanned offsets.
  - Draws are keyed by lineage, so the two modes agree by construction.
- **What emit costs and buys:** it runs the decisions twice. It needs no per-apex scratch region, no overflow-and-retry, and no queue, which are the sources of fn-261's renumbering. Section 7 measures what running twice costs.
- **Shared code:** the decision code is the existing contract: `wide.wgsl`, `lineage.wgsl`, `contract.wgsl`, `zones.wgsl`, and the unit tables. `apex.wgsl` and `sleep.wgsl` are rewritten from chain-local state to the arenas. Per AGENTS.md this is a rewrite, not a copy.
- **The budget** (`grown + n > budget`) is checked on the scanned total and named on readback, as `Error::Budget`.

### 2.2 Wake (step 4)

- **One invocation per sleeper**, in `asleep[c]` order, in the same count and emit modes.
- **`dormant::carried`** walks bearers' unit lists and successor lists as they stand after step 3. It reads nothing another sleeper writes. So the serial `Shoot` is a prefix sum over sleepers.

### 2.3 Layout (step 7)

- **Round 0:** one invocation per axis that grew. It runs `draw_bent` over its unlaid units, in order, writing its phytomers' tips and its pencil.
- **New axes:** they are framed by generation (`sketch_round`). Generation is set at birth: 0 if the parent existed at the cycle's start, otherwise the parent's generation plus one. Each generation is one `frame` dispatch then one `draw_bent` dispatch, each over a compacted list of that generation.
- **Shared code:** `Layer::step` is `lay.wgsl`; the lateral frame is `walk.wgsl`; the relay's stand is `stand.wgsl`; `bend` is `resolve.wgsl`. All are rewritten to the arenas.
- **Leaves:** the cycle's leaves are exactly the phytomers laid in it. The deposit takes them by range and is order-free, so `sketch_end`'s list order does not matter.

### 2.4 The settle (once per build)

The passes are the CPU's, in its order: mark, assign, attachments (leaf mean), shed, disused, scale, thicken, place, levers and sag place, then `lineage_order`. They run on the device over the whole tree, by depth level.

- **Reused:** the fine pipeline's kernels (`fades.wgsl`, `assign.wgsl`, `carry.wgsl`, `girth.wgsl`, `settle.wgsl`, `lay.wgsl`), rewritten from per-chain loops to per-axis passes by depth.
- **The leaf mean:** it is a whole-tree f32 sum in axis and phytomer order. It is taken in the one reduction order: blocks of 1,024 phytomers in that order, each block summed in order, the blocks then added in order. This is a change to the CPU's sequential sum, which the CPU takes with it (section 8, open item 2).
- **`lineage_order`:** a key per axis (`lineage`, `identity`), then a per-parent sort of children by key (children per parent: p99 29 to 54, at most 234). The BFS is a scan by depth.
- **The node table** is the settled, lineage-ordered tree's columns. It is what R2a holds byte-identical.

## 3. Blocks as the tree grows

Bounded subtrees, as COUNCIL.md decided: a block is a connected set of at most `cap` axes, summed by one workgroup with a barrier between local depths. Dispatch boundaries are block levels.

- **An axis joins a block at birth and never moves.** It joins its parent's block if that block holds fewer than `cap` axes.
  - The seed opens block 0 at level 0.
  - Block membership is an append-only list per block. The assignment is a scan over the cycle's new axes in index order, so it is the same on every run.
- **Overflow (revised 2026-10-11, after HIERARCHY.md).** A first rule gave each overflowing child a block of its own, which made about 9 axes a block on Telperion (166,539 blocks).
  - Now each block keeps one *open spill*. A child whose parent's block is full joins the open spill of that block, filled in index order up to `cap`. The spill's level is the full block's level plus one.
  - When an open spill fills, the full block opens a fresh spill at that same level. The spills of a block are siblings, never a chain.
  - A spill is an ordinary block: its own members' overflow goes to its own spills.
  - So levels stay bounded by the tree's depth (each block spans at least one depth).
  - A first wording that chained each spill to the last made every block its own level (1,430 levels on Telperion; `raw/hier-spill-probe-1.txt`).
  - So a block need not be connected. A block is any set of axes whose children outside it sit in blocks of deeper levels, which the rule keeps.
- **Why the block shape is free:** each axis computes its own sum from its children's finished sums, in the CSR's order. Any schedule that finishes children first gives the same bits; the spike proved this at caps 256 to 4,096. So the partition is a matter of speed, never of the tree.
- **Inside a block:** each axis stores its depth, fixed at birth, and each block keeps the least and greatest depth of its members (min and max, which are order-free). A workgroup loops the block's depths deepest first (bottom-up) or shallowest first (top-down), each thread taking the block's members at that depth. A spill's later members may be shallower than its first ones; the loop's range takes them.
- **Levels per cycle:** block levels only grow. Each level is one dispatch over the blocks of that level, listed by a scan when a block is opened.

**The whole tree, not the working set.** The device's balance and allotment run over every axis, not fn-255's open working set. They are bit-identical either way:

- A closed subtree's upkeep is recomputed each cycle from wood that no longer changes, in the same order. That gives the frozen value's bits.
- Its light and lit presence are none either way.
- The walk's closed laterals do not decide.
- The normaliser's blocks run over growing buds, which are independent of what is open.

The working set is a CPU memory and time lever. On the device it would add compaction for a pass the spike timed at 0.8 to 1.9 ms a busy cycle. It is added only if section 7's measurement asks for it.

## 4. Ordered child lists on the device

The order is fn-255's: children by node, then index, with a continuation or relay at the tip last. The device keeps it as a CSR (`kid_start[axis]`, `kids[]`) rebuilt incrementally each cycle:

1. **Count.** The cycle's new axes count themselves onto their parents. These are integer atomics on counts, which are order-free.
2. **Offsets.** `new_start = old_start + Σ new counts before`, by scan over axes.
3. **Merge.** One invocation per parent with new children. It merges its old run, which is already ordered, with its new children sorted by (node, index), and writes the run at its new offset.
   - New laterals sit at nodes beyond the old ones, so the merge is usually an append. A woken bud may sit at an old node, and the merge handles it.
4. **Copy.** One invocation per other parent copies its run.
5. **Swap.** The two CSR buffers swap.

The cost is O(axes) copying a cycle: 1.5M u32, a few tens of microseconds.

## 5. Capacity

- **Never an expectation.** Capacity comes from checked counts and bounded allocation (R3).
- **Growing the arenas:** after each cycle the host reads back one small record without blocking: the arena counts and the budget and overflow flags. The read is pipelined, so it is a cycle or two stale. When an arena's count passes half its capacity, the host grows it by copying the buffer on the queue, which needs no synchronisation.
- **Overflow:** each emit pass checks its scanned total against capacity and writes nothing past it. If that ever fires, the build stops and is rebuilt from cycle 1 with the capacities the counts named. The build is deterministic, so the tree is the same.
- **Dispatch sizes (added 2026-10-11, after LAYOUT.md and LIGHT.md).** Every dispatch is sized from the device. It is indirect, its arguments written by the scan that counts its items.
  - Some loops have a count only the device knows: the layout's generations, the hierarchy's block levels, the sweep's layer ranges. Each such loop is recorded up to a bound the host holds. Past the items, the indirect arguments are empty, and an empty dispatch costs about 5 µs.
  - The device flags any loop whose count exceeds its bound. That is an overflow like an arena's: the build is rebuilt from cycle 1 with the bound the flag names.
  - The bounds start from the last readback's counts plus a margin: generations 8; levels, the last count plus 4; layers, from the box grown by the reach of a cycle.
  - So no dispatch's size waits on a round trip.
- **Bounded work per dispatch.** Any dispatch over a list (apexes, axes, blocks, nodes) is cut into ranges by a constant work bound, so no dispatch exceeds about 1 ms on the busy cycles. Acceptance asks under 2 ms, the watchdog's margin. The bound is a count, never a timing.
- **Device limits:** a device whose limits cannot hold the counted tree returns a named error (R1), never a CPU fallback.

## 6. Retirement

**What it is for:** it is fn-255's memory lever. The CPU proved retirement leaves the tree as it was, to the bit. So on the device it may lag, run in batches, or (for a first version) not run at all, without changing the tree.

**What the device does:**
- At each cycle's start it marks the open axes as fn-255 does: the paths from living apexes and sleeping buds to the root.
- It evaluates the newly closed lateral subtrees with the settle's own kernels (`assign_living`, then `subtree_living`, which are the fades `up` pass).
- It honours fn-255's wait: a subtree holding a bud that woke at cycle `w` is evaluated only from cycle `w + 3`, once the balance after its bearer's next unit has run (R3B.md).
- A retired subtree's phytomers are marked and later compacted out of the arena. Its `Grew` records are kept where the species keeps shed pipes or leaf girth.

**Compaction** runs when the arena would otherwise grow:
- It is a stable scan. Kept phytomers keep their relative order, and the unit records' `first_phyt` and the axes' `node_phyt` are remapped.
- Because it is stable and deterministic, it changes no node slot in the settled table.

**As decided for the build (2026-10-11, the settle worker's five questions):**
1. **Inside the recorded cycle plan.** Retirement's passes run at each cycle's start, before the balance, sized by the axes bound the host already holds. No sync point is added.
2. **Evaluated by levels, not per subtree.**
   - Presences are assigned once per laid axis, when it first closes or is a candidate.
   - Living is carried up the hierarchy's block levels, given the retired roots' living.
   - Each candidate is then judged: closed or pending, beside an open axis, still open, its wait passed.
   - This equals fn-255's per-subtree evaluation (`copied` with stubs) because living folds by `max` and is final once a subtree closes. A test holds every verdict (root, cycle, kept or retired, living) to the CPU's.
3. **Only a retired root's living is stored.** A kept subtree's living is recomputed by the levels.
4. **Compaction happens at a move only,** when the build already rewrites its arenas. It remaps `first_phyt`, `node_phyt`, the layout's phytomer rows and the deposit's ranges.
5. **Records sit in a separate buffer:** five f32 words per phytomer of a retired subtree, plus the vigour, appended in scan order. They exist only where a PA keeps shed pipes or leaf girth. The settle reads the retired roots' living, drops the axes inside, and reads the records in its attachments pass.

## 7. What step 2 measures before the executor is built

The spike did not replay units and layout, which are the CPU's largest phases (Telperion: Units 2.38 s, Sketch 2.05 s). Task 2 therefore first builds steps 3 and 7 alone, the units in count and emit modes and the layout's round 0 and generations. It replays them on captured busy cycles: Telperion at 82, seed 42, and the beech at 40, seed 42.

- **Agreement:** counts and records against the CPU's cycle, integers exact.
- **Device time:** per kernel, the median of 9.
- **Extrapolation:** the build's units and layout time, from the busy cycle's time per phytomer laid and per living apex, times the build's totals (fn-255's census).

**The stop:** if units and layout extrapolate, beside the spike's 85 to 115 ms of hierarchy and light, past F4's warm budget (Telperion 1.2 s, beech 0.8 s), the task stops and reports NEEDS_HUMAN.

## 7a. The dispatch budget, first-class (added 2026-10-11, after the owner's two Chrome probes)

**The second probe** (Chrome 154, RTX 3080, Linux) priced each style of dispatch, 640 of them in a chain:

| Dispatch style | Per dispatch |
|---|--:|
| Indirect, one pass | 429 µs |
| Indirect, a pass each | 432 µs |
| Direct, one pass | 5.3 µs |
| Direct, a pass each | 6.7 µs |
| Direct, over 8 command buffers | 5.5 µs |
| One dispatch looping 640 steps with storage barriers | 3.4 µs |

The cost is Dawn's validation of indirect arguments, not the dispatch.

**What the first reading would have meant.** The first probe gave about 0.4 ms a dispatch. Priced at that, the executor's indirect builds (CYCLE.md: Telperion 28,016 dispatches, 342 a cycle) were about 11 s against 1.2 s, and bounded subtrees could not fit. Under direct dispatches at about 7 µs, the same counts cost about 0.2 s on Telperion.

**Compile is outside the budget.** The owner accepts device creation and compile as a once-per-page cost, so the per-tree budget is the build after them. Chrome compiled the spike's 24 pipelines cold in 65 to 367 ms.

**The budget per build**, at about 7 µs a direct dispatch:

| Build | Warm budget | Dispatch overhead the executor may spend (target) | Dispatches a build at most | A cycle at most |
|---|--:|--:|--:|--:|
| Telperion 82 | 1.2 s | 0.2 s | about 28,000 | about 340 |
| European beech 40 | 0.8 s | 0.1 s | about 14,000 | about 350 |

The executor reports its dispatches per cycle and per build beside its time, on every measured build.

**The host's binding rules (2026-10-11):**
1. **Direct dispatches only.** No `dispatchWorkgroupsIndirect` anywhere in the cycle, native included, so both paths share one shape.
   - Each phase is dispatched directly with a workgroup count the host knows: the block count, tracked from its own bookkeeping, or a fixed upper bound from the budget.
   - Each invocation reads the true count from a buffer and exits past it.
   - This replaces section 5's indirect arguments. Its bounds stay: the one-cycle-late counts plus a margin, a flag on overrun, and a rebuild with the bound raised.
2. **At most 16 storage buffers a shader stage, and 8 where it is cheap** (WebGPU's guarantee). Buffers are packed into structs or combined arrays. The spike's `shed` needed 17, which Chrome refused.
3. **Tint's rules** (section 7b):
   - No NaN or Inf constants. A sentinel is built at runtime from a uniform.
   - Barriers only under uniform control flow, with loop bounds read through `workgroupUniformLoad`.
   - The probe's fixed copies show both: `.worktrees/probe/src/probe/shed.wgsl` and `light.wgsl`.

## 7b. Tint's rules (added 2026-10-11)

The owner's Chrome refused two spike modules that naga accepts:
- **NaN and Inf constants:** for example `bitcast<f32>(NAN_BITS)` as a `const`.
- **Barriers under control flow Tint calls non-uniform:** for example a loop bound read from storage.

So every kernel loads its barrier loops' bounds through `workgroupUniformLoad` or from a uniform, and declares no NaN or Inf constant. CPU validation is naga-only (fn-260), so the cycle module carries a static check of its own for both patterns until a Tint check exists. The check is conservative: a barrier may sit only under conditions built from uniforms, constants, literals and values `workgroupUniformLoad` returned.

## 8. The CPU's f64, and what stays open

**The rule:** a last-place difference in a value a decision reads can flip that decision. Over a whole build, that makes a different tree by lineage, not a tolerance on one value. So every f64 operation of the cycle that feeds a decision is reproduced on the device bit for bit.

There are few of them:
- **The tick grid** (`share`, `spent`, `sleep`, `aged`, `grown()`): exact multiples of 2^-20 of a cycle. They are held as integer ticks.
- **The bend's years:** an exact tick count, rounded to f32 once.
- **The lattice sweep** (`light::swept_all`): f64 throughout, with `exp_fixed`.
  - Its size is small: about 61k nodes × 9 directions on Telperion.
  - The device runs it in a software f64 on u32 pairs: IEEE add, multiply and divide, round to nearest even, and the CPU's own `exp_fixed` polynomial.
  - That is exact by construction. The integer-emulation cost is measured beside the spike's f32 sweep (0.07 to 0.13 ms a busy cycle).
- **The settle's** `assign` rank sum and `1/e` in `radius`: the same software f64.

The f32 operations remain under COUNCIL.md's agreement standard: floats by lineage within measured tolerance, with no guards on fused multiply-adds. A float that differs reaches a decision only through a comparison. Where the agreement test finds such a flip, it is reported with the comparison named.

**Where a float becomes an integer (added 2026-10-12, after the review).** COUNCIL.md asks for integers exact and floats within tolerance, with no guards on fused multiply-adds. The two clauses meet wherever a float is floored or rounded into a stored integer. The measured case is the wake time in ticks: 261 axes differ in plain mode on the oak at 40.

The executor holds every such site to the CPU's order in both arithmetics: the f32 operations whose result is floored or rounded into a stored integer or a decision's integer. That covers the wake ticks, `below`, and the lived ticks. All other floats stay plain under COUNCIL.md's tolerance. That is the narrowest reading that keeps "integers exact".

**Open, for the host:**
1. **The settle's leaf mean** is a sequential f32 sum on the CPU. The device takes it in blocks (section 2.4). Matching it to the bit means the CPU takes the same blocked order, which moves the oak and the ordinary by f32 order. It is the same kind of contract as the normaliser's. Until that call, the device takes the CPU's sequential order in one workgroup (one dispatch, at most about 18M adds on Telperion), and its time is reported.
2. **If the software f64 sweep costs too much,** the alternative is the CPU's sweep in f32 under the one-reduction-order contract. That changes the CPU's lattice, so it is the host's call.

## 9. The host's rulings (2026-10-12)

1. **Arithmetic: plain, with tolerance.** COUNCIL.md's "integers exact" means identity integers: lineage keys, canonical order, slot allocation and births, and the counts the host reads.
   - Decisions taken from float inputs (a relay's `sleep` and the like) may differ within the lineage tolerances fn-254 R4 check 4 allows.
   - The floor sites stay held to the CPU's order (section 8), since they cost nothing.
   - **Exact mode is deleted,** not kept as a second build of the kernels. It cost 34% more GPU time and 96 s of cold compile, for bit equality that is not needed.
   - EXECUTOR.md records the agreement per preset.
2. **Subnormals:** no emulation. Both executors use plain arithmetic, inside the tolerance.
3. **Cold compile is a blocker.** The cycle's total cold compile, native with the driver's disk cache off, comes under 2 s before task 2 closes. The owner accepts about 0.5 s a page, and Chrome's watchdog kills long compiles. The units module (37.7 s) is split into small kernels, and big helpers stop being inlined into every entry. Time per module is reported before and after.
4. **The settle's leaf mean** becomes the canonical blocked reduction on both executors: blocks of 1,024 in subtree order. Section 8's open item 1 is closed. It may move the oak's and the ordinary's trees by f32 order; the agreement checks and stills are the evidence.
5. **The 2,048-cycle limit stays a named error.** Every shipped age is far under it, and widening it waits for a consumer.
6. **The budget** belongs to task 4. After items 3 and 4, the time is reported per phase.

## 10. Units as small kernels (2026-10-12, after the host's ruling 9.3 and the bisection in CYCLE.md)

**What the bisection shows.** The units entry compiles cold in 43.7 s. The cost is the volume of code the driver inlines into its one apex-run body, not any single construct. The zones, the 64-bit mixes behind every child and draw, the failure path and the abortion draw each carry 10 to 20 s. With the three largest removed, the entry still takes 9.4 s.

So the apex run is split into phase kernels over flat work lists. It is not a per-apex interpreter resumed across dispatches. Each entry holds one phase's code. No entry holds more than one call site of the 64-bit mix chain where a loop can share it.

**The phases of a round.** A round covers every apex with a unit to grow. Round 0 takes the living apexes, then the woken ones. Each phase is one dispatch, or a scan:

1. **`decide`**, one invocation per apex. It takes:
   - the unit's share and laid cut (`Carried::unit`, `Unit::laid`);
   - survival, with its draw and threshold (the held sites);
   - the failed-unit relay draw and the lived ticks and `below`.

   It writes the outcome (grow, fail and relay, stop) and the unit's record fields.
2. **`zones`**, one invocation per (apex, zone). It counts the zone's nodes by the node law's walk, cut by the laid count.
3. **Scan** to a node list of (apex, zone, j).
4. **`order`**, one invocation per node. It takes the order draw, then its rank among its zone's nodes by `Reverse(draw)` with ties to the lower j. The zone is a contiguous run of the node list, so the rank is a count over it, which is stable.
5. **`node`**, one invocation per node. It takes:
   - the phytomer's key and presence;
   - the places, each lateral's PA and presence;
   - each sleeper's draws.

   It writes per-node counts of laterals and sleepers.
6. **`after`**, one invocation per apex. It takes:
   - abortion and persistence after the unit;
   - the age's end: move, continuation, ended relay;
   - the apex's own return to the living list.

   It counts the axes and apexes it adds. An axis whose unit is carried on within the cycle (`carry_on`) enters the next round as an apex.
7. **Rounds** repeat until no apex carries on. The count is bounded and flagged as section 5 says.

**Order.** Every appended record carries the CPU's position key: (apex's place in the living list, round, zone in `ALONG` order, node rank, slot), with an apex's own after-records last in its round.
- **Count pass:** it runs all rounds and stores the counts per (apex, round, node).
- **Placing:** one scan, apex-major and then by round, places every record exactly where `grow_live` and `absorb` would append it.
- **Emit pass:** it runs the same phases and writes at those offsets.
- **Why this is sound:** the draws are keyed by lineage, so the two passes agree by construction, as they do today.

**Wake:**
- `dormant::carried`'s walk is a per-sleeper kernel, `carried`, which counts each bud's wakes. A scan gives the woken list.
- A woken bud's stage axes come from a small per-wake kernel.
- Each woken bud's first unit then enters `decide` in its own round 0, after the living apexes, as `wake` runs after the merge.

**The other large entries.** The settle's `walk_main` (15.1 s) and `frame_main` (2.0 s), and the layout's `frame_main` (2.84 s) and `draw_main` (1.64 s), are cut the same way. A walk step's turn, bend, sag and landing become their own entries over a per-phytomer-step list, or the step runs through the light module's instruction-list form, where each software operation has one call site.

**Budgets:**
- **Compile:** every entry compiles cold in at most about 0.2 s, and the cycle's total under 2 s (ruling 9.3).
- **Dispatches:** a cycle adds about 6 kernels and 3 scans a round, twice (count and emit). With about 3 rounds that is about 60 dispatches a cycle, about 5,000 for Telperion, about 35 ms at Chrome's 7 µs.
- **Work:** the per-node phases spread work that one thread per apex ran serially. That should cut the units' 360 ms, but it is measured, not assumed.

**Amended (2026-10-12, the growth worker's reading of `Shoot::move_on`, stop.rs:63-83).** The CPU appends depth-first within an apex. `carry_on` grows the continuation's whole cycle, including its own stops, relays and carried-on buds, before the apex makes its ended-age relay (host decision 11). The failed-unit path chains the same way. So the position key is (apex place, **step**, zone, node rank, slot), not (apex place, round, …), where step counts the apex's operations in execution order.

- **Per-apex state in buffers:**
  - the current operation;
  - the After stack (depth at most 8, as `relay_after` has it today);
  - the time spent;
  - the frame, as axis columns.
- **Each round:** `decide`, `zones`, `order` and `node` run only for apexes whose current operation is an advance. `after` loops through the operations that are not an advance (stop, relay, move, `relay_after`, the hand-over of `carry_on`) until the apex's next advance or its end. So a round is about one advance per apex, and the number of rounds is the longest chain of advances in the cycle, bounded and flagged.
- **`after`'s body** holds no zones, so it stays small. It keeps the 0.2 s compile bound, or it is cut further.
- **Placing:** one scan, apex-major and then by step.

## 11. The walks' cold compile, and pipelines made side by side (2026-10-12)

**Native transcendentals in the geometry.** The layout's and the settle's walk and frame entries compiled cold in 1.4 to 14.8 s. Most of that was the libm-exact ports (`sincosf` in 64-bit integers, `atanf`/`atan2f`, `asinf`, `hypotf`), built only for word-exact agreement.
- Under ruling 9.1, geometry that floors into no integer takes plain arithmetic within the ruled tolerance.
- So the walk and the sag use WGSL's own `sin`, `cos`, `atan2` and `asin`, and a plain hypotenuse.
- The ports and their tests are deleted.

**Pipelines made side by side.** A browser already compiles every program at once (`create_compute_pipeline_async`, `programs.rs`). Natively, `io::Pass` now asks for a module's pipelines on parallel threads (`Program::make_many`), so a module's cold compile is its slowest entry's, not their sum.

**Cold, native, driver cache off:**

| Measure | Before | After |
|---|--:|--:|
| Modules one after another, entries side by side | 23.6 s (entries in turn) | 1.67 s |
| Every module at once | 16.7 s | 0.98 s |
| Units module | 3.0 s | 0.39 s |
| Layout | 3.9 s | 0.48 s |
| Settle | 15.2 s | 0.35 s |
| Light | 0.9 s | 0.18 s |
| Hierarchy | 0.35 s | 0.11 s |

The strictly sequential sum of every entry, one after another, is 6.0 s. That is not how either path builds.

