# The gap loop's first autonomous live run

## Conversation Evidence

> user: "i want you to review all specs that reload to the add-species pipeline. Find ways to simplify and make it as efficient to run as possible. Make sure we"
> user: "make sure we're aligned on the goals with this pipeline. See if you can understand my goals/targets clearly and make sure we're on the same page"
> user: "ideally i'd like the whole loop to be basically autonomous with a lot of judgement being done quickly and implementation to close gaps being done and I can just come back to review a new species template that looks awesome and what I would expect and i can just tick it off."
> user: "i also want you to evaluate how well jev fits into this pipeline. I feel a lot of judgement calls should be doable by it quickly and cheaply such that even cheap models can make the right decisions. Like escalating to a frontier model when a gap requires a complex new feature to be planned and implemented."
> user: "ok $flow-next-flow towards this pipeline I think you understood my intention well."

> user: "1" — selected the proposed allocation: two new specs and amendments to fn-68 and fn-80.

> user: "efficiency is mostly about choosing the right tool for the job. judgements/decisions/routing should be done by jev wherever possible to keep llm token costs low. Then the agent should be a cheap effective model. But when a gap turn up and something is judged complex (by jev i imagine) then we escalate to a high reasoning model with medium/high effort to design the spec. Then we judge who can implement the design. If the design has done most of the complex work ahead of time we can probably use a cheap model. If the implementation is complex then we should probably use a high reasoning model on low effort."
> user: "That's mostly what i meant with efficiency. Makes sense? do we need to adapt the specs? can you review them over? i really don't think we need to save on jev calls with fn-88"
> user: "ok" (accepted the recommendation to retire fn-88 and amend fn-89, fn-68 and fn-80).

> user: "the proposed flow is a straight line but it should contain a loop with automated visual readiness"
> user: "there should also be a path to escalate to human. We don't want to burn tokens on dead ends etc. So we should have judgement calls evaluating certainty of success within reasonable token amounts. If there's high uncertainty or unusually high complexity or other implementation risk loop should escalate."
> user: "ok then pls $flow-next-capture --rewrite them"

## Goal & Context
<!-- scope: business -->

