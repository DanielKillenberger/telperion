# Friction, fn-92

## 2026-09-20, worker, R1's red state

Doing: establishing R1 red on the base commit before touching `device.rs`.
Slowed by: the spec's test shape does not reproduce here. Eight threads behind
a barrier, eight rounds, crashed 0 of 17 on the base; with a per-thread stagger
of 3, 10 and 30 ms after the barrier, 0 of 36; a libtest-shaped probe of eight
plain `#[test]` functions each calling `common::gpu()`, 0 of 12. `bark_plates`
on the same base crashed 5 of 12 (not the 4 of 4 the spec records), and a fresh
core (pid 218184) shows the spec's mechanism exactly: one thread in
`vkEnumerateInstanceExtensionProperties`, one inside `vkCreateInstance`.
Cost: about 35 of the 60 minutes, five rebuild-and-count loops.
Would remove it: the spec's repro checked as a standalone test before R1 was
written around it; what `bark_plates` adds beyond a bare request is not known.

## 2026-09-20, worker, shell guard

Doing: capturing suite output to scratchpad logs. Slowed by: the local `dcg`
hook blocks `>` redirects to any shell-expanded path, including the session
scratchpad. Cost: one rejected command. Local setup, reported only.

## 2026-09-20, worker, R5 baseline on the base commit

Doing: timing the render crate on base `device.rs`. Slowed by: the local `dcg`
hook blocks `git restore --worktree` even on a clean tree, so the base was
built in a detached scratch worktree instead, sharing this target directory.
Cost: one rejected command and a second build of the workspace crates. Local
setup, reported only.

## 2026-09-20, worker, a shared target directory poisoned R2

Doing: R2's five runs, straight after timing the base from a scratch worktree
that shared this target directory to save a build. Slowed by: my own shortcut.
Test binary names do not depend on the checkout path, cargo did not rebuild at
HEAD, and four of five runs crashed on what was most likely the base binary.
Cost: about 12 minutes to suspect, force a rebuild, add an `nm` symbol check
and rerun. Would remove it: never share a target directory between two
checkouts of the same crate; a second build costs less than this did.
