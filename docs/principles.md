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

1. **Structure.** Every tree is built by `telperion_core::pipeline`. Its stages are private to it (fn-158, `docs/pipeline.md`), so a second chain outside the pipeline does not compile; the compile-fail tests `crates/telperion-render/tests/boundary.rs` and `crates/telperion-wasm/tests/boundary.rs` hold that.
2. **One test, every preset, every artifact.** `every_shipped_preset_builds_every_artifact_through_the_pipeline` (telperion-core) builds each catalogue preset's skeleton, surface, leaves, field and structure through the pipeline. The package entries have their own tests: `every_shipped_preset_builds_through_the_main_entry` (telperion-wasm) and `every_shipped_preset_answers_as_the_main_pipeline_does` (telperion-field, from fn-150). A preset that a shipped entry cannot build fails the suite, as the date palm did through the slim entry on `39348def`.
3. **The size budget.** `scripts/artifact-budgets.json` holds two limits on every shipped Wasm module and script, and CI's package job runs `node scripts/artifact-budgets.mjs` on the package it built. The first is a ceiling the owner set from what the product's consumers need, with its reason; an artifact over it fails the job, and crossing a ceiling is an owner decision, never a routine raise. The second limits one PR's growth: the job measures the package of the newest green master run that is an ancestor of the PR's head, and fails an artifact that grew by more than the growth share (5 percent, stated with its reason in the same file) unless the PR's `## Decisions` section names the artifact's file name; the body is read when the job runs, so a declaration added later needs the job re-run. When no such master package exists, the job prints a warning that growth was not checked and holds the ceilings alone; any other lookup failure fails the job. The baseline sizes in the file are recorded measurements, not limits. fn-150's rejected first slim build (355,100 to 526,965 bytes, +48 percent) fails both limits; fn-170's fork rule (+1.1 percent) passes with no edit to the file.

Timing and memory are measured on named hardware by the spec that changes them, never per push.

## Question, delete, then optimise

Every design takes these steps in order, and a later step never starts on something an earlier one would remove (owner, 2026-10-02, after Musk's five-step algorithm).

1. **Every requirement has a name.** A constraint in a spec or in the generator states who set it and why: the owner with a date, a measurement, a cited source, or a principle above. A spec's `[inferred]` requirement is a hypothesis to question before it is built, not a settled one. A constraint in code that no one can name is a candidate for deletion, not a fact to work around.
2. **Delete before simplifying.** Before a design makes something cheaper, simpler or more general, it answers, with a measurement, whether the thing can be removed instead, including whether the real tree has a botanical cause that makes the constraint unnecessary. Some deletions come back, as fn-183's curtain floor did for the clearance dial; a design that never restores anything has not deleted enough.
3. **Then simplify and optimise.** Count who asks before making the answer cheap: a profile ranks functions by time, not by which code calls them or what for, so a speed spec counts a hot function's calls by caller and purpose before designing a faster one. A spec is named for its purpose, not a mechanism, and its boundaries do not hand the deletion question to a neighbouring spec.
4. **Then shorten the loop, then automate.** Faster iteration and automation come last, on what survived the first three.

The reason: fn-173 spent two sessions and a design review making the crown's radius cheaper to compute. A count by caller on 2026-10-02 showed that 86 to 88 percent of the oak's and beech's crown queries, and the birch's curtain search on top, came from enforcing the crown as a wall on twig growth (`.flow/evidence/fn-183-the-twig-layer-grows-without-the-crown/R1-LADDER.md`). The wall had no owner and no reason; a real crown has none. fn-183 deleted it, and the trees read less regular and grow faster. The host then added an unnamed limit on twigs leaving the outline and picked a rung by it; the owner deleted that too.

## Sanctioned exceptions

Two sanctioned exceptions build outside the one-pipeline rule. Code visibility holds this list: the pipeline exposes `pipeline::build` and the executor interface, and nothing else may be called.

- **The growth path** (`specimen::view::SpecimenView::mesh`). A hidden feature, kept buildable and pinned (AGENTS.md, Mature trees are the product; PR #17).
- **The GPU executor** (`telperion_render::generation`). One algorithm with two executors: the GPU, and the CPU reference that defines correct (STRATEGY.md, Our approach; PR #50).

A trade-off names its principle and one of these in the PR's Decisions line. A claim of approval without one is none.

## History

fn-151 first built a policing layer: a boundary scanner, a pre-push hook and an advisory Jev reviewer, measured on 39 owner-labelled PRs. The owner kept what caught real breaches as structure and tests, and moved design review to fn-156. The measurements are in `.flow/evidence/fn-151-the-design-principles-are-checked/EVALUATION.md`, and the removed code is in git history at `132257f3` and `061ff37d`.
