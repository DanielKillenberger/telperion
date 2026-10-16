# Crown outline irregularity

## Goal

A fn-34 round-3 gap (`.flow/evidence/fn34/GAPS.md`): every crown had the same outline on every seed, an exact axisymmetric superellipse, so the beech and birch read as ovals against photographs with lobes where scaffold limbs end and hollows where they do not. This spec gave the envelope a per-seed low-frequency radius perturbation by height and bearing, amplitude and wavelength as rows, neutral at zero; growth, retention and the surface follow the perturbed shell.

## Decisions

- > user (2026-09-14, after fn-34 round 3): "so i would like these species to inspire the generator to improve. I guess we can try and get as close as possible with values and then make a gap analysis what the generator needs and then spec that and once implemented do another comparison."
- The perturbed shell drives containment, the scaffold and the twig planner; the smooth two-dimensional profile still drives shedding and the crown index, stated in the spec.
- The compare script gains an outline statistic (radial deviation from the fitted ellipse) for still and photograph.

## Acceptance Criteria

- **R1:** `skeleton.envelope.irregularity` (0 to 0.5, neutral 0) and `lobe_scale` (0.05 to 1) railed, blended, in metadata and on harness sliders; presets at zero byte-identical — met.
- **R2:** Seeded height-and-bearing noise; growth and twig planner reject against the perturbed shell; containment tests hold — met.
- **R3:** Compare script records outline radial deviation for still and photograph — met.
- **R4:** Beech and birch set an amplitude; pairs re-rendered (round 5); owner verdict in fn-34 — met (birch accepted in fn-34; beech moved to fn-62).
- **R5:** Tests for neutral identity, rails, perturbed radius bound, containment per fixed seed, determinism, blend walk, outline statistic — met.

## Result

The crown shell takes an irregular outline by amplitude and wavelength rows; neutral is the smooth shell to the byte. The owner accepted the silver birch in fn-34; the European beech's verdict moved to fn-62.

## Open

None.
