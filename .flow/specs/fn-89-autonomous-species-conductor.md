# Autonomous species conductor

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
<!-- Source: owner intent paraphrased; execution contracts inferred from the reviewed implementation. -->

The owner asks for one species and returns to a convincing mature-tree template, its botanical documentation, and a compact review packet. The system carries the research, routine choices, generator-gap work, tuning and validation without asking the owner to manage intermediate steps. A cheap conductor runs routine work while Jev supplies bounded semantic judgments and routing. Complex design goes to a high-reasoning model at medium/high effort. Once the design exists, Jev reassesses the remaining implementation complexity and selects a cheap implementer or a high-reasoning implementer initially at low effort. The owner supplies final acceptance. [paraphrase]

The current machinery already provides research stages, typed decisions, gap records, capability vocabulary, catalogue records and matched renders. This change connects those mechanisms into one persistent run. After initial evidence review the owner confirms/reorders/adds priorities once, then routine work continues under policy without asking the owner to choose implementation mechanics. Final acceptance and risk/stall/scope-change escalation remain human boundaries. [user decision, 2026-09-20, relayed by host]

## Architecture & Data Models
<!-- scope: technical -->

- Code owns stage eligibility, freshness, budgets, dispatch tracking and resume. An agent cannot turn a failed prerequisite into a pass through narration. Existing stage artifacts, decisions and Flow dependency records remain authoritative; the conductor records only the additional run/dispatch state needed to recover. [inferred]
- Jev sees bounded evidence containing the failed requirement, observed results, relevant capability definitions, existing matching specs and applicable owner rulings. Diagnosis support, option fit, design complexity and post-design implementation complexity are distinct judgments. The latter consumes the completed design and relevant code evidence, rather than inheriting the initial gap's difficulty. Missing evidence remains unknown. [inferred]
- A cheap effective agent conducts the run, gathers evidence and executes routine work. Jev handles bounded semantic decisions and routing wherever suitable; code owns exact checks and policy execution. A large-model analysis is not a prerequisite to each Jev decision. Complex design is dispatched to a high-reasoning model at medium/high effort; implementation is selected separately after design, using a cheap model for straightforward work and a high-reasoning model initially at low effort for complex implementation. Each completed escalation returns control to the cheap conductor. [paraphrase]
- A design handoff resolves interfaces, invariants, difficult cases and verification expectations, and names remaining unknowns. Jev evaluates whether those unknowns require further design before choosing an implementer. The host retains orchestration and authority checks while the assigned reasoning model performs complex design. [inferred]
- A gap covered by an existing spec reuses that dependency. A novel generator capability gets its own spec. The conductor observes completion of the required work and the relevant integration checks before rerunning affected species stages. [inferred]
- Routine source admission, replacement of an unavailable source with equivalent evidence, correction of an unsupported article claim, and captures inside the run's existing budget are automatic under explicit policy. Lowering evidence standards, overriding owner rulings, changing product boundaries, or exhausting the total progress budget remains an exception. No intermediate resolution is attributed to the owner unless the owner made it. [inferred]
- Visual readiness is a feedback loop. Matched renders produce a visual assessment; Jev routes unresolved defects to tuning or a capability fix; verified changes rebuild and render for another assessment. Each proposed continuation passes the early-escalation policy before further work. The loop exits either to a ready packet after all checks pass or to a resumable human escalation. Only the owner's final verdict establishes acceptance. [paraphrase]

- The continuation judgment consumes the unresolved defect, diagnosis evidence, proposed action, recent progress and failed attempts, affected shared behavior, implementation risks, tokens spent and remaining, and a bounded next-attempt estimate with its basis and uncertainty. Jev separately judges tractability, progress and risk; code combines them with explicit policy and hard limits. Ordinary complex work with a supported bounded path may still use the reasoning tiers; unusually risky or uncertain work pauses for the human. No arbitrary success percentage is inferred from Jev confidence. [inferred]

## API Contracts
<!-- scope: technical -->

