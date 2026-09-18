# The species catalogue as data

## Conversation Evidence

> user (2026-09-14): "should we actually spec a template/pipeline that would allow contributors to add trees as templates and improve the generator to make it possible to generate all trees in the world. If this is streamlined we could just recursively have grok add trees and update the generator?"
> user (2026-09-14): "i mean the goal should be all trees some time in the future. But ideally we just have a spec per tree species and have that follow a template"
> user (2026-09-18): "should we actually have a database for species documentation. Document store/wiki or smth that follows certain rules/structure => the species ingestion pipeline is much easier to consistently run. Also easier to update docs and not have them lost."
> user (2026-09-18): "but i'd have that catalogue at a level outside of flow. Evidence in flow should refer to that wiki/catalogue. It should be at the root of the repo."
> user (2026-09-18): "how can we have images stored there also to use as references? probably not fit to be in the repo directly?" / "we'd ideally have images in there"
> user (2026-09-18): "is there also a way to browse this catalogue that fits this structure? for humans i mean" / "aye"
> user (2026-09-18): "does fn-35 need planned? can we simplify it? 11 reqs seems alot" / "ok let's do the split as you proposed"

## Goal & Context
<!-- scope: business -->
<!-- Goal & Context: 50% [user], 35% [paraphrase], 15% [inferred] -->

The owner wants one structured home for everything known about a species, so the pipeline runs the same way every time and a record is updated in place instead of lost. The first pipeline run (fn-56, 2026-09-18) showed the repository has no such home. Species knowledge is filed by spec number in four places (fn19 for the oak and spruce bibliography, fn34 for the ash, beech and birch packets, fn26, fn29 and fn32 for further reference files, and an evidence folder named after the ash for the pipeline run), in three record formats for one fact (the fn19 reference record, the profile with its dimension definitions, the manifest source with its checksum and table blocks). The ash manifest cites "fn-34 ash profile" as the rationale for its envelope height, a pointer that breaks when fn-34's evidence is superseded. Discovery searched the web before finding the yield-table source that fn-34's reference file already named, which cost six driver dispatches on a run planned for one. [user]

This spec puts the catalogue at the root of the repository, outside Flow, one folder per species, holding the bibliography, the admitted manifest, the packet, the provenance, the decisions, the pins the tests read, the reference images the project may keep, and the owner's notes. Flow evidence holds runs and refers to the catalogue; it never duplicates a species record. A code check owns the folder's structure, and a generated page per species lets a person browse it on GitHub or in Obsidian with no server. [user]

The registry refactor that makes a species one Rust file and stops the twelve hand-maintained identity sites is the dependent spec fn-77-the-species-registry-one-file-per; it reads the pins file this spec places. The species spec template, the one-command script and the value-tier routing line landed in PR #29 and are not restated here. [paraphrase]

## Architecture & Data Models
<!-- scope: technical -->

