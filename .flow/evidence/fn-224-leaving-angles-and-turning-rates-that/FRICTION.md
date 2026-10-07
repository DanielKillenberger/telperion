# fn-224 friction

## 2026-10-07, task 1 (worker): the base hashes raced the first edits

- **Doing:** recording `space_hash` on the base before changing code, in the background while the first edits went in.
- **Hindered:** the build compiled the half-edited tree and failed, so the base had to be rebuilt with the work set aside as a patch (`git apply -R`, run, `git apply`).
- **Cost:** about 4 minutes and one extra 2-minute build.
- **Would remove it:** `space_hash` taking a git revision (or a `--base` flag that builds the merge base in its own target), so the before half never shares the working tree.

## 2026-10-07, task 1 (worker): the shell guard blocks ordinary edit and redirect idioms

- **Doing:** scripted multi-site edits (a Python heredoc whose strings held backticks) and a redirect to `$E/hash-before.txt`.
- **Hindered:** dcg refused both (`heredoc.shell:launcher-unverified`, `redirect-truncate-dynamic-path`); each had to be rewritten as a script file in the scratchpad or with a literal path.
- **Cost:** about 3 minutes and two retries.
- **Would remove it:** a note in AGENTS.md that edit scripts go in the scratchpad as files and redirects take literal paths.

## 2026-10-07, task 1 (worker): the walk binary is most of the space suite

- **Doing:** `cargo test --profile ci -p telperion-space` before the gate, to see the new walk settings pass.
- **Hindered:** the `walks` binary took 431 s of the suite's 8 minutes; the rest finished in under a minute. Adding six settings (each of three PAs' insertion and tropism spreads) lengthens it further, and the gate runs it again.
- **Cost:** about 7 minutes, twice (once here, once in the gate).
- **Would remove it:** a filter to walk only named settings (`WALK=form.insertion_spread`), so a change that adds a setting proves it alone before the gate walks them all.
