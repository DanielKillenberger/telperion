# {{common_name}} as a real species

## Conversation Evidence

> owner (2026-09-14, the catalogue track): every tree species in the world is the long-horizon goal; each species is one spec generated from a template and implemented as a value table by a value-tier model; a species that exposes an unsupported form or organ becomes a generator spec for the frontier tier.
> prototype: fn-34, beech and birch, implemented by cursor-agent with Grok 4.6 in one run of 36 minutes, 48 of 48 numeric cases; this template is that spec for one species.

## Goal & Context
<!-- scope: business -->

{{common_name}} ({{scientific_name}}) joins the catalogue as one preset that is a value table, calibrated to a frozen botanical profile with cited ranges and catalogued references, and judged on the fixed and fresh seed protocol with the owner's verdict on the stills. A developer selecting `{{id}}` gets a tree that reads as a {{common_name}} at every seed. [paraphrase]

Architectural form: {{model}}. Organs the profile needs: {{organs}}. {{base_for}}Context for the profile: {{context}}. [paraphrase]

This spec adds no generator or renderer capability. If the profile asks for a form or an organ the field cannot express, that gap is recorded in the capability matrix and captured as a generator spec for the frontier tier, and this species waits for it. [user]

## Architecture & Data Models
<!-- scope: technical -->

- **The onboarding packet is the contract.** Follow `docs/species-onboarding.md` stage by stage and fill `templates/species-profile.md`: research, profile, capability assessment, template, integration, seed freeze, numeric and visual validation, catalogue acceptance. Deliverables are `profile.json`, `references.json` and `species.json` in the fn-19 schema under `.flow/evidence/catalogue/{{id}}/`, with reference photographs in the ignored `.refs/catalogue/{{id}}/`, each recorded with source, attribution and sha256. Prefer silvics literature, forestry measurement tables and floras; a Wikipedia figure is a lead to a primary source, never a citation. [paraphrase]
- **A preset is one value table in one file.** `crates/telperion-core/src/presets/species/{{id}}.rs` holds the family function, its material row and its registry entry, following fn-35's registry; until fn-35 lands, the identity sites listed in fn-35's Resolved via Codebase are edited by hand and the browser catalogue is regenerated with `node scripts/build-wasm.mjs`, never by hand. The oak and the spruce are the pattern; copy their shape, never their values. [paraphrase]
- **Growth traits are copied, then calibrated.** Start from the nearest calibrated species' growth traits and set the age so the profile's mature reference height is met; record the height by age in the evidence file so fn-30's method has a target when it is applied. [inferred]
- **Evidence beside the table.** `.flow/evidence/catalogue/{{id}}.json` carries the identity pins, the leaf-count band and the growth reference; the catalogue-driven tests read it. [paraphrase]

## API Contracts
<!-- scope: technical -->

- **Catalogue id** `{{id}}`, display name "{{common_name}}", note "{{scientific_name}}", selectable natively, through the wasm catalogue and from the browser exports. [inferred]
- **Profile id** equals the catalogue id; `species.json` carries the profile path and checksum. [paraphrase]
- **No new parameters.** The wire, the blend and the browser metadata carry no new keys from this spec. [paraphrase]

## Edge Cases & Constraints
<!-- scope: technical -->

- **Evidence rules from fn-9 hold.** Missing, conflicting or ambiguous evidence is recorded; an estimate is labelled and cannot gate; a candidate with inadequate evidence is left unready rather than filled in. Definitions of height, DBH, crown dimensions, branch counts and foliage units are those in `.flow/evidence/fn9/profiles.json`. [paraphrase]
- **Counting.** A leaf is one placement; a leaflet, a needle or an organ is one instance; the profile states units per leaf so the metrics' units-per-instance value has a target. [paraphrase]
- **Seeds.** Twelve fixed seeds from the fn-9 list and twelve fresh seeds drawn once with `--draw-seeds` and recorded; fresh failures are retained as regressions and never resampled away. [paraphrase]
- **Stills.** Three fixed seeds in the whole and bare views at 960 by 720 through the headless target, rendered to the ignored evidence stills directory and recorded by sha256, preset, seed and view in a committed `stills.json`; the implementer reads at most four images and never awards a visual pass. [paraphrase]
- **Determinism.** Same seed and parameters give a byte-identical tree; the pins assert it. [paraphrase]
- **Budget.** The per-task budget from CLAUDE.md binds; a species that cannot pass its gates inside it stops with `NEEDS_HUMAN` and the profile marked unready. [paraphrase]
- **Two prohibitions.** No generator or renderer code change, and no hand edit of a generated file. [user]

## Acceptance Criteria
<!-- scope: both -->

- **R1:** {{common_name}} has a frozen profile, catalogued references and a species record in the fn-19 schema, with gating and contextual ranges cited and estimates labelled, and its architectural model and organs named against the catalogue's coverage file. Errors: inadequate evidence leaves the profile unready; an unsupported model or organ stops the spec with the gap spec named. [paraphrase]
- **R2:** The species ships as one named preset that is a value table in its own file, selectable natively, through wasm and from the browser, with no generator or renderer change and no hand-edited generated file. Errors: an unknown id fails naming it. [paraphrase]
- **R3:** The fixed and fresh seed protocol passes the profile's gating ranges with per-seed discrepancies retained, and the owner judges three fixed seeds in whole and bare views beside the references, recording the verdict in this spec; the spec closes only on an accepting verdict. Errors: a rejecting verdict stops the spec with the owner's words and a one-paragraph blocker; a numeric failure is a retained case, never a resample. [paraphrase]
- **R4:** The species has identity pins, a sweep leaf-count band and a growth reference with the mature height by age in its evidence file, and same seed and parameters yield a byte-identical tree. Errors: a moved pin is re-pinned once with the reason stated. [inferred]

## Boundaries
<!-- scope: business -->

- No generator or renderer capability; a gap becomes a spec. [user]
- No growth-trait calibration through time; fn-30's method is applied afterwards. [inferred]
- One species; siblings get their own spec from the same template. [paraphrase]

## Decision Context
<!-- scope: both -->

### Motivation

- The owner's catalogue track: every species as a value table over a supported form, one spec from a template, the value tier implementing, the frontier tier closing gaps. [user]

### Implementation Tradeoffs

- One spec per species over a cohort spec: fn-34 carried three species and its ash had to be held back inside the same task; one species per spec lets each land, wait or fail on its own. [paraphrase]

## Strategy Alignment

- Follows "The catalogue": every species a value table over a supported form, judged against references, the owner's eye on the final round.
- Follows "Growth and botanical fidelity": measured real-species profiles make branching and foliage rules answerable to references.

## Requirement coverage

| Requirement | Task |
|---|---|
| R1–R4 | one implicit task |
