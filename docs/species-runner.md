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
| `--look` | write the kept tree where the harness opens it and print the URL (below, "The look"); runs no stage |
| `--accept` | the owner accepts the tree they looked at |
| `--record DIR` | keep every external answer in `DIR` (below, "Record and replay") |
| `--tools DIR` | draw with render tools already built in `DIR`, never build them |
| `--replay DIR` | serve every external answer from `DIR`, with no network and no key |
| `--extend DIR` | serve what `DIR` holds and record what it lacks, so a stage change asks only its new questions live |
| `--settle-claims` | an article claim the cite check flagged stops the run for a person instead of being logged |

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

Within one run, a file a later stage writes that an earlier stage reads
leaves the earlier stage current; an edit between runs still reruns it.

## The stages

| Stage | Runs | Writes |
|---|---|---|
| Sources | gather, fetch (`docs/species-pipeline.md`): everything written about the species, read without admission; an unreadable document is dropped and logged | `gather.json`, `fetch.json`, the manifest's gathered sources |
| Profile | read, aggregate, fit: each value the aggregate of every document (below, "Values"); the reference photographs (below) and their inventory | `packet/profile.json`, `packet/references.json`, `runner/references.json`, `runner/inventory/<hash>/` |
| Capability | the gate: the host's `packet/capability.json` against the generator's vocabulary; it runs before Catalogue, which generates from it | `gate.json` |
| Catalogue | generate, the gate's seed audit, the records no stage writes, document (the article scaffold and the cite check over it), the pages | `packet/species.json`, `packet/specimens.json`, `sources.json`, `stills.json`, `NOTES.md`, the `pins.json` stub, source copies, `ARTICLE.md`, `README.md` |
| Start | the profile's values mapped onto dials (`data/profile-to-preset.json`) | `runner/start.json` |
| Tune | one tuning revision (`docs/tuning-loop.md`) | `runner/tuning/<n>/`, `runner/tuning/result.json` |
| Gaps | every trait still failing and every unsourced field, classed | `runner/gaps.json`, `runner/gaps.md` |
| Accept | the owner's look; with `--accept`, the tree into core as the species' preset and its pins | `crates/telperion-core/presets/<id>.values` (and `presets.rs` for a new species), `pins.json`, `stills.json`, `runner/accepted.json` |

Every stage's word goes to `runner/log.jsonl`, with each open decision the
run logged and did not wait on.

## Values

A value is the aggregate of every document that states it (fn-157,
`docs/species-pipeline.md`, "Values"): the median of one point per
independent source of the best kind that agrees (a flora above a forestry
manual, a garden, an extension page, a nursery), its range their extent,
with a record or a single tree kept as the field's maximum and a value far
from its tier set aside and named. Jev classes each document and labels each
span and never chooses between sources; code owns every number. Start takes the typical value, the median, never the middle of the range, and Tune narrows it. A
field no document states typically is `unsourced`: `gaps.md` lists it, and
the generator's default stands until Tune sets it from the photographs. Only
a value a flora, forestry or garden tier agrees on gates; any other is
contextual, and
`gaps.md` lists it beside the unsourced ones for a person to source.
Nothing about a value waits on a person: no claim is filed, searched again
or settled. The article's claims are the cite check's (below); with
`--settle-claims` an open one stops the run for a person, and without it it
is logged.

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
- **The owner's look.** The owner looks at the tuned tree in the harness
  (below, "The look") and runs `species <id> --accept`. Accepting refreshes the folder's pins and
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

## The look

`species <id> --look` writes the tree Tune kept to `harness/looks/<id>.json`,
an ignored path the dev server (`npm run dev`) serves, and prints the URL
that opens it, `http://localhost:5173/?look=<id>&seed=<n>` at the
revision's fixed seed. The file carries the overlay
(`runner/tuning/result.json`, `outcome.current.overrides`), the run, the
revision, the round and the rounds kept, and the two families core makes:
the preset as `headless --preset` builds it and the preset with the
overlay laid over it, as `headless --family` builds it. The harness lays
the overlay over the preset on its own dials and refuses to draw a family
that differs from core's by a single row, so the tree on screen is the
tree `headless --family` draws at the same seed and view. The panel says
where the look comes from and switches between the kept tree and the
preset under it without reloading, and says which of the two the dials
hold, or that a dial, a preset or reset has moved them off both; the seed
box (the run's seed unless the URL names another) and the views work on
both. Accepting stays `--accept`.

A run with no kept tree is refused naming its `result.json`; an overlay row
the preset's family does not have is refused naming its path, with core's
replacement where the catalogue retired it. A look is also a plain
overlay, the JSON `headless --family` takes: a host drops one at
`harness/looks/<name>.json` and opens `?species=<preset>&look=<name>`,
and the harness lays it over that shipped preset, refusing a row the
family lacks by its path.

## The tuning config

The config at `--tuning` is authored once per species with the manifest: the
preset, the profile manifest and id the measurer reads, the required cells,
the reviewer adapters, the tracks and the dial ids the run tunes. Its
`references` may be an empty list: the run then compares against the
photographs the Profile stage found. `max_rounds` caps the rounds a revision runs
(absent, no cap); a recorded fixture replays the baseline and one round. A
first revision from a name needs no
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

1. **Candidates**, at most twelve: up to two images from each open-licence
   source's page (four in all; a gathered document is open when its markup
   declares an open deed), then Wikimedia Commons: the files of the taxon
   category's subcategories that file single trees (standalone, solitary,
   famous, park or field trees; three each), Commons' copies of geograph.org.uk
   photographs under the scientific and the common name (three each), the
   category of the tree in winter (two) and its bark (one) (host decision,
   2026-09-26). The bare taxon found the beech's leaves, buds and nuts, and
   solitary-tree searches mostly copper cultivars.
