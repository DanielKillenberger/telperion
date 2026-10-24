# Tree space: the ordinary tree as the space's centroid

## Goal

The `ordinary` preset builds again on the engine path (fn-198 refused it) as the centre point of the tree space: the visible proof that a point between species is itself a tree. Owner (2026-10-08): "ordinary tree we don't have to replicate the old one it should just be a tree that's the center point of the tree space. The mid avg tree."

## Decisions

- Corners are the passed branching species: European beech, Oregon white oak, Norway spruce; the silver birch joins when fn-213 passes (host and owner, 2026-10-08). The date palm is excluded: its rosette has no lateral branching to average.
- **Centroid of shape, mean of size (host, 2026-10-08).** The raw centroid outgrew the engine's 20M-phytomer budget at 80 (86 to 288 s before refusal): the spruce's lateral production compounds with the broadleaves' lifespans. Every row stays the equal-weight centroid (light rows included); every age's lateral probabilities are then multiplied by one factor so the light-neutral expected kept phytomers at 80 equal the corners' geometric mean. `examples/centroid.rs` computes it; a rerun reproduces the preset byte for byte.
- Every averaging rule is the space's own two-family walk (`blend::families`, fn-206) folded to 1/3 weights; none invented. Dressing stays the default family's (a later owner question).
- Age-80 walks deferred to fn-230, which owns them (host, 2026-10-08): points between species outgrow the budget at 80 across the space, so the gap predates fn-215.
- The early refusal `foreseen` (Wasm only, expected vs 4M, not called by `settled`, no count for light-reading species): host said record, do not extend.

## Acceptance Criteria

- **R1:** Rows are the equal-weight centroid of the passed corners, values only, building natively and in the harness (amended: sized to the corners' mean) — met; lateral factor 0.7419, target 3,559,562 light-neutral phytomers.
- **R2:** A sheet (seeds 1, 7, 42; ages 40, 80; bare and in leaf) beside the corners', viewed; build cost reported — met.
- **R3:** Walks to each corner with no visible pop (amended: at 40; 80 deferred to fn-230) — met at 40.
- **R4:** When the birch passes, its corner is added and the sheet retaken — open until fn-213; three-corner centroid recorded.

## Result

- Light-neutral expected kept at 80: beech 2,187,240; oak 4,430,867; spruce 4,653,767; unsized centroid 21,391,939; target (geometric mean) 3,559,562.
- Sized tree: age 40 keeps 246k-365k phytomers, grows in 1.2-1.6 s, 12.5-13.0 m tall. Age 80 keeps 1.84M-3.31M (all under target), grows at most 15M (under the 20M budget), 22-43 s (slowest preset at 80; corners 0.7-14 s), about 23 m tall, 15 m wide.
- The TypeScript field test passes in 41.4 s under its 60 s limit (77 s on the unsized rows).
- Sheet viewed by the host: at 40 an excurrent leader with fine level laterals; at 80 a narrow columnar crown with a dense band of fine wood low on the stem. Walks at 40: all fifteen frames build, height moves by degree, no visible pop. Stills were raw (untracked).

## Open

- For the owner: build time at 80 and the default dressing.
- R4: add the birch corner and retake the sheet when fn-213 passes.

## Friction

- Centroid at 80 refused only after minutes of growth (no early refusal for light-reading species) -> left to the host.
- No public grown phytomer count; bracketed by regrowing under lower budgets -> proposed a public grown count on the still runner's line.
- Walk frames at 80 that neither finish nor refuse (110 s timeouts) -> same early-refusal fix.
