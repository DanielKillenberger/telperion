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
