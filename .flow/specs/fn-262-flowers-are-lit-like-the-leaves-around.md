# Flowers are lit like the leaves around them

## Goal & Context

The owner, looking at Laurelin and Telperion in the harness after fn-253 (2026-10-11): "one problem with the flowers is they're colored to uniformly. Don't look natural. They need some sort of lighting. But we can capture that for a spec after release". Unknown, to be measured first: which lighting terms the flower organ shader applies today (sun, sky, shadow, translucency, self-occlusion within an inflorescence) against the leaf shader's, and how much per-floret colour variation exists.

## Requirements

- **R1 (measure first):** the flower shader's lighting compared term by term with the leaves', checked against the code; the colour spread of florets in a mid still against the leaves'.
- **R2:** florets take the same lighting as the leaves where it applies (sun and shadow, sky, back-lighting through thin petals, darkening inside a cluster), by degree from existing rows where they exist; any per-floret colour variation comes from a botanical source or an owner verdict, no fitted factor.
- **R3:** close, mid and whole stills of both Two Trees, viewed, beside fn-253's; frame time held to fn-253's; the owner's verdict.

## Boundaries

Flower shading only. After the release (owner, 2026-10-11).
