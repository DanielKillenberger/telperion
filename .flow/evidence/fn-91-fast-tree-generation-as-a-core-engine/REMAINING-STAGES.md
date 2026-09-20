# Remaining delivery costs after shared rings

Read-only analysis of the committed task .6 browser samples. No new benchmark, implementation or acceptance. Warm medians below use the five recorded samples; subtraction is paired per sample before taking the median. Hypothetically removing one measured stage does not predict the behavior of a new implementation.

| Fixture | Completed frame ms | CPU wood preparation ms | Completed minus CPU wood preparation ms | Original / 10 target ms | Hypothetical remaining wood budget ms |
|---|---:|---:|---:|---:|---:|
| oregon-white-oak 1 | 320.1 | 170.8 | 149.8 | 157.0 | 7.2 |
| oregon-white-oak 7 | 378.8 | 200.7 | 177.8 | 186.2 | 8.4 |
| norway-spruce 1 | 313.9 | 141.6 | 171.9 | 762.9 | 591.0 |
| norway-spruce 7 | 293.6 | 130.6 | 161.0 | 738.4 | 577.4 |

Even a cost-free replacement of CPU wood preparation leaves all four fixtures above 100 ms in this arithmetic. The oak 10x requirement leaves little budget for replacement preparation if other stages stay constant. Task .7 therefore screens whether GPU positions are a useful next boundary; its 2x preparation threshold does not promise the parent target. Browser skeleton medians are 72.8/81.7 ms for oak and 56.8/54.7 ms for spruce; descriptor medians are 19.7/24.7 and 30.9/28.7 ms. Further work must be selected from measured integrated costs, including upload/completion overhead, after the probe.

CPU-owned outputs are separately qualified. The retained CPU builder regression checks do not establish a 10x improvement for consumers requesting CPU geometry. No website-only shortcut, curated seed cache or reduced specimen substitutes for engine performance.

Source inspection identifies a possible later profiling boundary, not an approved optimization or measured saving. `branching::generate` drains `Specimen::grow`, whose two `step` calls each solve structural radii and update identities; `finish` solves radii again, and `remap_after_shedding` builds a full identity map even when no nodes were shed. The local frontier, radius solve, identity/remap bookkeeping and compact path/sample/frame work should be distinguished by coarse stage measurements if integrated results still leave these stages dominant. No duplicated validation or identity work should be removed on this observation alone: incremental growth, stable identities, error behavior and changed-node dependencies remain contracts. Existing measured skeleton totals include all of this, but do not attribute its cost.
