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

## 14 Nov 2026 09:20 Zurich: measuring one budgeted test the gate's way needs gate.py's private steps by hand (Opus, native peak)

- Doing: measuring the six over-budget tests' peaks the way `scripts/gate.py` does, filtered, without running the full gate.
- What hindered: the gate's per-test time-outs live in a tool config `gate.py` writes to `target/gate/budgets.toml` at run time; a filtered `cargo nextest run --profile gate` without it cuts every test at the profile's 120 s, so `flowers_cross_the_binding...` (budget 239 s) timed out and its peak was lost. The runner, `GATE_REPORT` and the tool config each had to be read out of `gate.py` and set by hand. Also: the brief named `.flow/evidence/fn-351-the-release-candidate/`, which does not exist; the spec's directory is `fn-351-020-the-release-candidate`.
- Cost: one 7-minute run whose sixth test was lost, plus about 10 minutes reading `gate.py`.
- What would remove it: a `gate.py --only <filterset>` (or `--measure`) that runs a named set through the same runner, tool config and report, and prints each test's peak and time against its budget.
## 14 Nov 2026 10:00 Zurich: the film's bounds pass builds every walk a second time (Opus, L5 film re-render)

- Doing: re-rendering the README film (`examples/film`, `release.json`) on the candidate for L5.
- What hindered: the film builds every tree once for the person's bounds (`film/bounds.rs`) before it builds them all again to draw, so a five-frame probe of segment 0 cost 55.7 min wall (bounds pass 3,289 s, of it Laurelin to Telperion 1,976 s). fn-310's bounds file could not be reused: it lives in another checkout's ignored `raw/` and the engine has changed since. Also, the brief's path `.flow/evidence/fn-351-the-release-candidate/film/raw/` does not exist on this branch (the spec's directory is `fn-351-020-the-release-candidate`), and a `raw/` one level under `film/` is not covered by `.gitignore`'s `/.flow/evidence/*/raw/`; the frames went to `fn-351-020-the-release-candidate/raw/film/` instead.
- Cost: about 55 minutes of queue time spent before the first full-film frame, and about 5 minutes finding an ignored path.
- What would remove it: the bounds pass kept with the build it measured (the drawing pass reusing its trees, or the bounds written while drawing in a two-pass order), and the brief naming the spec's evidence directory as it is on the branch; the host decides whether either is specced.
## 14 Nov 2026 12:40 Zurich: a 20 s measurement waited ten minutes and more for the queue (Fable, Two Trees time)

- Doing: timing one Laurelin build at 80 (`early three laurelin 80 7 cycle`, 8 GB asked) to split the film's grow segment.
- What hindered: five heavy scopes ran at once (a full gate, a frames sweep, two other measurement loops); the queue showed room of -10 GB for 8, so the run sat from 12:29 and its seconds, taken beside a full gate at load 20 to 30, are noisy.
- Cost: over 10 minutes of waiting for one 20 s run; every later before/after run pays the same.
- What would remove it: nothing in the repository; the host staggering measurement dispatches so that timed runs do not share the desk with a full gate.

## 14 Nov 2026 13:45 Zurich: the desk at load 112 froze the measurement it had just admitted (Fable, Two Trees time)

- Doing: the before/after runs and the narrow tests for `begin_whole`, queued as one 8 GB batch and one 4 GB batch.
- What hindered: the queue showed room between -15 and 3 GB from 12:50 to 13:37, with four scopes holding claims (a full gate, two measurement loops and a frames sweep frozen since 09:35 on 36 MiB of its 6 GB claim). The 8 GB batch was admitted at 13:37 at load 67 and its scope was frozen by the watcher within minutes; at 13:44 the load was 112, most of it another project's test suites outside the queue. A waiting job cannot be told from a frozen one except by `systemctl --user status`.
- Cost: about 55 of the task's 120 minutes waiting; three queued jobs stopped and re-queued to ask for less.
- What would remove it: a local matter, reported and not specced: timed runs need a desk that is not running another project's suites. In the repository, `telperion-heavy` could print "frozen" for its own scope, and a frozen scope's unused claim could stop counting against the room.

## 14 Nov 2026 15:25 Zurich: a three-minute test took thirty, frozen for twenty-six (Fable, Two Trees time)

- Doing: `every_preset_grows_the_same_tree_again_and_on_any_cores`, admitted at 14:58 on the host's word that the desk was free.
- What hindered: the scope was frozen 18 s after it started, at load 30 to 42 from another project's suites, and thawed at 15:24. The test reports 1,815 s.
- Cost: 26 minutes, and the timed runs behind it started half an hour late, at load 7 to 15 instead of under 4.
- What would remove it: as the 13:45 entry; a test's own clock cannot tell a freeze from work, so a gate budget read during a freeze would fail a healthy test (the 13 Nov 13:30 entry is the same thing).
