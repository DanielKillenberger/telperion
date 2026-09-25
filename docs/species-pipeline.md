# Species template pipeline: literature to preset

The pipeline takes a taxon from admitted sources to a species packet, a
provenance sidecar, fitted growth curves, a decision list and the species'
documentation. The species runner (`docs/species-runner.md`) runs its stages
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
| Sources | discover, fetch |
| Profile | extract, screen, quality, select, verify, fit, and search-again while a requirement has a round left |
| Capability | gate |
| Catalogue | generate, gate again over the specimens generate wrote, document |

Every command reads the manifest and the earlier artifacts at fixed paths
under `DIR` and writes one artifact atomically to `DIR/<stage>.json`. Discovery lists every source the
repository already knows, the catalogue's bibliographies first, as a candidate
before it searches the web, so a source the repository has already verified is
never rediscovered; `--catalogue DIR` names the catalogue and defaults to
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
is void and the decision reopens. The discover stage's `manifest-proposed`
decision stops every later stage until a person writes the admitted manifest
to `DIR/manifest.json` and resolves it. That decision binds to the seed
(species, taxon, the fields with their conditions and required ages), and
discover keys on the seed too: admitting sources, curves, proxies or
engineering rows reruns nothing and keeps the admission, while a seed edit
reruns discover, reissues the proposal and voids the old admission.

Four kinds carry options a stage consumes. A resolution to one of them with
an option outside its list is refused when the next stage reads it, naming
the kind and the options that are consumed; the stage that acted on a
resolution records itself as `consumed_by` on the decision.

| Kind | Options | Consumed by |
|---|---|---|
| `manifest-proposed` | `admit`, `reject` | every stage after discover; a rejected proposal stops them until the seed is edited and discover runs again |
| `unavailable-source` | `retry`, `replace-source`, `drop-source` | fetch: `retry` fetches again, `replace-source` fetches the `url` in the resolution's `payload` under the same source id and records both urls, `drop-source` skips the source and records it under `dropped` |
| `coverage-gap` | `accept-rows`, `fix-table`, `drop-table` | fetch: `accept-rows` keeps the rows as parsed, `fix-table` reads the table entry the manifest now admits (and stops if the count still differs), `drop-table` records the table with no rows |
| `data-insufficient` | `admit-proxy`, `add-sources`, `lower-bar` | quality, by editing the manifest's fields only |

```json
{"id": "european-ash/fetch/unavailable-source/M1", "inputs_sha256": {"url": "..."},
 "option": "replace-source", "payload": {"url": "https://example.test/the-same-page"},
 "by": "owner", "at": "2026-09-18"}
```

Other kinds: `claim-contradicted`, `claim-unsupported`, `obligation-unmet`,
`structural-unmet`, `missing-curve`, `tolerance-miss`, `onboarding-gate`,
`level-miss`, `no-reference`, `visual-unassessed`.

## Sources and tables

Discovery lists what the repository already knows before any search: the
sources every species bibliography under `catalogue/*/sources.json` holds, the
sources every admitted manifest under `.flow/evidence` names, with the
dimensions their tables cover and any fetch error their run recorded, and the
URLs the specs cite under `## Resolved via Research` (`.flow` is the tree the
run directory sits under, else the one under the working directory the runbook
runs from). They enter Jev's ranking as candidates of kind `known` with their
origin marked - `catalogue:<species>#<id>`, `manifest:<path>#<id>` or
`spec:<id>` - never admitted by being known, and a known candidate carrying a
fetch error is listed and never proposed. The search query is the field in plain words (`Fraxinus excelsior
height at age, open grown`), not the field id.

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
the bundled roots lack; a page the host store also rejects files
`unavailable-source` with the TLS error verbatim.

## Documentation

`document` runs after the packet is verified, and it is
what leaves a species legible to a person and reachable by an agent without a
second fetch. It writes one markdown copy per admitted source from the fetch
cache, then the species article, then re-runs the citation check over the
article's own sentences.

The repository is public, so a source's rights decide the copy's shape. An
explicit permitting statement - a named open licence or a public-domain
statement - gets the full markdown. Everything else gets the passages the
packet cites, each a verbatim run of the fetched text quoted under its
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
`article-claim-unsupported` decision a person resolves; it does not ship.

## Cost

Every artifact's header carries `cost`: the runs that wrote it, the Jev calls
it made and the Firecrawl credits it spent, counted from the CLI's
`creditsUsed` where a response prices itself and estimated at one credit per
call where it does not, with the method named.

## Artifacts

| Path | Schema | Written by |
|---|---|---|
| `manifest.json` | manifest v1 | a person |
| `discover.json` | discover v1 | discover |
| `fetch.json` | sources v1 | fetch |
| `extract.json` | candidates v1 | extract |
| `screen.json`, `quality.json`, `select.json`, `verify.json`, `fit.json`, `gate.json`, `generate.json`, `document.json` | one schema each, v1 | the stage of that name |
| `sources/<source id>.md` | front matter `{source, url, title, attribution, rights, fetched, sha256, source_sha256, form}`; a full copy where the rights permit one, the cited passages where they do not | document |
| `ARTICLE.md` | the sources distilled, a citation on every claim | document |
| `packet/profile.json`, `packet/references.json` | fn19 closed records | select |
| `packet/species.json`, `packet/specimens.json` | fn19 closed records | generate |
| `provenance.json` | provenance v1, keyed by JSON Pointer | select, generate |
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

The versioned sets under `crates/telperion-jev/data/questions`: source
ranking per field, data sufficiency per field with its dominant gap, described
level scoring over levels a person wrote, and the semantic obligations
(`inspected_image`, `measurement_not_invention`).
Their labelled cases with negative and held-out entries live under
`data/cases`; `jev cases` reruns them live and fails when a held-out accuracy
is below 0.9 (0.8 top-one agreement for ranking), listing the missed case
ids. `--only labelled|pipeline` runs one family alone, so a set being tuned costs one family's calls.
