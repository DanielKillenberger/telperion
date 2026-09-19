# The species registry: one file per species

## Conversation Evidence

> user (2026-09-14): "i mean the goal should be all trees some time in the future. But ideally we just have a spec per tree species and have that follow a template"
> user (2026-09-18): "does fn-35 need planned? can we simplify it? 11 reqs seems alot" / "ok let's do the split as you proposed"
> user (2026-09-18): "so you mean the template in code yea?" (confirming this spec is the wiring around the value table, and the value table itself is unchanged)

## Goal & Context
<!-- scope: business -->
<!-- Goal & Context: 30% [user], 50% [paraphrase], 20% [inferred] -->

Adding one species today touches twelve hand-maintained identity sites: the enum, the id parser, the profile map, the wire catalogue, six test lists, the browser exports and the bindings count. fn-34 showed each one is a place a value-tier model can make a subtle mistake, and each multiplies by the number of species. The sweep test walks every pair of presets, so its cost grows with the square of the catalogue. A binding fixture needed a per-species height patch. [paraphrase]

This spec makes a species one Rust file with its value table, its material row and a one-line entry, and a registry every test, export and runner reads, so nothing else is edited when a species lands. The sweep samples pairs so it grows with the count. The browser fixture derives its number from the species' own data. The value table itself, the numbers the pipeline produces and the renderer draws, does not change. [user]

Split out of fn-35-the-species-catalogue-as-data on 2026-09-18. It depends on that spec for the pins file the tests read. [user]

## Architecture & Data Models
<!-- scope: technical -->

- **A species is one value table in one file.** `crates/telperion-core/src/presets/species/<id>.rs` holds one function returning the `Family`, its material row beside it, and a `const ENTRY: Species` naming the catalogue id, display name, note, abi id and profile id. A registry in `presets/species/mod.rs` lists the entries once; `Preset` stays as the enum for the five synthetic and founding families and gains one `Species(&'static Species)` arm, so the enum never grows per species and no match statement is edited when one lands. `from_id`, `profile_id`, `CATALOGUE` and `parameters` all read the registry. [inferred]
- **Every enumeration reads the registry.** The tests that list identities today (`sweep.rs`, `mesh.rs`, `species.rs`, `growth_reference.rs`, `identity.rs`, the specimen unit-test loops, `examples/measure.rs`, `tests/browser/bindings.mjs`) iterate `CATALOGUE` and fail on a registry entry that lacks a pin or a band, so a missing pin is a test failure with the species named rather than a silently unpinned preset. Per-species numbers those tests need (pins, leaf-count bands, the growth reference height by age) are read from `catalogue/<id>/pins.json`, which fn-35 places; the copies in test source are deleted. [inferred]
- **The sweep samples.** The walk keeps every pair among the founding five and the Two Trees, and for the rest walks each species against its nearest founding family and one seeded random partner, so the walk grows linearly in the catalogue. The held-paths proof stays complete because it reads the union of moved paths across the sampled walks. [inferred]
- **The binding fixture is derived, not patched.** The browser bindings test builds its compact fixture from the family's own leader internode so no species needs a per-species height line. [inferred]

## API Contracts
<!-- scope: technical -->

- **Registry shape** `Species { id, name, note, abi_id, profile_id, nearest_founding: Preset, family: fn() -> Family }` in `presets/species/mod.rs`; `CATALOGUE` in `params.rs` is derived from the founding entries plus the registry, in that order, so existing abi ids never move. [inferred]
- **Pins read** from `catalogue/<id>/pins.json` with the shape fn-35 fixes: `pins { wood_vertices, wood_triangles, instances, min, max, skeleton, placement, element }`, `leaf_band [min, max]`, `growth_reference { age, height_m }`. Missing or malformed fails the test naming the id. [inferred]
- **Sweep pairs** `pairs()` returns every founding pair, then for each registry species the pair with its `nearest_founding` and one pair with a partner drawn from a fixed seed. [inferred]
- **Views, wire and browser metadata unchanged**; the generated browser catalogue gains entries as species land. [paraphrase]

## Edge Cases & Constraints
<!-- scope: technical -->

