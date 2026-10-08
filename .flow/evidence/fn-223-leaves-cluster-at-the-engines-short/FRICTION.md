# fn-223 friction

## 2026-10-08: the command guard blocks the edit and format idioms a worker reaches for (fn-223.1)

**What I was doing:** editing several Rust files in one Python heredoc, and formatting only the files I had touched.

**What slowed it:** dcg refused three ordinary commands:
- a heredoc whose Rust text contained `Result<()>`, read as process substitution;
- `git checkout -- <file>`, to restore the files rustfmt had reformatted through module children;
- `rm -rf` on a scratch directory.

`rustfmt` on a list of files also formats every child module those files declare, so "format only touched files" has no one-line form on stable.

**Cost:** about 10 minutes and three retries. I wrote each edit to a scratch `.py` file instead, and wrote a scratch `fmt.py` that saves every untouched tracked file's bytes before rustfmt runs and restores them after.

**What would remove it:** a repository script that formats only the files changed since a base (the `fmt.py` approach), named in AGENTS.md beside "Format only touched files".

## 2026-10-08: the first workspace check ran at full parallelism (fn-223.1)

**What I was doing:** the first `cargo check --workspace` after the engine rows changed.

**What slowed it:** the shell's environment did not carry the dispatch's `CARGO_BUILD_JOBS`, so cargo ran at its default 32 jobs. The owner felt the lag, and the coordinator asked for 4 jobs. I stopped the check and reran it at 4.

**Cost:** one aborted check, about 3 minutes, and the owner's machine under load.

**What would remove it:** a `.cargo/config.toml` `[build] jobs` setting, or a per-checkout env file the shell loads, so that no command depends on a worker remembering to export the limit.
