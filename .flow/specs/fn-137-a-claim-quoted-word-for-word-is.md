# A claim quoted word for word is accepted by code

## Conversation Evidence

> fn-80 live run, 2026-09-24: the palm's height "50 - 100 feet", verbatim in A1's size table, was flagged `claim-unsupported` (Jev "supports" at 0.65 against the 0.8 auto-accept cut); the cheap tier may only `replace-source` or `drop-value`, the search rounds were spent, and the decision reached the owner.
> user (2026-09-24): "Accept, and fix (Recommended)" — verbatim spans are accepted by code so they stop reaching the owner.

## Goal & Context
<!-- scope: business -->

A value whose exact span stands in the cited sentence of an admitted source is supported by the source text itself; Jev's confidence that the page supports it only matters when the value is not quoted verbatim. Today every verbatim value on a noisy page reaches the owner. [user]

## Architecture & Data Models
<!-- scope: technical -->

- **What exists, checked 2026-09-24 on the fn-80 branch.** `cite.rs` (the listing rule around line 345-370) lists a claim whose relation is `supports` when `confidence < cuts.citation_auto_accept`; the palm's height was listed at 0.65 below 0.8. [checked]
- **Shape.** [host design] Code checks, before the confidence cut, whether the claim's span (the number and unit as selected) occurs verbatim in the cited sentence, after whitespace normalisation only. A verbatim span with relation `supports` is not listed on confidence; the other listing rules (site-quality criterion, height-at-age kind) still apply. A non-verbatim span keeps today's rule. The record says `verbatim` so the reason is visible.

## Acceptance Criteria
<!-- scope: both -->

- **R1:** A red test on the palm's height claim ("50 - 100 feet" in A1's sentence, supports at 0.65): after the fix it is not listed, and the record says verbatim. [inferred]
- **R2:** A supports claim whose span is not in its sentence is still listed below the cut; a verbatim span on a site-quality criterion is still listed. [inferred]
- **R3:** The gate is green: `cargo test --profile ci --workspace --no-fail-fast`. [paraphrase]

## Decision Context

- The owner approved this fix on 2026-09-24. [user]

## Open Questions

- None.

## Settled

Closed as landed (2026-09-26, fn-149 R8): carried by the fn-149 runner rewrite, merged in #121 (c2ac430b).
