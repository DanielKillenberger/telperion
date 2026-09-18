# Jev in CI: the citation gate and the licence scan

## Conversation Evidence

> user (2026-09-19): "ok can you add this to CI? also can add a CI job to scan the docs for leaked licensed material that we shouldn't have"
> user (2026-09-19): "license check" / "using jev"
> user (2026-09-19): "i can add a jev API key" / "I added JEV_API_KEY"

## Goal & Context
<!-- scope: business -->
<!-- Goal & Context: 50% [user], 30% [paraphrase], 20% [inferred] -->

fn-81 built the citation check and wired it into the pipeline, but it never ran over the five articles that shipped, because it reads a run's own fetch and selection artifacts and four of the five species predate the pipeline. Those five articles therefore rest on a person's reading. The owner wants the check to run in CI so no article lands on a claim its source does not support, and wants a second job that scans what the repository ships for licensed material it should not be redistributing. Both run on Jev. The owner added `JEV_API_KEY` as a repository secret on 2026-09-18. [user]

The repository is public and now carries copies of other people's writing, four of them full copies under an open licence or the public domain and eighteen as cited passages. The structure check already refuses a full copy filed under rights that do not permit one. What it cannot see is an extract that has grown into a de facto reproduction, or licensed text that reached `docs/` or an article by another route. That judgment is a reading, which is what Jev is for. [paraphrase]

## Architecture & Data Models
<!-- scope: technical -->

- **The citation check reads the committed source copies, not a run's artifacts.** Today the document stage opens `fetch.json` and `select.json` to know what was fetched. A second entry point takes a species folder and checks every cited claim in its `ARTICLE.md` against the copy under `sources/` that the citation names. This is what makes the check runnable in CI at all, and it is also what makes the five backfilled articles checkable for the first time. The pipeline stage keeps its existing path; both call one function. [user]
- **Two jobs, one key, one budget.** `cited` runs the citation check; `licensed` runs the licence scan. Both read the key from `JEV_API_KEY` in CI and `TYPESAFE_API_KEY` locally, through the existing shared caller, and neither ever prints it. Both write their ledger to the job's artifacts and commit nothing. [user]
- **Scope is the diff, not the catalogue.** `cited` checks only the species whose `ARTICLE.md` or `sources/*.md` changed against the merge base, so a pull request touching one species costs one species' calls rather than the whole catalogue's. A change to the question set, the thresholds or the checker itself widens the scope to every species, because the judgment changed. [inferred]
- **A budget cap that fails loudly.** Each job declares a maximum number of judgments for one run. Code counts the candidates before it spends anything, and a run whose candidate count exceeds the cap fails immediately naming the count and the cap, asking for a narrower change, rather than spending the quota and then failing. The fn-13 weekly-quota incident is why this is a precondition rather than a running total. [paraphrase]
- **The licence scan extracts in code and judges by reading.** Code walks every text file the repository ships under `catalogue/`, `docs/` and `README.md`, and extracts each candidate: a quoted block in a source copy, a verbatim run above a length code fixes, and the source each one names. Code measures the run's length and its share of the copy, and reads the source's rights. Jev answers one question per candidate over described options, with a no-match answer: the passage is a brief quotation, a substantial reproduction of the named source, or not attributable to it. A substantial reproduction under rights that do not permit a copy fails the job naming the file, the passage and the source. [user]
- **The structural half stays in code.** A `full` copy under non-permitting rights, a missing rights field and a raster outside the images directory are already the structure check's, and stay there. The scan adds only the judgments code cannot make. A candidate the scan cannot attribute to any source is reported, never failed, because unattributed text in this repository is usually the project's own prose. [inferred]
- **A fork skips, and says so.** A pull request from a fork has no secret. Both jobs detect the absent key, skip, and report the skip as their result with the reason, so an absent key never reads as a pass. The branch protection that requires these jobs, if the owner adds one, is the owner's call and not this spec's. [inferred]
- **Judgments propose.** Neither job writes a verdict, a memory entry or a receipt. A failure is a job failure with its evidence in the log and the ledger in the artifacts; resolving it is a person's, as every Jev judgment in this repository is. [paraphrase]

