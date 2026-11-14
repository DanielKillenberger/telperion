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

## 14 Nov 2026 09:10 Zurich: the release checks have no script of their own (Opus, release checks L3)

- Doing: finding the commands for fn-237 R2's checks (harness timing, the field against 0.1.4, sizes, docs, version) on the candidate.
- What hindered: no file lists them with commands. They are spread over fn-198 task 9's text, fn-259 `RELEASE.md`, `docs/gpu-tests.md` (the `frames` probe and `gpu-row.mjs`), and fn-352's scoreboard (`wasm.mjs`, `wasm014.mjs`). The scoreboard's `wasm.mjs` sets `family.fidelity`, which fn-367 retired, so it is refused on the candidate and needed a one-line scratch copy. `every_pipeline_within_2_5_s_cold`, the cold probe `docs/gpu-tests.md` still names, no longer exists (the GPU grower is gone), and the doc was not updated. The 0.1.4 package was found already unpacked in the session scratchpad from 11 Nov.
- Cost: about 20 minutes of reading before the first run.
- What would remove it: one `scripts/release-checks` entry (or a section in `docs/gpu-tests.md`) listing each check, its command and its bar, kept current when a probe is retired; the host decides whether it is specced.

## 14 Nov 2026 09:12 Zurich: a GPU row inside the queue holds its memory while it waits for the card (Opus, release checks L3)

- Doing: the cold `frames` rows (`gpu-row.mjs` inside `telperion-heavy 6 --gpu 2`), while `npm test` waited for 8 GB.
- What hindered: `telperion-heavy` admitted the row's unit at 09:07, then `gpu-row.mjs` waited for the three other heavy units (a codex review, the film probe, another worktree's gate) to end. The idle unit's 6 GB counted as claimed, so `npm test` sat at "room 0 GB for 8" behind a job that was itself only waiting. I stopped the row job at 09:12 to let the test run first.
- Cost: about 5 minutes of both jobs waiting; the cold rows moved after `npm test`.
- What would remove it: `gpu-row.mjs` waiting for an empty queue before `telperion-heavy` admits it (a pre-admission wait, or `telperion-heavy` not counting a unit that carries the `.row-waiting` marker as claimed); the host decides whether it is specced.

## 14 Nov 2026 10:06 Zurich: the cold rows wait for an empty queue that other sessions keep filling (Opus, release checks L3)

- Doing: the cold `frames` rows, queued again at 09:35 after `npm test` and the Wasm timings.
- What hindered: `gpu-row.mjs` starts only when no other `telperion-heavy` unit runs. Since 09:35 the queue has held, in turn, the film probe (from 09:03), the host's `gate.py --full` (from 09:39) and a film cut (from 09:59). Each new unit is admitted beside the waiting row, so the row's start moves with every one.
- Cost: 30 minutes of waiting by 10:06, still waiting; no agent time beyond checking.
- What would remove it: a GPU row holding a place in the queue (new heavy units wait behind a waiting row), or the host scheduling the release checks in a quiet window; the host decides.
- Update, 16:01: the rows ran in bursts between other units. The first started at 11:16 after 101 minutes of waiting. The second Telperion row then waited from 12:07 to 15:58. Each row runs for seconds to a minute, but the 24 rows took 6.5 hours of wall clock. Laurelin's third row is invalid because GNO's model worker came onto the card during it.
