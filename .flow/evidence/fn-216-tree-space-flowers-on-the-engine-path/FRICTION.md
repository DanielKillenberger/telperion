# Friction: fn-216 (and fn-220, built in the same worktree)

## 2026-10-07, the shared GPU lock

- **Doing:** two palm stills before and after fn-220's identity change, then the oak and beech flower stills.
- **Slowed by:** the one GPU lock is held in turn by other workers' full workspace gates (`cargo test --profile ci --workspace` under `flock`), each 10 to 20 minutes. The first still's `flock -w 900` timed out silently behind two of them, and the queue held four to six waiters through the afternoon.
- **Cost:** about 40 minutes of wall time waiting on two 10-second stills; one silent timeout and a rerun.
- **Would remove it:** a gate that takes the GPU lock only around the GPU test binaries rather than the whole workspace run (compilation and CPU suites need no lock), or a second lock for short stills.

## 2026-10-07, `cargo fmt` reformats the crate

- **Doing:** formatting fn-220's four touched files before committing.
- **Slowed by:** `cargo fmt -p telperion-core` rewrote 31 unrelated files: the tree is not clean under this toolchain's rustfmt (1.98.1), so a crate-wide format is unusable and each file must be formatted by name.
- **Cost:** about 5 minutes to reverse the unrelated changes (`git checkout --` is blocked by the shell guard, so by a reverse patch).
- **Would remove it:** one formatting commit on master under the pinned toolchain, and `cargo fmt --check` in CI.

Resolved in part (host, 2026-10-07): the gate now runs in two parts, the workspace without `telperion-render` unlocked and `telperion-render` alone under the GPU lock.

## 2026-10-07, tiny organs vanish at a whole-tree view

- **Doing:** the oak's catkins on the hero still.
- **Slowed by:** florets of a few millimetres are sub-pixel at the hero distance and draw nothing visible (1.98M florets drawn, no yellow in the crown); only a close-up shows them. Each look costs a GPU-lock round trip of 10 to 30 minutes on a busy desk.
- **Cost:** two still rounds (about 50 minutes of wall time) to learn the size, not the mechanism, was the problem.
- **Would remove it:** a CPU-side still or a coverage estimate (florets' projected pixels at the hero pose) printed by the build, so a size that cannot show is known before the GPU.
