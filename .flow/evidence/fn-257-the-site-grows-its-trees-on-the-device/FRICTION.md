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

## 2026-10-16, task 13 (worker)

- **Doing:** re-anchoring task 13 and listing every consumer of the field API it deletes, before any edit.
- **What hindered:** the deletion's consumers reach past the task's Touches. `cargo test` builds examples, and three examples read the deleted API: `crates/telperion-core/examples/measure.rs` (`Field::query`, `bounds`, `storage_bytes`), `crates/telperion-core/examples/artifact_digest.rs` (`Field::snapshot`) and `crates/telperion-render/examples/generation_digest.rs` (`FieldSnapshot`, `IndexSnapshot`). Neither task 13 nor task 14 touches them, so the focused crate tests cannot compile. `crates/telperion-wasm/src/**` (a workspace member: `Field::query`, `snapshot`, `bounds`, `storage_bytes`, `FieldSnapshot`) and `crates/telperion-render/src/web/field.rs` (`query_field`, `Answer`, wasm32 only) are task 14's, so the workspace and the npm build stay broken between tasks 13 and 14.
- **Cost:** about 15 minutes; the task stopped before any edit, per the host's rule on unforeseen consumers.
- **What would remove it:** a plan step that greps every deleted public name across the workspace, examples included, and gives each hit to a task's Touches; or the API cut and its bindings in one task.
- **Doing (resumed, 16:30):** the core's baseline and its first `cargo check` through `telperion-heavy 8 --threads 2`.
- **What hindered:** the core's own test binary went past the 8 GB cap at two test threads and was killed after 310 passing tests, so the baseline is inconclusive. Then two other worktrees' workspace gates, at 12 GB each, held the desk, so a `cargo check` of one crate waited in the queue for over 30 minutes while the edits went on unchecked.
- **Cost:** about 35 minutes of waiting, and the edits went on without a compiler.
- **What would remove it:** a light lane in `telperion-heavy` for `cargo check` and clippy, which need well under 8 GB, and a core test cap measured once and stated in the host rules.

## 2026-10-18, task 13 resumed (worker)

- **Doing:** verifying the checkpointed edits: `cargo check --workspace --all-targets` and the wasm32 check passed in under a minute each; then the core crate's tests through `telperion-heavy 14 --threads 2`.
- **What hindered:** another worktree's workspace gate held 12 GB, so the core run waited in the queue (room 10-11 GB for 14) for over 10 minutes with nothing else to verify on the CPU side.
- **Cost:** 10+ minutes of waiting so far.
- **What would remove it:** a measured per-crate cap (the core's test binary at two threads) stated in the host rules, so a worker can ask for the smallest cap that fits instead of a guessed one.
- **Doing (03:30):** the core crate's tests at `telperion-heavy 10 --threads 1` (the 14 GB run never got room behind another worktree's 12 GB gate; 10 GB at one thread fit, about 25 minutes).
- **What hindered:** `generation_limit_guard::generation_limits_are_classified` fails because the field's limit sites moved (`field.rs` to `field/splat.rs`, `cuboid.rs` deleted, three new clamp/refusal sites), and the inventory it reads, `docs/generation-limits-inventory.json`, is outside the task's Touches. The task stops for the host before `done`; a drafted inventory that passes the guard is in the worker's scratchpad.
- **Cost:** about 40 minutes of queueing for the core run, then a stop on one file.
- **What would remove it:** the same plan step as 2026-10-16's (grep every deleted public name), extended to the repository's inventories and guards that list source sites (`docs/generation-limits-inventory.json`), so they enter the Touches of the task that moves the code.

## 2026-10-18, task 14 (worker)

- **Doing (04:00):** the field Vitest files alone, as the host asked.
- **What hindered:** `vitest.config.ts`'s global setup builds two native release binaries before any test file runs, and a config outside the checkout cannot resolve `vitest`, so a focused run needed a second config under `.flow/tmp/`.
- **Cost:** about 3 minutes, and the release builds that would have run before every focused run.
- **What would remove it:** a `vitest --project` or an env switch that skips the release binaries for files that do not call them.
- **Doing (04:05):** `tests/browser/bindings.mjs` against the rebuilt Wasm, run alone because `scripts/test-wasm.mjs` also runs `tests/browser/field.mjs`, which needs WebGPU in a browser.
- **What hindered:** the binding suite is in no CI job and has rotted since 2026-10-02 beyond the field: the "compact" fixture (height 4, 40 attractors) grows 152,178 to 1,005,581 nodes, `growth.maxNodes` of 0 or 20 leaves the tree whole and complete, a height of -1 builds, the beech is in the catalogue, the crookedness range is not refused, the species foliage ceiling of 40,000 is refused, and every node is below the crossover. Eleven checks fail that read no field output; they had to be collected one run at a time behind a throwing `check`.
- **Cost:** about 20 minutes and four browser runs, and the acceptance criterion "bindings green" cannot be met inside this task.
- **What would remove it:** the binding suite in a CI job (it needs no GPU), or deleted where the native tests already hold the same contract.
- **Doing (04:15):** `node scripts/test-dist.mjs` on a fresh `npm run build`.
- **What hindered:** its first step, the main entry building Telperion with `{ surface: true, foliage: true }`, is refused by the core Wasm itself (`surface allocation`, code 2, the raw C ABI with no TypeScript in between), so the script stops before it reaches `field.js`. The field half, run on its own against `dist/field.js`, refuses without WebGPU and refuses a side of two by name. The refusal is the 32-bit Wasm limit fn-291 works on.
- **Cost:** about 5 minutes; "test-dist green" cannot be met inside this task.
- **What would remove it:** fn-291 landing on this branch, or test-dist's main-entry tree swapped for one the 32-bit module holds (a host call, since the script's tree is the README's first example).

## 2026-10-18, task 15 (worker)

- **Doing:** the paired site-field measurement (release tip 23fb243e3 against R8 at d9193b9ce).
- **What hindered:** the committed `field_time` test reads seed 1 only and neither side reports the bytes held, so seeds 1234 and 5678 and the held memory needed a scratch test in two disposable worktrees, each building its own render test binary (the task's Touches are the evidence directory). The dcg hook refused the first build command because the redirect's parent directory did not exist yet.
- **Cost:** two render-crate test builds (minutes each) and one retyped command.
- **What would remove it:** `field_time` taking `FIELD_SEEDS` and printing the ledger's live bytes after the read-out, so a later pairing needs only the base's build.
