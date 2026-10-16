# Tree space: girth from secondary growth by share

## Goal

The engine gives every axis pipe-model girth (`girth.rs`). A palm has no secondary thickening: its stem keeps the width it was established with, a column, not a taper. On fn-196's palm the pipe model gave 0.099 m at the base against 0.020 m at the apex, a spike (MODEL-PALM.md Q2). Palms and dicots differ by degree in this trait, so it is a setting, not a switch (host, 2026-10-05). Girth only; a palm's base swell is fn-196's values question.

## Decisions

- Design (host, 2026-10-05): `form.secondary` per physiological age, 1 neutral; radius = `secondary * pipe_radius + (1 - secondary) * established_radius`.
- Built reading: the established radius is the phytomer's own pipe when its apex laid it down (`pipe * scale * ripe(1 year)`), since the end-of-first-cycle reading would give a yearly sawtooth (12^(1/4) = 1.86 base over top). The load handed on stays the pipe model's at every share.

## Acceptance Criteria

- **R1:** The setting, neutral at 1, every existing tree byte-identical — met (beech, spruce, palm structures at 10/40/80, seeds 1 and 7, identical; oracle and walks pass).
- **R2:** A single axis at 0 is a column (base/apex near 1) and at 1 matches today — met (`a_stem_without_secondary_growth_is_a_column`, red first).
- **R3:** Walk test on `secondary` within bound 30 — met (9.53 on sagging limbs; girth moves shape only through sag).
- **R4:** Workspace gate; Codex review — met (1,044 passed; Codex SHIP, no findings).

## Result

As above; the setting is continuous by construction.

## Open

None.