- A start or resume operation names one species and its existing or new species spec. It exposes the current stage, the blocking requirement if any, active dependency/dispatch, remaining budget, and the next executable action. Repeated resume observes existing work instead of creating duplicate workers or specs. [inferred]
- A dispatch result identifies its input evidence and design revision, assigned scope, routing judgments, selected role, model tier, actual model and effort, observed output and verification, or a typed failure. Results for obsolete inputs cannot advance the run. Model and effort selections are explicit and independently recorded. [inferred]
- A handoff exposes the preset, matched reference views, checklist results, sources and article, unresolved limitations, and actual run costs. It is marked ready for owner review only after the automated prerequisites pass. [inferred]

- A human-escalation handoff contains the current renders when available (otherwise the reason they are missing), unresolved requirement, attempted fixes and outcomes, actual spend, remaining budget, the proposed next action and estimated allowance, uncertainty/risk signals and the concrete decision requested. It persists a paused run; resuming needs an explicit scoped human decision, retains spent budgets and rechecks changed evidence. It launches no further tuning, design, implementation or rendering while awaiting that decision. [inferred]

## Edge Cases & Constraints
<!-- scope: technical -->

- A confident choice among weak proposals is not proof of feasibility. Incomplete diagnosis, conflicting evidence and unfamiliar changes must pass the continuation judgment before investigation is dispatched. Jev's distribution confidence is not a calibrated probability of engineering success; missing evidence or an unbounded cost estimate cannot establish a credible bounded attempt. Measured regressions override a proposed fix's claims. [inferred]
- A provider outage or unavailable model is recorded as infrastructure failure. An explicitly configured alternative of adequate capability may take over; absence of that alternative cannot silently demote design work to a cheap worker. [inferred]
- Failed verification supplies evidence to the continuation judgment before another design or implementation dispatch. If continuation is justified, unresolved design returns to design and difficult implementation can receive a stronger model or increased effort. Otherwise the run pauses for the human. Low effort is an initial allocation, not a mandate to repeat failures; Jev confidence cannot substitute for engineering evidence, tests or review. [inferred]
- Existing task, capability-round, value-round and capture budgets persist across resume and escalation; the run also records its configured total token allowance and per-attempt bounds. Code enforces the bounds; a restart does not replenish them. Jev evaluates whether the proposed next attempt has a credible path to success within a reasonable allowance before the limit is reached. High uncertainty, unusually high complexity, implementation risk or repeated no-progress can escalate directly to the human without first buying another frontier attempt. [paraphrase]
- The dispatch mechanism follows the host's existing implementation and landing authority. The conductor records missing external authority as an exception rather than treating a Jev answer as permission to merge. [inferred]
- The mature direct build remains the species target. Growth calibration and parity reporting do not become species acceptance gates. [inferred]

## Acceptance Criteria
<!-- scope: both -->

- **R1:** One species run advances through research, routine decisions, gap handling, tuning, validation and documentation without requiring the owner to conduct routine stages. Visual assessment feeds tuning or a capability fix, then verified changes rebuild and render for reassessment until ready or escalated. Errors: unresolved defects cannot escape the loop as readiness, and an unjustified next attempt pauses under R8. [paraphrase]
- **R2:** The run resumes after interruption without duplicating completed stages, open gap specs or active dispatches, and rejects results based on obsolete inputs. Errors: incomplete output is retained as an interrupted attempt and cannot satisfy a stage. [inferred]
- **R3:** Jev performs bounded semantic routing over explicit evidence, with separate decisions for design complexity and remaining implementation complexity after the design is available. Routine work uses the cheap agent; complex design uses a high-reasoning model at medium/high effort; straightforward implementation uses a cheap model and complex implementation starts with a high-reasoning model at low effort. Errors: no match, missing design evidence or contradictory judgments trigger investigation rather than a guessed cheap route. The consumed model and effort are recorded independently. [paraphrase]
- **R4:** A missing capability attaches an existing dependency or creates one scoped generator spec. After the continuation check, the selected designer produces an implementation-ready handoff; Jev then selects the implementer from the remaining complexity. Verified integration returns the cheap conductor to matched rendering and visual reassessment. Errors: design or implementation failure goes through R8 before another attempt; stale results and failed integration leave the dependency unresolved without duplicate specs. [inferred]
- **R5:** One explicit policy maps Jev judgments to routine work, design, implementation, recovery and human escalation, and current conductor instructions and consumers agree. Held-out cases cover routine progress, complex design followed by cheap implementation, still-complex implementation, high uncertainty, unusual complexity/risk and repeated no-progress with budget remaining. Record expected versus selected routes and test continuation and early-stop decisions separately. Errors: unvalidated judgments cannot authorize unattended continuation; policy cannot lower standards, override owner rulings, reset budgets or impersonate acceptance. [inferred]
- **R6:** A ready-for-review packet requires numeric validation, the automated visual-readiness result, reproducible matched views and checked documentation. Only the owner can accept the species. Errors: missing views, outstanding checklist defects or an unavailable visual judge keep the species unready. [paraphrase]
- **R7:** The report measures end-to-end elapsed time, interruptions, dispatches by role, model tier and effort, input/output tokens and cost when available, Jev calls/credits, captures, retries, wrong routes, failed fixes and waiting. It attributes expensive reasoning to design, implementation, visual assessment or review so allocation can be evaluated against verified outcomes; autonomous escalation counts as autonomous. Errors: unknown costs are explicit, service durations are not total wall time, failed attempts remain in totals and no saving is claimed without a measured comparison. [inferred]

