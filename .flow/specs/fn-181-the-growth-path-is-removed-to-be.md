# The growth path is removed, to be rewritten later

## Conversation Evidence

> user (2026-09-27, on fn-173 making the hidden growth path 70× slower on lobed trees): "Why is it 70x slower now? I mean we can scrap it and we'll have to rewrite it at a later date. The implementation was never good enough anyway. We'll need some different approach."
> user (2026-09-27, selected): "Capture the removal"

## Goal & Context
<!-- scope: business -->
<!-- Goal & Context: 60% [paraphrase], 40% [inferred] -->

The growth path grows a specimen year by year and shows it at an age, behind the harness's `?growth=1`. Since 2026-09-18 it has been a hidden feature: never a default, never a gate, kept buildable and pinned (AGENTS.md, "Mature trees are the product"). Its design has not held up. On 2026-09-17 the birch at seed 1 was 73,337 nodes on the direct build and 590,410 on the growth path, and fn-173's prepared crown shell, a 19 to 25% win on the mature build, made age-12 growth 70× slower on the birch, because every yearly slice builds new shells for a young tree that asks few questions of them. [paraphrase]

The owner's decision: remove it now and rewrite growth later with a different approach, rather than keep paying to maintain and pin code that will be replaced. [paraphrase]

## Architecture & Data Models
<!-- scope: technical -->
<!-- Architecture: 30% [paraphrase], 70% [inferred] -->

- **The line to draw, checked 2026-09-27.** The direct build is `Specimen::grow` (`branching::generate` calls it), so `Specimen` and its grower stay. What goes is the growing-over-time layer built on it: the age-sliced timeline, keyframes, the chronicle, change history, retention and storage, the snapshot reads of a specimen at an age, `SpecimenStore` / `SpecimenView` and the executor's `present` step that serves them, the render crate's web growth view, the Wasm growth bindings, the harness's `?growth=1` and grower view, and the growth half of the growth-profile example. The exact file boundary is settled by reading the code before deleting (Parked unknowns). [paraphrase]
- **Shared code stays only if the direct build reads it.** A module the mature build never reaches after the removal is deleted, not kept for the rewrite; the rewrite starts from the lessons, not the code. [inferred]
- **Lessons are kept.** A short note under `docs/` records why the design did not hold: per-slice rebuilds of state a young tree barely uses, the node gap to the direct build, and the costs measured by fn-67 and fn-173, so a rewrite starts from them. [inferred]

## API Contracts
<!-- scope: technical -->

- **Removed public surface:** the growth-path types and functions in `telperion-core`, the Wasm growth exports, and the harness `?growth=1` parameter. A package consumer that named them gets a compile error, and the package's changelog names the removal. [inferred]
- **Unchanged:** `pipeline::build`, `mesh::build`, the executor's other steps, and every artifact the direct build returns. [inferred]

## Edge Cases & Constraints
<!-- scope: technical -->

- **The mature tree is byte-identical.** Removing the growth path changes no preset's direct-build output. [inferred]
- **Specs that assumed the growth path** are re-scoped or closed, never left pointing at deleted code: fn-67 (growth parity), fn-124 (growth speed), fn-28 and fn-30 (growth on the page, calibrated growth), fn-11's leftovers, and any open spec that names the specimen at an age. [inferred]
- **The size budget** for shipped artifacts should shrink; it must not grow. [inferred]

## Acceptance Criteria
<!-- scope: both -->

- **R1:** No growth-over-time code remains: the age timeline, keyframes, chronicle, history, retention, storage, the specimen-at-an-age reads, `SpecimenStore` / `SpecimenView`, the executor's `present`, the web growth view, the Wasm growth exports and the harness `?growth=1` are gone, and a search of executable code, public exports and current usage documentation (crates, harness, src, the Wasm bindings, `docs/` apart from the lessons note) finds none of their names; historical specs, the lessons note, the changelog and AGENTS.md's dated removal line may still name them. Errors: no error surface beyond the compiler. [paraphrase]
- **R2:** Every catalogue and in-work preset's direct build is byte-identical to master's at seeds 1 and 7 (mesh and species metrics), and the full `ci` gate passes. Errors: a byte change stops the removal as a finding, since the product must not move. [inferred]
- **R3:** The harness loads and draws every preset with no growth control and no dead code path; `?growth=1` is ignored or reports that growth was removed, never a blank view. Errors: no error surface beyond R2. [inferred]
- **R4:** AGENTS.md's "Mature trees are the product" paragraph states that the growth path was removed on this date and why, with the rule that a future growth feature is its own spec and never a gate on species work. [paraphrase]
- **R5:** The open specs that depend on the growth path are each closed with a reason or re-scoped, listed in the PR. Errors: no error surface beyond R1. [inferred]
- **R6:** The lessons note exists under `docs/` and each shipped artifact's size is recorded before and after. Errors: no error surface beyond R2. [inferred]

## Boundaries
<!-- scope: business -->

- No new growth implementation here; the rewrite is a later spec. [paraphrase]
- `Specimen` and the direct build stay. [paraphrase]
- No change to any tree the harness, stills, species QA or owner verdicts judge. [inferred]

## Decision Context
<!-- scope: both -->

Removal over repair because the owner judged the design inadequate, and fn-173 showed its per-slice structure turns an optimisation for the product into a large regression for itself. Keeping it would mean fixing it (sharing lobe tables across slices) only to replace it. [paraphrase]

This overturns AGENTS.md's 2026-09-18 rule that the growth path is "kept buildable and pinned" (owner, 2026-09-27). [paraphrase]

## Strategy Alignment

- **Growth and botanical fidelity:** STRATEGY.md names growth over a lifecycle as a direction; this removes a failed implementation, not the direction, and the lessons note carries it to the rewrite. [strategy:Growth and botanical fidelity]

## Strategy Conflicts

- STRATEGY.md's approach lists "growth, surroundings and damage shaping its lifecycle" and "age and growth rate are numeric traits". Removing the growth path leaves no implementation of growth over time until the rewrite; the direction stands. Flagged for the owner's next strategy pass. [inferred]

## Parked unknowns

- The exact file boundary between `Specimen::grow`'s mature build and the growing-over-time layer; resolved by reading `pipeline/branching/specimen/` and its callers before deleting.
- Whether the lifecycle traits the growth path reads, `Family.age` and `Family.growth` (`GrowthTraits`), are removed or stay dormant for the rewrite; resolved by whether the direct build reads them. They are distinct from `SkeletonParams.growth` (`GrowthOverrides`), which the mature build uses and which stays.

## Requirement coverage

| R-ID | Task |
|------|------|
| R1 | fn-N.M (TBD - populate via /flow-next:plan) |
| R2 | fn-N.M (TBD - populate via /flow-next:plan) |
| R3 | fn-N.M (TBD - populate via /flow-next:plan) |
| R4 | fn-N.M (TBD - populate via /flow-next:plan) |
| R5 | fn-N.M (TBD - populate via /flow-next:plan) |
| R6 | fn-N.M (TBD - populate via /flow-next:plan) |
