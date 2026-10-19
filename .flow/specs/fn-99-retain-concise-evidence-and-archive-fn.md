# Retain concise evidence and archive fn-91 run output

## Goal

The owner requested a cleanup PR after fn-91 merged hundreds of evidence files: keep the repository useful for review and reproduction without carrying every experimental run. [user]

## Decisions

- No Git history rewrite; the merged Git revision stays the portable recovery source, and a local archive is a convenience, not shared hosting.
- A retention convention (`docs/evidence-retention.md`, ignored `.flow/evidence/<spec>/raw/`) keeps future raw evidence out of normal commits; uncommitted evidence needs durable storage before its only copy is deleted.

## Acceptance Criteria

- **R1:** Archive removed fn-91 artifacts with checksums and a recovery path from their original revision; keep reports, decisions and verification scripts with their inputs — met.
- **R2:** Remove bulky run output without touching engine code, test fixtures or unrelated specs — met.
- **R3:** Document and enforce the retention convention, check retained references, open the cleanup PR — met.

## Result

Removed 602 generated fn-91 artifacts (19,536,031 bytes); kept 232 reports, JSON summaries/provenance, replay sources and patches, plus fn-91's recovery README and checksum manifest (`ARCHIVE.json`) (archived: see .flow/evidence/ARCHIVE-MANIFEST.md, `git show 8dab10634:.flow/evidence/fn-91-fast-tree-generation-as-a-core-engine/ARCHIVE.json`). Originals are recoverable from revision `c4871093bd7ef208c2c3f65fb46a223c4e397a0b`. A host-side verify script checked all 602 checksums against the local archive, the ignore rules, retained Markdown links and code/test exclusions (commit `3f6476e4`).

## Open

None.

## Friction

- Local SSH agent refused signing (~2 min) -> HTTPS fetch; local setup, reported only.
