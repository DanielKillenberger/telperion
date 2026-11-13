# fn-362 R6: the pins

## The committed species digests: not runnable, not repinned

`crates/telperion-core/src/suite/species/digests.json` digests each species' fixed seeds over the tree, the CPU wood and the placed leaves together. Its only reader is `suite::species::grow_and_check`, called from the four `fixed_*_pass_geometry_and_profile_gates_with_repeatable_varied_specimens` tests, all `#[ignore]` since fn-198 ("the scaffold's grower on an engine preset's rows, which no build grows now; R0 removes both"). They cannot be run on request either: run with `--ignored`, the oak's test re-executes itself in a child process (`species::budget::alone`) without `--ignored`, so the child selects nothing and the parent fails with "selected nothing" (`raw/species-oak-ignored.log`, 13 Nov 2026). The live tests of that suite pass on the head (9 passed, 5 ignored, `raw/species-1.log`). So no committed digest of the CPU wood is checked by any test that runs, and none was repinned; repinning a test that cannot run would write numbers nothing reads. Whether to revive or delete those pins is the host's call; it predates this task.

## Pins upstream of the surface stage

Measured base against head with the scratch probe (MEASURED.md): the digests of the grown tree, the curve and the leaves are equal on all six trees measured, except the Norway spruce's leaves, whose needles sit on the wood's faces (`surfaceContact` 1), so they move with the polygons:

| Tree | Tree digest | Curve digest | Leaves digest |
|---|---|---|---|
| European beech 80, seed 1 | 32c3781631af4030, equal | e188241e4209c26a, equal | 97620bfa00858ba0, equal |
| European beech 40, seed 1 | c8fd371ee3dd0c5a, equal | ea3fcbed7abf0909, equal | 8ec430509ef63eb6, equal |
| Oregon white oak 120, seed 1001 | equal | equal | equal |
| Norway spruce 40, seed 1 | 2caa72b377c06942, equal | b4f35f4dbc40a18b, equal | 1cbbacca7ae4ff02 → e3df37eee36b55d8 (3,484,945 leaves both) |
| Telperion 82, seed 1 | equal | equal | equal |
| Laurelin 80, seed 1 | equal | equal | equal |

The wood's digest moves on all six, as the change intends.

## Tests whose expected values moved

- `suite::surface::root_only_and_float32_limits`: the far, 0.1 nm tree drops 10 triangles (its two caps' six, at three sides, and four strip slivers; measured) against 40.
- `suite::surface_attachments::needle_origins_touch_rendered_facets_across_bends_and_sockets`: the plane bound is now `Σ h_k |n_k|` (MEASURED.md, decisions).
- `pipeline::surface::curve::tests::*` and `tests/ground_reach.rs`: walk each run at its own sides (`CurveRun::sides`); what they check is unchanged.
