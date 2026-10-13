# fn-255: what the CPU cycle costs against F4-Grow (worker, 2026-10-11)

**The target was the cycle within 1.5× of F4-Grow on every preset, with the same trees. This stopped early, before any lever was built, on a design call.** The whole-tree grower alone already costs 4.4× F4-Grow on Telperion, with the layout, light, balance and allotment all removed. Levers 1 to 4 act only on the work above that floor.

**How it ran:** default ages, seed 42, `--profile ci`, cores 0 to 3, one process at a time. Phases are the grower's own `Stage` reports. The census no longer builds the other deposit beside the one in use (`grow_census(.., beside: false)`), so ShadeField is the lattice alone. There is no sampling profiler: `perf_event_paranoid` is 2 and there is no sudo.

## Before

| Phase (s) | Telperion (82) | Spruce (40) |
|---|--:|--:|
| Units | 3.03 | 0.36 |
| Sketch (rough layout) | 2.06 | 0.53 |
| Balance (with `rows()`) | 1.69 | 0.45 |
| Allot | 1.07 | 0.29 |
| ShadeField (deposit, sweep, read) | 0.82 | 0.15 |
| Merge | 0.34 | 0.13 |
| Homes | 0.12 | 0.05 |
| ShadeRead | 0.03 | — |
| Settle: Shed, Assign, Place, Thicken, Scale and the rest | 1.29 | 0.71 |
| **Cycle total** | **11.44** | **2.77** |
| **F4-Grow total** (same phytomers grown) | **1.23** | **0.58** |
| **Target (1.5×)** | **1.85** | **0.87** |

## The floor: the same grower with no layout, light, balance or allotment

This was a throwaway probe, not committed. It is the cycle with no rough layout, so every gate the layout opens stays shut. Every lateral is still grown. The kept trees are identical to the cycle's.

| | Telperion | Spruce |
|---|--:|--:|
| Units | 3.11 | 0.35 |
| Homes + Merge | 0.40 | 0.15 |
| Settle (Shed 0.56 and 0.15, Assign, Place, Thicken, Scale, …) | 1.29 | 0.71 |
| **Total** | **5.43** | **1.33** |
| **Against F4-Grow** | **4.4×** | **2.3×** |

- **The target cannot be reached by trimming the light work.** Remove every pass levers 1 to 4 touch, and the whole-tree grower is still 4.4× F4-Grow on Telperion.
- **Two costs F4-Grow does not pay** account for the floor:
  - **Per-unit growth in one grower over the whole tree:** Units are 3.1 s for 18.2M phytomers. F4 grows the same units root by root (`Grower::bare`, one thread each, small vectors) in under 1.2 s, its settle included. Inferred cause: locality and the per-cycle work over 1.5M axes. Not profiled.
  - **The settle of the whole grown tree:** Shed alone, copying 18.2M grown phytomers down to 3.8M kept, takes 0.56 s. F4's roots are shed one at a time as they finish.
- **Above the floor,** the cycle's own work is 6.0 s on Telperion: layout 2.06, balance 1.69, allot 1.07, light 0.82, and growth's sketching share.

## What each lever can do, from the code

- **1. Persistent ordered adjacency.** `rows()` rebuilds the links and PAs of all n laid axes every cycle, inside Balance. `vigours_from` regroups children every cycle. Both can be kept across cycles to the bit. They are part of Balance's 1.69 s and Allot's 1.07 s.
- **2. Reused scratch.** Balance, allot and shed allocate n-sized vectors every cycle (light, cost, living, laterals, λ, lit, presence, ψ, sizes). Reuse is exact to the bit.
- **3. Working set.** It can be exact to the bit for the light, upkeep and living sums:
  - every descendant of an axis has a larger index, so the reverse loop adds each child's finished subtotal to its parent in decreasing index order;
  - a subtree with no living apex and no sleeping bud has a fixed subtotal, because its wood no longer grows;
  - so freezing that subtotal and adding it at the same place keeps every f32 sum identical.

  The same holds for allot, where such a subtree has no lit presence and takes no vigour. This needs no design call. The work is a compacted index space through `balance.rs`, `contract::shed::walk`, `allocation::vigours_from` and `sizes`. It would cut Balance and Allot, which run over every axis every cycle, roughly by the share of axes that stay open: 13 to 40% are active apexes in the busiest cycle (EARLY.md). Memory falls only if retired axes' phytomers also leave the grower, which reaches the settle (see 2 below).
