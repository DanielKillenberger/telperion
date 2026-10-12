# fn-256 task 2: the device cycle executor, its design (worker, 2026-10-11)

This design ports fn-255's one cycle (`grow/cycle.rs`, `Grower::year`, then `Grower::settle`) to the device. The COUNCIL's decisions hold throughout: bounded subtree blocks, one reduction order, the integer deposit, the one-workgroup-per-direction sweep, the f32 viability, and agreement measured by lineage. This file adds what COUNCIL.md and SPIKE.md left open: the buffers, each cycle's dispatch graph, the block bookkeeping as the tree grows, and how ordered child lists are kept on the device.

Task 2 builds the executor test-only. Task 3 routes production through it.

## 0. The governing rule: every append is a scan, never an atomic

The node table must be byte-identical over fresh contexts and block sizes (R2, R2a). Every record the executor creates therefore gets its index from an exclusive prefix sum over its producers, taken in the CPU's order. No index ever comes from an `atomicAdd` slot.

- **Atomics are allowed only where their result is order-free.** That means the integer deposit, integer counts, and min/max folds.
- **The CPU's order** is the order `Grower` appends in:
  - Units go in live order. Each apex's records follow in its emission order (`grow_live`, `absorb`, which is equivalent to one sequential pass).
  - Wake goes in `asleep[cycle]` order, after the merge.
  - New axes are numbered after the cycle's existing ones, in the CPU's order. **Retired (host, 2026-10-12, section 13):** an axis index on the device no longer equals the CPU's once the arena compacts; an axis is known by its lineage key, and agreement is checked by lineage.

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


## 12. The seam: the device's settled tree, whole, to the draw (task 3, 2026-10-12)

The host's decision (B): the tree stays on the device. The whole cycle-grown tree is one resident part; nothing of it is converted on the CPU, and only totals and summaries are read back. CPU builds grow through the CPU cycle and draw through `prepare_whole`, the reference.

### 12.1 What the drawing stages read today

F4's resident build (`fine/resident.rs`) meets the CPU at four places:

| Stage | Reads from the fine roots | Reads from the CPU part |
|---|---|---|
| dress (`dress.wgsl`) | each wave's chains: their items, axes (`AXW`), scratch (`SA`) and laid buffer (`AL`, `PL`) | each root's stub: its bearer's drawn node, origin and distance (`stubs.rs`, `Tree::attachments`) |
| fold (`fold.wgsl`) | the drawn records (`CV`) | the CPU nodes' folds, bins and box, combined |
| expansion (`Part::expansion`) | the folds, crown front and leaf box | the tree: `clothe`, the envelope's points, `Reference::grown`, `wood_box`, `bearing_bounds` (root radius, `stem_radius`) |
| sweep, stations, rings, place, handoff, florets | the drawn records, `Swept` | the CPU block's curve, stations, segments, florets |

Every stage after the dress reads only `Dressed` (the drawn records, `CV` a node, `CA` an axis, one buffer a wave, the stubs first) and the expansion's rows. So the seam is the dress and the CPU part.

### 12.2 The adapter: the cycle's dress

**Chains, planned on the device from the node table.** The cycle's node table holds the settled tree in lineage order, depth by depth, with each axis's parent, kind, node, PA, lineage, phytomer start and count, base, join, and each phytomer's key, cycle, scale, radius and laid tip, heading and side (`settle_state.wgsl`'s `A_` and `PH_`).
- A chain is a lateral's first axis (or the seed) and every axis reached from it through continuations and relays: what `dress.wgsl` draws in one invocation. Its wave is the number of laterals between it and the seed.
- **Planning** (new kernels in `cycle/`, a depth at a time, then per wave):
  1. Each axis's chain head and wave, from its parent's (laterals and the seed head their own).
  2. Each parent's first child and child count (order-free folds; a parent's children are contiguous in the next depth, in lineage order).
  3. Per wave, the heads in index order (a flag scan); each chain's axes and phytomers counted by its head (order-free).
  4. One invocation per chain walks its chain breadth first from its head, children in order, skipping laterals. On a BFS-ordered table that visits the chain's axes in increasing index, which is the conversion's order. It writes each axis's place in its chain and its wave-local axis and phytomer offsets.
  5. One readback: each wave's axes, phytomers and chains, which size its drawn buffer.
- **The dress** (`cycle/dress.wgsl`): `dress.wgsl`'s conversion rewritten to read the node table's columns in place of the chain's `AXW`, `SA` and `PL` words. It keeps the drawn layout (`CA`, `CV`, joins after the wave's phytomers), so the fold, sweep, stations, rings, place and handoff run unchanged. It is a rewrite of a shader the fine path deletes, not a copy kept beside it.
  - Positions are the tree's own: no origin and no root-local frame.
  - Each phytomer's internode spread factor is taken from its key and its PA's spread, as the CPU's `internode_factor`.
  - Joins read the node table's join columns and the relay's first phytomer's key (`join_key`).
