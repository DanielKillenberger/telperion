# Species conductor

`species-conductor` belongs to `telperion-jev`. It carries one species run
from the admitted manifest to the packet the owner reviews, without asking
the owner to conduct routine stages: it runs the runbook's stages
(`docs/species-pipeline.md`), hands a halt to the gap loop, runs the tuning
loop (`docs/tuning-loop.md`) and answers the check on every gap its result
lists, drives a gap spec through design and implementation on separately
judged model tiers, and assembles the packet once every automated
prerequisite holds. It designs nothing, implements nothing and accepts
nothing. A new gap is packaged and escalated to the host; only the owner's
verdict accepts a species.

Build with `cargo build --release -p telperion-jev --bin species-conductor`.
Every command takes `--config FILE`:

```text
species-conductor start    --config FILE
species-conductor status   --config FILE
species-conductor step     --config FILE
species-conductor dispatch --config FILE --id ID --result FILE
species-conductor attach   --config FILE --spec SPEC --gap GAP
species-conductor land     --config FILE --spec SPEC --commit SHA
species-conductor adopt    --config FILE --spec SPEC --commit SHA --result FILE
species-conductor resume   --config FILE --decision FILE
species-conductor packet   --config FILE
species-conductor report   --config FILE
species-conductor cases
```

The config names the species and its spec, the catalogue folder and the run
directory, the Flow tree, the tuning loop's config, the runbook's binaries,
and whether the continuation question set has been validated. The run record
and every other file the conductor writes live under `RUN/conductor/`.

The config needs no `budget` block. Each of its caps (`max_tokens`,
`attempt_max_tokens`, `max_dispatches`, `max_tuning_revisions`) is optional
and an absent one is no cap: while the loop is being built, a run is not
stopped for what it spends (owner, 2026-09-23). A config that sets a cap
still pauses at it. The spend is recorded either way, in the run record, the
handoff and the report, so a usual run's cost can be learned and an
outlying one flagged later.

## What code decides and what Jev answers

Code owns stage eligibility, the spend record, any cap a config sets,
dispatch tracking and resume. The
stages carry their own idempotence keys, so the conductor runs them in order
and a current stage does nothing; a full pass records a fingerprint of the
manifest, the resolutions and every landed fix, and a landing expires it.
A stage whose artifact records a pipeline build other than the current one
had its key expired by a code change (fn-132). The conductor reruns the
stages from the first such stage before it searches again or pauses for the
owner on a literature decision, once per build: a stage that stops leaves
its artifact stale, and the run moves on rather than rerunning it. On the
palm's rerun after fn-130 and fn-131 the conductor searched while `fetch` to
`verify` still carried the old build, and two empty rounds went to the owner
before the stages had read P8's raw body. An artifact that records no build
predates the build id and is left to the fingerprint. An
open decision is the gap loop's (`onboarding-gate`, `level-miss`), the cheap
agent's under the policy's `decisions.routine` list (a source to retry or
replace, a table to accept, an article sentence to recite or rewrite), the
pipeline's, or the owner's; lowering a bar or overriding a ruling is never
routine.

The pipeline resolves two kinds by code, never through an agent (fn-129).
It admits a `manifest-proposed` draft that only adds sources, each chosen
by the ranking for its field and classified `open-licence` or
`public-cite-only`; any other draft pauses for the owner. A
`requirements-unmet` decision on a field or an appearance trait is
searched again first: while it has one of its two rounds left, the conductor's next action is
`search_again`, which runs `species-pipeline search-again` once, and a
source it admits changes the manifest, so the stages rerun. The search
waits while a manifest proposal is open. A source found for a trait joins
the trait's `sources` list, the one trait change the pipeline makes. Once
the rounds are spent the run pauses for the owner as before, and
the handoff's `sources_tried` lists every URL the rounds tried per
decision. See `docs/species-pipeline.md`, "Admission by the pipeline".

Jev answers four bounded questions, each with a no-match answer, each
through the shared caller with a ledger entry:

- **Reachable with** an untried dial: over the authored dials the tuning
  run has not tried on the trait.
- **Covered by** an open spec: over the open specs' ids and titles.
- **Design complexity**, over the spec's text and the gap evidence, before a
  design exists: routine, complex or insufficient evidence.
- **Implementation complexity**, over the completed design handoff alone,
  never inheriting the gap's difficulty: straightforward, complex, needs
  design or insufficient evidence.

A first attempt on a dependency is bounded by construction, one dispatch
within the attempt bound on a route the table justified, and asks no
continuation question; nothing has been tried, so the trio could only
answer insufficient evidence, which is what paused the first live run.
The shared continuation trio (`tuning/continuation.rs`) runs before every
repeat, that is before every
design or implementation dispatch and every tuning revision after the first,
and code combines it with any cap the config sets under the contract every
tuning run already obeys. With no attempt bound and no usage reported yet
the next attempt's estimate is recorded as unknown; it is needed only to fit
a token cap.

## The policy

`crates/telperion-jev/data/conductor-policy.json` is the one table that maps
those answers to a route. Its rows are tried in order over the recorded
signals; the human rows sit first, so an unjustified attempt is never
bought by a later row, and a signal the record lacks is the human's by rule.

| Route | Who | Effort |
|---|---|---|
| `routine`, `investigate` | the cheap agent | default |
| `design` | the high-reasoning model | medium |
| `design_again` | the high-reasoning model | high |
| `implement_cheap` | the cheap agent | default |
| `implement_strong_low` | the high-reasoning model | low |
| `implement_strong_medium` | the high-reasoning model | medium |
| `human` | nobody: the run pauses | |

A tier is a name; the instruction file's routing block resolves it to a
model at dispatch time, and the result records the model and effort that
actually ran, so the two are independent. A failed cheap implementation goes
to the strong tier at low effort; a failed low-effort one to medium; a
design that leaves unknowns goes back to design at high effort; three
attempts without a verified result, a set cap reached, an unavailable or
unjustified continuation judgment, and any signal the table cannot read all
pause for the human, whatever the spend.

A complexity answer under the confidence floor buys one investigation. After
a verified investigation at the dependency's current design revision, the
same answer still under the floor is decided on the mass its side carries,
and when neither side reaches the floor the judgment reads
`insufficient_after_investigation`, which the table sends to the human as
"insufficient evidence after one investigation", on the design side and the
implementation side alike. A second investigation of the same revision is
never bought: the date palm's run opened two for fn-144 (dispatch-18 and
dispatch-19) before this rule.

`species-conductor cases` scores `data/cases/conductor.json` against the
policy and the continuation contract with no call, held-out cases apart
from the tuned ones, and prints expected against selected. The workspace
test `cases::tests` fails when any case disagrees.

## Dispatches and results

A dispatch records its role, route, tier and effort, the dependency it
serves, the identity of the evidence it was cut against, the design revision
where one exists, the judgments that chose it and the tokens it reserved.
The dispatched agent writes a result file:

```json
{"input_identity": "...", "design_revision": null,
 "actual_model": "...", "actual_effort": "medium",
 "usage": {"input_tokens": 1000, "output_tokens": 200}, "cost_usd": null,
 "verification": "verified", "observed": "...", "handoff": "design.md",
 "failure": null}
```

A result naming another identity or revision is obsolete; one without
verification or attribution is interrupted; neither advances the run, and
both stay in the record and the report. A verified design records its
handoff by content hash as the design revision; verified implementation
waits on the host's landing authority, which `land --commit` records, and a
landing reruns the affected stages and the next tuning revision.

The host sometimes builds a dependency itself, outside the conductor's
dispatches (fn-110 and fn-144 were designed and built by agents the host
sent before the conductor opened its own). `adopt --spec SPEC --commit SHA
--result FILE` records that work after the fact for a dependency that is
Designed or Awaiting: a verified `implement` dispatch on route `host_built`,
tier `host`, at the dependency's design revision with no reserved tokens,
its result read from a file in the format above with `verification`
`verified`. Any dispatch of that dependency still open is closed as
obsolete with a note naming the adoption, and the dependency lands at the
commit in the same step. A dependency already landed, an unknown spec and a
result that is not verified are refused and change nothing; verified work
already awaiting landing lands with `land`. A pause the run holds stays
until `resume`.

## The tree tuning starts from

