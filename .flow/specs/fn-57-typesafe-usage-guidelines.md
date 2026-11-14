# TypeSafe usage guidelines

## Goal

The owner installed TypeSafe on 2026-09-16 and asked for a survey of where a typed Jev judgment could stand in for fragile parsing or heuristics, live experiments, and a spec that prepares the repo with usage guidelines so the next agent applies Jev where it earns its place and nowhere else. The owner's standing rule fixes the poles: Jev never touches generation (determinism of pins, presets, seeded streams); it earns its place screening source literature, where code finds candidate numbers and owns every calculation and the model only judges which candidates are measured values at a stated age and condition.

## Decisions

- user (turn 1): "we just installed typesafe-ai Using the TypeSafe skill, explore the project and find opportunities for using intelligent judgement to stand in for complex parsing or other fragile code."
- user (turn 1): "From that we want to capture a spec to prep repo for typesafe usage guidelines."
- user (turn 2): "you can also do this: Using the TypeSafe skill, run some experiments using the TypeSafe API key that I've exported to `TYPESAFE_API_KEY`. Propose changes based on the most promising results."
- One shared caller and one ledger entry per call; code finds, the model chooses an option key, code copies (a `none` option always present); four question sets kept as data; compose screen and citation check, thresholds from the labelled set; judgments never write state.
- Jev judges text and structured state, never a still; the owner's eye stays the visual verdict. The two plain parsing bugs found (hand-rolled JSON scan in a foliage test, stdout regex in species QA) are code fixes, not Jev work.
- Pilot finding: a citation check alone passed a claim restating the O1 site criterion as a typical rate (1.00); the kind screen catches it, so the two compose.

## Acceptance Criteria

- **R1:** usage guideline a new agent can follow — met (`docs/typesafe.md`).
- **R2:** one shared caller with interactive-shell key, backoff, ledger entry per call — met (`crates/telperion-jev`).
- **R3:** literature screen and selection tool; 12/12 kind, 5/5 selection, O1 lands as site criterion — met.
- **R4:** citation check composing with the screen; O1 misuse listed for the owner, true claims pass — met (9/9).
- **R5:** triage tool routing, duplicates, severity, writing nothing — met (8/8 routing, 4/4 severity).
- **R6:** guard keeping Jev out of core, render, Wasm and browser source; workspace tests pass with the key unset — met (also fixed a walk that skipped `src/browser/render`).
- **R7:** labelled cases, sixteen-site inventory and results recorded; pilot scripts not shipped — met (cases in `crates/telperion-jev/data/cases`; inventory and results under `.flow/evidence/fn57/`).

## Result

The pilot (model `jev-latest`, 0.56 to 0.78 s per request, under 60,000 input tokens over 44 requests) scored: screen 12/12 (O1 at 0.99 site criterion), citation 7/9 alone with O1 routed to a person at confidence 0.26, selection 5/5 after the candidate regex learned "foot", routing 8/8, duplicates separated, severity 4/4, report-section selection 3/3. The shipped rewrite reproduces these in `cases-live.txt`. Review round 1 (cursor gpt-5.6-sol-high) NEEDS_WORK on 8 findings, all verified fixed with named tests. 39 jev tests pass with the key unset; `npm test` 77 passed. The owner was asked to rotate the TypeSafe API key after a shell fallback printed it into a session transcript.

## Open

Whether TypeSafe exposes a pinned model version for ledger entries (parked).

## Evidence

- `.flow/evidence/fn57/README.md`, `INVENTORY.md`, `cases-live.txt`, `pilot/RESULTS.md`, `pilot/SOURCES.md`, `pilot/jev.py`
