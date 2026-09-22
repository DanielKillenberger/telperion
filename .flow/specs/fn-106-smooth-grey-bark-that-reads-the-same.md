# Smooth grey bark that reads the same near and far

## Conversation Evidence

> user (2026-09-22, on the beech stills): "The materials from afar doesn't look like the detailed view?"
> user: "let's mint them specs but check how things are correlated."

## Goal & Context
<!-- scope: business -->
<!-- Goal & Context: 40% [user], 40% [paraphrase], 20% [inferred] -->

Two observations on the beech, one from the reviewer and one from the owner. The reviewer, at the base view: "pervasive tile-like relief and strong horizontal scoring" against the reference's continuous smooth grey, and the marks "remain conspicuous at whole-tree scale". The owner, on the same stills: the bark from afar does not look like the detailed view. The fn-68 materials run (six rounds, 91 material dials) never isolated a family: every bundle darkened the bark, lowered roughness, raised `ridge_scale` from 0.002 to 0.012 in one half step, and switched bark plates on from zero. This gap is independent of the structural ones. [paraphrase]

## Architecture & Data Models
<!-- scope: technical -->

- **Two unknowns come first, and each is a measurement.** One: can the material dials reach a smooth surface at all? Author a hand overlay for smooth grey (ridge and plate strengths near zero, mottle and lichen low) and render the base view; the reviewer's sheet grades it against the current tree. Two: does the far look diverge from the near because of the renderer's distance terms? Render the trunk at the base view's distance and at the whole-tree distance with the same overlay and compare against the decision "the far draw is the near draw minus what the eye cannot resolve" (`knowledge/decisions/the-far-draw-is-the-near-draw-minus-2026-09-18`). [host design]
- **Outcomes.** If smooth is reachable and near matches far, this is a tuning fix: the dial table's material steps are re-authored (the `ridge_scale` step is an order of magnitude too large) and the beech's rows move through fn-62. If near does not match far, it is a renderer defect and its own criterion here. If smooth is not reachable, one material parameter is the gap and is stated on the evidence. [inferred]

## Acceptance Criteria
<!-- scope: both -->

- **R1:** Both measurements are run and recorded with stills, overlay and grades before any code changes.
- **R2:** Any material step re-authored in the dial table is cited to a rendered comparison, and every row still reads a number on every family's wire.
- **R3:** If a renderer defect is found, a red/green test on the 8-tree forest reproduces it before a fix, per the capture rule; at most one full-forest capture per commit.
- **R4:** The owner's eye on the base and whole-tree stills decides.

## Boundaries
<!-- scope: business -->

- Independent of fn-103. No species branch; the beech's values ship through fn-62.

## Strategy Alignment

- Serves "Growth and botanical fidelity". [strategy:Growth and botanical fidelity]
