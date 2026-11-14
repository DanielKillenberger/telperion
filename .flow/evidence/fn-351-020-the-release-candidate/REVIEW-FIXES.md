# Review fixes on the release candidate

14 Nov 2026, branch `fn-351-review-fixes` off `fn-351-candidate` (a7ae69477).
This note covers two defects that a codex review of the candidate found.

## `Curve::rings` with a cap below three

`Rounds::new` built entries `0..=cap`, but `build::sides` never returns fewer
than three sides, so a cap of 0, 1 or 2 indexed past the cache. The cache now
runs through `cap.max(3)`. No public parameter reaches such a cap: the
`radialSegments` rail is 3 to 64 and is validated. Only the `segments`
argument of `Curve::rings` can pass one.

## A supernatural leaf size past the species' leaf-area edge

The host's decision: the engine's contract (fn-264, the `leaf_area` edge
of 5 m²) and the engine's interface stay as they are. A `canopy.size`
whose drawn leaf (the natural `leaf_area` times (size / natural size)²)
passes the edge is refused at `supernatural.canopy.size`. The message gives
the drawn area, the edge (read from `telperion_space::setting("leaf_area")`)
and the largest size the family admits, about 38.1 for the oak. The refusal
comes from `Family::validate` and from `GrowInput::of`, so `mesh::build` gives
the same error.

### What the grower does with the drawn leaf

I measured this with the engine's leaf-area edge bypassed in a probe that is
not committed. The probe is the Oregon white oak with the group on. Every grow
took 0.02 s or less. Every value stayed finite, and nothing panicked.

| age | size | nodes | height | widest base radius | leaves placed |
|---|---|---|---|---|---|
| 2 | 1 | 37 | 0.72 m | 0.0095 m | 0 |
| 2 | 100 | 17 | 0.72 m | 0.18 m | 0 |
| 2 | 1000 | refused, `light.deposit` | | | |
| 10 | 1 | 10,940 | 3.69 m | 0.040 m | 1,314 |
| 10 | 10 | 1,118 | 3.70 m | 0.12 m | 24 |
| 10 | 38 | 136 | 3.69 m | 0.12 m | 0 |
| 10 | 100 | 83 | 3.69 m | 0.21 m | 0 |
| 10 | 400 | 83 | 3.69 m | 0.85 m | 0 |
| 10 | 600, 800, 1000 | refused, `light.deposit` | | | |

### Open

- The crown loses its leaves as the drawn leaf enlarges. The oak at age 10
  places 1,314 leaves at size 1, 24 at size 10 and none from size 38 on.
  Size 38 is still inside the edge.
- From about size 600, the engine's light lattice refuses the tree:
  `light.deposit: a lattice node holds at most 2^32 quanta of 2^-20 m² of
  leaf area`. The new refusal at about size 38.1 now stops these sizes first.
  The size override's own stated range is still 0 to 1000.