- **The root.** Wave 0 has one bearer record: the tree's root node, `Node::root()` at the origin. The seed's first node starts a run without parting as a lateral (no codominance, start radius its own), as `convert` draws the seed from node 0.
- **The stem.** The dress marks stem nodes (`STEM`, flag bit 128): the seed chain's axes of a trunk PA carried from the seed or a stem node, a stopped module's head turned aside unmarked, as `convert` does. Only the seed chain can hold stem.
- **What goes:** `LANDED`, `FINE` and `SEAM` (every node is the device's, and no run starts at a seam), the stubs (`stubs.rs`), `Stub`, `Part`, `Tree::attachments`, `Tree::seams` and the conversion's fine marks.

### 12.3 The whole-tree summaries the CPU part supplied

| Summary | Was | Now |
|---|---|---|
| Height, crown's lowest, widest | `Folds::of` over the CPU nodes, folded with the device's | `fold.wgsl` alone; its `LOW` skips stem nodes, as `Folds::of`'s crown |
| Bins, crown front | `bins` over both | `fold.wgsl` alone |
| The leaf box and the reference | `Reference::grown`, `wood_box` of the CPU nodes, with the device's corners | `Reference::spanning` of the device's corners and the root |
| The root's radius | `tree.nodes[0]` (`convert`: node 1's start radius) | the seed's first drawn node's start radius, read back with the folds |
| The slender bound's stem radius | `Tree::stem_radius` | a fold: each drawn node's stem children counted, the first fork by the dress's per-chain sequence (the seed chain's order is the conversion's node order), then its stem children's widest radius; none, the root's |
| Leaf retention recorded | `tree.leafing` non-empty | the PA rows (`convert`'s `leafing_on`) |
| Leaf bases (`clothe`) | the CPU tree | none: only a rosette clothes, and a rosette has no stations (12.6) |

`executor::Expansion` gets a constructor from the family and these summaries, with no tree. The resident build keeps no CPU block: no CPU curve, stations, segments or florets. The handoff packs the device's curve alone (an empty CPU curve), and the placement places the device's block alone.

### 12.4 The build, natively and in a browser

The cycle's build becomes async end to end, as fn-212 made the fine path: the census's late readback awaits its mapping (natively it polls its submission first, as `io::PendingRead` does), every `read_words` is `io::read_async`, `Instant` is `Clock`, and the trace switch goes. `mod cycle` is no longer test-only and compiles on wasm32. Its tables come from `grow_capture_tables`, the CPU's per-species tables, once per build (no per-cycle CPU work).

`Generator::prepare_async`: resident delivery grows through the device cycle and draws resident; CPU delivery grows through the CPU cycle (`executor::grow`, now `grow_cycle`) and draws through `prepare_whole`. The executor is the delivery's, chosen with the context; no species reaches the choice. A device capacity failure is the build's named error, never a CPU rebuild.

### 12.5 Deleted, each with the consumer checked

Each item of R3B.md's list, with what was checked for a reader.

| Item | Done | Consumer checked |
|---|---|---|
| `executor::apart`, `Part`, `settle`, `Settling` | deleted | `prepare_async` routes by delivery alone; no family is parted, and nothing names a species |
| `Tree::attachments`, seams, `in_fine_roots`, stubs | deleted | paths, frames, rings, compact, foliage plan, placement, prepared, station: each now reads one tree with no seam; frames seed from `tree.frames` |
| the fine-root cut and marks (`space` `fine/{cases, geometry, survey, landing, spare, likely}`, the marks in `convert`) | deleted | `pipeline::space::grow` converts `telperion_space::grow` (the cycle) with no marks |
| the stepped executor's fine side (`grow/stepped/{buds, schedule, settle}`, the fine branches of `feedback` and `phases`) | deleted | the stepped executor now holds only `grow_coarse`'s coarse tree and stands |
| `alone.rs` and `alone/` (field, shed) | deleted | no reader after the fine side went; `grow/alone.rs` keeps `rules`, `drawn`, `Bud`, `Pending`, read by `run` |
| `reads_light`'s start gate and the alone refusal | deleted | the cycle reads light for every species |
| render `generation/fine/` (61 files: stubs, roots, laid, the resident and parity tests) | deleted | the resident build reads the cycle's settled tree (`cycle/tree.rs`) |
| `chain.wgsl` | deleted | the chains are planned by `cycle/chains.wgsl` |
| `contract.wgsl`, `wide.wgsl` | moved to `cycle/decide.wgsl` (the contract's decisions) and `cycle/u64.wgsl` (wide integer arithmetic) | read by `cycle.rs`'s `contract()` |
| `plan/measured.rs` | deleted | its only reader was the cut's seam |
| `ChainTables::new`, `boundary::curves`, the space examples `alone, coarse, contract, cut, cutlight, digests, field, stepped` | deleted | no reader; `ChainTables::cycle` serves the device |
| `Predictor::new` | test-only | read by the closed-form tests; the site's coarse tree reads predictors through `Rows` |

Kept, each for its reader, until fn-257 replaces the site's coarse route (`pipeline::space::grow` at `cut > 0`):

- `reads_light`: `closed_form::expected_kept` and `fine/coarse.rs`'s lit route.
- `Certain`/`lasting`: `grow::run`, the coarse engine and the `whole` test reference; the cycle sets `lasting` to none.
- the f32 `Field::on` deposit: `grow/sketch.rs` and the stepped executor's coarse feedback.
- the f64 survival path (`dormant`, `presence`, `couple`, `contract`): the closed form's rows, which the coarse tree's expectations read.
- `grow_coarse`, `stepped::standing` and the stands: the site path itself.

Production-dead, kept as the reference: `prepare_whole`'s resident branches (CPU-built wood with GPU stations). `prepare_async` sends every resident build to the device, and the route tests hold the device to this path's output. Whether it stays as a test reference or goes is a follow-up.

### 12.5a Agreement and stills

The render gate's resident-against-CPU checks are now lineage checks (`resident/agree_tests.rs`), at age 12 on every preset the device draws: nodes equal in count and lineage, positions within 3e-5 m; station runs equal; every wood run matched point for point, the trunk included (a 1% run tolerance had hidden a missing `trunk_run` until the stills showed it; FRICTION.md).

The stills at each preset's default age, whole, mid and close, F4 against the device after the review's fixes (`raw/t3-stills/sheets/`, every sheet viewed; share of pixels differing past 2%):

| Preset | Whole | Mid | Close | Seen |
|---|--:|--:|--:|---|
| ordinary | 1.3% | 6.2% | 7.5% | same form; twigs and leaves placed differently |
| oak | 2.3% | 7.3% | 8.8% | same crown; the close view darker, the cycle's leaves seated elsewhere (the CPU cycle and the device alike, 1.1% apart) |
| spruce | 0.5% | 2.8% | 4.1% | near identical |
| beech | 0.2% | 0.6% | 0.5% | identical to the eye |
| birch | 1.0% | 5.5% | 11.3% | same habit; hanging twigs placed differently |
| palm | refused | | | refused resident (12.6) |
| Telperion | 1.0% | 1.4% | 1.4% | identical to the eye |
| Laurelin | 1.6% | 7.4% | 5.0% | same crown; flowers and leaves placed differently |

Two device defects showed in the first stills and are fixed: the trunk's foot (above) and the curve's section rows, which the handoff had taken as defaults (no lobes, twist or height; the codex review's finding, now held by `every_preset_draws_its_wood_by_the_cpu_rows`). What remains is the one cycle's (fn-255): the CPU cycle and the device differ by 0.2 to 1.6% on the oak and the beech.

### 12.5b Build time

Warm ms, `build_time_tests.rs` (`raw/t3-build-time-{before,after}.txt`): ordinary 523 to 518, oak 1159 to 515, spruce 403 to 657, beech 210 to 287, birch 716 to 739, palm 4 to refused, Telperion 671 to 5929, Laurelin 627 to 5834. The Two Trees' cost is the device grow's restarts from its least tables, task 2's carried item 1, which task 4 owns.

### 12.6 Open, for the host

**A family whose leaves stand on no regular twig stations (the date palm's rosette).** The device's stations number regular stations only; F4 drew the palm through `prepare_whole` by `executor::apart`'s capability gap. Under 12.4 every resident build is the device's. The options: (a) the device grows it and its settled tree is read back for the CPU's rosette drawing, chosen by the leaves' capability, never the species (the palm is 480 phytomers); (b) refused by name in the harness until the stations draw rosettes; (c) the device's stations learn rosettes. Until the call, the palm's resident build is refused by name.

Host decision, 2026-10-12: (c), the device learns the rosette. (b) drops a shipped species from the harness; (a) is a second drawing path kept for one leaf arrangement.

### 12.7 The palm on the device: what (c) takes (sized before code, as the host asked)

What the CPU draws for the palm, read from the code:

1. **The rosette** (`pipeline/foliage/rosette.rs`, `leaflet.rs`). At every stem apex (`Tree::stem_apices`: a stem node no stem continues, in identity order) the fronds stand on one spiral: frond k at `k · divergence`, leaving the axis `depth · age` below the apex at `pitch + spread · age`, then the skirt's dead fronds further down the same spiral at the skirt's pitch and share of length. Each frond is a rachis of `leaflet_count` leaflets: point `t` along an arched rachis, alternating sides at `leaflet_pitch`, the first `acanthophylls` drawn as spines at their own pitch and share, and the last closed by `terminal_leaflet`. Every leaflet takes the station's draws (scatter, size) from a stream keyed by the seed, the apex's birth order and k. Dead fronds' leaves carry the withered bit. Where a rosette stands, it is the tree's only leaf source.
2. **The retained leaf bases** (`pipeline/branching/leaf_bases.rs`, 408 lines; `lattice.rs`, 117 lines). Expansion's first step (`pipeline::clothe`) hangs 256 boots on every stem, after the radius solve: each is two wood nodes at an exact height along the stem's polyline, on the crown's spiral continued downward, worn toward the ground, with a lattice cell (`tree::Section`) per packed base.
3. **The boots' wood** (`pipeline/surface/section.rs`). A run whose last node has a cell is swept onto it: each ring centred on the cell's radial at its reach, its radius the cell's extent, and its frames squared to the cell. The renderer reads the cells as `Curve::sections`.

The device form, by part:

- **Rosette, contained in the stations stage.** `stations.wgsl` records each stem apex (a drawn STEM node with no STEM child) as `blooms.wgsl` records a bloom: counted, scanned, one record each. A rosette entry in `pack.wgsl`/`place.wgsl` places leaf `index` as a pure function of (apex record, frond k, leaflet j): the spiral, the arched rachis and the leaflet pitch mirrored from `rosette::fronds` and `leaflet::leaflets`, the draws on the device's keyed stream (`random(key, draw)`, as the stations' leaves take theirs), and the withered bit for k past the living fronds. The rosette rows ride in the placement config. Regular stations stay by degree: at `rosette_fronds` 0 nothing changes, and above it the stem's own stations count zero, as the CPU's plan does. Agreement by lineage: the same apices, fronds and leaflets, their points and axes before the draws within tolerance, counts equal. About 150 lines of WGSL and 80 of Rust.
- **Leaf bases, a new stage and a new data path.** The boots are not stations. They are wood the expansion adds to the tree, laid out from the stem's polyline, girth and the crown's spiral. On the device this is a stage between the dress and the folds: per stem, walk its drawn records to the exact heights, compute the 256 bases and their cells (`leaf_bases.rs` and `lattice.rs` mirrored, about 500 lines of CPU geometry), and append two drawn records a base plus a cell table. The dress's record count, the folds and the sweep's gather all grow by them.
- **Shaped sections in the sweep, an extension of the sweep and the handoff.** The sweep reshapes a boot's run onto its cell (`section::reshape`) and squares its frames, and the handoff packs the device's cells for the renderer in place of the rows curve's empty `sections`. That is a new buffer from the drawing stages to the renderer.

Without the second and third parts the palm would draw on the device with a bare trunk: the lattice of boots, the palm's defining trunk (fn-110, fn-144), would be gone. So (c) is the rosette (contained) plus a leaf-base stage and shaped sweep sections (not contained). This is the size the host asked to hear before building.

Host decision, 2026-10-12: (ii). Task 3 builds the rosette. The boots and their cells become their own spec, which fn-256 depends on. Until it lands, the palm's resident build is refused on this branch by that capability (retained leaf bases), never by species.

#### 12.7.1 The rosette in the stations stage, as built

- **Rows.** The stations' config gains the rosette's rows: fronds, skirt fronds (none without a skirt length), divergence, depth, pitch, pitch spread, skirt pitch and skirt length, and the frond's leaflets (`rosette::leaflets`), rachis length and arch, leaflet pitch, terminal share, spines' count, share and pitch. At zero fronds every row is dormant and the stage is as it was.
- **By degree.** Where fronds stand, no wood bears and no tip clusters (`plan::bearing_runs`, `cluster::of`). An apex is a drawn stem node with no stem child (`kids` marks a stem's parent), and it starts a run of its own: `starts_run` takes it as it takes a tip. `measure` gives it `(fronds + skirt) · leaflets` stations in `fronds + skirt` segments, one a frond. The runs are keyed, sorted, scanned and numbered as every run is.
- **Fronds.** `emit` writes each frond as a segment of a third kind (`distance.y = -2`, after the tip cluster's `-1`): where it leaves the axis, its heading, the apex's axis, its rachis length and size share (a skirt frond's `skirt_length`), whether it withers, and the leaflet rows. These are `rosette::fronds`' spiral mirrored. Its key is the apex's under the registry's new `ROSETTE`, then the frond's place on the spiral.
- **Leaflets.** `pack.wgsl`'s `station` places leaflet j of a frond segment as `leaflet::leaflets` does: the point on the arched rachis, the run's direction, the alternating pitch closed by the terminal share, and a spine's own pitch and share. It takes the same leaf frame, scatter and size draws as every station (`random(key, j)`), and the withered bit (`packed::WITHERED`) on a skirt frond.
- **Agreement.** The palm with its bases set aside, by lineage: the same leaf count, every leaflet's position within the tolerance in the CPU's order (apex, frond, leaflet), and the same withered leaves. The draws are the device's own, as every station's are.

Measured (`every_rosette_is_placed_as_the_cpu_places_it`, the palm with its bases set aside): 6,554 leaflets at 12 and at 40, each within 3.2e-5 (12) and 4.1e-5 (40) of the height of the CPU's in the CPU's order. The withered leaflets are the same ones, and the mean size is 0.2% apart. The palm's nodes and curve agree by lineage like every other preset's (144 nodes matched, positions within 1.2e-6 m), and two builds give the same leaves. Stills (`raw/t3-stills/sheets/date-palm.png`, F4 against the CPU cycle and the device, bases set aside on both; viewed): the crown is F4's in form, fronds, skirt and spines. The trunk is bare where F4 has its boots, as expected until their spec lands. CPU against device differs by 2.5% whole and 13–14% mid and close, which is the leaflets' own draws.

### 12.8 The palm's leaf bases on the device (fn-266, designed before code)

12.7 sized parts 2 and 3: the boots as wood and their cells in the sweep. This section is their design. The CPU's reference is `leaf_bases.rs` (the table and `hung`), `lattice.rs` (the cell) and `surface/section.rs` (the reshape and the squared frames). The device mirrors them in plain f32 under COUNCIL.md's tolerance; the CPU stays the reference and does not change.

**What the CPU does, and the order the device keeps.** Expansion's first step clothes the tree, after the conversion and the radius solve. The envelope is read before it (`Inputs::of(family, grown_envelope(&tree))`), so no boot moves the height, the crown's lowest, the widest or the shoulder's bins. The leaf box (`Reference::grown`) is read after it, so boots enter the box. The stem radius (`Tree::stem_radius`) reads stem children only, and a boot is no stem node. The canopy rows a boot reads are the family's own (`family.canopy`), not the envelope's, so the boots can be laid before the folds.

**The stage, `resident/boots.rs` and `boots.wgsl`, between the dress and the folds.** It runs after `tree::drawn` (the dress and the trunk's measures) and before `extent`. Only the seed's wave can hold stem nodes (12.2), so the stage binds that wave's buffer and no other: a stem node's parent is a record of the same wave or the root (id 0, which is no stem node). Bindings: the uniform rows, the seed wave (read), a scratch buffer, the boots' wave and the cells (read-write). Four storage buffers. Direct dispatches only, no barrier, no NaN or Inf constant.

1. **`kids`**, one invocation a seed-wave record: a made stem record marks its parent's record "carried" (an atomic or). That is `Tree::stem_apices`' `carried`.
2. **`apices`**, one invocation: the seed wave's made stem records no stem carries, in record order (the conversion's node order; the CPU sorts its apices by identity, and the parity test matches by key, so the order is not load-bearing). For each, its polyline's length is walked up the parent ids. A stem whose clothed length (`run - rosette_depth`) is not positive hangs nothing, as on the CPU. Each clothed apex gets a slot: its record and its first boot. The count of clothed apices is written to the scratch.
3. **One readback:** the clothed apices. The boots are that count times `leaf_bases`; they size the boots' wave and the cells, and every later dispatch is direct over a count the host knows.
4. **`place`**, one invocation a boot (apex, k): the `LeafBase` of `leaf_bases()`, k counting from the newest down. It takes the depth wanted, then `along` by walking the polyline from the apex (`position(d >= want)` clamped to the first segment and the deepest), the smoothed centreline where the bases pack (`centreline`: nine samples either way of a trunk's width), the spiral angle `(rosette_fronds + k) · divergence`, the crown's reference carried by the one rotation (`carried`), the radial and the heading leaned by the clamped pitch. It writes from, axis, radial, heading, girth, the bearing record and the age to the scratch.
5. **`lattice`**, one invocation a clothed apex: its boots' mean girth (the bark) and, where they pack, their mean spacing along the centreline (else the polyline spacing). Then `lattice::cell` with Lagrange's reduction. `f64::round` rounds half away from zero and WGSL's `round` to even, so the device rounds as `sign(x) · floor(|x| + 0.5)`. It is a sequential f32 sum of 256 terms in one invocation, the CPU's order, held to tolerance.
6. **`hang`**, one invocation a boot: `hung()`'s two nodes and, where the bases pack, `section()`'s cell. Two records a boot are written in the drawn layout (`CV`), so the folds, the sweep and the stations read them with no new reader.
   - The first record stands at `from`. Its radius, start and base radius are `thick`, its parent is the bearing stem record's id, its run and branch are its own, its codominance is 0.
   - The second stands at the tip (a packed base's outer ring's centre, else `from + heading · reach`). Its radius is `thick · wear`, start and base `thick`, its parent the first.
   - Both carry TAKEN, MADE, FINE and a new flag, BOOT (bit 30, above a tip's leaf count, which the dress keeps from bit 8 and is at most 8 leaves), and no STEM, CLUSTERED or TIP. Their distance along the wood is the bearer's plus each chord. Their key, grown wood, phytomer key, flowering share and retention are zero. The first record's word 29 holds its cell's index, or none for a round peg.
   - The boots' wave has one axis record: depth 63 and lineage all ones, so a boot loses every tie for its bearer's run, as a node appended after every other does on the CPU.

**By degree.** At `leaf_bases` 0 or `leaf_base_length` 0 (`hangs`), no pass runs, no wave is added and the build is as it was. At `leaf_base_width` 0 the boots are round pegs: records only, no cells, no shaped run. The rows are read through one executor call that validates the canopy as `clothe_leaf_bases` does (`validate_canopy`) and returns them. It replaces `hangs_bases` and the refusal in `build.rs`.

**Records, ids and the folds.** The boots' wave is appended to `Dressed::waves` after every drawn wave. Its records take the ids after every drawn node (`Dressed::ids` grows by two a boot), so the sweep's `nodes`, `links` and keys are sized for them with no change. The folds take the wave like any other, but BOOT records are skipped by `extent`, `widest`, `publish`, the shoulder's bins and `sift` (the CPU folds the envelope before clothing). `crowns` still takes them into the leaf box's corners (the CPU's `Reference::grown` is after clothing).

**The sweep.** No change to `gather`, `lead`, `ties` or `starts`: a boot's first record starts a run, because its bearer's leader is the stem continuing. The run is [bearer, first, second], three samples, as on the CPU, where a boot leaves the axis at a stem node of the polyline. Two changes:
- **`walk`** ranks a shaped run by its rings' extents (`Section::extent`), as `rings::rank` ranks it after `reshape`.
- **`emit`** writes a shaped run's points from its cell: each sample's centre is `origin + axis · rise + radial · reach` for ring `k + 3 - clamp(count, 2, 3)`, its radius the ring's extent, and its distance along its own. Every frame is squared, the normal `across` and the binormal `axis` (`section::square`).
- The cluster record gains a sixteenth word (`KW` 16): its run's shape flags, `4 | cell << 8` for a shaped run, else 0.

A shaped run's first record is the boot's first node. The CPU keys a section by the run's last node, and a boot's run ends at its second node, which nothing continues, so the two name the same run. The sweep binds the cells in the place of the drawn records for every pass after `gather`, which alone reads records there.

**The stations.** `fine()` leaves BOOT records out. A boot then bears no station, counts as no child in `kids` and keeps no floret, as the CPU's boots, which are wood appended after the plan's kinds, bear nothing. Where fronds stand nothing bears anyway; this holds a canopy without fronds that hangs bases to the CPU. The rings stage is unchanged: a seated leaf reads only its own run's rings, and no leaf stands on a boot.

**The handoff.** `pack_clusters` ors the sweep's shape word into the cluster flags. The renderer's curve takes the device's cells buffer as its sections (`CurveGpu::on_device` takes a sections buffer; `CurveGpu::new` packs the CPU curve's as before). The cells are written in `packed_sections`' layout, 28 floats a cell: origin, axis, radial, across, the corners, the flatness and three rings of reach, rise and scale. So `curve_emit.wgsl` draws them unchanged. With no cells, one zero cell is bound, as now.

**Costs, before measurement.** Six small entries, five dispatches and one readback a palm. About 300 lines of WGSL and 150 of Rust, against 12.7's estimate of about 500 lines of CPU geometry mirrored. `place` walks the polyline about 27 times a boot (the centreline's samples). That is about 256 × 27 × 150 steps a palm, spread over 256 invocations. Build time is reported beside F4's.

**The parity test** (`resident/boots_tests.rs`, the palm at its default age and at 12):
- **Matching:** boots are matched by their stem's apex key (the device's apex record's key; the CPU's `tree.key(apex)`) and ordinal k. The CPU's boots carry no keys (`hung`), so its boot b of a clothed apex's run is k = b mod `leaf_bases`.
- **Exact:** the clothed apices and the boot count. The bearing node by key. Whether each boot packs.
- **Within bounds:** the first node's position, the tip, the radii and the spiral angle (the radial against the CPU's). Then the cell: origin, axis, radial, across, corners, flatness and the three rings. The bounds are set from the measured run (about twice the worst, the convention of `ROSETTE_APART`), stated in metres for positions and as an absolute difference for unit vectors, corners and ring shares.
- **Swept runs:** each shaped run of the device's curve is matched to the CPU curve's shaped run by its first point, its points within the same bounds.
- **Negative controls, each red:** the device's table with one boot removed fails on count; the device's table with one cell's corner moved by 1e-2 fails on cell geometry. The comparison is one function returning its first failure, so each control asserts that function's failure and names it.

**What this leaves.** `agree_tests`' node comparison reads the dress's waves, before the boots, so it keeps setting the bases aside. Its sweep, station and rosette checks take the palm whole once the boots draw. `repeat_tests` and `route_tests` take the palm whole. The refusal goes.

#### 12.8.1 As built (fn-266 task 1)

Three things changed from the design above, each found by a run:
- **The storage limit the device holds is 8, not 16.** A ninth binding on the sweep passed naga and the shader check, and the test device refused it ("limit is 8"; `Gpu::request` opens WebGPU's default). So the cells ride in the drawn records' binding, which only `gather` reads. Every stage stays at 8 or under: the bases' stage binds 6.
- **The flags.** BOOT at 256 collided with a tip's leaf count, and the silver birch's stations fell from 7,033 to 859. BOOT is bit 30 and CELL bit 29. CELL marks the record a packed base's run opens at, and its word 29 holds the cell's index.
- **A base standing on its bearer.** The oldest base, at a stem's foot, is laid at the polyline's last node, so its first node stands on its bearer. `paths.rs` merges a node within 1e-9 of its parent into the parent's stand, so the CPU sweeps that base in two samples (bark and outer end, `Section::ring`'s two-sample case), not three. The device merges it too: its first record is taken and not made, its second is borne on the bearer, and the run opens at the second. In f32 the stand is a few ulps of the depth walked down the stem, so the bound is `1e-9 + 1e-6 · run`. At 40 the device's foot base stood 9.6e-7 m off its bearer; the nearest real base stood 1.8e-4 m off.

**Measured** (`every_leaf_base_is_hung_as_the_cpu_hangs_it`, the date palm at 12 and at 40; raw output in fn-266's `raw/`):

| Measure | At 12 | At 40 | Bound |
|---|--:|--:|--:|
| Bases, CPU and device (exact) | 256 | 256 | |
| Shaped runs, CPU and device (exact; one of two samples) | 256 | 256 | |
| Positions: nodes, radii, cell origin, rings' reach and rise (m) | 4.5e-5 | 8.8e-5 | 2e-4 |
| Swept points of the shaped runs, centre and radius (m) | 5.1e-5 | 9.0e-5 | 2e-4 |
| Unit vectors: the spiral's radial, the cell's axis and across | 1.3e-4 | 2.2e-4 | 5e-4 |
| Corners (radians round, metres along) | 3.9e-7 | 2.5e-7 | 1e-6 |
| Shares: rings' scale, flatness | 6e-8 | 6e-8 | 2e-7 |

Each bound is about twice the worst run. The directions grow with the stem's height, which fits plain f32 sines of spiral angles up to about 800 radians. The error is held under COUNCIL.md's tolerance, not guarded. The controls: with one base removed, the comparison is refused on count; with one cell's corner moved by 1e-2, it is refused on the cell corner.

The resident suite agrees on every preset with the palm whole. The sweep test matches 257 of 257 palm runs (the trunk and 256 bases), centres within 5.1e-5 m. The palm builds the same leaves twice, and the route test draws it.

**Stills** (fn-266's `raw/stills/sheets/`, F4 against the device, viewed): the trunk carries F4's diamond lattice of boots on the same spiral, the foot included (`date-palm-trunk.png`). The crown is F4's in form. Pixels differing past 2%: whole 2.5%, mid 12.8%, close 14.0%. That matches 12.7.1's figures with the bases set aside on both sides (2.5%, 13 to 14%), so the difference is the leaflets' own draws, not the trunk.

**Build time** (`build_time`, the palm at 40, warm): 98, 101 and 98 ms with its bases, against 99 and 96 ms with them set aside. The stage's cost is within noise.

## 13. The arena compacted in place, and sized once (task 5, the host's rulings of 2026-10-12)

**Identity is the lineage key, never a slot.** COUNCIL.md's "integers exact" covers lineage keys, canonical order and births, not physical slots. The rule in section 0 that an axis keeps its CPU index is retired:
- axes compact on the same cadence as phytomers, by a stable scan, so a compacted slot order is still the canonical order;
- every test and consumer that compared by CPU index compares by lineage key: the retirement log (`r_lineage`) and the grown index (`s_grown`), whose runs a continuation and its relays share in order.

**Compaction** (`compact.rs`, `compact.wgsl`) runs between two cycles, a column at a time through one column of scratch:
1. Phytomers of retired subtrees are flagged; where a PA keeps shed pipes or leaf girth they are recorded first.
2. Where no PA keeps records, the axes inside retired subtrees and their units are flagged too. A kept record is read by the settle through its axis's row, so those axes stay.
3. Each retired root freezes its subtree's upkeep (`A_FROZEN`, read as the tree's `T_FROZEN`), and the sums charge it as `Working::frozen` does.
4. Every column holding an index is remapped, then each column's kept rows are moved and the counts set:
   - the axes' bearers, successors and unit links;
   - the units' axis and links;
   - the phytomers' axis;
   - the sleeping, living and woken buds' axes;
   - the hierarchy's children and members and their starts.
5. A table holding more rows than the pass covered raises `Flag::Compaction`.

**Sizing** (`build_arena.rs`): each table is sized once from the closed form's expected kept phytomers, a hint and never a bound. The light's terms are let go for a species that reads light. Each table takes its measured most against the hint over every preset at seeds 1, 7, 42 and its own, past a 1.5 margin. The plan's bounds are stated from the tables. A table the hint undersized doubles once before it overflows, counted as a move: on the rows its last compaction kept for a compacted table, else on its projection. A count past any table or limit is refused by name, and no build begins again.

## 14. Fewer dispatches a cycle (task 6, COUNCIL.md round 4 step 4)

**The units' round, re-fused.** Section 10 cut the apex run into 27 entries for the cold compile. A job's steps in a round read only its own job, and a zone's only its own zone and the zones before it. So consecutive steps of one domain fuse, each step in the order its entry ran:

| Entry | Domain | Steps | Cold, alone |
|---|---|---|--:|
| `init_main` | job | the job begun; a waking bud woken on its first axis | 0.26 s |
| `woke_main` | job | a waking bud woken on its next axis (the wake's rounds after the first) | 0.19 s |
| `decide_main` | job | the unit decided, then its stop, unit, death or abortion | 0.79 s |
| `zones_main` | zone | the zone's nodes laid, its laterals and sleeping buds counted | 0.88 s |
| `written_main` | zone, emitted | its nodes ordered; its phytomers, laterals, sleeping buds | 0.87 s |
| `placed_main` | zone, emitted | its laterals' and sleeping buds' presences | 0.78 s |
| `after_main` | job | the unit's records taken in, then move, moved, stop, relay, ended | 0.80 s |

A round is 3 dispatches counted and 5 emitted, from 14 and 21. Every fusion was timed alone before it was kept: the round in one entry took 5.2 s, the zones and their order 1.2 s, so those stayed apart. A step called with constant `what`s through one call site in a loop compiles once.

**The scan in two dispatches** (`hier_scan.wgsl`). A column of 1,024 records or fewer is scanned whole by one workgroup. A longer one writes each block's total; the second dispatch's workgroups each add the totals before their own block and scan it in place. Integer adds: any order gives the same words. A scan was up to 5 dispatches; the scans' level limits, census words and flags are deleted.

**The narrow levels folded** (`fold.wgsl`). Each sweep over the block levels (the sums' four, retirement's two) dispatches its wide levels one by one. The levels at the top and the deepest, each of at most 8 blocks in the census read, are looped by one workgroup of 256 lanes: 8 groups of 32, a block each, its members by depth as a block's own workgroup takes them. Each axis still takes its own value and then its children in order, so every word is the per-level dispatch's: the cap-invariance tests (`the_grown_state_is_the_same_on_every_context_and_partition`, `the_node_table_is_the_same_on_every_context_and_block_cap`) pass at caps 64, 128 and 1,024 (the per-level replay of task 2 reaches 4,096 but does not fold). The deepest fold loops however many levels the device holds, so the levels' slack and `Flag::HierLevels` are deleted.

**Smaller cuts.**
- The placing pass's end (`round_end_main`) is folded into `follow_main`.
- The placing columns are cleared by clear commands sized on the device (`Step::Zeroed`), not a glue dispatch.
- The densities take the lattice in one dispatch.
- The first two cycles, which read no census, record the sweep's 8 layer ranges, not its bound's 224.
- The sweep's work bound is 256 nodes a thread a dispatch (about 1 ms on a lit oak, under the 2 ms bound).

**The layout's frame, cut by path** for the cold compile: `frame_main` frames the seed and laterals; `gather_main` takes each carried-on axis's stand (`stood`) into its own frame columns; `frame_on_main` finishes it. The stand inside the frame took 1.1 s of its cold compile; in the gather, 0.19 s.

**Kept as it was, measured:**
- **The units' range** stays 65,536 living apexes. At 262,144 Telperion's `written_main` took 5.9 ms a dispatch, past the 2 ms bound.
- **The placing slack** (`STEP`) stays. Over every preset at seeds 1, 7 and 42, the hierarchy's generations rose by 2 over two cycles, and the passes and layout generations by 1 and 2: each at its slack.

**The axes' own hint.** The axis-indexed tables are sized from `telperion_space::expected_axes`, the closed form's grown axes (the walk of `expected_grown`, counting each bud that lives as one axis). Their most held against it runs 0.35 to 1.38 (the beech at seed 42), so `ARENA[0]` is 1.3792.

**Compaction's cadence.** A table is compacted where its projection two cycles on passes 0.85 of its rows (`TIGHT`), not half. Each compaction cost Telperion about 24 ms; see PROOF.md.

## 15. The lattice dormant where nothing shades (task 9, COUNCIL.md round 5 step 9)

`Light::dormant` holds where the extinction is zero, or where no PA has leaf area and the directions' shares, summed in order and rounded to f32, are one. Either way every node's light is one. A dormant cycle records the box, the grid, the deposit and the read, but none of the lattice's own dispatches. The read takes a literal 1.0 at each corner (`C.dormant`), the light the combine would have written. Every flag and refusal is recorded as before.

## 16. Upward sums and retirement reuse unchanged subtrees (task 10, COUNCIL.md round 5 step 10)

### 16.1 What changes in a cycle's inputs, counted first

Each laid axis's own part of each upward sum was held against what the sum last took of it, and the device counted what changed (`census::REUSE`, `reuse_is_counted`, commit `de0eb93c9`; raw `raw/t10/reuse`). Seed 42, grown as a build grows them, every count summed over the build's cycles. Shares are of the laid axes the sweeps visit (16.38M, 1.63M and 369k over the build) unless stated.

| Input, per axis a cycle | Telperion 82 | Oak 40 | Beech 40 |
|---|--:|--:|--:|
| Births (axes made, census) | 1.53M | 164k | 66.9k |
| Not taken before: births, and every axis after a compaction moved them | 2.67M (16.3%) | 153k (9.4%) | 88.1k (23.8%) |
| Light: lit now or when last taken | 4.23M (25.8%) | 343k (21.1%) | 136k (36.8%) |
| Wood upkeep changed | 3.84M (23.5%) | 275k (16.8%) | 124k (33.6%) |
| Frozen summary changed outside a compaction | 0 | 0 | 0 |
| **Totals' axes to take**, the changed and their ancestors | **7.42M (45.3%)** | **523k (32.1%)** | **237k (64.0%)** |
| Blocks holding one, of the blocks visited | 75.3% | 79.3% | 96.3% |
| Own lit presence or growth changed | 1.97M (12.0%) | 343k (21.0%) | 60.8k (16.5%) |
| **Subtrees' lit presence: axes to take** | **5.57M (34.0%)** | **522k (32.0%)** | **178k (48.1%)** |
| Blocks holding one | 75.2% | 78.7% | 96.3% |
| Growing buds; their own lit presence held from the last cycle | 4.98M; 45.2% | 368k; 0% | 177k; 42.5% |
| Woken buds (census) | 0 | 19 | 0 |
| Apexes shed by the balance (census) | 0 | 683 | 0 |
| Retirement: closed axes it carries up today, of the laid | 10.77M (65.7%) | 1.23M (75.6%) | 176k (47.6%) |
| **Retirement: closed this cycle**, of those | **1.20M (11.1%)** | **125k (10.1%)** | **32.1k (18.3%)** |
| Blocks holding one, of the blocks visited | 49.3% | 54.6% | 68.0% |

- **The upward sums.** The totals need not retake 55%, 68% and 36% of the axes they visit, but the paths from the changed axes to the root cross three quarters of the blocks (nearly all on the beech).
- **Retirement.** Nine in ten of the axes it carries up each cycle closed in an earlier cycle.
- **The normaliser's 1,024-bud blocks.** Nothing about them is cached, so nothing needs invalidating. On the lit oak no growing bud's own lit presence holds from one cycle to the next. On Telperion and the beech, 43-45% hold, but the cycle's new buds enter the subtree order between the old ones. That reason is inferred: no count shows how often a block's membership or ranks hold.

### 16.2 The upward sums: built, measured, stopped

**As built** (`de0eb93c9`, `hier_reuse.wgsl`):
- A flat pass held each axis's own part of the totals against what they last took: its wood's upkeep as an f32 (or its frozen upkeep), and whether it is lit now or was then. A second pass did the same for the subtrees' lit presence: the own lit presence bits and the growth flag.
- A changed or untaken axis was marked with its ancestors up to the first already marked, each block with it, by the cycle's number.
- `totals_main` and `sub_main`, and their folds, entered only marked blocks and took only marked axes. Each took its own value, then every child's cached or recomputed value in order; nothing was subtracted or added back.
- The walk and the split took every axis.
- A compaction cleared what the sums had taken.
- The cycle suite passed (contexts, caps, compaction every cycle against none).

**Measured** (stamped GPU ms, the two sums' kernels with their folds, then the two marking passes; one run each against the base's):

| | Totals | Subtrees' lit presence | Marking | Net |
|---|---|---|--:|--:|
| Telperion | 56.3 → 53.6 | 43.4 → 44.3 | +2.9 | **+1.1** |
| Oak | 10.6 → 10.5 | 8.1 → 8.2 | +0.6 | **+0.7** |
| Beech | 10.6 → 10.6 | 8.5 → 8.0 | +0.5 | **−0.1** |

**Stopped**, as the council's rule asks: invalidation and scheduling ate the saving.
- A sweep's cost is its blocks' depth loops, and three quarters of the blocks still hold a marked axis.
- Narrowing each marked block's depth range by a serial scan was tried; it cost more than it saved.
- The marks, the cached columns and the skips are deleted. The census keeps retirement's count only.

### 16.3 Retirement: each living carried up once

**Carried up once.** An axis's living and wait are carried up only in the cycle it closes (`R_CLOSED`).
- **Why that is exact:**
  - Nothing in a closed subtree grows, wakes or sleeps again.
  - A candidate stands beside an open axis, so no axis below a closed one is judged again.
  - What it carried up at its close is therefore what every later cycle would carry up.
  - The one word that changed later, a retired root's wait set to none, is now set by its verdict.
  - The retired and inside branches of `carried_up` could no longer be reached and are gone.
- **The sweep's blocks.** It enters only the blocks holding an axis closed this cycle. `close_main` stamps those blocks by cycle number, atomically (every axis closing in a block stamps it), in the words past the verdicts' log, which no compaction moves; a compaction renumbers no block.

**Each child's terms taken once.** The first build of the above saved only 15% (Telperion's living sweep 190.0 → 161.7 ms) while carrying up 11% of the axes. Its dispatches kept their 150-400 µs. `inside_main`, the same sweep shape, takes 20-30 µs a dispatch. Inferred cause: the unit and phytomer loops inlined in `carried_up` (`node_scale`, `presences`) held every block's workgroup to their registers. So each child's part in its bearer's living is now taken once, by `terms_main`, a flat pass after the woken buds are sized, for each axis whose bearer closed this cycle:
- its weight;
- the bearer's living before it was made;
- the cycles between.

`carried_up` folds these three in the children's order, with `shed::fades`' own expressions. They are three more `ret` columns per axis, moved by a compaction with the rest.

### 16.4 Exact

- **Node tables:** all 8 presets at their default age and seed, at caps 64, 128 and 1,024, on ten fresh contexts. All 240 hashes equal the post-task-9 hashes (`node_tables_are_hashed`; `raw/t10/t10-nodes-10b.log`, after the review's fix).
- **The cycle suite (42 tests):**
  - every verdict and living equals the CPU's;
  - node tables are the same on ten contexts and at three caps;
  - compaction every cycle matches compaction never.

### 16.5 Saving per phase

The base `1c69bf648` and this task's tip were run alternately, three runs each, every dispatch stamped (`the_roofline_is_measured`, `raw/t10/time`). Medians in ms; each side's three runs lie within 0.5% of each other.

| Phase | Telperion 82 | Oak 40 | Beech 40 |
|---|---|---|---|
| Retirement | 220.9 → **60.8** (−160.2) | 37.4 → **11.5** (−25.9) | 26.3 → **11.3** (−15.0) |
| of which: the living's sweep with its folds | 190.8 → 27.1 | 32.1 → 5.4 | 20.8 → 5.1 |
| of which: `terms_main` (new) | +3.0 | +0.6 | +0.6 |
| Balance and allotment | 291.3 → 291.5 | 72.4 → 72.4 | 62.8 → 62.4 |
| Every other phase | within 0.3 ms | within 0.2 ms | within 0.2 ms |
| **Stamped GPU time** | **1,066.6 → 907.0 (−15.0%)** | **222.2 → 196.4 (−11.6%)** | **154.4 → 138.8 (−10.1%)** |

## 17. The dominant kernels (task 11, COUNCIL.md round 5 step 11)

Every change below keeps each word: each schedules the same arithmetic in the same order. PROOF.md has the measurements.

- **A block's depth bounds, taken at once** (`hier_sweep.wgsl`, `retire_sweep.wgsl`, `fold.wgsl`). A block's sweep found each depth's members by two binary searches on its first lane, serial, before every depth. Now the block's lanes each take one depth's start at once, 64 depths at a time (`depth_starts`), and a folded block's 32 lanes take 32 (`fold_start`). The depths are walked in the same order, with the same barriers.
- **A bearer's children, four loaded at once** (`totals`, `sub`, the split's ranks and rests, retirement's `carried_up`). Each child's words were loaded and waited on before the next child's. Now four children's words load first, then each is added in the canonical order. No float is reassociated: the adds are written one by one, and a skipped non-positive light is skipped as before.
- **The units' emitted zone phases, cut where each is cheapest** (`units.wgsl`). The zone's order, phytomers and sleeping buds stay in `written_main`. The laterals and the sleeping buds' presences go to `sprouted_main`, and the laterals' presences to `placed_main`. A zone writes only its own records at places the scan left, so the phases may be cut anywhere between them. The cut costs one dispatch a round emitted. Section 14's fused entry held every zone to the registers of all its paths; this is inferred from its 1.66 ms dispatches, as no driver report was available.
- **The normaliser's block in the workgroup's memory** (`partials_main`). Every lane reads and draws (v/p)^ψ for the block's 1,024 buds at once; then each PA's thread adds its own in order from the workgroup's memory. `hm` stays at the add, so the product is rounded as before. A block takes one dispatch; the 256-bud runs and the partial sums they carried are deleted.
- **The settle's dispatches in shared passes** (`settle_run.rs`, `settle_place.rs`, `io::Pass::run_all`). The dispatches recorded between two scans or submissions share one compute pass, as the cycle's do. Each is its own stamped pass while a profile is on.
- **A dormant lattice's field and scratch stay at their least** (`build_light.rs`). Only the lattice's own dispatches read them, and section 15 records none of those on a dormant lattice.

## 18. Three small cuts (task 12, the host's rulings of 2026-10-12 on task 11's calls)

Each cut keeps every word: the node tables at caps 64, 128 and 1,024 hash as before (PROOF.md).

- **The inside marks, taken where they are read** (`plan_retire.rs`'s `Inside`, `Build::inside`). Retirement's inside sweep is recorded apart from the cycle, one list per hierarchy parity. The build replays it over the last submitted cycle's hierarchy, under that cycle's limits and given workgroups, before each compaction (and ahead of any move made with it, which carries the marks over) and before the settle. Its slots are the cycle's retirement's own, set at that cycle's point, so it reads what the cycle-start sweep read. Section 6 marked every cycle; the marks are the same because:
  - only the compaction (`compact.wgsl`, `retire_units.wgsl`) and the settle (`retired_root`) read a fate of `INSIDE`, or a root's `RETIRED` that a later root covers;
  - nothing is laid under a retired root;
  - the verdict judges no axis inside one: its bearer is closed, so it stands beside no open axis, and a pending wait is cleared the same either way.
- **A dormant lattice skips its deposit** (`plan_lay.rs`). The deposit's refusals (a leaf's area, a node's quanta, a leaf outside its box) guard only the lattice's input, which a dormant lattice never reads. Section 15's dormant cycle now records the box, the grid and the read; the quanta's clear and the deposit-ranges flag go with the deposit, and the quanta stay at their least.
- **Units rounds from the rounds used** (task 6's option 2). A job that begins a round with work left marks the round used (`atomicMax` into the lists' `T_USED`, the living apexes' and the wake's apart), tallied into the census. A cycle records the rounds the census it reads used, plus a stated slack: 2 for the living apexes, 3 for the wake, the most either rose over the two cycles a limit looks ahead, on every shipped preset at seeds 1, 7 and 42. The bound is 8. A job left past the cycle's rounds raises `UnitRounds`, and the build is refused by name; no build begins again. The first mark counted only advancing jobs and missed rounds that begin on a move or relay (FRICTION.md).
