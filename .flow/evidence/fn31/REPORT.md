# FN31: growth rule and botanical form

The renderer prerequisite is repaired through growth and fixture-state fixes.
The final task report will add the age strips, calibration, convergence and
gates after the sapling rule is complete. Round 1's superseded diagnosis is
retained in [ROUND1.md](ROUND1.md).

## Protocol

Native release builds at seed 7. Test cameras, assertions, tolerances, pins
and shaders remain unchanged. All five pre-change growth/envelope measurements
are retained in `prechange/` before any identity, audit or look pin moves.

The inherited references and their composition remain those of fn-30.
`logs/checksums.log` verifies all five original local source files and the
unchanged `../fn30/curves.py`. No new reference was fetched.

## Renderer prerequisite

A paused structural axis owns its terminal bud until it finishes. Local
growth previously flushed that bud too early, and the retained width diverted
the surface's thickest path out of the oak bole at 2.395 m. The new regression
fails on that height before the fix and passes afterwards.

Retained axes plan their lengths against a crown-base blend controlled by
`growth.crownBaseRetention`. The species rows retain the mature base, while
the ordinary tree and Two Trees preserve their expanding-crown planning.
This removes overlong low spruce branches from the fixed grazing view.
Both grazing masks now contain wood (oak 8,637 pixels, spruce 1,975).
The final prerequisite bark-distance mean is 2.153633/255 at 4x against 3.0;
p95 is 8.25/255 against 12.

The bark fixture's cap changes from 400 to 40,000 nodes so the grown trunk
reaches the test's existing 0.16 m relief scale. The conformance fixture moves
shell depth, surface contact, bark roughness and furrow strength to 0.5 before
its unchanged jitter, and uses expanding-crown retention 0 for its compact
parameter sets. These values preserve its whole-tree intent and keep the
jittered inputs away from domain bounds. All assertions and cameras stay exact.

The frozen original Ordinary set 6 is retained as `ordinary-growth.json`.
Its diagnostic produces 1,440 triangles / zero placements with vigour floor 0,
and 14,280 triangles / 576 placements with floor 0.75. The conformance sweep
renders 20 valid sets in 20 attempts.

## Thickening and shedding

The pipe model still splits structural radii by the pinned power sum.
Its common scale now includes an age-dependent radius fraction. Numeric
juvenile-radius, delay and shape rows start slender wood and increase its
radius share toward one at derived maturity. The existing historical maxima
and canonical keyframes retain monotone radii.

Vigour is exposure times the greater of its age-decaying share and the numeric
floor. The floor is 0.75. Ordinary and the Two Trees restore their authored
0.45 shedding thresholds. Before the floor, the regression measured 33
Ordinary, 30 Telperion and 175 Laurelin nodes at maturity. After the prerequisite
changes, it measures 18,030, 122,729 and 151,166 respectively. Synthetic shaded
interior shoots on those same preset fixtures receive death stamp 174;
the exposed siblings survive. Final task measurements will supersede these
intermediate counts after sapling form is implemented.

## Validation evidence

- `logs/bole-red.log` and `logs/form-checkpoint-final.log`.
- `logs/thickening-red.log` and `logs/thickening-green.log`.
- `logs/survival-red.log` and `logs/survival-checkpoint-final.log`.
- `logs/original-conformance-set.log`.
- `logs/renderer-checkpoint-final.log` for the three bark targets.
- `logs/conformance-checkpoint-final.log` for final conformance.
- `logs/sapling-red.log` records all four seedling/sapling regressions before
  their implementation. Their source is retained in `sapling_regressions.rs`.

The complete gates, final numeric tables, timings, captures and docs are
pending the remaining implementation. No pin has moved and no owner verdict
has been supplied.

## Owner verdict

- R1, oak strip and mature comparison:
- R1, spruce strip and mature comparison:
- R2, reference composition and diameter deviations by age:
