# The species catalogue as data

## Conversation Evidence

> user (2026-09-14): "should we actually spec a template/pipeline that would allow contributors to add trees as templates and improve the generator to make it possible to generate all trees in the world. If this is streamlined we could just recursively have grok add trees and update the generator?"
> user (2026-09-14): "i mean the goal should be all trees some time in the future. But ideally we just have a spec per tree species and have that follow a template"
> user (2026-09-14, on when): "After fn-34 returns"

## Goal & Context
<!-- scope: business -->
<!-- Goal & Context: 40% [user], 40% [paraphrase], 20% [inferred] -->

The owner's goal is every tree species in the world, each one a spec generated from a template and implemented as a value table by a value-tier model, with the generator improved whenever a species exposes a form it cannot make. fn-34 was the first run of that loop: cursor-agent with Grok 4.6 onboarded beech and birch in 36 minutes, 48 of 48 numeric cases, on the fn-9 packet. [user]

What fn-34 also showed is that the repository is not yet shaped for a catalogue. Adding one species touched twelve hand-maintained identity sites (the enum, the id parser, the profile map, the wire catalogue, six test lists, the browser exports and the bindings count). The sweep test walks every pair of presets, so its cost grows with the square of the catalogue. A species pair committed 7 MB of judging stills. A binding fixture needed a per-species height patch. Each of those is a place a value-tier model can make a subtle mistake, and each one multiplies by the number of species. [paraphrase]

This spec makes a species one file and one command. A species template is a data entry the catalogue reads; every test, export and runner iterates the catalogue rather than a hand list; the sweep samples pairs; stills stay on disk with checksums in the evidence; and a species spec is rendered from a template so a contributor, human or model, needs no repository knowledge beyond the packet. [inferred]

## Architecture & Data Models
<!-- scope: technical -->

- **A species is one value table in one file.** `crates/telperion-core/src/presets/species/<id>.rs` holds one function returning the `Family`, its material row beside it, and a `const ENTRY: Species` naming the catalogue id, display name, note, abi id and profile id. A registry in `presets/species/mod.rs` lists the entries once; `Preset` stays as the enum for the five synthetic and founding families and gains one `Species(&'static Species)` arm, so the enum never grows per species and no match statement is edited when one lands. `from_id`, `profile_id`, `CATALOGUE` and `parameters` all read the registry. [inferred]
- **Every enumeration reads the catalogue.** The tests that list identities today (`sweep.rs`, `mesh.rs`, `species.rs`, `growth_reference.rs`, `identity.rs`, the specimen unit-test loops, `examples/measure.rs`, `tests/browser/bindings.mjs`) iterate `CATALOGUE` and fail on a registry entry that lacks a pin or a band, so a missing pin is a test failure with the species named rather than a silently unpinned preset. Per-species data those tests need (pins, leaf-count bands, the growth reference height by age) live in one `.flow/evidence/catalogue/<id>.json` the tests read, so a species lands with its numbers beside its table and no test source is edited. [inferred]
- **The sweep samples.** The walk keeps every pair among the founding five and the Two Trees, and for the rest walks each species against its nearest founding family and one seeded random partner, so the walk grows linearly in the catalogue. The held-paths proof stays complete because it reads the union of moved paths across the sampled walks. [inferred]
- **Stills stay off the repository.** The judging set renders to the ignored evidence directory; the committed `stills.json` carries each file's sha256, preset, seed and view, and the owner's verdict. fn-29's and fn-34's committed stills stay where they are; no future species commits a raster. [paraphrase]
- **A species spec is rendered from a template.** `templates/species-spec.md` is the fn-34 body for one species with placeholders for the taxon, the catalogue id, the architectural model, the organs and the base-for legend; `node scripts/new-species-spec.mjs --id <id> --scientific "<name>" --common "<name>" --model "<Hallé model>" --organs "<list>"` renders it and creates the spec through `flowctl spec create`. The contributor guide in `docs/species-onboarding.md` gains the one-command path and the two things a species may not do: touch generator or renderer code, and hand-edit a generated file. [user]
- **Coverage is measured against the 23 architectural models.** `.flow/evidence/catalogue/models.json` lists the Hallé and Oldeman models with, for each, whether the field expresses it today, which shipped species demonstrates it, and the spec that would close it. A species spec names its model; a model marked unsupported blocks the species spec with a gap spec named, before any value table is written. [user]
- **The binding fixture is derived, not patched.** The browser bindings test builds its compact fixture from the family's own leader internode so no species needs a per-species height line. [inferred]

## API Contracts
<!-- scope: technical -->

- **Registry shape** `Species { id, name, note, abi_id, profile_id, family: fn() -> Family }` in `presets/species/mod.rs`; `CATALOGUE` in `params.rs` is derived from the founding entries plus the registry, in that order, so existing abi ids never move. [inferred]
- **Per-species evidence file** `.flow/evidence/catalogue/<id>.json`: `pins { wood_vertices, wood_triangles, instances, min, max, skeleton, placement, element }`, `leaf_band [min, max]`, `growth_reference { age, height_m }`, `profile_path`, `profile_sha256`. Missing or malformed fails the test naming the id. [inferred]
- **Template placeholders** `{{id}}`, `{{scientific_name}}`, `{{common_name}}`, `{{model}}`, `{{organs}}`, `{{base_for}}` (optional), `{{context}}`; unknown placeholders left in the rendered body fail the script. [inferred]
- **Views, wire and browser metadata unchanged**; the generated catalogue gains entries as species land. [paraphrase]

## Edge Cases & Constraints
<!-- scope: technical -->

- **Byte identity across the move.** Every shipped preset's identity pins, sweep bands and browser metadata are unchanged by the registry refactor; the refactor lands as its own commit that moves no numbers. [paraphrase]
- **Abi ids are stable.** A species keeps its abi id forever; the registry assigns the next free id and the test asserts no two entries share one. [inferred]
- **The founding families stay in `presets.rs`.** Ordinary, the oak, the spruce, Telperion and Laurelin are not moved into the registry in this spec; they are the pattern the registry mirrors, and moving them risks the pins for no gain. A later spec may fold them. [inferred]
- **File sizes.** One species file is about 120 lines with its material row; the registry file is a list. `presets.rs` shrinks. [paraphrase]
- **The sweep's sampling is seeded** so the same pairs walk on every run and a failure names the pair. [inferred]
- **Stills ignored, hashes committed.** `.gitignore` gains the catalogue evidence stills directory; a still whose sha256 does not match `stills.json` is not evidence. [paraphrase]
- **No generator or renderer change** in this spec beyond the registry and the derived fixture; the models file is data, and a gap it names is a spec, not a task here. [paraphrase]
- **fn-30, fn-31, fn-33 in flight.** This spec touches presets and tests; it rebases over the growth specs on landing, and fn-33's organ rows join the species file shape when they exist. [inferred]

## Acceptance Criteria
<!-- scope: both -->

- **R1:** A species is one file in the species registry with its value table, material row and catalogue entry, and one evidence file with its pins, band and growth reference; adding a species edits no enum arm, no match, no test source and no generated file by hand. Errors: a registry entry without an evidence file, or a duplicate id or abi id, fails a test naming it. [user]
- **R2:** Every test, example and runner that enumerates presets reads the catalogue; the five founding families and the two fn-34 species keep byte-identical pins, bands and browser metadata through the refactor, committed as a numbers-free move. Errors: a moved pin fails the refactor commit. [paraphrase]
- **R3:** The sweep walks every founding pair and, for each catalogue species, its nearest founding family and one seeded partner, and the held-paths proof still covers the union of moved paths. Errors: a species with no nearest founding family declared fails naming it. [inferred]
- **R4:** Judging stills render to the ignored evidence directory and are recorded by sha256, preset, seed and view in a committed `stills.json` with the owner's verdict field; no new raster is committed. Errors: a hash mismatch is not evidence. [paraphrase]
- **R5:** `templates/species-spec.md` and `scripts/new-species-spec.mjs` render and create a species spec from the taxon, id, model and organs in one command, and `docs/species-onboarding.md` documents that path and the two prohibitions. Errors: an unfilled placeholder fails the script naming it. [user]
- **R6:** `.flow/evidence/catalogue/models.json` lists the 23 Hallé and Oldeman architectural models with support status, demonstrating species and closing spec, and a species spec that names an unsupported model is blocked until the closing spec is captured. Errors: a model absent from the file is unsupported. [user]
- **R7:** The browser bindings test derives its compact fixture from the family's leader internode, and beech passes without its per-species height line. Errors: no error surface beyond the test. [inferred]
- **R8:** Species specs route to the value tier and gap specs to the frontier tier by a line in CLAUDE.md's routing block, replacing the fn-34 exception. Errors: no error surface. [user]

