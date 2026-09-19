# Revised bark scales and edges

The current candidate addresses the owner's structural rejection. Oak and spruce have smaller, flatter plate faces, narrower separations and irregular chipped boundaries. Their improved base-colour and grain rows stay fixed. The cellular partition and tree geometry stay intact.

The comparison in review.html includes the colour pass, rejected deeper relief, revised patches, local references and all eight fn-32 poses. Each flat patch spans 0.4 m. The photographs have unknown physical scale and lighting; oak's 650×567 source limits fine-detail judgment. Neither a photo-calibrated score nor photorealism is claimed. Owner acceptance is pending.

The revised physical p95-p5 height ranges are 3.185 mm for oak and 1.491 mm for spruce at a 1 mm footprint over the same sampled square. The rejected deep candidate measured 6.105 mm and 3.444 mm. Height increase is no longer the target; the owner explicitly asked for finer, shallower structure. The revised test compares rejected and revised rows through one current shader and checks that relief remains resolved while becoming shallower.

## Material contract

An explicit plateEdgeShape row blends rounded edges at zero into narrow chipped scales at one. Oak and spruce opt in; other presets and older documents default to zero. The field, validation, family blending, JSON parameter schema, generated browser types and shared renderer uniform carry the same value.

The shape value controls the physical profile independently of the smooth-material optimization. Enabling lichen, lenticels or peel must not silently select another bark shape. A runtime-uniform GPU test checks interpolation and specialization parity at 1e-6 profile units. The new profile's measured near/far means remain within the original 0.02 bound. The historical smooth reference configurations retain their existing mean coverage; current oak and spruce rows are tested through their actual authored profile.

Rough edges use two filtered noise projections to perturb the boundary distance. Their amplitude stays authored; the pixel footprint averages them. Narrow walls use denser quadrature. No near/far material switch, independent amplitude fade, image texture, species shader branch or geometry displacement was added.

## Verification

The final height, plate mean, interpolation, shader validation, distance, grazing and redraw checks pass at their stated bounds. Broader bark, grain and smooth-material suites passed; final integration and build receipts are listed below. Oak grazing error is 2.531334/255 and spruce 1.777487/255 against 3.0. The trunk error is 1.809105/255; repeated images are byte-identical. Beech and birch close-up sweep steps remain 0.027001 and 0.028662 against 0.03.

The historical hero reconstruction still fails its absolute 0.03 gate. The controlled pre-fn-42 source measures 0.467093, and the candidate measures 0.467411. The maximum difference between corresponding band ratios is 0.000926. Matching Whole view and its subject mask fixes the earlier mask mismatch but does not reproduce the original historical curves. Those curves lack a complete driver and mask population. The failure is present before this work. An owner decision is pending on separating that inherited failure from fn-42 acceptance; its absolute result remains failed, never relabelled green.

## Friction reviewed

The previous report retains every earlier friction entry and proposed remedy. The stale design was reconciled; replacement references and explicit fixture radius/light are recorded; optimized tests replaced the slow debug mask; native and browser timing now run serially. The premature host stops are recorded as host mistakes, not worker failures.

This continuation found four further issues. The narrower profile needed a measured far mean, now covered at the unchanged bound. Concurrent GPU probes once crashed natively; serial execution avoided that crash, without claiming a confirmed driver diagnosis. A smaller cell size exposed finite-sample variation, so the mean probe covers more independent axial sites. The final review caught an optimization flag selecting appearance; the explicit material row removes that coupling. Its initial test compiled each shape as a different literal, introducing f32 hash differences; runtime-uniform input fixes the fixture and preserves the original 1e-6 parity bound.

The inherited hero receipt needs exact camera, view, mask counts, source hashes and invocation preserved in future captures. The new helper records these and rejects empty masks. A repository evidence-tooling follow-up is proposed only for the owner's decision; none has been created. Local compilation/GPU setup issues do not warrant repository specs.

stage: work - ran (structural correction and material independence)
stage: impl-review - skipped(config: review.backend=none; host inspected the diff)
stage: completion-review - skipped(config: review.backend=none)
stage: native-visual-check - ran (two flat patches and both trunk views inspected; eight poses captured)
stage: browser-performance - ran (isolated WebGPU orbit)
Tracker sync: n/a (bridge inactive)

