# The twig layer: the branching order between shoots and leaves

> **SUPERSEDED, not shipped.** Merged into `fn-5-branch-until-the-tips-bear-leaves-one` before any work started: a tree is one branching recursion, and splitting depth from twigs put the crossover between them in neither spec's acceptance (how 5 mm twigs end up on 20 cm logs). Its measurements and reasoning were carried into fn-5 verbatim.

## Goal

After the canopy shipped, the owner judged in clay that the leaves attach to nothing a leaf could attach to. The generator had no branching order fine enough to bear foliage: on Telperion the finest wood was 0.79 m thick and the leaf 0.12 m long, the 12.85 m "shoots" were metre-thick poles, and 2,904 leaves sat as beads on 133 poles. This spec proposed a twig layer below the skeleton.

## Decisions

Owner (verbatim):
> "there's a problem with how leaves attach though? This doesn't seem natural to me. The leaves themselves seem fine but i guess we'd smaller branches to attach them to?"
> "It looks fine and could maybe be a tree in a desert but there's not enough volume to the tree either. With this dense of a branch system i'd be expecting a full ass covered canope"
> "this might be a small adjustment though you basically just need another branching layer to attach the leaves to?"
> "but maybe i'm wrong"
> (boundary) "we also need a spec to fix cliping and overlying branches separately"
> "how would a 150m tall tree work in real life? would it have giant leaves? or would there be millions of leaves?"

- Rejected: turning up density (40x attractors moved the finest wood only from 0.79 m to 0.69 m; colonization's step is a fraction of envelope height), more leaves per shoot (denser poles, same attachment count), and bigger leaves (leaf size shrinks with height; a crown this size carries 10^5 to 10^7 leaves).
- Branch clipping is a separate spec (became fn-4).

## Acceptance Criteria

- **R1:** A twig stage generates second-order branches on the canopy's shoots, every term a named parameter with a rail — superseded by fn-5.
- **R2:** Leaves attach to twigs, leaf length a multiple of its wood's diameter on both presets — superseded by fn-5.
- **R3:** Attachment points and leaf count rise at least two orders of magnitude at preset density — superseded by fn-5.
- **R4:** Twig geometry visible in clay with at most one extra draw call per tree — superseded by fn-5.
- **R5:** Same seed gives identical twigs and canopy; earlier stages byte-identical — superseded by fn-5.
- **R6:** Frame budget and shell-cull fraction re-measured at the new element count — superseded by fn-5.
- **R7:** The owner confirms in clay that the crown reads covered, not beaded — superseded by fn-5.

## Result

Not built; closed as superseded by fn-5.

## Open

None here; carried by fn-5.
