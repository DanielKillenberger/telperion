# fn-252 friction

## 2026-10-14, task 1 (measure), worker

- **Doing:** the birch's ground and crown-base probe at seven ages and three seeds under `telperion-heavy 8`.
- **Hindered:** the wrapper held the job waiting for 8 GB for five minutes while another worker's `units_bound` test ran; the young trees need well under 1 GB (age 10 grew in 0.02 s), so the request was sized for the 80-year leaves, not for the run.
- **Cost:** about 6 minutes of wall time and one stopped run.
- **Would have removed it:** a one-line memory estimate per age in the probe's usage note (or the wrapper printing the peak of the last run of the same binary), so a worker asks for what the run needs and splits the large ages off. Done here by hand: young ages at 2 GB, 80 alone.

## 2026-10-14, task 1, worker

- **Doing:** the ground counts for every other species at 40 and 80 in one wrapped script under a 5 GB cap.
- **Hindered:** the spruce at 80 overran the cap and the scope was killed at 15:40 with no line in the log; the script's exit was 0, so the run read as finished until the counts were checked. The beech, palm, Laurelin and Telperion were lost with it and rerun.
- **Cost:** about 15 minutes of waiting and one rerun.
- **Would have removed it:** `telperion-heavy` reporting an OOM kill of its scope (exit nonzero, a line in the output) instead of passing on the script's status.