2. **Rights.** A Commons file whose machine-readable licence code
   (`extmetadata.License`) is CC0, public domain, CC BY or CC BY-SA is open
   by that code. Any other file's full licence metadata (`License`,
   `LicenseShortName`, `UsageTerms`, `LicenseUrl`, `AttributionRequired`,
   `Artist`) goes to Jev's rights question set. The question weighs a page's
   text and sets a photograph's licence aside as a figure credit, and it
   classed eleven of the beech's twelve CC files "none" (2026-09-25). Only
   `open-licence` goes on. A source's image takes its source's class.
3. **Copies.** Code downloads each JPEG or PNG into the run's cache,
   named by its sha256.
4. **One look.** The reviewer adapter in the tuning config (`vision`,
   `scripts/reference-first.py`, stage `screen`) looks at the whole batch
   once and says, per photograph, whether it shows the species (a copper,
   weeping or fastigiate cultivar is not the species as it grows wild), a
   mature open-grown tree and the whole tree, and its view: leaf-on, bare,
   bark or other.
5. **Kept**: up to two whole trees in leaf, one bare, one bark close-up,
   appended to `packet/references.json` with their source (`R<n>` for a
   Commons file), attribution, licence and sha256. A recorded reference is
   never removed or rewritten.

A run seeded with a species' curated references (the catalogue's
`packet/references.json`, as stored) needs no search and no hand patch
(fn-179). A record with a shot and no `view` takes its view from its first
scale that names one: `whole` in leaf is leaf-on, `bare` is bare, `base` is
bark. A photograph the project may not keep is found under the tuning
config's `matched.refs` by the file name its `url` ends in, checked against
its `asset_sha256` (bytes that differ stop the stage) and copied into the
run's cache under that hash; neither place is committed.

When none is kept and the tuning config lists none, the Profile stage still
completes: it skips the inventory and writes a `references` line to
`gaps.md`, which stops nothing. Tune refuses, saying so, until a photograph
is recorded in the config or a later Profile run finds one.

The cost is bounded: three free Commons queries, one Jev rights call per
Commons candidate without an open licence code (twelve at most) and one
vision call. The counts and the
look's token usage go to `<run-dir>/cache/photos/find.json`.

A photograph found from a name carries no camera, so Tune chooses its shot
before the first revision compares against it (host, 2026-09-26;
`runner::shots`). Code renders the tree Tune starts from under six candidate
cameras (a whole or bare tree framed full height at three elevations and two
fills; bark aimed at breast height from three distances through two lenses),
and the reviewer names the one whose framing matches the photograph (stage
`shot`). Code measures that render's tree box (`scripts/shot-frames.py`),
draws six candidate boxes about it, narrower, wider, shorter and taller, and
five crown-base lines over the photograph, and the reviewer names a box, a
line and one of four light presets (overcast; a low sun on the camera's left,
right or behind the tree) (stage `outline`). Every number is code's; a look
names labels or none. Two looks per reference. No camera leaves the reference
unused; no box or no line keeps it out of the numeric targets; no light draws
it under the overcast preset. The candidates, the selection and the shot go
to `runner/shots.json`, which a rerun reuses, and which the revision compares
against. When the tuning config names no required cell, the revision takes
them from the kept views: a whole tree in leaf at the fixed seed and seed 42,
a bare tree and a bark close-up when kept (`runner::cells`).

## Gaps

Code classes each failing trait from what the run recorded:

- **reachable**: a live dial moved it to passing in a rendered attempt the
  reviewer judged, so the owner can keep that value; the line names the
  dial, the two values it was drawn at and links the renders of both sides.
- **identity**: no capability assessment at all, a missing capability the
  assessment classes identity or leaves unclassed, a failing trait an
  assessed trait (`traits` in `packet/capability.json`, outcome
  unreachable-value or unsupported-anatomy) names, with its `depends_on`
  spec, or any other failing trait no dial brought to pass, for the host to
  assess (host, 2026-09-26). An assessed entry names a failing trait by its
  `trait` or by listing its id in `covers`, and by nothing else.
- **global**: a capability the assessment classes an improvement, a failing
  trait such an improvement covers, or a trait the config lists
  unexpressed, with the specs that capture it.

Each gap is one line in `gaps.md`, its first three pieces of evidence cut
short; `gaps.json` keeps the whole.
- **unsourced**: a profile field no document states typically (above, "Values"); the
  generator's default stands and Tune sets it from the photographs. A
  contextual field, one no agreeing sources settled, is listed here too.
- **references**: no reference photograph to compare against (above,
  "Reference photographs"). It stops nothing; Tune refuses until one is
  recorded.

The host reviews `gaps.md` and writes every spec; the runner mints none.