R1 is implemented; R2 awaits the inherited-gate decision; R3 has a revised candidate; R4 is measured; R5 awaits the owner's visual verdict. No completion, PR or merge is claimed.

## Final build and timing receipts

The explicit material row preserves both inspected flat patches byte for byte (shape-row-image-parity.json). Final Rust material validation/blending/compatibility tests, renderer distance/resolution/smooth tests, material-shader validation, profile interpolation/parity, Clippy, formatting, TypeScript, browser material round-trip and catalogue checks pass. Broader unchanged bark contracts passed in shape-final-other-gates.log. Receipts are shape-integration.log, shape-row-runtime.log and shape-typescript.log. All numerical tolerances remain unchanged.

RTX 3080/Vulkan, 1600×1000, mature seed-7 oak; 8 conditioning, 8 warmup and 120 measured frames. Native runs precede the isolated browser orbit. All current timing reports are valid under the existing classifier.

| Native p50 | Pre-fn-42 | Final candidate | Increase |
|---|---:|---:|---:|
| Whole-tree total | 8.9231 ms | 14.5405 ms | 5.6174 ms |
| Full-screen cylindrical trunk vegetation | 14.8695 ms | 38.4031 ms | 23.5336 ms |

Final p95 is 14.9348 ms whole-tree total and 38.8321 ms trunk vegetation. Browser orbit averages 66.6 fps; wall p50 is 10.1 ms and p95 30.0 ms. Average fps does not imply even cadence. Rendering cost is explicitly permitted for this fidelity pass; the measured regression is retained for future optimization.


## Owner response and colour continuation, 2026-09-20

The owner judged the revised structural candidate “much better” and requested red/brown colour variation before ending the session. Directional overlapping flakes are captured separately as fn-90. This feedback accepts the direction of the shape correction; the forthcoming colour revision still needs its visual verdict. The inherited hero-sweep decision remains open.

The quick colour pass changes existing fissure RGB rows for oak and spruce only; crest RGB remains at the structural candidate values. It preserves base colour, physical relief, grain, strengths, shader code and all other species. Fresh colour captures and focused checks supersede the shape candidate only where explicitly labelled. Existing timing measurements belong to the structural candidate; no new performance claim is made from changing colour constants.

The final colour patch changes mean RGB by (+3.66, -0.59, -2.47) for oak and (+1.21, -0.32, -1.41) for spruce in 8-bit image codes. The mean is slightly warmer, not numerically identical. Spruce channel-difference spread increases; oak spread decreases despite its warmer recessed patches. These are diagnostics, not claims of greater colour diversity in every metric or a photo-match score. See colour-final/comparison.json.

Final colour checks pass: material validation, structure-colour, material shaders, distance/resolution, deterministic calibration redraw, browser material round-trip, and Wasm/catalogue generation. Commands and receipts are in COLOUR-HANDOVER.md. The colour worker spent about one minute locating an image-metrics interpreter; preserve its exact invocation in future capture recipes. This is local setup friction, not a proposed repository spec.

## Acceptance, 2026-09-20

The owner accepted the appearance and live distance render and authorized squash merge after the inherited hero failure had been disclosed. Birch material tuning remains separate; its authored controls already exist and were unchanged. This supersedes the pending visual/gate statuses above. The historical hero absolute result remains failed, with an acceptance exception supported by the controlled baseline comparison and owner live verdict. All other stated numerical bounds remain unchanged.

Final live QA: species switching, bare views, zoom/orbit and invalid-seed handling passed in isolated Chromium. No P0/P1; pre-existing missing favicon is P2. Live artifacts are under .flow/tmp/qa-fn-42-bark-depth-level-and-a-fair-capture. One incorrect view-button locator cost a 60-second timeout; the labelled select fixed it. That host mistake is in FRICTION.md, with no new spec proposed.
stage: qa - ran (jev ui 0.59; live Vite renderer; SHIP with one pre-existing P2)
