# Friction reports, fn-69 session

## 2026-09-18 host: no SSH agent in the tool shell

- **Doing:** creating the spec branch from a freshly pulled master before dispatching the worker (work Phase 2 fence).
- **Hindered by:** the tool shell has no `SSH_AUTH_SOCK`, so `git fetch origin` over the ssh remote failed with "communication with agent failed"; the fence's `&&` chain stopped before the checkout and the spec commit landed on local master. Recovery was one `git branch --`, one `git reset --hard HEAD~1` on master, and a fetch through `gh auth git-credential` with an `insteadOf` rewrite passed as `-c` flags.
- **Cost:** about four minutes and two command-guard (dcg) refusals on the recovery commands.
- **What would remove it:** an SSH agent reachable from the tool shell, or a repo-local `url.https://github.com/.insteadOf=git@github.com:` plus `gh auth setup-git` so every git remote call goes through the gh token this session already holds.
- **Early return:** not taken; the recovery was short and the branch is correct.

## 2026-09-18 12:49 worker: the Codex quota is exhausted, the implementer bridge cannot run

- **Doing:** Phase 1b of the task worker, bridging the implementation of fn-69.1 to `codex exec -m gpt-6-astra -c model_reasoning_effort=high` from the repo root, as the CLAUDE.md routing block asks for every implementation.
- **Hindered by:** codex returned at once with "You've hit your usage limit ... try again at Sep 19th, 2026 4:27 PM" (exit 1, no digest). The routing block has no reachability check before dispatch, so the quota state is discovered by a failed run.
- **Cost:** about three minutes (one blocked launch from a dcg redirect rule, one real launch, the diagnosis).
- **What would remove it:** a one-line quota probe before the bridge (a cheap `codex exec` on a trivial prompt, or reading the usage endpoint), and a dated line in the routing block naming who implements while the quota is out, as the block already records for 2026-09-08 to 2026-09-11.
- **Early return:** not taken; the worker's Phase 1b degrade rule applies (session model implements, the summary records `implement - skipped(reach: gpt-6-astra unreachable, session model used)`), and the task is one workflow file.
