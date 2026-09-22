# The field reads the plan, not placed leaves

## Conversation Evidence

> user: "We can just use the generated field and make a voxel tree no? one that fits the black white aesthetic. But for that we'd need the generated field to be lightweight enough to be included in killenberger.com We shouldn't drag in the full rendering thing."
> user: "no we don't want voxels output in telperion-core. I thought we'd have a step that outputs an intermediary step that could inform voxels that happens before generating the full mesh. A field informing where branches/twigs and foliage are before it's actually placed."
> user: "How can the field be slower than the full mesh build?"
> user: "but clearly a minecraft style/voxel consumer or whatever it is should be able to consume information from before the mesh gets built to build its own mesh otherwise it's completely pointless and overly complex."
> user: "the voxels end up too dense there's no gaps anywhere is that fixable?"
> assistant: "Dropping clumps is a styling choice for the homepage, so I would keep it in the consumer. The core field would then only answer where wood is, where foliage is, and which limb owns that foliage."
> user: "i agree with that last bit."
> user: "this pipeline restructure's performance must be faster or at worst equal to current performance."

## Goal & Context

<!-- Goal & Context: 50% [paraphrase], 50% [inferred] measurements -->

A voxel consumer asks the core where a tree's wood and foliage are. Today that answer costs more than the full mesh. A field request first places and culls every leaf, then stores a bounding box per placed leaf and builds a box tree over them. Measured on master `3fdfb440` on 2026-09-21, a field-only build through the Wasm binding took 1.0 s and 96 MB for the Oregon white oak, 3.5 s and 32 MB for the silver birch, and 8.9 s and 668 MB for the Norway spruce. The solved tree alone took 124, 163 and 58 ms.

The information a voxel consumer needs exists before any leaf is placed. The solved tree carries every wood segment with both radii. fn-91 added prepared station segments: for each leaf-bearing wood segment, its endpoints, radii, frame and the count of leaf stations on it, with no per-leaf position built. Native station preparation took 18, 8 and 52 ms for the same three presets; that figure includes building the wood's contact surface, which the field does not need. All six shipped presets support stations.

Station segments do not yet carry everything the field needs. They hold no owning node, no limb id and no leaf reach, they are tiled into groups of at most 256 stations with repeated endpoints, and their preparation depends on placement validation and surface types. Short shoots, limb clumping, families with no twig layer, and a numeric-range fallback have no station form at all; among the presets only the European beech, still under fn-62, is excluded, through its short shoots and limb clumping. An outside review on 2026-09-21 checked these points against the code, and the host re-checked them on 2026-09-22 (`foliage/prepared.rs`, `supports_stations`).

This spec moves the field onto those two inputs. It unblocks the killenberger.com voxel tree, which needs the field to be cheap, and it makes the strategy's sentence about voxel worlds sampling trees "without paying to construct a surface mesh" true of the expensive stage as well.

A throwaway prototype in `experiments/voxel-field/` showed that the solved tree alone gives a 64-cell voxel tree that reads as its species, in 93 to 205 ms per tree. It used raw twig nodes because station segments had not been found yet.

## Architecture & Data Models

The field is built from the solved tree and a foliage descriptor layer. It never calls leaf placement or the cull and holds no per-leaf record. [paraphrase]

The descriptor layer is the part of station preparation the field needs, and it compiles with no wood-surface and no leaf-placement code: shared parameter types move out of the placement and surface modules so the layer depends on neither. It builds no contact surface. Each descriptor is one leaf-bearing wood segment with its endpoints, its station count before any cull, its owning limb system and the family's leaf reach. The renderer's station preparation is built on the same descriptors, so there is one definition of which wood bears leaves. [inferred]

Growth means the direct build's skeleton generation, the path the product draws. [inferred]

Wood occupancy stays as it is: tapered sphere sweeps along solved edges. [inferred]

