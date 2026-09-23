# Dead fronds hang as a skirt below a rosette's living crown

## Conversation Evidence

> host, fn-80 gap list (2026-09-22 and 2026-09-23): the reviewer names "no skirt of drooping lower fronds" in every revision; the inventory lists `dead-frond-skirt` (secondary).
> user (2026-09-23): "4. spec it"

## Goal & Context
<!-- scope: business -->

A date palm keeps its oldest fronds after they die: brown to grey, collapsed downward against the upper trunk below the living crown. Washingtonia and many other palms do the same, and a rosette of any kind that retains dead leaves shows it. The generator's rosette (fn-109) draws only living fronds, so the palm's crown ends abruptly and the reviewer asks for the skirt every round. [paraphrase]

## Architecture & Data Models
<!-- scope: technical -->

- **What exists, checked 2026-09-23 on the fn-80 branch.** `foliage/rosette.rs` places fronds at each stem apex on a phyllotactic spiral, rows on `CanopyParams` (`rosette_fronds`, `rosette_pitch`, `rosette_pitch_spread`, `rosette_depth`, divergence); each frond expands through `fan` into leaflets. The trunk's leaf bases (fn-110) follow the same spiral downward. [checked]
- **Shape.** [inferred]
  - The skirt is the rosette's history continued: a count of retained dead fronds below the living ones on the same spiral, a pitch that droops them toward the trunk, a length share of the living frond, and a colour the material layer takes from the leaf's back colour aged toward a stated dead colour. Neutral values (count zero) draw nothing and leave every family byte-identical.
  - Generic to any rosette, never a palm branch; every parameter a validated row with a doc comment and a dial-table entry; `dead-frond-skirt` joins the capability vocabulary's expressed list.
- **Unknown.** Whether the dead colour rides the existing per-instance-free foliage draw (a second colour pair, as fn-33 plans for organs) or needs its own draw. The implementer measures both and picks. [unknown]

## Acceptance Criteria
<!-- scope: both -->

- **R1:** A family with a skirt count above zero draws that many dead fronds below the living rosette, drooped by the skirt pitch, in the dead colour; count zero is byte-identical to today. [inferred]
- **R2:** Every new row is validated, documented and in the dial table. [inferred]
- **R3:** The palm's preset sets a first skirt from its references; a matched still shows it (one capture, small). [inferred]
- **R4:** The gate is green: `cargo test --profile ci --workspace --no-fail-fast`. [paraphrase]

## Boundaries

- No change to the living rosette. [inferred]
- The date cluster is fn-111 and fn-33's organ slot, not this. [paraphrase]

## Decision Context

- The owner asked on 2026-09-23 for the skirt to be specced; it goes into the palm's stack. [user]

## Open Questions

- The unknown in Architecture.