- **4. Incremental layout.** It is already incremental, read from `draw.rs`:
  - `draw_bent` continues from `pencil.units` and `pencil.laid`;
  - `sketch_round` frames only the axes made in the cycle;
  - `sketch_end` deposits only the phytomers laid in it.

  Nothing is revisited. Its 2.06 s is laying every grown phytomer once, 18.2M of them, about 113 ns each on 4 cores. F4 never lays the 14.4M that are shed: its final lay places the 3.8M kept in 0.17 s.

**Best case of 1 to 3, inferred:** Balance and Allot from 2.76 s to perhaps 0.5 s, with Telperion at about 9 s. That is still about 7× F4-Grow.

## Design calls this needs (for the host)

1. **The grower's shape.**
   - Reaching F4-Grow's cost on the CPU means growing with F4's locality: subtrees each in a small grower, retired whole once nothing in them lives. The phase order stays the same, with the per-cycle sums crossing subtree boundaries.
   - That is the stepped executor's decomposition, whose handover COUNCIL.md lists for deletion.
   - Does the one cycle keep a subtree-local executor, or accept the whole-tree grower's floor (4.4× on Telperion, 2.3× on spruce)?
2. **What may leave the grower before the end.** Retiring a dead subtree's phytomers early is the memory fix: Telperion's 5 GB against 1.2 GB. But the settle reads the whole grown tree (shedding, attachments, girth of shed wood where a PA keeps it). Which settle inputs may be summaries is a design question. Exact equality is not the issue; the issue is what the settle must still see.
3. **Laying what is shed.** The rough layout of laterals that will be shed is 2.06 s of work F4 never does. While they live, their tips and leaves are read by light. With leaf area 0 (every preset but the oak and the ordinary tree), nothing reads their leaf positions. The council rules out an engine switch, so whether that work stays is the host's call.

## Committed with this report

`grow_census` takes `beside`, so phase timings no longer carry the second lattice. No lever was built.

# Host decisions of 2026-10-11, built so far (worker, 2026-10-11)

The host's decisions: no subtree-local executor; records in subtree (DFS) order for locality; a shed subtree retired at its shedding cycle with a summary on its parent; laying shed laterals kept. Levers 1 to 3 are built and exact to the bit. Decisions 1 and 2 stop on two questions below.

## Levers 1 to 3: built (`grow/cycle/working.rs`, `grow/cycle/passes.rs`)

The one cycle's balance and allotment visit only the **open** axes: those on the way from the root to a living apex or a sleeping bud.

- **Kept across cycles:** the links, and each axis's children by index and by node.
- **No longer rebuilt or sorted:** the per-cycle `rows()` rebuild, and the global child sort in `allocation::split`.
- **Reused:** the passes' lists.
- **A closed subtree's upkeep is frozen once.** It is taken as the whole tree's pass takes it: own, then each child's subtotal, the last child first. Every descendant has a larger index, and a closed subtree lays no more wood, so every sum is the whole tree's to the bit. A closed subtree's light and lit presence are none.
- **Proved by `the_working_set_grows_the_whole_trees_tree`:** the oak, which sheds by its balance and sizes by its light, grows through the one cycle to the whole-tree engine's tree, byte-equal at ages 24 and 30, seeds 1, 7 and 42.
- **The open set** is 19% of the laid axes at Telperion's end: 248k of 1.33M, against 219k live apexes.
- **The stepped executor's path is untouched.** It still takes the `rows()` route until R3b deletes it.

### Before and after, per phase

Each side ran twice, interleaved under the same load (load average 3; base binary from `cd947430b`).

| Phase (s) | Telperion before | Telperion after | Spruce before | Spruce after |
|---|--:|--:|--:|--:|
| Units | 3.11–3.81 | 3.13–3.28 | 0.35–0.36 | 0.36–0.37 |
| Sketch | 2.00–2.36 | 2.15–2.21 | 0.51–0.52 | 0.57 |
| Balance | 1.59–2.01 | 1.26–1.29 | 0.36 | 0.33–0.34 |
| Allot | 1.06–1.22 | 0.59–0.61 | 0.27–0.28 | 0.12 |
| ShadeField | 0.81–0.85 | 0.83 | 0.15 | 0.15 |
| Settle (Shed, Assign, Place, Thicken, …) | 1.3–1.4 | 1.3 | 0.7 | 0.7 |
| **Total** | **10.92–13.14** | **10.43–10.62** | **2.57–2.60** | **2.50–2.52** |
| **Peak memory (MB)** | **5,052–5,060** | **5,234–5,237** | **1,307–1,308** | **1,358** |

- **Gains:** Allot halves and Balance falls by a fifth; together they save about 0.8 s on Telperion.
- **Memory rises** by 50 to 180 MB: the persistent links and child lists, which nothing yet retires.
- **The whole is still dominated** by Units, Sketch and the settle, which decisions 1 and 2 address.

## Decision 2 (retirement): the question it raises

How big the records are:

| Record | Bytes |
|---|--:|
| Phytomer | 136 |
| Axis | 264 |
| Draws | 152 |
| Pencil | 176 |

For Telperion's 18.2M grown phytomers that is about 2.5 GB, plus about 0.9 GB for its 1.5M axes with their draws and pencils. Its kept tree is about 0.5 GB.

### What the settle reads of a shed subtree, from the code

| Reader | What it reads | Exact summary on the parent? |
|---|---|---|
| `shed::fades` (every species) | The subtree's lived time and its living presence, from its unit presences, sleep, vigour and its parent's node scale. Folded into the parent by `max`. | **Yes.** Once the subtree closes these are final (`presence::assign_one` is per axis), and `max` is order-free. `shed_given`'s `Given<Living>` already carries them. Its drop at the final age is known when it closes, because the final age is in the request. |
| The balance's upkeep (every species with a balance hazard) | `upkeep × wood` of every axis it ever grew, for the whole run. `pencil.wood` never falls, so a shed branch keeps charging its bearer. | **Yes**, as the frozen subtotal (lever 3). This is today's behaviour. The host's premise that it "drew upkeep only while alive" does not hold in the code. Changing that would change the oak's and the ordinary tree's trees. |
| `girth::disused::attachments` and `leaf_partials` (`keeps`: any `retained` or `leafGirth`; the oak and the ordinary tree) | The leaf term's mean over **every grown phytomer**, as an f32 sum in the order axes and phytomers stand. Also each shed child's **radius as grown** under that final mean, `section(pipe × ripe × lit(light)^χ × mean, e)` summed over its subtree. | **No closed summary is exact.** The mean is known only at the end, and the sum's order interleaves shed and kept axes. Exact retirement needs a per-phytomer record of 8 bytes: the leaf-free pipe product as f32, and the light term. That is about 146 MB at Telperion's 18.2M, against 2.5 GB today. A per-subtree, per-exponent moment `Σ (pipe·lit)^e` is exact only in real arithmetic, and would move the oak and the ordinary tree by f32 order. |

**The question:** for the two presets that keep shed pipes or leaf girth (the oak and the ordinary tree), which should retirement keep?

- **(a)** The 8-byte per-phytomer record, which is exact.
- **(b)** A per-subtree moment summary, which is close but moves those two trees by f32 order, checked with the agreement tests.

Restricting retirement to species without `keeps` would be the engine switch COUNCIL.md forbids. For the other six presets retirement is exact either way.

## Decision 1 (subtree order): not started

There is no profile of where Units' 3.1 s goes. `perf_event_paranoid` is 2 and there is no sudo (FRICTION.md).

**Locality is a hypothesis, not a measurement.** The live apexes are already visited in near index order, because new axes append in live order. The per-unit code is the same as F4's roots, which grow the same units in under 1.2 s.

**Reordering the records into DFS order each cycle breaks byte equality.** It changes the order of the per-PA size sums and of the settle's sums, so those would have to be taken in a canonical rank to stay exact.

