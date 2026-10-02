# Limbs are thick and their twigs fine

## Conversation Evidence

> owner (2026-09-27, on fn-177's plane candidate beside the Schützenmattpark photograph): "The plane i sent has very thick multiple leaders and thick branches from that. Then there's actually very few branches off that secondary level that lead to many twigs with leaves."
> owner (2026-10-02, on fn-182's hierarchy stills): "the plane is slightly closer but not there at all."
> owner (2026-10-02): agreed to delete fn-182's twig shell and take Candidate B's direction to fn-62 ("y").

## Goal & Context
<!-- scope: business -->
<!-- Goal & Context: 25% [user], 55% [checked], 20% [inferred] -->

In the owner's photographs a few thick limbs divide into progressively thinner, shorter branches, and the fine twigs are much finer than the wood they leave. fn-182's bounded comparison on the beech (`.flow/evidence/fn-182-branching-hierarchy-few-strong-limbs/R5-RESULT.md`, "Bounded comparison") found that no row it tested moved how thick a branch is against its parent: at substantial divisions (daughter radius at least 0.3 of the parent's) a daughter stays about 0.78 of its parent's radius, across `lateralLengthRatio`, `raggedReach`, `crookedness`, `pitchVariation`, the short-shoot rows and `limbRadius`. Only `codominance` 0.6 with `forkWays` 2 thinned it (0.69), at the cost of two of the four leaders, and the effect disappeared once combined with the other rows. [checked]

This spec makes the thickness of a branch fall from limb to twig the way the references show, starting from the rows the comparison did not test. [inferred]

## Architecture & Data Models
<!-- scope: technical -->

**Step 1 and 2 first (`docs/principles.md`, "Question, delete, then optimise").** The comparison did not test the rows that set thickness: `girthHold` and `girthFall` (fn-177, `pipeline/radius/hold.rs`), `ratioPower` (twig radii by `lengthRatio^ratioPower`), and the scaffold's pipe allocation, `forkExponent`, `lateralShare` and `forkBalance` (`pipeline/radius.rs`). Astra's visual review (`ASTRA-VISUAL-REVIEW.md`) names them as the thickness controls. R1 measures them before anything is designed. [checked: rows exist; their effect on the ratio is unknown]

**Known limits that may be part of it, verified in code by Astra (2026-10-02).** [checked]
- Forks are placed by absolute tree height (`scaffold/fork.rs:87`), so a limb cannot fork along its own course; repeated subdivision of a high or sideways limb is not expressible.
- Scaffold laterals below order 0 take the whole parent axis's length times `lateralLengthRatio`, wherever they depart (`scaffold.rs:317`), with no departure-position term.
- The pipe solve sums only nodes before the crossover (`radius.rs:169`); the twig layer does not feed scaffold girth.

**Unknown until measured (R1).** Whether the existing girth rows can bring the beech's daughter-to-parent ratio at substantial divisions from about 0.78 down toward the photographs' reading (fn-182's R1: leaders 0.4 to 0.6 of the trunk, order-2 branches 0.3 to 0.5 of their leader), and at what cost to the rest of the tree. [unknown]

## Edge Cases & Constraints

- Any new row is one continuous law, dormant at today's look, refused by name off its rails. [principles]
- Identity is not required; the owner's verdict on bare and leafy stills decides. [AGENTS.md]
- No full-forest capture. [AGENTS.md]

## Acceptance Criteria

- **R1:** On the beech at seed 1, starting from fn-182's Candidate B, each girth row walked one at a time across its window, measured on the same traced branch systems as fn-182's bounded comparison (daughter-to-parent thickness at substantial divisions, length against the parent's remaining extent, length between divisions, tortuosity, angle spread), with nodes, leaves and skeleton time, and the workspace suite run on the exploratory build. [inferred]
- **R2:** The host's decision is recorded before production code: either existing rows reach the references (values handed to fn-62), or the missing capability is named with its measured shortfall, including whether the two verified limits above take part. [inferred]
- **R3:** If code is needed: new rows dormant at today's look (every shipped preset byte-identical at seeds 1 and 7), declared once, blended, on the dials, refused by name off their rails, with a test of the law on a synthetic family. [inferred]
- **R4:** The owner's verdict on the beech, bare and whole, beside its reference. [user]
- **R5:** `cargo test --profile ci --workspace --no-fail-fast` and `npm test` green; costs reported. [AGENTS.md]

## Boundaries

- The beech's values are fn-62's; this spec hands it a direction or a capability. Not a London plane preset. [inferred]

## Decision Context

- Split from fn-182, which closed without merging its twig shell: `sheddingThreshold` 0.25 clears the inner crown the same way at this scale (owner, 2026-10-02). [user]

## Strategy Alignment

- Serves "Mature trees are the product": the owner judges trees by their structure first. [strategy:Our approach]
