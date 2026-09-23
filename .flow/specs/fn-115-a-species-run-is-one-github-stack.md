# A species run is one GitHub stack

## Conversation Evidence

> user (2026-09-23): "we should probably just make one stack for one of these species adds of prs that go in together?" / "github stack"
> host: recommended stacking the palm's open PRs now and specifying the stack as the onboarding method for the next species.
> user (2026-09-23): "aye"

## Goal & Context
<!-- scope: business -->

A species run grows gap fixes as it goes: the palm minted fn-108, fn-109 and fn-110 from the capability gate and fn-113 and fn-114 from its tuning run. Each landed or waits as its own PR against master, so master took fixes one at a time whose only proof is a species that renders on all of them together, and the run waited on each merge or ran on a hand-merged branch (fn-80 merged #65 and #66 into itself on 2026-09-23). [paraphrase]

The owner wants the PRs of one species onboarding to go in together as one GitHub stack: the fixes at the bottom, the species on top, merged atomically once the owner ticks the checklist. [user]

## Architecture & Data Models
<!-- scope: technical -->

- **The tool, checked 2026-09-23.** `gh stack` (github/gh-stack v0.1.1, gh 2.100.0) is installed. `init` adopts existing branches bottom to top; `add` puts a branch on top; `submit` pushes and creates or rebases the PRs; `rebase` cascades; `link` builds a stack from existing PRs without local tracking; `merge` is GitHub's atomic stack merge, all or nothing up to a chosen PR, refusing drafts and applying branch protection when it runs. [checked]
- **What exists, checked 2026-09-23.**
  - The conductor records a landing by any commit (`species-conductor land --commit SHA`, `conductor/dependency.rs:453`); nothing checks the commit is on master, so a commit on the stack's branch already counts. [checked]
  - `docs/pr-format.md` opens one PR per spec against `origin/master` by default and never merges. [checked]
  - `flowctl spec chain` admits a dependent spec for work only with every dependency done, or exactly one open dependency whose tasks are done and whose branch is on origin; two open parents park it (flow-next `auto.md`, Phase 1). A species with several open fixes therefore cannot chain on them today. [checked]
  - `add-species` (`.claude/skills/add-species/SKILL.md:79`) mints each fix as its own spec, makes the species spec depend on it, works it "under the repo's review" and resumes on landing. [checked]
- **Shape.** [inferred]
  - The species spec's branch is the stack's top. `add-species` runs `gh stack init` for it at the manifest stage.
  - A gap fix's branch is inserted below the species branch. It is based on the stack's current top fix and rebased up with `gh stack rebase`; its PR is opened by the stacked make-pr path.
  - The conductor's `land` records the fix's commit on the stack, and the run resumes without a master merge.
  - The owner's checklist tick is followed by `gh stack merge` of the whole stack. Nothing merges earlier unless the owner lands a fix alone.
- **The design (host, 2026-09-23).** [host design]
  - *Base.* `docs/pr-format.md` already replaces make-pr's body phases in this repository, so a stacked base is set there: a spec whose branch is in a species stack opens its PR against the branch below it. No flow-next change.
  - *Admission.* `flowctl spec chain` gates `flow --auto` selection only. A species spec is driven by `add-species` and the conductor, never selected by `flow --auto` while its fixes are open, so the single-parent rule never meets it. The species spec keeps its dependencies for the record; a fix spec is worked by `flow --auto` on its own branch as today and inserted into the stack when its task is done.
  - *Linear branches.* A stack branch is rebased, never merged into: `gh stack rebase` brings master up the stack. A branch that already carries merge commits (fn-80) is linked with `gh stack link`, which rebases nothing.
  - *Insert below the species.* `gh stack add` only stacks on top and `modify` is interactive, so a fix is inserted by `gh stack unstack --local` from a member branch, `gh stack init` in the new order and `gh stack rebase --no-trunk` (dry-run 2026-09-23, below). [checked]
## Acceptance Criteria
<!-- scope: both -->

- **R1:** `docs/pr-format.md` gains a stacked mode: a PR whose spec belongs to a species stack is opened with the stack's lower branch as its base, the body names its place in the stack, and the procedure never merges. [inferred]
- **R2:** `docs/species-onboarding.md` and `add-species` state the stack: init at the manifest, each fix inserted below the species, landing recorded by the commit on the stack, one atomic merge after the owner's tick. [inferred]
- **R3:** A species spec with several open fixes on its own stack is admitted for work and resumes; `flowctl spec chain`'s single-parent rule is either satisfied by the stack's shape or answered by a documented local rule. Errors: two species sharing one fix put the fix in its own stack below both, or land it alone first. [inferred]
- **R4:** A local dry run on throwaway branches shows `init`, an insert below the top and `rebase --no-trunk` producing the chained order, with nothing pushed and no PR opened. [inferred]
- **R5:** The gate is green if any code changes: `cargo test --profile ci --workspace --no-fail-fast`. [paraphrase]

## Boundaries

- The palm's current PRs (#65, #66, #61) are linked into a stack by hand now, not by this spec. [user]
- The owner merges; no stage of any driver runs `gh stack merge` without the owner's word. [paraphrase]
- No change to the gap loop's routing or the tuning loop. [inferred]

## Decision Context

- The owner chose a stack per species on 2026-09-23 after the palm's fixes had landed piecemeal and fn-80 had merged two open fix branches into itself to keep running. [user]
- `gh stack` is v0.1.1; the R4 dry run exists because the tool is new. [inferred]

## Open Questions

- None. The three unknowns were settled by the host on 2026-09-23 (see Architecture).
