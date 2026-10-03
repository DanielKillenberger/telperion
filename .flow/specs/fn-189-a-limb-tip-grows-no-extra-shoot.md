# A limb tip grows no extra shoot

## Conversation Evidence

> owner (2026-10-02, on the beech's thick leaders running bare past the crown): "now they look like devil horns"
> owner (2026-10-02, on fn-188's step 2 and the host's recommendation to ship the deletion on its own): "makes sense"

## Goal & Context
<!-- scope: business -->
<!-- Goal & Context: 20% [user], 70% [checked], 10% [inferred] -->

When a scaffold limb's tip has no children, the twig layer seeds one continuation shoot at the tip's full radius, with a length grown from that radius: 38.97 · r^(2/3), about 13 m on a 0.2 m limb (`local/seed.rs:158`, `twigs.rs:394`, kept along the run by `local/advance.rs:108`). On the beech it made 77% of each visible horn and pushed tips 5 m above the crown (fn-182 `R8-HORNS.md`). fn-188's step 2 deleted it outright (`.flow/evidence/fn-188-one-branching-law-from-trunk-to-twig/R2-ABLATIONS.md`, ablation b): on the shipped beech at seed 1 the trunk path tops out at 27.27 m instead of 31.87 m (envelope 32), the visible pole falls from 4.14 m (R8's shortening) to 0.49 m, nodes fall 12% (213,044 to 187,972) and leaves 15%, skeleton time 71 to 56 ms, generate and mesh 3,326 to 2,815 ms cold. [checked]

This spec deletes that special case on the direct build, for every preset. [inferred]

## Architecture & Data Models
<!-- scope: technical -->

- The deletion is the one fn-188 measured: a childless scaffold tip seeds no full-radius continuation. Terminal buds elsewhere, laterals, curtains and the twig law's own continuations are unchanged. [checked: R2-ABLATIONS ablation b]
- Code that becomes dead on every path is removed; the hidden growth path (removed by fn-181 where landed) is not tuned. [inferred]
- Every preset's tree changes; byte identity is not required (AGENTS.md "Generator evolution"); pins and digests are re-recorded with the reason. [AGENTS.md]

## Edge Cases & Constraints

- The date palm's rosette apex already grows no twig layer (fn-183); its drawn tree is checked unchanged or explained. [checked]
- No full-forest capture. [AGENTS.md]

## Acceptance Criteria

- **R1:** On the direct build no childless scaffold tip seeds a continuation shoot; a test pins it on a synthetic family and on every catalogue preset. [inferred]
- **R2:** Paired `growth_profile` runs against one named base for every catalogue preset at seeds 1 and 7: nodes, leaves, skeleton time, generate-and-mesh time; no preset slower beyond the noise. [inferred]
- **R3:** Stills of every catalogue preset at seeds 1 and 7 beside today's, bare and whole, at a fixed camera per preset, with the owner's verdict. [user]
- **R4:** `cargo test --profile ci --workspace --no-fail-fast` and `npm test` green; artifact sizes and peak memory reported. [AGENTS.md]

## Boundaries

- The branching hierarchy and twig density are fn-188's and fn-182's. [inferred]

## Decision Context

- Shipped on its own because it helps every tree and does not depend on fn-188's outcome (owner, 2026-10-02). It replaces fn-182's paused "the continuation takes the ordinary shoot length". [user]

## Strategy Alignment

- `docs/principles.md`, "Question, delete, then optimise": a special case deleted, every tree faster. [strategy:Our approach]

## Held (owner, 2026-10-02)

Deleting the tip shoot strips the twig layer from trees whose scaffold is everywhere thicker than the twig gate: telperion at seed 7 drops from 18,140 nodes and 128,852 leaves to 363 nodes and none; ordinary, laurelin and telperion at seed 1 lose 25 to 41% of their leaves (`RESULT.md` on branch `fn-189-a-limb-tip-grows-no-extra-shoot`). The claim that the deletion "helps every tree" rested on the beech alone and was wrong. The tip shoot compensates for the same thickness gate fn-188 is examining, so this spec waits on fn-188; the mechanism is traced before any choice (owner: "figure out what it was and make sure that we have the elegant right solution. Don't just accept looks good without understanding why"). [checked]

## Superseded by fn-190-one-growth-law-species-are-points-in-a (owner, 2026-10-03)

The tip shoot goes together with the thickness gate it compensated for, as part of fn-190-one-growth-law-species-are-points-in-a's deletions; deleting it alone strips whole twig layers (`TRACE.md` on branch `fn-189-a-limb-tip-grows-no-extra-shoot`). [user]