- **Byte identity across the move.** Every shipped preset's identity pins, sweep bands and browser metadata are unchanged by the refactor; it lands as one commit that moves code and no numbers, and the pins test proves it. [paraphrase]
- **Abi ids are stable.** A species keeps its abi id forever; the registry assigns the next free id and the test asserts no two entries share one. [inferred]
- **The founding families stay in `presets.rs`.** Ordinary, the oak, the spruce, Telperion and Laurelin are not moved into the registry; they are the pattern the registry mirrors, and moving them risks the pins for no gain. A later spec may fold them. [inferred]
- **File sizes.** One species file is about 120 lines with its material row; the registry file is a list. `presets.rs` shrinks. [paraphrase]
- **The sweep's sampling is seeded** so the same pairs walk on every run and a failure names the pair. [inferred]
- **No generator or renderer change** beyond the registry and the derived fixture. [paraphrase]
- **Lands after fn-35.** The pins file must exist for every shipped species before the source copies go; on a branch cut before fn-35 lands, the tests keep reading source and the switch is the last commit. [inferred]

## Acceptance Criteria
<!-- scope: both -->

- **R1:** A species is one file in the species registry with its value table, material row and catalogue entry; adding a species edits no enum arm, no match, no test source and no generated file by hand; every test, example and runner that enumerates presets reads the registry and the pins from the species' catalogue folder; the five founding families and the shipped species keep byte-identical pins, bands and browser metadata through the refactor, committed as a numbers-free move. Errors: a registry entry without a catalogue folder, a duplicate id or abi id, or a moved pin fails a test naming it. [user]
- **R2:** The sweep walks every founding pair and, for each registry species, its nearest founding family and one seeded partner, the held-paths proof still covers the union of moved paths, and the browser bindings test derives its compact fixture from the family's leader internode so beech passes without its per-species height line. Errors: a species with no nearest founding family declared fails naming it. [inferred]

## Boundaries
<!-- scope: business -->

- No species onboarded here. [paraphrase]
- No generator capability; a model gap is a spec. [user]
- The founding families stay where they are. [inferred]
- No catalogue folder work; fn-35 places the files this spec reads. [user]
- No numeric whole-tree identity instrument; the owner's eye stays the final judge per species. [inferred]

## Decision Context
<!-- scope: both -->

### Motivation

- fn-34 exposed the twelve-site, quadratic-sweep, patched-fixture friction; every future species pays it until the registry exists. [paraphrase]

### Implementation Tradeoffs

- A registry arm on `Preset` over a per-species enum variant: the enum is what forces twelve edits; one arm holding a static entry keeps `Preset` copyable and every match closed. [inferred]
- Pins in a file over pins in test source: a value-tier model editing a pins array is the kind of edit that goes wrong silently; a file beside the record is checked by the test. [inferred]
- Sampled sweep over the full walk: the full walk is the only proof that the tree space has no kind switch, and it stays complete for the founding families where the risk lives; a species is a point near a founding family, so one walk to it and one random walk keep the proof at linear cost. [inferred]
- Its own spec over one with the catalogue: the byte-identity gate is the risky part and deserves a quiet landing; the catalogue does not need to wait for it. [user]

## Parked unknowns

- Whether the founding families fold into the registry once its shape has held for a few species.

## Strategy Alignment

- Follows "The catalogue": every species as a value table over a supported form, the value tier implementing.
- Follows "The core and integration": one lean core keeps a new template accessible with no renderer change.

## Resolved via Codebase

- Identity sites touched by fn-34: `crates/telperion-core/src/params.rs:163` CATALOGUE, `presets.rs:13` enum, `:54` profile_id, `:62` from_id, `:73` parameters, `tests/sweep.rs:16` IDS and `:44` BANDS, `tests/mesh.rs:11`, `tests/species.rs`, `tests/growth_reference.rs`, `tests/identity.rs` PINS, `examples/measure.rs`, `tests/browser/bindings.mjs:129`, `src/browser/core.ts`, `src/index.ts`, six specimen unit-test preset loops (`branching/specimen/*tests.rs`, `contacts.rs`), `src/browser/presets.generated.ts` via `scripts/build-wasm.mjs`.
- The sweep walks every pair: `tests/sweep.rs:97-100` `pairs()` over `IDS`, ten steps each (`:24`).
- The beech binding fixture patch: `tests/browser/bindings.mjs` sets a 6 m envelope for `european-beech` because a 4 m envelope is under two 2.2 m internodes.
- `presets/species.rs` (fn-34) is 122 lines for two species; `presets.rs` 328; `presets/materials.rs` 217.
- `crown_reference.rs` is an ignored FN6 pin list that excludes the oak and the spruce already; it is not an identity site.

## Requirement coverage

| Requirement | Task |
|---|---|
| R1–R2 | the one implicit task (direct route) |
