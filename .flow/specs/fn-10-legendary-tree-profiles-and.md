# Legendary tree profiles and supernatural traits

## Conversation Evidence

> E1: "can we do this for certain \"magical\" trees also? famous ones? that have plenty of references."
> E2: "I think we should have all \"supernatural\" attributes be separated from natural ones. and by default have them off."
> E3: "separate spec is also fine"
> E4: "so each species has a profile that we work towards with the template"

> E5: "ofc the supernatural templates will have the supernatural traits on.."

> user (2026-09-14, refine, tree set): "Mallorn (Lothlórien), White Tree of Gondor, Yggdrasil (Norse ash)"
> user (2026-09-14, refine, on scale): "Legend scale, rails moved"
> user (2026-09-14, refine, on new capabilities): "no we want to leave it open to find things that need implementation. Ideally we iterate. Try and find parameters if can't be built with existing engine identify gap and close it."
> user (2026-09-14, refine, on where gaps go): "it seems like we need at least 2 new specs before we start this? we should have roots and flowers also for natural trees. So generator has a gap there."
> user (2026-09-14, refine): "ok so then i'd agree we capture a spec for flowers separately. Leave luminosity for later"

## Goal & Context

Extend reference-driven profiles and templates to famous magical or legendary trees with ample reference material, while keeping natural anatomy distinct from optional supernatural traits. [paraphrase] (E1, E2, E4)

## Architecture & Data Models
<!-- scope: technical -->

Each profile identifies the natural base and the supernatural interpretation. Templates expose these as separate controls. General defaults and ordinary templates leave supernatural attributes off; supernatural templates explicitly enable the traits that define them. [paraphrase] (E2, E4, E5)

- **Three trees, each on a real base.** The Mallorn of Lothlórien on the European beech, the White Tree of Gondor on the silver birch, and Yggdrasil on the European ash. The base species are onboarded by fn-34-beech-ash-and-birch-as-real-species; this spec starts when that spec, fn-20 and fn-33-flowers-cones-and-compound-leaves-as have landed and fn-30 and fn-31 have settled the growth rule. [user]
- **A legendary profile in the fn-19 schema** under `.flow/evidence/fn10/<tree>/`, with `kind: legendary`, the base species' profile id, the declared interpretation, and every dimension the sources give as an estimate with its source. The depiction rule is fixed: primary text first, then official film or game frames, then official concept art; fan art is never evidence. [user]
- **One preset per tree, natural base by switching off.** Each tree is one `Preset` variant in the legendary submodule of the presets, starting from its base species' table and differing by the supernatural rows, the colour rows and any habit rows the profile supports. The natural base is the same preset with the supernatural group at its neutral value; there is no second natural table. [user]
- **A supernatural scale row.** The shared group gains `scale`, neutral 1.0, that multiplies the envelope height and spread when the group is enabled. Radius rows are fractions of height so the wood scales with it; element, twig and internode rows are metres so leaves keep the base species' size and the crown reads as a giant tree rather than a magnified one. Off means a species-sized tree, for every supernatural template. [user]
- **The Two Trees move onto the same rule.** Telperion and Laurelin take a natural base height and the scale row instead of a 148 m and 132 m envelope written directly, so "off" gives a species-sized tree there too, and their identity pins are re-recorded once with the reason. In the same pass their colour rows become silver and gold from the text; they carry the default brown and green today. [user]
- **Gaps become specs.** The implementer builds each tree from the rows the field has, records what the profile asks for that the field cannot express as an unmet requirement in the profile and a candidate spec under Parked unknowns, and never adds a field, a shader term or a generator branch inside this spec. Two gaps found during this refine, roots and flowers, already went that way. [user]
- **Validation reuses fn-9's runner in both modes.** The species runner measures each legendary preset with the group off against the base species' profile gates, and with the group on against the legendary profile, where the declared height is the one gating value as an authored target and everything else is contextual. [paraphrase]
- **Cost is measured, not bounded.** Frame and build cost of each legendary template at its declared scale are taken with the existing rig and recorded beside Telperion's; the owner accepts or rejects a template costlier than Telperion. [user]

## Edge Cases & Constraints
<!-- scope: technical -->

Distinguish documented descriptions, visual depictions and estimated dimensions. Conflicting depictions require a declared target interpretation. A natural base need not reproduce features that exist only when supernatural traits are enabled. [inferred]

