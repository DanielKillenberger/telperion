# Reference-matched stills and the species QA pass

## Conversation Evidence

> user (2026-09-14, on the fn-34 judging set beside the Oregon State references): "now i think we should make a shot that closely imitates the reference image to be able to compare. And after that do a QA pass. To me it's clear that it's not there yet."
> owner (2026-09-14, the catalogue track): the owner's eye is spent on the final round of each species and the value tier spends the rest.

## Goal & Context
<!-- scope: business -->
<!-- Goal & Context: 40% [user], 40% [paraphrase], 20% [inferred] -->

The owner judged fn-34's beech and birch beside their reference photographs and rejected the round, with the reason that the comparison itself was unfair: the stills use the renderer's one fixed three-quarter camera, its default noon sun, a scale figure and a flat grey-green disc, while each photograph has its own viewpoint, framing, light, season and aspect. A species cannot be judged until the generated tree is shown the way the photograph shows the real one. [user]

This spec gives every reference photograph a matched still: the same camera direction and framing, the same sun, the same foliage state and the same aspect, with the figure and the room out of the way, rendered by the same headless target and recorded by the same species runner. Beside it, a pair composite puts photograph and still at the same height for the owner's eye, and a first measured comparison records what the eye would otherwise have to estimate: the crown's silhouette proportions and the centre-crop colour on both images. The QA pass is that instrument run over a species' references, the numbers recorded, and then the owner's verdict on the pairs. [paraphrase]

fn-34's second round is the first use, on the beech and the birch. Every species spec from the template judges on matched pairs from then on, and the legendary trees in fn-10 judge on film frames the same way. [paraphrase]

## Architecture & Data Models
<!-- scope: technical -->

- **A shot block on the reference record.** The fn-19 reference schema gains an optional closed `shot` object: `camera { azimuth_deg, elevation_deg, fill, target, fov_deg }`, `light { sun_azimuth_deg, sun_elevation_deg, overcast }`, `foliage: leaf-on | hidden`, `aspect: [w, h]`, `confidence: high | medium | low`, and a `derivation` sentence saying how the numbers were read off the photograph (horizon height gives eye elevation, the tree's base and apex give distance and tilt, shadow direction gives the sun, the page's season gives foliage, the pixel size gives aspect). `fill` is the fraction of the frame's height the crown occupies; `target` is `crown | base | trunk` and names what the photograph centres on. A record without a shot block is judged at the existing fixed views, as today. [inferred]
- **Two flags on the headless target.** `--camera '<json>'` is parsed against a default the way `--scene` is, refuses unknown keys, and replaces the hero pose's fixed direction and margin with the authored azimuth, elevation, fill, target and field of view while keeping the bounds solve for distance, so the crown lands at the declared fill on any aspect. `--no-room` leaves out the scale figure and draws the ground disc in the scene row's ground colour with no figure, so the frame holds the tree and the sky. Overcast is expressed through the existing scene row: the sun colour scaled down and the sky colours lifted by the `overcast` fraction, no new light. [inferred]
- **One job per reference in the species runner.** For each reference record with a shot block, `tests/species.mjs` adds a job `<case>-<reference-id>` at the record's aspect, with the camera, the scene and the view derived from the block, beside the existing whole, bare and leaf jobs; the receipt records the block and the argv. A record at the `attached-shoot` scale pairs with the existing leaf view and takes no camera. [inferred]
- **The pair composite.** `scripts/compare-references.py` (Pillow, the way `analyze-species-occupancy.py` already uses it) writes `<case>-<reference-id>-pair.png`: photograph left, still right, scaled to one height, a thin rule between, the reference id and the seed in a caption strip. It never modifies either image. [inferred]
- **The instrument.** The same script measures both images on the matched framing and writes `<case>-<reference-id>-compare.json`: the tree mask by background difference on the still and by a recorded crop box on the photograph, then crown width to height ratio, crown base as a fraction of height, fill, and for bare shots the dark-pixel fraction inside the crown box; and the fn-29 centre-crop mean RGB with channel order on both. The photograph's crop box and mask threshold are in the shot block so the numbers are reproducible. No number gates in this spec; the values are recorded beside each pair and the round's report tabulates still against photograph. [paraphrase]
- **The QA pass.** For a species, the runner renders every matched job at the first fixed seed, the script writes the pairs and the comparison JSON, `REPORT.md` tabulates them, and the owner judges the pairs. fn-34 round two runs this on the beech and the birch. [user]

