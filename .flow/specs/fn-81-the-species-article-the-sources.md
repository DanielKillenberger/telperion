# The species article: the sources distilled, every claim cited

## Conversation Evidence

> user (2026-09-18): "but i can't browse all the info like a human would. i thought we'd copy all the sources also to be able to have them available at an agent's finger tips?"
> user (2026-09-18): "or at least build the profile in more detail from the sources"
> user (2026-09-18): "kinda like grokipedia"
> user (2026-09-18): "that's the better option"
> user (2026-09-18): "that also needs a loop adjustment to always fill this profile documentation yea?"
> user (2026-09-18): "should probably distill the essence from the sources but always link to sources. Keep a copy of the sources in .md format to reference to with original link there also. Makes sense?"

## Goal & Context
<!-- scope: business -->
<!-- Goal & Context: 55% [user], 30% [paraphrase], 15% [inferred] -->

The owner wants to read a species the way a person reads an encyclopedia entry, and wants an agent to reach the source text without fetching it again. fn-35 put the record at `catalogue/<id>/` and generated a page from it, but that page shows the bibliography, the image list and the owner's notes only. The material that would make it worth reading is already in the folder and invisible: every species carries five to seven measured metrics with a range, a source and a confidence level, three anatomy entries, and four to six habit rubric entries written in plain sentences, plus an observation per reference image. [paraphrase]

This spec adds two things to each species folder. A copy of every admitted source in markdown, carrying the original link, so an agent reads what a claim rests on without a fetch. And an article distilled from those sources and the packet, where every claim carries the source it came from. The owner's words: "distill the essence from the sources but always link to sources. Keep a copy of the sources in .md format to reference to with original link there also." [user]

## Architecture & Data Models
<!-- scope: technical -->

- **A source copy is one markdown file, and its rights decide its shape.** `catalogue/<id>/sources/<source-id>.md` opens with the original link, the title, the attribution, the rights text and the fetched bytes' sha256, then carries the source itself. Where the rights permit redistribution the file holds the source's full markdown. Where they do not it holds only the passages the packet cites, each as a quotation with the location it came from, which is quotation rather than republication. The repository is public, so this distinction is not optional: of the seven distinct rights statements the catalogue records today, two permit a full copy (the European Atlas under CC BY 4.0, the US Forest Service publication in the public domain) and five do not, including two journal articles recorded as "no redistribution inferred". [user]
- **The rights field chooses, and the check enforces.** The structure check reads each source's `rights` and fails a full copy filed under a source whose rights do not permit one, naming the source. A source whose rights text the check cannot classify is treated as not permitting a copy. This supersedes fn-58's "source bytes stay outside the repository" for the extract form and narrows it for the full form; the fetch cache stays ignored either way, and no PDF or raw HTML is ever committed. [user]
- **The article is prose with a citation on every claim.** `catalogue/<id>/ARTICLE.md` reads as an encyclopedia entry: what the tree is, how large it grows and by what age, its crown and branching habit, its bark, its leaves, and what the record does not know. Every sentence that makes a claim carries the source id it rests on, rendered as a link to the local copy beside the original URL. A section whose material the records do not hold says so rather than reaching for general knowledge. [user]
- **Numbers are copied, never restated from the writer's memory.** Every measurement in the article is copied from the packet's metrics, its range and unit rendered by code from the record, with the record's own confidence and note beside it. The writer selects and arranges; it never supplies a number. This is the project's standing rule and the article does not weaken it. [paraphrase]
- **The article is checked against its sources before it lands.** fn-57's citation tooling runs over every claim in the article against the source copy it cites, and a claim the source does not support is a decision a person resolves, not a sentence that ships. [paraphrase]
- **The article is not byte-deterministic, so it is gated on staleness instead.** A rendered page is regenerated and compared; a written article cannot be. Its front matter records the sha256 of every record and source copy it was written from, and the structure check fails an article whose recorded inputs no longer match the folder, naming the species and the record that moved. The article is then rewritten rather than patched. [inferred]
- **The onboarding loop fills the documentation, every time.** A documentation stage runs in the pipeline after packet verification and before the report: it writes a source copy for every admitted source, then the article, then re-runs the citation check. The `add-species` route and the onboarding method call that stage as part of a species run rather than leaving it to a later pass, and both docs pages name it. A species reaching the owner's checklist without its documentation is the failure this stage removes. [user]
- **Required files are the enforcement.** `ARTICLE.md` and a `sources/` copy per admitted source join the structure check's required set, so a species folder that lacks them fails the check by construction and no species can land undocumented. The five species already in the catalogue are backfilled in this spec, so the requirement is true the moment it is added. [inferred]
- **The generated page links to both.** `README.md` gains a line linking to the article and a column in the bibliography linking each source to its local copy, so the folder's front page is the way in. [inferred]

