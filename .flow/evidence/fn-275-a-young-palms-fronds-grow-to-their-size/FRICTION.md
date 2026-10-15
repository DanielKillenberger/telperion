# fn-275 friction

## 2026-10-14, worker, R1 sources

- **Doing:** finding a date palm leaf length at a stated early age for R1.
- **Slowed by:** the establishment literature states the law (Tomlinson 2012, IFAS EP473) and the years to a full crown, but no source on disk or in the research index gives a seedling's or young palm's leaf length at a stated age; the research index returned no full text for four candidate PMC papers, so each was scraped by hand and grepped.
- **Cost:** about 25 minutes and roughly 60k tokens of search and scrape before the first early-age length.
- **Would remove it:** a source-screening tool that takes a species and a question ("leaf length at a stated age") and returns candidate sentences with their checksums across the fetched sources, so the agent reads candidates rather than whole pages.

## 2026-10-14, worker, first focused test run

- **Doing:** the first focused core test run (8 GB admission) after the CPU change.
- **Slowed by:** `telperion-heavy` held it in its queue while another worker's release film (fn-272) held the memory; room read -2 GB against 8.
- **Cost:** the queue time logged in `raw/core-focused.log` (one "waiting" line every 30 s), with the build not yet started.
- **Would remove it:** a queue that reports its expected wait (the running job's elapsed time and cap), so a worker can choose a smaller admission or do other work instead of polling.
- 11:22: no heavy job was running and room still read 6-7 GB against my 8 (the desktop and dev server hold the rest); after 40 minutes queued I withdrew the 8 GB ask and resubmitted at 5 GB. Would remove it: an admission size per job kind that a worker can read, rather than a guess.

## 2026-10-14, worker, the gate's admission

- **Doing:** the end-of-task gate at the host's 9 GB admission.
- **Slowed by:** room has read 6 to 7 GB for over 20 minutes with no other heavy job holding a claim; the desktop and dev servers hold the rest, so a 9 GB ask waits on memory nobody frees.
- **Cost:** the queue time in `raw/gate.log`; the first gate attempt was also withdrawn for the review's fix after 25 minutes queued without starting.
- **Would remove it:** the gate's measured peak, so its admission asks what it needs, or a reserve that tracks the desk's resident set rather than a fixed figure.
- 12:49: the gate was still queued after 70 minutes (room 0 to 7 GB against 9) and was withdrawn unrun; the worker returned to the host with the gate and `npm test` open rather than hold the task on the queue.

## 2026-10-14, worker, R2a's focused runs

- **Doing:** a 1-test probe and the focused suites while building R2a.
- **Slowed by:** two workspace gates from other worktrees held the memory; a 5 GB probe waited 35 minutes unstarted and was withdrawn; the suites were resubmitted at 3 GB.
- **Cost:** about 40 minutes queued over two runs.
- **Would remove it:** a lighter admission class for an incremental single-crate test build (it peaks well under the gate's), or a gate queue that runs one workspace gate at a time.

## 2026-10-15, worker, R2b's gate

- **Doing:** the end-of-task workspace gate for R2b.
- **Slowed by:** the generation-limit inventory guard (`tests/generation_limit_guard.rs`) failed on three new guard sites in `crown.rs`. It was the gate's only failure, and it surfaced only at the end of a 30-minute gate, because no focused run reaches the guard.
- **Cost:** one gate run's last failure, fixed with three inventory entries and a re-run of the guard target alone (seconds).
- **Would remove it:** a `cargo test -p telperion-core --test generation_limit_guard` step in the focused-run recipe for any change under `src/pipeline`, or a pre-commit check that runs the guard.

## 2026-10-15, worker, R2c: a frond draws its rachis

- **Doing:** looking for the existing primitive the brief said would carry a drawn rachis (fn-275.3).
- **Slowed by:** the brief assumed that a path existed. None did: a leaf instance has no free bit and only a uniform scale, and the bloom stalk's machinery works only on the organ draw. That took one escalation. Under the host's ruling B (rachis as wood, like the leaf bases), the device builds its wood (dress, boots, folds, sweep) before its stations pass, and that pass holds the frond clock and frond placement (`stations.wgsl` `rosette`, `crown_of`, `*_frond`). Hanging the rachis as device wood therefore means moving or sharing that code into the boots stage, past the host's 400-line stop. That took a second escalation.
- **Cost:** two host round trips, and about 30 minutes of reading with no code written.
- **Would remove it:** a brief that names the stage and executor order a new piece of geometry must fit into. For the device, that is `resident/build.rs`'s order: hang, folds, sweep, stations.
- 2026-10-15, R2c base rule: a one-test diagnostic of a single library test, admitted at the brief's 10 GB, waited about 20 minutes in `telperion-heavy`'s queue behind another worktree's `diag_rows` run. The room read 2 to 4 GB. Would remove it: a smaller admission class for an incremental single-test run, the same ask as the R2a entry above.
