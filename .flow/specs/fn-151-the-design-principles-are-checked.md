## Conversation Evidence

> owner (2026-09-25): "let's do it properly and make sure that we have something in place that makes sure these design philosophies really get adhered to ... It shouldn't be overbearing but agents should live and breathe this and have their decisions guided."
> owner: "can we actually have a test suite that we run through jev to check if the diff adheres to our guiding principles." / "but why just ci? if we have it part of the local suite we don't have to open a PR first to figure out the mistake" / "we could make it a pre_commit hook?" (host: pre-push; owner: "yes")
> owner: "so 151 really needs to be designed well. It needs to be token efficient, fast but be highly reliable in catching offending pushes ... can we run jev on specs before we start work?"
> owner: "fn-151 should also make sure that STRATEGY.md is upheld" / "for fn-151 we should check through previous PR's identify ones that should definitely have raised flags and make sure that the hook catches it"
> owner, on continuity: "I can imagine a multidimensional tree space where if you're on one part of the space some dimensions just don't change the output ... we need to find a principle here that holds."
> owner: "I want an in depth review with astra on how to make sure this isn't overbearing and full of false positives. This needs to be tight." (review: `.flow/evidence/fn-151-the-design-principles-are-checked/ASTRA-REVIEW.md`, verdict "needs redesign before implementation") / host's revised outcome / owner: "ok"

## Goal & Context
<!-- scope: business -->

STRATEGY.md's principles slipped three times in a week, and nothing checked them. The slim field package kept a copied build chain (#58). The date palm shipped unbuildable through that package (#115). A first fix grew its Wasm 48%, stopped only by a hand instruction. The species runner grew about 30 stop sites. This spec puts the principles in front of every push and every spec. Deterministic checks block what is certain. Jev warns on judgement-shaped breaches, and earns blocking only through measured precision on real pushes. The guard must be tight: it catches real breaches and stays quiet on healthy work, including sanctioned exceptions and harmless dormancy. Otherwise people learn to bypass it. [paraphrase]

## Architecture & Data Models
<!-- scope: technical -->

**What exists, checked 2026-09-25.** [checked]
- There are no git hooks in the repository, and `core.hooksPath` is unset.
- The `jev` binary has the subcommands screen, select, cite, triage, ask and cases. Question sets and labelled cases are in `crates/telperion-jev/data`. The shared caller retries with delays and uses the `jev-latest` alias.
- CLAUDE.md keeps Jev out of workspace tests, and its key is visible only to an interactive shell.
- STRATEGY.md allows a GPU executor beside the CPU reference, and a qualified fallback geometry, outside "Our approach". The growth path is sanctioned in CLAUDE.md.
- The core pipeline's `placed_field` fallback remains for families the plan cannot describe.

**The policy.** A short maintained list of question ids, each citing the exact STRATEGY.md clause or owner ruling it rests on, versioned on its own. Nothing is extracted automatically. It includes:
- one pipeline;
- never a fallback path, with STRATEGY.md's qualified allowances;
- no switch between ways of building, where dormancy is clean and a jump is a breach;
- only the detail the consumer reads;
- measured cost and look;
- a step that stops work must catch defects the steps around it cannot. [paraphrase]

**Exceptions.** A registry of scoped entries. Each names the principle, the specific symbols or operation it covers, the rationale and its authoritative source: an owner decision or a strategy allowance. Code annotations, specs and a PR's Decisions line cite an entry by id. A claim of approval without an entry creates none. No entry covers a whole PR, a directory or future behaviour. The day-one entries are the growth path (#17) and the GPU executor (#50). [paraphrase]

**Deterministic guards, which block** (in `cargo test` and the pre-push hook): [paraphrase]
1. **Production boundary.** Production code outside the pipeline never calls the build stages. Aliases are resolved, tests are excluded, registered exceptions are honoured, and removed and added callers are compared together.
2. **Entry coverage.** Every shipped preset builds through every callable generation entry (native, main Wasm and slim field) in their shipped configurations. It triggers when preset values, catalogue membership, entry wiring or their dependencies change. This owns omissions such as #115; it reuses fn-150's regression.
3. **Artifact budgets.** Each shipped Wasm and package artifact's size, from a fixed build recipe, against a checked-in budget. A change to the budget carries measured evidence and a Decisions reference.

Timing and memory are measured separately, on named hardware, never per push.

**The Jev reviewer, `jev principles`, which advises.** [paraphrase]
- **Candidates.** Code extracts only newly introduced or worsened behaviour of three kinds, each with before and after spans, relevant callers or consumers, removed implementations, the governing clause and the applicable exceptions:
  - a setting that selects a builder or suppresses existing structure;
  - surviving duplicate implementations, or redundant blocking steps;
  - output that is generated or uploaded with no consumer reading it.
