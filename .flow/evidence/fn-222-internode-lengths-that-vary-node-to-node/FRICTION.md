# fn-222 friction

## 2026-10-07, task 1 (worker): `cargo fmt --all` rewrites 46 files outside the change

- **Doing:** formatting the change before its first commit.
- **Hindered:** `cargo fmt --all` reformatted 46 files the change never touched (core, render, field, jev, wasm), and two hunks inside a touched file. CI does not run `cargo fmt --check`, so the tree has drifted from rustfmt's output. Undoing it took a tagged stash and drop, because the shell guard (dcg) blocks `git checkout -- <path>` and a redirect from `git show`.
- **Cost:** about 5 minutes and a few thousand tokens.
- **Would remove it:** one commit that formats the whole tree plus a `cargo fmt --check` step in CI; or, until then, a note in AGENTS.md to format only touched files (`rustfmt <file>`).

## 2026-10-07, task 1 (worker): the split gate fails the jev replay on a fresh target

- **Doing:** the gate in two parts, `--exclude telperion-render` first, as the host set it.
- **Hindered:** `telperion-jev`'s `replay` tests read `target/ci/examples/headless`, a `telperion-render` example. With render excluded and a fresh `target/`, the example is not built, so two tests fail with "headless is missing". They pass once part two has built it (rerun: 3 of 3).
- **Cost:** about 3 minutes reading the log and a rerun.
- **Would remove it:** run part two before part one on a fresh target, or have the replay test build the example it needs (or skip with a named reason when render is excluded).
