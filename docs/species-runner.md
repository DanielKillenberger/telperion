# The species runner

`species <id>` takes a seeded manifest to the owner's look in eight stages.
Each stage writes the artifacts the next one reads, and each records the
content hash of every file it read. A stage reruns when one of those files
changes or one of its own outputs is gone, and never otherwise, so a second
run with nothing changed reruns nothing. The code is
`crates/telperion-jev/src/runner`.

```sh
cargo build --release -p telperion-jev --bin species
bash -ic 'target/release/species date-palm'                 # every stage
target/release/species date-palm --status                   # what each would do
bash -ic 'target/release/species date-palm --until profile' # stop after a stage
bash -ic 'target/release/species date-palm --stage start'   # one stage alone
```

`--status` prints each stage as `current`, `stale` (with the inputs that
changed) or `missing` (never ran, or an output is gone), then preflights the
paid services: the Jev key is visible, Firecrawl answers (`firecrawl
--status`), and the tuning config's vision adapter makes its smallest call
(the `probe` stage of `scripts/reference-first.py`, no image). It prints what
each stage that would run is expected to spend, and exits 1 when a check
fails. It runs no stage and builds nothing. `--until <stage>` runs every stage up to and including it.
`--stage <stage>` runs that stage alone when it is not current; it refuses,
naming the file and the stage that writes it, while a file an earlier stage
writes for it is missing.

The runner builds the three render tools itself (`species_measure`,
`geometry_benchmark`, `headless`, in release) from the checkout it runs in,
at the first stage that reads them (Capability, Catalogue and Tune), so a
revision never draws with a binary from an older commit and a run that stops
before them never builds. The key must be visible to an interactive shell
(`docs/typesafe.md`).

| Flag | Default |
|---|---|
| `--dir DIR` | `catalogue/<id>`: the manifest, decisions, resolutions and every stage artifact, which the catalogue scripts read in place |
| `--run-dir DIR` | `.flow/evidence/<id>/run`: the fetch cache, the ledger and the stills; the runner's own files go to `<run-dir>/runner/` |
| `--tuning FILE` | `.flow/evidence/<id>/tuning.json`: the tuning config (below) |
| `--catalogue DIR` | `catalogue` |
| `--adapter` | `firecrawl`; `fixture:DIR` for pinned sources |
| `--accept` | the owner accepts the tree they looked at |
| `--record DIR` | keep every external answer in `DIR` (below, "Record and replay") |
| `--tools DIR` | draw with render tools already built in `DIR`, never build them |
| `--replay DIR` | serve every external answer from `DIR`, with no network and no key |
| `--settle-claims` | a claim the search could not settle stops the run for a person instead of the runner settling it |

## Record and replay

`--record <dir>` keeps every external answer a run receives in `<dir>`,
keyed by a stable hash of its request: Firecrawl searches, scrapes and PDF
parses (`firecrawl/`), Jev calls (`jev/`, the key never recorded), the
Wikimedia Commons API and image bytes (`web/`), and every vision adapter
reply (`adapter/`, through `scripts/tape-adapter.py`, which wraps each
adapter program the configs name). A key ignores every `path`, so a replay
from another directory finds the same answer. `--replay <dir>` serves them
back with no network and no key and fails, naming the request, on any the
recording lacks. The layer is `crate::tape`; the stages take their adapter,
transport and adapter programs through it and have no second path. A
recording holds the renders' bytes in its adapter keys, so a generator change
that moves a render needs the tuning part recorded again.

