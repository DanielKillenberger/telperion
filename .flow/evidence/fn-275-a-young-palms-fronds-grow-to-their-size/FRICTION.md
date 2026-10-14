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
