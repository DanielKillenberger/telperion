# A resolved claim stays resolved while the claim is unchanged

## Conversation Evidence

> fn-80 live run, 2026-09-24: the owner's and the routine tier's resolutions on the palm's appearance claims (bark roughness, leaf underside, hue, height) were voided by every stage rerun and asked again, because each claim decision is keyed to the whole `select.json` checksum.
> user (2026-09-24): "Accept, and fix (Recommended)" — a resolution is keyed to its claim (field, value, source span), so reruns stop re-asking.

## Goal & Context
<!-- scope: business -->

A resolution answers a claim: this value, from this source's sentence, for this field. Any change elsewhere in `select.json` (another field filled, a default added) voids it today, so the owner is asked the same question again. The decision must be keyed to the claim itself. [user]

## Architecture & Data Models
<!-- scope: technical -->

- **What exists, checked 2026-09-24 on the fn-80 branch.** `pipeline/stages/verify.rs` files `claim-unsupported`, `claim-contradicted`, `obligation-unmet` and `structural-unmet` with `inputs_sha256` = `{"select.json": <whole-file sha>}` (lines 85, 156, 267, 358); `decision.rs:220` `apply_resolutions` binds a resolution only while its `inputs_sha256` equals the decision's. [checked]
- **Shape.** [host design] Verify keys each claim decision on the claim: a checksum over its pointer, the selected value (level or range), the source id and the cited sentence. A rerun that leaves that claim unchanged files the same inputs, so its resolution still binds; a changed value, source or sentence voids it as today. Existing decisions keyed the old way stay void once and are refiled on the new key.

## Acceptance Criteria
<!-- scope: both -->

- **R1:** A red test: a resolved claim stays resolved when another field in `select.json` changes, and is voided when its own value, source or sentence changes. [inferred]
- **R2:** The gate is green: `cargo test --profile ci --workspace --no-fail-fast`. [paraphrase]

## Decision Context

- The owner approved this fix on 2026-09-24. [user]

## Open Questions

- None.
