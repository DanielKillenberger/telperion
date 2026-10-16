# Bark and foliage appearance

## Goal

The owner found the trees "pretty bland" and wanted them "more impressive soon" (2026-09-10): species bark and foliage appearance from procedural material rows, surface coordinates from the core, one sun with a shadow map, a sky and ground, per-leaf colour variation inside biological ranges, interior darkening and multisampling for the crown speckle, judged lit beside reference photographs. At write-back the owner split the work: this spec is the lit tree; bark relief, veins, transmission and close-up verdicts moved to fn-26.

## Decisions

> user (interview, 2026-09-10): "atm it's looking pretty bland and i want it to be more impressive soon."
> user (interview, 2026-09-10): "can we have biology dictate the range the leaf hue and brightness can be and we have random jitter within those ranges?"
> user (interview, 2026-09-10): "is there a way we can do full translucency properly? in the budget we have in this spec? or should we aim simpler and refine transclucency more later?"
> user (interview, 2026-09-11): "one thing that might need addressed also in this spec is the noise in the renderer. Does that just come from all the same color overlapping?"
> user (interview, 2026-09-11): "we can include but a lot of the noise is actually in the center of the tree with lots of overlapping leaves atm"

- Appearance is numeric rows: a material row in the family (blended like every field) and a separate renderer-side scene row (sun, sky, ground); no species branch in any shader; no image textures.
- Core emits two coordinates per wood and leaf vertex in a separate buffer, so existing strides and pins are untouched.
- Per-leaf colour is a seeded offset keyed by placement index inside the row's hue and brightness ranges.
- One sun shadow map over wood and coarsest-level leaves, timed on its own; interior darkening by depth into the crown's inscribed ellipsoid; 4x MSAA with 1-sample fallback; filmic tone map; clay view kept.
- Budget: oak native vegetation + selection + shadow p50 at most 3.8 ms (2.30 + 1.5 ms); browser orbit p95 under 16.7 ms.
- Reference photographs stay out of git (fn-9's catalogued OSU images).

## Acceptance Criteria

- **R1:** Reference-based bark and foliage appearance with coordinates — met (translucency moved to fn-26).
- **R2:** Appearance validated at trunk, branch and leaf scales; fallbacks explicit — met at trunk and crown; close-ups in fn-26.
- **R3:** Clay inspection kept; material GPU and memory costs reported — met.
- **R4:** Every appearance value a numeric row; no species branch — met.
- **R5:** Core emits surface coordinates through every level; existing pins unchanged — met.
- **R7:** Per-leaf colour inside biology-set ranges, seeded — met.
- **R9:** One timed shadow map with self- and ground shadow — met.
- **R10:** Sky, sun and ground from the scene row; clay view selectable — met.
- **R11:** Owner verdicts on the lit oak and spruce beside fn-9's photographs — accepted, both.
- **R12:** Oak frame at or under 3.8 ms and browser orbit within bounds — missed (4.949 ms); owner accepted the number with fn-27 as the fix.
- **R14:** Interior darkening by crown depth — met.
- **R15:** 4x multisampling with fallback, timed — met.

R6, R8 and R13 moved to fn-26.

## Result

Lit oak and spruce at seed 7 (1600x1000, 4 samples, RTX 3080): oak 8,255,000 wood triangles / 869,310 leaves, spruce 5,580,040 / 7,012,326 needles. Oak native total p50 4.9487 ms against 3.8 ms; the sun's depth pass drawing full-resolution wood costs 1.813 ms of it. Browser orbit wall p95 10.10 ms, worst frame 10.20 ms.

Owner verdicts (2026-09-11):
- Oak: accept. "it reads the same as before coloring etc. or looks better even so i'd say it's clearly a pass. We do need to work more on accuracy though later. The oak we render doesn't have thick inner branches like the reference. But that's for another spec."
- Spruce: accept. "It reads fine. The brown branches comes through a bit too much behind the needles? that doesn't look quite right atm"
- R12: accept the number, 4.949 ms against 3.8 ms, for this spec. "The fix is fn-27, coarse shadow casters and a filtered shadow, which fn-26 now depends on and which runs before it."

## Open

- Oak inner-branch thickness (owner note) and spruce branches showing through needles: later specs.
- Frame budget: fn-27.

## Evidence

- `.flow/evidence/fn14/REPORT.md` (measurements, sources, checksums)
- `.flow/evidence/fn14/oregon-white-oak-hero.jpg`, `norway-spruce-hero.jpg` (verdict stills)
- `.flow/evidence/fn14/*-timing.json`, `*-orbit.json` (session records)
