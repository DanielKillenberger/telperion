# Sprouts and leaf size by age

## Conversation Evidence

> user (2026-09-15, on fn-31's round-6 strips): "we probably need the ability to have sprouts and smalelr leaves? just having full grown leaves makes it look bad."
> user (2026-09-15): "i think smaller leaves and sprouts should probably be a separate spec. We just need structural integrity for now and then the sprouting leaves will carry a lot."

## Goal & Context
<!-- scope: business -->
<!-- Goal & Context: 50% [user], 50% [paraphrase] -->

Every leaf and needle Telperion draws is the family's mature element at full size from the day its shoot is born, so a one-year seedling carries adult leaves and a ten-year sapling reads as a stem coated in them. The owner judged fn-31's strips and named it: full-grown leaves make the young tree look bad, and sprouting leaves will carry a lot of the realism once the structure is right. [user]

This spec gives foliage an age: a leaf or needle starts as a sprout and grows to the family's element size as its shoot and the tree age, as numeric traits every family walks. fn-31 owns the structure it hangs on; this spec owns the element's size and its unfolding. [paraphrase]

## Architecture & Data Models
<!-- scope: technical -->

- **Element scale by age.** A leaf station carries its element scaled by a function of the shoot's age since the station's birth and the tree's size, from a sprout fraction at birth to one at full size, as traits on the family (sprout scale, seasons to full size) that the blend walks; the element geometry itself is unchanged, so the leaf counts, cohorts and the chronicle stay as fn-11 pinned them. [inferred]
- **Reads, not stamps.** The scale is a pure function of ages the chronicle already records, so a tree at any age reads its sprouts without a new stamp, and fn-28's blend between years scales a sprouting leaf continuously from its bud. [inferred]

## Acceptance Criteria
<!-- scope: both -->

- **R1:** On fn-31's strip protocol, the oak at 1 and 10 years and the spruce at 1 and 5 years carry sprouts and small leaves or needles that read as young foliage, and the mature trees carry the family's full element; the owner judges the strips and records an accepting verdict in the owner's own words. [paraphrase] Errors: a rejecting verdict stops the spec with the owner's words and a one-paragraph blocker.
- **R2:** Element scale is a numeric trait every family carries, validated by range naming the field, walked by the blend, with no species or preset branch; the mature identity pins are byte-identical with the traits at their mature values. [inferred] Errors: a field outside its range is refused naming it.
- **R3:** The scale is a function of ages the chronicle records, never a new stamp; determinism and native-to-wasm parity hold. [inferred] Errors: a parity or determinism failure names the preset and the age.

## Boundaries
<!-- scope: business -->

- No change to fn-31's structure, crown or stem rules; this spec scales what hangs on them. [user]
- No bud-opening or unfurling animation; a sprout scales up, and folded blades are fn-28's or a later fidelity step. [paraphrase]
- No seasons, no leaf fall by colour; those stay with the lifecycle specs. [paraphrase]

## Decision Context
<!-- scope: both -->

### Motivation

- Split from fn-31 on 2026-09-15 at the owner's choice so fn-31 finishes structural integrity first; the owner expects sprouting leaves to carry much of the remaining realism at young ages. [user]
