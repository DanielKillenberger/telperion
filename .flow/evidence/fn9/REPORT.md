# FN9 species completion evidence

2026-09-06. The required numeric and visual checks pass at production `0a8f343`; same-model implementation review is SHIP. Spec-completion review remains unassessed: its preflight could not resolve the default main ref, and further Astra review was stopped after the owner questioned using the builder model for review. No owner approval or merge is claimed. Earlier reports and captures are preserved intact in [the iteration archive](../../../experiments/fn9-iterations/README.md). The frozen [profiles](profiles.json), [seeds](seeds.json) and [references](REFERENCES.md) remain unchanged.

## Current implementation

Production through `0a8f343` retains generated oak and spruce interior subdivisions, replacing five post-growth occupancy transforms. Spruce grows lateral shoots along descending secondary spans and bounds their downward component without sacrificing their lateral reach. Oak directs existing intermediate infill toward reachable gaps between terminal supports. No camera or seed-specific dispatch enters the generator. Needle dimensions and station spacing remain unchanged; retained anatomy changes the foliage counts.

The requested [live spruce viewer](http://127.0.0.1:5184/?species=norway-spruce&seed=1) opens the mature preset with full foliage. Species and seed query parameters initialize the harness. `npm test` rebuilds Wasm first to prevent stale generated bindings. Compact browser fixtures specify their anatomy within the existing resource budget; an explicit one-instance budget must reject instead of truncating. Invalid species links show a recoverable error and preserve viewer controls. Peg capture selection uses the viewing direction projected into the twig cross-section, avoiding an impossible angular test on radially aligned twigs.

## Numeric and software checks

All 48 frozen fixed/fresh cases pass the gating numeric checks in [the final numeric receipt](final/numeric.json). Counts and non-gating dimensions remain contextual.

| Species | Height m | Crown width m | Individual foliage units |
|---|---:|---:|---:|
| Oregon white oak | 16.418–20.359 | 20.680–24.475 | 479,986–584,844 |
| Norway spruce | 15.000–15.006 | 8.294–8.935 | 7,465,250–8,009,273 |

The pre-correction replay at `1297516` also passed all 48 numeric cases, with 327,275–541,606 oak leaves and 5,121,018–5,240,583 spruce needles. Numeric success alone did not establish botanical fidelity. Final native measurements ran concurrently on the same host while the baseline ran sequentially: recorded timings are observations, not a controlled speed comparison.

Final native workspace tests pass (68 passed, 6 explicitly ignored historical/diagnostic tests), as do 106 harness tests, rustfmt, strict workspace Clippy, typecheck, build, Flow validation and the Wasm/browser integration suite. Raw logs are retained in [the final gate record](final/gates.json). The earlier small-fixture foliage budget failure is retained in `/tmp/fn9-final-browser.log`; the corrected compact fixture passes without increasing the budget.

## Inspected visual disposition

All 84 captures succeeded; 59 views were inspected for the final decision: the 39 required mature whole/bare/foliage views, nine shared-template views and eleven representative anatomy/curtain/junction views. [Per-view observations](final/visual.json) retain limitations and unassessed optional angles; [capture receipts](final/captures.json) retain renderer, preset, camera, source/Wasm/runner and PNG hashes. The 35 numeric-only specimens remain visually unassessed. Per-view owner feedback fields remain null. After using the served viewer, the owner said, “but good job on the implementation looks good”; that feedback is recorded separately below.

The six oaks have broad irregular crowns, low crooked spreading limbs and fine subdivisions, with alternate attached lobed blades. Seed 2 retains a deep upper notch; its former enclosed central window now has foliage. The frozen rubric asks for irregular windows and substantial spreading crown mass, not an opaque or perfectly symmetric crown. The final oak PNGs exactly match the already inspected pre-taper images.

All seven spruces show conical taper, a dominant leader, separated irregular tiers and substantial hanging/diagonal secondary sprays. Spaces remain between the sprays. Attached individual needles surround the twigs with forward bias. Isolated original elements resolve the four-sided profile; the connected peg-clear views resolve local base contact. Occluded optional socket views remain unassessed rather than being promoted to complete hidden-seam approval.

The junction view exposed an actual 4.09 mm diameter flat-ended structural terminal, node 5978/parent 5977 in spruce seed 1. [The regression record](final/terminal-regression.json) and before/after PNGs retain it. The correction narrows genuine childless structural endpoints to fine-twig distal scale after radius solving. This endpoint remains 41.899 mm long with the same 2.048824 mm proximal radius; its distal radius changes from 2.045393 to 0.25 mm. Its final connected render now shows continuous taper. All handoffs, positions and topology remain intact. The regression checks all five templates and 807 affected seed-1 spruce endpoints. Long apparent bare shoots in local detail views have needles outside the local foliage display subset; camera-plane cut polygons are not interpreted as anatomical cuts.

| Required specimens | Crown | Habit | Gaps | Taper | Foliage |
|---|---|---|---|---|---|
| Oak 1, 2, 3, 2666899686, 762807349, 1444323199 | Supported | Supported | Supported | Supported | Supported |
| Spruce 1, 2, 3, 1982700925, 281313742, 2271779095, 4250668600 | Supported | Supported | Supported | Supported | Supported |

These are scoped reference-based assessments of the required views, not owner approval or a claim of photographic equivalence. See each view's limitations in the receipt. The retained width counterexample remains numerically passing.

Ordinary, Telperion and Laurelin were checked in all nine required views. Their historical sparse foliage and sweeping forms remain. [The taper comparison](final/taper-comparison.json) records identical shared geometry hashes and PNGs before/after the final correction; these templates are not assessed against the natural-species rubric.

## Reproduction and cost

[Replay instructions](../../../tests/migration/README.md) cover CPU generation, browser capture and the optional hardware wrapper. Final bulk output is `/tmp/fn9-complete-20260906`. Mature captures used NVIDIA Vulkan, with no generation cap. Conservative frustum rejection omits only offscreen prototype bounds; ordered batching submits every retained instance. Raw capture status remains separate from manual visual assessment. The standard species runner intentionally exits 1 while visual results are unassessed; the final coordinator's exit 0 means only 84 successful captures. Exact temporary coordinator scripts are retained as historical execution records, not portable replay entrypoints.

The compact numeric receipt summarizes individual branch-length samples by count/range/quantiles and pins the full raw JSONL by hash; all gating checks remain intact. [Per-case costs](final/costs.json) preserve before/after native counts, times and output sizes. Baseline numeric concurrency was one, final concurrency three alongside capture work; no controlled speed comparison or software-GPU performance claim is made. Spruce seed 1 renders roughly 7.9 million individual needles. [Sources](final/sources.json) pin production and local reference hashes. [Gate records](final/gates.json) preserve commands, scope and logs, including earlier corrected failures.

Historical exact-count/protected-hash/no-lower-infill conditions were iteration controls, not attributed requirements; task 9 records their supersession by the architecture repair. Frozen botanical targets, numeric ranges and seeds were not relaxed. The iteration archive is preserved intact.

## Owner feedback and handoff

The owner questioned same-model implementation review after expecting it to be skipped. The completed Astra review is retained honestly as same-model review; no cross-model independence is claimed. The separate completion-review command failed before dispatch because its default `main...HEAD` snapshot could not resolve. No completion verdict was obtained and no further Astra review was dispatched. The spec remains open with completion_review_status unknown; all nine implementation tasks are done.

Owner feedback on 2026-09-06 after the live spruce was served: “but good job on the implementation looks good”. This is implementation feedback, not an invented per-seed rubric score. The live full-foliage page remains at http://127.0.0.1:5184/?species=norway-spruce&seed=1. Temporary test servers and the session-owned worker worktree were removed after integration. No PR, merge or release was performed.

stage: completion-review - failed(error: default main snapshot unavailable before dispatch; further Astra review stopped following owner steering)
