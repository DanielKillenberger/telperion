# The Wasm binding names the allocation behind its peak

## Conversation Evidence

> assistant: "Both fn-102 and fn-134 lost 25–60 minutes hunting browser memory regressions by trial and error. Nothing reports which allocation set the peak, so each guess cost a full rebuild plus 4 minutes of measuring. I'd propose a small evidence-tooling spec: a browser-build allocator that names the allocation behind the peak."
> user: "yes to both"

## Goal & Context

<!-- Goal & Context: 20% [user], 80% [inferred] from two friction reports -->

fn-102 and fn-134 both required Wasm peak linear memory to stay at or below base, and both met a rise with byte-identical output that came from allocation layout, not extra data. fn-102's worker spent about 60 minutes and four hand-built probe binaries (its FRICTION.md, third entry); fn-134's spent about 25 minutes and four rebuild-and-measure rounds of the 240-build binding pass (its FRICTION.md). In both cases nothing said which allocation set the high-water mark. [inferred]

## Architecture & Data Models

- An allocator wrapper for the Wasm binding, compiled only under an evidence-tooling feature, never in the default or shipped build. When `memory.grow` raises the high-water mark it records the pipeline stage that was running, the request size, and the live bytes per stage at that moment. [inferred]
- Stage markers come from the pipeline's existing stage boundaries (`pipeline.rs`), so no allocation call site changes. [inferred]
- The shipped binding's exports and metadata are unchanged; the feature build reports through the evidence tools under `.flow/evidence/`. [inferred]

## Acceptance Criteria

- **R1:** A feature build of the binding reports, for one request, the stage and allocation size that set the peak and the live bytes per stage at that moment. [inferred]
- **R2:** Reproducing fn-134's first-candidate regression (a scratch buffer outliving its stage) on a scratch branch, one run of the tool names the stage whose allocation set the higher peak. [inferred]
- **R3:** The default and shipped Wasm builds have the same size and exports with and without this spec. [inferred]
- **R4:** `docs/` gains one short page on running it, linked from `docs/evidence-retention.md`. [inferred]

## Boundaries

- Evidence tooling only; generation, rendering, presets and the shipped bindings behave as before. [CLAUDE.md]
- No native allocator change. [inferred]

## Decision Context

- Proposed from fn-102's and fn-134's friction reports and accepted by the owner on 2026-09-24. [user]
