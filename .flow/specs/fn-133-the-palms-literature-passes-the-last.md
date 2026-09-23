# The palm's literature passes: the last four stops

## Conversation Evidence

> host, 2026-09-24, the palm's rerun on fn-132: trunk diameter fails on `no_mature_size` at `proxy_only`; height scores `proxy_only` against a `partial` bar; leaflet sizes pass quality but select's floor rejects the picks; leaf brightness range unstated in every source.
> user (2026-09-24), on the host's proposal to close all four in one last spec: "yes"

## Goal & Context
<!-- scope: business -->

Four stops remain on the palm's literature, none a new kind of defect. After this spec the host brings any further stop to the owner instead of speccing. [user]

## Architecture & Data Models
<!-- scope: technical -->

- **What exists, checked 2026-09-24 on the fn-80 branch (`.flow/evidence/date-palm/pipeline/`).** [checked]
  - `quality.json`: `dbh_m` level `proxy_only`, 3 points, gap `no_mature_size`, `passed: false` against bar `proxy_only`; fn-132 replaced `no_mature_size` only for `partial` and `sufficient` levels (`stages/quality.rs`, `run`).
  - `height_m`: 12 points, level `proxy_only`, gap `bound_only`; the palm row in `data/species-requirements.json` asks `partial`.
  - `leaflet_length_m`, `leaflet_width_m`: quality `sufficient` with 7 points; `select.json` unavailable "pick below the selection floor" (floor 0.34, fn-131, from few labelled cases).
  - `leaf_brightness_range`: `unstated` in every source; `docs/species-onboarding.md` already says a species whose foliage reads uniform states a zero-width range.
- **Design.** [host design; the bar per the owner's choice of 2026-09-23]
  - *The gap follows the points at every level.* A field with any mature-size point never carries `no_mature_size`, whatever its level.
  - *Palm height is a mature range.* The palm row asks `height_m` at `proxy_only`, like `dbh_m`: the owner chose growth rate plus mature range for palms, and a mature range is scored as a bound.
  - *No selection floor until it is calibrated.* The selection floor applies only when its labelled set holds at least 20 cases with at least 5 wrong picks; below that, select takes the most probable span and records its probability, and verify's field-aware check stays the guard.
  - *An unstated variation is zero width.* A variation trait (hue range, brightness range) that every source leaves `unstated` takes the zero-width level, recorded as a default with its reason, not as a sourced value; a colour or roughness trait still stops.

## Acceptance Criteria
<!-- scope: both -->

- **R1:** A red test on the palm's `dbh_m` rows: `proxy_only` with 3 points passes a `proxy_only` bar with no `no_mature_size`. [inferred]
- **R2:** The palm row asks `height_m` at `proxy_only`; broadleaf and conifer rows are unchanged. [inferred]
- **R3:** With the labelled set below the calibration minimum, select fills the palm's leaflet sizes from their most probable span and records the probability. [inferred]
- **R4:** An unstated `leaf_brightness_range` becomes the zero-width level with a default record; an unstated colour still files `requirements-unmet`. [inferred]
- **R5:** The gate is green: `cargo test --profile ci --workspace --no-fail-fast`. [paraphrase]

## Boundaries

- No other change to the literature step. [user]

## Decision Context

- The owner approved this as the last literature spec on 2026-09-24. [user]

## Open Questions

- None.
