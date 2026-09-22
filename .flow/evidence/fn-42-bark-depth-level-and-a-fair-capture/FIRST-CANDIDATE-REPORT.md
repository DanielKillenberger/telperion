# fn-42 bark candidate

The isolated branch now corrects the bark parallax direction and refines its below-crest intersection. Oak is darker with fine grain; spruce changes from saturated brown to muted grey-brown. The existing plate network and geometry remain intact. The owner selected visual reference matching, so unscaled photographs no longer supply exact RGB or physical-scale acceptance targets.

Open `review.html` for the two calibrated before/after patches, local references and the eight recorded fn-32 poses. The host inspected both patches and four trunk/branch images. The spruce branch pose is heavily obscured by twigs; the flat patch is the useful material comparison. Historical fn-32 images include intervening generator/rendering differences. The current oak still has smooth, rounded large relief shapes; this candidate does not redesign the accepted plate network or claim photorealism.

| Flat patch mean RGB | Before | Candidate |
|---|---|---|
| Oak | 131.82, 135.03, 127.34 | 117.29, 119.83, 111.32 |
| Spruce | 95.89, 71.73, 53.05 | 117.11, 112.79, 104.39 |

These are render diagnostics under the recorded lighting, not calibrated photo matches. Both patches span 0.4 m. Source and image hashes are in `candidate/provenance.json`.

## Checks and limits

The selected bark depth, detail, distance, field, filter, occlusion, plates, resolution, structure, structure-colour, grain, smooth-bark and smooth-mean test suites pass on the final relevant rows. Deterministic redraw and geometry checks pass. The analytic GPU test failed the original parallax direction and passes the correction; ramp intersection error falls from 0.005 to 0.0028125 m. Production WGSL validation, six core material tests, the calibration coordinate test, formatting and renderer all-target Clippy pass. Logs retain the two rejected grain strengths; no test bounds changed.

Oak 4× distance error is 2.595817/255. Grazing errors are oak 2.697367 and spruce 2.980475 against 3.0. Spruce has little margin. The final grain strengths are oak 0.20 and spruce 0.15; the initial 0.45/0.35 failed the existing bounds.

The beech and birch close-up footprint sweeps pass the unchanged 0.03 adjacent-step bound at 0.027001 and 0.028662. The additional hero reconstruction remains unresolved. Its recorded historical dimensions are 1350×900, but its full camera recipe is absent. At those dimensions both the candidate and pre-change shader control reach a 2.26 far/near band ratio at 4×, unlike the historical 1.05. This fails the reconstructed diagnostic and cannot certify the historical hero gate. It is not evidence that this patch introduced that discrepancy. The failed sweep JSON and both control tables remain recorded; R2 is not declared fully complete.

R4 remains incomplete. Baseline GPU utilization was 11%, so no valid native before/after or browser-orbit timing is claimed. The optional timing fixture is ready; its 400×400 patch is diagnostic and does not replace the required full-screen trunk measurement.

R1 is implemented. R2 needs the historical hero sweep recipe recovered and verified. R3 is a captured visual candidate. R4 needs an uncontended measurement session. R5 awaits the owner’s oak and spruce verdicts; its requirement is “the spec closes only on accepting verdicts.” No task completion, merge or PR is claimed.

## Friction reviewed

- The old near/far design conflicted with fn71; reconciled before implementation. Re-anchor older rendering specs against later owner constraints.
- Missing originals and an unnecessary continuation stop cost searches and a user turn. Persist local source metadata and continue bounded replacement searches through rejection.
- The oak source search spent about seven minutes and a spruce download was rate-limited. Keep inspected subject labels, dimensions and recoverable URLs beside source records; no new source-search spec created.
- The flat fixture needed an explicit radius and front-facing light. Both contracts now live in the helper and capture record.
- Compilation and the debug grazing mask cost time. The warmed optimized profile reduced the two resolution tests from 112.34 to 11.13 seconds after an 8.38-second build. This is local setup, not a repository spec.
- GPU contention prevented valid timing. Use an idle interval; no polling or benchmark retry loop was run.
- Excess grain failed existing gates and was reduced. This is a normal caught regression; no workflow change proposed.
- The hero sweep lacks a complete replay recipe. Preserve camera, view/mask, dimensions, seed, source hashes and command alongside curves. Proposed as a line in an open evidence-tooling spec only if the owner chooses it. No spec created.

Route: fn-42 → approved qualitative references → isolated implementation → visual checkpoint.
Work ran. Automated implementation/completion review skipped because review.backend=none; host inspected the diff. Native visual captures ran; browser timing is pending. PR creation and completion remain pending R2/R4/R5. Tracker sync: n/a (bridge inactive).
