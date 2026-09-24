# An unstated leaf underside takes the front colour

## Conversation Evidence

> fn-80 live run, 2026-09-24: no admitted source describes the palm's leaf underside; two search rounds found none; `requirements-unmet/leaf_back_colour` reached the owner with `add-sources` as its only option.
> user (2026-09-24): "Default to front colour (Recommended)" — for every species, recorded as a default, tuning refines it from the photographs.

## Goal & Context
<!-- scope: business -->

Many sources describe a leaf's colour without separating its faces. When every admitted source leaves the underside unstated and the front colour is sourced, the underside takes the front colour as a recorded default, and tuning refines it against the photographs. [user]

## Architecture & Data Models
<!-- scope: technical -->

- **What exists, checked 2026-09-24 on the fn-80 branch.** fn-133 made an unstated *variation* trait take its zero-width level as a recorded default (select output and profile with its reason and no source, provenance under `defaults`); a colour trait still files `requirements-unmet`. The palm's `leaf_front_colour` is `grey_green` from F1; `leaf_back_colour` is unstated. [checked]
- **Shape.** [host design] The same default path, for `leaf_back_colour` only: when it is unstated and `leaf_front_colour` has a sourced level, the back takes the front's level and its ranges mapped onto the back's material fields, recorded as a default citing the front's source and this rule. No other colour trait defaults.

## Acceptance Criteria
<!-- scope: both -->

- **R1:** A red test on the palm's shape: unstated back, grey-green front from F1, gives a back of `grey_green` recorded as a default with its reason; no `requirements-unmet` is filed for it. [inferred]
- **R2:** An unstated back with an unstated front still files `requirements-unmet` for both; bark colour never defaults. [inferred]
- **R3:** The gate is green: `cargo test --profile ci --workspace --no-fail-fast`. [paraphrase]

## Decision Context

- The owner chose this rule for every species on 2026-09-24. [user]

## Open Questions

- None.
