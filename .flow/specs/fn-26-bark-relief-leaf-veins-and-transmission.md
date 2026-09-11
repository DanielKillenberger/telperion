# Bark relief, leaf veins and transmission

## Conversation Evidence

> user (interview, 2026-09-10): "atm it's looking pretty bland and i want it to be more impressive soon."
> user (interview, 2026-09-10): "is there a way we can do full translucency properly? in the budget we have in this spec? or should we aim simpler and refine transclucency more later?"
> user (interview, 2026-09-11): "looks like a massive spec with many reqs. Should we split it? or can we make it leaner?"

Split out of fn-14 at the interview's write-back on 2026-09-11. fn-14 delivers the lit tree: rows, coordinates, sun and shadow map, per-leaf colour, sky and ground, interior darkening and multisampling. This spec adds the surface detail that shows up close, drawn along the coordinates fn-14 emits and lit by the shadow map fn-14 builds.

## Goal & Context

Give the lit tree the surface detail a close view needs: bark ridges and plates, leaf veins and margin tone, and light through the blade, so the oak and the spruce hold up beside their photographs at trunk, branch and leaf scale. [paraphrase]

## Architecture & Data Models

- **Bark in the shader from the coordinates.** Ridges and plates are a procedural pattern along the distance and angle coordinates fn-14 emits, perturbing the shading normal; the silhouette is the generator's, so the fn-24 pins and the level ladder are untouched. [paraphrase]
- **Veins and margin.** The blade shader draws veins and margin tone from the along and across coordinates; at section roundness 1 the same code draws nothing, so the needle stays plain. [paraphrase]
- **Two-sided transmission.** Sun through the blade toward the eye, scaled by strength and thickness, tinted by the transmission colour, gated by fn-14's shadow map so a leaf deep in the crown does not glow. [user]
- **Rows.** Ridge scale, plate scale, vein scale and contrast, transmission strength, tint and thickness join the material row fn-14 defines; the blend walks them like any other value. [paraphrase]

## API Contracts

- **Material row additions.** Ridge scale, plate scale, roughness detail, vein scale, vein contrast, transmission strength, transmission tint, thickness; validated by range naming the field. [paraphrase]
- **Views and commands** unchanged from fn-14; the headless target renders the close-up stills at the three scales through the existing views and camera. [inferred]

## Edge Cases & Constraints

- **Seams.** The angle coordinate wraps and the shader treats it as periodic. At forks the plait's two axes meet: the pattern is continuous along each axis and may mismatch across the socket, which is exposed in the branch-scale still and either accepted or handled, never hidden. [paraphrase]
- **Levels.** Vein and transmission terms read the same at every leaf level or the crown shimmers when levels switch. [inferred]
- **The needle.** Transmission near zero and no veins at section roundness 1; the same shader, driven by the row. [paraphrase]
- **Budget.** The added terms stay inside fn-14's 3.8 ms native bound and the browser orbit's 60 fps; the shadow map is fn-14's and is not re-timed here. [user]
- **References.** fn-9's manifest supplies the close-up photographs, re-fetched into the ignored references directory and never redistributed; the report names each source and checksum. [paraphrase]

## Acceptance Criteria

- **R1:** Bark ridges and plates are a procedural pattern along the coordinates that perturbs the shading normal only; the silhouette is byte-identical to fn-24's. [user] Errors: no error surface beyond the row validation.
- **R2:** Leaf veins and margin tone are drawn procedurally from the blade coordinates, present on the oak blade and absent at section roundness 1. [paraphrase] Errors: no error surface beyond the row validation.
- **R3:** Leaves carry two-sided transmission: sun through the blade toward the eye, scaled by the row's strength and thickness, tinted by its transmission colour and gated by the shadow map; the needle row reads near zero. [user] Errors: no error surface beyond the row validation.
- **R4:** The owner judges the oak and spruce at trunk, branch and leaf scale beside fn-9's catalogued references, O-BARE and O-LEAF for the oak, S-BRANCH and S-NEEDLE for the spruce, with fork seams exposed in the branch-scale still, and records the verdicts in this spec; the spec closes only on accepting verdicts. [user] Errors: a rejecting verdict stops the spec with the owner's words and a one-paragraph blocker.
- **R5:** The oak's native frame with the added terms stays at or under fn-14's 3.8 ms bound and the browser orbit holds 60 fps, recorded beside fn-14's numbers. [user] Errors: an unavailable, disjoint or contended session does not count; a number over the bound stops the spec with the number.

## Boundaries

- No new light, no shadow change, no vertex layout change; those are fn-14's. [paraphrase]
- No physically based subsurface scattering; two-sided transmission is the model. [paraphrase]
- No image textures; every value is a row. [paraphrase]
- Supernatural effects, seasons and lifecycle stay separate work. [paraphrase]

## Decision Context

- Depends on fn-14: the coordinates, the material row, the sun and the shadow map all come from it.
- Two-sided transmission over a single strength number or physically based scattering: three row values and the shadow map give the glow a crown has against the light inside the budget; multi-bounce scattering is invisible at crown scale. [paraphrase]
- Normal perturbation over vertex displacement: the silhouette stays the generator's and the fn-24 pins are untouched. [paraphrase]

## Requirement coverage

| Requirement | Task |
|---|---|
| R1–R5 | TBD during planning |