## API Contracts
<!-- scope: technical -->

- **Headless flags** `--camera '{"azimuth":210,"elevation":8,"fill":0.82,"target":"crown","fov":32}'` and `--no-room`; unknown keys and out-of-range values refused naming the field; `--camera` composes with `--view`, `--scene` and `--size`. [inferred]
- **Reference schema** `shot` as above, added to `.flow/evidence/fn19/protocol.json` `$defs.reference` as optional with `additionalProperties: false`; existing records validate unchanged. [inferred]
- **Runner outputs** `<case>-<reference-id>.png`, `<case>-<reference-id>.json`, `<case>-<reference-id>-pair.png`, `<case>-<reference-id>-compare.json`, listed in `captures.json`; the round's `stills.json` lists the pairs with sha256 and the verdict slot. [inferred]
- **Script** `python3 scripts/compare-references.py --references <references.json> --captures <dir> --refs <.refs dir> --out <dir>`; exits non-zero on a missing photograph or a record whose asset sha256 does not match. [inferred]

## Edge Cases & Constraints
<!-- scope: technical -->

- **Camera ranges.** Azimuth 0 to 360, elevation minus 20 to 80, fill 0.2 to 1.0, field of view 10 to 90; the eye never goes below the ground plane; a subject shorter than the figure keeps its existing eye floor rule off since the figure is gone. [inferred]
- **The sky has no sun disc**, so a low sun shows in shading and shadow only; a photograph's sky is not matched and the comparison is of the tree, not the setting. [paraphrase]
- **The photograph's camera is estimated**, never known. The shot block's confidence says how well, and a low-confidence block is still a fairer comparison than the fixed view; the owner may correct any value, and the runner re-renders from the record. [inferred]
- **Foliage state has two values**, leaf-on and hidden, as the protocol already names; a photograph in autumn colour or partial leaf-out is matched as leaf-on and the limitation is recorded. [paraphrase]
- **Reference bytes stay ignored.** The script reads `.refs/` locally; pairs and comparison JSON are evidence, but a pair composite contains the photograph, so pairs are written to the ignored stills directory and recorded by sha256 in `stills.json`, never committed. [paraphrase]
- **Determinism.** Same record, seed and commit give a byte-identical still and identical comparison numbers. [paraphrase]
- **Reading budget.** The implementer reads at most four pairs per round; the owner judges the full set. [paraphrase]
- **fn-32 and fn-33 in flight.** fn-32's structure statistics for bark, when they exist, join the same comparison JSON; this spec measures silhouette and colour and leaves bark structure to fn-32. [inferred]

## Acceptance Criteria
<!-- scope: both -->

- **R1:** A reference record may carry a shot block naming camera, light, foliage state, aspect and confidence with a derivation sentence, validated by the fn-19 schema; every existing record validates unchanged. Errors: an unknown key or out-of-range value is refused naming the field. [inferred]
- **R2:** The headless target renders a still from an authored camera through `--camera` and without figure or room through `--no-room`, composing with view, scene and size, with the crown at the declared fill on any aspect; the existing views render byte-identically when neither flag is given. Errors: an invalid camera is refused naming the field; a fixed-view pin that moves fails. [paraphrase]
- **R3:** The species runner renders one matched job per reference record with a shot block at the record's aspect, records the block in the receipt, and the compare script writes a pair composite and a comparison JSON per job with silhouette proportions and centre-crop colour for both images. Errors: a missing photograph or a mismatched asset hash fails the job naming the record. [user]
- **R4:** The beech and the birch each carry a shot block on every whole, bare and base reference, and fn-34's second round renders the matched pairs at the first fixed seed, tabulates the comparison in its report, and the owner judges the pairs and records the verdict in fn-34. Errors: a rejecting verdict stops fn-34 again with the owner's words. [user]
- **R5:** `templates/species-spec.md` and the onboarding doc judge a species on matched pairs from this spec on, and the automated tests cover camera parsing and ranges, the fill rule on two aspects, the no-room frame, schema validation of a shot block, and the compare script's numbers on a synthetic pair. Errors: a missing case is a review finding, not implementer discretion. [inferred]

## Boundaries
<!-- scope: business -->

