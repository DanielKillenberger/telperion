# Build friction from fn-69: the branch base, the shared checkout, and the implementer probe

## Goal & Context
<!-- scope: business -->

The fn-69 build (the CI workflow, 2026-09-18) recorded four friction entries in its FRICTION.md. Three of them are repository work and are proposed here; the fourth, the tool shell's git credentials, is a local setup matter on the owner's machine and stays out of the repository. Together the three cost about fifteen minutes, one owner question, and a commit that landed on master by accident. None of them is about the tree generator; every one is about the shop floor an agent works on.

## Acceptance Criteria
<!-- scope: both -->

- **R2:** The work stage cuts a spec branch from `origin/<default branch>` after a fetch, never from the local default branch, when the two differ. The fn-69 branch carried four of the owner's unpushed commits and had to be rebuilt before the PR. Errors: an unreachable origin stops the branch cut with `NEEDS_HUMAN` instead of falling back to the local branch.
- **R3:** Every session in this clone works in its own worktree; the main checkout is never a commit target for a build. A second session committed fn-58 bookkeeping onto the checked-out fn-69 branch. The rule lives in CLAUDE.md under a dated owner line. Errors: none beyond the rule.
- **R4:** The worker probes the implementer's reachability before bridging: one cheap `codex exec` call, and on a quota refusal it records the retry time and degrades to the session model without a failed full launch. The routing block in CLAUDE.md carries a dated line naming who implements while the Astra quota is out, as it did for 2026-09-08 to 2026-09-11. Errors: a probe that itself fails is treated as unreachable.

## Boundaries
<!-- scope: business -->

- No change to the generator, the renderer, the presets, or the workflow file fn-69 landed.
- No new tooling beyond a shell config, a CLAUDE.md line, and a worker prompt line; the flow-next plugin is not forked.

## Decision Context
<!-- scope: both -->

Three small fixes in one spec because they share one cause: the build's shop floor, the branch base, the shared checkout and the implementer bridge, was never set up for unattended work, and each build rediscovers that at the moment it needs it. Splitting them would give three specs of one line each. The fn-69 FRICTION.md entries of 2026-09-18 are the evidence for every criterion.
