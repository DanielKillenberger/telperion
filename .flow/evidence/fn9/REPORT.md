# FN9 species completion evidence

2026-09-06. The required numeric and visual checks pass at production `0a8f343`; same-model implementation review is SHIP. Spec-completion review remains unassessed: its preflight could not resolve the default main ref, and further Astra review was stopped after the owner questioned using the builder model for review. No owner approval or merge is claimed. Earlier passes are archived in [the iteration archive](../../../experiments/fn9-iterations/README.md). The frozen [profiles](profiles.json), [seeds](seeds.json) and [references](REFERENCES.md) remain unchanged. Raw receipts (numeric, visual, captures, costs, gates, sources, previews) are archived; see `.flow/evidence/ARCHIVE-MANIFEST.md`.

## Current implementation

Production through `0a8f343` retains generated oak and spruce interior subdivisions, replacing five post-growth occupancy transforms. Spruce grows lateral shoots along descending secondary spans and bounds their downward component without losing lateral reach. Oak directs intermediate infill toward reachable gaps between terminal supports. No camera or seed-specific dispatch enters the generator. Needle dimensions and station spacing are unchanged.

Species and seed query parameters initialize the harness (`?species=norway-spruce&seed=1`). `npm test` rebuilds Wasm first. An explicit one-instance budget rejects instead of truncating; invalid species links show a recoverable error.

## Numeric and software checks

All 48 frozen fixed/fresh cases pass the gating numeric checks. Counts and non-gating dimensions are contextual.

| Species | Height m | Crown width m | Individual foliage units |
|---|---:|---:|---:|
| Oregon white oak | 16.418–20.359 | 20.680–24.475 | 479,986–584,844 |
| Norway spruce | 15.000–15.006 | 8.294–8.935 | 7,465,250–8,009,273 |

The pre-correction replay at `1297516` also passed all 48 numeric cases (327,275–541,606 oak leaves, 5,121,018–5,240,583 spruce needles); numeric success alone did not establish botanical fidelity. Timings are observations, not a controlled speed comparison.

Native workspace tests (68 passed, 6 ignored), 106 harness tests, rustfmt, strict Clippy, typecheck, build, Flow validation and the Wasm/browser suite pass.

## Inspected visual disposition

All 84 captures succeeded; 59 views were inspected (39 required mature whole/bare/foliage views, nine shared-template views, eleven anatomy/curtain/junction views). 35 numeric-only specimens remain visually unassessed. Per-view owner feedback is null.

Oaks: broad irregular crowns, low crooked spreading limbs, fine subdivisions, alternate attached lobed blades. Spruces: conical taper, dominant leader, separated irregular tiers, hanging/diagonal secondary sprays, needles surrounding twigs with forward bias.

The junction view exposed a 4.09 mm flat-ended structural terminal (spruce seed 1, node 5978). The correction narrows childless structural endpoints to fine-twig distal scale after radius solving (distal radius 2.045393 -> 0.25 mm); topology unchanged; checked across all five templates and 807 affected endpoints.

| Required specimens | Crown | Habit | Gaps | Taper | Foliage |
|---|---|---|---|---|---|
| Oak 1, 2, 3, 2666899686, 762807349, 1444323199 | Supported | Supported | Supported | Supported | Supported |
| Spruce 1, 2, 3, 1982700925, 281313742, 2271779095, 4250668600 | Supported | Supported | Supported | Supported | Supported |

Scoped reference-based assessments, not owner approval. Ordinary, Telperion and Laurelin were checked in all nine views; their shared geometry hashes are identical before/after the final correction.

## Reproduction

[Replay instructions](../../../tests/migration/README.md) cover CPU generation, browser capture and the optional hardware wrapper. Mature captures used NVIDIA Vulkan with no generation cap. Spruce seed 1 renders ~7.9 million needles. Frozen targets, ranges and seeds were not relaxed.

## Owner feedback

Owner, 2026-09-06, after the live spruce was served: "but good job on the implementation looks good". Implementation feedback, not a per-seed rubric score. The owner questioned same-model implementation review; no cross-model independence is claimed. No PR, merge or release was performed.