## API Contracts
<!-- scope: technical -->

- **Source copy** `catalogue/<id>/sources/<source-id>.md`, front matter `{source, url, title, attribution, rights, fetched, sha256, form}` where `form` is `full` or `extract`. An `extract` file's body is a sequence of quotations, each with the location it came from. [inferred]
- **Article** `catalogue/<id>/ARTICLE.md`, front matter `{species, written, inputs: {<path>: <sha256>}, sources: [<source-id>]}`; body is markdown with a citation on every claim. [inferred]
- **Check additions** to `scripts/catalogue-check.mjs`: a full copy under non-permitting rights fails `catalogue/<id>/sources/<source-id>.md: rights do not permit a full copy`; a stale article fails `catalogue/<id>/ARTICLE.md: <path> changed since the article was written`; a claim with no source id fails naming the line. [inferred]
- **Writer command** `node scripts/catalogue-article.mjs --species <id>` assembles the record material and the source copies, dispatches the writing, and writes the article with its input checksums. [inferred]

## Edge Cases & Constraints
<!-- scope: technical -->

- **A source that cannot be fetched again** keeps whatever copy the catalogue already holds; the article is written from that and the gap is stated. Nothing is invented to fill it. [inferred]
- **A species with no measured metric** gets an article that says the record holds no measurement for that dimension, rather than a general-knowledge range. The ash is the live case, with no preset and a halted run. [paraphrase]
- **The catalogue grows.** It is 380 KB today. Full copies of two rights-clear PDFs in markdown and extracts for the rest put it in the low megabytes at five species, and the growth is text, not rasters. [inferred]
- **No number reaches a preset from this work.** The article is a reading surface; presets keep taking their values from the packet through the pipeline. [paraphrase]
- **The owner's eye is unaffected.** The article judges nothing and carries no verdict; `NOTES.md` stays the owner's surface and the article quotes it nowhere. [inferred]
- **Rights classification is conservative.** The check permits a full copy only on an explicit permitting statement (a named open licence or public domain); silence, ambiguity or "no redistribution inferred" all mean extract. [user]

## Acceptance Criteria
<!-- scope: both -->

- **R1:** Every admitted source of every species has a markdown copy at `catalogue/<id>/sources/<source-id>.md` carrying the original link, title, attribution, rights, fetch date and checksum in its front matter, holding the full source where its rights permit redistribution and the cited passages as quotations where they do not. Errors: a full copy under non-permitting or unclassifiable rights fails the structure check naming the source; a copy whose front-matter checksum does not match its recorded source fails the same way. [user]
- **R2:** Every species has an `ARTICLE.md` distilled from its source copies and packet, covering identity, size and growth, crown and habit, bark, leaves and what the record does not know, with a citation on every claim linking to the local copy and the original URL, and every measurement copied from the packet record with its confidence beside it. Errors: a claim with no source id, or a number that appears in no packet record, fails the check naming the line. [user]
- **R3:** fn-57's citation check runs over every claim in an article against the source copy it cites before the article lands, and an unsupported claim becomes a decision a person resolves rather than a shipped sentence. Errors: an article committed with an open unsupported-claim decision fails the check. [paraphrase]
- **R4:** An article records the checksum of every record and source copy it was written from, and the structure check fails it when any of them has changed, naming the species and the input. Errors: none beyond that failure. [inferred]
- **R5:** A documentation stage runs inside the pipeline after packet verification and before the report, writing every source copy and the article and re-running the citation check; the `add-species` route and `docs/species-onboarding.md` and `docs/species-pipeline.md` name it as part of a species run, and `ARTICLE.md` plus a copy per admitted source are required files in the structure check, backfilled for all five species this spec covers. Errors: a species folder without its article or a source copy fails the structure check naming what is missing, so an undocumented species cannot land. [user]
- **R6:** The species page links to the article and each bibliography row links to its local source copy, and `node scripts/catalogue-article.mjs --species <id>` writes an article end to end for a named species. Errors: an unfilled section or a missing source copy stops the command naming it. [inferred]

