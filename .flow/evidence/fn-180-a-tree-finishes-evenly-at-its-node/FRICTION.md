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
- Resolved by the host the same day: a dial declares the value its unset row stands for; a reduced tree is
  whole and feasible, and only an incomplete one is refused, by name.

## 2026-09-27, worker, task .1 (re-recording the beech tape)

- Doing: extending the beech's Tune round for the new dial (`species --extend`, 9 Jev calls and two sheets), then
  removing the superseded answers.
- Slowed by: the command guard refused a recursive delete of a scratch directory, a shell redirect to a variable
  path, and an inline script whose text mentioned a delete. The `inotifywait` watch had to be written as a script
  with literal paths. The watch itself (fn-177's recipe) worked the first time, and `--extend` changed no
  existing file.
- Cost: about 10 minutes, 9 Jev calls.
- Would have removed it: a `tape_trim --prune <replay>` that lists or removes the files a replay did not open.

## 2026-09-28, worker, task .1 (a review fix re-recorded the tape)

- Doing: correcting the dial's advertised cost (the review found the beech's measured 0.6 to 0.9 s per step above
  the stated 0.2 to 0.5 s).
- Slowed by: the cost sits in the dial's `ask`, which is inside every Tune proposal request, so a one-number text
  fix re-recorded the beech round (`--extend`, 9 more Jev calls) and a second watched replay.
- Cost: about 8 minutes and 9 Jev calls.
- Would have removed it: a tape key that does not hash the dial wording (fn-177's entry asks the same), or
  checking every number in a text against its measurement table before the first recording.

## 2026-09-28, worker, task .1 (the size check caught a clone after the PR opened)

- Doing: trimming the slim field module after CI refused +7.9%.
- Slowed by: the size check (`node scripts/artifact-budgets.mjs`) runs only after `npm run build`, which neither
  the workspace gate nor `npm test` includes, so the growth showed up only in CI's package job after the push. `twiggy diff` on two unstripped builds named the cause
  (`Specimen::clone`) at once.
- Cost: one CI round trip and about 15 minutes.
- Would have removed it: the task's gate list naming `npm run build` and the budget check for any change under
  `crates/telperion-core`, which the slim module compiles.
- Disposition (host, 2026-09-28): the proposal to run the size check in the local gate for core changes is the
  host's to spec at close.