A species' first tuned tree starts from its sourced profile (fn-135). Before
a tuning revision starts, and so again after the literature has changed the
profile, the conductor derives wire values from `DIR/packet/profile.json`
(the profile whose id is the tuning config's `profile_id`) and writes them
into the tuning config's `initial_overrides`. A revision already under way
(its `run.json` exists) keeps the overlay it started from, because its
resume refuses a changed baseline. A species folder with no profile derives
nothing and leaves the config alone.

The derivation is code over `crates/telperion-jev/data/profile-to-preset.json`,
which names no species. Every appearance range keyed by a material field
sets `/material/<field>` to its middle. Each metric then goes through the
table's rows whose condition the family meets, the family being the preset
with the manual entries laid over it: `midpoint` takes a range's middle,
`identity` a single stated value, `ratio` a middle times a factor over
another metric's middle or a family value. Height reaches the envelope
height, half the trunk diameter over the height the trunk radius, crown
width over twice the height the envelope spread (no rosette), frond length
the rachis (a rosette), and leaflet or leaf sizes the element over the
canopy size. Each value is clamped to its dial in `data/dials.json`.

`TUNING.derived.json` beside the tuning config records every value with its
profile entry, citations, formula and any clamp, every profile entry left
out and why, and the manual entries. A manual entry wins: an entry counts as
derived only while it holds the value the last provenance recorded for it,
so an edited or added entry is the person's and stays. The same inputs write
the same bytes. Shipping tuned values into the preset stays the species
spec's last step.

The same step makes each gating metric of the tuning profile (the tuning
config's `profiles`) that `species_measure` cannot read contextual, so it
reports and never fails an evaluation; the list is `derive::MEASURED`, and a
test checks each of its names against the measurer's source.

## Gaps, pauses and resume

Every tuning revision ends with `result.json`; the conductor checks each
gap that is not passing: reachable goes back to tuning with the dial named,
covered attaches the open spec as a dependency, new writes
`RUN/conductor/gaps/<gap>.json` in the shape fn-95 shares with the studio
and pauses. The handoff (`RUN/conductor/handoff-<pause>.json` and `.md`)
carries the current renders or why they are missing, the unresolved
requirement, every attempt and its outcome, the spend, what remains under
each cap the config sets (null for none), the proposed next action with its
allowance and basis, the risk
signals and the decision requested. Nothing dispatches while it stands.
`resume` needs a decision that names the pause, the identity and the action
exactly, with who and why; spend stays spent and the evidence is
rechecked on the next step. A spec the owner mints for a packaged gap is
attached with `attach`, once; attaching it again is a no-op.

A tuning revision pauses by design: for its pilot authority, for the
owner's priority approval, at a preflight. `tuning-loop run` then exits
non-zero with "paused; see run.json", and the conductor reads the tuning
run's own `run.json` rather than the exit: its `pause` becomes the
conductor's pause under the same id, identity and proposed action, with the
tuning run's reason and decision request, a handoff whose signals name the
tuning run, and nothing recorded as a revision. The one decision file then
serves both: `resume --decision FILE` resolves the conductor's pause and
runs `tuning-loop run --resume FILE` in the same tuning directory. A
decision either side refuses leaves the run as it was; a new tuning pause is
carried the same way; a revision that ends is recorded and its gaps are
checked on the next step. A revision counts as ended only when its record
holds neither a pause nor an attempt in flight, because the tuning loop
rewrites `result.json` on every save.

Converged is a finish (fn-136). A revision the runaway guard stopped, one
that stopped with no supported proposal while every listed gap passes, and
one that ended with every listed gap passing are recorded with `converged`
and the reason, and the conductor assembles the packet next: no pause is
carried, no gap is checked and no further revision is asked for. A landing
still asks for the next revision.

## The packet and the report

`packet` is ready only when the report reads complete and `metrics.json` is
written, the latest tuning revision is machine ready and not a bootstrap, or
converged, every listed gap passes, every matched still is on disk, and
`ARTICLE.md` and `document.json` exist with no open decision; otherwise it
names every limitation. A converged bootstrap revision still reaches the
owner: `machine_readiness` reads `unqualified reviewer`, which is neither
ready nor failed, and reviewer qualification is listed as a known gap, so
the owner's verdict is the acceptance. An unconverged bootstrap stays
withheld. After a converged revision a gap that does not pass
is listed under `outstanding` for the owner to judge rather than withholding
the packet. The checklist lists every known gap with status `known gap` and
the specs that capture it: each improvement capability the gate recorded and
each trait the tuning result lists as one. Its owner acceptance is pending
until the owner says otherwise, and the owner may reverse any class there.

`report` writes elapsed wall time, waiting, interruptions, dispatches by
role, tier and effort with tokens, cost and the models that ran, Jev calls
by tool, tuning revisions, captures, retries, wrong routes, failed fixes and
human escalations. Unknown costs are counted, never estimated; failed
attempts stay in the totals; no saving is claimed, because the report
compares nothing.

## What is not yet proven

The conductor's tests run its whole loop against a scripted transport and
scripted stage and tuning outcomes. No live species has run under it, no
labelled set has validated the four question sets against the live model,
and `continuation_validated` in the config asserts what the tuning loop's
calibration establishes, never a validation of its own. fn-80 owns the live
proof.
