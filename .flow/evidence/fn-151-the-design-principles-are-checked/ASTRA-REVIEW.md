## Verdict

**Needs redesign before implementation.** Fn-151 combines broad syntactic triggers, underspecified context and weak calibration with push blocking. It would challenge sanctioned architecture and healthy refactoring while missing breaches inside existing functions or untouched dependencies. Ship deterministic contract checks and an advisory Jev pilot first. I inspected historical diffs for positives #59 and #115, clean cases #50 and #82, and supporting cases #13, #70 and #112. This was read-only; I ran no live Jev evaluations.

References below use **spec** for [fn-151](.flow/specs/fn-151-the-design-principles-are-checked.md) and **survey** for [PR-SURVEY.md](.flow/evidence/fn-151-the-design-principles-are-checked/PR-SURVEY.md). Historical paths refer to the cited PR revision.

## Findings ranked by severity

### Critical — The policy would reject its required clean examples

**Problem.** The reviewer reads only “Our approach” and “Key metrics”, excluding the GPU allowance in `STRATEGY.md:49`. Its absolute fallback prohibition also conflicts with qualified fallback geometry at `STRATEGY.md:57`. The growth exception lives in `CLAUDE.md:30`, outside its stated inputs.

**Evidence.** Clean #50 adds `Delivery`, `Backend::CpuFallback`, native/Wasm cfg gates and a GPU executor in `crates/telperion-render/src/generation.rs`. Spec:84 requires it to pass. Survey:15 still calls it positive; survey:20 calls sanctioned #17 likely positive. Spec:53 says “four principles”, while survey:6 defines five, including process complexity.

**Exact change.** Replace automatic commitment extraction with explicitly maintained question IDs citing exact strategy clauses and applicable owner rulings. Include the relevant Tracks allowances. Reconcile survey labels with the owner-confirmed labels before calibration. Define “duplicate botanical rules” separately from execution backends, output representations and scheduling choices. Preserve generator-evolution permission for measured improvements that change bytes.

### Critical — Stored diff replay cannot prove omission detection

**Problem.** Spec:107 promises every positive is caught by replaying stored diffs against recorded Jev answers. That cannot establish #115’s runtime failure.

**Evidence.** In #115 (`39348def`), `date_palm` already exists and changes from 40 to 42 fronds. The release also promotes it into the served catalogue. The actual refusal is in untouched `telperion-field/src/grow.rs:25`; untouched `foliage/plan.rs:20` excludes rosettes. Dormant twig values are irrelevant to this failure.

The surviving slim copy likewise cannot be newly discovered from an unrelated branch diff. Conversely, clean #82 moves existing fallback and rosette behavior into a new pipeline, making old behavior appear as additions.

**Exact change.** Split acceptance into:

- Offline extractor and decision-rule replay.
- Deterministic integration regressions for omissions.
- Live, held-out model evaluation.

Trigger preset-entry checks on changed values, catalogue promotion, entry wiring and relevant dependencies. Exercise the actual slim/main package configurations. Distinguish newly introduced or worsened defects from existing debt; put repository-wide debt discovery in an explicit audit mode.

### High — Every extractor class needs narrowing

Spec:52 treats architectural vocabulary as suspicion. Healthy changes frequently introduce exactly these shapes.

| Candidate class | Concrete false-positive source | Required change |
|---|---|---|
| New build-stage callers | #82 adds `pipeline::skeleton` calling `branching::generate` while deleting the Wasm copy. Stage-isolation tests legitimately call builders; current `tests/specimen_view.rs:28` calls `surface::build`. | Make this a deterministic production-boundary check. Resolve aliases, exclude tests and respect scoped backend/growth exceptions. Compare removed and added callers together. |
| Cargo features and cfg gates | #82’s `pipeline.rs:267` selects native concurrency; `:291` uses serial execution on Wasm. #50 gates native synchronous wrappers. | Cut as standalone Jev candidates. Check supported configurations deterministically; attach cfg context only to an identified behavioral change. |
| Pause, decision or approval sites | Clean #70 adds branches and veto terminology while removing false stops. `CLAUDE.md:9` explicitly requires some host escalations. | Extract added blocking transitions and their trigger conditions. Require evidence that a neighboring check already catches the same defect. Ordinary validation and mandated escalation are clean. |
| Look-alike modules/public items | #112 adds validation by extracting shared `rows()` from `Specimen::new`; #82 consolidates familiar operations under a new module. | Cut name similarity. Consider only surviving duplicate implementations with the same responsibility, including deletion/delegation evidence. |
| CLI subcommands | The survey supplies no established clean subcommand-addition case. A command name reveals nothing about duplicated implementation. | Cut this class. Follow the handler only if it introduces another supported candidate mechanism. Add a clean command-wrapper fixture before claiming coverage. |
| Presets | #62 and #114 add shared organ capabilities with neutral defaults. Dormancy is valid. | Route preset changes to entry coverage and targeted continuity checks. Do not ask whether inactive rows “violate continuous space”. |
| Package entries | Clean #63 adds three Wasm asset exports in `package.json`; these are distribution entries, not three generators. | Classify entries by role and test their declared contracts. Only callable generation entries join the preset-build matrix. |

### High — The extractor misses the behavior the owner most wants caught

**Problem.** No explicit candidate class covers runtime parameter dispatch, suppression of existing output, or unread output buffers.

**Evidence.**

- **#59:** `foliage/placement.rs:284` returns no ordinary runs when `rosette::bearing` becomes true; `:308` replaces short-shoot counts with rosette counts. `rosette.rs` defines bearing as `rosette_fronds > 0`. A cfg or similar-name hit would only catch this accidentally.
- **#3:** `BranchHabit` dispatches among builders. Enum/runtime dispatch is absent from the extractor contract.
- **#13:** `surface.rs:71` adds `coords`; shaders declare `coord` explicitly unread. Finding an allocation or upload alone does not establish lack of consumption.
- **#55:** Understanding the fallback requires relating `supports()`, planned-field construction and placed-field construction.
- **#52/#53/#61 and runner positives:** Recognizing duplication requires both implementations or adjacent checks. One candidate’s “few lines” cannot demonstrate redundancy.
- **The +48% fix:** Its magnitude comes from built artifacts, not source classification.

**Exact change.** Add bounded before/after evidence for parameter-controlled dispatch or suppression, duplicated responsibilities, and generated/uploaded outputs with consumer-use evidence. Code owns measurements. Require an extraction fixture for each confirmed defect mechanism. Explicitly report unsupported coverage; remove the promise that every unknown missed class automatically becomes a test failure.

### High — Probability alone is an inadequate blocking rule

**Problem.** “Touches a principle” is not “breaches a principle”. A high probability can confidently identify legitimate architecture.

**Evidence.** Spec:53 supplies only “touches none”. `docs/typesafe.md:48` explicitly says confidence does not establish correctness; `:79` describes composing judgments. Existing `tuning/veto/judge.rs:17` already distinguishes insufficient evidence.

**Exact change.** Require code-extracted evidence spans, an assessable breach question and a separate confirmation of the alleged mechanism after presenting applicable allowances. Both questions need no-match answers. Use per-principle thresholds and abstain on missing context, disagreement or uncalibrated classes. Do not multiply the two probabilities or treat repeated questioning as independent validation. Jev findings remain advisory.

### High — The labelled set cannot support the proposed reliability claim

**Problem.** The final spec names **17 positives and 22 clean PRs**, with many correlated runner changes. That is a regression corpus, not enough independent evidence for calibrated blocking thresholds.

**Evidence.** Survey:4 says closed runner PRs survive inside #61, creating leakage if parent and descendants cross calibration/holdout boundaries. Spec:69 permits 10% false flags, while spec:107 demands every clean case pass. Even zero failures among 22 independent clean cases gives a one-sided 95% upper false-flag bound of approximately **12.7%**.

**Exact change.**

- Label individual defect mechanisms and evidence spans, not just whole PRs.
- Freeze separate calibration and holdout groups by change lineage.
- Add matched negatives for dormancy, count increments, backend selection, extraction/delegation, necessary validation and accepted byte changes.
- Add omission mutations, removed consumer reads, aliases, ordinary functions containing new switches, and refactors moving old debt.
- Count a positive as caught only when the finding identifies the labelled breach.
- Measure end-to-end recall, including extractor misses and abstentions; measure false flags per clean push.

For advisory release, target **≥90% recall, ≥95% finding precision and ≤1% clean-push false flags**, reporting counts and uncertainty. These are deployment targets, not claims the current corpus can substantiate. Require zero false findings on curated clean regressions. Keep Jev nonblocking; even zero false flags in 299 independent clean cases only supports an approximately 1% upper bound.

Grow the corpus from confirmed findings, dismissals and a small chronological sample of clean changes. Store immutable revisions, minimal context and labels. A PR number alone cannot reconstruct an offline fixture.

### High — Exceptions are acknowledged but have no executable meaning

**Problem.** Spec:89 makes authors bypass the whole hook and explain later in PR Decisions. That cannot prevent repeated findings before a PR exists.

**Evidence.** #17 and #50 must pass immediately. Fn-150’s final design deliberately introduces another field primitive and geometry gating; its Architecture section also retains superseded proposals.

**Exact change.** Allow code/spec annotations to reference a scoped exception ID backed by an existing owner decision or strategy allowance. Store principle, affected symbols or operation, rationale and authoritative source. A PR Decisions line can cite that ID; an unsupported claim of approval cannot create one. Honor existing approval without requesting it again. Do not exempt an entire PR, directory or future behavior.

### Medium — Spec bullets lose intent, relationships and supersession

**Problem.** Prose bullets are useful retrieval units but poor independent verdict units.

**Evidence.** Fn-150:19 proposes a “second primitive”; :23 replaces that design, and :25 explicitly marks earlier rounds superseded. Fn-151:64 would judge each bullet separately. It could reject the very design R4 requires to pass.