The proof of the runner is a recorded run (owner, 2026-09-25): the beech,
recorded live from a bare seed and replayed in the workspace gate through
Start by `crates/telperion-jev/tests/replay.rs`, with no network and no key,
and a second replay reruns nothing. The recording is
`tests/fixtures/replay/european-beech`. A defect a later live run finds is
recorded as a case beside it. The repository is public, so a recording is
trimmed before it is committed (`tape_trim <dir>/tape`, then `--check`): a
page that is not openly licensed keeps only the passages the run quoted to
Jev and its licence statements, and every recorded answer is filed under
the key the current tape computes. A key ignores a fetched page's own
`sha256` and `bytes` beside its `url`, which a trimmed page changes. `--tools <dir>` draws with render tools already
built there (the gate's own examples) instead of building them.

Within one run, a file a later stage writes that an earlier stage reads (the
resolutions Profile settles claims into, which Sources reads) leaves the
earlier stage current; an edit between runs still reruns it.

## The stages

| Stage | Runs | Writes |
|---|---|---|
| Sources | discover, fetch (`docs/species-pipeline.md`); an unadmitted proposal is skipped and an unreadable source dropped, both logged | `discover.json`, `fetch.json`, the admitted manifest |
| Profile | extract, screen, quality, select, verify, fit; search-again while a requirement or a flagged claim's field has a round left, then the profile again; once the rounds are spent, each claim settled by the runner (below); the reference photographs (below) and their inventory | `packet/profile.json`, `packet/references.json`, `runner/references.json`, `runner/inventory/<hash>/` |
| Capability | the gate: the host's `packet/capability.json` against the generator's vocabulary; it runs before Catalogue, which generates from it | `gate.json` |
| Catalogue | generate, the gate's seed audit, the records no stage writes, document (the article scaffold and the cite check over it), the pages | `packet/species.json`, `packet/specimens.json`, `sources.json`, `stills.json`, `NOTES.md`, the `pins.json` stub, source copies, `ARTICLE.md`, `README.md` |
| Start | the profile's values mapped onto dials (`data/profile-to-preset.json`) | `runner/start.json` |
| Tune | one tuning revision (`docs/tuning-loop.md`) | `runner/tuning/<n>/`, `runner/tuning/result.json` |
| Gaps | every trait still failing and every unsourced field, classed | `runner/gaps.json`, `runner/gaps.md` |
| Accept | the owner's look; with `--accept`, the tree into core as the species' preset and its pins | `crates/telperion-core/presets/<id>.values` (and `presets.rs` for a new species), `pins.json`, `stills.json`, `runner/accepted.json` |

Every stage's word goes to `runner/log.jsonl`, with each open decision the
run logged and did not wait on.

## Claims

A value the citation check flags (`claim-contradicted`,
`claim-unsupported`) goes to the search while its field has a round left.
Once the rounds are spent the runner settles it itself and logs it:

- **Contradicted measurement: the range the sources span.** Select keeps
  the field as the range from the lowest to the highest of each source's
  most probable span, parsed by code, with every such source cited
  (`keep-range`). Start takes its midpoint and Tune narrows it.
- **Anything else: unsourced.** The value is dropped (`drop-value`); the
  profile marks the field `unsourced`, `gaps.md` lists it, and the
  generator's default stands until Tune sets it from the photographs.

Jev never supplies a number: the range is code's parse of the spans the
sources state. A resolution a person writes in `resolutions.json` still wins
(`docs/species-pipeline.md`, "Decisions"). With `--settle-claims` the runner
settles none of them and the run stops on the open claims, the article's
included, for a person. Without it, an open article claim is logged.

## The article

The document stage scaffolds `ARTICLE.md` and the cite check verifies every
claim it makes. The prose is the add-species agent's: when `--accept` is
refused because `scripts/catalogue-check.mjs` fails the article, the agent
writes it from the folder's source copies, then runs
`species <id> --stage catalogue`, whose cite check reads the written article
(its bytes are an input of the stage and key the document stage), and hands
the owner the look again.

## Stops

A run stops for two things, and prints `STOPPED:` with the reason:

- **An identity gap.** A capability the species needs and the generator
  cannot express stops the run at the Capability stage (evidence in
  `gate.json`); a trait tuning could not move stops it at Gaps (`gaps.md`). It waits until its spec lands, which rebuilds the
  tools and reruns Tune, or until the host reclasses it in
  `packet/capability.json`.
- **The owner's look.** The owner looks at the tuned tree in the harness and
  runs `species <id> --accept`. Accepting refreshes the folder's pins and
  stills, and writes the tree into its preset value file,
  `crates/telperion-core/presets/<id>.values`, with core's writer: a shipped
  species keeps its file's lines and comments, with each moved row's line set
  or added under the acceptance's note; a new species gets a file of every
  row off the default family and its
  registration in `presets.rs`. It is refused, with core untouched, while
  `scripts/catalogue-check.mjs` fails the folder or does not run. An
  acceptance names the tree's key, so a later revision waits for a look of
  its own.

Every other condition is rerun or logged. A stage that fails names itself and
leaves no record, so the next run tries it again. A vision adapter's failure
carries the adapter's own words (a spent Claude quota reads as "You've hit
your weekly limit", a replay's missing answer as `replay: ...`).

## The tuning config

The config at `--tuning` is authored once per species with the manifest: the
preset, the profile manifest and id the measurer reads, the required cells,
the reviewer adapters, the tracks and the dial ids the run tunes. Its
`references` may be an empty list: the run then compares against the
photographs the Profile stage found. A first revision from a name needs no
`owner_notes`; a revision refuses only what cannot run: no live dial, no
fixed and fresh seed among the required cells, or no reference photograph of
the whole tree. The runner fills the rest per revision:

- `measure_binary` and `matched.headless` are the tools the runner just
  built, `references` are the config's own or else the found ones
  (`runner/references.json`), and `reference_first` is the inventory the
  Profile stage built of them.
- `initial_overrides` is the last kept tree's overlay, or `start.json`'s on
  the first revision. The config's own `initial_overrides` are the person's
  entries and win over a derived value in Start.
- `dials` are the rows of the dial table the parameter catalogue generates
  (`tuning::table`), that the config names, all of them when it names none. A row the table no longer
  has is dropped and named in the log; a config that names its dials at a table revision is refused
  once the table has moved.

## Reference photographs

No run from a name needs a person to supply photographs (owner,
2026-09-25). Once the profile is settled, the Profile stage finds them
(`pipeline::photos`) unless `packet/references.json` already records two:

1. **Candidates**, at most twelve: up to two images from each admitted
   open-licence source's page (four in all), then Wikimedia Commons, asked
   for the tree (six), its bark (three) and the tree in winter (three).
   Commons is a photograph host; the no-Wikipedia rule is about citing
   values.
2. **Rights.** A Commons file whose machine-readable licence code
   (`extmetadata.License`) is CC0, public domain, CC BY or CC BY-SA is open
   by that code. Any other file's full licence metadata (`License`,
   `LicenseShortName`, `UsageTerms`, `LicenseUrl`, `AttributionRequired`,
   `Artist`) goes to Jev's rights question set. The question weighs a page's
   text and sets a photograph's licence aside as a figure credit, and it
   classed eleven of the beech's twelve CC files "none" (2026-09-25). Only
   `open-licence` goes on. An admitted source's image takes its source's
   class.