- **What is never a candidate on its own:** cargo features, cfg gates, names, CLI commands and package exports. They are context, or triggers for the deterministic guards.
- **Decision.**
  - Jev selects a breach mechanism and code-supplied evidence ids, or "none", or "insufficient evidence".
  - Only an above-threshold answer gets a second, confirming question, which states the legitimate readings and the exceptions.
  - A warning needs both to clear calibrated per-principle thresholds, and code to validate the cited spans. Otherwise the reviewer abstains.
- **Graduation.** Each principle starts in shadow mode (logged, not shown), moves to warnings, and blocks only after its measured precision on real pushes reaches at least 95%.
- **Specs.** Spec mode judges each current proposal with its decision context, and excludes rejected, historical and superseded designs. It advises before ready and adds no approval step.
- **Audit mode.** An explicit audit mode reports debt that already exists. Push and spec modes report only what a change introduces.

**Where it runs.** [paraphrase]
- A checked-in `.githooks/pre-push`, installed once through `core.hooksPath` by the setup script, runs the guards and the reviewer on the pushed revisions against an explicit merge base. It calls Jev through `bash -ic`, and a missing key or network reports "incomplete".
- CI runs the same checks on each PR.
- **Output:** at most three findings, deduplicated. Each carries its location, the principle, the before and after evidence, the consequence and one repair or test command, with the full structured output on request.

**Cost.** [paraphrase]
- Cached p95 at most 0.5 s; uncached p95 at most 5 s; a 10 s deadline.
- At most two batched Jev phases, at most 12 candidates, and at most 8,000 input tokens per run.
- An overflow, timeout or unavailable service reports "incomplete", never "clean".
- The cache key is the candidate's full inputs plus the extractor, question, model, policy, threshold and exception versions.

**Guidance.** [paraphrase]
- STRATEGY.md takes the owner-approved continuity sentence ("every parameter is a numeric trait defined for every tree ... a small change in any parameter makes a small change in the tree. A parameter may be dormant where the structure it shapes is absent, and it wakes smoothly ... No parameter is a switch between ways of building, and a count steps only by one unit of the structure it counts") and the process line above.
- AGENTS.md states that specs and pushes are checked against these principles, and gives four short questions (host decision, 2026-09-25: fn-154 made AGENTS.md the one instruction file and removed CLAUDE.md).
- `docs/pr-format.md` asks that a trade-off name its principle and exception id.

**The evaluation corpus.** [paraphrase]
- **Owner-confirmed labels (2026-09-25).**
  - Positives:
    - #6, #58 and #55;
    - #115, owned by entry coverage;
    - #3, #59, #52, #53, #61 and #13;
    - the runner's approval layers #38, #75, #76, #77, #83, #87 and #94.
  - Clean: #17, #50 and #82, plus the survey's other clean PRs.
  - The survey's own draft labels (#50 positive, #17 likely) are superseded by these.
- **How it is labelled:** by defect mechanism and evidence span, on immutable revisions, with calibration and holdout groups separated by lineage.
- **Matched clean cases:** dormancy, count steps, backend choice, extraction or delegation, necessary validation, and accepted byte changes.
- **Mutations:** omission mutations for the guards.
- **Growth:** it grows from confirmed findings and dismissals, plus a small chronological sample of clean changes.

**Unknown.** The corpus size needed before any principle can graduate, which the implementer reports from real pushes. [unknown]

**The cut (owner, 2026-09-25).** The shipped change keeps the three deterministic guards, the exception registry, the policy's question ids, the pre-push hook running the guards and the guidance. The Jev reviewer (candidate extraction, the two-phase decision and its cuts, spec mode, the replay corpus and its recorded answers) was built, measured and removed; design review moves to fn-156. Evidence: `.flow/evidence/fn-151-the-design-principles-are-checked/EVALUATION.md` (Jev recall 2/3 in calibration, 0/11 in the holdout, 0/22 clean pushes flagged; the labelled duplicate-path spec not flagged) and the ignored recordings under `raw/`. [decision]

**Structure over policing (owner, 2026-09-25).** The shipped change is structural: one pipeline (its stages private in fn-152), one ordinary test that builds every preset's every artifact through it and through the package entries, and a size budget in CI's package job. The boundary scanner, the exception registry file, the policy file, the `jev principles` guards and the pre-push hook were built, then removed; they are in git history at `061ff37d`. [decision]

## Acceptance Criteria
<!-- scope: both -->

- **R1:** The production-boundary guard fails on a scratch change that calls a build stage from production code outside the pipeline. It passes on #82's revision and on test code. Errors: an unresolvable alias fails the guard with its location. [paraphrase] **Moved to fn-152 (owner, 2026-09-25):** "stages private to the pipeline; a second chain does not compile". The host amends fn-152.
- **R2:** The entry-coverage guard fails for `date-palm` through the slim entry on the master before fn-150. It passes on master after fn-150, and triggers on a change to preset values, catalogue membership or entry wiring. Errors: a failing entry names the preset and the entry. [paraphrase] **Now (owner, 2026-09-25):** the preset-through-pipeline test: every catalogue preset builds every artifact kind through `pipeline` (telperion-core), and the Wasm binding and slim field entry tests hold the package entries; red on `39348def` for date-palm through the slim entry.
- **R3:** The artifact-budget guard fails on the +48% slim build from fn-150's first attempt, and passes on the shipped 0.1.4 build. Errors: a budget change without evidence fails. [paraphrase] **Now (owner, 2026-09-25):** the CI size budget, `scripts/artifact-budgets.mjs` on the package job's build against `scripts/artifact-budgets.json`.
- **R4:** On the offline replay (frozen extractions with recorded answers), every confirmed positive's labelled mechanism is caught by a guard or by a Jev finding that cites that mechanism, and every confirmed clean case yields no finding. Errors: a positive with no supporting candidate class is reported as unsupported coverage, never as a pass. [paraphrase] **Moved to fn-156 (owner, 2026-09-25):** see "Jev judges a spec's design against what already exists" and Decision Context below.
- **R5:** A live, held-out evaluation reports Jev's recall, precision and clean-push false flags with their counts and uncertainty. A principle leaves shadow mode only when its warnings reach at least 95% precision on real pushes. Errors: no Jev finding blocks at launch. [paraphrase] **Moved to fn-156 (owner, 2026-09-25):** see "Jev judges a spec's design against what already exists" and Decision Context below.
- **R6:** The pre-push hook runs on a push from any worktree, and meets the cost bounds above, measured and reported. Errors: timeout, missing key or overflow reports "incomplete"; `--no-verify` skips it. [paraphrase] **Withdrawn (owner, 2026-09-25):** no hook; see Decision Context.
- **R7:** Spec mode yields no finding on fn-150's final design, and flags a labelled spec that proposes a surviving duplicate path. It adds no readiness step. Errors: missing decision context abstains. [paraphrase] **Moved to fn-156 (owner, 2026-09-25):** see "Jev judges a spec's design against what already exists" and Decision Context below.
- **R8:** STRATEGY.md, AGENTS.md and `docs/pr-format.md` carry the guidance above (host decision, 2026-09-25), the exception registry holds its day-one entries, and the workspace gate and `npm test` are green. [paraphrase] **Now (owner, 2026-09-25):** the guidance in STRATEGY.md, AGENTS.md, `docs/pr-format.md` and `docs/principles.md`, which names the two sanctioned exceptions.

## Boundaries
<!-- scope: business -->

- Not the continuity measure (fn-148). Not per-push timing or memory benchmarks. Not consolidating AGENTS.md and CLAUDE.md, which is its own change. Not removing the core `placed_field` fallback, which the audit mode reports for its own fix.

## Decision Context
<!-- scope: both — conditionally substructured -->

The pre-review draft let Jev block pushes on "touches a principle", derived principles automatically, and treated names, cfg gates and CLI commands as suspicion. Astra's review showed it would reject its own clean cases, miss omissions and breaches inside existing functions, and over-claim reliability from about 40 labelled PRs. The design now blocks only on deterministic checks. Jev findings need evidence and a confirming question, and earn blocking one principle at a time by measured precision. [paraphrase]

The implementation (PR #120) measured the advisory reviewer on the owner-labelled corpus. The guards caught every guard-owned positive (#6, #55, #58, #115), while Jev caught #3 and #13 and missed or could not reach the runner's stop sites and the palm's switch. The owner cut the reviewer from fn-151 on 2026-09-25 and kept what caught real breaches; R4, R5 and R7 moved to fn-156, which judges a spec's design against what already exists. [decision]

Structure over policing (owner, 2026-09-25): a principle is held by code that cannot express the breach, an ordinary test and a CI size budget, never by a scanner or a hook that polices a push. R1 moves to fn-152, which makes the stages private so a second chain does not compile. R6 is withdrawn with the hook. The two sanctioned exceptions, the growth path and the GPU executor, are named in `docs/principles.md` until fn-152 expresses them as visibility. [decision]

## Strategy Alignment

- Serves "Our approach": one pipeline, no fallback, continuous tree space, only the detail the consumer needs, measured cost and look. [strategy:Our approach]
