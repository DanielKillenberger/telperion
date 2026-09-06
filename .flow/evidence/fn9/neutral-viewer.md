# FN-9.8 — species and anatomy in the neutral viewer

The exported five-entry catalogue drives selection; changing identity preserves the independently entered seed. Whole-tree, bare-branch and foliage-detail views are available through the labelled `view` selector. Selection, seed changes and view changes request framing of the incoming specimen. The Two Trees comparison still uses only its two authored presets, with its existing full-size geometry/aggregation tests retained.

Botanical and Supernatural fieldsets separate controls. Botanical controls include each active habit's numeric settings and foliage length, width and connector length. Supernatural effects have an explicit enable checkbox and their own bending controls; torsion no longer changes botanical lean. Native validation continues to report invalid settings through the existing recoverable build error UI.

The finalized single-prototype/matrix representation remains intact. The adapter measures every transformed vertex (including connectors) for the aggregate instance box and sphere. Detail mode selects the middle placed unit, retains its matrix and geometry, recomputes its bounds, and looks along its transformed broad face. It isolates one unit rather than claiming to show attachment distribution across a shoot. Comparison detail units are arranged side by side at their own scale. Empty detail has a finite fallback frame. Detail framing supports sub-centimetre near planes and hides the scale figure.

## Verification

- `npm run typecheck`
- `npm test -- harness/stage.test.ts harness/skeleton-view.test.ts`: 82 tests. Includes full-size Two Trees preservation, stale/prototype-bound rejection, actual placed-unit detail bounds, and empty detail/bare handling.
- `BROWSER_EVIDENCE=.flow/tmp/fn98-browser node tests/browser/integration.mjs`, with Vite on `127.0.0.1:5184`: binding ownership/validation/determinism gates, transactional replacement, load/build retry, species selection preserving seed, seed-driven geometry differences, all view selections, empty foliage rendering, finite bounds/spheres and rendered draw counts before/after orbit.
- `git diff --check`

The torsion separation intentionally invalidated two old assumptions: zero torsion no longer cancels botanical lean. The conversion test now asserts preserved nonzero lean, and the supernatural straight-to-writhing geometry fixture explicitly sets lean to zero.

## Durable captures and CPU backend

All six species PNGs under [viewer/](viewer/) were inspected. The whole captures contain foliage; the detail captures show the oak's lobed blade and connector and the spruce needle's length and distinct faces. Bare captures show the same specimen's branches. These are viewer/anatomy visibility evidence, **not a mature-species fidelity pass**. The spruce fixture is sparse, and the coarse oak lobes remain visible as generated; those traits must be judged against references in task 9.

The backend reported `ANGLE (Google, Vulkan 1.3.0 (SwiftShader Device (Subzero) (0x0000C0DE)), SwiftShader driver)`. Repository Playwright's bundled Chromium ran headless, with no visible window or added consent-bypass flags. Captures use 960 × 720 at DPR 1 and a 240-second screenshot timeout. No hardware frame-time claim is made.

Both seed-42 fixtures use height 4 m and 40 attractors; spruce uses three tiers. Other preset settings are retained, without new node/instance caps. Both report complete generation: oak has 1,571 nodes and 9,306 foliage instances; spruce has 298 nodes and 3,420 instances. Whole/bare/detail render counts are 4/3/2 respectively, unchanged after orbit. `captures.json` records exact input presets, timings, bounds, counts and backend. The same fixtures bound UI rendering from its first build; the catalogue defaults themselves are unchanged.

Earlier attempts closed the browser during the UI retry/seed sequence, including an attempt that rendered full-size Ordinary before applying the fixture override. The successful gate applies the override before the first UI render and matches Vite's query-suffixed module URLs. Full-size mature oak/spruce captures were not attempted here; reliable large-scene software rendering remains a task-9 concern. All requested reduced-size whole/detail captures succeeded. An initial end-on spruce detail was replaced after orienting the camera from the selected instance.

Replay: start `npx vite --host 127.0.0.1 --port 5184`, then run the browser command above. Its generated artifacts are copied into `viewer/` for versioned evidence. Native code and Wasm were not changed; no Rust rebuild is required for this task.

Implementation commit: `b038e44`; follow-up verification/evidence commit recorded in Git history. No PR, review workflow, or merge was run. Next job: fn-9.9, including the fixed/fresh seed and reference-comparison protocol.
