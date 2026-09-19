# Pipeline fixes from the first ash run

> HTML render lens: `.flow/artifacts/fn-75-pipeline-fixes-from-the-first-ash-run/spec.html` (gitignored — open locally; regenerable, markdown is the record). <!-- flow-next:artifact-link -->

## Conversation Evidence

> user (2026-09-18): "ok merged so can we now have a test run with a cheap model adding a new species right?"
> user (2026-09-18): "the loop should identify gaps also at the end and propose specs to glose the gaps. These should escalate to frontier models if jev cannot judge the likelihood of the proposed gap fix actually fixing the gap. So having a species that already needs a spec should not be a blocker for the loop. The loop should fire off specs to close the gaps"
> user (2026-09-18): "make sure to keep notes on how to improve this loop"
> user (2026-09-18): "so i was hoping that we'd define a tree species iterate with jev, judge what specs need to be added to cover gaps to render the tree and then this way hill climb to a tree that passes the QA model viewing the tree visual gate so i can evaluate it at the end. Only to stop if the path isn't clear or needs a high stakes decision that can't be decided by frontier model who's orchestrating"
> user (2026-09-18): "what's spec A?"
> user (2026-09-18): "i'm not following pls explain this without mannered prose for stupid human"
> user (2026-09-18, after the plain-language list of the four defects and the question "Do you want me to write it up as a spec?"): "yes /flow-next:capture"

## Goal & Context
<!-- scope: business -->
<!-- Goal & Context: 30% [user], 50% [paraphrase], 20% [inferred] -->

The owner wants the species pipeline to run under a cheap driver from a seed manifest to a report with a person stepping in only for the choices a person owns, so that a species can be defined, iterated with Jev, and hill-climbed toward a tree that passes a visual gate, stopping only when the path is unclear or a high-stakes decision is due. [paraphrase] The first live run on 2026-09-18, the European ash under a Grok driver, reached the onboarding gate but needed six driver dispatches instead of two and cost 107 Firecrawl credits, because four defects in the pipeline's own bookkeeping made a person babysit it. [paraphrase] This spec fixes those four defects and two smaller ones the same run exposed. It changes no value table, no generator behaviour and no question set; it makes the pipeline honor the choices a person already made and read the sources the repository already has. [paraphrase]

The defects, in the owner's plain words: the pipeline forgets that the manifest was approved and asks again; some decision options are offered but nothing acts on them; a yield table that packs several species into one markdown table is read as one species, and the guard only catches tables with too few rows; discovery searches the web before looking in the repository's own notes. [paraphrase] The friction entries behind each are recorded in the ash run's evidence and are the source for the criteria below. [inferred]

## Architecture & Data Models
<!-- scope: technical -->

- **The manifest proposal is keyed on the seed, not the whole manifest.** Discovery's idempotence key covers the fields a person seeds (taxon, evidence fields, conditions, required ages) and not the sources, curves or engineering rows a person later admits, so admitting the manifest does not rerun discovery. The manifest-proposed decision binds its resolution to the admitted manifest's checksum, so an admission stays valid across the edit that admits it. [paraphrase]
- **Stages consume the resolutions of their own decisions.** Each decision kind names the stage that acts on each option. For unavailable-source: retry fetches again, drop-source skips the source and records it as dropped in the fetch artifact, replace-source reads the replacement URL from the resolution's payload and fetches it under the same source id. A resolution the naming stage has consumed is recorded on the decision. [paraphrase]
- **An admitted table names its block.** A table record carries, beside its index, the label of the row that opens its block (the species-name row Firecrawl emits between species in one markdown table); the row parser starts at that row and stops at the next label row or the table's end. The coverage guard files coverage-gap when the parsed row count differs from the stated count in either direction. [paraphrase]
- **Discovery seeds from what the repository knows.** Before any web or research search, discovery lists the sources every admitted manifest in the evidence tree already names and the URLs cited in the specs' research sections, fetches nothing, and puts them in the candidate list Jev ranks; the search queries are written in plain words from the field's reading, not from the field id. [paraphrase]
- **The raw fetch trusts the system's certificates.** The adapter's plain request for raw bytes uses the host's certificate store, so a page Firecrawl can scrape is not refused by the raw request over a chain the bundled roots do not carry. [inferred]
- **The report carries cost.** Each stage's artifact records the Firecrawl credits and Jev calls it spent, and the report sums them per stage. [inferred]

## API Contracts
<!-- scope: technical -->

The admitted table record gains one field; the fields shown are the contract.

```json
{"id": "E1-ash-height-I", "table_index": 5, "block": "Esche", "expected_rows": 11, "dimension": "height_m", "unit": "m", "value_column": 1, "condition": "stand_grown", "taxon": "Fraxinus excelsior"}
```

A resolution the naming stage consumed is marked on the decision:

```json
{"id": "european-ash/fetch/unavailable-source/M1", "status": "resolved", "consumed_by": "fetch", "resolution": {"option": "drop-source", "by": "owner", "at": "2026-09-18"}}
```

The decision kinds table states, per kind, which stage consumes which option: manifest-proposed (admit, reject: every stage after discover), unavailable-source (retry, replace-source, drop-source: fetch), coverage-gap (accept-rows, fix-table, drop-table: fetch), data-insufficient (admit-proxy, add-sources, lower-bar: quality, by editing the manifest fields only). [inferred]

## Edge Cases & Constraints
<!-- scope: technical -->

- **A seed edit still reruns discovery.** Changing a field, a condition or a required age changes the seed key and discovery runs again with a new proposal; adding a source, a curve or an engineering row does not. [paraphrase]
- **A block label that does not appear** in the parsed table files coverage-gap naming the label and the labels found, never a silent empty table. [inferred]
- **A replaced source keeps its id** so provenance entries written under the old URL stay valid; the fetch artifact records both URLs. [inferred]
- **Seeded candidates are ranked like searched ones.** A repository source enters Jev's ranking as a candidate with its origin marked; it is never admitted by virtue of being known. [paraphrase]
- **Credits are counted from the adapter's responses** where the CLI reports them and estimated from the call count where it does not, with the method named per line. [inferred]
- **Nothing here reaches generation, rendering or presets**; the fn-57 isolation guard stays green and the workspace tests keep the key unset. [paraphrase]

## Acceptance Criteria
<!-- scope: both -->

- **R1:** Admitting the manifest after discovery's proposal does not rerun discovery and does not reissue the manifest-proposed decision; on the ash run's recorded seed and admitted manifest, the driver reaches fetch on the first dispatch after admission. [paraphrase] Errors: a seed change reruns discovery and reissues the proposal, and the driver's stop names the new decision.
- **R2:** Each of retry, replace-source and drop-source on an unavailable-source decision changes the next fetch run as the option says, and the decision records which stage consumed it; a resolution's option that no stage consumes is refused when written, naming the kind and the options that are. [paraphrase] Errors: a replace-source resolution without a replacement URL is refused.
- **R3:** An admitted table with a block label yields only that block's rows; the ash's Esche block in the Ertragstafeln extract yields exactly eleven rows from 20 to 120 years, and the coverage guard files coverage-gap when the parsed count differs from the stated count in either direction, including 31 against 11. [paraphrase] Errors: a block label not found in the table files coverage-gap naming the labels present.
- **R4:** Discovery's candidate list for a field includes every source an admitted manifest in the evidence tree already names for that dimension before any web or research search runs; on the ash's seed, the Ertragstafeln extract is a ranked candidate for height. [paraphrase] Errors: a seeded candidate whose URL no longer resolves is listed with its fetch error and not admitted.
- **R5:** The adapter's raw request succeeds on a page whose certificate chain the system trusts and Firecrawl scraped; the Missouri Botanical Garden page from the ash run fetches. [inferred] Errors: a page the system store also rejects files unavailable-source with the TLS error verbatim.
- **R6:** The report lists Firecrawl credits and Jev calls per stage and in total; on a rerun of the ash from its recorded seed the report shows one discovery. [inferred] Errors: a stage whose cost cannot be read from the adapter reports an estimate marked as such.

## Boundaries
<!-- scope: business -->

- No change to any question set, threshold, value table, preset, generator or renderer behaviour. [paraphrase]
- The gap loop (proposing and minting fix specs at a halt) is fn-63's, the tuning loop fn-68's, and a visual gate is its own spec; none of them is here. [paraphrase]
- A person still admits the manifest and resolves data-insufficient and onboarding-gate decisions; this spec makes those decisions stick, it does not remove them. [paraphrase]

## Decision Context

- The owner asked for the pipeline to run under a cheap model as a test of adding a species, and for notes on how to improve the loop; the notes became this spec after a plain-language reading of the four defects. [user]
- Fixing the pipeline is separate from fn-58 because fn-58 landed on 2026-09-18 and closes on its fn30 validation; rewriting a landed spec would blur what shipped. [paraphrase]

## Requirement coverage

| R-ID | Task |
|---|---|
| R1 | fn-N.M (TBD - populate via /flow-next:plan) |
| R2 | fn-N.M (TBD - populate via /flow-next:plan) |
| R3 | fn-N.M (TBD - populate via /flow-next:plan) |
| R4 | fn-N.M (TBD - populate via /flow-next:plan) |
| R5 | fn-N.M (TBD - populate via /flow-next:plan) |
| R6 | fn-N.M (TBD - populate via /flow-next:plan) |
