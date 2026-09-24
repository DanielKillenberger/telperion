# fn-144 result

## Route for the non-round section: a section on the sweep's own rings

Measured on the date palm at seed 1 (`route_cost.rs`, copy into
`crates/telperion-core/examples/` to rerun, release profile, best of 30 wood builds):

| bases | wood triangles | wood ms, round pegs (before) | wood ms, lattice (after) |
|---|---|---|---|
| 0 | 2,000 | 0.073 | 0.074 |
| 32 | 5,800 | 0.147 | 0.179 |
| 100 | 13,960 | 0.326 | 0.425 |
| 256 | 32,680 | 0.825 | 1.028 |

The section rides the swept run: no new ring, vertex or triangle, since a base
already sweeps 3 rings of 20 segments. The extra cost is the cell's vertex
evaluation, 0.1 ms at the palm's 100 bases, out of a 3.5 ms mesh build. A
separate mesh piece per base (a 4-sided prism: 8 vertices, 12 triangles) would
have drawn about 3,070 triangles at 256 bases instead of about 30,700. That
saves no measurable build time at this scale. It would also need a second
geometry path through the run table's radius order, the prepared GPU wood
path, the compact ring words, the attachment surface and the extent
prediction. The prism figures are counted, not built. With the section on
the rings, all of those follow from one table on `Tree::sections`. The
compact ring words describe only a round section, so a shaped surface is never
`qualified()` and falls back to the CPU sweep.

## R1 (straight trunk, width 1, flatness 1, weathering 0; `tests/leaf_base_lattice.rs`)

| bases | rings read | widest gap | deepest crowding | thinnest cover |
|---|---|---|---|---|
| 32 | 44 | 1.9e-15 | 1.7e-6 | 1.00000 |
| 96 | 158 | 4.1e-15 | 1.9e-6 | 1.00000 |
| 256 | 458 | 6.8e-15 | 3.1e-6 | 1.00000 |

Gap and crowding are fractions of the cell's size (sqrt of its area); the
tolerance is 0.05. A negative control fails at width 0.9 on gaps and cover,
and at width 1.1 on crowding. The test was red before the implementation: the
round pegs covered 0.003 of their cell.

## R3: palm values, chosen by code (`lattice_sweep.rs`, output `raw/sweep.txt`)

The shipped palm at seed 1 (crooked trunk, weathering 0.6) was swept at
width 0.8 to 1.1 and flatness 0, 0.5 and 1.

- **Flatness 1:** it had the least bark crowding at every width.
- **Width 1.0:** the only width whose thinnest cover is 1.000.
- **Measured at width 1.0, flatness 1.0:** bark cover 1.000, widest gap 0.119,
  deepest crowding 0.245. The residue comes from the palm's crookedness 10:
  the stem polyline turns 10 to 35 degrees between nodes. The lattice is laid
  on the polyline smoothed over a trunk's width either way, and that smoothing
  cuts the crowding from 0.81 to 0.245. Crookedness is tuning's.

Set in the preset: `leaf_base_width = 1.0` and `leaf_base_flatness = 1.0`. No
other palm value changed.

Stills, rendered once each through `target/release/examples/headless`, with the
palm run's P-TRUNK and P-BASE shots and its 768x1024 matched-view scene:

- `.flow/evidence/fn-144-retained-leaf-bases-pack-into-a-lattice/raw/P-TRUNK.png`
  sha256 975fa3ea85cc9bcb39b718446db2826f2a4498decf439ebccd0016ec2b01be7d
- `.flow/evidence/fn-144-retained-leaf-bases-pack-into-a-lattice/raw/P-BASE.png`
  sha256 52a80e00f511dab8b28f3bc9b64fa21927fc55e64392fdf61ab2c866c76b04da

P-TRUNK shows the diamond lattice of boots over the whole trunk. P-BASE is a
dark close-up of a few large faceted boots.

- **Opus reviewer:** the `trunk-leaf-base-diamond-pattern` cell is pending.
  The host records it.

## Gate

`cargo test --profile ci --workspace --no-fail-fast`, second run: 145 test binaries, 1,060 passed, 0 failed, 21 ignored. The first run failed 35 tests; see FRICTION.md. The `tests/species.rs` memory ceilings passed in the same run.