- **R8:** Before each further tuning round or design/implementation dispatch, evaluate whether the proposed attempt has a credible prospect of success within a reasonable token allowance using evidence, progress, cost bounds and risk. High uncertainty, unusually high complexity, implementation risk or repeated no-progress routes to a resumable human handoff before another unjustified attempt, even with budget remaining. Errors: a hard-limit breach, missing basis for a bounded attempt or an unavailable/unvalidated continuation judgment pauses the run; no mandatory frontier detour, automatic retry or budget reset bypasses the pause. [inferred]

- **R9:** Consume fn-68's explicit human gap-priority checkpoint before proposal or fix dispatch, including initial model PASS. Present the proposed top three with reference/render evidence and retain all findings; record owner confirmation/reordering/additions scoped to the initial packet, species and objectives. The owner chooses what matters, while the conductor handles implementation mechanics under existing qualification, continuation and budget gates. Errors: missing/stale approval blocks; changed objectives/reference or finish standard requires renewed approval; ordinary rerenders preserve it; approval cannot resolve a gap, bypass explicit relevant-view coverage, reset spend or supply final acceptance. [user and host design, 2026-09-20]

## Boundaries
<!-- scope: business -->

- The species remains a value table over shared generator capabilities; a gap is its own generator spec. [strategy:The catalogue]
- Jev never writes a numeric preset value or implements a fix. Code owns calculation and measurement. The selected agent designs or implements within its assigned role; a high-reasoning design does not require high-reasoning implementation. [paraphrase]
- The owner retains final visual acceptance. An automated readiness result is not an owner verdict. [paraphrase]
- No general-purpose scheduler, second task database, replacement research pipeline or new catalogue format. [inferred]
- Candidate optimization and visual refinement belong to fn-68. Resume preserves valid completed stages and rejects obsolete results; cross-run caches, duplicate-Jev-call suppression and cache benchmarks are not prerequisites or deliverables. [paraphrase]
- No specific species or missing organ is implemented as part of the machinery. fn-82 owns the palm and fn-80 owns its end-to-end proof. [inferred]

## Decision Context

### Round boundary moved into code, 2026-09-21

- fn-68's R11 continuation gate is now a code decision, with one uncalibrated evidence-difference question asked only after a stall on moved evidence, and risk alone asked before a handoff; see "R11 code-first continuation (owner, 2026-09-21)" in the fn-68 spec. The conductor inherits that boundary and must not reintroduce a per-round judgment. [host design]

### Human priority selection, 2026-09-20

- The owner approved a conversational checkpoint after initial visual discovery: show the proposed top three gaps and evidence, accept confirmation/reordering/missed-gap additions, then autonomously plan, implement, tune and verify those objectives under existing gates. This supersedes fully unattended initial priority selection, not final acceptance. The host records the scoped decision; the owner need not hand-author transport JSON. [user, relayed by host]
- fn-68 owns the hash-bound packet, explicit approval and relevant-view verification contract. The conductor preserves all findings, treats owner priority as authoritative over model severity, and returns for scope change, risk, stall or final approval. Approval never proves a gap resolved or a reviewer qualified. Independent blind calibration remains separate. [host design]

