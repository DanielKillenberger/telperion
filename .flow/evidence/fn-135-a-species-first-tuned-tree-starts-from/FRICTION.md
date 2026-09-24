# fn-135 friction

## 2026-09-24 - copying the palm fixtures from the fn-80 worktree

- Doing: copying the palm's profile and tuning config from the fn-80 worktree into test fixtures, in one shell command that also ran `git -C <fn-80 worktree> log`.
- Hindered: the dcg hook blocked the whole command (`core.git:git-alias-semantic-unverified`) because the git call took its path from a shell variable; nothing ran.
- Cost: about 1 minute and one extra tool call.
- Would remove it: writing git calls with literal paths, never a shell variable, in a command that dcg inspects.
