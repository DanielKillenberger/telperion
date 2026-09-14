# Beech, ash and birch as real species

## Conversation Evidence

> user (2026-09-14, on the natural base of each legendary tree): "Onboard the base species"
> user (2026-09-14, on where that work lives): "Separate species spec"
> user (2026-09-14, on the White Tree's base): "Leafed, bare state via age" (a birch-like base was the recommendation accepted)
> user (2026-09-14, on the tree set for fn-10): "Mallorn (Lothlórien), White Tree of Gondor, Yggdrasil (Norse ash)"

## Goal & Context
<!-- scope: business -->
<!-- Goal & Context: 30% [user], 50% [paraphrase], 20% [inferred] -->

Three more real species join the catalogue: European beech (Fagus sylvatica), European ash (Fraxinus excelsior) and silver birch (Betula pendula). Each gets what the oak and the spruce got in fn-9: a frozen botanical profile with cited ranges, catalogued references, a preset that is a value table, the fixed and fresh seed protocol, numeric gates and inspected stills. A game developer selecting "beech" gets a tree that reads as a beech at every seed. [paraphrase]

The owner chose these three because they are the natural bases of the legendary trees fn-10 builds: the Mallorn grows from the beech, the White Tree of Gondor from the birch, and Yggdrasil from the ash. With the supernatural group off, a legendary template is its base species at natural size, so the base has to exist with numbers before the legend can be judged. The species are worth having whether or not the legends ship. [user]

This is fn-9's method applied three times with no new engine work of its own. The one anatomy the generator cannot express, the ash's pinnate compound leaf, is delivered by fn-33-flowers-cones-and-compound-leaves-as, and the ash template waits for it. [paraphrase]

## Architecture & Data Models
<!-- scope: technical -->

- **The onboarding packet is the contract.** Each species follows `docs/species-onboarding.md` stage by stage and fills `templates/species-profile.md`: research, profile, capability assessment, template, integration, seed freeze, numeric and visual validation, catalogue acceptance. Deliverables per species are `profile.json`, `references.json` and `species.json` in the fn-19 schema under `.flow/evidence/<this spec>/<species>/`, with reference images in the ignored references directory under this spec's id. [paraphrase]
- **A preset is a value table.** Each species is one `Preset` variant whose values live in a new `presets/species.rs` submodule beside `presets/materials.rs`, because `presets.rs` is at 313 lines against the 400-line rule and one preset in the oak's style is about 80 lines. Material rows go in `presets/materials.rs` or a sibling when it passes the rule. No generator or renderer code changes; a species that needs one is a gap recorded in its capability matrix and a candidate spec, never closed here. [paraphrase]
- **Every identity site is updated.** Adding a preset touches the catalogue, the enum and its id parser, the profile-id map, the test identity lists, the sweep ids and leaf-count bands, the measurement example's help, the browser bindings count and the generated browser catalogue; the list is in Resolved via Codebase. [inferred]
- **Growth traits are copied, then calibrated.** Each species starts with the oak's growth traits (the beech and the ash) or the spruce's (none; the birch is a broadleaf and takes the oak's), and its age is set so the mature reference height is met. Full calibration of the growth traits through time is fn-30's method, applied to these species after fn-30 and fn-31 land; the profile records the reference height by age so that calibration has a target. [inferred]
- **Order within the spec.** Beech, then birch, then ash; the ash's template lands only after fn-33-flowers-cones-and-compound-leaves-as ships compound leaves, while its profile and references are gathered with the other two. That gate is this line and R4, not a flow dependency edge, so the beech and the birch are never blocked by fn-33. [paraphrase]

## API Contracts
<!-- scope: technical -->

- **Catalogue ids** `european-beech`, `european-ash`, `silver-birch`, each with a name and a one-line note, selectable natively by id, through the wasm catalogue and from the browser exports the way the oak and the spruce are. [inferred]
- **Profile ids** match the catalogue ids and are returned by the preset's profile-id map; `species.json` carries the profile path and checksum. [paraphrase]
- **No new parameters.** The wire, the blend and the browser metadata carry no new keys from this spec; the generated browser catalogue gains three entries. [paraphrase]

## Edge Cases & Constraints
<!-- scope: technical -->

- **Evidence rules from fn-9 hold.** Missing, conflicting or ambiguous evidence is recorded; an estimate is labelled and cannot gate; a candidate with inadequate evidence is left unready rather than filled in. Height, DBH with its measurement height, crown width and base, branch order and counts, leaf dimensions and leaf area use the definitions in `.flow/evidence/fn9/profiles.json`. [paraphrase]
- **Ash counting.** An ash leaf is one placement and its leaflets are instances; the profile states leaflet count per leaf and leaf length so the metrics' units-per-instance value has a target. [inferred]
- **Seeds.** Twelve fixed seeds from the fn-9 list and twelve fresh seeds drawn once and recorded per species; fresh failures are kept as regressions and never resampled away. [paraphrase]
- **Stills.** Whole, bare and leaf views at the protocol size; the owner judges three fixed seeds per species in the whole and bare views beside the references, and every numeric failure, within the four-images-per-capture reading rule for agents. [paraphrase]
- **Determinism and pins.** Same seed and parameters give a byte-identical tree; each species gets identity pins (wood counts, bounds, skeleton, placement and element hashes) and leaf-count bands in the sweep. [paraphrase]
- **Budget.** A species task gets the per-task budget from CLAUDE.md; a species that cannot pass its gates inside it stops with `NEEDS_HUMAN` and the profile marked unready. [paraphrase]
- **Growth specs in flight.** This spec waits for fn-30 and fn-31 so that presets carry the final growth rule and the tables are written once. [user]

## Acceptance Criteria
<!-- scope: both -->

- **R1:** European beech, European ash and silver birch each have a frozen profile, catalogued references and a species record in the fn-19 schema, with gating and contextual ranges cited and estimates labelled. Errors: inadequate evidence leaves the profile unready and the species out of the catalogue. [user]
- **R2:** Each species ships as one named preset that is a value table in a submodule, selectable natively, through wasm and from the browser, with every identity site updated and no generator or renderer change. Errors: an unknown id fails naming it; a species needing an engine change records the gap and stops. [paraphrase]
- **R3:** The fixed and fresh seed protocol passes each species' gating ranges with per-seed discrepancies retained, and the owner judges three fixed seeds per species in whole and bare views beside the references, recording the verdicts in this spec; the spec closes only on accepting verdicts. Errors: a rejecting verdict stops the spec with the owner's words and a one-paragraph blocker; a numeric failure is a retained case, never a resample. [paraphrase]
- **R4:** The ash template uses fn-33-flowers-cones-and-compound-leaves-as's compound leaf rows, with leaflet count and leaf length gated against the profile and the metrics reporting placements, instances and units per instance. Errors: the ash template does not land before fn-33-flowers-cones-and-compound-leaves-as; a simple-blade ash is not a pass. [user]
- **R5:** Each species has identity pins, sweep leaf-count bands and a growth-reference entry with the mature height by age, and same seed and parameters yield a byte-identical tree. Errors: a moved pin is re-pinned once with the reason stated. [inferred]

## Boundaries
<!-- scope: business -->

- No new generator or renderer capability; a gap becomes a spec. [user]
- No legendary trees; those are fn-10's, built on these bases. [paraphrase]
- No growth-trait calibration through time; fn-30's method is applied afterwards. [inferred]
- No exhaustive catalogue beyond these three. [paraphrase]

## Decision Context
<!-- scope: both -->

### Motivation

- The owner chose to onboard the real base species rather than judge the natural mode by eye or reuse the oak's numbers, because a legendary template with its supernatural group off must be a species-sized tree with numbers to gate on. [user]
- The owner split this out of fn-10 so that the value-table work stands alone and can be implemented by a weaker model than the legends need. [paraphrase]

### Implementation Tradeoffs

- A submodule for the tables over growing `presets.rs`: the file is 87 lines under the rule and one preset costs about 80; `presets/materials.rs` set the precedent. [inferred]
- Copying the oak's growth traits over inventing per-species ones now: fn-30 owns the calibration method and is in flight; a copied trait with a recorded height-by-age target is honest and cheap to replace. [inferred]

## Strategy Alignment

- Follows "Growth and botanical fidelity": measured real-species profiles make branching and foliage rules answerable to references.
- Follows "The core and integration": one lean Rust core and generated bindings keep a new template accessible to native and browser consumers with no renderer change.

## Resolved via Codebase

- Five presets today: `enum Preset` (`crates/telperion-core/src/presets.rs:13`), `from_id` (`:62-71`), `profile_id` (`:54-60`); `params::CATALOGUE` (`crates/telperion-core/src/params.rs:163-174`).
- Identity sites to update: `params.rs:163`, `presets.rs:13` and `:62`, `crates/telperion-core/tests/sweep.rs:16` and `:44`, `tests/mesh.rs:11`, `tests/species.rs:50`, `tests/crown_reference.rs:51`, `tests/growth_reference.rs:29`, `examples/measure.rs:17`, `examples/species_measure.rs` help line, `tests/browser/bindings.mjs:129` (`PRESETS.length === 5`), `src/browser/core.ts:14-20` exports, `src/browser/presets.generated.ts` regenerated by `scripts/build-wasm.mjs`.
- Onboarding contract: `docs/species-onboarding.md`, `templates/species-profile.md`; schemas in `.flow/evidence/fn19/PROTOCOL.md` and `protocol.json`; worked packets under `.flow/evidence/fn19/onboarding-examples/`.
- Protocol runner: `tests/species.mjs` (npm `species:measure`, `species:qa`), measurement binary `crates/telperion-core/examples/species_measure.rs`, metrics `examples/species_metrics/mod.rs`, identity tests `crates/telperion-core/tests/species.rs`.
- Headless stills: `target/release/examples/headless --preset ID --seed N --view V --size WxH --out FILE`, views `whole`, `bare`, `leaf`, `clay` (`crates/telperion-render/src/view.rs:10-20`).
- Reference images are gitignored under `.refs/` per spec id (`.gitignore`; `.flow/evidence/fn9/REFERENCES.md:36-39` has the retrieval commands).
- `presets.rs` is 313 lines; the oak block is about 80 and the spruce about 65.

## Requirement coverage

| Requirement | Task |
|---|---|
| R1–R5 | TBD during planning |
