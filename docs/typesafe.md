# TypeSafe usage

Jev is the typed-judgment model behind TypeSafe. A new agent follows this page.
The short form lives in `AGENTS.md` under TypeSafe. The tools live in
`crates/telperion-jev`. The labelled cases and the 2026-09-16 sweep live under
`.flow/evidence/fn57/`.

## Where it may run

Evidence tooling, research checks, QA triage, and report assembly. Those paths
read fetched source bytes, a spec's research section, an owner's written note,
or a report's section headings. They write a ledger entry and a proposal the
host session reads.

## Where it never runs

Generation, rendering, presets, the Wasm bindings, the browser source, and any
test on the workspace test commands (`cargo test --release --workspace`,
`npm test`, `npm run typecheck`). Pin tests, byte-identical presets and seeded
streams stay deterministic. A guard in `telperion-jev` fails when the core,
render or Wasm crates or the browser source import the caller or name
`https://api.typesafe.ai/v1/systemone`.

## Code finds, the model chooses, code copies

Code extracts every candidate number, heading or spec id and owns every
calculation, render and measurement. Jev answers a Choice, a Noul or a Score
over those options. The value that reaches a preset is a span code copied
verbatim from the source bytes. No model output is parsed for a number.

## Every question offers a no-match answer

A Choice always includes `none`, `unstated`, `says_nothing`, `not_about_tree_size`
or `new_spec`, named for that question. A Noul's false criterion is the
no-match, including triage `assessable` (false leaves severity `unassessed`).
A selection whose recall regex dropped a span cannot choose it.

## Candidate coverage is tested before a selection is trusted

The recall regex is the regression gate. The pilot dropped `foot`, so
`6-7 foot` was missing and Jev answered `none` at 0.49 against
`8 to 11 years` at 0.46. That case lives in `crates/telperion-jev/data/cases`.
A `none` with a spread distribution prints the candidate list beside the
source sentence.

## Thresholds come from the labelled set

`crates/telperion-jev/data/thresholds.json` records the cuts chosen on the
pilot's labelled answers. Confidence summarizes how concentrated a
distribution is. It never stands in for correctness. A change to a question
set reruns the labelled cases and reports accuracy and confidence spread
before the change lands.

## Every call leaves a ledger entry

The shared caller owns the endpoint, the bearer key and the record. The key
sits in `~/.bashrc` below the non-interactive guard
(`[[ $- != *i* ]] && return`). A tool reads it through `bash -ic` and never
prints, echoes or copies it. A missing key stops with a message that names
that path. A failed call after retries records the failure and exits
non-zero.

The ledger entry holds a unique immutable id, the state's checksum, the
questions, the answers with their probabilities and confidence, the model
name, token usage, elapsed time and the source checksum. Reports cite that
id. Filenames use it, and a write never overwrites an existing file. A
pipeline that consumed a judgment cites that entry.

## A judgment proposes and never writes project state

A triage run returns the owning spec, the most similar prior finding and a
severity level. `--standard` is required. An observation that fails the
assessable Noul is reported as `unassessed` rather than a level. It does
not create a memory entry, a QA receipt, a finding, an outcome or an owner
verdict. The host session reads the proposal.

## Compose, then threshold

A height-at-age number reaches a profile author only when the screen calls
the sentence a measured size at a stated age and the citation check calls
the claim supported, each above the labelled threshold. The cite report
carries the screen kind, kind confidence, `anchor_usable`, and both the
citation and screen ledger references. The O1 sentence
("Sustained height growth of 1 to 2 ft per year for trees 10 to 30 years
old") is a site-quality criterion. The citation check alone called a
restatement of that criterion supported at 1.00. The screen is why it does
not ship.

## Commands

From the repo root, with the key available to an interactive shell:

```sh
bash -ic 'cargo run -p telperion-jev -- screen --source <file> --species <id> --id <source-id>'
bash -ic 'cargo run -p telperion-jev -- select --document <file> --question "<field>"'
bash -ic 'cargo run -p telperion-jev -- cite --research <spec-or-section.md>'
bash -ic 'cargo run -p telperion-jev -- triage --observation "<note>" --specs <open.json> --standard "<owner standard>" --findings <prior.json>'
bash -ic 'cargo run -p telperion-jev -- cases'
```

`cite --research` uses only `## Resolved via Research` when that heading is
present, so acceptance and boundary bullets are not judged as claims. A
file without the heading is treated as section-only input.

`jev cases` reruns the four labelled sets against the live model, prints
one row per case with expected, answered, top probability, confidence and
ledger reference, then accuracy and confidence spread per set. It exits
non-zero when a set misses its pilot score. Workspace tests keep the mock
transport and the key unset.

Each run prints its rows, the ledger path, and the total input and output
tokens with wall time. No call cap is hardcoded.

## Asking a question of a still

`jev ask --assets <file>` sends images with the question. The file maps a
reference string to an image path. The reference is written in the state
or a question wherever the picture belongs, and the API puts the image in
its place (https://docs.typesafe.ai/images.md). Pick a reference that cannot
occur in other text by accident, and use every reference you list.

```sh
bash -ic 'cargo run --profile ci -p telperion-jev --bin jev -- ask --tool ask-still --state <state.json> --questions <questions.json> --assets <assets.json>'
```

```text
state.json      {"still": "still-oak-1001-6b1e"}
questions.json  {"foliage": {"type": "choice", "instructions": "Do the trees in `still` carry leaves?",
                 "criteria": {"in_leaf": "...", "bare": "...", "neither": "No trees, or it cannot be told."}}}
assets.json     {"still-oak-1001-6b1e": "path/to/still.jpg"}
```

Each image is PNG or JPEG, 16 MiB at most, with 280 to 1568 pixels on each
side. The caller refuses one outside those limits by name before any call;
fit a larger still first (`.flow/evidence/fn-348-the-jev-caller-carries-a-still/prepare-still.sh`).
The ledger entry records each image's reference, path and SHA-256, never
its bytes. A worked call is in that spec's `RESULT.md`.

The answer is evidence about a picture: it may order review sheets, flag a
defect the owner has named, or answer a narrow question a run's host put,
always as a proposal with its ledger reference. It never passes a visual
inspection, never stands in for an owner verdict, and never supplies a
number; measurements come from code.

## What the sweep left as code

The hand-rolled JSON scan in `crates/telperion-core/src/suite/foliage_reference.rs`
and the stdout regex in `tests/species.mjs` are code fixes, not Jev work.
Jev may read a still (owner, 2026-11-01): it may order review sheets and flag
a defect the owner has named, as a proposal. It never passes a visual
inspection or stands in for an owner verdict. The species QA runner keeps
exiting non-zero while a visual inspection is unassessed.