fn-63 landed the gap loop on 2026-09-18 (PR #38): a species run's halt becomes typed options, one Jev request over them, a versioned table that routes the gap to the loop, to a stronger model or to the owner, a fix minted as its own reviewed spec, and a landing that expires the halted stage's idempotence key so the run resumes there. Six of its seven criteria were evidenced against a fixture transport. The seventh asked for one real species end to end and could not be met inside it: under the one-species-per-spec rule that needs a species onboarding, a generator spec worked and reviewed, and the owner's verdict on stills. [paraphrase]

The owner decided on 2026-09-18, on being shown the six-of-seven reading, to ship the machinery and carry the live run here rather than hold the loop unmerged or drop the criterion. The reason is that holding it blocks the very run that would exercise it. [user]

This spec is that run, riding the date palm of fn-82. It proves the loop when the halt is real: a species stops, the loop routes the gap, the fix lands as its own spec, the run resumes where it stopped, and an automated-ready packet reaches the owner. Its finish line is demonstrated autonomous loop behaviour through readiness; fn-82 retains the species' final acceptance. [paraphrase]

## Architecture & Data Models
<!-- scope: technical -->

- **This spec onboards no species.** The species it rides is onboarded by its own spec, which owns the manifest, the value tables and the owner's checklist. This spec observes that run and completes the loop around it; every artifact it writes lives under the gap loop's own records and the run's evidence tree. [paraphrase]
- **The gap is whatever the run files.** The loop takes an `onboarding-gate` or `level-miss` decision as its halt. No gap is planted, and a run that reaches the checklist without halting does not satisfy this spec; it is reported and the next species carries it. [inferred]
- **The fix is minted through the loop, not by hand.** The spec the loop mints is worked and merged under the repo's ordinary gates, with the route recorded before the spec exists, so the record shows the loop chose and a person did not. [paraphrase]
- **The thresholds are tuned once, from reversals.** Every route the owner disagrees with is recorded as a reversal by decision id. After the run, the table moves once from those reversals, and the recorded gaps are re-routed through the loop's own reroute path with no new Jev call, with the before and after recorded. [paraphrase]

## Acceptance Criteria
<!-- scope: both -->

- **R1:** One real species run halts on a gap the loop can take, and the gap record carries the capability line the halt names, its options and the judgments over them. [paraphrase] Errors: a run that reaches the checklist without halting is reported and does not satisfy this spec.
- **R2:** The gap is routed by the table, and the record names the matched row, the signals read and the table version. [paraphrase] Errors: an owner reversal is recorded and retained; a run requiring an exceptional owner decision is not proof of unattended completion.
- **R3:** The chosen fix lands as its own reviewed spec, minted through the loop and merged under the repo's ordinary gates, with the route recorded before the spec existed. [paraphrase]
- **R4:** After verified integration, the species run resumes at the halted stage, preserves unaffected artifacts and returns to matched rendering and automated visual assessment. The live evidence shows at least one assessment-to-refinement-or-gap-fix-to-reassessment cycle before reaching fn-68's ready handoff without routine owner intervention. Errors: a linear build-to-checklist trace, failure to resume, unresolved defects or an incomplete run does not prove autonomous visual refinement. [inferred]
- **R5:** The run's numbers are written and its report reads complete: gaps by route with the share the loop took, rounds to acceptance, reversals by decision id, and the run's tokens, wall clock, Jev calls, credits and captures. [paraphrase]
- **R6:** The table is tuned once from the run's reversals, the recorded gaps are re-routed with no new Jev call, and the before and after are recorded. [paraphrase] Errors: no reversals means no tuning, recorded as such. Replay cases used to tune policy are not its independent held-out test; original observations are preserved. [inferred]

- **R7:** Keep fn-82's date palm as the real gap exercise. Record the missing capability, Jev's design-complexity route, selected designer and effort, the resulting design, Jev's separate post-design implementation route, selected implementer and effort, verification/integration and return to the cheap conductor through visual readiness. Errors: an inherited implementation tier without reassessment does not prove the allocation policy; routine owner intervention or an unresolved exception does not pass the unattended proof. [inferred]
- **R8:** Replay a supported catalogue species without declaring it newly onboarded. It reaches the same readiness handoff through the cheap conductor and Jev decisions when no novel design is required. Exercise complex-design-to-cheap-implementation and still-complex-implementation routes in labeled cases; distinguish these from the live run and do not fabricate a live gap to force every route. Errors: record every stronger-model call with its role and effort rather than calling the run cheap despite unexplained escalation. [inferred]
- **R9:** Report end-to-end time, cost and input/output tokens by role, tier and effort, alongside Jev usage, interruptions, retries, rejected candidates, wrong routes and the first owner verdict. Assess model allocation against verified quality; reducing repeated Jev calls is not a success criterion. Keep tuning cases separate from independent route evaluation and preserve original observations. Errors: unknown costs remain unknown, unmeasured savings are not claimed, and an incomplete run cannot satisfy the real-run proof. [inferred]

- **R10:** A separate labeled dead-end or high-risk replay demonstrates timely human escalation while token budget remains, before another unjustified render, tuning round or design/implementation dispatch. Record the uncertainty/progress/risk evidence, bounded cost estimate or its absence, spend at pause and the actionable handoff; verify no further work dispatches until a scoped human decision and that resume preserves spend. Errors: waiting for exhaustion, forcing a frontier attempt first, or automatically resuming fails this criterion. Label replay evidence separately; a correct early stop passes this safety proof but cannot replace the successful live autonomous-run proof. [inferred]

## Boundaries
<!-- scope: business -->

- No species is onboarded here; the species spec owns its manifest, its value tables and its checklist. [paraphrase]
- No change to the loop's code except where the live run proves a defect, and such a change is named in the run's record. [paraphrase]
- The owner's visual verdict stays the owner's; no route decides it. [paraphrase]
- The captures are the species spec's, under the standing budget. This spec adds none beyond the checklist's own stills. [paraphrase]

## Decision Context

- The owner's loop and human-escalation clarification requires two distinct proofs: successful autonomous visual refinement and refusal to burn tokens on an unjustified continuation. A safe pause is correct escalation behavior; it does not claim that the paused species reached readiness. Existing captures and recorded or labeled replay inputs supply the dead-end case; no live gap is manufactured. [inferred]

- The owner's 2026-09-19 correction makes fn-89's model-allocation policy and fn-68's visual readiness the prerequisites. fn-88 is retired without implementation. This proof observes fn-82 rather than waiting for its final acceptance and uses a supported catalogue species as the routine-path control. [paraphrase]
- The control reuses existing evidence and the total recorded budget; any additional required captures need an explicit budget exception. No gap is fabricated to manufacture a successful proof. [inferred]


- The owner chose on 2026-09-18, from three options put to them, to merge fn-63 and carry its seventh criterion here. [user]
- The species is the date palm of fn-82, chosen by the owner on 2026-09-18 as a deliberate curve ball over the recommended European ash. [user] A palm is a monocot: it does not branch, it has no secondary thickening, and its leaf is a frond, so it asks the generator for capabilities at its core rather than at its edges. The owner was told before choosing that this is not one gap and that several are likely to route to them because they touch rules shipped presets depend on. [paraphrase]
- The consequence accepted with that choice: this spec's finish line may sit behind a chain of generator specs rather than one, and a run that stalls at an owner route remains recorded evidence, but under the 2026-09-19 autonomy goal it does not satisfy the unattended-run proof. [inferred]

## Open Questions

- None. The species question was answered by the owner on 2026-09-18: the date palm of fn-82.

## Settled

Closed as superseded (2026-09-26): the palm shipped in 0.1.3 (#115), and the runner this run exercised was rebuilt by fn-149 (#121). The branch fn-80-the-gap-loops-first-live-run stays on GitHub as the palm run's archive; its PR #61 closed unmerged.
