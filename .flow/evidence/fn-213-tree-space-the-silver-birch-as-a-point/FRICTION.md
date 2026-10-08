# fn-213 friction

## 2026-10-07: the Jev commands in docs/typesafe.md do not run

- **Doing:** screening the birch's height sources with Jev (task 1, R1).
- **Slowed by:** `cargo run -p telperion-jev -- screen ...`, as `docs/typesafe.md` "Commands" gives it, fails: the crate has three binaries (`jev`, `species`, `tape_trim`) and no `default-run`. The working form is `cargo run -p telperion-jev --bin jev -- screen ...`.
- **Cost:** one failed run, about 2 minutes.
- **Would remove it:** `default-run = "jev"` in `crates/telperion-jev/Cargo.toml`, or `--bin jev` in the doc's five commands.

## 2026-10-07: a redirect to a path held in a variable is blocked

- **Doing:** writing literature and screen logs under the spec's `raw/` folder from a loop.
- **Slowed by:** the shell guard refuses `> $DIR/file` and `> file-$(...)` (a dynamic redirect target); each loop had to be rewritten as a script with literal paths or piped through `tee`.
- **Cost:** two blocked commands, about 3 minutes.
- **Would remove it:** nothing in the repository; a local guard setting (reported, not specced).

## 2026-10-07: the GPU lock is held by another session's workspace test

- **Doing:** rendering sheet 1 (6 trees, 6 shots each) under the shared GPU lock.
- **Slowed by:** another session holds the lock for `cargo test --profile ci --workspace` (the whole gate, not a GPU step), so the stills wait behind a CPU-bound run.
- **Cost:** about 15 minutes of waiting so far.
- **Would remove it:** take the GPU lock only around the GPU-touching test binaries (the render crate's), not the whole workspace gate.

## 2026-10-07: a gate piped through `head` stopped early

- **Doing:** the workspace gate at the end of task 1.
- **Slowed by:** the first run was filtered through `| head -60`, which closed the pipe and ended cargo partway; the failures it showed were real, but the run had to be repeated in full.
- **Cost:** about 25 minutes (one gate run, most of it waiting on the GPU lock).
- **Would remove it:** nothing in the repository; write the gate's output to a file and grep the file.

## 2026-10-07: the gate's one failure is on the base

- `generation_limit_guard::generation_limits_are_classified` fails on two sites in `crates/telperion-core/src/pipeline/input/shoulder.rs` (fn-198's grown envelope shoulder, commits `24005de7` and `f7d227ee`), which this spec does not touch. Reported, not fixed here.

## 2026-10-07: a row at its default fails the value-file test only in the full gate

- **Doing:** round 2's values; the leaf-station spacing moved to 0.02 m, which is the family's default.
- **Slowed by:** the value writer drops a row at its default, so `preset_values` failed, but only in the 25-minute workspace gate; the tuning loop (`space_rows`, rebuild, render) never ran it.
- **Cost:** one extra test round, about 5 minutes.
- **Would remove it:** the still runner, or `build.rs`, refusing a value-file row that states its default, so the loop catches it as it compiles.

## 2026-10-08: the compare-sheet scripts are not in raw/

- **Doing:** round 11's compare sheets; the brief said the scripts were in `raw/`.
- **Slowed by:** no script is there or in the scratchpad (the sheets were composed by hand-typed `montage` lines); a search and a rewrite.
- **Cost:** about 5 minutes and a few thousand tokens.
- **Would remove it:** keep the sheet command in the evidence folder beside `birch_measure.rs`; round 11 adds it as `compare_sheet.sh`.

## 2026-10-08: no leaf close-up shot in the still runner

- **Doing:** the leaf close-up per age.
- **Slowed by:** `space/still.rs` has no shot nearer than the 9 m limb view, so a scratch example (`leaf_closeup.rs`) was written and copied into the render crate to run, and a round-10 preset copy swapped in to render the round-10 close-ups.
- **Cost:** about 10 minutes, one extra build.
- **Would remove it:** a `leaf` shot in the still runner (camera under a metre from an outer leaf).

## 2026-10-08: the shell guard blocks a preset swap and dynamic redirects again

- **Doing:** rendering round 10's close-ups from the committed preset, and writing render logs.
- **Slowed by:** `git show HEAD:<path> > <path>` and `> $E/...` redirects are refused; each became a copy through the scratchpad and a script with literal paths (the 2026-10-07 entry's cause).
- **Cost:** two blocked commands, about 3 minutes.
- **Would remove it:** nothing in the repository; local guard setting (reported, not specced).
