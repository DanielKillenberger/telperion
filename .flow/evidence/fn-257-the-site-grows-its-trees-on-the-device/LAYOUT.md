# fn-257 R7: the drawn records, and where a tree's memory is (worker, 2026-10-16)

R7 asks for the drawn node records (152 B a phytomer, 48 B an axis) in a compact layout, for memory. Before writing it, where the records sit in a build's memory was measured (`growth_profile`, warm, RTX 3080; `raw/r5/<case>.md`, each stage's peak and its largest buffers).

## Today's records

A drawn phytomer (`dress.wgsl`, "A drawn phytomer"; `resident/fold.rs` `CV` = 38 words):

| Words | Field |
|---|---|
| 0-2 | position (y up) |
| 3, 4, 5 | radius, start radius, base radius |
| 6 | flags (taken, made, fine, seam, clustered, tip, stem) |
| 7, 8, 9 | the node it is drawn as, its node's run, its laid axis |
| 10-12 | its parent's position |
| 13 | codominance |
| 14, 15, 16 | id, distance along the wood, parent's id |
| 17-22 | its wood's frame as laid: heading, side |
| 23, 24, 25 | branch, key, wood as grown |
| 26-27, 28, 29 | its phytomer's key (two words), flowering share, cluster spread |
| 30-31, 33 | leaf retention: metres before the boundary, retentions before and from it |
| 32, 34-37 | its leaves within the year (fn-270 R5c): grow, fall, colour, regrow, life |

Readers: `dress.wgsl` (the previous wave's), `fold.wgsl`, `boots.wgsl`, `sweep.wgsl` (`gather`, which copies each into the swept node of `NW` = 39 words), `stems.wgsl`, `drawn.rs`, `agree_tests.rs` and `growth_profile_tests.rs`. The swept nodes are read by `sweep.wgsl`, `stations.wgsl`, `rings.wgsl`, `handoff.wgsl` and the field (`field_build.wgsl`).

A compact layout could take a phytomer from 38 words to about 16: the parent's position is the parent's own (3 words), the frame two octahedral words (6 to 2), the radii three halves (3 to 2), the flags, run and axis one word with the axis in the high bits, the leaf-year words halves (6 to 3), the retention words halves.

## Where the memory is (m)

| Tree | growth's peak | of it, the drawn records | the dress's peak | the field kept (`field_tests`) |
|---|--:|--:|--:|--:|
| spruce 40 s1 | 1,572 MB (pencils 275, axes 227, counts 211, phytomers laid 145) | none: they are made after growth's tables are let go | 665 MB (drawn 222, settle work 151, settle nodes 145, settle place 125) | 109 MB |
| Telperion 82 s1 | 2,740 MB (pencils 396, ...) | none | | 108 MB |
| birch 40 s1 | 573 MB | none | | 17 MB |

The drawn records live only after growth: a build's peak is growth's tables, which the records never sit beside. Compacting them from 38 words to about 16 lowers the dress's 665 MB to about 540 MB on the spruce, and a build's peak by nothing. What a tree keeps once built is the harness's mesh buffers, or, for the site, the field (`DeviceField::bytes`).

## The open choice (for the host)

R7 as written changes ten shaders and every reader of the drawn tree (the trees the owner just approved) and saves no peak memory. Two targets that would save memory a consumer holds:

1. **The field's kept primitives** (what a site page holds a tree by): two vec4 a primitive today (32 B), about 16 B with positions quantized within the field's box and the radii as halves; the spruce's field 109 MB to about 60 MB. Within `field_build.wgsl` and `field_query.wgsl` alone.
2. **Growth's tables** (every build's peak): fn-270 R4's levers, the pencils and axes tables, outside this spec.

R7 is held until the host chooses: the drawn records as written, the field's primitives, or growth's tables.