- **The catalogue is a root directory, outside Flow.** `catalogue/<id>/` is the source of truth for one species and the only place a species record lives. It holds `sources.json` (the bibliography, one schema for every source), `manifest.json` (the admitted pipeline manifest), `packet/` (the profile, references, species and specimens records the onboarding protocol defines, unchanged), `provenance.json`, `decisions.json` and `resolutions.json` from the pipeline, `pins.json` (the identity pins, leaf band and growth reference the tests read), `stills.json` (rendered stills by hash with the owner's verdict), `refs/` (reference images the project may redistribute), `NOTES.md` (the owner's verdicts and tuning notes, the one free-text surface a person writes) and `README.md` (generated, never written by hand). Runs stay under `.flow/evidence/<spec>/`: ledger, fetch cache, driver logs, rendered stills, friction. A run refers to the catalogue by species id and copies nothing out of it. [user]
- **A code check owns the catalogue's structure.** One test walks every species folder and fails one that is missing a required file, a required field, a source without rights text, a kept image without a hash, an image whose bytes do not match its recorded hash, a raster outside `refs/`, or a README that differs from its regeneration, naming the species and the field. Whether a source says what the packet claims stays a Jev question under the fn-58 obligations. [paraphrase]
- **Pointers run toward the catalogue.** A manifest's engineering rationale cites source ids from `sources.json`, never a spec number. Pipeline discovery reads every `catalogue/*/sources.json` before any web or research search, which is where fn-75's discovery seed (its R4) looks once both land, in place of walking admitted manifests across the evidence tree. The species runner (fn-72) finds a preset's reference records in its catalogue folder. The pipeline's stages write their fixed-path artifacts into the species folder; no stage logic changes. [paraphrase]
- **Reference images live in the catalogue under Git LFS.** `catalogue/<id>/refs/` holds the images whose rights allow the project to keep a copy (the owner's own photographs, and licensed images with attribution recorded), tracked as LFS objects by a `.gitattributes` rule on that path so a clone stays small and a checkout pulls them on demand. Each image is a reference record in `packet/references.json` with its sha256, rights, attribution, access date and the shot block fn-36 defines, so a matched still can be rendered against it. An image whose rights do not allow a copy is recorded by URL, hash and access date only; the fetch adapter caches it in the ignored cache directory and the runner refuses to compare against a file that is absent or does not match its hash. Rendered stills never enter the catalogue; they are reproducible from the pins and stay in the ignored evidence directory with their hashes in `stills.json`. [user]
- **Humans browse a generated page, never an authored one.** `scripts/catalogue-pages.mjs` renders `catalogue/<id>/README.md` from the species folder (taxon and context from the packet, the architectural model, the bibliography with rights, the pins and growth reference, the kept reference images inline, the stills table with the owner's verdicts, and `NOTES.md` quoted verbatim) and `catalogue/README.md` as the index, one row per species with its model and verdict status. GitHub renders the folder as a page with LFS images inline, Obsidian opens the directory as a vault, and GNO can index it as a collection; none of them needs a server. The harness view that shows the tree beside its references is a later spec. [user]
- **Existing records move into the catalogue once.** The fn19 oak and spruce references, the fn34 ash, beech and birch packets, and the ash run's manifest, packet, provenance and decisions move to their species folders in one commit that changes no value; the evidence folders they leave keep a one-line pointer to the new path. The fn34 report, rounds and trial tables stay where they are as run history. The oak and the spruce get folders even though their presets stay in `presets.rs`, because their fn19 records are the oldest species knowledge in the repository. [paraphrase]
- **Pins move out of test source.** `pins.json` per species carries the identity pins, the leaf band and the growth reference height by age that `identity.rs`, `sweep.rs` and `growth_reference.rs` hold in source today. This spec places the files and adds a test that each shipped species' file matches the numbers still in source; fn-77 switches the tests to read the file and deletes the source copies. [inferred]

## API Contracts
<!-- scope: technical -->

- **Species folder** `catalogue/<id>/` with required files `sources.json`, `manifest.json`, `packet/profile.json`, `packet/references.json`, `packet/species.json`, `packet/specimens.json`, `provenance.json`, `decisions.json`, `resolutions.json`, `pins.json`, `stills.json`, `NOTES.md`, `README.md`, and an optional `refs/` directory. A species that predates the pipeline (oak, spruce, beech, birch) carries `manifest.json`, `provenance.json`, `decisions.json` and `resolutions.json` as `{"schema": "<name>", "schema_version": 1, "empty": true}` until a pipeline run fills them. [inferred]
- **Source record** in `sources.json`, one schema for every source: `id`, `url`, `title`, `attribution`, `rights`, `sha256` of the fetched bytes, `verified` (date), `use` (what the source is good for, in words), and `tables` (the admitted table records fn-75 defines, with `block`, `expected_rows`, `dimension`, `unit`, `value_column`, `condition`, `taxon`). The fn19 reference schema and the manifest source record are both rendered from this file; neither is authored by hand again. [inferred]
- **Reference image record** in `packet/references.json`, the frozen fn19 reference schema plus `kept: true|false`; a kept image has `path` under `refs/` and `asset_sha256`, an unkept one has `url`, `asset_sha256` and `access_date` only. [inferred]
- **Pins file** `pins.json`: `pins { wood_vertices, wood_triangles, instances, min, max, skeleton, placement, element }`, `leaf_band [min, max]`, `growth_reference { age, height_m }`, `profile_sha256`. [inferred]
- **Structure check** `node scripts/catalogue-check.mjs`, on the workspace test commands; the failure message is `catalogue/<id>: missing <file>` or `catalogue/<id>/<file>: <field> <reason>`. It also runs the page generator in memory and reports `catalogue/<id>/README.md: differs from regeneration`. [inferred]
- **Page generator** `node scripts/catalogue-pages.mjs` writes `catalogue/README.md` and every `catalogue/<id>/README.md`; it takes no arguments and is idempotent. [inferred]
- **LFS rule** `.gitattributes`: `catalogue/**/refs/** filter=lfs diff=lfs merge=lfs -text`. [inferred]

## Edge Cases & Constraints
<!-- scope: technical -->

- **The move changes no value.** The record move is one commit; a diff of any moved JSON against its old path, keys sorted, is empty. [paraphrase]
- **Source bytes stay outside the repository.** The catalogue holds checksums, extracted spans and table rows, never a fetched page or PDF; fn-58's rule stands. The one exception is a reference image the project may redistribute, which is an LFS object with its rights recorded. [paraphrase]
- **LFS is a local and a hosting constraint.** `git-lfs` is not installed on the owner's machine today, which is a local setup step and not this spec's work. GitHub Free grants 10 GiB of LFS storage and 10 GiB of bandwidth a month; at about half a megabyte a photograph and ten photographs a species, three hundred species is 1.5 GiB. CI checkouts pull LFS objects only in the job that renders matched stills, so the workspace tests spend no bandwidth. [inferred]
- **Rasters already in history stay.** master carries 507 committed rasters, 236 MB, most under `experiments/fn9-iterations` and fn19's evidence. This spec rewrites no history; it stops the growth. A raster committed under `catalogue/` outside `refs/` fails the structure check. [paraphrase]
- **Stills ignored, hashes committed.** `.gitignore` covers the evidence stills directories; a still whose sha256 does not match `stills.json` is not evidence. [paraphrase]
- **A hand-edited README fails the check** the same way a stale one does; the notes file is where a person writes. [inferred]
- **No generator, renderer or preset change.** The pins test added here reads numbers that exist; it moves none. [paraphrase]
- **fn-58 landed, fn-75 in flight.** This spec changes the paths the pipeline stages read and write and adds nothing to a stage's logic. It rebases over fn-75 on landing, and fn-75's discovery seed reads the catalogue once both are on master. [inferred]

## Acceptance Criteria
<!-- scope: both -->

- **R1:** Every shipped species (oak, spruce, ash, beech, birch) has a folder under `catalogue/` with the required files, and the fn19, fn34 and fn-56 records live there with their old locations pointing at them; the structure check passes on every folder and is on the workspace test commands. Errors: a missing file or field, a source without rights, a kept image without a matching hash, or a raster outside `refs/` fails the check naming the species and the field. [user]
- **R2:** A reference image the project may keep is an LFS object under `catalogue/<id>/refs/` with its record in `packet/references.json`, and `npm run species:qa` renders its matched still against it after `git lfs pull`; an image the project may not keep is recorded by URL and hash and the runner refuses to compare against an absent or mismatched file, naming the reference id. Errors: a missing `.gitattributes` rule for `catalogue/**/refs/**` fails the structure check. [user]
- **R3:** Every species folder carries a generated `README.md` and the catalogue an index page, rendered by `scripts/catalogue-pages.mjs` with `NOTES.md` quoted verbatim, kept reference images inline and the stills table with verdicts; the pages render on GitHub and open in Obsidian with no server. Errors: the structure check fails naming a page whose committed bytes differ from its regeneration, and a hand-edited page fails the same way. [user]
- **R4:** The pipeline's stages write their artifacts into the species folder, discovery lists every source in `catalogue/*/sources.json` as a candidate before any web search, the species runner reads a preset's reference records from its folder, and each shipped species' `pins.json` matches the pins, band and growth reference still held in test source. Errors: a stage writing outside the species folder, or a pins file that differs from source, fails a test naming the species. [paraphrase]

## Boundaries
<!-- scope: business -->

- No species onboarded here; species specs follow from the template. [paraphrase]
- No registry refactor; the twelve identity sites, the sampled sweep and the derived binding fixture are fn-77's. [user]
- No wiki and no database service; the catalogue is files in git with schemas and a code check, and the browsable view is generated. [user]
- No harness view; a catalogue route that renders the direct build beside its references is its own spec. [user]
- No architectural-model coverage file; the 23-model list is its own small spec when a species spec first needs to name an unsupported model. [user]
- No change to a pipeline stage's logic, question set, threshold or value table; fn-75 owns the discovery and fetch fixes. [paraphrase]
- No history rewrite for the rasters already committed. [inferred]

## Decision Context
<!-- scope: both -->

### Motivation

- The owner wants species documentation in one structured store so the pipeline runs the same way every time and a record is updated in place rather than lost; the first ash run spent six dispatches partly on knowledge the repository already held. [user]

### Implementation Tradeoffs

- A root directory over Flow evidence: a species outlives every spec that touched it, and a spec's evidence folder is a run's history. Putting the record at the root makes the spec number an annotation on the record instead of its address. [user]
- Files in git over a wiki or a database: the pipeline reruns stages on input checksums and canonical JSON, git gives the history and the PR is the review gate, and the dispatched Codex and Cursor workers can read a file where they cannot reach a wiki. A wiki page has no schema, no hash and no test. [paraphrase]
- Git LFS over plain commits or an external bucket for reference images: plain commits already put 236 MB of rasters in the clone; a bucket needs credentials on every worker and CI job. LFS keeps the image beside its record, versioned, with the bytes pulled only where a still is rendered. [inferred]
- A generated README over an authored wiki page: the records are the truth and the page is a view; a drift check keeps the two equal without a person remembering to update either. [user]
- Two specs over one: the catalogue is scripts and JSON, the registry is a Rust refactor with a byte-identity gate; the owner found eleven requirements too many for one task, and the catalogue is what unblocks the pipeline today. [user]

## Parked unknowns

- Whether the root directory is named `catalogue` or `species`; the spec uses `catalogue` to match its own title.
- Whether the `.git/flow-artifacts` store, 20 GB on the owner's machine today, should hold rendered stills instead of the ignored evidence directories. Out of scope here; noted because it is where the local disk goes.

## Strategy Alignment

- Follows "The catalogue": every species as a value table over a supported form, one spec from a template, the value tier implementing.
- Follows "The core and integration": one lean core keeps a new template accessible with no renderer change.

## Resolved via Codebase

- Species records today: `.flow/evidence/fn19/references.json` and `final/sources.json` (oak, spruce), `.flow/evidence/fn34/{european-ash,european-beech,silver-birch}/{profile,references,species}.json`, `.flow/evidence/fn26/references.json`, `fn29`, `fn32`, and the fn-56 run at `.flow/evidence/european-ash/pipeline/` on the pipeline-run branch (manifest, discover, fetch, extract, screen, quality, select, verify, fit, gate, provenance, decisions, resolutions, packet, ledger, cache, six driver logs).
- The ash manifest's engineering rows cite "fn-34 ash profile reference_height_by_age" and "the fn-34 ash profile's dbh_m range" as rationale.
- fn-75 R4 makes discovery walk every admitted manifest in the evidence tree for known sources; with a catalogue that walk is one glob over `catalogue/*/sources.json`.
- Pins in source today: `tests/identity.rs` PINS, `tests/sweep.rs:44` BANDS, `tests/growth_reference.rs`.
- master as of 2026-09-18 carries 507 rasters, 236 MB: 276 under `experiments/fn9-iterations/qa-preview` (92 MB), 76 under `.flow/evidence/fn19` (54 MB), the rest across fn9, fn24, fn26, fn27, fn29, fn30, fn31, fn32, fn34 and fn55 evidence.
- `templates/species-spec.md`, `scripts/new-species-spec.mjs` and the CLAUDE.md routing line landed in `2f628da2` (PR #29).
- `git lfs` is not installed on the owner's machine; the repository has no `.gitattributes`.

## Requirement coverage

| Requirement | Task |
|---|---|
| R1–R4 | the one implicit task (direct route) |