## API Contracts
<!-- scope: technical -->

- **Check entry point** `cargo run -p telperion-jev --bin jev -- cite-article --species <id> [--species <id>...] --json` reads `catalogue/<id>/ARTICLE.md` and `catalogue/<id>/sources/*.md`, and prints one row per claim: claim, source id, section, relation, confidence and whether it passed the threshold. Exit is non-zero when any claim is contradicted, or unsupported above the citation threshold. [inferred]
- **Scan entry point** `cargo run -p telperion-jev --bin jev -- licence-scan [--paths <glob>...] --json` prints one row per candidate: file, passage location, named source, measured run length, measured share, the source's rights, the chosen option and its confidence. Exit is non-zero on a substantial reproduction under non-permitting rights. [inferred]
- **New question set** `crates/telperion-jev/data/questions/licence.json`, one `choice` question with the three described options above, each with its criterion, and a no-match answer. Versioned like its siblings. [paraphrase]
- **New labelled set** `crates/telperion-jev/data/cases/licence.json` with positive, negative and held-out cases, including a brief quotation that must not fail, a long verbatim run that must, and project prose that must come back unattributable. Its threshold joins `thresholds.json`. [paraphrase]
- **Workflow jobs** `cited` and `licensed` in `.github/workflows/tests.yml`, each with `JEV_API_KEY` from secrets, each path-filtered to the trees it reads, each uploading its ledger as an artifact. [user]

## Edge Cases & Constraints
<!-- scope: technical -->

- **The isolation guard stays green.** Jev still never appears in `crates/telperion-core`, `crates/telperion-render`, `crates/telperion-wasm` or `src`, and the workspace test commands still run with the key unset. A CI job is neither, so nothing in the standing rule bends; the rule's own words are that Jev may run in evidence tooling, research checks, QA triage and report assembly. [paraphrase]
- **Candidate coverage is tested before a selection is trusted.** A test asserts the extractor finds every quoted block in a fixture source copy and every verbatim run above the cut, because a scan that misses a candidate reports a clean repository it never read. [paraphrase]
- **The key is never printed,** never written to a log line, never echoed into a failure message, and the ledger records the model and the question-set version rather than the request. [user]
- **A claim whose key terms match no section** is already recorded as unsupported without a call; that path costs nothing and stays. [inferred]
- **The five backfilled articles are the first real run.** The spec is not complete until `cited` has run over all five on this branch and every claim it fails has been resolved, either by rewriting the sentence or by recording why the relation is wrong. That run is the acceptance evidence, and it is the roughly 150 calls fn-81 deferred. [user]
- **Cost is stated, not discovered.** The report prints the call count and the cap for each job, so the owner can see what a pull request spends before deciding whether to require the jobs. [inferred]
- **No new species, no preset, generator or renderer change, no change to the article prose beyond what a failed claim forces.** [paraphrase]

## Acceptance Criteria
<!-- scope: both -->

- **R1:** The citation check runs from a species folder's committed `ARTICLE.md` and `sources/*.md` with no run artifacts present, through the same function the pipeline's document stage calls, and reports per claim the source, section, relation and confidence. Errors: a cited source with no copy in the folder fails naming the claim and the source. [user]
- **R2:** A `cited` job in the Tests workflow runs that check over every species whose article or source copies changed against the merge base, widening to every species when the question set, the thresholds or the checker changed, and fails naming each contradicted claim and each unsupported claim above the citation threshold. Errors: an absent `JEV_API_KEY` skips the job and reports the skip with its reason, never a pass. [user]
- **R3:** A `licensed` job scans every text file the repository ships under `catalogue/`, `docs/` and `README.md`. Code extracts each quoted block and each verbatim run above a fixed length with the source it names, and measures its length and its share of the copy; Jev chooses among brief quotation, substantial reproduction, and not attributable, with a no-match answer; the job fails on a substantial reproduction under rights that do not permit a copy, naming the file, the passage and the source. Errors: an unattributable candidate is reported and never fails the job. [user]
- **R4:** The licence question set ships with a labelled set carrying positive, negative and held-out cases, its threshold lives in `thresholds.json`, and a coverage test asserts the extractor finds every quoted block and every over-length run in a fixture. Errors: a labelled case the question set answers wrongly fails the crate's tests. [paraphrase]
- **R5:** Each job counts its candidates before spending, fails immediately when the count exceeds its declared cap, naming the count and the cap, and prints the calls spent and the cap in its report; neither job writes a verdict, a memory entry, a receipt or any committed file, and the key never reaches a log line. Errors: a run that exceeded its cap reports zero calls spent. [user]
- **R6:** `cited` has run over all five backfilled articles on this spec's branch, every failing claim is resolved in the article or recorded with why the relation was wrong, and the run's call count is in the spec's evidence. Errors: an unresolved failing claim blocks the spec. [user]

