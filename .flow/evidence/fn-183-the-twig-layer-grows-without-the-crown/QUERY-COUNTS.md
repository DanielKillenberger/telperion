# Crown queries by caller, master e12a9a28, 2026-10-02

Scratch instrumentation (`query-count.diff`): counters on `Envelope::radius_at`, the twig-layer stride check and its bisection in `local/planner.rs`, `Planner::run` calls and `scaffold::reach`. Growth is single-threaded, so the deltas are exact. One `growth_profile` sample per preset, native release, seed 1.

| Preset | `radius_at` | Stride checks | Radius in stride checks | Strides refused | Radius in bisection | Local axes | Scaffold reach radius | Rest |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| Oregon white oak | 183,563 | 67,559 | 67,546 | 868 | 34,719 | 11,738 | 305 | 80,993 |
| European beech | 293,726 | 104,082 | 104,081 | 1,800 | 71,999 | 16,116 | 378 | 117,268 |
| Silver birch | 1,267,571 | 43,952 | 320,952 | 1,190 | 616,959 | 12,655 | 426 | 329,234 |
| Ordinary | 115,529 | 23,020 | 23,020 | 0 | 0 | not counted | not counted | 92,509 |
| Telperion | 466,728 | 152,921 | 152,921 | 43 | 1,720 | not counted | not counted | 312,087 |

- The birch's stride checks average 7.3 radius queries each: `Curtain::admits` runs the lower-surface search per check.
- `Envelope::profile()` was called once per build on Ordinary and Telperion and never on the others, so shedding's outline is not in the rest.
- "Rest" is the scaffold's containment and attractor sampling, scheduling and the curtain outside stride checks; it was not split further.

## Corrections (Astra review, 2026-10-02)

- The birch's 7.3 radius queries per stride check are an average: `Curtain::admits` short-circuits for a point inside the outline, and only points outside run the band search.
- Terminal and leaf-bearing twigs are admitted separately (`local/advance.rs:193`) and are not in the stride or bisection columns; they sit in "Rest".
- Shedding's per-node outline queries (`branching.rs:299`) are in "Rest"; one `profile()` call per build does not exclude them.
- The direct build does not schedule, so scheduling contributes nothing to "Rest" on these presets.
- Timings above were taken with the atomic counters compiled in; they are not timing evidence.
