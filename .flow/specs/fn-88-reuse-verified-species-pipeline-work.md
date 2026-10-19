# Reuse verified species pipeline work (retired)

## Goal
Proposed so an added species and a resumed run spend only on work whose inputs changed: discovery reuses known evidence for the taxon and context, identical successful Jev judgments and candidate evaluations are reused, stage keys cover their real inputs, and independent questions are batched. Retired without implementation on 2026-09-19.

## Decisions
- > user: "ideally i'd like the whole loop to be basically autonomous with a lot of judgement being done quickly and implementation to close gaps being done and I can just come back to review a new species template that looks awesome and what I would expect and i can just tick it off."
- > user: "efficiency is mostly about choosing the right tool for the job. judgements/decisions/routing should be done by jev wherever possible to keep llm token costs low. Then the agent should be a cheap effective model. But when a gap turn up and something is judged complex (by jev i imagine) then we escalate to a high reasoning model with medium/high effort to design the spec. Then we judge who can implement the design. If the design has done most of the complex work ahead of time we can probably use a cheap model. If the implementation is complex then we should probably use a high reasoning model on low effort."
- > user: "That's mostly what i meant with efficiency. Makes sense? do we need to adapt the specs? can you review them over? i really don't think we need to save on jev calls with fn-88"
- > user: "ok" (accepted retiring fn-88 and amending fn-89, fn-68 and fn-80).
- Owner, 2026-09-19: duplicate-Jev-call savings are a premature optimisation; efficiency work goes to model allocation in fn-89. Dependent specs no longer require this one.

## Result
Retired. The withdrawn criteria were: R1 no external search where verified local evidence covers a field; R2 reuse identical successful judgments and evaluations, re-evaluate on changed inputs; R3 narrow stage dependencies and equivalent bounded batches; R4 measured cold, resume and targeted-change costs.

## Open
None.
