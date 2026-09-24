# fn-135 friction

## 2026-09-24 - copying the palm fixtures from the fn-80 worktree

- Doing: copying the palm's profile and tuning config from the fn-80 worktree into test fixtures, in one shell command that also ran `git -C <fn-80 worktree> log`.
- Hindered: the dcg hook blocked the whole command (`core.git:git-alias-semantic-unverified`) because the git call took its path from a shell variable; nothing ran.
- Cost: about 1 minute and one extra tool call.
- Would remove it: writing git calls with literal paths, never a shell variable, in a command that dcg inspects.

## 2026-09-24 - the workspace gate failed on memory ceilings this diff never touches

- Doing: the one end-of-task gate, `cargo test --profile ci --workspace --no-fail-fast` (474 s).
- Hindered: four `telperion-core` tests in `tests/species.rs` (`fixed_{birches,oaks,beeches,spruces}_pass_geometry_and_profile_gates_with_repeatable_varied_specimens`) failed at `tests/species/budget.rs:186`: peak resident 6.36 GB over a 6.11 GB process ceiling. The diff changes only `telperion-jev`, docs and the spec. The same test binary alone passed 14 of 14 in 24 s (load average near 6 during the gate, other sessions on the machine).
- Cost: about 9 minutes of gate plus one 1-minute classification rerun; the task cannot claim a green gate.
- Would remove it: a process ceiling that reads the machine's free memory at the time, or the budget tests in a binary of their own that the gate runs alone.

## 2026-09-24 — host: the core memory-ceiling tests fail under a concurrent gate

The four `tests/species.rs` ceiling tests failed at 6.36 GB against 6.11 GB while fn-136's gate ran in another worktree, and passed 14 of 14 on a quiet rerun (host, same branch). The ceiling is load-sensitive, so parallel workers make the workspace gate flaky. What would remove it: gates run one at a time on the machine (a lock), or a ceiling measured per test process rather than under shared load.