### Provisional role routing, 2026-09-20

- Owner-selected roles: Astra medium for initial gap discovery and final readiness, Opus for cheaper targeted intermediate checks, Jev for text routing/progress. Astra is provisional until independent detection is established. fn-68 owns the reference-first inventory/comparison evidence contract; this does not authorize this conductor to bypass qualification, uncertainty, cost or resume gates. [user and host clarification]

### Joint-review handoff, 2026-09-20

- The owner asks for independent issue discovery from multiple references and renderings, without supplying the reviewer's expected conclusions. fn-68 owns the joint evidence packet, cross-view findings and blind evaluation. This conductor consumes those attributed findings under R1, R3-R6 and R8. [paraphrase]
- Routing preserves the distinction between an observed visual gap and a proposed cause. A design dispatch receives relevant cross-view constraints, factual generator capabilities and uncertainties. It cannot treat a reviewer's causal hypothesis as established mechanics or infer seasonal change from unrelated reference specimens or a foliage-visibility toggle. [inferred]
- A new generator capability still receives its own bounded design and implementation. Completing a manually tuned beech does not establish autonomous pipeline completion. Readiness, independent discovery quality and end-to-end cost remain separately evidenced. [inferred]

### Motivation

- The owner made the render-assess-fix loop and early human escalation explicit. Autonomous progress is preferred while supported by evidence; avoiding token spend on dead ends takes priority over forcing every run to finish unattended. A human escalation is a valid paused outcome, never a ready species. [paraphrase]

- The owner clarified the efficiency target on 2026-09-19. Model allocation, including a fresh implementation decision after design, replaces the earlier cache-led efficiency proposal. fn-88 is retired without implementation; ordinary resume correctness remains here. [paraphrase]

- The owner wants to return to a template that looks as expected and tick it off. Interventions and first-review acceptance matter alongside speed and model cost. [paraphrase]
- Cheap execution is useful when Jev and frontier escalation make it dependable. The goal does not require forcing complex work onto the cheapest model. [paraphrase]
- The host retains orchestration authority and delegates complex design explicitly. Expensive reasoning is scoped to the selected assignment, then control returns to the cheap conductor. Jev-first decisions reduce generative reasoning work; reducing Jev call count is not the objective. [paraphrase]

### A tuning run ends with a gap list (owner, 2026-09-22)

- Every tuning run, invoked by the owner or by the add-species agent, ends with three things: the best tree with its overlay, seed and stills; the trial and handoff record; and a gap list. A run without the gap list has not finished. Owner: "so this assessment tuning loop should end with the end result + gap list and the invoker would decide if the specs should be made yea?" then, to the contract below, "ok that works". [user]
- A priority that kept routing to tuning and then stalled converts to a gap candidate instead of vanishing into the stall. Each entry carries the trait in the owner's words, the rounds that tried it with their overlays, the reviewer's words per attempt, and the stills that show it. The loop assembles those four from its own record. [host design]
- The invoker runs fn-95's gap check on each entry: reachable with dials not yet tried goes back to tuning; covered by an open spec becomes a dependency and the run parks on it; new escalates. The cause in generator terms and the shape of the spec are system design under the dispatch rule, so a driver packages a new gap and stops, the host conducting the run writes the candidate spec, and the owner holds the word on minting until delegated. [user and host design]
- Why: fn-68's close-out on 2026-09-22 stopped at the reviewer's symptoms and one handoff, and the beech's real gaps (leaders that lose girth at every fork, fn-103) were sitting in priorities that had routed to tuning for 44 rounds. Owner: "otherwise the next spec that runs the add species pipeline won't have specs to make to close gaps". [user]

## Strategy Alignment

- Supports the catalogue track by making each species a repeatable run and each capability gap shared generator work. [strategy:The catalogue]
- Supports botanical fidelity by requiring matched visual evidence before the owner reviews a candidate. [strategy:Growth and botanical fidelity]
