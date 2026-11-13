# fn-351 friction

## 13 Nov 2026 13:30 Zurich: a desk freeze inside the gate is counted as test time (Opus, gate-5 diagnosis)

- Doing: placing the two time-outs of the phase-top gate (`walks a_walk_between_species_changes_the_tree_by_degree`, `packed_leaf every_station_stands_inside_its_box...`).
- What hindered: the yield watcher froze the gate's scope (cgroup freeze) from 12:15:53 to 12:16:59, 66 s, and both tests were running then (12:15:47 to 12:17:47 and 12:15:25 to 12:17:45 in the gate's junit). nextest's per-test clock keeps counting through a freeze, so 66 s of the 120 s and 139 s cut-offs went to the freeze. Alone, with no PAUSE inside, the two take 25 s and 48 s on the merged branch and the same on all three parents. The same freeze hit one of my own alone-timings (fn-361's packed_leaf: 276 s wall, 98 s CPU, PAUSE 13:10:04 to 13:13:42), which had to be rerun.
- Cost: about 15 minutes to place two time-outs that were not regressions, and one gate red for nothing.
- What would remove it: `gate.py` reading the yield log for its own run window and marking any time-out whose window holds a PAUSE as "frozen, rerun" instead of a failure (or rerunning those tests first, as it does for failures), or the watcher pausing nextest's clock (unknown whether nextest offers this). A local-machine matter in part (the watcher is the owner's setup), so the host decides whether it is specced.
