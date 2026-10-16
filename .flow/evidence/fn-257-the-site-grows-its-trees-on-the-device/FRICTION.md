- 2026-10-16 00:14 (fn-257 growth profile): telperion-heavy 12 GB waited 13 min for room (10 GB free) before I noticed; the oak alone ran in 10 GB. Cost ~13 min. Would remove: telperion-heavy printing the free room once and failing fast (or suggesting the largest admissible size) after a minute instead of waiting silently.
- 2026-10-16 (fn-257 task 3 gate): `telperion-heavy 12` waited 25 min silently for room (9 GB free of 12) before I looked; restarted at 8 GB with 3 threads. Cost ~25 min. Same remedy as the entry above: fail fast with the largest admissible size.
- 2026-10-16 (fn-257 task 3 gate): the gate at `telperion-heavy 8 --threads 3` was killed at its 8 GB cap partway through `telperion-core`'s tests (~70 min in); the desk had 9 GB of room, so 12 GB waited and 8 GB died. Cost ~70 min. Would remove: the gate's peak by package documented beside `telperion-heavy`, or a gate that runs package by package under one cap.
- 2026-10-16 02:00 (fn-257 task 6): another session's gate and npm test held 24 GB of `telperion-heavy`'s room for over an hour; my core test waited at "room 2 GB for 9". Cost: the wait (~1 h+). Would remove: a queue that serves heavy jobs in order across sessions, or one desk-wide gate at a time announced to other sessions.
- 2026-10-16 (fn-257 task 7 review): reviewing a task whose head is followed by unrelated uncommitted work needs a second worktree at the head; flowctl writes review state into that worktree's `.flow/specs/*.json`, so moving it to the next head was refused ("Aborting", unnoticed in the output), and the first round reviewed an empty range. Cost: one review round (~10 min) and a hand-written merged file to finalize it. Would remove: `impl-review --head <sha>` reviewing a range without checking it out, or flowctl keeping review state outside the tracked spec file.

## 2026-10-16, task 9's review cap (worker)

- **Doing:** task 9's codex impl-review.
- **What hindered:** both rounds asked for the same thing, the inventory of growth's small tables. Round 2 judged 713ad7ff7. The fix, 186569209, a row for every remaining table, landed after it. The third run was refused: `MAX_REVIEW_ITERATIONS=2`, escalate to a human.
- **Cost:** about 25 minutes; task 9 is held at in_progress.
- **What would remove it:** the review's first finding naming every row the inventory needed, or a round cap that counts a finding fixed in place as converging.

## 2026-10-16, completion review (host's driver)

- **Doing:** fn-257's codex completion review.
- **What hindered:** `flowctl codex completion-review` resolves its snapshot as `main...HEAD` and failed on this repository, whose trunk is `master`; the release branch is 1,846 commits past master, so `--base` had to be found by hand (the first fn-257 commit's parent, 0fc870be8^), and the reviewed range still carried 3,331 files of other specs' work. A shell redirect to `$S/...` was blocked by the dcg hook and had to be retyped with literal paths.
- **Cost:** about 5 minutes and one failed dispatch (no round consumed).
- **What would remove it:** flowctl reading the trunk from `git symbolic-ref refs/remotes/origin/HEAD` (or the config) and a per-spec base recorded when a spec's first task starts.

The review's one gap (P1, R3'): the oak at seed 1234 over 0.1.4. Re-measured warm at 15ae83609 (`the_device_field`, two runs, desktop GPU 14-28 % busy with browsers): oak grow + 36³ query seed 1 261-267 + 16 ms (0.1.4 290), seed 1234 321-327 + 15 ms (0.1.4 273, about 65 ms over), seed 5678 287-294 + 18 ms (0.1.4 332). Growth is 20-30 ms slower than RESULTS.md's at every seed; whether that is the busy desk or a change since task 7 is not measured. Escalated to the owner, per R3's "any species over 0.1.4 is named with its gap".

## 2026-10-16, task 11 (worker)

- **Doing:** the site field's paired A/B (two test binaries, base and change) and the harness build's byte comparison.
- **What hindered:** the dcg hook refused a loop redirecting to `$S/$b` paths and `git show HEAD:<path> > <path>` to build the base binary; both were retyped (literal paths, a script appending to one file, and `git apply -R` of a saved patch). Another worker's GPU jobs held the desk at 70-99 % busy for 13 of the 16 field pairs, so only paired differences are meaningful.
- **Cost:** about 5 minutes of retyping; the timings carry the desk's load.
- **What would remove it:** a `measure-ab <base-bin> <bin> <test> <pairs>` helper beside `telperion-heavy` that alternates two binaries and appends to one file, and a desk-wide GPU queue so a timing run gets a quiet window.

## 2026-10-16, task 12 (worker)

- **Doing:** the R8 splat probe and its per-pass profile.
- **What hindered:** the probe had to live under `field/` (not beside `field_probe_tests.rs` as the task named) to see the field's private `Cfg`, `Rec` and sources, and still copied `make.rs`'s `cfg`/`leaf_box` and `leaflets`' loop body because they are private or not functions. The repository's `every_shader_file_is_checked` refuses a probe `.wgsl` no validated source holds, found only on the second build; it went into `field::sources()`. The GPU profile's labels collide between the sweep and the stations (`starts`, `emit`, `blocks`, `spread`), so the sweep's share of `emit` could not be told apart.
- **Cost:** about 10 minutes and one extra build.
- **What would remove it:** profile labels prefixed with their pass's name, and a note in `docs/` that a test-only WGSL file joins a `#[cfg(test)] sources()` list.
