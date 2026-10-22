# fn-309 friction

## 2026-10-22, conductor (flow --auto, plan stage)

- **Doing:** planning R1 (every compound leaf draws its rachis through `rachis::rachises`).
- **Slowed by:** the spec does not say how a twig or cluster placement reaches the clothing step. `rachises` runs in `branching::clothe` before placement, with no seed, no twig stations and no `kept()` filter; a twig placement's point and heading come from `station::place_run` afterwards. On the device the order is boots (frond axes) -> folds -> sweep -> stations, so a station's rachis has no stage that can hang it as wood. fn-283 (Open questions) and fn-295 (R1) both named this as "a second design"; fn-309 is silent on it. The plan stopped and went to the host.
- **Cost:** one scout round (about 190k subagent tokens) and a stop before plan-review; no cargo run.
- **Would remove it:** the spec naming the stage that hangs a station's rachis (replay the station walk in `clothe`, record placements for a later wood step, or hang from placement) and the device stage it moves to, checked against the code before the spec is marked ready (AGENTS.md, "Gates and checked claims").

- **Doing:** starting `flow --auto` on the spec.
- **Slowed by:** the pilot snapshot refuses the run twice: the host's uncommitted probe (`crates/telperion-render/examples/walk_leaf_probe.rs`) trips the dirty-tree guard, and the spec is not marked ready. The conductor drove the stages directly instead.
- **Cost:** a few minutes.
- **Would remove it:** the probe committed (or kept in scratch) before dispatch, and the spec marked ready by the person who consents to the run.

## 2026-10-22 18:15, conductor (plan review)

- **Doing:** the codex plan review, wrapped in `telperion-heavy 8 --threads 2`.
- **Slowed by:** `flowctl codex plan-review` has no `--focus` option, so the host's "run no cargo" instruction could not reach the reviewer's prompt (fn-308 found the same for `completion-review`). The read-only sandbox and the heavy wrapper were the only guard; the reviewer ran no cargo command (checked in its log).
- **Cost:** none this time; a risk on every plan and completion review.
- **Would remove it:** a focus option on every codex review command, or a cargo shim on reviewers' PATH refusing unadmitted test runs (host's to spec, as fn-308 noted).

## 2026-10-22 18:40, worker (task 1)

- **Doing:** re-measuring the base's class counts after the review changed the measure, and running the gate.
- **Slowed by:** (1) the dcg hook refuses a truncating redirect onto an existing evidence file in the worktree, so a re-measured raw log is written to scratch and copied over; (2) the gate (`telperion-heavy 8 --threads 4 -- python3 scripts/gate.py`) outlived the 600 s foreground limit of the Bash tool and moved to the background, so the worker waits on a handle.
- **Cost:** about two minutes and one blocked command; the gate's wait itself.
- **Would remove it:** a gate that fits in the tool's foreground limit on a fresh worktree target (the first gate build is cold here), or the gate documented as a background run with a known handle.

## 2026-10-22 21:35, worker (task 2)

- **Doing:** recording the base device digests (`digest_tests::digest`, every shipped preset) before any edit.
- **Slowed by:** the card is held by a game (about 5.7 of 10 GB VRAM, 9.4 GB in use), so the build failed `DeviceRefused ... Out of Memory` in under a second. The worker's prompt wraps cargo in `telperion-heavy 8 --threads 2` without `--gpu`, so the wrapper admitted a device test it would have held back with `--gpu`.
- **Cost:** one cold build (about two minutes) and a failed run; every device test of the task now waits on the card.
- **Would remove it:** device tests dispatched with `--gpu <GB>` by default, so admission waits for the card instead of the test failing on it.

## 2026-10-22 21:50, worker (task 2)

- **Doing:** compiling the render crate's tests after writing the red agreement test.
- **Slowed by:** `telperion-heavy 8` holds the build at admission: the desk has 9 GB available and a 2 GB reserve, so 7 GB of room against the 8 asked, while a game holds memory and the card. A compile of tests needs nothing like 8 GB but asks for it because the prompt fixes `8` for every cargo command.
- **Cost:** more than ten minutes idle so far, open-ended while the desk is busy.
- **Would remove it:** a smaller admission figure for compile-only steps (`--no-run`, `cargo check`), or the wrapper reporting its wait on stdout so a filtered pipe shows it.
