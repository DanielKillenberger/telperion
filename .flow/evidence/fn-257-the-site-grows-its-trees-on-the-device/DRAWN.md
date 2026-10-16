# fn-257 task 10: what the drawing keeps, measured (worker, 2026-10-16)

`resident::drawn_memory_tests::the_drawing_s_memory`, RTX 3080, each shipped preset at its default age, seed 1, a harness build (`Generator::prepare`, `Delivery::Resident`):

- **The build's peak:** its largest buffers.
- **Kept:** what the drawn tree keeps once submitted.
- **A still:** what a 1,280 × 960 still at the hero pose adds (`Renderer::curve_bytes` after it).
- **The dress's peak:** from `growth_profile` (`raw/r5`).

| Preset | Build's peak | Dress's peak (drawn records) | Kept by the drawn tree | A still adds (the curve's view budgets) |
|---|--:|--:|---|--:|
| ordinary | 180.1 MB | | 3.8 MB (curve points 3.0) | 344.9 MB |
| oak | 872.0 | | 13.6 (curve points 9.7) | 377.9 |
| spruce | 1,586.8 | 684.2 (226.2) | 181.0 (unbudgeted 80.8, foliage 48.3, curve points 38.3, clusters 13.2) | 717.8 |
| beech | 201.0 | | 5.7 | 346.4 |
| birch | 576.9 | | 15.0 | 414.5 |
| date palm | 33.9 | | 0.4 | 322.3 |
| Telperion | 2,690.0 | | 108.6 (curve points 68.2) | 748.4 |
| Laurelin | 2,469.6 | | 134.3 (curve points 63.9) | 748.1 |

Where each candidate sits:

- **The drawn records** (152 B a phytomer, 226 MB on the spruce) live from the dress to the sweep, inside the dress's peak (684 MB). Growth's tables (1,587 MB) are let go before them, so the build's peak is never theirs. Compacting them to about 64 B a phytomer lowers the dress's interim peak by about 130 MB on the spruce. It lowers the build's peak and what the tree keeps by nothing.
- **The wood's normals and bark coordinates** are the surfaced vertices (`curve.rs`'s `VERTEX_FLOATS` = 9: position, normal, bark along and angle, radius), written per frame into each view's budget. The budget (`curve/target.rs`) is sized from the view's pixels (12 vertices a pixel) and capped by two 128 MiB bindings. At this still the vertices, rings and indices take 322-827 MB, the largest thing the drawing holds. A 24 B vertex (normal octahedral, bark and radius halves) leaves the capped buffer's bytes as they are, 268 MB at 1,280 × 960, and fits 1.5 times the vertices in it. It saves a third of the vertex bytes only on views under the cap. So it is a gain in detail at hero views, not memory, unless the budget is restated in vertices.
- **The curve points** are already 32 B, compact by host decision 8 (`telperion-core` `pack.rs`).

## No step taken: three decisions for the host

1. **The drawn records.** Should they be compacted for the dress's interim peak alone? That is 130 MB on the spruce, with ten shaders and every reader of the approved trees changed, and nothing a consumer holds is lowered.
2. **The vertex budget.** Should a compact vertex pay off as memory (the budget restated in vertices, host decisions 19 and 21) or as detail (the same bytes, more vertices)? Either way the surfaced wood's normals change, and the stills would need viewing.
3. **The 80.8 MB unattributed.** The spruce's kept tree charges 80.8 MB as unattributed (`Owned::new`). It should be named before it is weighed.
