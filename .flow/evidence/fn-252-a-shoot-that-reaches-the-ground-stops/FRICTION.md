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

## 2026-10-14, task 1, worker

- **Doing:** the spruce at 80 alone, under `telperion-heavy 12` after it overran 5 GB.
- **Hindered:** the wrapper held it 30 minutes (15:53 to 16:24) at "room -1 to 3 GB" while other workers' suites ran; it never started. Stopped and left unmeasured, since the spruce's 40-year skirt already shows its resting wood and R1's birch reading does not rest on it.
- **Cost:** 30 minutes of wall time.
- **Would have removed it:** a measure of ground contact that does not grow the whole spruce at 80 at once (its 1.1 M phytomers at 40 imply several million at 80), or a queue position in the wrapper so a worker can tell a short wait from an indefinite one.
