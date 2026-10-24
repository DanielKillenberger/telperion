# A limb tip grows no extra shoot

## Goal

When a scaffold limb's tip had no children, the twig layer seeded one continuation shoot at the tip's full radius, 38.97 · r^(2/3) long (about 13 m on a 0.2 m limb; `local/seed.rs:158`, `twigs.rs:394`). On the beech it made 77% of each visible horn and pushed tips 5 m above the crown (fn-182's horn trace). fn-188's ablation deleted it: on the beech at seed 1 the trunk path fell from 31.87 m to 27.27 m (envelope 32), the visible pole from 4.14 m to 0.49 m, nodes −12% (213,044 to 187,972), leaves −15%, skeleton 71 to 56 ms, generate-and-mesh 3,326 to 2,815 ms cold. This spec was to delete the special case on the direct build for every preset.

## Decisions

- owner (2026-10-02, on the beech's thick leaders running bare past the crown): "now they look like devil horns"
- owner (2026-10-02, on shipping the deletion on its own): "makes sense" — it replaced fn-182's paused "the continuation takes the ordinary shoot length".
- Held (owner, 2026-10-02): deleting the tip shoot strips the twig layer from trees whose scaffold is everywhere thicker than the twig gate: Telperion at seed 7 dropped from 18,140 nodes and 128,852 leaves to 363 nodes and none; ordinary, laurelin and Telperion at seed 1 lost 25 to 41% of their leaves. The "helps every tree" claim rested on the beech alone and was wrong. Owner: "figure out what it was and make sure that we have the elegant right solution. Don't just accept looks good without understanding why".
- Superseded by fn-190 (owner, 2026-10-03): the tip shoot goes together with the thickness gate it compensated for, as part of fn-190's deletions.

## Acceptance Criteria

- **R1:** No childless scaffold tip seeds a continuation shoot, pinned on a synthetic family and every preset — not merged (superseded).
- **R2:** Paired `growth_profile` runs for every preset at seeds 1 and 7, no preset slower — measured on the branch; not merged.
- **R3:** Stills of every preset beside today's with the owner's verdict — superseded.
- **R4:** Gates green, sizes and memory reported — superseded.

## Result

Not merged. The trace (`TRACE.md` and `RESULT.md` on branch `fn-189-a-limb-tip-grows-no-extra-shoot`) showed the tip shoot compensates for the `limbRadius` thickness gate: Telperion at seed 7 has every scaffold node at or just above the gate, so its tip shoots carried 100% of its twigs.

## Open

Carried by fn-190.
