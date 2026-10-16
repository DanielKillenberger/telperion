# Calibrated growth: the oak and the spruce through time

## Goal

Split out of fn-11 on 2026-09-13 so fn-11 could land the growth machine on its own and this research- and judgment-heavy part could start on fresh quota. fn-11 delivered a deterministic specimen with an age and a chronicle, but with placeholder growth traits and no judged age strip. This spec was to calibrate the oak and the spruce against open-grown height and trunk-diameter curves at three ages, route production through the growth path and re-pin once, render age strips and mature stills, and record the owner's verdict.

## Decisions

- > user (2026-09-13): "should we have split the spec?"
- > user (2026-09-13, on splitting calibration and the stills out of fn-11 so fn-11 lands after its bindings and the page, and holding the research-heavy part for fresh quota): "alright let's do it"
- > user (refine of fn-11, 2026-09-12): "we should try and find open-grown curves and match them as closely as possible. The closer to reality the better. We'll judge what rules over what once we get there."
- > user (refine of fn-11, 2026-09-12): "age parameter has to apply to any kind of tree. Spruce and oak we calibrate specifically. But growing should [work] for any tree parameter set."
- Curves by named composition (stand-grown age axis converted through open-grown allometry) since no single open-grown curve exists for either species; sources start from fn-11's `## Resolved via Research`. Traits, not tables; no species branch in growth code.
- Shelved 2026-09-18: growth became a hidden feature (fn-65; AGENTS.md "Mature trees are the product").

## Acceptance Criteria

- **R1:** Oak and spruce follow height and trunk-diameter curves at three ages within 15 percent, sources and SHA-256 in the report — not done (closed).
- **R2:** Production routed through the growth path at the derived mature age; pins re-pinned once with convergence numbers stated — not done (superseded).
- **R3:** Age strips and mature stills accepted by the owner — not done; the uncalibrated strips were rendered but never judged.
- **R4:** Report in fn-14's shape with costs, curves, deviations and verdict slots — not done.

## Result

Closed (2026-10-02) without a build: it calibrates and routes production through the growth path fn-181 removes (owner, 2026-09-27). Last known uncalibrated convergence against the one-shot build: nodes +29% oak and -15% spruce, crossover +50% and +22%, bounds within 2.3% and 6%.

## Open

Calibrated growth over a lifecycle stays a direction in STRATEGY.md; a rewrite is its own spec and starts from `docs/growth-path.md`.