**What would let me proceed:** a sampling profile, with `kernel.perf_event_paranoid = 1` on this machine, to show whether Units is bound by memory traffic (each phytomer is 136 bytes, and every axis's vectors grow by push) or by the work itself, before the reorder is built.

# Decisions 1 and 2 built, with the spike's contracts (worker, 2026-10-11)

These follow the host's decisions of 2026-10-11: (a) exact retirement; timers before any reorder; the upkeep finding written up and left unchanged. They also take in fn-256's two contracts from COUNCIL.md's "Decided by the spike".

## Decision 1: where Units' per-unit gap comes from

Per-thread timers were put inside Units, in `Shoot::advance`, `grow_unit` and `sprout`. These are the same code F4's roots run. The figures below are before retirement.

| Telperion (82) | ns per apex | ns per node | grow_unit (thread-s) | sprout (thread-s) |
|---|--:|--:|--:|--:|
| F4-Grow's roots | 673 | 184 | 2.76 | 0.23 |
| The cycle | 2,961 | 809 | 11.49 | 0.82 |

Spruce reads 248 against 695 ns per node.

**The gap is all in emitting the units.** It comes from the same code, so it is memory, not work. The rusage of the two runs shows it:

| Telperion | user s | sys s | Minor faults | Major faults | Peak |
|---|--:|--:|--:|--:|--:|
| F4-Grow | 5.45 | 0.90 | 321k | 0 | 1.2 GB |
| Cycle | 24.1 | 6.4 | 1.33M | 1,463 | 5.2 GB |

- **How the two grow:** F4's roots grow each chain whole while it is hot and free each root as it settles. The cycle pushed into every axis's cold phytomer and draw vectors once a cycle, in a heap that only grew.
- **Swap:** the major faults are swap. This machine had 19 GB of swap in use by other sessions.
- **The remedy the timers name** is to let shed wood go (decision 2), not to reorder the records. With retirement, Telperion's cycle drops to 545 ns per node, 349k minor faults, 0 major faults and 1.9 s of sys time.
- **The remaining 3× against F4's roots** is the whole grower visiting every living apex once a cycle in a tree it holds. That is the shape the light coupling needs. No reorder was built.

## Decision 2: retirement (`grow/cycle/retire.rs`)

**When a lateral's subtree closes** (nothing in it lives or sleeps), it is evaluated once by the settle's own code:

1. `presence::assign_living` runs on a copy, each woken bud by its kept share.
2. `shed::subtree_living` takes the copy with the lateral as root.

**Where its fade at the tree's age is none,** the settle would drop it, so it is retired:

- its phytomers and draws are let go;
- its living stands for it in its bearer's fade (`shed::shed_past`, through the existing `Given<Living>`);
- its upkeep stays frozen in the balance (lever 3).

**Where it is kept,** its living is remembered, and an ancestor's evaluation stops there. Evaluations run on every core. Newly closed axes are tracked from one cycle to the next, never scanned.

**What the settle reads of a retired subtree**, named from the code:

- **The fade:** the root's lived time and living presence. They fold into the bearer by `max`, so the order does not matter, and the summary is exact.
- **The balance:** the upkeep subtotal, frozen as charged today.
- **The girth pass, where any row keeps shed pipes or leaf girth:** each phytomer's cycle, scale, size and light, as the f32s it reads (`girth::Grew`, 16 bytes), plus the axis's vigour. The host's decision named 8 bytes: the pipe product and the light term. That product needs the axis's base, which runs through ancestors not yet assigned when the subtree retires. So the four inputs are kept, and the settle takes bases, the leaf mean and the radii from them exactly as it does today (`girth::disused`, `*_past`). The record holds nothing where those rows are zero.

**Exact to the bit.** `the_working_set_and_retirement_leave_the_tree_as_it_was` grows the oak (which keeps leaf girth, so its records are used), the spruce and the beech at 40, seed 7. It retires 16k to 51k subtrees and gets the reference tree (every axis open, none frozen or retired) byte-equal.

## The spike's contracts, in the one cycle only

**One reduction order.** Every subtree sum (light, upkeep, living, lit presence) is the axis's own value, then each child's subtotal in the order the children stand on it: by node, then index, a continuation or relay at the tip last.

- The normaliser's per-PA sums run over the growing buds in subtree preorder.
- They are cut into blocks of 1,024 buds. Each block is summed in order, and the blocks are then added in order.
- The blocks are over buds, not axes, so the partition does not depend on what is frozen or retired, and the device can take the same blocks.

**What stayed as it was:**

- The records are not physically reordered on the CPU. Sums follow the canonical order through the kept child lists.
- The settle's sums (leaf mean, thickening) are as before. COUNCIL.md names balance, allocation and normalisation.

**Viability.** The one cycle reads each PA's viability as the f32 its table holds and shades it as the device does: `exp2(log2 v · exp2(−φ log2 l))`, with the contract's f32 `exp2` and `log2`. A whole cycle's threshold is taken from `p`, a part's from `−log2 p` (`decide::shaded`, `survival32`).

- **Precision:** the shipped viabilities are at most 0.999, so f32 holds `1 − p` to about 6e-5 of itself, and shade only moves `p` away from one.
- **Where it would bite:** settings much nearer one would lose the resolution `neg_log2_of` kept (fn-254). The walks were not run here.
- **Scope:** the stepped executor and F4's GPU roots keep the f64 path, because the render gate checks one against the other. Both are retired by R3b.

**The light deposit** is the integer one the host adopted (2^-20 m², u32). It is order-free.

## The kept trees, against F4

**Kept phytomers equal F4's on every preset** at its default age, seed 42 (raw/cost-final).

**Agreement by lineage with today's whole-tree engine** (`agreement_with_todays_tree`, ignored; it prints):

| Tree | Unmatched | Tip | Scale | Radius |
|---|--:|--:|--:|--:|
| oak 24 s7 | 0 | 3.6e-5 | 1.7e-3 | 1.7e-5 |
| oak 40 s1 | 20 of 357,601 | 1.5e-3 | 0.31 | 2.1e-3 |
| beech 24 s7, 40 s1 | 0 | ≤4.9e-6 | ≤7.6e-5 | ≤5.1e-6 |
| spruce 24 s7, 40 s1 | 0 | ≤9.7e-6 | ≤1.6e-4 | ≤9.5e-6 |

The oak moves most because its light and balance amplify f32 order, as fn-254 measured.

**Stills: every preset, whole, mid and close, seed 42** (`raw/stills-final/*-stack.png`, all viewed). The renderer was routed through the cycle by a throwaway patch (`raw/stills-route-cycle.patch`, not committed).

- The cycle cannot be told apart from F4's own tree at any view.
- Against F4's build as it ships, beech, birch, Laurelin and Telperion differ by up to 86% of pixels at close range, by more than 16 levels. The wood is the same. **Only the leaves' arrangement differs.**
- **The cause is F4's fine-root marks** (`grow_marked`): a conversion seam the cycle does not make. F4's own tree rendered without its marks differs from the cycle by at most 0.8% of pixels by more than 16 levels:

| Pixels over 16 levels, cycle against F4's tree unmarked | whole | mid | close |
|---|--:|--:|--:|
| european-beech | 0.00% | 0.00% | 0.00% |
| norway-spruce | 0.39% | 0.77% | 0.62% |
| silver-birch | 0.01% | 0.01% | 0.01% |
| laurelin | 0.47% | 0.37% | 0.33% |
| telperion | 0.42% | 0.38% | 0.30% |

The oak (2.5 / 5.9 / 7.0%), the ordinary tree (0.03 / 0.06 / 0.06%) and the palm (0%) carry no marks in F4, so they compare directly.

When R3b retires the fine roots, the species F4 builds apart will lose those seams. Their leaves will then sit as the unmarked column shows.

## The per-phase cost table after levers 1 to 3 and decisions 1 and 2

Default age, seed 42, 4 cores, one process per build (`raw/cost-final`). Peak memory is per process.

| Preset (age) | F4 Skip s | F4 Grow s | Cycle s | Peak MB: F4 Skip | F4 Grow | Cycle (before, after) |
|---|--:|--:|--:|--:|--:|--:|
| norway-spruce (40) | 0.50 | 0.59 | 2.61 | 523 | 620 | 1,302 → 1,021 |
| telperion (82) | 0.67 | 1.30 | 9.86 | 1,043 | 1,239 | 5,058 → 2,239 |
| laurelin (80) | 0.64 | 1.17 | 8.91 | 977 | 1,154 | 4,604 → 2,046 |
| european-beech (40) | 0.08 | 0.09 | 0.34 | 101 | 103 | 210 → 150 |
| oregon-white-oak (40) | 1.09 | 1.08 | 0.82 | 547 | 549 | 419 → 270 |
| date-palm (40) | 0.00 | 0.00 | 0.00 | 12 | 11 | 11 → 12 |
| silver-birch (40) | 0.18 | 0.21 | 0.71 | 182 | 193 | 362 → 252 |
| ordinary (40) | 0.48 | 0.48 | 0.35 | 206 | 206 | 154 → 112 |

**The cycle's phases (s), before (EARLY.md) and after:**

| Phase | Telperion before | Telperion after | Spruce before | Spruce after |
|---|--:|--:|--:|--:|
| Units | 3.03 | 2.38 | 0.36 | 0.33 |
| Sketch | 2.06 | 2.05 | 0.53 | 0.59 |
| Balance (with retirement's evaluation) | 1.69 | 2.32 | 0.45 | 0.55 |
| Allot | 1.07 | 0.59 | 0.29 | 0.13 |
| ShadeField (integer deposit) | 0.82 | 1.10 | 0.15 | 0.19 |
| Shed | 0.59 | 0.23 | 0.18 | 0.12 |
| Assign | 0.31 | 0.12 | 0.08 | 0.06 |
| **Total** | **11.44** | **9.86** | **2.77** | **2.61** |

- **Memory:** the cycle's peak halves on the Two Trees, and it is below F4's on the oak and the ordinary tree.
- **Time:** the cycle is faster than F4 on the oak and the ordinary tree. On the rest it is 3.5 to 8× F4-Grow.
- **Where that time goes:** Units, Sketch (laying the shed laterals, kept by decision 3) and the light, each over the whole grown working tree. The host has placed full-resolution speed with fn-256.
- **Retirement's own evaluation is in the Balance stage:** about 1.2 s on Telperion for 1.0M closed subtrees.

## The upkeep finding: shed wood keeps charging its bearer (model question, not changed here)

`pencil.wood` never falls, so a lateral the balance sheds keeps adding its `upkeep × wood` to its bearer's cost for the rest of the run. The working set freezes that subtotal exactly as charged. COUNCIL.md's premise that "it drew upkeep only while alive" does not hold in today's model.

**Measured with a throwaway switch** (retired subtrees stop charging; `raw/upkeep.txt`, reverted):

| Tree | Kept (charged → not) | Height m | Crown x × y m | Live crown base m | Unmatched nodes |
|---|---|---|---|---|--:|
| oak 40 s1 | 357,613 → 363,858 (+1.7%) | 10.60 → 10.60 | 9.84 × 7.97 → 9.84 × 8.35 | 0.14 → 0.15 | 6,483 |
| oak 40 s7 | 488,475 → 498,035 (+2.0%) | 11.00 → 11.00 | 9.97 × 10.32 → 10.31 × 10.32 | 0.38 → 0.38 | 10,378 |
| oak 40 s42 | 354,122 → 361,227 (+2.0%) | 9.94 → 9.94 | 9.89 × 9.20 → 9.88 × 9.20 | 0.49 → 0.49 | 7,563 |
| oak 80 s1 | 1,878,375 → 2,076,148 (+10.5%) | 16.31 → 16.32 | 18.79 × 17.23 → 23.49 × 17.20 | 5.11 → 2.37 | 254,033 |
| ordinary 40 and 80 s42 | unchanged | unchanged | unchanged | unchanged | 0 |

The ordinary tree's tree is the same to the bit either way, despite its upkeep of 0.006 and hazard of 0.049. Inferred, not traced: no lateral's remembered balance falls below its tolerance in either run, so `hazard × max(0, tolerance − memory)` is none, and the upkeep never reaches a decision. The oak at 80 keeps 10.5% more phytomers, its live crown base falls from 5.1 to 2.4 m, and its crown widens from 18.8 to 23.5 m along x. The host will spec this separately.

## Gates (2026-10-11, on `ae7ba4fee` and this report)

| Gate | Command | Result |
|---|---|---|
| Workspace gate, renderer excluded | `cargo test --profile ci --workspace --exclude telperion-render --no-fail-fast` | 1,068 passed, 0 failed |
| Renderer, under the GPU lock | `cargo test --profile ci -p telperion-render --no-fail-fast` | 242 passed, 0 failed, 40 ignored |
| Package | `npm test` | 14 files, 155 tests passed |

Every shipped preset's kept count equals F4's, and the stills are viewed (above). The zero crossings hold: `zero_and_near_zero_grow_the_same_tree`, extinction and response rows at 0 against 1e-30, on both deposits. The per-phase cost table is above.
