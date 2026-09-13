# Colour, cavity and occlusion

## Conversation Evidence

> user (turn 1, on fn-26's eight stills): "hm it looks very cartoony. Will it get more realistic within fn-26?"
> user (turn 2, on the proposal to capture a colour, cavity and occlusion spec with fn-4 and fn-20 queued for geometry): "ok /flow-next:capture that"

## Goal & Context
<!-- scope: business -->
<!-- Goal & Context: 20% [user], 50% [paraphrase], 30% [inferred] -->

The owner looked at fn-26's stills, the oak and the spruce at trunk, branch and leaf scale with relief, veins and transmission in place, and judged them "very cartoony". [user] The relief did what fn-26 asked of it; what still reads as a drawing is everything relief cannot touch: one flat colour on the bark with no change in the fissures and no weathering on the ridges, one flat green on the blade with a hard outline and no sheen, one sun and a flat sky so a crevice is lit like a crest, and blunt geometry at tips and forks. Real bark and real leaves read as real mostly through colour variation and through light that fails to reach where surfaces meet. [paraphrase]

This spec is the next appearance step after fn-26: colour that varies with the surface, cavity that darkens where light cannot reach, and sky light occluded inside the crown, all as rows on the family and all inside fn-14's frame budget. The geometry half of the same judgment, blunt tips, interpenetrating limbs and the fork anatomy, belongs to fn-4 and fn-20, which are open and queued behind it. [paraphrase]

## Architecture & Data Models
<!-- scope: technical -->

- **Bark colour from the relief and along the run.** fn-26 already computes a relief height per fragment from the distance and angle coordinates. This spec reads it: fissures take a darker, cooler tint and ridge crests a paler, drier one, by two colour offsets and a strength on the row, and a low-frequency mottling along the run breaks the base colour so no two metres of trunk are the same. Every term is a numeric row the blend walks; there is no image and no species branch. [inferred]
- **Cavity.** A cavity term, from the relief height's local minimum and from wood meeting wood at forks and at the trunk base, scales both the sun and the sky contribution down in crevices, so a fissure is dark because light does not reach it, not because it was painted dark. [inferred]
- **The blade.** Mottling across the blade at a scale and strength from the row, a lighter margin band, and a specular cuticle as a gloss value with the sun's highlight shaped by the blade normal; the back face keeps its own colour. The needle takes the same rows and reads matte and near-uniform because its row says so. [inferred]
- **Sky occlusion.** The sky's contribution to wood and leaves is reduced by how deep the fragment sits inside the crown, extending the interior darkening fn-14 applies per leaf to the wood under the canopy and to the underside of limbs, from the crown envelope the generator already knows. No new light, no new shadow map, no vertex layout change; a term in the existing lighting. [inferred]
- **The judgment surface.** fn-26's eight stills, re-rendered with the same seed, sun and camera, so the owner sees exactly what colour, cavity and occlusion changed. [paraphrase]

## API Contracts
<!-- scope: technical -->

- **Material row additions**, validated by range naming the field: fissure tint (three components) and strength, crest tint and strength, bark mottle scale and strength, cavity strength, blade mottle scale and strength, margin width and tint, cuticle gloss, sky occlusion strength. On the wire, in the generated browser metadata and in the sweep's inventory the way fn-14's and fn-26's rows are. [inferred]
- **Views and commands** unchanged; the headless target renders the same eight stills through the existing views and camera. [inferred]

## Edge Cases & Constraints
<!-- scope: technical -->

- **The silhouette is untouched.** Every term shades; none displaces a vertex; the fn-24 pins and the wood mesh bytes are unchanged with the rows on and off. [paraphrase]
- **Levels.** Blade terms read the same at every leaf level or the crown shimmers when levels switch. [inferred]
- **Determinism.** The look test's redraw check holds: one tree under one row is one picture, and no new term may widen its tolerance. [paraphrase]
- **Budget.** The added terms stay inside fn-14's 3.8 ms native bound on the oak and the browser orbit's 60 fps, recorded beside fn-26's numbers. [paraphrase]
- **References.** The same four catalogued references fn-26 judges against, O-BARE and O-LEAF for the oak, S-BRANCH and S-NEEDLE for the spruce, re-fetched into the ignored references directory and never redistributed. [paraphrase]

## Acceptance Criteria
<!-- scope: both -->

- **R1:** Bark colour varies with the surface: fissures darker and cooler, crests paler, and a low-frequency mottling along the run, each from a row and none from an image; the wood mesh bytes are identical with the rows on and off. [paraphrase] Errors: no error surface beyond row validation.
- **R2:** A cavity term darkens crevices, fork sockets and the trunk base under both sun and sky light, derived from the relief height and from wood meeting wood, never painted. [inferred] Errors: no error surface beyond row validation.
- **R3:** The blade carries mottling, a lighter margin and a specular cuticle from the row, with the back face keeping its own colour; the needle reads matte and near-uniform by the same rows. [inferred] Errors: no error surface beyond row validation.
- **R4:** Sky light is occluded by crown depth on wood and on leaves, so limbs under the canopy and the underside of branches read darker than the crown's edge, with no new light and no change to the shadow map. [inferred] Errors: no error surface beyond row validation.
- **R5:** The owner judges the same eight stills fn-26 rendered, re-rendered with these terms, beside the catalogued references and beside fn-26's versions, answering whether the oak and the spruce still read as a drawing, and records the verdicts in this spec; the spec closes only on accepting verdicts. [paraphrase] Errors: a rejecting verdict stops the spec with the owner's words and a one-paragraph blocker.
- **R6:** The oak's native frame with every term on stays at or under 3.8 ms total p50 and the browser orbit holds 60 fps, recorded beside fn-26's numbers. [paraphrase] Errors: an unavailable, disjoint or contended session does not count; a number over the bound stops the spec with the number.
- **R7:** Every value is a row the blend walks, validated by range naming the field, with no species or preset branch in the shaders. [strategy:Surface and rendering at scale] Errors: a field outside its range is refused naming it.

## Boundaries
<!-- scope: business -->

- No image textures; every value is a row. [paraphrase]
- No geometry change: blunt tips and interpenetrating limbs are fn-4's, the root collar and fork anatomy fn-20's, the leaf outline fn-24's. [paraphrase]
- No new light and no new shadow map; sky occlusion is a term in fn-14's lighting. [inferred]
- No lichen, moss or damage as things of their own; colour mottling may suggest them, nothing more. [inferred]
- No seasons and no lifecycle appearance; those stay with their own specs. [paraphrase]

## Decision Context
<!-- scope: both -->

### Motivation

- The owner's verdict on fn-26's stills was that they look very cartoony, and the question was whether fn-26 would fix that. It cannot: its scope is relief, veins and transmission with no colour and no change to the light, and what reads as a drawing is flat colour, unlit crevices and unoccluded sky. This spec is that answer. [paraphrase]

### Implementation Tradeoffs

- Rows over image textures: a texture would make the bark real faster, and the strategy forbids it for a reason, since a texture cannot be blended between two families or walked along the tree space; procedural colour driven by the relief height fn-26 already computes costs almost nothing and blends like every other trait. [paraphrase]
- Cavity from the relief height over screen-space ambient occlusion: the relief already knows where its own fissures are, so cavity is one multiply per fragment and needs no extra pass; a true screen-space term would cost a pass fn-14's budget cannot spare and is the reserve if crevices still read flat. [inferred]
- Sky occlusion from crown depth over a baked or ray-traced term: crown depth is what the generator already computes for interior darkening, so extending it to the wood is free; a baked per-vertex term would need a vertex layout change and is out of scope by fn-14's boundary. [inferred]

## Parked unknowns

- Whether the relief-derived cavity is enough at fork sockets, where the fissure pattern of two runs meets, or whether a geometric term for wood meeting wood is needed there; the branch-scale still decides it.
- Which preset values read best: the oak's and spruce's rows are set against the references by the implementer and judged by the owner, and the other three presets take values that read as their kind.

## Strategy Alignment

- Follows "Surface and rendering at scale": continuous surfaces, foliage and bark that make the structure legible from close views, without species-specific paths or hand-modelled assets, judged with measured runtime cost.
