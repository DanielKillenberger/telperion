---
satisfies: [R1]
---
# fn-312-closed-specs-keep-their-result-and.2 Write the closed-spec rule into the retention doc and the manifest

## Description
R1: the rule lives in `docs/evidence-retention.md`, and the prune is recoverable from the manifest.

**Size:** S
**Files:** `docs/evidence-retention.md`, `.flow/evidence/ARCHIVE-MANIFEST.md`, `AGENTS.md`
**Touches:** [docs/evidence-retention.md, .flow/evidence/ARCHIVE-MANIFEST.md, AGENTS.md]

### Approach
- `docs/evidence-retention.md`: add a short section for closed specs stating R1 (what a done spec keeps, what goes, the open-spec and fn-237 protection, the readers and FRICTION hold), and replace the sentence "This policy does not retroactively delete other specs' evidence" (line 19) so the doc no longer contradicts the prune. Keep the owner's date (2026-10-24) and words.
- `.flow/evidence/ARCHIVE-MANIFEST.md`: a dated section above the 2026-10-16 file list naming the source revision (the commit before task 1's first prune commit), the release PR ref `refs/pull/159/head` as the squash-proof copy, the recovery command for a file or a spec's tasks, and the counts removed. No per-file list (spec, Decision Context).
- `AGENTS.md:89` (Evidence retention) gains one sentence pointing at the closed-spec rule; `AGENTS.md:35` (Friction reports) gains that a closed spec's FRICTION.md is removed once its dispositions are in the spec.

### Investigation targets
**Required:**
- `docs/evidence-retention.md`
- `.flow/evidence/ARCHIVE-MANIFEST.md:1-15`
- `AGENTS.md:35`, `AGENTS.md:89`

### Key context
- Plain owner prose, no new headings in AGENTS.md; follow the doc's own voice.

## Acceptance
- [ ] `docs/evidence-retention.md` states R1 and no longer says the policy does not delete other specs' evidence.
- [ ] The manifest names the source revision, `refs/pull/159/head`, a recovery command checked by running it for one removed file, and the counts.
- [ ] AGENTS.md's retention and friction paragraphs point at the rule.

## Done summary
`docs/evidence-retention.md` gains a "Closed specs" section that states R1 with the owner's words from 2026-10-24, and it drops the sentence saying the policy does not delete other specs' evidence. `.flow/evidence/ARCHIVE-MANIFEST.md` gains a dated 2026-10-24 entry above the 2026-10-16 one. The entry names source revision f6d89317d, `refs/pull/159/head`, the three prune commits and the counts (677 files and 26,225 lines; 255,825/406,703 to 245,060/422,110 against origin/master), with no per-file list. AGENTS.md's Evidence retention and Friction reports paragraphs each gain one sentence pointing at the rule.

The recovery command is `git archive f6d89317d -- <path> | tar -x`. In a scratch worktree it restored `.flow/evidence/fn-102-one-build-pipeline-in-place-of-the/REVIEW-2026-09-23.md`, whose blob id matched, and fn-102's task pair. dcg blocks the `git show > path`, `checkout` and `restore` spellings, which is logged in FRICTION.md. `flowctl validate --all` still reports 1 error, the fn-82 error that predates this spec.

Baseline: none (docs-only; the task-1 handoff changed only .flow/ paths).
Tier: (model: claude-opus-5-5)

stage: impl-review - ran [triage-skip SHIP, docs-only (4 files), 2026-10-24]

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: d6657e9366577b272d16038f7670bc34339ba864, 4bb24f11d736162a965ac87f2c4e97cac1c25fdd
- Tests: flowctl validate --all --json | jq .total_errors (1, pre-existing fn-82), git archive f6d89317d -- <removed path> | tar -x in a scratch worktree; blob id matched
- PRs: