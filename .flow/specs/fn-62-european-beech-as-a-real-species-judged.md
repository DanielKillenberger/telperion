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

**Method (owner, 2026-09-27): the species runner, not hand-run value rounds.** The beech is run by `species european-beech` (fn-149, fn-157). The rounds, Jev routing and judging page this spec first described are superseded by the runner's stages; the owner's checklist below stays the acceptance. [user]

- **Sources and profile, free.** fn-157's recorded live beech run (its tape, seeded from a bare seed with `taxon.native_range`: height 40 m from three agreeing forestry sources) is replayed with `--extend`, so Sources and Profile cost no Firecrawl and no Jev; only questions the rebuilt runner asks anew are recorded. [checked]
- **References.** The run carries the beech's curated references with matched shots (`catalogue/european-beech/packet/references.json`: B-WHOLE, B-BARE and B-BASE with shots, B-LEAF and B-LEAVES without), so the Profile stage's photograph search does not run (it runs only below two references) and Tune compares against the views the owner already judged. [checked]
- **Capability (host).** The host's assessment names `woody-axes`, `entire-blade` and `alternate-petiole` (expressed) and covers the traits the new rows serve: the codominant V fork (fn-170), the lower limbs spreading wider and the ragged crown edge (fn-61), limbs holding their girth far out (fn-177); every such need is now expressed. [inferred]
- **Tune** moves every live dial, including fn-61's `pitchByHeight` and `raggedReach`, fn-170's fork rows and fn-177's `girthHold` and `girthFall`, from the shipped beech's rows. [inferred]
- **Gaps decides branching hierarchy.** The owner's reading of the beech and the London plane (2026-09-27): a few strong scaffold limbs that stay thick far out, sparse branching in the middle orders, dense twigs at the crown's edge. If Tune cannot reach it with the live dials, Gaps classes it identity with the host to assess, and that finding becomes the hierarchy spec this spec then depends on. [inferred]
- **The owner's look** is in the harness (fn-166, `species european-beech --look`). Accepting writes the beech's value file and its catalogue pins (fn-149, fn-152). [inferred]

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
- (Superseded 2026-09-27 by the runner.) One round is run by the fn-68 tuning loop as its live pilot (owner, 2026-09-21), in visual bootstrap mode from the round-22 rows at seed 1. The loop's candidate and its gap handoffs are inputs to that round; they change no shipped row by themselves, and acceptance stays the owner's checklist. [user]

## Resolved via Codebase

- The beech's rows: `crates/telperion-core/src/presets/species.rs` (`european_beech`), `crates/telperion-core/src/presets/materials.rs` (`beech()`).
- The beech's references: `.flow/evidence/fn34/european-beech/references.json` (all `matching: qualitative`; B-WHOLE `fasy951`, B-BARE and B-BASE `fasy896`, two different trees).
- The path review: `.flow/evidence/fn34/review-2026-09-16/README.md`.