## Boundaries
<!-- scope: business -->

- No PDF, raw HTML or image bytes committed; markdown only, and the fetch cache stays ignored. [user]
- No change to any preset, generator, renderer, question set or threshold. [paraphrase]
- No new species onboarded; this spec writes about the five the catalogue already holds and makes the loop fill the sixth. [inferred]
- No change to the pipeline's existing stages beyond adding one after verification; discovery, fetch, screen, select, fit and gate keep their logic. [inferred]
- No verdict, no judgement of a tree, and no edit to `NOTES.md`. [inferred]
- No harness view; a catalogue route that renders the tree beside its references stays its own spec. [paraphrase]
- Not a replacement for the packet. The packet stays the machine-read record and the article is the reading surface over it. [inferred]

## Decision Context
<!-- scope: both -->

### Motivation

- The owner could not browse the catalogue as a person would, and wanted the sources at an agent's fingertips without a re-fetch; asked which shape to build, the owner chose distilled prose with a source copy behind it over storing raw sources alone. [user]

### Implementation Tradeoffs

- Two file shapes over one: the repository is public and five of seven rights statements forbid redistribution, so a single "copy the source" rule would republish two journal articles. The rights field the catalogue already records is what chooses. [user]
- A written article over a rendered one: a render from the records is deterministic and cheap, and the owner judged the distilled prose the better option. The staleness gate is what buys back the honesty determinism would have given. [user]
- Checksummed inputs over regeneration: an article cannot be regenerated byte for byte, so the check asks whether its inputs moved rather than whether its bytes match. [inferred]
- Numbers copied by code: the project's rule that no number reaches a record from a model holds here too, and it is what keeps the article quotable. [paraphrase]

## Parked unknowns

- Whether an article is rewritten wholesale or section by section when one input moves.
- Whether the rights classification should read a licence identifier rather than the rights sentence, once a source carries one.

## Strategy Alignment

- Follows "The catalogue": the species record is the unit, and this makes it legible to a person and reachable by an agent.

## Resolved via Codebase

- The repository is public (`gh repo view`: `"visibility":"PUBLIC"`, github.com/DanielKillenberger/telperion).
- Rights statements across the catalogue today: CC BY 4.0 (J1) and US public domain (USFS-OAK) permit a copy; "no redistribution inferred" (THOMAS-ASH, BALTIC-ASH, KEW-ASH), "extract cited with attribution" (E1) and a bare "cited with attribution" (O1) do not.
- Article material already in each profile: 5 to 7 measured metrics with range, source and confidence, 3 anatomy entries (attachment, branching, element) and 4 to 6 rubric entries (branching habit, crown gaps, crown silhouette and kin); observations sit on each reference record.
- `provenance.json` carries the copied span per filled value with its ledger reference, e.g. `/profiles/0/metrics/height_m` -> span `20-35 m`, source `J1`.
- The citation tooling exists: `crates/telperion-jev/src/cite.rs` and `data/questions/citation.json`.
- `catalogue/` is 380 KB across five species.
- fn-58's rule that source bytes stay outside the repository is the constraint this spec amends with the owner's word.

## Requirement coverage

| Requirement | Task |
|---|---|
| R1–R6 | TBD |
