# Tuning loop: measured candidates and automated visual readiness

## Goal

The owner tuned species by hand rounds (26 on the birch, 23 on the beech in fn-34), with the host model reading each verdict and guessing rows. On 2026-09-18 the owner asked whether Jev could take the parameters and research and return a set that gets closer, with a model judging each render. A probe on the beech (`experiments/fn58-tuning-loop/`) showed a code sweep over twelve dials took the five-number distance from 0.245 to 0.145 in 74 evaluations and that Jev choosing one direction per dial over the owner's verdict reached 0.154 in 13. This spec turns that into the loop a species round runs: code steps the dials, Jev routes sourced observations to directions, a vision reviewer judges progress and readiness, and the owner's eye gives final acceptance.

## Decisions

- user: "i want you to review all specs that reload to the add-species pipeline. Find ways to simplify and make it as efficient to run as possible. Make sure we"
- user: "make sure we're aligned on the goals with this pipeline. See if you can understand my goals/targets clearly and make sure we're on the same page"
- user: "ideally i'd like the whole loop to be basically autonomous with a lot of judgement being done quickly and implementation to close gaps being done and I can just come back to review a new species template that looks awesome and what I would expect and i can just tick it off."
- user: "i also want you to evaluate how well jev fits into this pipeline. I feel a lot of judgement calls should be doable by it quickly and cheaply such that even cheap models can make the right decisions. Like escalating to a frontier model when a gap requires a complex new feature to be planned and implemented."
- user: "ok $flow-next-flow towards this pipeline I think you understood my intention well."
- user: "1" — selected the proposed allocation: two new specs and amendments to fn-68 and fn-80.
- user: "efficiency is mostly about choosing the right tool for the job. judgements/decisions/routing should be done by jev wherever possible to keep llm token costs low. Then the agent should be a cheap effective model. But when a gap turn up and something is judged complex (by jev i imagine) then we escalate to a high reasoning model with medium/high effort to design the spec. Then we judge who can implement the design. If the design has done most of the complex work ahead of time we can probably use a cheap model. If the implementation is complex then we should probably use a high reasoning model on low effort."
- user: "That's mostly what i meant with efficiency. Makes sense? do we need to adapt the specs? can you review them over? i really don't think we need to save on jev calls with fn-88"
- user: "ok" (accepted the recommendation to retire fn-88 and amend fn-89, fn-68 and fn-80).
- user: "the proposed flow is a straight line but it should contain a loop with automated visual readiness"
- user: "there should also be a path to escalate to human. We don't want to burn tokens on dead ends etc. So we should have judgement calls evaluating certainty of success within reasonable token amounts. If there's high uncertainty or unusually high complexity or other implementation risk loop should escalate."
- user: "ok then pls $flow-next-capture --rewrite them"
- user: "alright a sweep sounds very inefficient though? doing renders and assessing them for tiny steps?"
- user: "do we need the sweep fallback? if jev gets us there much faster. Can we have jev say how much a parameter should change?"
- user: "ok" (accepted removing automatic sweeping, testing bounded direction-and-magnitude proposals, and retaining historical sweep results only as a comparison).
- Autonomous-loop amendment (2026-09-19): no automatic sweep; code evaluates, Jev routes text, a demonstrated-capable vision model inspects images; fn-89 owns continuation policy and model routing; fn-88 retired.
- Human gap-priority checkpoint (owner, 2026-09-20): after initial review the owner confirms, reorders or adds the top gaps (what matters, not values); approval is hash-bound to the packet and scope.
- Reference-first review and roles (owner, 2026-09-20): Astra medium for discovery and final readiness, Opus for targeted intermediate checks, Jev for text routing; Astra provisional. Joint visual diagnosis: the reviewer finds gaps without being shown the owner's assessment.
- Visual bootstrap mode (owner, 2026-09-21): "this makes sense to me". No owner-accepted positive existed (the owner rejected the birch reframe: "yes it looks too dark and streaky and yes the top crown is a bit more leafy"), so readiness is never awarded in bootstrap; exit is the first owner-accepted render passing one comparison.
- R11 code-first continuation (owner, 2026-09-21): "ok r11 change makes sense". Bounded first rounds proceed without judgment; one uncalibrated question only when evidence has moved.
- Reviewer-judged selection (owner, 2026-09-21): the five numbers rejected every move aimed at the owner's complaints. Owner: "that doesn't seem like a generalizable approach i thought we'd have a model judge the picture for progress and then say what was good/miss and then we keep iterating or deciding that there's a feature gap we need to close." then "yes". The numbers became telemetry.
- Bundle search (owner, 2026-09-21): hanging dials were inert one at a time (`.flow/evidence/fn-68-tuning-loop-code-steps-the-dials-jev/hanging-inert-on-beech-2026-09-21.md`). Owner: "is there some sort of multidimensional binary search we can do?", "so still approach 3 with the bundle that jev identifies?", "ok go". A round is one bundle of Jev-supported dials at four strengths, judged on one shuffled contact sheet, halved by family when it breaks something.
- Contract (owner, 2026-09-22): "so this assessment tuning loop should end with the end result + gap list and the invoker would decide if the specs should be made yea?", "so either me or later the add-species agent", "ok that works". Then: "did we fix that the runner would produce the gap list? i think we can define a endresult artifact that includes the run outcome and the gaps at the end for easy review and it makes sure nothing gets forgotten." (built: `result.json`/`RESULT.md` per run, `tuning-loop result`).
- Owner's correction on the close-out (2026-09-22): "i was expecting fn-68 completion would be able to come up with these kinds of gaps ... or at least propose them ... that was kinda the point otherwise the next spec that runs the add species pipeline won't have specs to make to close gaps." A tuning close-out without proposed gap specs has not finished.

## Acceptance Criteria

- **R1:** loop command proposes overlays, measures, renders matched stills, records every trial with hashes and ledger references; refused candidates infeasible, never rendered — met (`tuning-loop run`).
- **R2:** Jev selects named directions only; code computes values; reviewer's comparative verdict selects; failures never sweep — met as amended (bundle at up to four strengths plus splits).
- **R3:** weighted mean relative distance as score — met as telemetry, never selecting.
- **R4:** round stops on budget or no better candidate; finalists kept as matched pairs; owner gives final acceptance — met as amended.
- **R5:** versioned, evaluated adjustment question — partly met (direction re-qualified per table: 7/8, 4/5, 4/4; magnitude became authored strengths judged on the sheet).
- **R6:** overlay, `--family` and `jev ask` tested; loop outside workspace tests — met (`scripts/test-tuning-empty-overlay.sh`, isolation guard).
- **R7:** bounded beech pilot against the 0.154 direction and 0.145 sweep baselines — run, not passed (no numeric win claimed; it validated bundle routing on reviewer verdicts).
- **R8:** demonstrated-adequate vision reviewer emitting readiness apart from acceptance — open (bootstrap; gpt-6-astra medium; no owner-accepted render to qualify against).
- **R9:** Jev routes defects to dials, gaps, investigations or appearance; numeric gains never erase visual defects — met.
- **R10:** refine until readiness or resumable escalation — mechanism met, readiness never reached.
- **R11:** continuation policy at each round boundary — met as amended (code-first).
- **R12:** hash-bound gap-priority packet with scoped owner confirmation — met, used live in every run.

## Result

Seven live runs on the beech at seed 1 (bootstrap pilot, wide table, reviewer-judged, bundle search 1 and 2, materials, final). The final run went 44 rounds unattended (216 evaluations, 1,128 images, 97 visual passes): two adoptions kept (rounds 38 and 42), four rolled back, stalled with every structure family excluded; five gaps, one handed off, four stalled in tuning (`.flow/evidence/fn-68-tuning-loop-code-steps-the-dials-jev/final-run-RESULT.md`). Owner's eye, 2026-09-22, on bundle 1 against rounds 38 and 42: "ok i think it looks better!"; "The materials from afar doesn't look like the detailed view?"; "you have two massive strong stems that grow super high that only taper towards the top. And branches at the top taper more? ... Currently it seems like the dials can only achieve thin branches if they taper evenly all over. This needs a new parameter i'd say."; "the left most packet clearly attempted to have the strong center stem." Proposed gap specs (host assessment): leaders keep their girth at forks (drafted as fn-103), foliage as branch-associated masses, leaf-weighted droop of the outer crown, smooth grey bark that reads the same near and far. A prior Jev-labelled retention pass dropped 37 files on 2026-09-22.

## Open

- Whether round 42 is the owner-accepted positive that exits bootstrap mode (R8).
- Material-track split renders only the trunk view, so its halves never reach the sheet.
- `tuning/live.rs` (1,001 lines) and `engine.rs` (770) exceed the file-size rule.
- The ignored end-to-end test `the_real_command_reaches_the_priority_pause_then_routes_and_writes_handoffs` is stale since the bundle redesign (needs a fixture answering the bundle question).

## Friction

Seventy entries grouped into ten causes by the host's 2026-09-21 review; the owner decides which become specs.
- A. Gates admitted formats no shipped command produced (9) -> a rule and test: an admitted format must be emitted by a shipped command, with an offline fixture for every pause.
- B. Reservations byte-sized, ~4x likely spend (11) -> a measured bytes-per-token bound; estimators return errors, never defaults.
- C. Vision spend not importable through `ExternalUsage` (2) -> an attested vision usage record.
- D. One identity named both configuration and evidence (2) -> separate names.
- E. Judgment contracts shaped wrong (11) -> fixed in fn-68, no spec.
- F. Prompts leaked expected answers (6) -> fixed; a test that no dispatched prompt carries a label or identifying path.
- G. Local and orchestration overhead (13) -> gate logs under the worktree's ignored scratch, ci profile only for crate suites.
- H. Dial table reconstructed from outside the core (4) -> one authored parameter table in `telperion-core` (later fn-152).
- I. CI cannot run the offline end-to-end (no `uv`) -> add `uv` or an in-process compare seam.
- J. Calibration and historical evidence (14) -> superseded by bootstrap mode, no spec.

## Evidence

- `.flow/evidence/fn-68-tuning-loop-code-steps-the-dials-jev/final-run-RESULT.md` (outcome and gap list), `contact-sheet-protocol.md`, `pilot-config.json`, `cases/` (labelled direction, magnitude and continuation sets), `vision-replay-v2.json`, `vision-v2-result.json`, `v3-generated/manifest.json`, `v4-generated/manifest.json`, `bundle-search-run-2-2026-09-21.md`, `hanging-inert-on-beech-2026-09-21.md`, `claude-comparison.py` with `joint-blind-request.json`, `joint-blind-audit.json`
