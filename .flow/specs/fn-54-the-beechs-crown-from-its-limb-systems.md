## Conversation Evidence

> user (2026-09-15, on the round-18 beech): "birch is approaching good now. But the beech is still pretty bad"
> user (2026-09-15): "material is bad structure is off"
> user (earlier, rounds 5 to 5c): "clearly not structurally sound. The reference grows relatively straight up and out. Our generation bends too much." / "much more thick core trunks for almost the entire height of the tree" / "Fewer larger branches compared to ours which has many more thinner ones directly attached to the trunk"
> worker, round 18 (2026-09-15): "every crown term in the leaf shader reads one smooth ellipsoid ... so no row can darken a hollow between two clumps or light a clump's own face. Lobes need two things no row states: limbs that group their leaves into clumps with gaps between them, and a shading term that reads local depth in the leaf mass."

## Goal & Context
<!-- scope: business -->
<!-- Goal & Context: 35% [user], 40% [paraphrase], 25% [inferred] -->

The birch now approaches its photographs; the beech does not, in structure or in material. Eighteen rounds on the beech's table have moved it between a vase (a low split into many limbs), a leader with level limbs (round 11, the host's direction, not the owner's), a conifer of fronds, and now an even green felt on a round head. [paraphrase]

The photographs say what the beech is. Winter (B-BARE): a thick, straight trunk to about two fifths of the height, where it gives way to a handful of large limbs that rise steeply and straight, the crown an upright oval narrowing to the top, the fine twigs out on those limbs. Leaf-on (B-WHOLE): the same architecture clothed, each big limb system carrying its own rounded, lit leaf mass, so the crown reads as lobes with shade pockets between them, leafed nearly to the ground at the sides. The structure and the lobes are one thing: the limb systems are the lobes. [paraphrase]

This spec builds the beech's crown from its limb systems: the architecture from the photograph by values where values reach it, sub-crowns per limb system where they do not, a leaf shading term that reads the depth of a leaf inside its own sub-crown, and the leaf's material (colour, transmission, sheen) set against the photograph. It is judged on the matched pairs. [inferred]

## Architecture & Data Models
<!-- scope: technical -->

- **Values first, measured against the photograph.** From B-BARE, measure the photograph's trunk height to the first large limb, the number of large limbs, their angle from vertical and their straightness, and the crown's width profile; set the beech's habit, envelope and radius rows to match with the quick look (`npm run species:quick`), keeping the node budget. The owner's words above bind: straight up and out, a thick core, fewer larger branches. Round 11's rule that the leader runs through the crown is withdrawn: the photograph's trunk gives way to its limbs at about two fifths. [paraphrase]
- **Sub-crowns per limb system, if values cannot make them.** Today the envelope samples its attractors uniformly (`Envelope::sample`), so every limb grows into one shared cloud and the leaf mass fills the shell evenly. A clumping row lets the first-order limbs each claim their own cluster of attractors (or the twig and short-shoot layers thin toward the boundaries between limb systems), so each limb system forms its own rounded leaf mass with gaps between them. Neutral reproduces today's uniform sampling byte for byte. [inferred]
- **Depth inside a sub-crown.** Each leaf placement carries how deep it sits inside its own limb system's leaf mass (computed once at placement from the placements around it, not per frame), and a canopy row darkens the sky and transmitted light by that depth, so a lobe's face is lit and the pocket between lobes falls into shade. Neutral draws today's frame byte for byte. [inferred]
- **The leaf's material.** The beech's leaf faces, transmission colour and sheen are set against B-WHOLE's own leaf pixels; the photograph's clipped sky means the level is compared in the photograph's own exposure (scale by its sky), as round 18 measured. [paraphrase]

## Edge Cases & Constraints
<!-- scope: technical -->

- **Neutral is inert.** Every shipped preset at the new rows' neutral values is byte-identical, asserted by the pins and the pinned stills. [paraphrase]
- **Budget.** The node ceiling does not move; foliage instances stay inside the protocol's fidelity band; the depth term's frame cost is recorded beside fn-52's. [paraphrase]
- **Other species.** The birch, oak and spruce are not tuned here. [paraphrase]
- **Tests do not pin the shipped beech.** New tests build their own families. [inferred]

## Acceptance Criteria
<!-- scope: both -->

- **R1:** The photograph's architecture is measured from B-BARE and recorded, and the beech's habit, envelope and radius rows are set to it; B-BARE reads, to the implementer's eye on the quick look, as a thick straight trunk to about two fifths giving way to a handful of large, straight, steeply rising limbs in an upright oval. Errors: if values cannot reach it, the mechanism that holds it is named with numbers. [user]
- **R2:** If the leaf-on crown does not break into lobes by R1 alone, the clumping row exists with a rail, neutral byte-identical, refused by name outside it, on the wire, blended, in the regenerated metadata and on the harness, and the beech states it so each limb system carries its own leaf mass with gaps between. Errors: a value outside the rail is refused naming the field. [inferred]
- **R3:** The depth term exists as a canopy row, neutral byte-identical, native and browser, its cost recorded; the beech states it so lobes are lit on their faces and shaded between. [inferred]
- **R4:** The beech's leaf material is set against B-WHOLE's leaf pixels in the photograph's exposure. [paraphrase]
- **R5:** The matched pairs are rendered through the full runner, the 48-case protocol passes, the numbers are recorded beside round 18's in `.flow/evidence/fn34/REPORT.md`, the implementer answers after looking whether B-BARE reads as the photograph's architecture and B-WHOLE as lobed limb systems leafed nearly to the ground, and the owner judges in fn-34. Errors: a rejecting verdict stops the spec with the owner's words and a one-paragraph blocker. [user]

## Boundaries
<!-- scope: business -->

- The beech only; the birch is approaching good and is not touched. [user]
- No bark; fn-40 owns it. No new species. [paraphrase]

## Resolved via Codebase

- Attractor sampling: `crates/telperion-core/src/envelope.rs:134` (`Envelope::sample`, uniform over the shell); used from `crates/telperion-core/src/branching/specimen.rs:84-89` and `branching.rs:116`.
- The leaf shader's crown terms: `crates/telperion-render/src/shaders/foliage.wgsl:67-71`, `:123` (`crown_shade` through one ellipsoid chord), `canopy.wgsl` (fn-52's five rows).
- Short shoots and placement: `crates/telperion-core/src/foliage/short_shoots.rs`, `foliage/placement.rs`.
- The beech's table: `crates/telperion-core/src/presets/species.rs` (round 18), `presets/materials.rs`.
- The quick look: `npm run species:quick -- --profiles .flow/evidence/fn34/profiles.json --quick european-beech` (commit f0430ec7).
- Rounds 5 to 18 on the beech: `.flow/evidence/fn34/REPORT.md`.
