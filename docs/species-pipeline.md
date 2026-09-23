# Species template pipeline: literature to preset

The pipeline takes a taxon from admitted sources to a species packet, a
provenance sidecar, fitted growth curves, a decision list, the species'
documentation and a report. A
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
$P document $D
$P report   $D
$P search-again $D   # after a stage prints SEARCH_AGAIN; then rerun the stages
```

Every command reads the manifest and the earlier artifacts at fixed paths
under `DIR`, writes one artifact atomically to `DIR/<stage>.json`, and
appends itself to `RUN/command-log.json`. Discovery lists every source the
repository already knows, the catalogue's bibliographies first, as a candidate
before it searches the web, so a source the repository has already verified is
never rediscovered; `--catalogue DIR` names the catalogue and defaults to
`catalogue`. A command whose idempotence key
(input checksums, manifest checksum, question-set versions, model name, tool
versions) matches the artifact on disk prints `current` and does nothing.
The tool versions carry the build identity (`species-pipeline-build`), a
digest of the crate's code and data baked in at build time, so changed code
reruns its stages; the crate version alone left three stages `current` after
fn-128's fix. A
command that stops prints the stage or the decision that stopped it; the
driver resolves nothing and reads nothing. `--adapter fixture:DIR` replaces
Firecrawl with pinned fixtures for the model-swap test.

## The capability assessment

The pipeline's eleven commands are mechanical. Deciding what the generator
cannot express is not, so the capability assessment is a **host step**, never
a driver's and never a stage: reasoning and system design escalate to the host
(`CLAUDE.md`, routing). The pipeline only checks its output.

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


## The requirements table

A species' literature has to cover what the generator needs, or the run
stops. `crates/telperion-jev/data/species-requirements.json` lists, for each
growth form (`broadleaf`, `conifer`, `palm`), the evidence fields a manifest
must ask for and the lowest sufficiency bar each must reach, and the
appearance traits it must describe. The bar is set by the table, not by the
manifest's author. The table binds a manifest at `schema_version` 2, the
version `discover` writes its proposal at. A version 1 manifest, such as the
ash's or a test fixture, loads unchanged.

- **Admission.** A version 2 manifest that leaves out a required field,
  lists a required field below the table's bar, or leaves out a required
  appearance trait is refused when any stage loads it. The refusal names
  every shortfall. A growth form with no row in the table is refused the
  same way.
- **Size fields** are ordinary `fields` entries. Height and trunk diameter
  carry the bar `partial`; the crown and organ sizes (crown width and base,
  leaf, needle, frond and leaflet sizes) carry `proxy_only`. The palm's
  trunk diameter carries `proxy_only` too (fn-132): the only trunk sentence
  its literature gives is a bound. So does the palm's height (fn-133): the
  owner chose growth rate plus mature range for palms, and a mature range
  is scored as a bound. The table also holds the words a
  sentence must contain to count as a point for each field.
- **How a field is asked** (fn-132). Each field is asked one of three ways:
  `age`, sizes at the manifest's required ages (the default); `mature`, a
  stated mature value or range; or `rate`, a stated growth rate, a size per
  year. A growth form may ask a field otherwise than the field's own way,
  under `asked` in its row. Broadleaf and conifer ask height and trunk
  diameter at 20, 50 and 80 years. Palm literature gives growth rates, a
  few age classes and mature ranges, and a palm grows near-linearly, so the
  palm asks `height_growth_m_per_year` as a rate and height and trunk
  diameter as mature sizes. A field asked at no age needs no
  `required_ages_years`; an empty or absent list is admitted, and a listed
  age is ignored.
- **Growth rates** (fn-132). A rate field counts the screen's
  `typical_growth_rate` rows that carry one of its words ("a year", "per
  year", "growth rate"). The growth-rate set scores a stated yearly rate for
  the taxon on the same four levels, with its own gap (`no_growth_rate`,
  `wrong_taxon`, `single_source`, `none`); a rate stated in words only, such
  as "slow growing", is `none`. Select fills a rate in metres a year
  (`m/yr`): the palm's A1 and P5 both state "30-45 cm (1 to 1.5 feet) a
  year", 0.30 to 0.45 m a year.
- **Mature sizes.** The table marks crown width and the leaf, needle, frond
  and leaflet sizes `mature`, and the palm's height and trunk diameter. `quality` lays out
  the rows the field can use (below). The mature-size set then scores a
  stated mature value or range for the taxon on the same four levels, with
  its own gap. A single source that states the size reaches `partial`; a
  bound such as "up to" reaches `proxy_only`. On the palm's first live run,
  A1's "The leaflets are ½ m (18 inches) long" and "**Width:** 20 - 50 feet"
  scored `none` under the age question; they are now labelled cases.
- **The gap follows the points** (fn-132, fn-133). A field asked at no
  age whose gap says no value is stated (`no_mature_size`,
  `no_growth_rate`) fails its bar, and a required one files
  `requirements-unmet`. A field with any point never carries that gap,
  whatever its level: beside `partial` it is `single_source`, beside
  `sufficient` it is `none`, and beside a lower level it is the most
  probable gap Jev gave that names a shortfall of a stated value
  (`bound_only`, `wrong_taxon`, `single_source`). On the palm's rerun after
  fn-131 the leaflet length and width were `sufficient` on 7 points each
  and failed on `no_mature_size`; after fn-132 its trunk diameter was
  `proxy_only` on 3 points, `bound_only` its next most probable gap, and
  failed the same way.
- **The rows a field can use** (fn-131). The screen gives an organ size its
  own class (`leaf_size`, `leaflet_size`, `frond_size`, `needle_size`,
  `cone_size`) and a named cultivar's size `cultivar_size`, which counts
  toward the species. The table lists, per field, the kinds that count for
  it (an organ field takes its organ class and `cultivar_size`, a tree field
  the tree-size kinds and `cultivar_size`) and the words its sentence must
  carry, each matched as a whole word or its plural, so "leaf" never matches
  "leaflet". `quality` counts only these rows and `select` reads only these,
  so the gate never passes a field on a row select drops. A growth-rate
  sentence that states a size reached at an age counts as a point: code
  parses "reaching 5 m (20 feet) in 15 to 20 years" as a point at 15 to 20
  years. A point under the required condition weighs 1; one whose
  condition the source leaves `unstated` weighs 0.5, and a required age is
  covered by points weighing 1 together.
- **Select is exact** (fn-131). One document per field, each row labelled by
  its source and place (`F1.1`), and every span keyed to its row (`F1.1: 20
  feet`), so a value is credited to the sentence it was chosen from. The
  selection floor applies only once its labelled set holds at least 20
  cases with at least 5 wrong picks (fn-133); then a pick below
  `selection_floor` fills nothing. Until then select takes the most
  probable span and records its probability in the select body
  (`pick_probability`; provenance carries none), and verify's
  field-aware check is the guard: the
  palm's leaflet picks, at 0.22 and 0.24, had been dropped by a floor of
  0.34 set on 16 cases. Code parses the chosen span with the
  one number and unit grammar the extractor uses (`quantity`): glued units
  ("6–10m"), millimetres, thousands separators and em-dash ranges, and "in"
  only as "in." or spelled out. A required field select leaves unfilled
  files `requirements-unmet`, which `search-again` takes like quality's.
- **Levels are choices** (fn-131). An appearance, described, sufficiency or
  mature-size or growth-rate level is the level Jev gave the highest
  probability, never
  the rounded average; `unstated` (or `none`) wins a tie. An appearance or
  described level below `level_floor` is `unstated`. The live palm's
  `leaf_back_colour` had come out `silvery_white` and its
  `leaf_brightness_range` `strongly_varied`, each at probability 0. Both
  floors are calibrated on labelled live answers
  (`data/cases/selection_floor.json`, `level_floor.json`): the lowest floor
  that answers the most cases right. The selection floor waits on its
  calibration minimum (above). No labelled set of live sufficiency
  answers exists yet, so the sufficiency levels take no floor.
- **Appearance traits** are `appearance` entries, each a trait name and the
  admitted sources that describe it:

  ```json
  {"trait_name": "bark_colour", "sources": ["A1"]}
  ```

  The level table comes from the requirements table and is derived from the
  `MaterialParams` fields the trait feeds. The traits are bark colour and
  bark roughness, leaf front colour and leaf back colour, and the leaf hue
  and brightness ranges. Each level maps to a `[low, high]` range per fed
  field; colours are linear reflectance. The table also holds each trait's
  words: `bark`, `trunk` and `stem` for the bark traits, and the leaf,
  frond, needle and leaflet words for the leaf traits. Code extracts every
  sentence of a source's cached text that carries one of those words.
  `select` has Jev score each sentence alone over the trait's levels, with
  the no-match level `unstated` last, as the screen judges a field's
  sentences one at a time. It asks the sources in the order the entry lists
  them and each source's sentences in page order, and stops at the first
  sentence it places on a level. That sentence is the chosen span. Code then
  copies the level's ranges into `profiles[0].appearance.<trait>` of the
  profile packet and records a sidecar entry with route `appearance`, the
  sentence, and the id of its source. A value with no source is never
  written as a sourced value: a trait no sentence states is `unstated`.
  A variation trait (the hue or brightness range, whose table has a level
  every range of which holds zero width) that every source leaves
  `unstated` takes that zero-width level, `uniform`, as a default (fn-133):
  the select body and the profile entry carry the level with `default`,
  its reason, and no source, and the sidecar records it under `defaults`,
  not `entries`, so verify never asks a source for it. A colour or
  roughness left `unstated` still files `requirements-unmet`. On the palm's second
  pass, the old route judged a 600-character page chunk: `bark_roughness`
  cited A1's navigation links, and `bark_colour` came out `unstated`
  although A1 says the trunk "is rough gray".
- **Verifying an appearance value.** `verify` checks the level against its
  source with the citation check, as it does for every value: the claim is
  the level as the table summarises it, beside the sentence it was read
  from, so a sentence that does not state the level can fail it. An
  appearance value is a level, not a number, so `verify` does not ask it
  `measurement_not_invention`. It asks `appearance_supported` instead:
  does the cited sentence describe the trait at this level, as the table
  summarises the level? The false side is the no-match answer. It covers a
  sentence that states another level, does not state the trait, or is page
  navigation. A value that is not supported files `claim-unsupported` with
  the value's pointer as its field. Measured values keep
  `measurement_not_invention`, asked with the field the value fills and the
  source text around the value's sentence; a source that no longer holds the
  sentence leaves the check unchecked, never judged on the page's start.
  Nothing renders or measures an appearance trait: `generate` records each
  one under `appearance` in its body as skipped. The material row is
  authored from these ranges.

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
decision stops every later stage until the manifest is admitted and the
decision resolved. The pipeline admits it itself when the draft only adds
sources and every new source passes both checks under "Admission by the
pipeline" below; it writes `DIR/manifest.json` and a resolution with `by:
pipeline`. Any other draft waits for a person to write the admitted manifest
and resolve it. That decision binds to the seed
(species, taxon, the fields with their conditions and required ages), and
discover keys on the seed too: admitting sources, curves, proxies or
engineering rows reruns nothing and keeps the admission, while a seed edit
reruns discover, reissues the proposal and voids the old admission.

Seven kinds carry options a stage consumes. A resolution to one of them with
an option outside its list is refused when the next stage reads it, naming
the kind and the options that are consumed; the stage that acted on a
resolution records itself as `consumed_by` on the decision.

| Kind | Options | Consumed by |
|---|---|---|
| `manifest-proposed` | `admit`, `reject` | every stage after discover; a rejected proposal stops them until the seed is edited and discover runs again |
| `unavailable-source` | `retry`, `replace-source`, `drop-source` | fetch: `retry` fetches again, `replace-source` fetches the `url` in the resolution's `payload` under the same source id and records both urls, `drop-source` skips the source and records it under `dropped`; a `retry` or `replace-source` that fails again reopens the decision |
| `coverage-gap` | `accept-rows`, `fix-table`, `drop-table` | fetch: `accept-rows` keeps the rows as parsed, `fix-table` reads the table entry the manifest now admits (and stops if the count still differs), `drop-table` records the table with no rows |
| `data-insufficient` | `admit-proxy`, `add-sources`, `lower-bar` | quality, by editing the manifest's fields only |
| `requirements-unmet` | `add-sources` | quality and select, once the manifest's sources change |
| `claim-contradicted`, `claim-unsupported` | `accept`, `replace-source`, `drop-value` | select: `drop-value` takes the flagged value (its source and span) out of the packet and files the field's `requirements-unmet` when the table requires it; `replace-source` does the same for any field, so `search-again` looks for another source; `accept` keeps the value |

`quality` files `requirements-unmet` for a required field whose sufficiency
level falls below the requirements table's bar, in place of
`data-insufficient`. `select` files it for a required colour or roughness
trait that the sources leave `unstated` (a variation trait reads zero
width instead), and for a required field it filled no value
for (fn-131): no candidate span, a pick below the floor, or a value a
resolution dropped. It blocks the later stages for that field. On
a field or a trait, `add-sources` is the pipeline's first: `search-again`
runs one round each, two at most, as "A requirement unmet searches again"
below describes. After the two rounds the decision is NEEDS_HUMAN and only
the owner resolves it. The conductor's policy lists no routine option for
it: no agent resolves it.
An open `manifest-proposed` decision, or a `requirements-unmet` decision
with no round left, comes before the gap loop, the stages and tuning: the
conductor pauses with a handoff (`pause-owner-<id>`) that lists every such
decision and the sources the rounds tried, and it opens no dispatch until
the owner resolves them and resumes. On the palm's first live run the
gate's halt sorted first and the conductor opened the gap loop while seven
of these stood open. After any stage the pipeline command prints
`SEARCH_AGAIN: <ids>` for the fields with a round left and `NEEDS_HUMAN:
<ids>` for the rest. The table's bar is never lowered: `lower-bar` is
refused by name. An `add-sources` resolution binds only once the manifest's
`sources` differ from the ones the decision recorded
(`payload.sources_sha256`). A resolution that adds no source is void on the
next read, and the decision stays open.

A stage's rerun retires its own stale decisions. When `quality`, `select`
or `verify` reruns with changed inputs, it marks each of its open decisions
that it did not file again as resolved, with the option `superseded`, and
records `by` as that stage's rerun. The field passed, or the trait was
found stated, on the sources the manifest already had. A superseded
`requirements-unmet` decision is therefore not held open when the sources
are unchanged. No person writes `superseded`, and no stage consumes it; a
person's resolution written against the decision's old inputs leaves it
retired rather than reopening it (fn-131). On
the palm's second pass, `quality` passed crown width and leaflet length,
but the first pass's decisions on them stayed open, and `select` skipped
both fields as "below the data-quality bar". `verify`'s `obligation-unmet`
decisions on four appearance values, and its `claim-unsupported` on A1 and
F1, stayed open the same way and stopped `generate`.

```json
{"id": "european-ash/fetch/unavailable-source/M1", "inputs_sha256": {"url": "..."},
 "option": "replace-source", "payload": {"url": "https://example.test/the-same-page"},
 "by": "owner", "at": "2026-09-18"}
