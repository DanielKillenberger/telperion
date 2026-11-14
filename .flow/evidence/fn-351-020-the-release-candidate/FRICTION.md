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
## 14 Nov 2026 12:28 Zurich: the first build sat frozen behind a tree grown outside the queue (speed round 2)

- Doing: the first build of `early` in the new `speed2` worktree, through the queue (`telperion-heavy 4 --threads 4`), admitted at 12:22.
- What hindered: the yield watcher froze every queue scope at 12:21:59 (`PAUSE (cpu 7 cores)`) and held them: a `target/ci/examples/film_grow laurelin 7 1 80 split` was running outside the queue at 2.2 cores beside a `walk_leaves` test binary run directly, which with the desk's own two cores stays over the watcher's four. The build had 2.4 CPU seconds after six minutes. `perf` is not installed either; the profile is taken with `samply` (present) and counts with `count.py` beside this file (`perf_event_open`, user-space instructions and cache misses).
- Cost: 6 minutes at the time of writing, the whole of the round's work so far; reading the grower's source filled it.
- What would remove it: every long tree run started through the queue, so the watcher counts it as the queue's own and not as the desk's; or the watcher naming the foreign process in its PAUSE line.
## 14 Nov 2026 12:39 Zurich: seventeen minutes in the queue bought fifteen seconds of build (speed round 2)

- Doing: the same two builds (the `early` example, and a second with line tables for the profile), both admitted, both frozen.
- What hindered: the watcher thawed the queue once, 12:33:46 to 12:34:01, and froze it again (`PAUSE (cpu 13 cores)`). The foreign load since 12:22 has been other sessions' work started outside the queue: `film_grow laurelin`, `catalogue_identity` test binaries from the `count` and `spruce` worktrees at two cores each, `walk_leaves` and `walk_sizes` from `cand`, a `node conformance/gate.ts`, and the vault's indexer. Each needs only to hold the desk over four cores for ten seconds in any minute to keep every queued job frozen, so the sessions that use the queue wait on the sessions that do not.
- Cost: 17 minutes of a two-hour round with no profile and no base binary yet; the time went to reading the grower and writing the measurement scripts (`count.py`, `speed2-ab.sh`, `speed2-ab.py`).
- What would remove it: the watcher counting only the owner's own work as foreign (the desktop session's cgroups), not other agents' test binaries; or every agent's cargo runs going through the queue, as this round's do.
## 14 Nov 2026 13:21 Zurich: no profiler on the desk, and the queue full of claims (speed round 2)

- Doing: the profile of the oak at 40 on one thread, then the first candidate's build.
- What hindered: (1) `perf` is not installed, and `samply` (0.13.1) cannot profile here: with the machine's 32 cores its buffers pass the 516 kB `perf_event_mlock_kb` (`mmap failed`), and pinned to one core it records no sample of the child. A sampler had to be written (`sample.py`: `perf_event_open` on the child's cycles, one 64-page ring, `llvm-symbolizer` for the names; `sample_report.py` reads it). It has no call chains, since the binary has no frame pointers, so `malloc`'s share cannot be laid at its callers. (2) The queue's room: eleven jobs of other sessions queued or running, 14 to 17 GB claimed against 16 to 24 GB available under the 8 GB reserve, so a 4 GB claim found no room from 12:57 to past 13:21; a 2 GB claim for the oak at 40 alone (its peak is 0.15 GB) was admitted at once, twice. (3) The same binary on the same tree took 1.48 s at 12:48 under the desk's load and 0.95 s at 13:11 pinned to a core: seconds cannot carry a 3 % verdict today, so each change is read in user-space instructions (`count.py`), beside the seconds.
- Cost: about 25 minutes (13 on the sampler, 12 waiting on admissions that were then withdrawn and repacked into one job).
- Done about it: every candidate is a patch against the base, and one admission builds each on its own and reads each against its base (`speed2-build.sh`), so the round needs two admissions, not ten.
- What would remove it: `perf` installed, or `kernel.perf_event_mlock_kb` raised so `samply` runs; a profile build with frame pointers kept beside the `ci` one; and a queue that admits by measured peak, as fn-368's FRICTION.md already asks.
## 14 Nov 2026 13:44 Zurich: the round's builds never ran (speed round 2)

- Doing: waiting on the one admission that builds the six candidates and reads each against the base (`speed2-build.sh`, queued 13:21 at 4 GB).
- What hindered: the queue has been frozen since 13:10:59 (`PAUSE (cpu 7 cores)`), 33 minutes at the time of writing, and thawed for about 12 minutes in the hour before that. The desk's load average stood at 62 at 13:37, 76 at 13:40 and 118 at 13:43, from another repository's test suite (`node --test`, `oxlint`, the vault's indexer) and a `scoreboard-probe` run directly; available memory fell to 10 GB against 17 GB of queue claims.
- Cost: the round's verdicts. In 82 minutes the queue gave this round one base run with counters, one sampled run, and two builds of the base; no candidate has been compiled.
- Done about it: the round returns early with its profile, its ranking and the candidates as patches (`SPEED-ROUND-2.md`, `speed2-candidates/`), and the one command that builds and reads them all, as this file's rule asks of a build stopped by a known inefficiency.
- What would remove it: a measurement window the owner grants (the watcher paused, or the other repository's suite held, for the twenty minutes six builds and their reads take); the fixes named at 12:39 and 13:21 above.
## 14 Nov 2026 16:00 Zurich: no quiet core for a verdict in seconds (speed round 2, second part)

- Doing: the round reopened at 14:52: six candidates built and read, five proved alone, the kept four timed against the base.
- What hindered: the queue froze again from 14:55 to 14:57 and from 14:58 to 15:24 (26 minutes, `PAUSE (cpu 15 cores)`), and from 15:46 to 15:48; outside the freezes the desk stood at a load of 4 to 9, so pinned runs of one binary spread by a third (the base on one thread, 0.97 to 1.41 s over 20 runs). The ruling's bar of 3 % in median seconds over five runs cannot be read cleanly from one batch at that spread: the tip's four one-thread batches read 2.8 %, 5.1 %, 4.9 % faster and 6.3 % slower, its four-thread batches from 5 % slower to 6 % faster.
- Cost: about 30 minutes frozen of the 68 the second part took, and a four-thread verdict that stays open.
- Done about it: verdicts carried by instructions and cycles, which the load does not move, with every batch's seconds given beside them (`SPEED-ROUND-2.md`, "Measurements").
- What would remove it: a measurement window with the desk at a load under 1, or an isolated core (`isolcpus`) kept for timings.
