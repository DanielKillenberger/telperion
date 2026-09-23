# A family's rows can be validated without growing a tree

## Conversation Evidence

> fn-113 FRICTION.md, 2026-09-23: "the bounds are spread over ten validators, and there is no one call that checks a family's rows without growing a tree ... about 25 minutes of reading before any edit."
> user (2026-09-23): "yes pls fix the frictions if they're obvious."

## Goal & Context
<!-- scope: business -->

The tuning loop's dial table and every preset are checked against the generator's bounds, but the bounds are enforced piecewise, some only inside placement or `Specimen::new`, so a caller that wants to know whether a family is valid has to grow a tree. One entry point that validates a whole family without growth makes the dial-table tests, preset checks and tuning preflight direct. [paraphrase]

## Architecture & Data Models
<!-- scope: technical -->

- **What exists.** Confirmed by the implementer against the code at 9ac7d064 (2026-09-23). [checked]
  - Row validators that need no tree: `Envelope::validate`, `BiasParams::validate`, `HabitParams::validate`, `RadiusParams::resolved`, `TwigParams::resolved`, `SurfaceParams::validate`, `build_element`, `MaterialParams::validate`, `GrowthTraits::validate` with `Age::from_years`, and `GrowthConfig::validate` through `SkeletonParams::resolved_growth`.
  - `params::parse` already runs the material, age and growth checks; nothing else is judged at the wire.
  - Only inside `Specimen::new`: `samplingAttemptsPerAttractor`, `attractors <= MAX_ATTRACTORS`, `step > 0`, a positive attractor weight with no attractors, `resolved_growth` and `stems_placed` (stems outside the envelope or through each other).
  - Only inside placement, behind `tree.validate_solved()`: the canopy rails in `foliage/canopy.rs::validate`, with the short-shoot rows, the rosette rows and the twig placement rails. The rosette rows alone are also reached tree-free as `foliage::validate_canopy`, which the leaf bases call.
  - Only after growth: `shell_depth` in `cull`, the envelope's height above zero in `surface::build`, and the foliage reference box's finiteness in `Instances::validate`.
  - Growth-dependent, not a row bound: attractor scattering that falls short of its count (`ResourceLimit`, seed and envelope together). `Family::validate` leaves it to growth.
- **Shape.** `Family::validate(&self) -> Result<()>` in `telperion-core` calls every row set's existing validator and the checks now reached only during growth, with no tree built; the existing call sites keep their checks. [inferred]

## Acceptance Criteria
<!-- scope: both -->

- **R1:** `Family::validate` refuses every value the growth path refuses, for every row, with no tree grown; a test walks each row past its bound on every family. [inferred]
- **R2:** fn-113's `dial_bounds.rs` uses it in place of its one-node tree. [inferred]
- **R3:** The gate is green: `cargo test --profile ci --workspace --no-fail-fast`. [paraphrase]

## Decision Context

- Owner's standing rule of 2026-09-23: obvious friction fixes are the host's to spec. [user]

## Open Questions

- The unknown in Architecture is confirmed by the implementer before any edit.
