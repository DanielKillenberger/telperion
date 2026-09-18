# Species template pipeline: literature to preset

The pipeline takes a taxon from admitted sources to a species packet, a
provenance sidecar, fitted growth curves, a decision list and a report. A
driver runs the commands below in order and passes nothing but paths. Every
reading step is a Jev question through `crates/telperion-jev`, every
arithmetic step is code, and every choice that belongs to a person lives in
the manifest a person admits or in a decision a person resolves.

## The runbook

Run from the repository root with the key available to an interactive shell
(`bash -ic '...'`, see `docs/typesafe.md`). `DIR` is the species' pipeline
directory, `.flow/evidence/<species>/pipeline`, holding `manifest.json`.

```sh
cargo build --release -p telperion-jev
cargo build --release -p telperion-core --example species_measure --example geometry_benchmark
cargo build --release -p telperion-render --example headless
P=target/release/species-pipeline
$P discover --dir DIR
$P fetch    --dir DIR
$P extract  --dir DIR
$P screen   --dir DIR
$P quality  --dir DIR
$P select   --dir DIR
$P verify   --dir DIR
$P fit      --dir DIR
$P gate     --dir DIR --example
$P generate --dir DIR --example --profile-id <profile id>
$P report   --dir DIR
```

Every command reads the manifest and the earlier artifacts at fixed paths
under `DIR`, writes one artifact atomically to `DIR/<stage>.json`, and
appends itself to `DIR/command-log.json`. A command whose idempotence key
(input checksums, manifest checksum, question-set versions, model name, tool
versions) matches the artifact on disk prints `current` and does nothing. A
command that stops prints the stage or the decision that stopped it; the
driver resolves nothing and reads nothing. `--adapter fixture:DIR` replaces
Firecrawl with pinned fixtures for the model-swap test.

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

Discovery lists what the repository already knows before any search (`.flow`
under the working directory, the repository root the runbook runs from): the
sources every admitted manifest under `.flow/evidence` names, with the
dimensions their tables cover and any fetch error their run recorded, and the
URLs the specs cite under `## Resolved via Research`. They enter Jev's ranking
as candidates of kind `known` with their origin marked, never admitted by
being known, and a known candidate carrying a fetch error is listed and never
proposed. The search query is the field in plain words (`Fraxinus excelsior
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

## Cost

Every artifact's header carries `cost`: the runs that wrote it, the Jev calls
it made and the Firecrawl credits it spent, counted from the CLI's
`creditsUsed` where a response prices itself and estimated at one credit per
call where it does not, with the method named. The report sums them per
stage and in total under `costs` and in its `## Cost` table.

## Artifacts

| Path | Schema | Written by |
|---|---|---|
| `manifest.json` | manifest v1 | a person |
| `discover.json` | discover v1 | discover |
| `fetch.json` | sources v1 | fetch |
| `extract.json` | candidates v1 | extract |
| `screen.json`, `quality.json`, `select.json`, `verify.json`, `fit.json`, `gate.json`, `generate.json`, `report.json` | one schema each, v1 | the stage of that name |
| `packet/profile.json`, `packet/references.json` | fn19 closed records | select |
| `packet/species.json`, `packet/specimens.json` | fn19 closed records | generate |
| `provenance.json` | provenance v1, keyed by JSON Pointer | select, generate |
| `decisions.json`, `resolutions.json` | decisions v1 | every stage; a person |
| `command-log.json` | command-log v1 | the driver binary |
| `ledger/entries/`, `ledger/index.json` | fn-57 ledger entries; identity index | the caller |
| `report.md` | rendered from `report.json` | report |
| `cache/` | source bytes, markdown, measurement scratch; ignored by git | fetch, generate |

Every artifact is canonical JSON (sorted keys, compact, one trailing
newline), so two runs over the same inputs are byte-identical. The sidecar
and the decisions carry chosen options, levels, copied values and ledger
identities, never probabilities; the probabilities live in the ledger entries.

## The model-swap test

`species-pipeline swap --left DIR_A --right DIR_B` compares the packet, the
sidecar, the decision list and the report of two runs byte for byte, reports
every difference per artifact and per JSON Pointer, and fails a trial whose
command log holds a command outside the runbook. The two runs use isolated
directories, the fixture adapter over pinned sources, one admitted manifest,
and one recorded model name. The test reaches the network for Jev and is
never part of the workspace test commands.

## The question sets

Four versioned sets under `crates/telperion-jev/data/questions`: source
ranking per field, data sufficiency per field with its dominant gap, described
level scoring over levels a person wrote, and the semantic obligations
(`inspected_image`, `measurement_not_invention`). Their labelled cases with
negative and held-out entries live under `data/cases`; `jev cases` reruns them
live and fails when a held-out accuracy is below 0.9 (0.8 top-one agreement
for ranking), listing the missed case ids.
