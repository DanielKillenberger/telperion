## Goal

After eighteen rounds the birch approached its photographs but the beech did not, in structure or material: it had moved between a vase, a leader with level limbs, a conifer of fronds and an even green felt. The photographs show a thick straight trunk giving way to a handful of large, steeply rising, straight limbs in an upright oval; leaf-on, each limb system carries its own lit leaf mass, so the crown reads as lobes with shade pockets. This spec builds the beech's crown from its limb systems: architecture by values where they reach, sub-crowns per limb system and a depth shading term where they do not, and the leaf material set against B-WHOLE.

## Decisions

- user (2026-09-15, on the round-18 beech): "birch is approaching good now. But the beech is still pretty bad"
- user (2026-09-15): "material is bad structure is off"
- user (earlier, rounds 5 to 5c): "clearly not structurally sound. The reference grows relatively straight up and out. Our generation bends too much." / "much more thick core trunks for almost the entire height of the tree" / "Fewer larger branches compared to ours which has many more thinner ones directly attached to the trunk"
- worker, round 18 (2026-09-15): "every crown term in the leaf shader reads one smooth ellipsoid ... so no row can darken a hollow between two clumps or light a clump's own face. Lobes need two things no row states: limbs that group their leaves into clumps with gaps between them, and a shading term that reads local depth in the leaf mass."
- Round 11's leader-through-the-crown rule withdrawn. The host accepted the measured core reading (trunk clear to about a fifth, core thickest to about half) over the spec's two fifths; beech cuticle gloss 0.48 to 0.18 (owner). The beech only; birch untouched.
- The 62 ms mass grid is built for every preset at submission; skipping it at `lobe_shade: 0` needs a contract change (material in force before submit, or a GPU-built grid), left to the owner.
- Owner's round-22 read (2026-09-16): "The tree has clear regular outline/border that doesn't look natural. The taper approaching the border needs to produce thinner branches twigs. It's also too dense at the shoulder of the crown. It's more sparse lower and gets more dense at the top. It generally looks too dense everywhere?" The owner kept depth shading at 0.7 (choice A).

## Acceptance Criteria

- **R1:** B-BARE's architecture measured and the beech's habit, envelope and radius rows set to it — met (limbs 28 degrees from vertical, spread 0.36, shoulder 1.8; 4cba7a0d).
- **R2:** clumping row railed, neutral byte-identical, wire, blend, metadata, harness; beech states it — met (`canopy.limbClumping`, thinning toward walls between limb systems).
- **R3:** depth term as a canopy row, neutral byte-identical, native and browser, cost recorded — met (`material.lobeShade`; beech hero 10.23 to 10.24 ms p50, grid build 62 ms for 4.73 M leaves).
- **R4:** beech leaf material set against B-WHOLE in the photograph's exposure — measured, unmet (centre 56 against 80; depth shading costs 14 counts and buys no outline); carried to fn-62.
- **R5:** pairs through the full runner, 48-case protocol, implementer read, owner judges — not accepting at round 22; carried to fn-62.

## Result

R1 to R3 delivered and merged (3b6f4e7f, a061b46d). Final rows: crown base 0.06, fullness 0.3, twig length ratio 0.23, lobe shade 0.7. The owner accepted the silver birch at fn-34 round 25 (2026-09-16); the European beech's verdict moved to fn-62 with the owner's round-22 notes.

## Open

The beech's look (R4, R5) is fn-62's. Skipping the mass grid for presets at lobe shade 0 is an owner decision.