- **Scale rail and validation.** `scale` is finite and in 0.1 to 100, validated naming the field, blended linearly, read as 1.0 whenever the group is disabled through the same path that reads the writhe and spiral rows as none. [inferred]
- **Ceilings move with the tallest declared height.** The harness height slider, the growth node caps and the sweep's leaf-count bands are raised so the tallest tree grows to completion; a truncated growth is a failure, never a smaller tree. [user]
- **The White Tree is one preset in leaf.** Its bare courtyard state is fn-11's chronicle at an age with the foliage shed, not a second preset; blossom waits for fn-33-flowers-cones-and-compound-leaves-as's organ slot and is set only if that spec has landed. [user]
- **Yggdrasil's roots come from fn-20.** Visible structural roots at the base are fn-20's capability; reach across the ground beyond what fn-20 ships is recorded as unmet. [paraphrase]
- **Determinism.** Same seed and parameters give a byte-identical tree in both modes, and toggling the group is the only difference between them. [paraphrase]
- **Reading budget.** The owner's judging set is six images per tree, one fixed seed in whole, bare and leaf views in both modes; the full seed protocol is measured numerically and never rendered for reading. [user]
- **References.** Fetched into the ignored references directory under this spec's id, never redistributed, with an fn-19 reference record per image naming its source tier. [paraphrase]
- **Budget.** The per-task budget from CLAUDE.md binds; a tree that cannot pass inside it stops with `NEEDS_HUMAN` and its profile marked unready. [paraphrase]

## API Contracts
<!-- scope: technical -->

- **Family row added:** `skeleton.bias.supernatural.scale`, on the wire, in the wasm metadata, in the generated browser catalogue and in the harness supernatural fieldset. [inferred]
- **Catalogue ids** `mallorn`, `white-tree-of-gondor`, `yggdrasil`, with names and one-line notes, selectable natively, through wasm and from the browser exports. [inferred]
- **Profiles** `.flow/evidence/fn10/<tree>/profile.json`, `references.json`, `species.json` in the fn-19 schema with `kind: legendary` and `base_profile_id`. [inferred]
- **Views and commands unchanged.** [paraphrase]

## Acceptance Criteria

- **R1:** Select a bounded set of famous fictional or legendary trees with ample attributed references and define profiles for their form, dimensions, branching and foliage. Identify insufficient evidence and label estimates rather than treating them as measured facts. [inferred]
- **R2:** Build a template for each selected profile and validate its identifying features across multiple seeds using visual and quantitative comparisons. Record both the natural-base target and the supernatural reference interpretation; report mismatches in the mode being judged. [inferred]
- **R3:** Keep all supernatural attributes separately identifiable and controllable from natural attributes. General defaults and ordinary templates leave them off. Selecting a supernatural template enables that template's intended supernatural traits; the user can independently disable them. Disabling them restores the natural result for the same seed and natural parameters, without residual supernatural deformation. [paraphrase] (E2, E5)
- **R4:** Reuse the shared botanical and foliage foundation, adding only the profile-required geometry capabilities. Unsupported traits remain explicit unmet requirements; natural-template comparisons detect unintended effects on ordinary species. [inferred]
- **R5:** The set is the Mallorn on the European beech, the White Tree of Gondor on the silver birch, and Yggdrasil on the European ash, each with a legendary profile in the fn-19 schema whose declared interpretation follows the fixed depiction precedence. Errors: a tree whose sources cannot pin silhouette, bark, leaf and a declared height is left unready rather than estimated into readiness. [user]
- **R6:** The supernatural group carries a scale row, neutral 1.0, that scales envelope and wood while leaves keep the base species' size; a legendary template with the group off is byte-identical to its base species at natural size for the same seed, and Telperion and Laurelin take the same rule with their pins re-recorded once. Errors: a scale outside 0.1 to 100 is refused naming the field; a disabled group reads scale as 1.0. [user]
- **R7:** Each legendary template passes the species runner in both modes: the base species' gates with the group off, and the declared height as the one gating value with the group on; growth completes at the declared scale after the ceilings move. Errors: a truncated growth fails the template; a numeric failure is a retained case. [paraphrase]
- **R8:** Frame and build cost of each template at its declared scale are recorded beside Telperion's numbers and the owner accepts or rejects each in this spec. Errors: an unavailable, disjoint or contended session does not count. [user]
- **R9:** Telperion and Laurelin carry silver and gold colour rows set from the text, judged by the owner in the same still round. Errors: no error surface beyond row validation. [user]
- **R10:** The owner judges six images per tree, one fixed seed in whole, bare and leaf views in both modes, beside the references, and records the verdicts in this spec; the spec closes only on accepting verdicts. Errors: a rejecting verdict stops the spec with the owner's words and a one-paragraph blocker. [paraphrase]
- **R11:** A trait the field cannot express is recorded as an unmet requirement in the profile and as a candidate spec under Parked unknowns; this spec adds no field beyond the scale row, no shader term and no generator branch. Errors: an unmet trait is never approximated into a pass. [user]
- **R12:** The automated tests cover: scale rail and neutral identity on every preset; off-mode byte identity against the base species for each legendary preset and for the Two Trees; the moved ceilings; identity pins and sweep bands for the three new presets; and the catalogue, wasm and browser exposure of the new ids and the scale row. Errors: a missing case is a review finding, not implementer discretion. [inferred]