## Boundaries
<!-- scope: business -->

- No branch protection change; whether these jobs are required to merge is the owner's call, made after seeing what a run costs. [inferred]
- No change to the isolation guard, and no Jev call from the workspace test commands. [paraphrase]
- No change to the existing citation question set or its threshold; this spec adds the licence set beside it. [inferred]
- No new species, preset, generator or renderer behaviour. [paraphrase]
- No scan of git history; this spec gates what the repository ships from here, and an earlier commit's content is a separate question. [inferred]
- No legal advice and no licence classification beyond the rights text each source already records. [inferred]

## Decision Context
<!-- scope: both -->

### Motivation

- fn-81 shipped five articles whose claims no machine has checked, and the owner asked for the check to run in CI rather than stay a manual reading. [user]
- The repository is public and now holds other people's text; the owner asked for a job that catches licensed material that should not be there. [user]

### Implementation Tradeoffs

- Reading the committed copies over a run's artifacts: it is what makes the check runnable outside a pipeline run at all, and it lets the five backfilled articles be checked. The pipeline keeps its path; both call one function. [user]
- Scoping to the diff over the whole catalogue: at three hundred species a full check is unaffordable per pull request, and a changed article is the only thing a pull request can have broken. A changed judgment widens the scope because then every article is newly in question. [inferred]
- A precondition cap over a running total: fn-13 spent a weekly quota before anyone noticed. Counting candidates first makes the spend knowable before it happens. [paraphrase]
- Code measures, Jev reads: the length of a passage and its share of a copy are arithmetic, and whether a passage reproduces a source is a reading. Splitting them is the project's standing TypeSafe rule, not a choice made here. [paraphrase]

## Parked unknowns

- Whether the licence scan should also read a source's licence identifier once sources carry one, rather than its rights sentence.
- What the caps should be, in numbers; the first run over five species is what sets them.
- Whether `cited` should become a required check once its cost is known.

## Strategy Alignment

- Follows "The catalogue": the species record is the unit, and this keeps its prose honest against the sources it cites.

## Resolved via Codebase

- `JEV_API_KEY` exists as a repository secret, set 2026-09-18; CI uses no secret today (`gh secret list`, and no `secrets.` reference in `.github/workflows/tests.yml`).
- The local key is `TYPESAFE_API_KEY`, read through the shared caller below the non-interactive guard in `~/.bashrc`.
- The citation question set is one `choice` question over supports, contradicts and says_nothing, with unit conversion allowed as paraphrase (`crates/telperion-jev/data/questions/citation.json`); its labelled set holds 9 cases (`data/cases/citation.json`); `citation_auto_accept` is 0.80 in `data/thresholds.json`.
- `cite()` already records a claim whose key terms match no section as unsupported without a call (`crates/telperion-jev/src/cite.rs`).
- The document stage opens `fetch.json` and `select.json`, which is why the check cannot run for the four pre-pipeline species.
- The isolation guard forbids the Jev endpoint and crate names under `crates/telperion-core`, `crates/telperion-render`, `crates/telperion-wasm` and `src` (`crates/telperion-jev/src/isolation.rs`).
- The Tests workflow runs `receipts`, `core` in four shards, `render`, `wasm and jev`, `rust-receipts` and `node`.
- The catalogue ships 22 source copies, 4 full and 18 extracts, and 5 articles of 89 to 108 lines.

## Requirement coverage

| Requirement | Task |
|---|---|
| R1–R6 | TBD |
