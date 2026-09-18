# Species template pipeline: literature to preset

The pipeline takes a taxon from admitted sources to a species packet, a
provenance sidecar, fitted growth curves, a decision list and a report. A
driver runs the commands below in order and passes nothing but paths. Every
reading step is a Jev question through `crates/telperion-jev`, every
arithmetic step is code, and every choice that belongs to a person lives in
the manifest a person admits or in a decision a person resolves.

## The runbook

Run from the repository root with the key available to an interactive shell
(`bash -ic '...'`, see `docs/typesafe.md`). `DIR` is the species' catalogue
folder, `catalogue/<species>`, which holds `manifest.json` and every canonical
artifact. `RUN` is the run directory, `.flow/evidence/<spec>/pipeline`, which
holds the scratch a run leaves behind: the fetch cache, the ledger, the command
log and rendered stills. Without `--run-dir` the two are one directory, which
is what a test and a swap trial use.

```sh
cargo build --release -p telperion-jev
cargo build --release -p telperion-core --example species_measure --example geometry_benchmark
cargo build --release -p telperion-render --example headless
P=target/release/species-pipeline
D="--dir DIR --run-dir RUN"
$P discover $D
$P fetch    $D
$P extract  $D
$P screen   $D
$P quality  $D
$P select   $D
$P verify   $D
$P fit      $D
$P gate     $D --example
$P generate $D --example --profile-id <profile id>
$P report   $D
```

Every command reads the manifest and the earlier artifacts at fixed paths
under `DIR`, writes one artifact atomically to `DIR/<stage>.json`, and
appends itself to `RUN/command-log.json`. Discovery lists every source the
repository already knows, the catalogue's bibliographies first, as a candidate
before it searches the web, so a source the repository has already verified is
never rediscovered; `--catalogue DIR` names the catalogue and defaults to
`catalogue`. A command whose idempotence key
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
| `gaps/<slug>/gap.json`, `rounds.json` | gap v1, rounds v1 | the gap loop |
| `metrics.json` | metrics v1 | `gap metrics` |
| `decisions.json`, `resolutions.json` | decisions v1 | every stage; a person |
| `RUN/command-log.json` | command-log v1 | the driver binary |
| `RUN/ledger/entries/`, `RUN/ledger/index.json` | fn-57 ledger entries; identity index | the caller |
| `report.md` | rendered from `report.json` | report |
| `RUN/cache/` | source bytes, markdown, measurement scratch; ignored by git | fetch, generate |
| `RUN/stills/` | rendered stills, reproducible from the pins; ignored by git | generate |

Every artifact is canonical JSON (sorted keys, compact, one trailing
newline), so two runs over the same inputs are byte-identical. The sidecar
and the decisions carry chosen options, levels, copied values and ledger
identities, never probabilities; the probabilities live in the ledger entries.

## The gap loop

A stage that files an `onboarding-gate` or a `level-miss` has met a **gap**: a
capability the species needs that no value table reaches. Every other decision
kind stays inside the run. The loop turns that halt into a reviewed generator
spec and resumes the run where it stopped; `.claude/skills/add-species` is the
conductor that walks it.

```sh
P=target/release/species-pipeline
$P gap open    --dir DIR --decision ID
$P gap options --dir DIR --decision ID --author agent|stronger --model NAME --options FILE
$P gap route   --dir DIR --decision ID [--verdicts FILE]
$P gap reroute --dir DIR --decision ID
$P gap spec    --dir DIR --decision ID --spec SPEC
$P gap review  --dir DIR --decision ID --verdict ship|needs-work
$P gap resume  --dir DIR --decision ID --commit SHA [--pin-note NOTE]
$P gap round   --dir DIR --species S --verdict V [--note N]
$P gap accept  --dir DIR --species S --verdict V
$P gap metrics --dir DIR --species S
```

Each gap keeps one record at `DIR/gaps/<slug>/gap.json`, the value rounds live
in `DIR/rounds.json` and the run's three numbers in `DIR/metrics.json`. Only
`gap route` reaches Jev.

### Options

An option file is `{"options": [...]}` or a bare list of two to four entries.
The agent or the stronger reasoning model writes them; Jev never does.

