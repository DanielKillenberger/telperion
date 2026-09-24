# A species run finishes: identity blocks, improvements are captured

## Conversation Evidence

> user (2026-09-24): "i think we need to have a end state where if the generator is missing a feature/attribute that applies to many tree species even ones added already to get this tree to be "realistic" then we capture it but don't expect it to be done for this run. Otherwise we just run towards perfect trees on every single run and never end? Or how do you see the finish condition?"
> user (2026-09-24): the host classifies at capability assessment ("Host at capability assessment (Recommended)"); the palm's date clusters are backlog ("Backlog (Recommended)").

## Goal & Context
<!-- scope: business -->

A species run needs a finish line. It finishes when (1) its literature passes, (2) the generator expresses every capability the species needs to be recognisable at catalogue quality, (3) tuning has converged on what the generator can draw, and (4) the owner ticks the checklist. A missing capability that only adds realism, especially one that would serve many species, is captured as a backlog spec and listed as a known gap; it never blocks the run. The host classifies each missing capability as identity or improvement at the capability assessment, with its reason, and the owner can reverse any classification in the packet. [user]

## Architecture & Data Models
<!-- scope: technical -->

- **What exists, checked 2026-09-24 on the fn-80 branch.** [checked]
  - The host's capability assessment is `.flow/evidence/fn80/capability.json` (keys `assessed`, `required_capabilities`, `shape`, `sources`, `species`, `taxon`, `traits`; each trait has an `outcome` and a `capability`).
  - The gate stage (`pipeline/stages/gate.rs:104-180`) splits the required capabilities into expressed, missing and unrecognised, and a missing one files the blocking `onboarding-gate` decision; the palm's gate lists `infructescence` missing.
  - Tuning readiness (`tuning/state.rs:70` `ready`) and the reference-first core-coverage gate (`tuning/reference_first.rs:657`) refuse a failing core trait; fn-116's `unexpressed` list removes a trait's flip from the veto but not from readiness.
- **Shape.** [host design]
  - *Class on the assessment.* Each missing capability in the assessment carries `class: identity | improvement`, a reason, and for an improvement the backlog spec that captures it.
  - *The gate blocks on identity only.* A missing improvement capability files no blocking decision; it is recorded as a known gap with its spec. A missing identity capability blocks as today.
  - *Readiness counts what can be drawn.* A reference trait tied to an improvement capability is listed `unexpressed` (fn-116) automatically and is left out of readiness and core coverage, recorded as a known gap, not as a failure.
  - *Converged is a finish.* When every drawable objective passes, or the tuning run's no-progress guard (fn-117) stops it with nothing kept, the conductor assembles the packet for the owner rather than asking for another revision; the packet's checklist lists the known gaps with their specs.
  - *The palm.* `infructescence` is an improvement captured by fn-33 (the organ slot) and fn-111 (the date cluster); `fruit-clusters-pendent` leaves the palm's readiness.

## Acceptance Criteria
<!-- scope: both -->

- **R1:** An assessment with a missing improvement capability passes the gate and records the known gap with its spec; a missing identity capability still blocks. [inferred]
- **R2:** A reference trait tied to an improvement capability is left out of `ready` and core coverage and listed as a known gap; the palm's `fruit-clusters-pendent` no longer blocks machine readiness (test on recorded state). [inferred]
- **R3:** A tuning run that stops on no progress with every drawable objective passing, or on the guard with nothing kept, leads the conductor to assemble the packet; the checklist lists every known gap. [inferred]
- **R4:** The palm's assessment records `infructescence` as `improvement`, captured by fn-33 and fn-111, with the owner's decision of 2026-09-24. [inferred]
- **R5:** The gate is green: `cargo test --profile ci --workspace --no-fail-fast`. [paraphrase]

## Boundaries

- The classification is the host's, reversible by the owner; no Jev question decides it. [user]
- No change to the literature step. [inferred]

## Decision Context

- The owner set the finish line and the classifier on 2026-09-24. [user]

## Open Questions

- None.
