# Select keeps the reference photographs a run recorded

## Conversation Evidence

> fn-80 live run, 2026-09-24: the palm's fourth tuning revision paused "baseline infeasible" ("no matched shots or height"): `packet/references.json` held no references. The three reference photographs with their matched shots, recorded on 2026-09-22 (commit 0995dac9), were overwritten with an empty list on the 2026-09-23 stage rerun (ba8a92e6).

## Goal & Context
<!-- scope: business -->

A species' reference photographs and their matched shots are recorded once and judged against in every tuning revision. A literature rerun must never erase them. [paraphrase]

## Architecture & Data Models
<!-- scope: technical -->

- **What exists, checked 2026-09-24 on the fn-80 branch.** `pipeline/stages/select.rs:211-212` writes `packet/references.json` as `{"reference_version": "fn19-references-v1", "sources": <sources>, "references": []}` on every run; `tuning/matched.rs:62` refuses a references file with no `shot` records ("no matched shots or height"). [checked]
- **Shape.** [host design] Select rewrites only the `sources` it owns and keeps every existing `references` entry byte for byte; a first run with no file writes an empty list as today.

## Acceptance Criteria
<!-- scope: both -->

- **R1:** A red test: a references file holding the palm's three recorded shots keeps them, byte for byte, after a select rerun with changed sources. [inferred]
- **R2:** The gate is green: `cargo test --profile ci --workspace --no-fail-fast`. [paraphrase]

## Decision Context

- An obvious friction fix under the owner's standing rule of 2026-09-23. [paraphrase]

## Open Questions

- None.

## Settled

Closed as landed (2026-09-26, fn-149 R8): carried by the fn-149 runner rewrite, merged in #121 (c2ac430b).
