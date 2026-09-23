---
satisfies: [R1, R2, R3, R4, R5]
---
# fn-115-a-species-run-is-one-github-stack.1 Implement A species run is one GitHub stack

## Description
TBD

## Acceptance
Every R-ID in the parent spec's ## Acceptance Criteria is satisfied; judge this task against the spec's criteria directly.

## Done summary
Docs only, host-implemented. R1: `docs/pr-format.md` stacked mode (base is the branch below; never merges). R2: `docs/species-onboarding.md` "The stack" and `add-species` (init at the manifest, fix inserted below the species, landing by the commit on the stack, one owner merge). R3: the species spec is driven by add-species and the conductor, never selected by `flow --auto`, so `spec chain`'s single-parent rule does not gate it (documented). R4: local dry run on three throwaway branches: init, insert via `unstack --local` + re-init + `rebase --no-trunk`, chained order confirmed by `gh stack view`; nothing pushed, no PR. R5: no code changed, no gate needed.
## Evidence
- Commits:
- Tests: gh stack init/unstack --local/rebase --no-trunk dry run (local)
- PRs: