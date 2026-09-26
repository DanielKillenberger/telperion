# Species template pipeline: literature to preset

The pipeline takes a taxon from everything written about it to a species
packet, a provenance sidecar, fitted growth curves, a decision list and the
species' documentation. The species runner (`docs/species-runner.md`) runs its stages
in process, in the order below, and passes nothing but paths. Every
reading step is a Jev question through `crates/telperion-jev`, every
arithmetic step is code, and every choice that belongs to a person lives in
the manifest a person admits or in a decision a person resolves.

## The runbook

Run from the repository root with the key available to an interactive shell
(`bash -ic '...'`, see `docs/typesafe.md`). `species <id>` runs the stages
below inside its Sources, Profile, Capability and Catalogue stages
(`docs/species-runner.md`); `DIR` is the folder that holds `manifest.json`
and every stage artifact, and `RUN` the run directory that holds the scratch
a run leaves behind: the fetch cache, the ledger, the command log and
rendered stills.

| Runner stage | Pipeline stages, in order |
|---|---|
| Sources | gather, fetch |
| Profile | read, aggregate, fit |
| Capability | gate |
| Catalogue | generate, gate again over the specimens generate wrote, document |

Every command reads the manifest and the earlier artifacts at fixed paths
under `DIR` and writes one artifact atomically to `DIR/<stage>.json`. Gather lists every source the
repository already knows, the catalogue's bibliographies first, before it
searches the web, so a source the repository has already verified is never
rediscovered; `--catalogue DIR` names the catalogue and defaults to
`catalogue`. A command whose idempotence key
(input checksums, manifest checksum, question-set versions, model name, tool
versions) matches the artifact on disk prints `current` and does nothing. The
key reads content, never the build: a new binary over unchanged inputs reruns
nothing. A stage that stops names the stage or the decision that stopped it.
`--adapter fixture:DIR` replaces Firecrawl with pinned fixtures.

## The capability assessment

The pipeline's eleven commands are mechanical. Deciding what the generator
cannot express is not, so the capability assessment is a **host step**, never
a driver's and never a stage: reasoning and system design escalate to the host
(`AGENTS.md`, routing). The pipeline only checks its output.

**It runs before the literature stages.** After the manifest is admitted and
before `fetch`. A species whose form the field cannot draw is then parked for
the cost of reading two files, instead of after the whole literature chain.
Adding an engineering row keeps the admission and reruns nothing, so writing
the assessment's result does not reissue the manifest proposal.

Each round reads the species spec's architectural model and organs, the habit,
element and attachment traits in `crates/telperion-core/src`, and the
generator's capability vocabulary below. For every trait the species needs it
records one of three outcomes: a value the trait space reaches, a value the
trait space cannot reach, or a structure no trait expresses
(`unsupported-anatomy`, the disposition `docs/species-onboarding.md` names).
A need an open spec already covers is recorded as depending on that spec, not
as a new gap. A judgment the host is unsure of is recorded as unsure and never
decided; an unsure item routes to the owner rather than into a gap.

The round writes the unmet names to `manifest.json` under
`engineering.required_capabilities` and its reasoning to
`DIR/packet/capability.json`, which is a list of rounds, not a single record.

### The vocabulary, and what the gate does with it

The vocabulary is `crates/telperion-core/src/capability.rs`: one declared list
of what the generator expresses, each name carrying the one line that says
what the name means, owned by no preset. A name the generator cannot express
is absent from that list and waits beside it under `UNEXPRESSED`, so an
assessment can state the need before the need can be met. The spec that gives
the generator a capability moves its line from the one list to the other, in
one line, with a test that failed before the implementation and passes after;
that move is how a later round sees the world change.

`geometry_benchmark --vocabulary` prints it: the version the round records,
then every declared name with its one line and the list it sits in. A round
reads the vocabulary from that command rather than from the source file, so
the version it writes is the one the build it assessed against declares.

The `gate` stage compares the required set against that list, and `gate.json`
records `vocabulary_version`, the digest of the declared names, beside
`required`, `expressed`, `missing` and `unrecognised`. A required name in
neither list is unrecognised, which is its own report rather than missing:
nobody has said what the name means. The version is the one a round records,
so a round and the gate that followed it can be compared. The gate fails
closed, so a species whose packet and manifest name no required capability has
had no assessment, and that is unresolved rather than met.

A registered preset is asked a second question, and it is not this one:
whether its own value table produces what the species requires of it.
`geometry_benchmark --support <preset>` reads that off the shipped values for
the six names it has a threshold for, and the gate records the answer under
`capability.preset` and files `preset-capability` when the table falls short.
It never contributes to `missing`, and an unregistered preset is not asked at
all. The empty answer it used to give was read as a missing capability: on
2026-09-19 the date palm's gate reported all six of its required names
missing, `woody-axes` among them, which every preset draws.

### Rounds, because one assessment is never the last

A landed gap fix changes what the generator expresses, so it can reveal a gap
the previous round could not have seen: the palm's frond rosette is invisible
while the frond itself is missing. The assessment is therefore a loop, and
`gap resume` already drives it, since a landing expires the key of the halted
stage and every stage after it, so `gate` reruns and the assessment reruns
before it.

Each round records the round number, the vocabulary version it assessed
against, the landing that triggered it, the required set, the unmet set, the
specs each unmet item depends on, and the unsure items.

**The convergence rule.** A round is legitimate when it either shrinks the
unmet set, or names a capability the landing itself revealed. A round that
does neither, the same unmet set twice, or a set that grows without a landing
to explain it, is the owner's: stop and say so. This is the rule fn-34's
twenty-three rounds cost us, written down.

**The budget is three rounds.** A fourth is refused, the way the third value
round is refused. Each round costs a landed generator spec, so the budget is
not about compute; it is the point at which the owner decides whether this
species is still worth the generator work it is asking for.

### A late gap owes two accounts

The specs land, the run resumes, and a gap is still there. That is allowed, and
it is the case the loop exists for, but a round that names a gap the earlier
rounds did not is the one that can spin forever: there is always one more thing
to find. So every round after the first records two accounts, and a round that
cannot give both stops with the owner whatever else it found.

**Why it was missed.** One of three, named, with the landing or the vocabulary
change that explains it:

- `revealed-by-landing`: the earlier round could not have seen it, because the
  capability it depends on did not exist. The palm's frond rosette is invisible
  while the frond itself is missing.
- `vocabulary-gained-a-term`: the generator's capability list grew a name that
  lets the need be stated at all. The need was always there; there were no
  words for it.
- `earlier-assessment-erred`: the earlier round could have seen it and did not.

The third is not refused, and hiding behind one of the first two is worse than
admitting it. But it is counted, and it is the assessment's own quality signal
rather than the species': a species accumulating more than one of these is
telling you the assessment method or the tier running it is wrong, not that the
species is difficult. A run whose late gaps are all `revealed-by-landing` has
an assessment that is working.

**Why the next step is worth taking.** What the species has cost so far, in
rounds used and specs landed; what is still unmet and what each remaining item
would take; and the case for continuing, which usually rests on whether the
remaining capabilities serve other species or only this one. The owner reads
this at the budget, and it is the only thing that makes the third round a
decision rather than a reflex. A species whose remaining gaps serve nothing
else is the one to park.


## Decisions

A stage that meets a choice a person owns files a decision in
`DIR/decisions.json` with a stable id (`species/stage/kind/field/age`), a
payload per kind, the input checksums it was issued under, and the stages it
stops while open. A person writes `DIR/resolutions.json`:

```json
{"resolutions": [{"id": "oregon-white-oak/curve-fit/tolerance-miss/dbh_m/26.7",
  "inputs_sha256": {"fetch.json": "..."}, "option": "reject",
  "by": "owner", "at": "2026-09-18", "note": ""}]}
```

A resolution binds only while its checksums match the decision's; a stale one
is void and the decision reopens.

One kind carries options a stage consumes. A resolution to it with an option
outside its list is refused when the next stage reads it, naming the kind and
the options that are consumed; the stage that acted on a resolution records
itself as `consumed_by` on the decision.

| Kind | Options | Consumed by |
|---|---|---|
| `coverage-gap` | `accept-rows`, `fix-table`, `drop-table` | fetch: `accept-rows` keeps the rows as parsed, `fix-table` reads the table entry the manifest now admits (and stops if the count still differs), `drop-table` records the table with no rows |

Other kinds: `missing-curve`, `tolerance-miss`, `onboarding-gate`,
`level-miss`, `no-reference`, `visual-unassessed`, and the article's claims.
Nothing before the values files a decision (fn-157): a document is gathered
without admission, an unreadable one is dropped with its reason, and a value
is composed from every document rather than chosen from one.

## Sources and tables

