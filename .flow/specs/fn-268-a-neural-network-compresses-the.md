# A neural network compresses the generator function

## Goal & Context

The owner (2026-10-13): "could we maybe train some neural net that would be able to generate trees from the learned set given params?"; "i think the model should be generalized against our generator function not per species"; "i have no problem baking things offline that ship as part of the engine that allows trees be actually generated quickly. I just don't want prebaked 3d models. Having a compressed version of the generator function in a neural net seems fine if that works." The simulator (fn-255/256's one cycle) stays the reference; a network trained offline from it may ship with a version as an engine component if it reproduces the generator closely enough, and no tree asset is ever stored. This spec is the first, bounded experiment: does a learned function over the whole parameter space reproduce the site's field.

## Requirements

- **R1 (data from the generator):** trees grown by the device cycle across the parameter space: the eight shipped presets, walks between them, and points sampled within each row's valid range, at several ages and seeds; for each, the site's field at 36^3 over its bounds (wood occupancy and maximum radius, foliage density, limb ownership). Held out entirely: the silver birch and one random region of the space. Data generation follows the GPU and memory rules (one heavy run at a time, waits while the owner games).
- **R2 (the model):** one network f(parameters, age, seed, position) over the whole space, not per species; trained on the owner's RTX 3080; size and query cost stated.
- **R3 (measured):** on held-out trees, the field's error against the exact field (per-cell occupancy Jaccard, radius and density errors, limb agreement), stills of the voxel grove beside the exact one, viewed; query time for a whole 36^3 field; the model's size. Stop: held-out trees visibly wrong at the site's view, in which case the space is too large to learn at this scale and the report says what scale would be needed.
- **R4:** the owner's verdict on the stills and numbers decides whether a shipped-model spec follows.

## Boundaries

An experiment: no production change. Environment: PyTorch with CUDA in its own virtual environment outside the repository, installed only with the owner's yes. After fn-256 task 16.
