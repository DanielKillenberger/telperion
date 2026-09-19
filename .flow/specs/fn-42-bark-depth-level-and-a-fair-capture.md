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

- **A bark-only capture at a known scale.** A still whose frame is filled with bark seen square-on at a recorded physical width, for the oak and the spruce, beside the same crop of each reference photograph at its own recorded or estimated scale. The structure score fn-32 built is then computed on comparable inputs, and the dark fraction stops measuring the cylinder. [paraphrase]
- **Depth under one continuous filter.** Improve the resolved relief and parallax while retaining fn-71's accepted contract: relief leaves the image only through averaging its height over the pixel footprint, with no separate amplitude fade or near/far material switch. First measure the current field and depth response in the calibrated capture. [inferred]
- **The level as a row the descent may move.** fn-29's bark base colour and the fn-14 bark colour become part of the hill-climb set, with the level constraint fn-32's round three added, so the crop mean can reach the reference band without a hand-tuned guess. [paraphrase]
- **Cost.** Extra rendering cost is allowed for this fidelity pass; optimization can follow. Record fresh baseline and candidate timings under the same conditions for the trunk close-up and the whole tree. The historical 5.2142 ms and 3.9823 ms figures are context, not acceptance limits. [paraphrase]
- **Judging.** The same order as fn-32: numbers first, the owner's eye last, four images at most per round. [paraphrase]

## API Contracts
<!-- scope: technical -->

- **Rows:** reuse the current physical bark and depth rows; do not add the older proposal's near-footprint threshold. The fn-29 bark base colour and the fn-14 bark colour are existing rows that this spec is allowed to move on the oak and spruce presets. [inferred]
- **The bark-only view:** one headless capture recipe per species with the camera distance and field of view chosen so the crop width in metres is recorded beside the still, added to the evidence recipe, not a new public view or command. [inferred]
- **The reference scale:** each catalogued reference photograph gains an estimated crop width in metres in the references catalogue, with the estimate's basis stated. [inferred]

## Edge Cases & Constraints
<!-- scope: technical -->

- **Filtering remains continuous.** Every existing distance, resolution and redraw test passes at unchanged bounds. Add close-range depth coverage without relaxing those bounds. [paraphrase]
- **Continuity through a camera walk.** The current footprint sweep remains a hard gate; no new amplitude fade or visible detail transition is permitted. [paraphrase]
- **Cost, both ways.** Measure and report the full-screen trunk and whole-tree costs by the fn-26 protocol, including any regression. No hard timing ceiling gates this fidelity pass. [paraphrase]
- **References.** The same catalogued photographs, never redistributed. [paraphrase]

## Acceptance Criteria
<!-- scope: both -->

- **R1:** A bark-only capture recipe renders the oak and the spruce trunk square-on with the crop's physical width recorded, and the structure score is computed on that still and on the reference photograph at its estimated scale, with both scales stated beside the numbers. Errors: a still or reference without a recorded scale does not count as evidence. [paraphrase]
- **R2:** Resolved furrow depth improves in the calibrated close-up while retaining one continuously footprint-filtered material. No separate amplitude fade or near/far switch is introduced; every existing distance, resolution, footprint-sweep and redraw test passes at unchanged bounds. Errors: a visible detail transition, a seam above the existing per-step bound or a moved tolerance fails the criterion. [inferred]
- **R3:** The oak trunk crop mean reaches within a stated band of its reference's 118 119 114 with the level found by the scripted hill climb over the bark colour rows, and the spruce likewise against the owner's spruce photograph. Errors: a level outside the band stops the round with the number. [paraphrase]
- **R4:** Record fresh baseline and candidate timings for the whole-tree native oak frame and full-screen trunk by the fn-26 protocol, and report browser orbit frame rate. Report absolute costs and before/after differences. Extra cost is allowed for this fidelity pass, with optimization deferred; the historical native timing and 60 fps are reference targets rather than pass/fail limits. Errors: an unavailable, disjoint or contended session does not count as valid performance evidence. [paraphrase]
- **R5:** The owner judges the bark-only stills and the eight fn-32 stills re-rendered, beside the references and beside fn-32's, answering whether the oak and the spruce bark read as real at the distance an eye would see them, and records the verdicts in this spec; the spec closes only on accepting verdicts. Errors: a rejecting verdict stops the spec with the owner's words and a one-paragraph blocker. [paraphrase]

## Boundaries
<!-- scope: business -->

- No image textures; every value is a row. [strategy:Surface and rendering at scale]
- No change to the plate network's structure; fn-32 owns it and it was accepted. [paraphrase]
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

### Implementation Tradeoffs
<!-- scope: technical -->

- The original near-only proposal is superseded by the owner's later fn-71 verdict of 2026-09-18. Preserve the accepted footprint-integral behavior while improving depth; current captures, rather than the historical four-millimetre observation alone, determine the required implementation. [inferred]
- Moving fn-29's colour row over adding a level row: the level is one number the bark already has, and a second row for the same thing would fight the first in the blend. [inferred]
- A bark-only capture over correcting the cylinder in the measurement: fn-32 tried a flat-field correction and it left two of three oak references with no measurable light regions; a capture that removes the cylinder is honest where a correction is a guess. [paraphrase]

### Parallel execution

Use an isolated branch. Only oak and spruce material rows may change for colour calibration; keep the patch confined to those rows and coordinate integration with fn-68. Do not change the leaf representation being worked in fn-86 or the species pipeline. [inferred]

## Parked unknowns

- The reference photographs' physical scale: estimated from plate sizes typical of the species until a scale is recorded for a photograph.

## Strategy Alignment

- Follows "Surface and rendering at scale": continuous surfaces, foliage and bark that make the structure legible from close views, without species-specific paths or hand-modelled assets, judged with measured runtime cost; hierarchy and continuous rendering representations keep rendering within game budgets from a hero tree to a forest, without visible stepping, thinning or shimmer.
