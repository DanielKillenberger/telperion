# The species runner

`species <id>` takes a seeded manifest to the owner's look in eight stages.
Each stage writes the artifacts the next one reads, and each records the
content hash of every file it read. A stage reruns when one of those files
changes or one of its own outputs is gone, and never otherwise, so a second
run with nothing changed reruns nothing. The code is
`crates/telperion-jev/src/runner`.

```sh
cargo build --release -p telperion-jev --bin species
bash -ic 'target/release/species date-palm'
```

The runner builds the three render tools itself (`species_measure`,
`geometry_benchmark`, `headless`, in release) from the checkout it runs in,
so a revision never draws with a binary from an older commit. The key must be
visible to an interactive shell (`docs/typesafe.md`).

| Flag | Default |
|---|---|
| `--dir DIR` | `.flow/evidence/<id>/pipeline`: the manifest, decisions, resolutions and every stage artifact |
| `--run-dir DIR` | `--dir`: the fetch cache, the ledger and the stills; the runner's own files go to `<run-dir>/runner/` |
| `--tuning FILE` | `.flow/evidence/<id>/tuning.json`: the tuning config (below) |
| `--catalogue DIR` | `catalogue` |
| `--adapter` | `firecrawl`; `fixture:DIR` for pinned sources |
| `--accept` | the owner accepts the tree they looked at |

## The stages

| Stage | Runs | Writes |
|---|---|---|
| Sources | discover, fetch (`docs/species-pipeline.md`) | `discover.json`, `fetch.json`, the admitted manifest |
| Profile | extract, screen, quality, select, verify, fit; search-again while a requirement has a round left, then the profile again | `packet/profile.json`, `packet/references.json` |
| Capability | the gate: the host's `packet/capability.json` against the generator's vocabulary | `gate.json` |
| Catalogue | generate, the gate's seed audit, document | `packet/species.json`, `packet/specimens.json`, `catalogue/<id>/ARTICLE.md` and source copies |
| Start | the profile's values mapped onto dials (`data/profile-to-preset.json`) | `runner/start.json` |
| Tune | one tuning revision (`docs/tuning-loop.md`) | `runner/tuning/<n>/`, `runner/tuning/result.json` |
| Gaps | every trait still failing, classed | `runner/gaps.json`, `runner/gaps.md` |
| Accept | the owner's look; with `--accept`, the tree as a value table | `runner/accepted.json` |

Every stage's word goes to `runner/log.jsonl`, with each open decision the
run logged and did not wait on.

## Stops

A run stops for three things only, and prints `STOPPED:` with the reason:

- **A claim.** `claim-contradicted`, `claim-unsupported` and their article
  kinds: a person settles each in `resolutions.json`
  (`docs/species-pipeline.md`, "Decisions"), and the next run reruns what
  reads the resolutions.
- **An identity gap.** `gaps.md` lists a trait the species is not
  recognisable without. It waits until its spec lands, which rebuilds the
  tools and reruns Tune, or until the host reclasses it in
  `packet/capability.json`.
- **The owner's look.** The owner looks at the tuned tree in the harness and
  runs `species <id> --accept`. An acceptance names the tree's key, so a later
  revision's tree waits for a look of its own, and it is refused while
  `scripts/catalogue-check.mjs` fails the species' catalogue folder or does
  not run.

Every other condition is rerun or logged. A stage that fails names itself and
leaves no record, so the next run tries it again.

## The tuning config

The config at `--tuning` is authored once per species with the manifest: the
preset, the profile manifest and id the measurer reads, the references and
required cells, the reviewer adapters, the tracks and the dial ids the run
tunes. The runner fills the rest per revision:

- `initial_overrides` is the last kept tree's overlay, or `start.json`'s on
  the first revision. The config's own `initial_overrides` are the person's
  entries and win over a derived value in Start.
- `dials` are the rows of the dial table compiled into the runner that the
  config names, all of them when it names none. A row the table no longer
  has is dropped and named in the log.

## Gaps

Code classes each failing trait from what the run recorded:

- **reachable**: a live dial moved it during tuning; the line names the dial
  and the two values it was drawn at.
- **identity**: no capability assessment at all, a missing capability the
  assessment classes identity or leaves unclassed, or a failing trait no dial
  moved.
- **global**: a capability the assessment classes an improvement, or a trait
  the config lists unexpressed, with the specs that capture it.

The host reviews `gaps.md` and writes every spec; the runner mints none.
