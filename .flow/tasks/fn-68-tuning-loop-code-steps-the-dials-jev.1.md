---
satisfies: [R1, R2, R3, R4, R5, R6, R7, R8, R9, R10, R11, R12]
---
# fn-68-tuning-loop-code-steps-the-dials-jev.1 Implement measured tuning and automated visual readiness

## Description
Build the reviewer-judged tuning loop the amended spec describes and run it live on the beech.

## Acceptance
Every R-ID in the parent spec's acceptance is satisfied; judged against the spec's criteria directly.

## Done summary
Jev routes priorities and reviewer notes to dial directions; code builds one bundle a round at four strengths; one contact-sheet review grades it; code adopts the smallest better strength; an all-view veto rolls back regressions; worse bundles are halved by family; repeats refused; continuation decided in code; the five numbers are telemetry. Seven live beech runs, ending with a 44-round unattended run (two adoptions kept, four rolled back). R1-R4, R6, R9, R11, R12 met; R5 partly; R7 run, not passed; R8 open (bootstrap); R10 mechanism live, readiness never reached. Not done: materials splits never shown to the sheet; live.rs and engine.rs over the size rule; the ignored end-to-end test is stale.

## Evidence
- Commits: bcf9b5cd, d219baa1, ecfe0a43
- Tests: env -u TYPESAFE_API_KEY cargo test --profile ci --workspace --no-fail-fast; cargo test --profile ci -p telperion-jev --test tuning_command -- --ignored (FAILS: stale fixture)
- PRs:
