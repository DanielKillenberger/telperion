## Conversation Evidence

> owner (2026-09-25): "can we actually have a test suite that we run through jev to check if the diff adheres to our guiding principles." / "can we run jev on specs before we start work?" / "This needs to be tight."
> fn-151's evaluation (`.flow/evidence/fn-151-the-design-principles-are-checked/EVALUATION.md`, on PR #120's branch): diff-level Jev review caught 0 of 11 holdout breaches [0%, 26%] and 2 of 3 calibration breaches. It never named a redundant stop for the runner's approval layers, although candidates reached every span. On the labelled duplicate-path spec it saw the duplicate (0.84) but judged an allowance covered it (0.64). Twin adapters fell past the 12-candidate cap (76 dropped). It cost 135 calls and about 437k input tokens.
> host (2026-09-25): keep fn-151's deterministic guards, cut the Jev reviewer from #120, and capture how to use Jev as a follow-up / owner: "ok can you capture the follow up spec for how to use jev and do the cut then"

## Goal & Context
<!-- scope: business -->

fn-151's code guards catch the concrete breaches: copied paths, missing entry support, size growth. The judgement-shaped breaches are still unguarded: approval layers that catch nothing, duplicate implementations of one responsibility, and a setting whose first step switches the kind of tree. Diff-line review by Jev failed on them. The breach is a property of a design against what already exists, not of scattered added lines. This spec finds a way for Jev to judge a design where that relation is visible, and ships it only if it meets a measured bar. [paraphrase]

## Architecture & Data Models
<!-- scope: technical -->

- **What exists, after fn-151.** The deterministic guards (production boundary, entry coverage, artifact budgets), the pre-push hook, the scoped exception registry, `docs/principles.md` with its question ids, and fn-151's labelled PR set and evaluation as the baseline. [paraphrase]
- **The approach to test first.** Judge a spec's proposed design against a code-built inventory of what already exists: the modules and their responsibilities, the stop and approval sites, the build stages, the reviewer paths. Jev picks, from inventory items that code offers, the existing thing a proposal duplicates or the existing check a new stop repeats, or "none". Jev selects; code offers every candidate (AGENTS.md, TypeSafe). [inferred]
- **Exceptions are code's.** Whether the exception registry covers a finding is a deterministic match, never a Jev question. fn-151's `covered` answer is what hid the duplicate-path spec. [paraphrase]
- **Placement.** A spec is judged before it is marked ready, where a design is whole. A push-time check follows only if the spec-time bar is met, and only for what the spec check cannot see. [inferred]
- **Unknown.** Whether an inventory-selection question separates breaches from clean designs better than fn-151's diff candidates did; this spec measures it before building any integration. [unknown]

## Acceptance Criteria
<!-- scope: both -->

- **R1:** An offline experiment runs the inventory-selection approach on fn-151's labelled set, with specs where they exist and each PR's design summary where they do not. It reports recall, precision and false flags with 95% intervals, on calibration and holdout groups kept apart by lineage. Errors: no integration is built if the holdout recall is below 0.8 or the precision below 0.9; the spec then closes with the evaluation as its result. [inferred]
- **R2:** If R1 meets the bar, `jev principles --spec` runs the approach before `flowctl spec ready`, advisory only, within 10 s and at most 8,000 input tokens per spec, and reports "incomplete" rather than "clean" on a timeout. Errors: the exception match is code's; a finding an exception covers is not shown. [inferred]
- **R3:** The run reports its Jev calls and tokens; the experiment stays under 150 calls. [inferred]

## Boundaries
<!-- scope: business -->

- Not the deterministic guards (fn-151). Not a blocking check: blocking stays with code guards unless a later spec graduates a principle on measured precision.

## Decision Context
<!-- scope: both — conditionally substructured -->

fn-151's diff-level reviewer was removed from #120 after its evaluation. Scattered added lines hide the relation that makes a design a breach: what it duplicates, or which check a new stop repeats. Its "is this allowed" question was also a judgement code can make exactly. This spec tests the relation directly, at the point where the whole design is visible, and builds nothing further unless the measurement clears the bar. [paraphrase]

## Strategy Alignment

- Serves "Our approach": lean code, measured before it ships. [strategy:Our approach]