```json
{"gap": "silver-birch/gate/onboarding-gate/capability",
 "option": "curtain-rows", "change_kind": "generator|value_table|appearance",
 "touches": "twig layer", "moves_pin": true,
 "changes_preset_output": ["norway-spruce"], "serves_species": ["silver-birch"],
 "reversible": true, "summary": "What changes, in one or two sentences.",
 "spends_captures": false, "lowers_bar": false, "changes_boundary": false}
```

The last three default to false and are the owner signals beyond the contract's
fields. An empty set from the agent routes to the stronger model; an empty set
from the stronger model too routes to the owner. The stronger model never
writes the first set.

### The route

`gap route` asks one Jev request over the set: per option its change kind, any
prior owner verdict for or against it, whether it generalizes and whether it
moves a pin; over the set, which option best answers the capability, with
`none` as the no-match answer. Code reads the answers into the record's
signals, and `crates/telperion-jev/data/gap-routes.json` names the route from
them. The table's rows are tried in order and the first whose conditions all
hold wins; the five owner signals sit first, so they override the spread.

| Route | Meaning |
|---|---|
| `proceed` | A clear winner inside the loop's remit. The loop resolves the `gap-fix` decision itself. |
| `stronger` | No winner, or an irreversible generator change: the stronger model writes the set. A set it wrote that routes here again is the owner's. |
| `owner` | A pin moves, another preset's output changes, a capture budget is spent, a data-quality bar is lowered, a Boundary changes, or a verdict already decided against it. The `gap-fix` decision is filed open. |

Every routed gap records its judgments, its signals, its route, the row that
matched and the table version, so `gap reroute` re-reads a changed table
against the recorded signals with no new call. A signal missing from a record
routes to the owner by rule, never to the catch-all row. Thresholds are data:
they are set from the labelled cases and tuned from the owner's reversals, and
no threshold is a constant in code.

### Resume

The chosen fix is minted as its own spec that the species spec depends on,
worked under the repo's review, and never applied inside the species run. A
gap spec reviewed `needs-work` twice files a `gap-review` decision for the
owner. `gap resume` records the landing as a tool version, `fix:<spec>` at its
commit, which enters the idempotence key of the halted stage and of every
stage after it: those rerun, the earlier ones stay current. A fix whose option
moves a pin lands only with `--pin-note` naming the preset, the change and the
reason (fn-53).

### Rounds and the numbers

`gap round` opens one value round on a verdict; the table's
`rounds_per_verdict` bounds them at two, and the third is refused and filed as
a `value-rounds` decision for the owner. `gap metrics` writes `metrics.json`
beside the report: the share of gaps the loop decided itself, the rounds each
verdict took to accept with the owner's reversals by decision id, and the
tokens, wall clock, Jev calls, Firecrawl credits and captures the run spent.
A reversal is recorded, never counted a failure: it is what the next threshold
tuning reads. The report reads that record: a run whose every decision is
resolved but whose numbers are not written is `incomplete`, not `complete`,
and its page names the missing record. `metrics.json` is one of the report's
inputs, so writing the numbers expires the report's key and it runs again.

### The labelled cases

`data/questions/gap.json` is the versioned question set and
`data/cases/gap.json` its labelled cases, the fn-34 gaps with the owner's
actual choices as the labels, a no-match case among them and a third held out.
`jev cases` scores them live beside the other sets and fails when change kind
or prior-verdict coverage falls below 0.9 held out, or best match below 0.8,
listing the missed case ids.

## The model-swap test

`species-pipeline swap --left DIR_A --right DIR_B` compares the packet, the
sidecar, the decision list and the report of two runs byte for byte, reports
every difference per artifact and per JSON Pointer, and fails a trial whose
command log holds a command outside the runbook. The two runs use isolated
directories, the fixture adapter over pinned sources, one admitted manifest,
and one recorded model name. The test reaches the network for Jev and is
never part of the workspace test commands.

## The question sets

Five versioned sets under `crates/telperion-jev/data/questions`: source
ranking per field, data sufficiency per field with its dominant gap, described
level scoring over levels a person wrote, the semantic obligations
(`inspected_image`, `measurement_not_invention`), and the gap loop's options.
Their labelled cases with negative and held-out entries live under
`data/cases`; `jev cases` reruns them live and fails when a held-out accuracy
is below 0.9 (0.8 top-one agreement for ranking and for the gap set's best
match), listing the missed case ids. `--only labelled|pipeline|gap` runs one
family alone, so a set being tuned costs one family's calls.
