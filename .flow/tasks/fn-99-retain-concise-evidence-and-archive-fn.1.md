---
satisfies: [R1, R2, R3]
---
# fn-99-retain-concise-evidence-and-archive-fn.1 Archive raw fn-91 output and establish evidence retention

## Description
Archive and checksum raw fn-91 outputs before removal; keep reports and replay sources, document recovery and retention, verify, open the PR.

## Acceptance
- Spec R1-R3.

## Done summary
Removed 602 fn-91 artifacts (19.5 MB), kept 232 reports/summaries/replay sources plus a recovery README and checksum manifest; recoverable from `c4871093`. A verify script passed checksums, ignore rules, Markdown links and exclusions; `flowctl validate` passed.

## Evidence
- Commits: 3f6476e4
- Tests: archive verify script (602 files), git diff --cached --check, flowctl validate --spec fn-99
- PRs:
