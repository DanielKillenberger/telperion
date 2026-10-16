# The gap loop's first autonomous live run

## Goal
fn-63 landed the gap loop (PR #38, 2026-09-18) with six of seven criteria evidenced on a fixture transport; the seventh, one real species end to end, could not be met inside it. The owner chose to ship the machinery and carry the live run here. This spec was that run, riding fn-82's date palm: a real halt routed by the table, the fix minted as its own spec, the run resumed where it stopped, and an automated-ready packet reaching the owner. Under the 2026-09-19 autonomy goal it also had to show model allocation by Jev route and a timely human escalation on a dead end.

## Decisions
- > user: "ideally i'd like the whole loop to be basically autonomous with a lot of judgement being done quickly and implementation to close gaps being done and I can just come back to review a new species template that looks awesome and what I would expect and i can just tick it off."
- > user: "efficiency is mostly about choosing the right tool for the job. judgements/decisions/routing should be done by jev wherever possible to keep llm token costs low. Then the agent should be a cheap effective model. But when a gap turn up and something is judged complex (by jev i imagine) then we escalate to a high reasoning model with medium/high effort to design the spec. Then we judge who can implement the design. If the design has done most of the complex work ahead of time we can probably use a cheap model. If the implementation is complex then we should probably use a high reasoning model on low effort."
- > user: "That's mostly what i meant with efficiency. Makes sense? do we need to adapt the specs? can you review them over? i really don't think we need to save on jev calls with fn-88"
- > user: "ok" (retire fn-88; amend fn-89, fn-68 and fn-80).
- > user: "the proposed flow is a straight line but it should contain a loop with automated visual readiness"
- > user: "there should also be a path to escalate to human. We don't want to burn tokens on dead ends etc. So we should have judgement calls evaluating certainty of success within reasonable token amounts. If there's high uncertainty or unusually high complexity or other implementation risk loop should escalate."
- Owner, 2026-09-18: merge fn-63 and carry its seventh criterion here; the species is the date palm, chosen as a deliberate curve ball over the recommended European ash, knowing several gaps would route to the owner.
- Owner, 2026-09-19: fn-89's model-allocation policy and fn-68's visual readiness are prerequisites; fn-88 retired; a supported catalogue species is the routine-path control. No gap is fabricated; extra captures need a budget exception.

## Acceptance Criteria
- **R1:** A real species run halts on a gap the loop can take, with capability line, options and judgments — met (palm halted on `apical-rosette`, `pinnate-frond`, then trunk organs).
- **R2:** The gap is routed by the table with matched row, signals and version — met.
- **R3:** The fix lands as its own reviewed spec minted through the loop — met (fn-109 apical rosette, fn-110 trunk organs, designed by a high-reasoning tier and implemented under the allocation policy).
- **R4:** The run resumes at the halted stage and reaches fn-68's ready handoff through at least one assess-fix-reassess cycle — superseded.
- **R5:** The run's numbers and report are complete — superseded.
- **R6:** The table is tuned once from reversals — superseded.
- **R7:** The palm's gap exercise records design and implementation routes, tiers and efforts — partial (fn-109 and fn-110 design and implementation dispatches recorded on the run branch).
- **R8:** A supported catalogue species replays to readiness through the cheap conductor — superseded.
- **R9:** End-to-end time, cost and tokens by role, tier and effort — superseded.
- **R10:** A labelled dead-end replay demonstrates timely human escalation — superseded.

## Result
Closed as superseded (2026-09-26): the palm shipped in 0.1.3 (#115), and the runner this run exercised was rebuilt by fn-149 (#121). The branch fn-80-the-gap-loops-first-live-run stays on GitHub as the palm run's archive; its PR #61 closed unmerged. Lasting economics finding (2026-09-18): the capability question is answerable before the literature spend (the ash spent 107 credits before its gate; the palm's Corner model was knowable for zero), so a capability pre-flight before `discover` turns a catalogue into a cheap sort into "drawable now" and "waiting on a named gap".

## Open
None here; the runner rebuild is fn-149's.

## Evidence
- `.flow/evidence/fn80/profiles-date-palm.json` (date-palm reference profiles; read by `scripts/species-profiles.mjs` and the palm tuning fixture)
