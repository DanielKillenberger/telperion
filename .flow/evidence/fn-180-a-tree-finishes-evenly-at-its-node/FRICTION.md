# fn-180 friction

## 2026-09-27, worker, task .1 (finding the beech's node-capped candidates)

- Doing: rebuilding the beech's node-capped Tune candidates from fn-62's r3 run, to measure the reduction on them.
- Slowed by: the measured candidates have no family file. `matched/<key>/family.json` exists only for the rendered
  candidates, and none of those is capped. A capped candidate's overlay is stored only in
  `runner/tuning/1/run.json` under `trials[].overrides`. It is matched to its measurement through
  `trials[].measurement.case_id`, and `node_capped` has to be grepped out of each `*.measure.jsonl`.
- Cost: about 10 minutes.
- Would have removed it: a per-candidate overlay file written next to each measurement, named by its case id.

## 2026-09-27, worker, task .1 (the dial half of R3 is a design fork)

- Doing: making `maxNodes` a dial with its cost stated, as R3 asks.
- Slowed by: three things the spec's architecture note did not check. (1) `maxNodes` is an `Option` that is unset in
  every preset. The dial table refuses unset options ("An unset option has no current value to step"), and a
  proposal batch fails with "missing dial" when a dial's current value is null (`tuning/judgments.rs:172`). So
  the row cannot join the table as it stands. (2) The tuning evaluation refuses every candidate with
  `node_capped` true as truncated (`tuning/evaluation.rs:172`). After this task, `node_capped` also marks a tree
  that is whole but reduced. Whether Tune accepts such a tree decides what raising the dial is for. (3) Any new
  dial changes the Jev request bodies, so the beech tape replay needs a live re-record (fn-177's entry: 8 paid Jev
  calls and about 20 minutes).
- Cost: about 15 minutes of reading, and the task returns to the host before building the dial.
- Would have removed it: a spec that names how an unset `Option` becomes a dial, and whether a reduced tree is
  feasible for Tune.
