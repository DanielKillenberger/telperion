## Conversation Evidence

> user (2026-09-15): "Please do visual QA on these before submitting another review from me. If you can see it's clearly not there and you have a way to fix it you should do that."
> worker read, fn-44 round 6b (2026-09-15): "thinning made the white shoots more prominent, not less — with fewer leaves covering them the crown reads frosted where the photograph's twigs are dark ... now the single loudest fault in the image."
> worker read, round 7 (2026-09-15): "The white fine shoots, which make the S-BARE winter crown a white fog."

## Goal & Context
<!-- scope: business -->
<!-- Goal & Context: 30% [user], 50% [paraphrase], 20% [inferred] -->

Every piece of wood Telperion draws takes one bark colour, the trunk's. A silver birch's trunk is white, and its shoots are dark red-brown to purple-brown until they are several years old and centimetres thick; the photograph's curtain is dark strands against the sky with a white trunk under them. The generated birch draws its hanging curtain in the trunk's white, so the crown reads as frost. Every species has the same fault less visibly: a beech's young shoots are olive-brown, not its trunk's silver-grey. [paraphrase]

This spec gives the material a young-wood colour and the radius below which wood takes it, blended smoothly into the bark colour as the wood thickens, with the neutral value drawing today's single colour. The birch is the first specimen judged on the matched pairs. [inferred]

## Architecture & Data Models
<!-- scope: technical -->

- **The material row.** `MaterialParams` gains `shoot_red`, `shoot_green`, `shoot_blue` and `shoot_radius` (metres). Below `shoot_radius` the wood's albedo is the shoot colour; from there to twice that radius it blends to the bark colour on a smoothstep of the radius the wood vertex already carries. `shoot_radius` 0 is neutral: no wood is young, every wood fragment takes the bark colour as today, byte for byte on the frame. [inferred]
- **The uniform.** The frame's uniform block carries the shoot colour and radius beside `bark_colour_detail`; the wood shader reads `in.radius` (already a varying) and mixes before the mottle, cavity and occlusion terms, so those still apply to young wood. [inferred]
- **One colour everywhere wood is drawn.** The wood pass, the shadow casters (no colour) and the browser renderer's wood path take the same term; the clay view is untouched. [inferred]
- **Rows, not species.** The birch's and the beech's material rows state a shoot colour from their references; the oak, the spruce and the Two Trees stay neutral until their own tables say otherwise. [paraphrase]

## API Contracts
<!-- scope: technical -->

- **Family rows** `material.shootRed`, `shootGreen`, `shootBlue` (0 to 1) and `material.shootRadius` (0 to 0.1 m), validated by name, on the wire, blended linearly, in the regenerated browser metadata and on the harness. [inferred]
- **Views and commands unchanged.** [paraphrase]

## Edge Cases & Constraints
<!-- scope: technical -->

- **Neutral is inert.** `shoot_radius` 0 draws every existing still byte-identically; the render tests' pinned stills hold. The core's mesh pins do not move at all, since colour is not geometry. [paraphrase]
- **Continuity.** No radius is a frame where wood changes colour abruptly; the blend is continuous in radius and in the row. [inferred]
- **Cost.** One smoothstep and one mix per wood fragment; the frame cost is recorded beside the last render test's numbers. [inferred]
- **File sizes.** `wood.wgsl` is at 186 lines and `material.rs` at 318; the term fits in both. [paraphrase]

## Acceptance Criteria
<!-- scope: both -->

- **R1:** The four material rows exist with rails, neutral at radius 0, refused by name outside them, on the wire, blended, in the regenerated browser metadata and on the harness; every shipped preset renders byte-identically at neutral. Errors: a value outside its rail is refused naming the field. [inferred]
- **R2:** With a positive radius, wood thinner than it takes the shoot colour and blends continuously to the bark colour by twice that radius, in the native and the browser renderer, with mottle, cavity and occlusion still applied. Errors: no error surface beyond R1. [paraphrase]
- **R3:** The silver birch's material row states a dark red-brown young-shoot colour and a radius against S-WHOLE and S-BARE, the beech's an olive-brown against B-WHOLE and B-BARE; the matched pairs of both are rendered again with the numbers beside the previous round's in `.flow/evidence/fn34/REPORT.md`; the implementer does visual QA before returning and the owner judges the pairs in fn-34. Errors: a rejecting verdict stops the spec with the owner's words and a one-paragraph blocker. [user]
- **R4:** The tests cover: neutral byte identity on the pinned stills, the rails, the blend at the two ends and its continuity on a synthetic radius ramp, and the native and browser paths agreeing. Errors: a missing case is a review finding, not implementer discretion. [inferred]

## Boundaries
<!-- scope: business -->

- No lichen, lenticels or peel; fn-40 owns those. [paraphrase]
- No change to any geometry or to the leaves. [inferred]

## Decision Context
<!-- scope: both -->

### Motivation

- The owner's loop: a species exposed a missing term and the renderer gains a row, never a species branch. Two worker reads named the white shoots the loudest fault on the birch. [user]

### Implementation Tradeoffs

- Radius over branch order or age: the wood vertex carries its radius already, it is continuous, and a shoot's colour follows its girth in the field. [inferred]

## Resolved via Codebase

- Material rows: `crates/telperion-core/src/material.rs:14-60` (`MaterialParams`: `bark_red/green/blue`, roughness, relief rows); the species rows `crates/telperion-core/src/presets/materials.rs` (`beech()` at about 104, bark 0.36/0.335/0.295; `birch()` at about 154, bark 0.78/0.76/0.70).
- The frame's uniform: `crates/telperion-render/src/scene.rs:75` (`bark_colour_detail`), filled at `crates/telperion-render/src/scene/frame.rs:124`.
- The wood fragment: `crates/telperion-render/src/shaders/wood.wgsl:105-135` (`in.radius` is a varying at `:14`/`:28`; mottle at `:122-127`; ground contact and maturity at `:131-133`).
- File sizes: `wood.wgsl` 186, `bark.wgsl` 187, `frame.rs` 152, `material.rs` 318.
