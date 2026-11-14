# fn-351 friction

## 13 Nov 2026 13:30 Zurich: a desk freeze inside the gate is counted as test time (Opus, gate-5 diagnosis)

- Doing: placing the two time-outs of the phase-top gate (`walks a_walk_between_species_changes_the_tree_by_degree`, `packed_leaf every_station_stands_inside_its_box...`).
- What hindered: the yield watcher froze the gate's scope (cgroup freeze) from 12:15:53 to 12:16:59, 66 s, and both tests were running then (12:15:47 to 12:17:47 and 12:15:25 to 12:17:45 in the gate's junit). nextest's per-test clock keeps counting through a freeze, so 66 s of the 120 s and 139 s cut-offs went to the freeze. Alone, with no PAUSE inside, the two take 25 s and 48 s on the merged branch and the same on all three parents. The same freeze hit one of my own alone-timings (fn-361's packed_leaf: 276 s wall, 98 s CPU, PAUSE 13:10:04 to 13:13:42), which had to be rerun.
- Cost: about 15 minutes to place two time-outs that were not regressions, and one gate red for nothing.
- What would remove it: `gate.py` reading the yield log for its own run window and marking any time-out whose window holds a PAUSE as "frozen, rerun" instead of a failure (or rerunning those tests first, as it does for failures), or the watcher pausing nextest's clock (unknown whether nextest offers this). A local-machine matter in part (the watcher is the owner's setup), so the host decides whether it is specced.

## 14 Nov 2026 07:00 Zurich: a still depends on the renderer's frame history (Opus, bark-distance evidence)

- Doing: reproducing `bark_distance`'s oak numbers in a scratch example (`examples/bark_distance_frames.rs`).
- What hindered: one extra Bare draw before the test's clay draw moved the 2x mask from 4,150 to 4,049 pixels and the 4x reading from 6.0038 / 16.25 to 5.9570 / 16.00; the same still drawn twice is byte-identical, so the state the renderer carries between frames (sun's map, wood surfacing) changes what a later frame draws. Matching the test needed its exact draw order.
- Cost: one rebuild and rerun, about 3 minutes.
- What would remove it: a doc line on `render` naming which renderer state carries from one still to the next, or a reset a measuring caller can call; the host decides whether it is specced.

## 14 Nov 2026 10:00 Zurich: the film's bounds pass builds every walk a second time (Opus, L5 film re-render)

- Doing: re-rendering the README film (`examples/film`, `release.json`) on the candidate for L5.
- What hindered: the film builds every tree once for the person's bounds (`film/bounds.rs`) before it builds them all again to draw, so a five-frame probe of segment 0 cost 55.7 min wall (bounds pass 3,289 s, of it Laurelin to Telperion 1,976 s). fn-310's bounds file could not be reused: it lives in another checkout's ignored `raw/` and the engine has changed since. Also, the brief's path `.flow/evidence/fn-351-the-release-candidate/film/raw/` does not exist on this branch (the spec's directory is `fn-351-020-the-release-candidate`), and a `raw/` one level under `film/` is not covered by `.gitignore`'s `/.flow/evidence/*/raw/`; the frames went to `fn-351-020-the-release-candidate/raw/film/` instead.
- Cost: about 55 minutes of queue time spent before the first full-film frame, and about 5 minutes finding an ignored path.
- What would remove it: the bounds pass kept with the build it measured (the drawing pass reusing its trees, or the bounds written while drawing in a two-pass order), and the brief naming the spec's evidence directory as it is on the branch; the host decides whether either is specced.