## Boundaries
<!-- scope: business -->

- No species onboarded here; fn-34 shipped two and species specs follow from the template. [paraphrase]
- No generator capability; a model gap is a spec. [user]
- The founding families stay where they are. [inferred]
- No numeric whole-tree identity instrument; the owner's eye stays the final judge per species, sampled as the catalogue grows. [inferred]

## Decision Context
<!-- scope: both -->

### Motivation

- The owner wants every tree species, each as a templated spec a value-tier model implements, with the generator improved on each gap; fn-34 proved the value tier on the packet and exposed the twelve-site, quadratic-sweep, committed-stills friction this spec removes. [user]

### Implementation Tradeoffs

- A registry arm on `Preset` over a per-species enum variant: the enum is what forces twelve edits; one arm holding a static entry keeps `Preset` copyable and every match closed. [inferred]
- Per-species evidence JSON over pins in test source: a value-tier model editing a pins array is the kind of edit that goes wrong silently; a file beside the table is checked by the test. [inferred]
- Sampled sweep over the full walk: the full walk is the only proof that the tree space has no kind switch, and it stays complete for the founding families where the risk lives; a species is a point near a founding family, so one walk to it and one random walk keep the proof at linear cost. [inferred]
- Stills off the repository: a 7 MB pair per species is 2 GB at three hundred species; hashes and the owner's words are the evidence, the rasters are reproducible from the pins. [paraphrase]

## Parked unknowns

- Which 23-model classification file to cite as the canonical list, and whether any model is out of scope for a game tree by policy.
- Whether the founding families fold into the registry once its shape has held for a few species.

## Strategy Alignment

- Follows "The catalogue": every species as a value table over a supported form, one spec from a template, the value tier implementing, coverage measured against the architectural models.
- Follows "The core and integration": generated bindings and one lean core keep a new template accessible with no renderer change.

## Resolved via Codebase

- Identity sites touched by fn-34: `crates/telperion-core/src/params.rs:163` CATALOGUE, `presets.rs:13` enum, `:54` profile_id, `:62` from_id, `:73` parameters, `tests/sweep.rs:16` IDS and `:44` BANDS, `tests/mesh.rs:11`, `tests/species.rs`, `tests/growth_reference.rs`, `tests/identity.rs` PINS, `examples/measure.rs`, `tests/browser/bindings.mjs:129`, `src/browser/core.ts`, `src/index.ts`, six specimen unit-test preset loops (`branching/specimen/*tests.rs`, `contacts.rs`), `src/browser/presets.generated.ts` via `scripts/build-wasm.mjs`.
- The sweep walks every pair: `tests/sweep.rs:97-100` `pairs()` over `IDS`, ten steps each (`:24`).
- fn-34 committed 12 PNGs, 7.0 MB; fn-29 committed 9, 7.9 MB (`.flow/evidence/fn29/stills`).
- The beech binding fixture patch: `tests/browser/bindings.mjs` sets a 6 m envelope for `european-beech` because a 4 m envelope is under two 2.2 m internodes.
- `presets/species.rs` (fn-34) is 122 lines for two species; `presets.rs` 328; `presets/materials.rs` 217.
- `crown_reference.rs` is an ignored FN6 pin list that excludes the oak and the spruce already; it is not an identity site.

## Requirement coverage

| Requirement | Task |
|---|---|
| R1–R8 | TBD during planning |