- No numeric gate on the comparison in this spec; the instrument is established and its numbers recorded, gating is decided once a few species have numbers. [inferred]
- No sky sun disc, no background matching, no photograph editing. [paraphrase]
- No bark structure statistics; fn-32 owns them. [paraphrase]
- No new light; overcast is the existing scene row's colours. [inferred]

## Decision Context
<!-- scope: both -->

### Motivation

- The owner rejected fn-34's first round because the fixed views made the comparison unfair before it was made, and asked for a shot that closely imitates the reference image and then a QA pass. [user]

### Implementation Tradeoffs

- A shot block on the reference record over a separate camera file: the record already carries scale, context and limitations for the photograph, and the runner already iterates records; the camera is one more fact about the photograph. [inferred]
- Authored camera through the existing bounds solve over a free camera: distance stays solved from the crown so a declared fill means the same thing on every seed, and the fixed views stay byte-identical. [inferred]
- Pillow over ImageMagick for the composite and the instrument: the repo already uses Pillow in a script and the fn-29 measurement was a hand-run `magick` command nobody could rerun. [paraphrase]
- Silhouette and colour first, structure later: proportions and colour are what a whole-tree photograph can be measured for at web resolution; bark structure needs the close references fn-32 works from. [inferred]

## Parked unknowns

- Which silhouette statistics separate a good match from a poor one across species; the first two species' numbers decide what gates.
- Whether the photograph's tree mask needs a hand-drawn crop box per record or a background-difference heuristic suffices on landscape photographs.

## Strategy Alignment

- Follows "The catalogue": the owner's eye on the final round, with the instrument doing the rest.
- Follows "Growth and botanical fidelity": reference-based visual QA exposes structural mistakes counts cannot catch, and it has to be a fair comparison.

## Resolved via Codebase

- Camera: `FIELD_OF_VIEW = 38.0` and `FRAME_DIRECTION = (0.62, 0.28, 1.0)` are constants (`crates/telperion-render/src/camera.rs:7`, `:16`); `hero_pose` solves distance from the bounds ellipsoid at `FRAME_MARGIN = 1.15` (`camera.rs:61-111`); no CLI flag sets azimuth, elevation, distance, target or field of view (`crates/telperion-render/examples/headless/walk.rs:140-258`); the seam is `headless.rs:53-55`, as `.flow/evidence/fn29/stills-driver.rs:24-32` already does with an explicit pose.
- Sun and sky: `SceneRow` (`crates/telperion-render/src/scene/row.rs:19-42`) with `sunAzimuth` default 135, `sunElevation` default 55, sun and sky colours, settable per still through `--scene`; the sky gradient has no sun disc (`shaders/sky.wgsl:22-29`).
- Foliage state: `--view bare` draws no crown and no crown shadow (`crates/telperion-render/src/foliage.rs:216-218`, `lib.rs:352-353`); the protocol's `leaf_state` enum is `leaf-on | hidden` (`.flow/evidence/fn19/protocol.json` `$defs.conditions`).
- Room: ground disc of 400 m and a 1.8 m figure drawn in every whole, bare and clay still (`crates/telperion-render/src/scene/room.rs:18-27`, `scene.rs:274-290`).
- Aspect: `hero_pose` and the frame uniform take width over height (`camera.rs:79-80`, `lib.rs:319`); tests cover 0.6 to 3.0.
- Reference schema: `$defs.reference` in `.flow/evidence/fn19/protocol.json` has no camera, season or lighting field; season hides in `context` prose (`.flow/evidence/fn34/european-beech/references.json`).
- Runner: `tests/species.mjs` `VIEWS = ['whole','bare','leaf']` (`:60`), one global `SIZE = '960x720'` (`:55`), jobs at `:156`, argv at `:83`, provenance at `:154`.
- Instrument precedent: a hand-run ImageMagick centre 400 by 400 crop mean, transcribed into `.flow/evidence/fn29/round5-crops.json` (`.flow/evidence/fn29/REPORT.md:241-256`); no committed code; fn-32's structure statistics are specified (`.flow/specs/fn-32-bark-as-plates-and-scales.md:27`) and not implemented on its branch.
- Composite tooling: none; `scripts/analyze-species-occupancy.py` is the Pillow precedent.

## Requirement coverage

| Requirement | Task |
|---|---|
| R1–R5 | TBD during planning |
