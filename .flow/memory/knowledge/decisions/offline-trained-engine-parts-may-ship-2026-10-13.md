---
title: Offline-trained engine parts may ship; prebaked 3D trees never
date: "2026-10-13"
track: knowledge
category: decisions
tags: [owner, baking, neural, strategy]
applies_when: "proposing precomputation, caching or learned models"
decision_status: accepted
related_to: [knowledge/decisions/judge-a-generator-change-by-its-2026-10-02, knowledge/decisions/the-four-missing-species-with-creative-2026-10-07, knowledge/decisions/trees-generate-in-realtime-with-screen-2026-10-05]
---

The owner (2026-10-13): "i have no problem baking things offline that ship as part of the engine that allows trees be actually generated quickly. I just don't want prebaked 3d models. Having a compressed version of the generator function in a neural net seems fine if that works." This refines the earlier "Don't ask me again to do baking offline" (2026-10).

**Why:** the product promise is trees generated from parameters at runtime, never stored assets; offline work that makes the generator itself faster is engine, not content.

**How to apply:** allowed: offline-trained or offline-computed engine components shipped with a version (for example a neural network that compresses the generator function over its whole parameter space, or lookup tables of the generator's own maths), provided the simulator stays the reference they are checked against. Never allowed: shipping prebaked 3D tree models, meshes or per-tree assets. A model is generalized over the generator's parameter space, not trained per species (owner, 2026-10-13).