```

Verify files one claim decision per value, keyed by its JSON Pointer, so
two values of one source are two decisions. Other kinds: `obligation-unmet`,
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

### Admission by the pipeline

Twice on 2026-09-23 the palm's run stopped for the owner to admit sources
the pipeline had found, and the owner asked that the pipeline admit them
itself (fn-129). A proposed source is admitted on two checks:

- **Relevance, by Jev.** The ranking chose it as the best evidence for the
  field it was found for. A candidate no ranking chose is never proposed.
- **Rights, by code and Jev.** Code fetches the page and lays out every
  licence or copyright statement in its metadata (`rel="license"`,
  `dc.rights`, `prism.copyright`) and a window around each licence word in
  its text, beside the open-access record it looks up: Europe PMC's record
  for a PMC article (its `license` field), DOAJ's for a DOI. Jev classifies
  them (`data/questions/rights.json`) as `open-licence`,
  `public-cite-only`, `restricted`, or the no-match answer `none`. Only the
  first two admit. A licence on a photograph credit is not the page's
  licence: the palm's F1 carries CC BY-NC-ND image credits and is
  `public-cite-only`.

A discover draft is admitted whole or goes to the owner whole: it adds at
least one source, keeps every admitted source unchanged, changes nothing
else (no field, bar, appearance trait or schema version; a draft that
lifts a version 1 manifest to version 2 is the owner's), and every new
source passes. One trait change counts as sources-only (host design,
2026-09-23): appending to a trait's `sources` list the id of a source the
manifest admits. A trait's name or level table stays the owner's. The decision's payload carries the verdict under
`admission`, with every reason a draft is the owner's. An admitted source
records its class in `rights_class` and a rights line naming the pipeline
and the ledger entry; its numbers are cited and no text is reproduced, as
for every source. The labelled cases are `data/cases/rights.json`, each
page as code lays it out from a live fetch: the palm's and the ash's pages
from their runs' caches, the rest and the open-access records fetched for
the set on 2026-09-23. Eleven admit: the
palm's F1, A1, M1 and P4 to P8, an open-access ScienceDirect article, and
the ash's J1 and O1. Five do not: two Facebook posts Firecrawl refuses and
an Elsevier article behind a paywall (`restricted`), an arXiv id that is no
URL and the ash's E1, a trade PDF with no statement (`none`). `jev cases`
scores them at the 0.9 accuracy bar.

### A requirement unmet searches again

`species-pipeline search-again` runs one round for every open
`requirements-unmet` decision whose field or trait has a round left:
`quality`'s on a field, `select`'s on an appearance trait or on a field it
left unfilled.
The query aims at the decision's dominant gap: `no_age_indexed_points` asks
for the field at stated ages (`Phoenix dactylifera height at stated ages in
years, open grown`), `age_range_uncovered` names the uncovered ages,
`wrong_condition` the condition, `wrong_taxon` quotes the taxon, and a
mature field's gap asks for the typical mature size, `no_growth_rate` for
the rate in centimetres or feet, and a trait's query
is the trait in words (`Phoenix dactylifera bark colour`). Every URL the
manifest holds or an earlier round tried is left out, Jev ranks the rest,
and the chosen source is admitted when its rights class admits. For a
trait, every admitted source the trait does not yet name is a candidate
too, ahead of the web's; the one the ranking chooses joins the trait's
`sources` list with no rights call, since it is already admitted, and a
new source that passes joins both lists. An admission resolves the
decision `add-sources` by the pipeline, so the stages rerun on it. A
trait's resolution binds once the trait's own list differs from the one
the decision recorded (`payload.sources_tried`), even when the manifest's
sources are unchanged. A round that admits nothing, or that
an adapter or Jev error ended, still counts. After two rounds the decision
is the owner's with the sources tried. Each round is recorded in
`DIR/search-rounds.json`: its query, hits, the sources tried with their
class and ledger reference, and what it admitted. The search waits while a
manifest proposal is open, because admitting that draft would overwrite
the sources a search added.

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

### Reading a source whole

The text every later stage reads is the text the source holds (fn-130; the
palm's run read a 196-byte cookie wall for two articles, and lost half of two
more to a tag stripper).

- **Fetch refuses what is not the source.** A scrape whose status is missing
  is a failed scrape. Its markdown is refused when it is empty, when a short
  page carries a known interstitial (a cookie wall, a bot check, a login
  page), or when it holds under 2% of the raw body's bytes (a PDF is exempt
  from the share). A refused scrape falls back on the raw body's own
  conversion, held to the same checks; `fetch.json` records that source with
  `markdown_from: raw` and the reason under `refused`.
- **One unreadable source stops only itself.** An adapter error, a checksum
  mismatch, a PDF that does not parse, or a page neither route can read files
  `unavailable-source` for that source with the reason verbatim, and fetch
  goes on to the rest and writes `fetch.json` without it. A `retry` or
  `replace-source` whose fetch fails again reopens the decision: the refiled
  inputs carry the checksum of the spent resolution, so it no longer binds.
- **Markdown is not HTML.** `extract`, `screen`, the appearance readings and
  `verify` read the cached markdown as it is; a "<" in a p-value is text. HTML
  is converted once, where it enters: the raw body fetch falls back on, and a
  page the citation check loads (`html::source_text`).
- **One splitter.** A `.`, `!` or `?` ends a sentence unless a digit follows
  it or it sits inside a quantity the unit pattern matches, so "18-20 ft
  (5.5-6.1 m) long by 2 ft (0.6 m) wide" stays one sentence. Candidate
  sentences, term sentences and the split of an over-long sentence share it.
- **One reading.** `screen` judges exactly `extract.json`'s candidates, in
  file order; it never re-extracts from the cache.

## Documentation

`document` runs after the packet is verified and before the report, and it is
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

When the profile holds appearance ranges, the article carries a generated
`appearance` block after the Leaves section. For each trait it lists the
level, how that level is described, each material field and its range, and
the sources, which are the values the material row is authored from. The
check requires this block only for a profile that holds appearance ranges.
When an existing article is refreshed after its record gains them, the block
is inserted before the gaps section.

## Cost

Every artifact's header carries `cost`: the runs that wrote it, the Jev calls
it made and the Firecrawl credits it spent, counted from the CLI's
`creditsUsed` where a response prices itself and estimated at one credit per
call where it does not, with the method named. The report sums them per
stage and in total under `costs` and in its `## Cost` table.