Gather (fn-157) collects what the repository already knows about this
species, then searches once for the species as a whole: the web search over a
fixed set of plain-word queries that name where a tree's literature lives
(floras, silvics and forestry manuals, arboreta, extension pages) and the
research index. What the repository knows is the sources its own
bibliography under `catalogue/<species>/sources.json` holds, and the sources
every manifest of the same species or taxon under `.flow/evidence` names,
with any fetch error their run recorded (`.flow` is the tree the run
directory sits under, else the one under the working directory the runbook
runs from). Another species' sources, the specs' method references and
anything under a `raw/` directory are never known (owner, 2026-09-25). Every
document found, one per address and at most sixteen, joins the manifest as a
source `P<n>`; a known source carrying a fetch error is listed and never
added. A seed that states the species' native range
(`taxon.native_range`: `{"region": "Europe", "names": {"de": "Rotbuche"}}`)
adds that region's floras and forestry literature and the species under its
names in the range's own languages, two documents a query beyond the sixteen
(host decision, 2026-09-26). Nothing is ranked, admitted or refused before it is read: reading a
document for its facts needs no licence, only a copy does (below,
"Documentation").

Wikipedia is a lead, never a citation (owner, fn-82 and 2026-09-25). A
tertiary encyclopedic page, known by its host (`wikipedia.org`,
`wikiwand.com`, `britannica.com`, `encyclopedia.com`,
`newworldencyclopedia.org`, `dbpedia.org`, subdomains included), is never
added or fetched. Gather reads its reference section and adds up to eight of
the primary sources it cites (silvics literature, forestry tables, floras,
papers) in its place. A tertiary source already in a manifest is recorded
under `dropped` in `fetch.json` and never read, so no profile value can cite
one. The rule is code (`pipeline::leads`), never a judgment. Wikimedia
Commons is a photograph host, not a citation, and the reference photographs
may come from it (`docs/species-runner.md`, "Reference photographs").

A rate limit is not a missing source. Every call the runner makes goes
through `adapter::Retrying`: a call refused by a rate limit waits the delay
the error names (ten seconds when it names none, never more than a minute)
and tries again, up to four tries, and once the provider has reported its
per-minute limit the calls are paced to stay under it. Only a permanent
failure, or a limit that outlasts every try, drops the document, its error
recorded under `dropped` in `fetch.json`.

An admitted table names its markdown table by `table_index` and, when one
markdown table packs several species under label rows (a name in the first
cell, every other cell empty, as Firecrawl parses the Ertragstafeln extract),
the `block` that opens its rows:

```json
{"id": "E1-ash-height-I", "table_index": 5, "block": "Esche", "expected_rows": 11,
 "dimension": "height_m", "unit": "m", "value_column": 1,
 "condition": "stand_grown", "taxon": "Fraxinus excelsior"}
```

The rows run from the label row to the next label row or the table's end.
Fetch files `coverage-gap` when the parsed count differs from `expected_rows`
in either direction (fewer is a flattened table, more is a merged one) and
when the block label is not there, naming the labels the table carries.

The plain request that records a source's raw bytes trusts the host's
certificate store, so a page Firecrawl scraped is not refused over a chain
the bundled roots lack; a page the host store also rejects is dropped with
the TLS error verbatim.

## Values

A species' values are the confident aggregate of everything its documents
say (fn-157). Read reads each fetched document once. Jev classes the
document's kind from its address, title and a few of its sentences: a flora
or monograph, a forestry or silvics manual or yield table (a woodland body's
account of the tree in its native woods among them), a botanical garden's or
arboretum's page, a university extension page, a nursery's, landscape
designer's or retailer's page, or another kind, with `unclear` its no-match
answer. Code finds every number-and-unit span above zero that states a length
in a sentence naming one of the manifest's fields, each occurrence on its own
(at most forty a document), and Jev labels each, marked in its sentence, with
the field it states or `none`, its basis (typical, a record, one specimen, a
cultivar, unclear), the age (mature, at a stated age, young) and the growing
condition. Jev never chooses between sources and never supplies a number.

Aggregate composes each field in code (`pipeline::agree`). A span counts when
it is labelled the field, of a grown tree, under the field's condition or an
unstated one; code parses its number and unit, and sets it aside when the
words beside it name another dimension ("Leaf Length: 3-6 inches" labelled a
width). Pages of one site are one source, and each source's typical spans
are one point, the median of their midpoints. Sources rank by their
document's kind, in the order above (host decision, 2026-09-26): the value
comes from the best tier holding two independent points within a factor of
1.5 of each other (`agreed`). A lower tier fills a field only when no tier
agrees and no better tier holds a value (`thin`). Within the deciding tier a
point beyond a factor of 2 of the tier's median is set aside and noted once
the tier holds three. The bounds are named constants, and
`data/cases/aggregate.json` is the labelled set they must answer. The value is
the median of the points left, the range their extent; a record or a single
specimen is the field's `maximum`, never its value. A field no document states
typically is `unsourced`: the generator's default stands and Tune sets it from
the photographs. A value gates only when its deciding tier agrees (host,
2026-09-26): an `agreed` value is `gating`; a `thin` one, one source or
sources that do not agree, is `contextual`: Start derives from it, Tune may
move it, and it never makes a baseline infeasible. `classified` says which
and why. The metric keeps the profile's shape, so Tune and Gaps read
it unchanged; it adds `value`, `tier`, `tiers` (the independent sources each
tier held), `sources_agreeing`, `spread_ratio`, `maximum` and `set_aside`, and
the sidecar keeps every span behind a value with its sentence and ledger
reference. Start takes `value`, the median, never the middle of the range.
An appearance trait that names no source reads the documents of the best tier
whose text carries the trait's words, and no other.

