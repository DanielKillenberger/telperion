---
title: Trees generate in realtime with screen-error LOD; never baked offline
date: "2026-10-05"
track: knowledge
category: decisions
module: "telperion-space,telperion-render"
tags: [realtime, lod, performance, owner]
applies_when: Trees generate in realtime with screen-error LOD; never baked offline
decision_status: accepted
related_to: [knowledge/decisions/judge-a-generator-change-by-its-2026-10-02, knowledge/decisions/the-far-draw-is-the-near-draw-minus-2026-09-18]
---

Trees are generated at runtime, never baked offline (owner, 2026-10-05: "Don't ask me again to do baking offline. It's against what we're trying to achieve here").

The target is realtime generation: far trees at a coarse level of detail, nearby trees at full fidelity streamed in, and a fast zoom that always shows the right detail ("If i stare at a twig from 5cm away it should appear in high detail ... this should be intelligent not have hardcoded numbers ... we should be able to zoom in fast and show the right detail. It should be that performant").

**How to apply:** never propose offline baking or precomputed tree assets as a performance answer. Level of detail follows one screen-space error budget (half a pixel), from what the generator resolves (lazy, axis by axis, closed-form stand-ins) to how the GPU surfaces wood and picks leaves; no hardcoded side, ring or depth counts. Specs: fn-209 (F1), fn-210 (F2), fn-208 (F3), fn-198 (F4).
