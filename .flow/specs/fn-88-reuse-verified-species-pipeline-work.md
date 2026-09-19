# Reuse verified species pipeline work (retired)

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

## Goal & Context

**Disposition: retired without implementation on 2026-09-19.** The owner rejected duplicate-Jev-call savings as a premature optimization and approved concentrating efficiency work on model allocation in fn-89. The original proposal below is retained as history, not active acceptance criteria. Its dependent specs no longer require it. [paraphrase]
<!-- scope: business -->

An additional species should benefit from the sources and judgments already gathered, and a resumed run should spend only on work whose inputs changed. This change reduces redundant discovery, semantic judgments and candidate evaluations while retaining the current evidence and acceptance requirements. [paraphrase]

## Architecture & Data Models
<!-- scope: technical -->

- Discovery examines relevant known evidence for the requested taxon, condition and field, and searches externally for unresolved coverage. Known does not mean sufficient, and an old admission does not transfer blindly to another taxon or context. [inferred]
- Successful semantic judgments are reusable only for identical relevant evidence, question definitions and resolved model identity. Cached failures and ambiguous model-version aliases do not silently stand in for fresh successful evaluations. [inferred]
- Stage keys cover their actual inputs and relevant versions. Deterministic extraction can be reused after a threshold change; a decision derived under the changed threshold must be recomputed from suitable recorded model outputs or re-evaluated when the question meaning changed. [inferred]
- Independent questions over bounded common context are batched. Evidence retrieval or questions that depend on previous answers remain sequential. [inferred]
- Candidate evaluations share fn-68's cache contract, including the full effective parameter set, seed, generator/renderer identity, reference/camera inputs and measurement definition. Identical override text alone is insufficient. [inferred]

## API Contracts
<!-- scope: technical -->

- Existing research, judgment and candidate-evaluation operations retain their consumed outputs and provenance; they report reused work separately from newly performed work. Cache misses follow normal evaluation. [inferred]

## Edge Cases & Constraints
<!-- scope: technical -->

- Cache reuse preserves provenance and records the original evaluation identity. An invalid or absent cache entry falls through to the normal evaluation and never becomes a pass. [inferred]
- Reference, seed, source, question or implementation changes invalidate affected results. An unrelated engineering edit does not force a refetch of unchanged admitted sources. [inferred]
- Count total cost including failed attempts and retries; distinguish cache hits from actual service requests. [inferred]

## Acceptance Criteria (withdrawn)
<!-- scope: both -->

- **R1:** When verified local evidence covers a requested field in the required context, discovery performs no external search for that field; missing coverage causes targeted search. Errors: wrong taxon, condition, unavailable source or insufficient evidence cannot satisfy coverage. [inferred]
- **R2:** Repeating a successful identical judgment or candidate evaluation reuses the recorded result without another service request or render, while a changed relevant input forces reevaluation. Errors: missing, corrupt, stale or failed entries are never trusted; model identity limitations are explicit. [inferred]
- **R3:** Narrow stage dependencies preserve unaffected work after a manifest or version edit, and bounded batches produce the same consumed judgments as the corresponding independent requests on pinned test responses. Errors: an invalid batch answer leaves its affected items unresolved; it cannot advance dependent stages. [inferred]
- **R4:** A fixed comparison run reports cold-run, unchanged-resume and targeted-change costs and elapsed times before and after the change, with no extra external work on unchanged resume and no unrelated research work on an engineering-only edit. Errors: skipped or unavailable observations are reported, and no speed claim is made without measured evidence. [inferred]

## Boundaries
<!-- scope: business -->

- No lowering the source-quality, numeric or visual requirements to save cost. [inferred]
- No new generator behavior, species table or autonomous decision authority. [inferred]
- No separate citation engine. fn-83 consumes the shared judgment and citation machinery. [inferred]

## Decision Context

### Motivation

- Superseded direction: choose Jev for bounded decisions, a cheap conductor for routine work, a high-reasoning designer at medium/high effort for complex design, then independently choose a cheap or high-reasoning low-effort implementer. No cache project replaces this withdrawn proposal. [paraphrase]

- The owner asked to simplify the pipeline and make it as efficient to run as possible, using fast, cheap judgments to support cheap drivers. [paraphrase]
- Reuse is independently measurable before autonomous orchestration lands. It should not wait for the first complex generator gap to be implemented. [inferred]

## Strategy Alignment

- Each added species contributes reusable evidence and parameter anchors to the catalogue. [strategy:The catalogue]

