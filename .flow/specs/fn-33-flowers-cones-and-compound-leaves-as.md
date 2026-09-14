# Flowers, cones and compound leaves as foliage organs

## Conversation Evidence

> user (2026-09-14, on fn-10's refine, after choosing blossom and root reach as new terms): "it seems like we need at least 2 new specs before we start this? we should have roots and flowers also for natural trees. So generator has a gap there."
> user (2026-09-14): "ok so then i'd agree we capture a spec for flowers separately. Leave luminosity for later"
> user (2026-09-14, on where ash's pinnate leaves live): "In the flowers spec"
> user (2026-09-14, on the organ model): "One reproductive organ slot"
> user (2026-09-14, on seasons): "Sub-annual window"
> user (2026-09-14, on the proof set): "Spruce cones, Ash compound leaves, Oak catkins"

## Goal & Context
<!-- scope: business -->
<!-- Goal & Context: 35% [user], 45% [paraphrase], 20% [inferred] -->

Real trees carry more than leaves. An oak hangs catkins in spring, a spruce hangs cones for a year or more, and an ash holds its leaflets in pairs along a stalk. The generator today grows one leaf element per family, places one element per station, and shows it in whole-year cohorts, so none of those organs can exist on a natural tree, and the legendary trees fn-10 wants (a white tree in blossom, a world ash) have nothing to build on. The owner named the gap while refining fn-10 and asked for it to be closed for natural trees first. [user]

This spec adds two capabilities to the shared foliage layer and proves each on a real species against references. The first is one reproductive organ slot per family, a numeric row set that describes a flower, a cone or a fruit by shape, colour, station rule and season, drawn beside the leaf and inert on every ordinary tree. The second is grouping on the leaf itself, so a placement can carry several leaflets along a rachis and an ash leaf reads as one compound leaf rather than a scatter of blades. The proof set is spruce cones, oak catkins and ash compound leaves. [paraphrase]

Every value is a row the blend walks. No organ kind is an enum, no species branch enters the generator or the renderer, and a family whose organ rows sit at their neutral values produces byte-identical wood, leaves and placements to today. [strategy:The supernatural field]

## Architecture & Data Models
<!-- scope: technical -->

- **One organ slot on the family.** `Family` gains an `organ` row set beside `element` and `canopy`. Its rows are the element shape rows the leaf already has (length, width, widest point, fullness, tip, cup, curl, section roundness, segment counts), a station rule (shoot radius, spacing, hang as a signed rise, presence as the fraction of eligible stations that carry an organ, a maturity age in years below which a shoot carries none, and its own instance budget), a colour pair with a gloss value, and a season (an opening point as a fraction of the year and a duration in years, fractional). Presence at zero is the neutral value and means no organ exists. A cone, a catkin and a blossom differ only in those numbers. [paraphrase]
- **A second foliage draw, not a per-instance kind.** The organ is a second `mesh::Foliage` (its own element, its own placement list, its own level ladder from the existing ladder rule) submitted through a second `Select` and drawn by the same foliage shader with a second colour uniform pair. The crown pass keeps no per-instance attribute beyond the matrix, as `render/src/foliage.rs` states today. The sun's depth pass draws the organ through the same coarsened caster path the leaf uses. [inferred]
- **Season is a rule over shoot age, never a stamp.** Leaf cohorts are already a function of the shoot's age and the lifetime row rather than chronicle stamps. The organ follows the same shape one level finer. For a shoot born in year b with the organ opening at fraction o and lasting d years, the organ on that shoot exists for ages in [y + o, y + o + d) for every whole year y at or after b plus the maturity age. The growth clock's split of an age into year and remainder supplies the fractional part. The chronicle gains no record and fn-11's byte-identity of a filtered age holds. [inferred]
- **Compound leaves are grouping at the placement.** The leaf row set gains leaflet count (neutral 1), rachis length, leaflet pitch and a terminal-leaflet flag carried as a number in 0 to 1. A placement whose leaflet count is above one expands into that many instance matrices along a rachis line from the station, alternating sides at the pitch, with the terminal leaflet on the line's end when the flag is set. The placement identity gains a leaflet index so instances stay deterministic and the chronicle keys stay unique. The element stays a single blade, so the level ladder's base, widest and tip anchors are untouched. [inferred]
- **The rachis is drawn, not implied.** Two candidates are stated and the implementer picks after measuring on the oak and the ash fixture: the leaflets' existing woody connector extended to meet a shared rachis line, or one slender instance of the leaf element's connector geometry scaled along the rachis. Either way the rachis costs no new buffer layout. [inferred]
- **Counting stays honest.** A leaf is one placement, a leaflet is one instance, and an organ is one instance in its own list. The species metrics report placements, instances and units per instance separately for the leaf and for the organ, in line with fn-9's rule that a cluster instance is not one needle. [paraphrase]
- **Blend and wire.** Every new row joins the `fields!` list, the `walk!` list, the wasm metadata and the generated browser catalogue the way fn-14's, fn-26's and fn-29's rows did. Presence blends linearly so a walk from an organ-less family to a coned one ramps up from nothing, and leaflet count blends as a count. [paraphrase]

## API Contracts
<!-- scope: technical -->

- **Family rows added**, validated by range naming the field: `organ.*` (shape rows as on `element`, `shoot_radius`, `spacing`, `hang`, `presence`, `maturity_years`, `max_instances`, `front_red/green/blue`, `back_red/green/blue`, `gloss`, `open`, `duration_years`) and on the leaf `element.leaflet_count`, `element.rachis_length`, `element.leaflet_pitch`, `element.terminal_leaflet`. On the wire, in the wasm metadata, in the generated browser catalogue and in the sweep's held list or its moved list. [inferred]
- **Mesh output.** `Tree` and the specimen path expose a second optional foliage (element plus instances) that is absent when presence is zero, so a consumer that asked only for wood and leaves pays nothing. The browser binding exposes it the same way it exposes the leaf. [inferred]
- **Species metrics.** `foliage_units`, `units_per_instance` and a new `organ_instances` and `organ_units` land in the measurement JSON with the same status envelope as every other value. [inferred]
- **Views and commands unchanged.** The headless target renders the same views; the leaf view shows the organ where a station carries one. [paraphrase]

## Edge Cases & Constraints
<!-- scope: technical -->

- **Neutral is inert.** Presence at zero and leaflet count at one produce wood, leaf element, placement and level bytes identical to today's on every shipped preset, asserted by the identity pins before any proof row is set. When a proof row is set on the oak or the spruce, the pins move and are re-pinned once with the reason stated. [paraphrase]
- **The season wraps.** An opening point plus duration that crosses the year boundary (a cone opening in autumn and lasting fifteen months) is valid and is tested. A duration of zero means no organ, the same as presence zero. [inferred]
- **The budget is per list.** The organ's instance budget is its own row and its own cap in the station reserve and in the visible-cohort count, so a cone-heavy spruce cannot starve its needles. Leaflet expansion counts against the leaf's budget as instances. [inferred]
- **No enum, no branch.** The organ kind is never a Rust enum, a shader branch or a preset condition. The one existing kind-shaped read, the vein suppression on a round section in `foliage.wgsl`, stays as it is and applies to the organ through its own roundness row. [strategy:Surface and rendering at scale]
- **Levels.** The organ element gets its own level ladder from the existing rule, so distance selection treats it as it treats the leaf and the crown does not shimmer when levels switch. [paraphrase]
- **Determinism.** Same seed, parameters and age give byte-identical organ placements and leaflet expansions; the leaflet index is part of the placement identity. [paraphrase]
- **Cost.** The oak's native frame with catkins on, the spruce's with cones on, and the ash fixture's with leaflets on are measured by the fn-26 protocol and recorded beside fn-29's 3.9823 ms. The bound is the owner's to set before the still round; the second draw is expected to cost in proportion to its instance count. [paraphrase]
- **fn-28 is downstream.** The presentation blend sprouts and fades organs the way it sprouts and fades leaves once fn-28 lands; this spec changes nothing in the blend and records the organ's open and close points so fn-28 can read them. [inferred]
- **Growth specs in flight.** fn-30 and fn-31 touch growth traits and presets; this spec touches the foliage layer and the family rows and rebases over them on landing. [inferred]
- **References.** Catalogued reference photographs for Norway spruce cones, Oregon white oak catkins and European ash leaves, fetched into the ignored references directory under this spec's id and never redistributed, with the fn-19 reference record for each. [paraphrase]

## Acceptance Criteria
<!-- scope: both -->

- **R1:** A family carries one reproductive organ slot as numeric rows (shape, station rule, colour pair, season) with presence as its neutral zero, drawn as a second foliage pass beside the leaf; a family at neutral produces byte-identical wood, leaf, placement and level output to today on every shipped preset. Errors: a row outside its range is refused naming the field; presence zero or duration zero yields no organ list and no second draw. [user]
- **R2:** The organ exists on a shoot only inside its season window, a rule over shoot age, opening point and duration with sub-annual resolution, and never through a chronicle stamp; the window may wrap the year boundary. Errors: a window that would place an organ on a shoot younger than the maturity age yields none; the tree at a whole-year age stays byte-identical to fn-11's tree at that age with organs off. [user]
- **R3:** The leaf row set carries leaflet count, rachis length, leaflet pitch and a terminal-leaflet flag; a placement expands into leaflets along a drawn rachis with a deterministic per-leaflet identity, the element stays one blade, and the level ladder is unchanged. Errors: leaflet count one is byte-identical to today; an expansion that would exceed the leaf's instance budget truncates at the budget and reports it. [user]
- **R4:** Spruce cones on the Norway spruce preset, oak catkins on the Oregon white oak preset and ash compound leaves on an ash fixture family are set against catalogued references and judged on the existing seed protocol: numeric counts and units per instance recorded for each, and the owner judges one seed per proof in the whole and leaf views beside the references, six images, recording the verdicts in this spec; the spec closes only on accepting verdicts. Errors: a rejecting verdict stops the spec with the owner's words and a one-paragraph blocker. [paraphrase]
- **R5:** Every new row is on the wire, in the wasm metadata, in the generated browser catalogue and blended by the walk, with presence ramping linearly and leaflet count as a count; no species, template or organ-kind branch exists in generator, renderer or shaders. Errors: an unknown key is refused naming it. [strategy:The supernatural field]
- **R6:** Frame cost with each proof's organ on is measured by the fn-26 protocol and recorded beside fn-29's number, and stays at or under the bound the owner sets before the still round. Errors: an unavailable, disjoint or contended session does not count; a number over the bound stops the spec with the number. [paraphrase]
- **R7:** The automated tests cover: neutral byte identity on every preset; season windows that wrap the year; the maturity age; per-list budgets; leaflet expansion count, identity determinism and truncation; ladder validity for the organ element; a blend walk from presence zero to a coned family; and the species metrics' organ and leaflet counts. Errors: a missing case is a review finding, not implementer discretion. [inferred]

## Boundaries
<!-- scope: business -->

- One organ slot, not a list; a tree that bears both catkins and acorns is a later widening. [user]
- No fruit on the oak in this spec; acorns wait for a second slot. [paraphrase]
- No showy blossom on a natural species; the White Tree's blossom is fn-10's, built from the slot this spec ships. [paraphrase]
- No pollen, seed dispersal, flowering physiology or environmental trigger; the season is authored rows, and fn-16 owns response to surroundings. [inferred]
- No presentation blend changes; fn-28 sprouts and fades organs when it lands. [inferred]
- No emission, materials beyond a colour pair and gloss, or bark change. [paraphrase]

## Decision Context
<!-- scope: both -->

### Motivation

- The owner found the gap while choosing supernatural terms for fn-10 and said flowers belong to natural trees first, so the legends inherit an honest layer rather than a special case. [user]
- Ash pinnate leaves surfaced in the same refine as anatomy the generator cannot express; the owner placed them here because leaflets, cones and catkins are all organs on the same layer with the same station and cohort rules. [paraphrase]

### Implementation Tradeoffs

- One slot over an open list: a list does not blend, does not fit the flat wire model, and would be the first family field that is not a number. One slot proves the mechanism and the second slot, if ever wanted, is a copy of the rows. [paraphrase]
- A second draw over a per-instance kind: the crown pass has never carried a per-instance attribute beyond the matrix and the strategy forbids topology assumptions in detail selection; two lists with two ladders keep that. [inferred]
- Grouping at the placement over leaflets baked into one element: a multi-blade element breaks the level ladder's base, widest and tip anchors and would count as one unit; expanding at the placement keeps the element single and the counting honest. [inferred]
- Sub-annual season over the annual cohort rule: the annual rule would leave flowers on all year, which is wrong for blossom; the clock already carries the year's remainder, so the finer rule costs one comparison. [paraphrase]

## Parked unknowns

- How the rachis is drawn, the extended connector or a scaled connector instance, resolved by the first measurement on the ash fixture.
- Whether the organ needs its own hue and brightness variation rows or reads well with a single colour pair; the spruce cone still decides it.

## Strategy Alignment

- Follows "Surface and rendering at scale": foliage that makes the structure legible from close views, without species-specific paths or hand-modelled clusters, judged with measured cost.
- Follows "Growth and botanical fidelity": organs are judged against real species references on the fn-9 protocol before any legend uses them.
- Follows "The supernatural field" and the approach's continuous-tree-space rule: every organ trait is a numeric row that blends and no family field is a switch.

## Resolved via Codebase

- The build path has no element-kind enum; `FoliageUnit { Leaf, Needle }` is derived from `section_roundness >= 0.5` for measurement only (`crates/telperion-core/src/foliage/element.rs:4-8`, `:233-244`).
- One element per tree and one placement list: `mesh::Foliage { element, instances }` (`crates/telperion-core/src/mesh.rs:18-23`); `PlacementIdentity { shoot, station }` has no sub-index (`crates/telperion-core/src/foliage/timeline.rs:20-23`); the renderer uploads one buffer set (`crates/telperion-render/src/foliage.rs:159-189`) and `Select` is wired to one element (`crates/telperion-render/src/select.rs:87-104`, `:145-250`).
- No grouping concept anywhere; `clump` is extra independent stations (`crates/telperion-core/src/foliage/station.rs:139-141`); spruce places one needle per station with no fascicle (`crates/telperion-core/src/presets.rs:170-172`, `.flow/evidence/fn9/profiles.json:41`).
- Leaf visibility is whole-year cohorts over `leaf_lifetime` (`timeline.rs:218-231`); the clock splits an age into year and remainder (`crates/telperion-core/src/growth/clock.rs`, used at `timeline.rs:222`).
- Colour rows are one leaf pair per family (`crates/telperion-core/src/material.rs:20-25`); per-instance variation is a seeded hue and brightness offset only (`crates/telperion-render/src/shaders/foliage.wgsl:33`, `:41`, `:86-87`).
- Level ladders anchor base, widest and tip of a single blade (`crates/telperion-core/src/foliage/levels.rs:172-186`).
- Wire path for a new group: `fields!` (`crates/telperion-core/src/params.rs:11-157`), `known()` (`:213-220`), `walk!` (`crates/telperion-core/src/blend.rs:34-117`), `scripts/build-wasm.mjs:8-33` regenerates `src/browser/presets.generated.ts`, harness `Traits` renders any row object (`harness/dials.tsx:44-60`), `HELD` list in `crates/telperion-core/tests/sweep.rs:52+`.
- Files nearest the 400-line rule: `element.rs` 319, `timeline.rs` 343, `render/src/foliage.rs` 344, `select.rs` 384; fn-24 set the precedent of splitting into submodules.
- Prior requirement language that anticipates this: fn-9 R4 "any required grouping and orientation", fn-21 R3 "incorrectly grouped organs fail", `templates/species-profile.md:38` already has a fascicle slot, `docs/species-onboarding.md:24` names the "missing organ primitive" convention.

## Requirement coverage

| Requirement | Task |
|---|---|
| R1–R7 | TBD during planning |
