---
satisfies: [R1, R2, R3, R4, R5, R6]
---
# fn-112-publish-telperion-to-npm-with-a-release.1 Implement Publish telperion to npm with a release pipeline in CI

## Description
Add a tag-driven release workflow that gates, builds and publishes `telperion` to npm with provenance.

## Acceptance
Spec R1-R6.

## Done summary
R1-R5 and R7 landed in PR #63 (`dd86fa05`). R6: the owner created the package with placeholder 0.0.1 and attached the trusted publisher; Release run 35791697498 (attempt 2) published telperion@0.1.0 with SLSA provenance and no token. A fresh install without Rust on Node v26.8.1 passed the field smoke and `npm audit signatures`.

## Evidence
- Commits: dd86fa05
- Tests: gh run rerun 35791697498 --failed; npm install telperion@0.1.0 && npm audit signatures && field smoke
- PRs: https://github.com/DanielKillenberger/telperion/pull/63