**Exact change.** Extract a proposed change with its surrounding subsection, current decision, replacement/deletion, boundaries and cited allowances. Exclude historical descriptions, rejected alternatives, quotations and superseded proposals. Treat “new stage” or “second X” as vocabulary only. Missing context yields abstention. Spec mode should advise before ready, never impose another approval checkpoint.

### Medium — Cache and latency requirements are incomplete

**Problem.** A diff hash omits policy, model, thresholds, exemptions and unchanged context. A documentation edit also invalidates the whole-diff cache unnecessarily.

**Evidence.** Spec:42 changes policy versioning, while :62 caches only the diff. Existing `pipeline/judge.rs:80` tests identity dependence on state, questions and model. `caller.rs:15` uses `jev-latest`; :184 retries with delays totaling seven seconds, before network time.

**Exact change.** Cache per candidate and complete input/version identity. Bound cold execution, retries, candidate count and total input size. Revalidate mutable model aliases periodically. Use immutable pushed revisions and an explicit merge base, rather than an ambiguous comparison with possibly stale `origin/master`. Deduplicate identical pushed tips.

A finding needs two evidence excerpts, the violated clause, observable consequence, applicable exception result and one concrete repair or verification command. Limit terminal output to three findings, with full structured output available.

### Medium — “Offline, free” guards include expensive and noisy measurements

**Problem.** Per-preset timing and peak-memory gates are neither free nor reliably deterministic. Updating `budgets.json` can also rubber-stamp a regression.

**Evidence.** Spec:46–49 places all these checks in `cargo test`; `CLAUDE.md:40` requires one final workspace gate. Fn-150:24 already reports artifact size and performance evidence.

**Exact change.** Reuse fn-150’s entry regression. Keep reproducible artifact-size checks on a fixed build recipe. Run timing/memory measurements separately on named hardware with repetitions and noise bounds. Require measured evidence and a Decisions reference for budget changes. Do not rebuild every artifact or rerun the workspace gate on each push.

## Proposed replacement reviewer design

**Policy and exceptions.** Questions have stable IDs and cite exact STRATEGY.md clauses, including relevant Tracks allowances and owner rulings. Question wording, extractor, thresholds and policy excerpts have separate versions. Dormancy is clean; removing unrelated structure at a parameter boundary is a continuity candidate. Code/spec annotations reference scoped, source-backed exceptions. PR Decisions documents those references.

**Deterministic checks.** Enforce production orchestration boundaries with explicit test/backend exceptions. Exercise shipped presets through supported generation entries when values, catalogue membership, entry wiring or relevant dependencies change. Check release artifact budgets from reproducible builds. These checks own blocking failures; live Jev never runs in workspace tests.

**Jev candidates.** Extract changed behavior involving:

1. Parameter-controlled builder selection or suppression of existing structure.
2. Surviving duplicate implementations or redundant blocking transitions.
3. Generated or uploaded output with evidence about actual consumption.

Each candidate includes before/after spans, relevant callees or consumers, removed implementations, the governing clause and applicable exceptions. Cargo features, cfg gates, names, commands and exports are context or deterministic-test triggers only.

**Decision rule.** Jev selects a supported breach mechanism and code-supplied evidence IDs, or `none`/`insufficient_evidence`. Only an above-threshold initial result receives a confirming question that includes legitimate interpretations and exceptions. Warn only when both calibrated per-principle cuts pass and code validates the selected spans. Otherwise abstain. Uncalibrated classes run silently in shadow mode. No Jev result blocks pushes or spec readiness.

**Evaluation.** Keep frozen extraction/replay fixtures separate from live calibration and lineage-separated holdouts. Require every curated positive’s named mechanism to be covered and every curated clean case to remain clean. Report detector misses separately. Advisory deployment targets are ≥90% end-to-end recall, ≥95% precision and ≤1% clean-push false flags, with denominators and uncertainty.

**Modes and cost.** Pre-push and CI advise on newly introduced/worsened behavior; explicit audit mode reports existing debt. Spec mode evaluates current proposals with decision context. Initial engineering budgets are cached p95 ≤0.5 seconds, uncached p95 ≤5 seconds, a 10-second deadline, two batched model phases, at most 12 candidates and 8,000 total input tokens. Measure these limits. Overflow, unavailable service or timeout reports `incomplete`, never “clean”.

**Caching and output.** Cache complete candidate inputs plus extractor, question, model, policy, threshold and exception versions. Show location, principle, before/after evidence, consequence and repair/test command. Deduplicate findings and show at most three by default.

## Minimal cut list

- Remove probabilistic push blocking and the prescribed `--no-verify` workflow.
- Remove name similarity, standalone cfg/features and CLI-command suspicion.
- Remove automatic principle derivation and isolated Architecture-bullet verdicts.
- Remove per-push timing/memory benchmarking and repeated full gates.
- Reuse fn-150’s integration coverage.
- Defer AGENTS/CLAUDE consolidation from this guard’s acceptance criteria.
- Keep deterministic checks, three bounded advisory candidate classes, scoped exceptions and an honest evaluation corpus.