# fn-42 added bark relief

Oak and spruce now have stronger physical height profiles, with the colour and grain rows held at the owner's preferred first-candidate values. Oak plate domes and edges rise, and its furrow floor broadens. Spruce plate domes and edges rise, with stronger directional cavity shading. The accepted plate network, geometry and silhouette stay intact.

Open [the comparison](review.html) for the previous colour pass beside added relief, local reference photographs and all eight re-rendered fn-32 poses. The host inspected both final flat patches and both trunk views. Relief is more visible; oak still has broad rounded forms, and spruce trunk views contain obscuring twigs. No claim of photorealism or owner acceptance is made.

| Physical p95-p5 height range over the 0.4 m patch | Previous profile | Added relief | Change |
|---|---:|---:|---:|
| Oak | 5.170 mm | 6.105 mm | +18.1% |
| Spruce | 2.779 mm | 3.444 mm | +23.9% |

The new GPU test reads production filtered height directly at a 1 mm footprint and requires at least 15% improvement. Colour and lighting cannot make this test pass. Photographs remain qualitative references with unknown scale and lighting; oak's source is only 650×567. The flat render alone has calibrated width. Capture records contain render mean RGB and structure diagnostics.

## Filtering and verification

Rough bark uses four shaded cells per axis to resolve steeper slopes before averaging light. The existing smooth-bark specialization retains its prior two/three-cell quadrature. Both use the same footprint-integrated height field. No amplitude fade, near/far switch, new material row or species-specific shader branch was added.

The selected native bark depth, detail, distance, field, filter, occlusion, parallax, plate, relief-range, resolution, structure, structure-colour, grain, smooth-bark and smooth-mean suites pass at unchanged bounds. Final material WGSL validation, fixture coordinate check, renderer all-target Clippy and formatting pass. Earlier failures remain in logs, including too much initial parallax and spruce furrow width. Final receipts are in relief-resolution and relief-final-gates.log.

Beech and birch close-up footprint sweeps pass the original 0.03 adjacent-step bound at 0.027001 and 0.028662. Their full-precision receipt is relief-sweep-check.json. Numeric sweep images are local diagnostics and excluded from version control.

The historical hero sweep remains unverified. Its original driver and mask counts are absent. The reconstructed Clay R>B mask also included foliage; removing foliage leaves no eroded wood pixels at factor four. Historical curves cannot establish that they measured the same pixels. This is neither a passing gate nor demonstrated new shader failure. No erosion, tolerance or required factor was relaxed. The bounded investigation is finished; R2 cannot be signed off against that historical recipe without a recoverable original or an explicitly agreed replacement measurement.

## Performance

RTX 3080, Vulkan, 1600×1000, seed-7 mature oak, 8 conditioning frames, 8 warmups, 120 measured frames. Before and candidate reports are valid under the existing fn-26 classifier. Runs were serialized. The earlier blanket rejection of nonzero desktop GPU utilization was too conservative; the earlier 11% reading was not itself proof of contention.

| Native p50 | Before | Added relief | Difference |
|---|---:|---:|---:|
| Whole tree, total | 8.9231 ms | 13.1840 ms | +4.2609 ms |
| Full-screen cylindrical trunk, vegetation pass | 14.8695 ms | 35.9537 ms | +21.0842 ms |

Whole-tree p95 total is 13.4200 ms; trunk p95 vegetation is 36.4155 ms. Trunk camera distance is 0.75 m from its centre. A separate flat full-screen benchmark is diagnostic and does not replace the cylinder. Full camera, light and material rows are in native-before/native-relief metadata. Candidate clocks were refreshed after the final smooth-bark specialization.

The final rebuilt browser renderer reports a valid orbit, 70.0 average fps over 10 seconds, with wall p50 10.1 ms and p95 30.0 ms. Average fps does not imply steady frame cadence. Browser provenance includes source/WASM hashes, adapter, flags, camera and visibility. The isolated canvas closed without leaked devices. Extra cost is permitted by the owner; these regressions are disclosed for later optimization.

## Requirement status

R1 is implemented. R2 has a measured relief improvement and passing native/close-up gates, but the historical hero gate is unverified. R3 has a fixed-colour stronger-relief candidate. R4 is measured and reported. R5 awaits the owner's judgment of both species; its exact closure rule is “the spec closes only on accepting verdicts.” The spec remains open. No PR, merge or completion is claimed.

## Friction review and proposals

- The stale near/far design was reconciled with fn-71. Re-anchor old rendering specs against later owner decisions before implementation.
- Missing originals, rejected source photos and premature search stopping cost searches, about seven minutes of source inspection and a user continuation turn. Keep source URL, subject, dimensions and credit with local cached references. Missing local files are setup problems, not a repository spec.
- The flat fixture needed explicit material radius and front-facing light. Both contracts now live in the helper and metadata.
- Initial compilation and a 112-second debug mask gate were avoidable local costs. The optimized profile reduced that gate to roughly 11 seconds. Keep using the warmed ci profile; no repository spec is proposed for local setup.
- Nonzero desktop GPU utilization was treated too strictly as a timing blocker. The existing validity classifier now provides the recorded decision; native and browser measurements are complete.
- Grain, stronger parallax, widened spruce floors and global denser sampling exposed real regressions. Existing gates caught them, and bounds stayed fixed. These normal implementation corrections need no new process spec.
- The host stopped at a narrow worker handoff before completing the relief objective, costing two explanatory turns and another continuation. Keep the full spec objective active across worker returns. The relief pass is now implemented and measured.
- Historical hero curves lack sufficient mask/capture provenance. The bounded diagnosis found foliage contamination and then an empty wood-only mask. Proposed evidence-tooling work would persist exact masks, pixel counts, camera, view, dimensions, seed, source hashes and command with every curve. The owner decides whether this becomes a line in an open spec; none was created.

Route: fn-42 → qualitative references → fixed-colour relief implementation → measured candidate. Automated review remains disabled by review.backend=none; the host reviewed the implementation diff. Native captures and browser timing ran. R2 historical evidence and R5 owner verdicts remain open. Tracker sync is not applicable.
