# Bark depth, level and a fair capture

## Conversation Evidence

> user (turn 1, on fn-32's round-two stills): "it's much much better. but still has much room for improvement. Not sure if in this spec or not though"
> user (turn 2, on the host's recommendation to accept fn-32 for plates and scales and carry the remaining gaps to a new spec): "ok do it"

> user (2026-09-19, on the performance budget): "i think we'll have to find performance improvements all over the place anyway so i think i'd allow extra cost for now and optimize later?"

## Goal & Context
<!-- scope: business -->
<!-- Goal & Context: 20% [user], 60% [paraphrase], 20% [inferred] -->

fn-32 gave the bark a plate network and the owner judged it "much much better. but still has much room for improvement." [user] Two scripted hill-climb rounds then mapped, by measurement, what fn-32's rows could not reach, and the owner chose to accept that spec for plates and scales and carry the rest here. [paraphrase]

Three gaps remain, each measured in fn-32's round three and each outside that spec's boundary or its contract. The oak trunk is too light and too cool: with every fn-32 row at its darkest the crop mean reached only 140 144 133 against the reference photograph's 118 119 114, because the level lives in the bark base colour row fn-29 owns. Furrows read as crackle rather than valleys: relief depth is capped at 4 mm by the anti-aliasing contract the distance tests enforce, and widening the furrow floor over that cap reads as flakes with gaps, so the row shipped inert. And the dark fraction the references are judged by is mostly the cylinder: with every fn-32 term at zero the oak still measures 0.127 of its 0.147, because the crop compares a curved trunk under a sun with a flat photograph. [paraphrase] These are historical fn-32 findings. Re-measure the current renderer before choosing a depth change; fn-71 subsequently changed its filtering. A bark-only capture at a known scale makes the material judgeable. [inferred]

## Architecture & Data Models
<!-- scope: technical -->
<!-- Architecture: 50% [paraphrase], 50% [inferred] -->

- **A bark-only capture at a known scale.** Render square-on oak and spruce patches at a recorded physical width. Use replacement photographs as visual morphology references; their physical scale and colour calibration are unknown. Render structure metrics are diagnostics, not photo-match scores. [user decision, 2026-09-19]
- **Depth under one continuous filter.** Improve the resolved relief and parallax while retaining fn-71's accepted contract: relief leaves the image only through averaging its height over the pixel footprint, with no separate amplitude fade or near/far material switch. First measure the current field and depth response in the calibrated capture. [inferred]
- **Reference-guided colour.** Existing oak and spruce bark colour rows may move to improve exposed mature bark appearance. Judge colour under the recorded render lighting; do not fit uncalibrated photographs to an exact RGB target. [user decision, 2026-09-19]
- **Cost.** Extra rendering cost is allowed for this fidelity pass; optimization can follow. Record fresh baseline and candidate timings under the same conditions for the trunk close-up and the whole tree. The historical 5.2142 ms and 3.9823 ms figures are context, not acceptance limits. [paraphrase]
- **Judging.** The same order as fn-32: numbers first, the owner's eye last, four images at most per round. [paraphrase]

## API Contracts
<!-- scope: technical -->

- **Rows:** reuse the current physical bark and depth rows, with one explicit `plateEdgeShape` blend for the newly requested thin chipped profile. Default 0 retains the rounded profile; 1 selects the revised scales; intermediate values blend their height profiles continuously. Do not add the older proposal's near-footprint threshold. The fn-29 bark base colour and the fn-14 bark colour are existing rows that this spec is allowed to move on the oak and spruce presets. [inferred]
- **The bark-only view:** one headless capture recipe per species with the camera distance and field of view chosen so the crop width in metres is recorded beside the still, added to the evidence recipe, not a new public view or command. [inferred]
- **The reference scale:** record source, dimensions, credit and limitations; unknown photographed widths remain unknown. Only the rendered patch carries a calibrated physical width. [user decision, 2026-09-19]

## Edge Cases & Constraints
<!-- scope: technical -->

- **Filtering remains continuous.** Every existing distance, resolution and redraw test passes at unchanged bounds. Add close-range depth coverage without relaxing those bounds. [paraphrase]
- **Continuity through a camera walk.** The current footprint sweep remains a hard gate; no new amplitude fade or visible detail transition is permitted. [paraphrase]
- **Cost, both ways.** Measure and report the full-screen trunk and whole-tree costs by the fn-26 protocol, including any regression. No hard timing ceiling gates this fidelity pass. [paraphrase]
- **References.** Locally cached replacement exposed-bark photographs documented in REFERENCE-DECISION.md; never redistributed. Oak resolution limits fine-detail judgment. [user decision, 2026-09-19]

## Acceptance Criteria
<!-- scope: both -->

- **R1:** A bark-only recipe renders oak and spruce square-on with physical crop width, lighting and camera recorded. Report structure metrics as render diagnostics and compare exposed mature bark morphology visually with the documented replacement references. State unknown photo scale and lighting limitations; do not claim a calibrated photo score. [user decision, 2026-09-19]
- **R2:** Scale and furrow relief matches the owner's structural feedback in the calibrated close-up: flatter faces, finer separations and irregular chipped edges rather than broad deep valleys while retaining one continuously footprint-filtered material. No separate amplitude fade or near/far switch is introduced; every existing distance, resolution, footprint-sweep and redraw test passes at unchanged bounds. Errors: a visible detail transition, a seam above the existing per-step bound or a moved tolerance fails the criterion. [inferred]
- **R3:** Improve oak and spruce colour and relief readability, including restrained spatial red/brown variation among scales and exposed edges, through existing bark colour controls where possible, judged visually beside the replacement exposed-bark references and baseline under fixed render lighting. Report before/after render colour metrics without an exact photo RGB acceptance target. Owner acceptance remains R5. [user decision, 2026-09-19]
- **R4:** Record fresh baseline and candidate timings for the whole-tree native oak frame and full-screen trunk by the fn-26 protocol, and report browser orbit frame rate. Report absolute costs and before/after differences. Extra cost is allowed for this fidelity pass, with optimization deferred; the historical native timing and 60 fps are reference targets rather than pass/fail limits. Errors: an unavailable, disjoint or contended session does not count as valid performance evidence. [paraphrase]
- **R5:** The owner judges the bark-only stills and the eight fn-32 stills re-rendered, beside the references and beside fn-32's, answering whether the oak and the spruce bark read as real at the distance an eye would see them, and records the verdicts in this spec; the spec closes only on accepting verdicts. Errors: a rejecting verdict stops the spec with the owner's words and a one-paragraph blocker. [paraphrase]

## Boundaries
<!-- scope: business -->

- No image textures; every value is a row. [strategy:Surface and rendering at scale]
- Preserve the cellular network topology; its scale, profile proportions and localized edge shaping may change to address the owner's 2026-09-20 rejection. Do not replace the material with textures or geometry. [paraphrase]
- No geometry change to the skeleton or the silhouette; depth stays a shading term inside the outline. [paraphrase]
- The Two Trees and Ordinary keep their colours; only the oak and spruce presets move under R3. [inferred]
- No engine work; the surface split is fn-41's. [paraphrase]

## Decision Context
<!-- scope: both -->

### Motivation
<!-- scope: business -->

- The owner's verdict on fn-32 was acceptance with room to improve, and the choice to carry the remainder to a new spec rather than a fourth round, because the numbers said the rows had nothing left. [paraphrase]

### Performance decision, 2026-09-19

The owner allows extra cost now and optimization later. R4 therefore requires comparable measurements and disclosure of regressions rather than a timing ceiling. Visual continuity, unchanged filtering-test bounds and the owner's final visual acceptance remain required. [paraphrase]

### Reference decision, 2026-09-19

The owner chose option 1, “Continue with visual reference matching,” after the original photographs could not be recovered and replacement sources lacked physical scale and calibrated RGB. This explicitly replaces R1/R3 exact-photo calibration. The calibrated render fixture, unchanged filtering gates, performance reporting and final owner visual judgment remain required.

### Owner feedback on the first candidate, 2026-09-19

The owner says “colors look better” for spruce and oak, but expected added relief. This accepts the direction of the colour change, not R2 or final R5 completion. Keep those colour rows fixed during the depth pass; measure actual height contrast as well as rendered differences. The host resumes the unfinished spec after stopping at too narrow a worker checkpoint.

### Implementation Tradeoffs
<!-- scope: technical -->

- The original near-only proposal is superseded by the owner's later fn-71 verdict of 2026-09-18. Preserve the accepted footprint-integral behavior while improving depth; current captures, rather than the historical four-millimetre observation alone, determine the required implementation. [inferred]
- Moving fn-29's colour row over adding a level row: the level is one number the bark already has, and a second row for the same thing would fight the first in the blend. [inferred]
- A bark-only capture over correcting the cylinder in the measurement: fn-32 tried a flat-field correction and it left two of three oak references with no measurable light regions; a capture that removes the cylinder is honest where a correction is a guess. [paraphrase]

### Parallel execution

Use an isolated branch. Only oak and spruce material rows may change for colour calibration; keep the patch confined to those rows and coordinate integration with fn-68. Do not change the leaf representation being worked in fn-86 or the species pipeline. [inferred]

## Parked unknowns

- Replacement reference physical scales and colour calibration are unknown and are not acceptance claims; oak fine-detail comparison is limited by its 650 × 567 source.

## Strategy Alignment

- Follows "Surface and rendering at scale": continuous surfaces, foliage and bark that make the structure legible from close views, without species-specific paths or hand-modelled assets, judged with measured runtime cost; hierarchy and continuous rendering representations keep rendering within game budgets from a hero tree to a forest, without visible stepping, thinning or shimmer.

### Owner visual verdict, 2026-09-20

> i feel the relief highlights more clearly the structural difference between the reference and our implementation. The small scales with fine lines are quite different to our deep and more wide valleys in the relief. There's also a lack of roughness around the edges.

The stronger-relief candidate is not accepted. The owner identifies a morphology mismatch: small scales and fine separations in the reference versus broad deep valleys and smooth edges in the candidate. Increasing height contrast alone is not a success criterion for the next pass. Preserve the improved base colours; investigate shallower plate faces, narrow separations and irregular chipped edges. The current spec boundary preserving the accepted plate network must be reconciled with this structural feedback before a new implementation pass.

### Authorized continuation, 2026-09-20

The owner asks “ok so pls $flow-next-flow this to the end” after the structural correction was described. Continue the same fidelity objective with the improved colours fixed. The previous depth-only candidate and its 15%-more-height test are superseded by the explicit demand for shallower faces, fine separations and chipped edges. Preserve all existing continuity, distance and redraw tolerances. This is a refinement of the material fidelity acceptance, not authorization to waive numerical gates or invent the owner's final visual verdict.

### Material independence, implementation decision 2026-09-20

The thin chipped profile needs an explicit material value. Selecting it with the smooth-term optimization flag would alter bark shape when lichen or lenticels are enabled. The new plateEdgeShape row keeps that decision authored, preserves other presets at zero and lets the existing smooth shader optimization remain appearance-neutral. A runtime-uniform GPU probe checks profile interpolation and specialization parity at 1e-6 profile units. This is an implementation decision supporting the authorized structural correction.


### Colour refinement and deferred overlap, 2026-09-20

The owner says the revised shape is “much better,” then identifies more varied reference colouring with red/brown layered in and directional overlapping flakes. The owner asks to include colour in this spec and suggests separate scope for overlapping flakes. [user]

Include restrained red/brown spatial colour variation for oak and spruce in R3, retaining the improved overall colour balance and current shallow chipped relief. Variation should relate to the existing scales and edges rather than uniformly warming the whole trunk. Keep the colour field continuously footprint-filtered and preserve other presets. This supersedes the earlier instruction to freeze colours during structural correction; it does not authorize another structural redesign. [paraphrase]

Directional shingling, tucked-under edges and raised overlapping neighbours remain outside fn-42. They merit a separate relief-structure spec; no new spec is created by this scope decision. The favourable shape feedback is not a final R5 verdict on the forthcoming colour revision, nor a decision on the inherited hero-sweep gate. [inferred]