3. **Copies.** Code downloads each JPEG or PNG into the run's cache,
   named by its sha256.
4. **One look.** The reviewer adapter in the tuning config (`vision`,
   `scripts/reference-first.py`, stage `screen`) looks at the whole batch
   once and says, per photograph, whether it shows the species, a mature
   open-grown tree and the whole tree, and its view: leaf-on, bare, bark or
   other.
5. **Kept**: up to two whole trees in leaf, one bare, one bark close-up,
   appended to `packet/references.json` with their source (`R<n>` for a
   Commons file), attribution, licence and sha256. A recorded reference is
   never removed or rewritten.

When none is kept and the tuning config lists none, the Profile stage still
completes: it skips the inventory and writes a `references` line to
`gaps.md`, which stops nothing. Tune refuses, saying so, until a photograph
is recorded in the config or a later Profile run finds one.

The cost is bounded: three free Commons queries, one Jev rights call per
Commons candidate without an open licence code (twelve at most) and one
vision call. The counts and the
look's token usage go to `<run-dir>/cache/photos/find.json`.

## Gaps

Code classes each failing trait from what the run recorded:

- **reachable**: a live dial moved it in a rendered attempt the reviewer
  judged; the line names the dial, the two values it was drawn at and links
  the renders of both sides.
- **identity**: no capability assessment at all, a missing capability the
  assessment classes identity or leaves unclassed, or a failing trait no dial
  moved.
- **global**: a capability the assessment classes an improvement, or a trait
  the config lists unexpressed, with the specs that capture it.
- **unsourced**: a profile field no source settled (above, "Claims"); the
  generator's default stands and Tune sets it from the photographs.
- **references**: no reference photograph to compare against (above,
  "Reference photographs"). It stops nothing; Tune refuses until one is
  recorded.

The host reviews `gaps.md` and writes every spec; the runner mints none.
