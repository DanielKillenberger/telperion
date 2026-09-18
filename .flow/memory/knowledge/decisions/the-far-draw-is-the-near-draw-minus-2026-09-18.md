---
title: The far draw is the near draw minus what the eye cannot resolve
date: "2026-09-18"
track: knowledge
category: decisions
module: crates/telperion-render/src/shaders
tags: [lod, filtering, bark, relief, fn-71]
applies_when: "Any renderer term that fades, filters or simplifies with distance or footprint: relief, tints, grain, foliage levels; any resolution test or bound that stands in for the owner's eye."
decision_status: accepted
alternatives_considered: [keep the 2x and 4x mean-difference bounds as the finish line; fade terms an octave early to avoid shimmer]
---

The owner's rule for every level-of-detail, filtering and fade term the renderer carries: the far draw is the near draw minus what the eye cannot resolve at that distance, and nothing more. Detail leaves at the scale the pixel stops resolving it, never earlier, so a walk from the base to the hero pose shows one material throughout with no step in fidelity.

Stated on 2026-09-18 while judging fn-71 in the harness, after fn-55 and fn-71 had both passed their 2x and 4x resolution bounds: "it still looks much less detailed from a distance. There's a noticeable change in fidelity which shouldn't be the case. Fidelity should go down to a degree where it's not noticeable at that distance."

Why the bounds passed and the eye did not: the relief, tint and grain terms converge by amplitude fades that begin an octave before a feature reaches a pixel, which keeps the 2x2 box comparison small while removing detail the eye still sees. A mean-difference bound over a whole frame cannot catch that; a continuous distance sweep judged in the harness can.

How to apply: a term that must survive resolution is prefiltered exactly (a box or analytic integral over the footprint), not faded early; the fade scale of any remaining amplitude fade sits at the pixel, never an octave above it; a spec that changes shading at distance carries a sweep of stills across a walk, not two fixed multiples, and the owner's eye on the mature path is the gate.