## Artifacts

| Path | Schema | Written by |
|---|---|---|
| `manifest.json` | manifest v1 | a person; the pipeline appends a source it admits |
| `discover.json` | discover v1 | discover |
| `fetch.json` | sources v1 | fetch |
| `extract.json` | candidates v1 | extract |
| `screen.json`, `quality.json`, `select.json`, `verify.json`, `fit.json`, `gate.json`, `generate.json`, `document.json`, `report.json` | one schema each, v1 | the stage of that name |
| `sources/<source id>.md` | front matter `{source, url, title, attribution, rights, fetched, sha256, source_sha256, form}`; a full copy where the rights permit one, the cited passages where they do not | document |
| `ARTICLE.md` | the sources distilled, a citation on every claim | document |
| `packet/profile.json`, `packet/references.json` | fn19 closed records | select |
| `packet/species.json`, `packet/specimens.json` | fn19 closed records | generate |
| `provenance.json` | provenance v1, keyed by JSON Pointer | select, generate |
| `gaps/<slug>/gap.json`, `rounds.json` | gap v1, rounds v1 | the gap loop |
| `metrics.json` | metrics v1 | `gap metrics` |
| `decisions.json`, `resolutions.json` | decisions v1 | every stage; a person, or the pipeline for an admission or an `add-sources` it made |
| `search-rounds.json` | search-rounds v1, keyed by field | search-again |
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
reason (fn-53). A gap spec that gives the generator a capability also moves
that name into the vocabulary's expressed list, in the one line, with the test
that failed before the implementation; a landing that leaves the name where it
was tells the next round nothing changed.

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

Eight versioned sets under `crates/telperion-jev/data/questions`: source
ranking per field, data sufficiency per field with its dominant gap, the
mature size of a `mature` field with its gap, the growth rate of a `rate`
field with its gap (fn-132, labelled on the palm's A1 and P5 rate
sentences), described
level scoring over levels a person wrote, the semantic obligations
(`inspected_image`, `measurement_not_invention`, `appearance_supported`),
a proposed source's rights class, and the gap loop's options.
Their labelled cases with negative and held-out entries live under
`data/cases`; `jev cases` reruns them live and fails when a held-out accuracy
is below 0.9 (0.8 top-one agreement for ranking and for the gap set's best
match), listing the missed case ids. `--only labelled|pipeline|gap` runs one
family alone, so a set being tuned costs one family's calls.
