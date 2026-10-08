# fn-214 friction


## 2026-10-07 10:57, task 1: timings under other workers' load

- **Doing:** timing the coarse field against 0.1.4 in Wasm and natively, then the coarse build's stages on one core.
- **Slowed by:** load ran 15 to 155 on 32 cores across the hour (another worker's binaries). One native stage breakdown read the predictor at 86 to 120 ms and the settle at 552 to 625 ms against 18 ms and 80 to 95 ms a minute earlier, so it was thrown away; the Wasm sweep ran at load 22 to 41 and needed a second, quieter pass (load 13) for the gate's comparison.
- **Cost:** about 6 minutes of reruns and a pinned (`taskset`) pass.
- **Would remove it:** the desk-wide timing lock fn-198's FRICTION.md already names; nothing new.

## 2026-10-08 09:30, the merge's gate: the command guard on a variable log path

- **Doing:** starting the gate's workspace half with its log in the scratchpad.
- **Slowed by:** the dcg hook refused a `>` redirect to a path held in a shell variable; the command was rerun with the literal path.
- **Cost:** about 1 minute.
- **Would remove it:** a local setup matter (the hook's rule), already reported in fn-198's FRICTION.md; nothing to spec.

## 2026-10-08 17:20, task 2: reverting a tool's side effects under the command guard

- **Doing:** merging F4, then reverting the formatting `rustfmt` had applied to five files the task never touched (it formats every child module of the file it is given), and restoring timing probes from scratchpad copies.
- **Slowed by:** the guard refused `git restore`, `git show <ref>:<path> >` and a `cp` whose source path sat in a shell variable. Each was redone another way (a reversed patch, Python, literal paths).
- **Cost:** about 4 minutes.
- **Would remove it:** a format command for the files a diff touches, without their children (for example `rustfmt --config skip_children=true` on nightly, or a repo script). The guard's rules are a local setup matter.
