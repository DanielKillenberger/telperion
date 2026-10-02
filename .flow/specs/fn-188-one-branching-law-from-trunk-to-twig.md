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

**Revised after Astra's challenge (2026-10-02, `ASTRA-REVIEW.md` in this spec's evidence).** [checked]
- **The beech uses no attractor competition:** its `attractorWeight` is 0 (`european-beech.values:31`), so its structure is rule-generated, not space-colonized. The scaffold's own rules (one lateral per deeper station, lengths from the whole parent axis, forks at absolute heights; `scaffold.rs:280`, `scaffold.rs:318`, `scaffold/fork.rs:101`) would survive a shared branching function unless changed deliberately.
- **The seam explains part of it, not all:** the horns are mostly the hand-off (`local/seed.rs:158`), the bare thick wood is an explicit eligibility rule (`local/seed.rs:132`), and thickness has two causes, the pipe solve excluding twig-layer wood (`radius.rs:166`) and near-equal downstream demand giving near-equal daughters under any pipe solver.
- **The 86% under a pixel** was P1's twig segments along thick limbs, a projected width, not proven lost coverage. Wood draws its full index range with no camera-based rejection; captures use 4x MSAA where supported (`wood.rs:277`, `wood.wgsl:25`, `pass.rs:14`, `headless.rs:54`); the capture's actual sample count is unchecked.

**Steps, smallest first.** [inferred]
1. **The judging lens.** A fixed-camera full-tree beech render at higher resolution downsampled in linear light, beside native resolution with the same geometry and lighting, plus one resolved branch crop; the same framing for every candidate. Only demonstrated coverage loss justifies coverage-preserving thin-wood drawing; opaque minimum-width inflation would exaggerate wood.
2. **Small ablations inside today's pipeline.** (a) On frozen topology and positions, solve radii from fine-branch demand over the whole tree, root diameter fixed, without feeding radii back into growth. (b) Delete the unconditional terminal extension (R8's shortening, taken to removal). (c) If thick scaffold poles remain, axis-relative subdivision. Each measured on the traced branch systems through the step-1 lens.
3. **Only if a topology defect remains:** a competitive-growth probe in the Pałubicki et al. 2009 extended Borchert-Honda family (terminal and axillary buds, one coarse light field, accumulated exposure, resource allocation with apical preference, resource-dependent extension, a whole-tree pipe pass), outside the pipeline, compared with neutral allocation. Rejected if the poles persist, if it needs the discarded rules back, or if acceptable detail exceeds the agreed cost.

**Runtime is measured, not assumed.** Same machine, profile, height, terminal detail and outputs; cold and repeated latency, median and p95, peak memory, cumulative nodes visited, generated against retained nodes, time by stage, budget retries included; against the shipped beech and fn-182's shortened candidate (22 to 26 ms). [inferred]

## Edge Cases & Constraints

- No production code, no preset change, nothing merged from the probe. [user]
- No full-forest capture; at most four stills per comparison, judged by the host before the owner sees them. [AGENTS.md]

## Acceptance Criteria

- **R1:** The judging lens above, with the capture's sample count stated and coverage loss shown or ruled out. [inferred]
- **R2:** Ablations (a), (b) and (c) as needed, measured on the traced systems through the R1 lens, with runtime as above. [inferred]
- **R3:** The probe of step 3, only if R2 leaves a topology defect. [inferred]
- **R4:** The owner decides in order: the visible structure that is sufficient, on resolved branch evidence; the latency and detail trade-off; whether R2 leaves a defect worth R3; and whether R3 justifies targeted integration, more investigation or a broader replacement. [user, Astra review]

## Boundaries

- fn-182's code changes (the continuation length, twig bearing on thick wood) are paused until R4. [user]
- The rendering of sub-pixel twigs in the product is its own spec if R1 shows it is needed; R1 only builds a judging lens. [inferred]

## Decision Context

- **Why now (owner, 2026-10-02).** The step-back asked what to change, delete or question; the scaffold/twig split is the assumption most of the day's defects trace to. [user]
- Evidence: fn-182's `R7-GIRTH.md`, `R8-HORNS.md`, `ASTRA-VISUAL-REVIEW.md`, `ASTRA-THICKNESS-REVIEW.md`. [checked]

## Strategy Alignment

- Questions STRATEGY.md's "Our approach" (space colonization for the crown, botanical rules below the crossover); any change to it is the owner's, at R4. [strategy:Our approach]

**Speed is a product requirement (owner, 2026-10-02):** "in the long term it needs to be super fast". Look decides first, but every step reports its cost, and a candidate that cannot show a credible path to being faster than today is rejected, whatever it looks like. [user]

## Direction after the probe and the tip trace (owner, 2026-10-02)

The owner: "i agree that the question about the fraction of the trunk is another arbitrary rule like the crown border checks. We should have a branching rule that makes them grow organically according to the params set." The twig gate (`limbRadius` × stem radius) and the tip shoot that bypassed it are the same arbitrary threshold (fn-189 `TRACE.md`: telperion at seed 7 has every scaffold node at or just above the gate, so its tip shoots carried 100% of its twigs). The probe (`R3-PROBE.md`) grew graded fine wood along thick limbs with no gate, by competition for light and space with vigour allocated down the hierarchy, but its limb count jumps from 2 or 3 to 33 or more between apical preference 0.48 and 0.5, its outline was the envelope wall, and it ran 4 to 6 times slower than the shipped skeleton. [user, checked]

Next (R3, second round): one branching rule from the scaffold's strong limbs down, growing by competition with no thickness gate, no tip shoot and no crown wall; every parameter changes the tree by degree (no jump in limb count); on every catalogue preset at two seeds; timed, growing only what survives. [inferred]
