# Directional overlapping bark flakes

## Conversation Evidence

> user (turn 1): "the new version is much better. reference has more varied coloring with some red/brown layered in and also the texture is more layered almost all in the same direction around the bark. Left lower and to the right layered on top the previous layer. Does that make sense? but that's probably hard to implement."
> user (turn 2): "can we do the color in this spec? i have to imagine the overlapping flakes need a separate spec?"
> user (turn 3): "ok can you do the color quickly before i sleep and $flow-next-capture the directional overlapping flakes"

## Goal & Context

The revised bark in fn-42 looks much better to the owner, but its scales still lack the reference's directional layering. Improve the impression of neighbouring flakes overlapping around the trunk: the reference reads as lower on the left, with the right lying over the previous layer. Colour variation remains in fn-42; this spec addresses the remaining structural appearance. [paraphrase]

## Architecture & Data Models

Express coherent overlap between neighbouring flakes, with tucked and raised edges rather than isolated domed plates. The implementation remains to be established; the reference's image-space left/right observation is visual guidance, not a requirement to anchor bark direction to the camera. [inferred]

## Edge Cases & Constraints

- Keep the small scales, fine separations and chipped edges that improved fn-42; overlap should not restore the rejected broad deep valleys. [inferred]
- Preserve continuous appearance as the camera moves and detail becomes unresolved, without visible stepping or shimmer. [strategy:Surface and rendering at scale]
- Use shared material behaviour that generalizes across trees rather than species-specific rendering paths. [strategy:Surface and rendering at scale]

## Acceptance Criteria

- **R1:** Reference-guided close-up comparisons show neighbouring bark flakes layering predominantly in a coherent direction around the trunk, reflecting the owner's lower-left/raised-right observation. Failure: the result still reads as independently raised isolated plates. No error surface beyond visual comparison. [paraphrase]
- **R2:** Show the owner matched before/after views of the affected bark and the reference, including enough trunk curvature to judge direction, and record whether the overlap addresses the observation. Failure: a rejecting visual verdict leaves this criterion unmet. [inferred]
- **R3:** Camera motion and changing viewing distance preserve continuous bark appearance without visible stepping or shimmer; report measured rendering cost for the added fidelity. Failure: discontinuities or unreported cost leave the criterion unmet. [strategy:Surface and rendering at scale]

## Boundaries

- The immediate red/brown colour variation belongs to fn-42, separate from this overlap change. [paraphrase]
- Start from fn-42's improved shape and colour rather than revisiting the rejected deeper-valley candidate. [inferred]
- No tree-growth, species-pipeline or foliage changes are needed for this bark appearance objective. [inferred]

## Decision Context

The owner explicitly separated a quick colour improvement from directional overlapping flakes. Capturing the latter preserves the structural observation without extending the current colour pass into another relief redesign. [paraphrase]

## Strategy Alignment

Directional bark layering supports the Surface and rendering at scale track: convincing continuous surfaces, judged visually and through measured runtime cost. [strategy:Surface and rendering at scale]

## Strategy Conflicts

No conflict identified with the active strategy. [inferred]
