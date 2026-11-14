# Colour, cavity and occlusion

## Goal

The owner judged fn-26's stills "very cartoony". Relief did its job; what still read as a drawing was flat bark colour, a flat green blade with no sheen, crevices lit like crests, and an unoccluded sky. This spec was the next appearance step after fn-26: bark colour that varies with the surface, a cavity term where light cannot reach, a textured blade with a cuticle, and sky light occluded inside the crown, all as rows on the family and inside fn-14's frame budget. Geometry (blunt tips, interpenetrating limbs, fork anatomy) stayed with fn-4 and fn-20.

## Decisions

- > user (turn 1, on fn-26's eight stills): "hm it looks very cartoony. Will it get more realistic within fn-26?"
- > user (turn 2, on the proposal to capture a colour, cavity and occlusion spec with fn-4 and fn-20 queued for geometry): "ok /flow-next:capture that"
- Rows over image textures: a texture cannot be blended or walked through tree space.
- Cavity from fn-26's relief height (one multiply per fragment) over screen-space AO, which is the reserve if crevices still read flat.
- Sky occlusion from crown depth, extending fn-14's interior darkening to the wood; no new light, shadow map or vertex layout change.
- Round 4 checkpoint: the oak fissure offset was owner-approved (recorded as "round 4 checkpoint; owner-approved oak fissure offset"; source: archived checks.json).
- Round 5 (recentring the crest/fissure map on the field's face mean after a host QA finding) owner-authorised with an extended budget: "pls go ahead" (2026-09-14). The owner extended the commit budget for round 5 on 2026-09-14 (source: archived REPORT.md).
- The bark references `owner-white-oak-bark.png`, `owner-black-oak-bark.png` and `owner-norway-spruce-bark.png` were owner-supplied local references (copied from the fn-26 worktree into `.refs/fn29/`, never redistributed; source: archived owner-reference-metadata.json).

## Acceptance Criteria

- **R1:** Bark colour varies with the surface (darker cooler fissures, paler crests, low-frequency mottling), all from rows; wood mesh bytes unchanged — met.
- **R2:** Cavity darkens crevices, fork sockets and trunk base under sun and sky, derived not painted — met.
- **R3:** Blade mottling, lighter margin and specular cuticle; back face keeps its colour; needle matte by the same rows — met.
- **R4:** Sky light occluded by crown depth on wood and leaves — met.
- **R5:** Owner judges fn-26's eight stills re-rendered, beside references and fn-26's versions — met (accepted 2026-09-14).
- **R6:** Oak native total p50 at or under 3.8 ms; browser orbit 60 fps — partial: 3.9823 ms (0.1823 over), accepted by the owner; orbit holds 60 fps.
- **R7:** Every value a blendable row validated by range, no species branch in the shaders — met.

## Result

- Nineteen material rows, default 0: fissure RGB and strength, crest RGB and strength, bark mottle scale and strength, cavity strength, blade mottle scale and strength, margin width and RGB, cuticle gloss, sky occlusion strength. Oak: fissure (-.1,-.05,.005)@.65, crest (.1,.085,.055)@.5, bark mottle .8 m@.2, cavity .6, blade mottle 6@.15, margin .08, gloss .35, sky occlusion .5. Spruce: fissure @.4, crest @.3, cavity .6, matte blade, gloss .05, sky occlusion .6. Ordinary keeps `MaterialParams::default()`.
- Five rounds via one codex session; round 3 captured the stills for the owner to look first, round 4 softened the oak fissure on approval, round 5 recentred the crest/fissure map.
- Owner verdict (R5 and R6, 2026-09-14), on the round-five stills at `aa17e5e` beside fn-26's and the references, and the live orbit at `dc109e4`: "oak looks much better, spruce looks better but not much. Both lack significant details to make realistic bark textures. Textured leaf is nice." and "i think we can accept this spec." R6 accepted with the native total p50 of 3.9823 ms recorded.

## Open

The missing bark detail (plates that break and lift, per-plate identity, weathering) is the next surface spec (fn-32).

## Evidence

- `.flow/evidence/fn29/stills/` (the eight verdict stills, PNG; referenced by the oak and spruce catalogue entries)
- `.flow/evidence/fn29/stills.json`, `.flow/evidence/fn29/references.json`
