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
TBD

## Evidence
- Commits:
- Tests:
- PRs:
