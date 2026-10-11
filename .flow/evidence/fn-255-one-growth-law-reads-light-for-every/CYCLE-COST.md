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