Foliage occupancy is a sweep along each descriptor with a conservative leaf reach as its radius, tested against the query cube the same way wood is. The reach bounds every point a placed leaf can occupy measured from its bearing segment: the largest displacement off the wood that placement can apply, plus the leaf element's farthest vertex from its attachment at the largest scale placement can draw. [inferred]

A foliage answer carries two values beyond the flag. The leaf-count estimate for a cell is each reaching descriptor's station count multiplied by the share of that descriptor's length lying inside the cell, so that over any grid of non-overlapping cells covering the tree the estimates sum to the descriptor total. Counts are before the crown-shell cull and the answer says so. The owning limb system comes from the limb-system assignment the clumping code already holds as a separate function over solved radii (`foliage/clumping.rs`, `systems`); it is module-private today and the field reaches it through a crate-visible export. A cell with no foliage reports no limb id. [paraphrase]

## Edge Cases & Constraints

- A cell far larger than the leaf reach must still report foliage when a station segment passes through it. The prototype's first run lost the spruce, whose needles reach 1.9 cm against 24 cm cells, until it floored the sweep at half a cell. The field's existing query inflates by the cell's half extent, which covers this; a test pins it. [inferred]
- A cell reached by descriptors of two limb systems reports the system with the larger count estimate in that cell; an exact tie reports the lower id. [inferred]
- A family the descriptor layer cannot describe (short shoots, limb clumping, no twig layer, or a value outside the numeric range) keeps today's field behaviour through leaf placement and reports which path it took. No supported parameter loses its field. [inferred]
- The browser binding's public query and snapshot shapes change; the typed wrapper and its tests change with them. [inferred]
- A preset that bears no leaves yields wood occupancy and no foliage, as today. [inferred]

## Acceptance Criteria

- **R1:** For a family the descriptor layer supports, a field-only build runs no leaf placement, no cull and no contact-surface construction. Stage instrumentation in the binding's metadata shows which stages ran, for every shipped preset. [paraphrase]
- **R2:** On the oak, birch and spruce, at seeds 1 and 7, the median of five field-only builds including a 64-cell batch query is faster than the base commit's, and faster than the same commit's surface-plus-foliage build. Peak Wasm linear memory for the field-only build is lower than the base's. Field storage is recorded for base and candidate; the worker reports the ratio and the wood share, and no ratio is promised before it is measured. [user]
- **R3:** Wood answers are unchanged: the existing field tests' wood assertions pass without edits, and a wood-only comparison of batch query results against the base is byte-identical on the 8-tree forest. [inferred]
- **R4:** Foliage coverage is conservative against real blades: for the oak, birch and spruce, at cells of 0.1 m, 0.25 m and 1 m, every vertex of every leaf the current CPU placement retains lies inside a cell the new field reports as foliage. The spruce's needles and a cube that only touches a sweep's boundary are covered by named tests. [inferred]
- **R5:** Over-coverage is bounded: at a 0.25 m cell the new field's foliage cell count is at most twice the count the base field reports for the same tree, for each of the three presets. If measurement shows the bound cannot hold with a conservative reach, the worker stops with `NEEDS_HUMAN` and the measured ratios. [inferred]
- **R6:** Over a non-overlapping grid covering the tree, the leaf-count estimates sum to within 1% of the descriptor total for each of the three presets, and a two-limb fixture reports different limb ids for the two limbs' foliage. [paraphrase]
- **R7:** The descriptor layer builds with the wood-surface and leaf-placement code compiled out, shown by a feature build in the test matrix. [inferred]
- **R9 (owner, 2026-09-22):** A batch answer carries, per cell, the largest wood radius of any wood sweep reaching the cell, in metres, zero where no wood reaches; the core's `Occupancy` gains the value, the binding exposes it in a slot beside the flags and the typed wrapper types it. A consumer can then cull twigs by thickness and draw limbs through foliage, which the base prototype did from the structure export and the field could not. [user]
- **R10 (owner, 2026-09-22):** The limb identity a field answer carries is selectable: the plan takes a limb order, defaulting to the family's `clump_system_order`, and the binding's field request accepts it (`"field": true` keeps the default; `"field": {"limbOrder": n}` selects). A higher order parts the crown into more and smaller systems, so a consumer's clump dropping can be as fine as the base prototype's per-branch coin. The default answers are unchanged from R6. [user]
- **R11 (owner, 2026-09-22):** The `experiments/voxel-field` script on the field uses R9 to cull wood thinner than a fifth of a cell and R10 for its clumps, and a new sheet of the oak, birch and spruce at 64 cells is produced beside the base sheet. The owner judged the first field sheet worse for the oak and birch (exposed wood specks, crowns breaking into islands at low keep) and better for the spruce; this sheet is what R8's verdict is taken on. [user]
- **R8:** The owner looks at one sheet of the oak, birch and spruce voxelised at 64 cells from the new field, produced by the `experiments/voxel-field` script, beside the sheet the same script drew from the structure export on the base, and accepts that each reads no worse than its base counterpart. The spruce reads as a low mass on the base too, because its twigs sit in the lower half of a 15 m leader; that is a preset matter recorded under Decision Context, not a defect of the field. [inferred]

