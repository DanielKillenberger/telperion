# An unstated leaf underside takes the front colour

## Conversation Evidence

> fn-80 live run, 2026-09-24: no admitted source describes the palm's leaf underside; two search rounds found none; `requirements-unmet/leaf_back_colour` reached the owner with `add-sources` as its only option.
> user (2026-09-24): "Default to front colour (Recommended)" — for every species, recorded as a default, tuning refines it from the photographs.
> host (2026-09-24): the underside default also applies when the back's value was dropped by a claim resolution (`replace-source` or `drop-value`, recorded under `dropped` in select's appearance entry) and no replacement source was found, as long as the front is sourced — a dropped underside is an unstated one. Live evidence in the fn-80 worktree's `.flow/evidence/date-palm/pipeline/select.json`: `leaf_back_colour` = {"dropped":"replace-source: date-palm/verify/claim-unsupported//profiles/0/appearance/leaf_back_colour","level":"unstated","source":null} with front `grey_green`, and `requirements-unmet/leaf_back_colour` filed.

## Goal & Context
<!-- scope: business -->

Many sources describe a leaf's colour without separating its faces. When every admitted source leaves the underside unstated and the front colour is sourced, the underside takes the front colour as a recorded default, and tuning refines it against the photographs. [user]

## Architecture & Data Models
<!-- scope: technical -->

- **What exists, checked 2026-09-24 on the fn-80 branch.** fn-133 made an unstated *variation* trait take its zero-width level as a recorded default (select output and profile with its reason and no source, provenance under `defaults`); a colour trait still files `requirements-unmet`. The palm's `leaf_front_colour` is `grey_green` from F1; `leaf_back_colour` is unstated. [checked]
- **Shape.** [host design] The same default path, for `leaf_back_colour` only: when it is unstated and `leaf_front_colour` has a sourced level, the back takes the front's level and its ranges mapped onto the back's material fields, recorded as a default citing the front's source and this rule. No other colour trait defaults.
- **Shape, dropped values.** [host design] A back a claim resolution dropped (`replace-source` or `drop-value`, recorded under `dropped` in select's appearance entry) with no replacement source found on the rerun is an unstated one: the same default applies, as long as `leaf_front_colour` is (still) sourced.

## Acceptance Criteria
<!-- scope: both -->

- **R1:** A red test on the palm's shape: unstated back, grey-green front from F1, gives a back of `grey_green` recorded as a default with its reason; no `requirements-unmet` is filed for it. [inferred]
- **R2:** An unstated back with an unstated front still files `requirements-unmet` for both; bark colour never defaults. [inferred]
- **R3:** The gate is green: `cargo test --profile ci --workspace --no-fail-fast`. [paraphrase]
- **R4:** [host design] A red test first: a back sourced (wrongly) off the same sentence as the front, then dropped by a `drop-value`/`replace-source` resolution with no replacement source found on the rerun - front still `grey_green` from F1 - gives a back of `grey_green` recorded as a default with its reason; no `requirements-unmet` is filed for it, matching R1's outcome for a back that was never sourced at all.

## Decision Context

- The owner chose this rule for every species on 2026-09-24. [user]

## Open Questions

- None.
