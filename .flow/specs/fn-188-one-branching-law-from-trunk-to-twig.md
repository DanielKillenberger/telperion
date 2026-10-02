# One branching law from trunk to twig

## Conversation Evidence

> user (2026-10-02): "let's take a step back and look at the problem from the start. What are we trying to do? Why haven't we been able to get there? Is there something that we fundamentally have to change? something to delete? an assumption to question?"
> user (2026-10-02, on the host's answer: question the scaffold/twig split, fix the judging lens first, probe one branching law before any production code): "yes that sounds good"

## Goal & Context
<!-- scope: business -->
<!-- Goal & Context: 20% [user], 50% [checked], 30% [inferred] -->

The goal is mature trees whose structure reads like the real species, the beech first: a few thick limbs dividing progressively into finer branches, ending in a dense mesh of fine twigs at the periphery and along the limbs, on a clear trunk, generated fast enough for runtime use. [user, paraphrase]

Spec after spec has fixed one visible symptom and exposed the next: fn-61, fn-170, fn-177, fn-179, fn-180, fn-183, fn-182 (shell, girth, horns, twig gate), after fn-34's 23 rounds. Most defects found on 2026-10-02 sit on one seam: the tree is built by two systems joined at a crossover, the scaffold (space colonization and scaffold rules) and the twig layer with its own laws. [checked]

| Defect | Rule at the seam |
|---|---|
| Horns: thick poles past the crown | a scaffold tip hands over to one twig-layer continuation at full radius, length 38.97·r^(2/3) (`local/seed.rs:158`, `twigs.rs:394`; fn-182 `R8-HORNS.md`) |
| Bare thick limbs | twig-layer laterals only on wood thinner than `limbRadius` of the stem (`local/seed.rs:133`, `local/advance.rs:51`) |
| Thickness that does not fall | the pipe solve sums only nodes before the crossover (`radius.rs:169`); patched by `girthHold`, `girthFall`, `lateralShare` |
| The crown wall | the twig layer clipped against the outline (fn-183, removed) |
| Clean-ups after the fact | shedding as structure, the apical twig clearing, the dropped twig shell |

In a real tree one branching process runs from trunk to twig; strong limbs are strong because they won more resource, and girth comes from everything a branch carries. AGENTS.md states the current split as the product's design: "space colonization for the crown, botanical rules below the crossover". This spec questions that assumption with a probe, under `docs/principles.md`, "Question, delete, then optimise". [checked, inferred]

## Architecture & Data Models
<!-- scope: technical -->

**Step 1: the judging lens.** The 960x720 hero stills draw 86% of the beech's twig segments under one pixel wide (fn-182 `R8-HORNS.md`); a 10 m close-up shows twigs the full view drops. The reference photographs show those twigs as a visible haze. Until a judging render shows sub-pixel twig mass as tone, a comparison with the photographs cannot separate a growth defect from a drawing one. How the renderer handles segments under a pixel is unknown. [checked: pixel counts; unknown: the renderer's handling]

**Step 2: a throwaway probe.** One beech grown by a single branching law over all orders, with no crossover: every axis bears laterals by one rule whose vigour, length and thickness are continuous functions of what the axis carries and where it is; thickness from the pipe model over the whole tree. Published self-organizing tree models (bud competition for space and light with vigour allocated down the hierarchy) are the reference point; the probe takes the smallest form that can show whether the hierarchy emerges. It lives outside the pipeline (a scratch example or crate), is never merged, and is measured on the same traced branch systems as fn-182 (thickness and length against the parent, length between divisions, tortuosity, angle spread, twig wood per metre of limb), with nodes and build time against today's beech. [inferred]

**Unknown until measured.** Whether one law produces few strong limbs and a fine periphery on the beech; whether it fits the runtime budget (today's beech skeleton is about 75 ms); what of today's pipeline it would replace. [unknown]

## Edge Cases & Constraints

- No production code, no preset change, nothing merged from the probe. [user]
- No full-forest capture; at most four stills per comparison, judged by the host before the owner sees them. [AGENTS.md]

## Acceptance Criteria

- **R1:** A judging render for the beech in which sub-pixel twig mass reads as tone, or a framing that resolves it, shown beside the reference with today's beech; the report states how the renderer draws sub-pixel segments today. [inferred]
- **R2:** The probe beech, its measurements on the traced systems against today's beech and the photograph's reading (fn-182 `R1-MEASUREMENT.md`), its build time, and bare and whole stills through the R1 lens. [inferred]
- **R3:** A written comparison: what the single law produced, what it did not, what of today's pipeline it would delete (crossover hand-off, twig-layer laws, girth patches, the twig gate, structural clean-ups), its cost, and the risks. [inferred]
- **R4:** The owner's decision: rebuild growth around one law (a strategy change, then specced on its own), or return to fn-182's two targeted fixes. [user]

## Boundaries

- fn-182's code changes (the continuation length, twig bearing on thick wood) are paused until R4. [user]
- The rendering of sub-pixel twigs in the product is its own spec if R1 shows it is needed; R1 only builds a judging lens. [inferred]

## Decision Context

- **Why now (owner, 2026-10-02).** The step-back asked what to change, delete or question; the scaffold/twig split is the assumption most of the day's defects trace to. [user]
- Evidence: fn-182's `R7-GIRTH.md`, `R8-HORNS.md`, `ASTRA-VISUAL-REVIEW.md`, `ASTRA-THICKNESS-REVIEW.md`. [checked]

## Strategy Alignment

- Questions STRATEGY.md's "Our approach" (space colonization for the crown, botanical rules below the crossover); any change to it is the owner's, at R4. [strategy:Our approach]
