# fn-292 friction

## 2026-10-16, task 1 worker

- Doing: formatting the render crate before the measurement commit.
- Hindered: `cargo fmt -p telperion-render` rewrites `generation/resident/field_probe_tests.rs`, which is unformatted on the base; the dcg hook then blocks both `git checkout -- <file>` and `git show HEAD:<file> > <file>` as ways to undo it, so the edit came out through `git diff | git apply -R`, twice.
- Cost: about 3 minutes and two blocked commands.
- Would remove it: formatting `field_probe_tests.rs` once on master, so `cargo fmt` on the crate touches only what a task changed.

## 2026-10-16, task 1 worker, review

- Doing: finalizing the codex impl-review panel, three draws all SHIP with no findings.
- Hindered: `impl-review-fanout-finalize --merge-plan` refused ("cannot parse draw contracts") because the contracts draw wrote no findings container; the finalize had to be redone with a hand-written `--merged-file`.
- Cost: about 2 minutes and two refused calls.
- Would remove it: a draw with a SHIP verdict and zero findings parsed as an empty findings list, so an empty merge plan finalizes.

## 2026-10-16, task 2 worker

- Doing: comparing frame p50 per preset against `raw/before/frames.txt`.
- Hindered: the base was measured once per preset, so the spruce's +0.9 ms (whole) and +0.6 ms (close) after, steady over three runs, could not be told from run-to-run noise without the base binary; rebuilding the base's `frames` example in a scratch worktree with its own target dir waited about 13 minutes for `telperion-heavy` room (2 to 4 GB free of the 8 asked) before a 1.5-minute build, and the 24 base reruns then took about 10 minutes more.
- Cost: about 25 minutes, most of it waiting for room.
- Would remove it: the base capture taking three runs per preset (the same count the after takes), or keeping the base's built `frames` binary beside `raw/before/`.

## 2026-10-16, task 2 worker, captures

- Doing: saving run output under raw/ to literal paths.
- Hindered: the dcg hook refused a redirect into a directory the same command had created with mkdir, and a redirect that would overwrite an existing build log. Each needed a second call: mkdir alone first, or a new file name.
- Cost: about 2 minutes and two refused commands.
- Would remove it: the hook accepting a redirect whose missing parent the same command creates.

## 2026-10-16, task 2 worker, gate

- Doing: running the workspace gate once, through `telperion-heavy 12 --threads 2`.
- Hindered: the gate waited for 12 GB of room from 15:56. Free memory sat at 4 to 10 GB while other worktrees' jobs ran, and after 23 minutes it had not started.
- Cost: 23 minutes or more of waiting before the gate's own 35 minutes.
- Would remove it: a queue in `telperion-heavy` that holds new admissions for a job already waiting, so the 12 GB gate is not starved by a stream of smaller jobs.
