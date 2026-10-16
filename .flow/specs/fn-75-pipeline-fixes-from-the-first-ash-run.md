# Pipeline fixes from the first ash run

## Goal
The owner wants the species pipeline to run under a cheap driver from seed manifest to report, a person stepping in only for choices a person owns, so a species can be defined, iterated with Jev and hill-climbed toward a tree that passes a visual gate. The first live run (European ash under a Grok driver, 2026-09-18) needed six driver dispatches instead of two and 107 Firecrawl credits because four bookkeeping defects made a person babysit it. This spec fixes those four and two smaller ones, changing no value table, generator behaviour or question set.

## Decisions
- > user (2026-09-18): "ok merged so can we now have a test run with a cheap model adding a new species right?"
- > user (2026-09-18): "the loop should identify gaps also at the end and propose specs to glose the gaps. These should escalate to frontier models if jev cannot judge the likelihood of the proposed gap fix actually fixing the gap. So having a species that already needs a spec should not be a blocker for the loop. The loop should fire off specs to close the gaps"
- > user (2026-09-18): "make sure to keep notes on how to improve this loop"
- > user (2026-09-18): "so i was hoping that we'd define a tree species iterate with jev, judge what specs need to be added to cover gaps to render the tree and then this way hill climb to a tree that passes the QA model viewing the tree visual gate so i can evaluate it at the end. Only to stop if the path isn't clear or needs a high stakes decision that can't be decided by frontier model who's orchestrating"
- > user (2026-09-18): "i'm not following pls explain this without mannered prose for stupid human"
- > user (2026-09-18, after the plain-language list of the four defects): "yes /flow-next:capture"
- Separate from fn-58 because fn-58 had landed; the gap loop is fn-63's, the tuning loop fn-68's, a visual gate its own spec. A person still admits the manifest and resolves data-insufficient and onboarding-gate decisions; this spec makes those decisions stick.

## Acceptance Criteria
- **R1:** Admitting the manifest does not rerun discovery or reissue the proposal; the driver reaches fetch on the first dispatch after admission — met (discovery keyed on `manifest::seed_sha256`).
- **R2:** retry, replace-source and drop-source each change the next fetch and record `consumed_by`; an option no stage consumes is refused — met (`pipeline/consume.rs` kinds table).
- **R3:** An admitted table's block label yields only its rows; Esche yields eleven rows 20-120 years; the guard is two-sided (31 vs 11) — met.
- **R4:** Discovery lists sources admitted manifests already name before any search; Ertragstafeln ranked for ash height — met (`pipeline/known.rs`).
- **R5:** The raw request trusts the system store; the Missouri Botanical Garden page fetches — partial: switched to ureq `native-certs`, but that server omits its intermediate (verify code 21, curl fails too), so the page is R5's error case, filed verbatim.
- **R6:** The report lists Firecrawl credits and Jev calls per stage and total; an ash rerun shows one discovery — met (`pipeline/cost.rs`).

## Result
All in `crates/telperion-jev` (commit 78d6e331), tests in `tests/pipeline_fixes.rs`; `cargo test --release -p telperion-jev` 140 passed, clippy and fmt clean. Esche block reads 12.0 to 33.1 m; a missing `Eiche` block files coverage-gap naming the labels present.

## Open
- Fetching a page whose server omits its intermediate (CLI `rawHtml` fallback or AIA chasing): owner's call.
- A resolution deleted from `resolutions.json` stays applied in `decisions.json` (pre-existing).

## Friction
- Branch cut from a local master that never pulled PR #34, so the pipeline code was absent -> merged `origin/master`; fix is to cut from `origin/master`.
- Codex quota refused the implementer -> session model; fix is a reach check in the conductor.
- R5's inferred positive example was a misconfigured server -> live test removed, error case pinned.
