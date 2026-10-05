# Tree space: girth from secondary growth by share

## Goal & Context

The engine gives every axis pipe-model girth: the radius below a node grows with the load it carries (`girth.rs`). A palm has no secondary thickening; its stem keeps the width it was established with at the apex, so it is a column, not a taper. On fn-196's palm the pipe model gives 0.099 m at the base against 0.020 m at the apex, a spike (checked by fn-196's worker; MODEL-PALM.md Q2). Palms and dicot trees differ by degree in this one trait, so it is a setting, not a switch (host, 2026-10-05).

## Design (host, 2026-10-05)

- `form.secondary` per physiological age, 1 neutral (today's pipe model, every existing tree byte-identical).
- The radius is `secondary * pipe_radius + (1 - secondary) * established_radius`, where `established_radius` is the radius the phytomer had when it was laid down at the apex (its pipe radius at its first cycle). At 0 a stem keeps its established width for life.
- Continuous by construction, walk-tested.

## Requirements

- **R1:** The setting, neutral at 1 with every existing tree byte-identical (beech and spruce stills, A's oracle, B's walks).
- **R2:** A test that a single axis at `secondary` 0 has a base-to-apex radius ratio near 1 (within the established width's own change), and at 1 matches today.
- **R3:** Walk test on `secondary` within the bound of 30.
- **R4:** Workspace gate; Codex review.

## Boundaries

Girth only. A palm's slight base swell, if the palm needs it, is fn-196's values question.
