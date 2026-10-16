# fn-281 R1: the unsolved-radii sweep

`sweep.sh` ran `examples/radii_sweep.rs` at seeds 1 to 50 over 20 shards. Each tree was grown through `executor::grow` and `expansion`, then its tree was held to `validate_solved`. Each shard was a capped `telperion-heavy` job; the highest peak was 5.8 GB (the Norway spruce at 80). Per-tree lines are in `raw/sweep/` (ignored), and the shard totals are in `raw/sweep-summary.log`. The oak was sampled at 60, 70, 80, 90 and 100, not at every whole age between them.

## Unsolved radii: the oak only

| Age | Seeds refused | Zero-radius nodes |
|---|---|---|
| 60 | 25 | 1 |
| 70 | 25, 34 | 3, 1 |
| 80 | 3, 7, 30, 39 | 2, 1, 1, 1 |
| 90 | none among the 11 that grew | - |
| 100 | none among the 4 that grew | - |

Every one of the ten nodes is a lateral head (`branch` is itself) at radius, start and base 0, a twig with no codominant weight: the signature traced in the fn-281 spec.

These presets have no refusal at their own age or at 80: the oak at its own age (40), the ordinary tree, the Norway spruce, the European beech, the silver birch, the date palm, Telperion (82) and Laurelin (80). That is 750 trees.

## The other refusal: the engine's phytomer budget

The oak also refuses with `resource limit: engine phytomer budget`, the conversion's `BUDGET` of 20,000,000:

- **At 80:** seeds 14, 25, 34, 43, 46 and 48, 6 of 50.
- **At 90:** 39 of 50.
- **At 100:** 46 of 50.

At 90 and 100 this hides whether the refused seeds also have an unsolved radius. This refusal is not the radius solve's; it is named here because the spec's title is that the oak at 80 builds at every seed.
