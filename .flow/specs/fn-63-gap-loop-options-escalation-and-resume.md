# Gap loop: options, escalation and resume

## Conversation Evidence

> user (turn 21): "what i want in the end basically is: i open the repo with an agent i tell it to add species A and it will prepare all the docs and artifacts and go through the pipeline to produce species A in the generator properly QA'd and improved the generator to fix missing gaps that allow the species to continue."
> user (turn 21): "Jev should evaluate the options on how to fix the gaps and should evaluate also if the decision needs a better reasoning model or human to make the decision."
> user (turn 22): "should be as autonomous as possible but also as efficient and high quality as possible. Min max problem."
> user (turn 24): "in my view we're building the machine to serve the conductor. And the conductor only really is a run-book or smth? so some skill file that comes with the repo so i'd probably merge it."
> user (turn 26): "how do we create the manifest? don't we need a conductor for that?"
> user (turn 27): "ok sure"

## Goal & Context
<!-- scope: business -->
<!-- Goal & Context: 30% [user], 50% [paraphrase], 20% [inferred] -->

The owner's end state is one instruction, "add species A", after which an agent prepares the docs and artifacts, runs the pipeline, produces the species in the generator properly QA'd, and improves the generator where a missing capability blocks the species. [user] fn-58 delivers the runbook and toolset that take a species from its name to a report, with the agent drafting the manifest and the owner admitting it, and it halts when the generator cannot express something the species needs. This spec is what happens at that halt. [paraphrase]

A gap is a capability the species needs and no value table reaches: the onboarding gate's capability line, or a described trait whose rendered candidates all miss their level. fn-34 met fifteen of them, pendulous shoots, multi-stem clumps, the beech's limb pitch among them, and each became a spec by hand after rounds of stills. The loop makes that path the run's own. The agent, or a stronger model, writes candidate fixes as typed options. Jev answers narrow questions over the option set. A policy table turns those answers into one of three routes: decide and proceed, hand the option set to a stronger reasoning model, or file a decision for the owner. The chosen fix is minted as its own spec, driven to landing under the usual review, and the species run resumes where it halted. [paraphrase]

The owner frames the loop as a min-max problem: as autonomous as possible, and as efficient and high quality as possible. The loop therefore records three numbers per species run, the share of decisions it took itself, the rounds to an accepting checklist with the count of its decisions the owner later reversed, and the tokens, wall clock and captures spent. The escalation thresholds are the dial between them, and reversals are the signal for tuning it. [paraphrase]

Jev and the owner's standing reviewer both advised keeping this loop separate from fn-58, on the finish lines: fn-58 closes on validation against fn30 with no species shipped, this spec closes on a real gap handled end to end. The owner's view that the machine serves the conductor holds either way; the conductor's surface, the skill file and runbook, ships with the repo. [paraphrase]

## Architecture & Data Models
<!-- scope: technical -->

- **A gap is a typed halt.** The species run stops with an onboarding-gate decision naming the capability, or a level-miss whose candidates all fell outside their range. The loop starts from that decision and nothing else; a value-table change is not a gap and stays a decision inside the run. [paraphrase]
- **Options are written, not judged into being.** The agent writes two to four candidate fixes in a fixed shape: what changes, whether it is a value-table change or a generator change, which parameter or module it touches, whether it moves a pin or changes any preset's output, which other species it would serve, and its reversibility. A stronger reasoning model may write them when the agent's set is empty or the route sends it there. Jev never writes an option. [paraphrase]
- **Jev answers narrow questions over the set.** Per option: is it a value-table change or a generator change; does it generalize across species as the strategy asks, or serve one; does an existing owner verdict already decide for or against it; does it move a pin. Over the set: which option best matches the gap's stated capability, and how close the top two are. Each is a versioned question with labelled cases drawn from fn-34's fifteen gaps, where the owner's actual choices are the labels. [paraphrase]
- **The route is a table, not a judgment.** Code reads the option spread, generator-touch, pin movement, reversibility and prior-verdict coverage and looks up one of three routes: proceed with the top option, hand the set to a stronger reasoning model with the same questions, or file a decision for the owner. The thresholds come from the labelled cases and live in one table a person can read and change; moving a threshold re-routes recorded runs without a new call. [paraphrase]
- **The fix is its own spec.** The chosen option is captured as a spec the species spec depends on, worked and reviewed the way every generator change is, never patched inside the species run. Its landing changes tool versions, so the species run's idempotence keys expire from the halted stage down and the run resumes there. [paraphrase]
- **Rounds are bounded.** A verdict that is not yet accepting allows two value rounds before the loop names a gap or files for the owner, as fn-62 already rules. A gap spec whose review returns needs-work twice goes to the owner. [paraphrase]
- **The skill file is the surface.** A repo-shipped skill that takes "add species A" and runs fn-58's runbook, this loop at every halt, the species QA and the checklist handoff, with the three per-run numbers written beside the report. [paraphrase]

## API Contracts
<!-- scope: technical -->

An option and a route record are the two new shapes. The fields shown are the contract.

```json
{
  "gap": "oregon-white-oak/onboarding-gate/capability/pendulous-shoots",
  "option": "shoot-rows-hang-under-gravity",
  "change_kind": "generator",
  "touches": "shoot placement",
  "moves_pin": true,
  "changes_preset_output": ["norway-spruce"],
  "serves_species": ["silver-birch", "european-beech"],
  "reversible": true,
  "summary": "Rows of short shoots hang under a gravity term the family authors."
}
```

```json
{
  "gap": "…",
  "options": ["…"],
  "judgments": {"…": "…"},
  "signals": {"spread": 0.31, "generator_touch": true, "moves_pin": true, "reversible": true, "prior_verdict": "none"},
  "route": "owner",
  "table_version": 1,
  "ledger": ["…"]
}
```

The per-run metrics record carries the species, the decisions taken by route, the rounds to acceptance, the owner's reversals by decision id, and tokens, wall clock and captures. [inferred]

## Edge Cases & Constraints
<!-- scope: technical -->

- **An empty option set routes to the stronger model, and an empty set from there routes to the owner.** The loop never proceeds on nothing. [inferred]
- **A stronger model that returns no preference routes to the owner.** Its answers are recorded like the agent's. [inferred]
- **The generator is never changed inside a species run.** Every fix is a spec with a review, and the species run only resumes after it lands. [paraphrase]
- **Thresholds are data.** They are set from the labelled cases before the first real gap and tuned from reversals after; no threshold is a constant in code. [paraphrase]
- **The stills stay with the owner.** No route decides a visual verdict; the loop hands the checklist to the owner at the end of every species run. [paraphrase]

## Acceptance Criteria
<!-- scope: both -->

- **R1:** Every gap halt yields an option set of two to four candidates in the fixed shape, written by the agent or the stronger model, each stating change kind, touch, pin movement, preset-output changes, species served and reversibility. [paraphrase] Errors: an empty set from the agent goes to the stronger model; an empty set from both files for the owner.
- **R2:** The option question set is versioned data with labelled cases from fn-34's fifteen gaps, labelled by the owner's actual choices, and its held-out accuracy is at least 0.9 on change kind and prior-verdict coverage and at least 0.8 on best-match. [paraphrase] Errors: a held-out miss below the bound fails with the cases listed.
- **R3:** The route is read from one threshold table over the recorded signals, one of proceed, stronger model or owner, and every routed gap records its signals, its route and the table version, so a changed table re-routes past gaps without a new call. [paraphrase] Errors: a signal missing from the record routes to the owner.
- **R4:** The chosen fix is minted as a spec the species spec depends on, worked under the repo's review, and never applied inside the species run; the species run resumes from the halted stage once it lands, rerunning only the stages whose keys expired. [paraphrase] Errors: a gap spec reviewed needs-work twice files for the owner; a landing that changes a pin records it under fn-53's rule.
- **R5:** Every species run records autonomy, quality and efficiency in the metrics record, decisions by route, rounds to acceptance and reversals by decision id, and tokens, wall clock and captures, and the reversals feed the next threshold tuning. [paraphrase] Errors: no error surface beyond a missing record failing the run's report.
- **R6:** A repo-shipped skill takes "add species A" and runs fn-58's runbook, this loop at every halt, species QA and the checklist handoff, with two value rounds per verdict before a gap or a hand-off. [paraphrase] Errors: a third round on the same verdict is refused and files for the owner.
- **R7:** One real species goes through the skill end to end with at least one real gap: the gap is routed by the table, its fix lands as a reviewed spec, the run resumes, and the owner's checklist is reached. [paraphrase] Errors: a route the owner reverses is recorded as a reversal, not a failure; the criterion fails only if the run cannot resume after the fix lands.

## Boundaries
<!-- scope: business -->

- Jev never writes an option, a fix or a number. It answers questions over options the agent or a stronger model wrote. [paraphrase]
- No generator change inside a species run; every fix is its own reviewed spec. [paraphrase]
- No visual verdict by any route; the owner ticks the checklist. [paraphrase]
- No preference for the cheapest model. The model per run and the stronger model per route are the owner's routing; the loop records which ran. [user]
- fn-58's stages, manifest drafting and decision list are consumed, not changed. [paraphrase]

## Decision Context
<!-- scope: both — conditionally substructured -->

### Motivation
<!-- scope: business -->

The owner wants to "tell it to add species A" and get the species "properly QA'd", with the generator "improved to fix missing gaps", and wants Jev to "evaluate the options on how to fix the gaps" and "if the decision needs a better reasoning model or human". [user] The loop is that instruction made concrete, and the min-max framing, "as autonomous as possible but also as efficient and high quality as possible", is why the route is a table with recorded signals rather than a judgment: the thresholds are the one dial, the reversals show where it sits wrong, and moving it costs no new calls. [paraphrase] The loop is separate from fn-58 because its finish line is a real gap handled end to end while fn-58's is validation on two existing species, which both Jev and the standing reviewer weighed the same way; the owner's view that the machine serves the conductor stands, with the conductor's surface shipping as a skill file in the repo. [paraphrase]

## Strategy Alignment

- Serves "Growth and botanical fidelity": a species that the generator cannot yet express becomes a generalizing capability rather than a one-off, with the question set asking which option serves other species. [strategy:Growth and botanical fidelity]
- Serves "The core and integration": generator changes land as reviewed specs with small boundaries, never as patches inside a species run. [strategy:The core and integration]

## Parked unknowns

- Which fn-34 gap decisions the owner actually took, needed as the labels for R2; they live on the fn-34-integration branch and are not in this checkout.
- Which stronger reasoning model the escalation route uses; the owner's routing changes with quotas, so the loop reads it from the routing block rather than fixing it here.