## Boundaries

- R9 and R10 add values to the field's answer and a dial to its request; no voxel or grid type is added, and the plan's descriptors, the placement and the mesh are untouched by them. [inferred]

- No voxel or grid output is added to the core. The core answers occupancy queries; turning answers into cubes belongs to the consumer. [user]
- Dropping or thinning foliage clumps to open gaps is consumer styling and is not built here. The core's part is the owning limb id that makes it possible. [user]
- Leaf placement, the mesh output and the renderer are untouched. Replacing per-leaf CPU placement is the sibling pipeline spec. [inferred]
- No full-forest capture. Every measurement here is on single trees or the 8-tree forest. [inferred]

## Decision Context

- The field was written on 2026-09-05, when placed leaves were the only foliage representation the core had. Inside this repository its callers are its own tests, a measurement example and the browser binding. Whether anyone outside consumes the published field API is not known. [inferred]
- Rejected: a `voxels` output in the core. The owner ruled it out; a grid is one consumer's representation. [user]
- Rejected: deleting the field and making the structure export the voxel contract. Every consumer would then reimplement the tapered-segment test and guess a leaf reach that the preset already holds. [inferred]
- Per-leaf exactness is lost. It only matters at cells smaller than a leaf. [inferred]
- Siblings from the same conversation: "A slim growth-and-field package for the homepage" depends on this spec, and "One build pipeline; leaves expand from stations" depends on it for the plan stage's contract.

## Resolved before ready (host, 2026-09-22)

Measurements and sources are in `.flow/evidence/fn-100-the-field-reads-the-plan-not-placed/READINESS.md`, taken on master `3fdfb440`.

- R5's factor of two holds. A capsule sweep along the bearing runs, with reach = the segment's larger radius times the seating factor plus the element's farthest vertex at the largest scale, tested with the cell circumsphere as wood is, covers 1.40 times the base field's foliage cells for the oak, 1.64 for the birch and 1.16 for the spruce at 0.25 m; 1.18, 1.26 and 1.21 at 1 m. One oak base cell at 0.25 m is not in the sweep; base cells are leaf box overlaps, so R4 is judged against vertices, as written.
- Nothing stretches the spruce's bounds. Its leader reaches 15.00 m at a 0.3 mm radius while the twig nodes sit low (by height decile: 2412, 35993, 26951, 13785, 7761, 3120, 1038, 245, 14, 12 nodes). The prototype's half-filled grid is the tree. Every field box also reaches one trunk radius below ground through the root sphere (oak 0.43 m), which is the existing wood rule and stays.
- The spruce's low crown against a bare leader is a fidelity observation for the owner; it belongs to the species preset, not to this spec.
