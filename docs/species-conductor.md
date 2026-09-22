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
species-conductor resume   --config FILE --decision FILE
species-conductor packet   --config FILE
species-conductor report   --config FILE
species-conductor cases
```

The config names the species and its spec, the catalogue folder and the run
directory, the Flow tree, the tuning loop's config, the runbook's binaries,
the total token allowance and the per-attempt bound, and whether the
continuation question set has been validated. The run record and every
other file the conductor writes live under `RUN/conductor/`.

## What code decides and what Jev answers

Code owns stage eligibility, the budgets, dispatch tracking and resume. The
stages carry their own idempotence keys, so the conductor runs them in order
and a current stage does nothing; a full pass records a fingerprint of the
manifest, the resolutions and every landed fix, and a landing expires it. An
open decision is the gap loop's (`onboarding-gate`, `level-miss`), the cheap
agent's under the policy's `decisions.routine` list (a source to retry or
replace, a table to accept, an article sentence to recite or rewrite), or
the owner's; lowering a bar, overriding a ruling or admitting a manifest is
never routine.

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
and code combines it with the hard limits under the contract every tuning
run already obeys.

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
attempts without a verified result, a hard limit, an unavailable or
unjustified continuation judgment, and any signal the table cannot read all
pause for the human, budget remaining or not.

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

## Gaps, pauses and resume

Every tuning revision ends with `result.json`; the conductor checks each
gap that is not passing: reachable goes back to tuning with the dial named,
covered attaches the open spec as a dependency, new writes
`RUN/conductor/gaps/<gap>.json` in the shape fn-95 shares with the studio
and pauses. The handoff (`RUN/conductor/handoff-<pause>.json` and `.md`)
carries the current renders or why they are missing, the unresolved
requirement, every attempt and its outcome, the spend, the remaining
budget, the proposed next action with its allowance and basis, the risk
signals and the decision requested. Nothing dispatches while it stands.
`resume` needs a decision that names the pause, the identity and the action
exactly, with who and why; spent budgets stay spent and the evidence is
rechecked on the next step. A spec the owner mints for a packaged gap is
attached with `attach`, once; attaching it again is a no-op.

## The packet and the report

`packet` is ready only when the report reads complete and `metrics.json` is
written, the latest tuning revision is machine ready and not a bootstrap,
every listed gap passes, every matched still is on disk, and `ARTICLE.md`
and `document.json` exist with no open decision; otherwise it names every
limitation. Its owner acceptance is pending until the owner says otherwise.

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
