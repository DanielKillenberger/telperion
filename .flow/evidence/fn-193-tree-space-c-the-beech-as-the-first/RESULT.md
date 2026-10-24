# fn-193 result: the beech as the first point

Condensed record; the round-by-round history (rounds 1 to 22, host and Astra gates) is archived (see `.flow/evidence/ARCHIVE-MANIFEST.md`). Decisions and the owner's verdict are in the spec.

## What the engine gained (geometry and the rendering path)

- **`Form`, per PA, each continuous and neutral where it changes nothing:**
  - `tropism` and `elevation`: the axis bends toward an elevation.
  - `wander`: each node turns the axis by a keyed draw.
  - `plane`: the turn of the plane a parent's laterals branch in.
  - `pipe`: each phytomer's section.
- **Laterals leave from their parent's direction at their node**, carried along the bending axis. fn-192's decision was "from the parent's heading". That was the same where axes were straight, and is not on a bent limb. This is a decision for the host.
- **Girth** comes from the pipe model (`girth.rs`); presence scales each phytomer's section.
- **The rendering path** is `examples/space/tree.rs`, which turns the structure into a pipeline `Tree` and hands it to `executor::expand(..).mesh()`. Leaves, bark and the leaf element are the european-beech preset's rows. Rules in the conversion:
  - Wood under 0.2 mm radius is not drawn.
  - Internodes under 1 mm merge into their node.
  - A lateral of 0.4 to 0.7 of its parent's radius grows in as a codominant fork, so the fork has no collar.
- **Tests:**
  - `tests/form.rs`: tropism, wander, plane, local frames, pipe model, and the beech growing at every sheet age.
  - Refusals for the five new settings.
  - The walk test walks the four geometric `Form` settings on every PA. Rates per metre are walked in radians over the metres the axis grows.
  - The whole crate is green.

## R4 for the final beech (round 22) against the pinned baseline

Measured on 2026-10-04 on the same desk, release profile. Peak RSS is the whole `space_beech` process at 80 cycles: growth, conversion, dressing and the GPU renderer with four stills. The baseline's RSS is mesh builds only, so RSS is not like for like.

| Seed | Kept nodes (baseline) | Growth ms (baseline skeleton ms) | Dress ms (baseline full build ms) | Fine wood km (baseline) | Peak RSS MiB (baseline) | dbh |
|---|--:|--:|--:|--:|--:|--:|
| 1 | 1,936,193 (213,044) | 3,687 (57.3) | 1,835 (3,471) | 23.75 (46.66) | 6,627 (727) | 1.09 m |
| 7 | 1,619,028 (215,470) | 2,816 (57.8) | 1,645 (3,474) | 21.29 (47.15) | 5,251 (733) | 0.68 m |

- Growth is 49 to 64 times today's skeleton; dressing is about half of today's full build. Fine wood is about half of today's. Phase F owns speed and memory.
- dbh now follows the trunk through its relay modules (`examples/beech.rs`). It measured 0.00 m at seed 7 before, because only the first trunk module was searched.
