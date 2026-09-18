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

## 2026-09-18 host: the branch cannot be pushed, the gh token lacks the workflow scope

- **Doing:** pushing the spec branch before make-pr, over HTTPS through `gh auth git-credential` because the tool shell reaches no SSH agent (the 1Password agent answers "communication with agent failed").
- **Hindered by:** GitHub refused the push: "refusing to allow an OAuth App to create or update workflow `.github/workflows/tests.yml` without `workflow` scope". The gh token carries `gist`, `read:org` and `repo` only, and a workflow file is the whole diff of this spec.
- **Cost:** the run stops here with the build complete and unpushed; one owner action (`gh auth refresh -s workflow`, or unlocking 1Password so the SSH remote signs) unblocks it.
- **What would remove it:** the `workflow` scope on the gh token, granted once, or an SSH agent the tool shell can reach.
- **Early return:** taken; the host stops with NEEDS_HUMAN instead of a workaround around the token's scope.

## 2026-09-18 host: a second session commits onto the branch checked out in the shared clone

- **Doing:** opening the PR for the spec branch after the push.
- **Hindered by:** the branch had been cut from local master, which carried four owner commits not on origin, and a concurrent session working fn-58 in the main checkout committed its task mint onto the checked-out fn-69 branch. A squash merge would have folded all of it into the CI commit. The owner chose a rebuild: the branch was recreated from origin/master in its own worktree with the four CI commits cherry-picked, the fn-58 commit was cherry-picked onto local master, and the main checkout returned to master.
- **Cost:** about eight minutes, one owner question, one force-push of a branch that had no PR yet.
- **What would remove it:** the work stage branching from origin/master rather than local master when the two differ, and every session running in its own worktree so a checked-out branch is never a shared commit target.
- **Early return:** not taken; the rebuild was short.
