# fn-257 R6: the tiny dispatches, bounded (worker, 2026-10-16)

GROWTH-PROFILE.md put 12,000-14,700 dispatches under 20 µs at 60-74 ms a tree, and the levers line estimated merging them at 30-40 ms. Those milliseconds are the stamped build's, where every dispatch runs in a pass of its own (5-13 µs a launch on this card, `raw/profile/floor.md`). Unstamped, consecutive dispatches share a compute pass (`build_plan.rs`, the recorder's runs), where a dependent dispatch costs about 1.7 µs: 13,000-16,000 dispatches a tree are about 22-27 ms of the 176-358 ms growth, the most any batching can take back.

What they are (oak at 40, cycle 20, `raw/r5/oregon-white-oak-1-stamps.csv`): the hierarchy's step, a round's placing passes each `target`, a scan, `join_place`, `decide`, a scan, `assign`, `follow` (about 84 a cycle); the units and wake rounds, `decide`, `zones`, `after`, `written`, `sprouted`, `placed` per round (about 90); the layout's generations (about 35); the folds and sums by level. Each reads what the one before wrote across the whole table, so none can share a dispatch with its neighbour without a barrier across workgroups, which WebGPU has only between dispatches. The passes already share a compute pass where no copy, clear or sizing point stands between them; the 707 sizing points a tree (`vm_main`, each with the copy of its sizes into the uniforms) break about 700 passes, at most about 5-8 ms.

Two levers remain, each a kernel's redesign rather than a batching:

- **Folding narrow levels into one workgroup**, as `fold.wgsl` already folds the sums' narrow block levels: a hierarchy level or a units round whose rows fit one workgroup runs its whole step chain in one dispatch with workgroup barriers. Bound: the share of the 22-27 ms on levels that narrow; not measured.
- **Placing passes stopped once a round has placed everything**: the passes after an empty one are recorded and dispatched anyway (`plan_hier.rs`, the `for p in 0..bound.passes` loop, sized by the round, not by what is left). A sizing point between passes could size the next from the last pass's unplaced count. Bound: up to about 70 dispatches a cycle, roughly 5 ms a tree, and one more point a pass.

Neither is taken in this task: the owner put the kernels' own speed after the release, and the measured bound (about 22-27 ms, 5-10 % of growth) is under the estimate the owner accepted the levers on. The task is held for the host's call.

## Task 4: the fold probed, built and dropped (worker, 2026-10-16)

**The probe.** Under a test-only switch every dispatch of the hierarchy's step was followed, in its compute pass, by an empty dispatch of the same workgroups bound to a scratch word. The added time is what that many launches cost, the most a fold could save. The tree's records were equal with and without it. Oak at 40, seed 1, warm, ten alternated pairs, one process, quiet card:

| Replayed | Base | Probe | Paired difference |
|---|---|---|---|
| Every balance and allotment dispatch | 228.2 ms | 235.2 ms | 6.9 ms (sd 1.1) |
| The hierarchy's step alone | | | 5.7 ms (sd 2.3) |

The spruce, its first pair an outlier: about +11 ms (balance) and +8 ms (step). The step's bound, 5.7 ms, is past twice the spread (4.6 ms), so the fold was built.

**The fold.** A placing pass whose new axes and blocks fit one workgroup ran in one dispatch, `pass_main`, with workgroup barriers between its steps and its two scans in the workgroup. Two things about it:

- The host gives each dispatch the workgroups its bounds reach (`given.rs`). Near the threshold both the folded and the separate path are dispatched, so each ran on a slot of its own whose new axes are none unless the device finds the pass its own: two slots a round where there was one.
- Every shipped preset's tree was the same folded and apart (`the_fold_leaves_every_preset_s_tree_the_same`).

Paired the same way, folded against apart (a negative difference is the fold slower):

| Preset, thresholds (new axes / blocks) | Folded | Apart | Paired difference |
|---|---|---|---|
| Oak, 1024 / 16384 | 237.0 ms (sd 2.3) | 231.4 ms (sd 1.6) | -5.6 ms (sd 2.4) |
| Oak, 256 / 1024 | 234.8 ms (sd 1.1) | 231.4 ms (sd 3.4) | -3.5 ms (sd 3.4) |
| Spruce, 1024 / 16384, pairs 2-10 | | | about -11 ms |

The pairs (folded, apart, in ms), oak at 1024 / 16384: 235.0/232.0, 235.0/229.7, 236.5/231.3, 241.9/232.9, 241.1/230.7, 235.1/229.5, 236.0/230.3, 236.7/230.2, 235.8/232.6, 237.0/234.9. Spruce: 615.8/395.6 (the cold outlier), 402.1/394.7, 410.1/413.1, 417.9/406.2, 421.4/409.1, 436.1/411.2, 421.1/413.1, 434.8/409.2, 414.2/413.7, 417.4/405.6. The probe's pairs and the step-only means were printed by the test and not kept; its run is reproducible from the experiment branch (`PROBE_PRESET`, `PROBE_PAIRS`, `PROBE_STEP`).

**Decision: dropped.** The fold costs more than the launches it removes. One workgroup walks up to a thousand axes and their ranks serially where the separate dispatches spread them over the card, and the second slot a round adds its sizing words to every cycle. No production code changed. The probe and the fold are kept on the branch `fn-257-r6-fold-experiment` (8069cec80) for the record. The other lever above, placing passes stopped once a round is placed, stays unmeasured.