## Boundaries

This pass covers geometry and procedural foliage. Supernatural materials, light emission, other appearance effects and lifecycle simulation remain later work; their absence must not be presented as complete visual reproduction. [inferred]

## Decision Context

Magical subjects can have a separate spec. [paraphrase] (E3)

Depends on “Real-species profiles and procedural templates” for the shared profile, template and validation foundation. [inferred]

### Implementation Tradeoffs

- Scale as a supernatural row over an envelope height written per preset: the row makes "off" mean a species-sized tree for every supernatural template and gives the blend one number to walk; writing the height directly, as the Two Trees do today, leaves no natural base to judge. [paraphrase]
- Onboarding the base species over judging the natural mode by eye or reusing the oak's numbers: a base with no profile has no gates, and an ash judged against oak numbers is wrong; the species work stands alone as its own spec. [user]
- Gaps as specs over closing them in place: the owner asked to find and close gaps by iteration, then chose that roots and flowers belong to natural trees first; a gap found during implementation follows the same path. [user]
- Luminosity later: none of the three trees glows in its sources, so a glow spec touches Telperion and Laurelin and is not a prerequisite here. [user]
- Dependencies recorded on this spec: fn-20 (roots and junction anatomy), fn-33-flowers-cones-and-compound-leaves-as (organ slot for blossom, compound leaves for the ash), fn-34-beech-ash-and-birch-as-real-species (natural bases), fn-30 and fn-31 (the growth rule the presets carry). [user]

## Parked unknowns

- Root reach beyond fn-20's visible structural roots, if Yggdrasil's profile asks for it; a candidate spec once fn-20 shows what it ships.
- Luminosity as a supernatural appearance row for Telperion and Laurelin; a candidate spec after fn-32 settles the shader budget.
- The declared heights: the Mallorn's and Yggdrasil's sources give no figure, so the profile research declares an estimate and the owner ratifies it before the still round.

## Resolved via Codebase

- The supernatural group is `SupernaturalParams { enabled, writhe_amplitude, writhe_wavelength, spiral_rate }` (`crates/telperion-core/src/bias.rs:13-31`); a disabled group is read as `NONE` in `GrowthBias::apply` (`bias.rs:100-106`) and in the blend (`crates/telperion-core/src/blend.rs:228-245`); the test `natural_bias_is_independent_of_disabled_effects` (`crates/telperion-core/tests/growth/limits.rs:105-138`) asserts skeleton identity with the group off.
- Telperion and Laurelin set envelope height directly (148 m, 132 m at `crates/telperion-core/src/presets.rs:225-240`) and enable the group at `:248-272`; `materials::radiant(silver)` sets gloss only, so both carry the default brown bark and green leaf (`crates/telperion-core/src/presets/materials.rs:104-115`, `crates/telperion-core/src/material.rs:79-88`).
- The core has no height ceiling: `Envelope::validate` requires finite and non-negative only (`crates/telperion-core/src/envelope.rs:25-42`); the ceilings are the harness slider 4 to 400 m (`harness/params.ts:223`), `GrowthConfig.max_nodes` 4000 (`crates/telperion-core/src/branching/colonization.rs:34`), `SWEEP_NODES` 8000 and the per-preset leaf-count bands (`crates/telperion-core/tests/sweep.rs:44`).
- Element and twig rows are metres, not fractions of height (`crates/telperion-core/src/foliage/element.rs:134-166` rails; spruce internode 2.5 mm at `presets.rs:170-172`); radius and flare rows are fractions of height, so a height multiplier scales wood and not leaves.
- No emission, multi-trunk, root geometry or blossom exists in the generator or the nine shaders; cavity is shading only (`material.rs:63`, `crates/telperion-render/tests/colour_cavity.rs`).
- Presets file at 313 lines against the 400-line rule; `presets/materials.rs` is the submodule precedent. Identity sites to update are listed in fn-34-beech-ash-and-birch-as-real-species.
- Species runner and stills: `tests/species.mjs`, `crates/telperion-core/examples/species_measure.rs`, `crates/telperion-render` headless example with views `whole`, `bare`, `leaf`, `clay`.
- No mention of any legendary tree beyond Telperion and Laurelin exists in the repo.

## Requirement coverage

| Requirement | Task |
|---|---|
| R1–R12 | TBD during planning |
