# The design principles

STRATEGY.md's "Our approach" is enforced by structure, not policing (owner, 2026-09-25). No hook and no scanner police a push. The code is shaped so a breach does not compile or does not pass the ordinary suite, and CI holds the shipped artifacts to their size. Jev review of specs and designs against what already exists is planned in fn-156.

## The principles

| Principle | Clause (STRATEGY.md, Our approach) |
|---|---|
| One pipeline | Generation is one pipeline that every tree passes through; each family feature is a term inside a stage, never a route around it. |
| Never a fallback path | An input the pipeline cannot represent is an explicit error, never a fallback path. |
| One continuous tree space | A small change in any parameter makes a small change in the tree; a parameter may lie dormant, and none is a switch between ways of building. |
| Only what the consumer reads | Generate only the detail the consuming engine needs. |
| Measured cost and look | Judge each advance through measured runtime costs and visual evidence. |
| Stops that earn their place | A step that stops work must catch a defect the steps around it cannot. |

## How they are held

1. **Structure.** Every tree is built by `telperion_core::pipeline`. fn-152 makes the build stages private to it, so a second chain outside the pipeline does not compile. Until then, a new caller of a stage outside the pipeline is a review finding.
2. **One test, every preset, every artifact.** `every_shipped_preset_builds_every_artifact_through_the_pipeline` (telperion-core) builds each catalogue preset's skeleton, surface, leaves, field and structure through the pipeline. The package entries have their own tests: `every_shipped_preset_builds_through_the_main_entry` (telperion-wasm) and `every_shipped_preset_answers_as_the_main_pipeline_does` (telperion-field, from fn-150). A preset that a shipped entry cannot build fails the suite, as the date palm did through the slim entry on `39348def`.
3. **The size budget.** `scripts/artifact-budgets.json` holds every shipped Wasm module's and script's budget, with the measurement behind it. CI's package job runs `node scripts/artifact-budgets.mjs` on the package it built, and an artifact over its budget fails the job. A budget rises only when the same PR edits that file with the new measurement and a Decisions line. The rejected first slim build of fn-150 (526,965 bytes, +48%) fails; 0.1.4 passes.

Timing and memory are measured on named hardware by the spec that changes them, never per push.

## Sanctioned exceptions

Two sanctioned exceptions build outside the one-pipeline rule. fn-152 turns this list into code visibility: what the pipeline exposes is what may be called.

- **The growth path** (`specimen::view::SpecimenView::mesh`). A hidden feature, kept buildable and pinned (AGENTS.md, Mature trees are the product; PR #17).
- **The GPU executor** (`telperion_render::generation`). One algorithm with two executors: the GPU, and the CPU reference that defines correct (STRATEGY.md, Our approach; PR #50).

A trade-off names its principle and one of these in the PR's Decisions line. A claim of approval without one is none.

## History

fn-151 first built a policing layer: a boundary scanner, a pre-push hook and an advisory Jev reviewer, measured on 39 owner-labelled PRs. The owner kept what caught real breaches as structure and tests, and moved design review to fn-156. The measurements are in `.flow/evidence/fn-151-the-design-principles-are-checked/EVALUATION.md`, and the removed code is in git history at `132257f3` and `061ff37d`.
