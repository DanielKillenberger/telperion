# FN23 probe: is the oak frame primitives or pixels?

Every foliage instance drawn as a two-triangle quad spanning the element's blade
extent, against the same frame with the element whole. Native headless, RTX 3080
on vulkan, 1600x1000, seed 7, whole view, hero pose, fn-22's session protocol.

| Run | Triangles drawn | p50 | p95 | Verdict |
|---|---:|---:|---:|---|
| Element whole ([record](probe-full-timing.json)) | 154,075,392 | 18.08 ms | 18.22 ms | valid |
| Two-triangle stand-in ([record](probe-quad-timing.json)) | 6,391,128 | 1.58 ms | 1.61 ms | valid |

Both runs draw the same 555,204 instances in the same three calls and cover the
same pixels; only the triangles per leaf change, 268 down to 2.

**Conclusion: vertex-bound.** The stand-in holds the same coverage for 1.58 ms,
under the 2 ms hero target, so the 18 ms is spent on primitives and not on fill.
Fewer triangles per leaf move the frame, which is what the spec's levels are for.
fn-22's report reads the oak as raster-bound on large leaves; that reading is
wrong, and the same vertex-bound explanation it gave the spruce covers the oak.

The stand-in is a measurement hook and not a level: it ignores the blade outline
and would thin the crown visibly. Task 2 replaces it with the deviation-chosen
sections that keep the silhouette.
