# Branching depth: the dial the skeleton never exposed

> **SUPERSEDED, not shipped.** Merged into `fn-5-branch-until-the-tips-bear-leaves-one` before any work started: a tree is one branching recursion, and splitting depth from twigs put the crossover between them in neither spec's acceptance. Its measurements and reasoning were carried into fn-5 verbatim.

## Goal

The owner noticed the panel had no branching-depth control. Growth distances were fixed fractions of envelope height (step 2.2% of height, so 3.26 m on a 148 m tree), making terminal wood metre-scale. This spec proposed one depth dial over the growth ratios.

## Decisions

Owner (verbatim):
> "Which of these 2 specs should add the ability to increase the branching depth? i don't see one in the parameters?"
> "that would already help the leaves problem dramatically"
> "yes /capture that and then the clipping one"

- Measured lever: halving the step to 0.89 m took shoots from 126 to 2,473 and finest wood from 79.8 cm to 20.6 cm (1.08 M triangles, 413 ms build vs 194 ms).
- Found an unguarded collapse: at a 0.44 m step with 1,600 attractors growth stalls (186 nodes, 4 runs, no shoots) because the influence radius is nine steps; the fix was to derive the search radius from attractor spacing.
- Rejected: the attractor dial (40x attractors buys 13% finer wood) and exposing step/kill/influence separately (their coupling caused the collapse).

## Acceptance Criteria

- **R1:** One named depth parameter on the skeleton stage, on the panel and in both presets, scaling growth distances together — superseded by fn-5.
- **R2:** The starved-growth collapse made unreachable (search radius from attractor spacing) — superseded by fn-5.
- **R3:** Hitting `maxNodes` is reported, not silent — superseded by fn-5.
- **R4:** Cost curve measured across the depth range on both presets — superseded by fn-5.
- **R5:** Builds at depth stay usable while dragging a dial — superseded by fn-5.
- **R6:** Deterministic; default depth reproduces today's trees byte-identically — superseded by fn-5.
- **R7:** Owner confirms in clay the deeper tree reads finer and states the ship depth — superseded by fn-5.

## Result

Not built; closed as superseded by fn-5.

## Open

None here; carried by fn-5.
