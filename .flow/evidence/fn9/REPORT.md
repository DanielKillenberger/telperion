# FN9 species completion evidence

2026-09-06. Final visual inspection and independent review are in progress; task 9 is not yet complete. Earlier reports and captures are preserved intact in [the iteration archive](../../../experiments/fn9-iterations/README.md). The frozen [profiles](profiles.json), [seeds](seeds.json) and [references](REFERENCES.md) remain unchanged.

## Current implementation

Production through `ec75925` retains generated oak and spruce interior subdivisions, replacing five post-growth occupancy transforms. Spruce grows lateral shoots along descending secondary spans and bounds their downward component without sacrificing their lateral reach. Oak directs existing intermediate infill toward reachable gaps between terminal supports. No camera or seed-specific dispatch enters the generator. Needle dimensions and station spacing remain unchanged; retained anatomy changes the foliage counts.

The requested [live spruce viewer](http://127.0.0.1:5184/?species=norway-spruce&seed=1) opens the mature preset with full foliage. Species and seed query parameters initialize the harness. `npm test` rebuilds Wasm first to prevent stale generated bindings. Compact browser fixtures specify their anatomy within the existing resource budget; an explicit one-instance budget must reject instead of truncating. Peg capture selection uses the viewing direction projected into the twig cross-section, avoiding an impossible angular test on radially aligned twigs.

## Numeric and software checks

All 48 frozen fixed/fresh cases pass the gating numeric checks in `/tmp/fn9-final-20260906/numeric.json`. Counts and non-gating dimensions remain contextual.

| Species | Height m | Crown width m | Individual foliage units |
|---|---:|---:|---:|
| Oregon white oak | 16.418–20.359 | 20.680–24.475 | 479,986–584,844 |
| Norway spruce | 15.000–15.006 | 8.294–8.935 | 7,465,250–8,009,273 |

The pre-correction replay at `1297516` also passed all 48 numeric cases, with 327,275–541,606 oak leaves and 5,121,018–5,240,583 spruce needles. Numeric success alone did not establish botanical fidelity. Final native measurements ran concurrently on the same host while the baseline ran sequentially: recorded timings are observations, not a controlled speed comparison.

Final native workspace tests pass (67 passed, 6 explicitly ignored historical/diagnostic tests), as do 106 harness tests, rustfmt, strict workspace Clippy, typecheck, build, Flow validation and the Wasm/browser integration suite. Raw logs: `/tmp/fn9-worker-final-workspace.log`, `/tmp/fn9-worker-final-clippy.log`, `/tmp/fn9-final-harness.log`, `/tmp/fn9-final-browser-compact.log`. The earlier small-fixture foliage budget failure is retained in `/tmp/fn9-final-browser.log`; the corrected compact fixture passes without increasing the budget.

## Visual assessment in progress

Final output is `/tmp/fn9-final-20260906`. Each capture records full preset parameters, camera, renderer, source/Wasm/runner hashes and PNG hash. The mature render uses NVIDIA Vulkan; compact integration uses SwiftShader. No foliage cap is introduced. Conservative frustum rejection only omits offscreen prototype bounds, and batching submits every retained instance in its original order. The runner deliberately leaves visual status unassessed for manual trait inspection.

Inspected oak pilots show substantial irregular crowns and retained interior subdivisions; seed 2 retains a deep upper notch while its former enclosed central window now has foliage. Spruce seed 1 shows a conical leader, irregular tier spaces and more substantial hanging sprays. These observations do not confer acceptance on uncaptured cases or hidden attachment seams. The other required fixed/fresh views, shared templates and close-up surfaces must be assessed before task completion. Owner feedback remains null.

See [replay instructions](../../../tests/migration/README.md) for CPU generation, browser capture and the optional hardware renderer wrapper. Historical exact-count/protected-hash/no-lower-infill restrictions were iteration controls, not attributed requirements; task 9 documents their supersession by the architecture repair. Botanical targets and numeric profile ranges have not been relaxed.
