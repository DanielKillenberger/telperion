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
to `DIR/manifest.json` and resolves it.

Kinds: `manifest-proposed`, `unavailable-source`, `coverage-gap`,
`data-insufficient`, `claim-contradicted`, `claim-unsupported`,
`obligation-unmet`, `structural-unmet`, `missing-curve`, `tolerance-miss`,
`onboarding-gate`, `level-miss`, `no-reference`, `visual-unassessed`.

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