## Documentation

`document` runs after the packet is written, and it is
what leaves a species legible to a person and reachable by an agent without a
second fetch. It writes one markdown copy per fetched source from the fetch
cache, then the species article, then re-runs the citation check over the
article's own sentences.

The repository is public, so a source's rights decide the copy's shape. An
explicit permitting statement - a named open licence or a public-domain
statement - gets the full markdown. A gathered document has one only when
its page declares an open Creative Commons deed in its markup (a
`rel="license"` link or a rights meta tag); a licence named only in its text
is as often a photograph's credit, and gets none (fn-157). Everything else
gets the spans the profile cites, each a verbatim run of the fetched text quoted under its
provenance pointer, which is quotation rather than republication. Silence and
ambiguity are not permission: `scripts/catalogue-check.mjs` classifies
conservatively and fails a full copy it cannot justify, naming the source.

`ARTICLE.md` is prose with a citation on every claim. Code owns every number:
the measurement table is rendered from `packet/profile.json` with each range,
unit, standing and note as the record holds them, and the check fails a number
in authored prose that appears in no packet record. A written article cannot
be regenerated byte for byte, so it is gated on staleness instead - its front
matter records the sha256 of every record and source copy it was written from,
and the check fails it when one has moved. The article is then rewritten, not
patched. A sentence the cited source does not support becomes an open
`article-claim-unsupported` decision; it does not ship. The add-species agent
writes the prose when the acceptance names the article, and the cite check
verifies it (`docs/species-runner.md`, "The article").

## Cost

Every artifact's header carries `cost`: the runs that wrote it, the Jev calls
it made and the Firecrawl credits it spent, counted from the CLI's
`creditsUsed` where a response prices itself and estimated at one credit per
call where it does not, with the method named.

## Artifacts

| Path | Schema | Written by |
|---|---|---|
| `manifest.json` | manifest v1 | a person |
| `gather.json` | gather v1 | gather, which also adds its documents to `manifest.json` |
| `fetch.json` | sources v1 | fetch |
| `read.json`, `aggregate.json`, `fit.json`, `gate.json`, `generate.json`, `document.json` | one schema each, v1 | the stage of that name |
| `sources/<source id>.md` | front matter `{source, url, title, attribution, rights, fetched, sha256, source_sha256, form}`; a full copy where the rights permit one, the cited passages where they do not | document |
| `ARTICLE.md` | the sources distilled, a citation on every claim | document |
| `packet/profile.json`, `packet/references.json` | fn19 closed records | aggregate |
| `packet/species.json`, `packet/specimens.json` | fn19 closed records | generate |
| `provenance.json` | provenance v1, keyed by JSON Pointer | aggregate, generate |
| `decisions.json`, `resolutions.json` | decisions v1 | every stage; a person |
| `RUN/command-log.json` | command-log v1 | document, for the catalogue scripts it runs |
| `RUN/ledger/entries/`, `RUN/ledger/index.json` | fn-57 ledger entries; identity index | the caller |
| `RUN/cache/` | source bytes, markdown, measurement scratch; ignored by git | fetch, generate |
| `RUN/stills/` | rendered stills, reproducible from the pins; ignored by git | generate |

Every artifact is canonical JSON (sorted keys, compact, one trailing
newline), so two runs over the same inputs are byte-identical. The sidecar
and the decisions carry chosen options, levels, copied values and ledger
identities, never probabilities; the probabilities live in the ledger entries.

## The question sets

The versioned sets under `crates/telperion-jev/data/questions`: the read
stage's document kind and its label (field, basis, age, condition) over a
marked span, described
level scoring over levels a person wrote, and the rights class of a
photograph's licence. Their labelled cases with negative and held-out entries
live under `data/cases` (the label's are the recorded beech pages and the
oak's silvics); `jev cases` reruns them live and fails when a held-out
accuracy is below 0.9, listing the missed case ids. `--only labelled|pipeline` runs one family alone, so a set being tuned costs one family's calls.
