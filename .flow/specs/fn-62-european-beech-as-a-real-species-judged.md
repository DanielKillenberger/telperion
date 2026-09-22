## Conversation Evidence

> user (2026-09-16, round-22 beech verdict): "The tree has clear regular outline/border that doesn't look natural. The taper approaching the border needs to produce thinner branches twigs. It's also too dense at the shoulder of the crown. It's more sparse lower and gets more dense at the top. It generally looks too dense everywhere?"
> user (2026-09-16): "We have trees where the branches point upwards when there are no leaves. But they droop under the weight of leaves. That's one problem. We still haven't achieved the reference for the bare one either. It still looks off. And you could probably inspect the image to figure out why. But the bigger issue is with the hwhole tree that doesn't match the reference at all."
> user (2026-09-16, on the path review): "ok let's do that."

## Goal & Context
<!-- scope: business -->
<!-- Goal & Context: 25% [user], 40% [paraphrase], 35% [inferred] -->

The European beech moves out of fn-34, as the ash did, so fn-34 can close on the birch. The beech's profile, references, packet, preset and every verdict so far carry over from fn-34 (`.flow/evidence/fn34/european-beech/`, fn-34's Owner verdicts, rounds 1 to 23 in `.flow/evidence/fn34/REPORT.md`). [paraphrase]

fn-34 ran 23 rounds against an acceptance that had no finish line. This spec closes on a short checklist the owner ticks, so each round knows what it is for and when to stop. [paraphrase]

## Acceptance checklist
<!-- scope: business -->

The owner answers each line yes or no on matched pairs at three fixed seeds. The spec closes when every line is yes and the last line is yes. [paraphrase]

Bare (B-BARE):
- One trunk stands clear to about a third of the tree's height before the crown divides.
- The lower limbs spread wider than the upper ones.
- Branches zigzag and turn rather than run as straight rods.
- The wood thins to a haze of fine twigs at the crown's edge.
- The crown's edge is ragged and open, not a smooth dome.
- The wood is grey-brown, not pale.

Leaf-on (B-WHOLE):
- The crown is a broad rounded column, reaching low at the sides.
- Its edge is ragged, with the limb systems' lumps showing.
- It carries a range from sunlit to shaded, not one flat tone.

Close-up (B-BASE): the bark reads as the photograph's smooth grey with lichen patches, without dark dashes.

Whole: the tree reads as a beech.

The compare script's numbers are kept for every round as regression readings. They do not gate a line. [paraphrase]

## Architecture & Data Models
<!-- scope: technical -->

- **Order.** (1) A bare-only value round: the crown base back near B-BARE's reading (fn-54 lowered it to 0.06 for the leaf mass), more `crookedness`, round 23's twig taper, and the beech's lichen and lenticel rows toned toward B-BASE. (2) The Troll's-model generator spec (pitch by height, a ragged reach), then its rows stated. (3) The whole tree judged on the checklist. (4) fn-59's leaf-load bend only if the summer crown still sits high. [paraphrase]
- **Two rounds per verdict.** A verdict that is not yet accepting allows at most two value rounds before the host names a generator gap or stops with `NEEDS_HUMAN`. A note that names something no row governs goes straight to a gap. [paraphrase]
- **Jev.** Jev classifies each note as a value row, a generator gap or a tone issue, and picks the checklist lines a verdict touches, from options code gives it. It does not run trial loops over rows. [paraphrase]
- **The judging page.** Each round is added to the owner's page with the checklist as the verdict form, saved where the host reads it back. [inferred]

## Acceptance Criteria
<!-- scope: both -->

- **R1:** Every checklist line is answered yes by the owner at three fixed seeds, recorded verbatim in this spec. Errors: a no names the line and the round; it is not a failure of the spec. [user]
- **R2:** The 48-case protocol passes at every shipped round, none capped, and the identity pins are re-recorded once per round with the reason. Errors: a numeric failure is a retained case, never a resample. [paraphrase]
- **R3:** Each round's pairs, numbers and the host's read are recorded in a round section with stills by sha256. Errors: none beyond the record. [paraphrase]
- **R4:** On acceptance the beech moves from `params::IN_WORK` back into `CATALOGUE` under its reserved ABI id 5, `EUROPEAN_BEECH` returns to the browser exports, and the binding test lists seven identities again. Until then it is unlisted and refused by name (owner, 2026-09-16: hold it out of the public list). Errors: the in-work test fails if the id is reused. [user]

## Boundaries
<!-- scope: business -->

- The beech only. The birch and the ash are not touched. [paraphrase]
- Materials beyond the beech's own rows wait for fn-55. [inferred]
- Leaf form is fn-60's and runs in parallel. [inferred]
- One round is run by the fn-68 tuning loop as its live pilot (owner, 2026-09-21), in visual bootstrap mode from the round-22 rows at seed 1. The loop's candidate and its gap handoffs are inputs to that round; they change no shipped row by themselves, and acceptance stays the owner's checklist. [user]

## Resolved via Codebase

- The beech's rows: `crates/telperion-core/src/presets/species.rs` (`european_beech`), `crates/telperion-core/src/presets/materials.rs` (`beech()`).
- The beech's references: `.flow/evidence/fn34/european-beech/references.json` (all `matching: qualitative`; B-WHOLE `fasy951`, B-BARE and B-BASE `fasy896`, two different trees).
- The path review: `.flow/evidence/fn34/review-2026-09-16/README.md`.
