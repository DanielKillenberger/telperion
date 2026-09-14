# Bark as plates and scales

## Conversation Evidence

> user (turn 1, on fn-29's round-four oak trunk): "why is the oak blue? lol"
> user (turn 2, after the round-five stills): "i mean i'm colorblind but still like it's blue to me. Maybe we do a QA pass and compare against references"
> user (turn 3, judging the round-five stills beside fn-26's and the references): "oak looks much better, spruce looks better but not much. Both lack significant details to make realistic bark textures. Textured leaf is nice."
> user (turn 4): "i think we can accept this spec. But do we have a path to procedural realistic looking bark?"
> user (turn 5, on the six-point path the host proposed): "aye /flow-next:capture it"

## Goal & Context
<!-- scope: business -->
<!-- Goal & Context: 30% [user], 40% [paraphrase], 30% [inferred] -->

The owner accepted fn-29, colour, cavity and occlusion, with this verdict on its stills: "oak looks much better, spruce looks better but not much. Both lack significant details to make realistic bark textures. Textured leaf is nice." [user] The leaf is done for now. The bark is not, and the owner asked whether a path to procedural realistic bark exists. [paraphrase]

It does, and it is mostly one missing primitive. The bark field today is parallel ridges with axial breaks and flakes, and fn-29 made colour follow that field. What real bark has and the field cannot make is structure: a network of plates and furrows that branch and merge on the oak, and small round scales that lift at the edge on the spruce, each plate or scale with its own identity in tilt, lift and hue, weathered on its face and fresh in its furrow. That structure is what the eye reads as bark before colour or relief. [paraphrase] The judgment that ended fn-29 also set the method for this spec: measure the stills against the reference photographs first, by channel order, level and structure, so the implementer iterates against numbers and the owner's eye is spent on the last round. [paraphrase]

## Architecture & Data Models
<!-- scope: technical -->
<!-- Architecture: 20% [paraphrase], 80% [inferred] -->

- **A plate network as the field's second primitive.** A cellular field over the arc and along coordinates the bark shader already has, anisotropic so cells stretch along the run, with cell size scaling with girth so old trunks carry big plates and young wood small ones. Furrow depth comes from distance to the cell edge, the plate face from the cell interior with a slight dome, and each cell hashes to an identity that sets its tilt, its edge lift and its hue and value jitter. The oak reads as blocky plates between furrows and the spruce as small round scales lifting at the edge, by values on the same rows. It is footprint-faded the way the existing ridges are, so distant trunks converge to the mean and cost scales with how much trunk fills the screen. [inferred]
- **Depth beyond a tilted normal.** Relief today perturbs shading only, so the silhouette and the self-shadowing stay those of a smooth cylinder. Two candidates, chosen by measurement on the eight-tree forest before either is built out: parallax on the height field, a per-fragment march that leaves geometry alone, or vertex displacement on the near wood level, which changes the silhouette and touches the fn-24 pins. Whichever is chosen sits behind a row and is off by default, so the wood mesh bytes are identical with the row off. [inferred]
- **Directional occlusion from the height field.** Sample the height toward the sun so a furrow floor is shadowed by its crest on the shade side and lit on the sun side. That asymmetry is what the eye reads as depth; the fn-29 cavity term darkens both sides alike and cannot give it. [inferred]
- **Colour by structure.** Per-plate hue and value jitter from the cell identity, paler and greyer weathered plate faces against fresher furrows, and an orientation term for the shade side and the base that mottling can suggest as algae without modelling it, all as rows the blend walks. [inferred]
- **Measurement before eyes.** The check that caught fn-29's blue oak in one measurement, a centre-crop mean and channel order against the reference photograph, becomes the acceptance instrument, extended with a structure statistic per still: furrow spacing in pixels and dark-pixel fraction. The implementer iterates against those numbers; the owner judges only the final round. [paraphrase]

## API Contracts
<!-- scope: technical -->

- **Material row additions**, validated by range naming the field, on the wire, in the generated browser metadata and in the sweep's inventory the way fn-14's, fn-26's and fn-29's rows are: plate scale, plate elongation, plate dome, edge lift, plate identity strength (tilt, lift, hue and value jitter), weathering strength and tint, orientation strength and tint, directional occlusion strength, and the depth term's strength. Every default is inert so an existing document renders exactly as before. [inferred]
- **Views and commands unchanged**; the headless target renders the same eight stills through the existing views and cameras. [inferred]
- **The measurement receipt.** Each still records, beside its checksum, the centre-crop mean RGB, the channel order, and the structure statistics for the still and for its catalogued reference, so a reader can see the numbers the round was judged on without opening an image. [paraphrase]

## Edge Cases & Constraints
<!-- scope: technical -->

- **The silhouette is untouched with the depth row off.** The fn-24 pins and the wood mesh bytes are identical with every new row off. With the depth row on, the chosen mechanism's effect on the pins is measured and recorded, and a pin that legitimately moves is re-pinned with the reason stated, never widened. [inferred]
- **Determinism.** One tree under one row is one picture; the look test's redraw check holds and no new term widens its tolerance. [paraphrase]
- **Distance.** The plate field is footprint-faded on both axes like the ridges, and every colour term stays affine in the filtered height per the fn-29 rule, so the distance and resolution tests hold at their unchanged bounds. [inferred]
- **Budget.** The oak's native frame with every fn-29 term on measures 3.9823 ms total p50, accepted by the owner over the 3.8 ms bound. The bound for this spec is the owner's to set before the still round; a cellular field costs about nine hash lookups per fragment, so the plate path is expected to need either a moved bound or a near-only path. [paraphrase]
- **References.** The same catalogued references, the owner's white oak and Norway spruce bark photographs and O-BARE, S-BRANCH, re-fetched into the ignored references directory and never redistributed. [paraphrase]

## Acceptance Criteria
<!-- scope: both -->

- **R1:** The bark field carries a plate network: an anisotropic, girth-scaled cellular primitive over the arc and along coordinates with per-cell identity, from which the oak reads as plates between furrows and the spruce as scales lifting at the edge, by row values with no species branch and no image. Errors: no error surface beyond row validation. [inferred]
- **R2:** Bark shows depth beyond the shaded normal through one mechanism, parallax on the height field or near-level vertex displacement, selected by a measured comparison on the eight-tree forest recorded in the evidence; the wood mesh bytes and the fn-24 pins are identical with its row off. Errors: a pin that moves with the row on is re-pinned with the reason stated; a mismatch with the row off fails the criterion. [inferred]
- **R3:** A directional occlusion term darkens a furrow floor on its shade side and leaves it lit on its sun side, derived from the height field toward the sun, with no new light and no shadow-map change. Errors: no error surface beyond row validation. [inferred]
- **R4:** Plate faces and furrows differ in colour by structure: per-plate hue and value jitter, weathered faces against fresh furrows, and an orientation term for the shade side and the base, each a row the blend walks. Errors: no error surface beyond row validation. [inferred]
- **R5:** Every still is judged first by measurement against its reference photograph, centre-crop mean, channel order and the structure statistics, with the numbers recorded beside the still's checksum, and a round is sent back on a number before anyone views an image. Errors: a still whose measurements are missing does not count as evidence. [paraphrase]
- **R6:** The owner judges the same eight stills fn-26 and fn-29 rendered, re-rendered with these terms, beside the catalogued references and beside fn-29's versions, answering whether the oak and the spruce bark now read as real, and records the verdicts in this spec; the spec closes only on accepting verdicts. Errors: a rejecting verdict stops the spec with the owner's words and a one-paragraph blocker. [paraphrase]
- **R7:** The oak's native frame with every term on is measured by the fn-26 protocol and recorded beside fn-29's 3.9823 ms, and stays at or under the bound the owner sets before the still round; the browser orbit holds 60 fps. Errors: an unavailable, disjoint or contended session does not count; a number over the bound stops the spec with the number. [paraphrase]

## Boundaries
<!-- scope: business -->

- No image textures; every value is a row. [strategy:Surface and rendering at scale]
- No change to the skeleton: blunt tips and interpenetrating limbs stay fn-4's, the root collar and fork anatomy fn-20's. Vertex displacement under R2 moves surface vertices along their normals only. [paraphrase]
- No lichen, moss or damage as things of their own; the orientation term may suggest them, nothing more. [paraphrase]
- No seasons and no lifecycle appearance. [paraphrase]
- No redo of fn-29's colour, cavity and occlusion rows; this spec adds structure on top of them and depends on fn-29 having landed. [paraphrase]
- The leaf is out of scope; the owner judged it good. [paraphrase]

## Decision Context
<!-- scope: both -->

### Motivation
<!-- scope: business -->

- The owner's verdict on fn-29 set the target: "Both lack significant details to make realistic bark textures." Realistic bark is the outcome; the leaf is done. [user]
- The owner asked for the QA pass against references after seeing a hue the numbers caught before the eye did, so measurement is the first judge in this spec and the owner's eye the last. [paraphrase]

### Implementation Tradeoffs
<!-- scope: technical -->

- A cellular plate network over extending the ridge field: ridges cannot branch or merge and cannot give a plate an identity, and every attempt to fake plates with axial breaks reads as parallel lines from two metres. A cellular field costs more per fragment and is the honest primitive. [inferred]
- Two depth candidates measured rather than one chosen: parallax leaves geometry alone but marches per fragment inside a budget fn-29 already spent; displacement is cheap per fragment but changes the silhouette the pins guard. The eight-tree forest decides it with numbers. [inferred]
- Directional occlusion over screen-space ambient occlusion: the height field is already in hand at the fragment, so a few samples toward the sun cost far less than a pass, and give the sun-side and shade-side asymmetry a screen-space term would not. [inferred]
- Measurement-first judging over stills-first judging: fn-29 spent two still rounds on a hue a one-line crop measurement identified, and the owner is colourblind and said so; numbers reach the implementer without a human in the loop and preserve the owner's eye for the question only it can answer. [paraphrase]

## Parked unknowns

- The native bound for this spec: the owner accepted 3.9823 ms for fn-29 against 3.8 ms; whether the bound moves for the plate terms, or the plate path is limited to near footprints, is the owner's call before the still round.
- Which depth mechanism wins, parallax or displacement; the eight-tree forest measurement under R2 decides it.
- Which structure statistics separate real bark from the current field reliably across the four references; the first measurement round establishes them before they gate anything.

## Strategy Alignment

- Follows "Surface and rendering at scale": continuous surfaces, foliage and bark that make the structure legible from close views, without species-specific paths or hand-modelled assets, judged with measured runtime cost.
- Follows the approach's measured-evidence rule: judge each advance through measured runtime costs and visual evidence.

## Owner verdict (R6, round one, 2026-09-14)

Judged on the capture-three stills at `bc38f63` beside fn-29's. In the owner's words: "i see some issues in how organic it looks. Looks like armor plating in some cases. Spruce is much better. But also doesn't look too organic." Not yet accepting. The owner directed that acceptance be reached within this spec by a measured hill climb rather than a rejecting stop: "wouldn't we want to achieve acceptance within this spec?" R7's bound stays the owner's call; the measured 4.5192 ms p50 stands as the number to beat or accept.
