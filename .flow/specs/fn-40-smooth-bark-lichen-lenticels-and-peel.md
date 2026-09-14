# Smooth bark: lichen, lenticels and peel

## Conversation Evidence

> user (2026-09-14, on the fn-34 round-3 pairs): "also do we have a texture spec? because the beech bark is clearly not representable with what we have atm."
> gap analysis (`.flow/evidence/fn34/GAPS.md`): the beech's bark reads blue-grey and featureless in the B-BASE pair against a photograph of smooth grey bark spotted with lichen.

## Goal & Context
<!-- scope: business -->
<!-- Goal & Context: 30% [user], 45% [paraphrase], 25% [inferred] -->

The bark field the renderer has is a field of ridges, furrows, plates and flakes, and fn-32 is making those plates a real cellular network with depth. Beech and birch bark are the other kind: thin and smooth, with no furrow to shade. What the eye reads on them is colour structure at two scales, lichen patches on the beech and dark lenticel bands and peeling strips on the birch, and a few folds where limbs meet. The B-BASE and S-BARK pairs show the gap: the generated trunks are a flat grey and a flat white, and the profile's own observation for the beech says "smooth grey bark with lichen spots". [paraphrase]

This spec adds the smooth-bark terms as rows on the material, inert by default: a lichen layer, a lenticel layer and a peel layer, each a numeric row set with its own scale, coverage, tint and strength, judged with fn-32's measurement instrument on the matched base and bark pairs. [inferred]

## Architecture & Data Models
<!-- scope: technical -->

- **Lichen as a second mottle with its own colour.** A cellular patch field over the arc and along coordinates at a lichen scale, with a coverage fraction that thresholds it, a tint and a strength; patches are footprint-faded the way the ridges are so distant trunks converge to the mean. The beech reads as grey with pale grey-green spots by its rows. [inferred]
- **Lenticels as short horizontal dashes.** A field of dashes across the arc coordinate at a stated density and length, dark on a pale bark, with a strength row; on the birch they band the trunk, on the beech they are faint. The relief height gains a shallow groove per dash so the existing cavity term shades them. [inferred]
- **Peel as lifted strips.** fn-32's plate edge lift and directional occlusion, with the plate network set to horizontal bands (elongation across the arc, a large plate scale), give a peeling strip its lit edge and shaded underside; this spec adds a curl strength row and a peel tint for the exposed inner bark, and nothing else, so the birch's peel is fn-32's primitive at birch values. [inferred]
- **Folds belong to the geometry.** Elephant-hide folds where limbs meet the trunk are fn-20's collars and ridges; this spec does not shade them. [paraphrase]
- **Judged by measurement first.** fn-32's receipt, centre-crop mean, channel order and the structure statistics, runs on the B-BASE and S-BARK matched pairs from fn-36 before any image is viewed; the owner judges the pairs last. [paraphrase]

## API Contracts
<!-- scope: technical -->

- **Material row additions**, validated by range naming the field, on the wire, in the generated browser metadata and in the sweep's inventory the way fn-14's, fn-26's, fn-29's and fn-32's rows are: `lichen_scale`, `lichen_coverage`, `lichen_red/green/blue`, `lichen_strength`, `lenticel_density`, `lenticel_length`, `lenticel_strength`, `lenticel_tint`, `peel_curl`, `peel_red/green/blue`. Every default is inert so an existing document renders exactly as before. [inferred]
- **Views and commands unchanged**; the fn-36 matched base and bark shots render through the existing views and cameras. [paraphrase]

## Edge Cases & Constraints
<!-- scope: technical -->

- **The silhouette is untouched.** Every term shades; the wood mesh bytes and the fn-24 pins are unchanged with the rows on and off, and fn-32's depth mechanism is used as it lands, not extended. [paraphrase]
- **Distance.** Lichen and lenticels are footprint-faded on both axes like the ridges, and every colour term stays affine in the filtered height per the fn-29 rule, so the distance and resolution tests hold at their unchanged bounds. [paraphrase]
- **Determinism.** One tree under one row is one picture; the look test's redraw check holds and no new term widens its tolerance. [paraphrase]
- **Budget.** The oak's native frame with every fn-32 term on is the baseline; this spec's terms are measured beside it and the owner sets the bound before the still round, as fn-32 does. [paraphrase]
- **References.** The beech B-BASE and birch S-BARK photographs already catalogued for fn-34, plus one close bark photograph per species fetched into the ignored references directory with an fn-19 record and a shot block. [inferred]
- **fn-32 in flight.** This spec starts after fn-32 lands so the plate network, the edge lift, the directional occlusion and the measurement receipt exist to build on. [paraphrase]

## Acceptance Criteria
<!-- scope: both -->

- **R1:** A lichen layer as rows, a cellular patch field with scale, coverage, tint and strength, footprint-faded, inert by default; the beech's table sets it against B-BASE. Errors: a row outside its range is refused naming the field. [user]
- **R2:** A lenticel layer as rows, horizontal dashes with density, length, tint and strength and a shallow groove in the relief height, inert by default; the birch's and the beech's tables set it against S-BARK and B-BASE. Errors: no error surface beyond row validation. [inferred]
- **R3:** Peel as fn-32's plate network at horizontal band values plus a curl strength and an inner-bark tint, inert by default; the birch's table sets it against S-BARK. Errors: no error surface beyond row validation. [inferred]
- **R4:** The wood mesh bytes and the fn-24 pins are identical with every new row on and off; the distance, resolution and redraw tests hold at their unchanged bounds; the frame cost is recorded beside fn-32's numbers and stays under the bound the owner sets. Errors: a number over the bound stops the spec with the number. [paraphrase]
- **R5:** The B-BASE and S-BARK matched pairs are rendered again through fn-36's rig, fn-32's measurement receipt is recorded for still and photograph, and the owner judges the pairs and records the verdict in fn-34. Errors: a rejecting verdict stops the spec with the owner's words and a one-paragraph blocker. [user]

## Boundaries
<!-- scope: business -->

- No image textures; every value is a row. [strategy:The catalogue]
- No folds, collars or ridges at junctions; fn-20 owns the geometry. [paraphrase]
- No moss, damage or age-dependent bark change; colour rows may suggest, nothing more. [inferred]
- No new depth mechanism; fn-32's is used as it lands. [paraphrase]

## Decision Context
<!-- scope: both -->

### Motivation

- The owner saw the beech's bark in the round-3 pairs and asked whether a texture spec exists; fn-32 covers plated and scaly bark, and smooth bark with lichen, lenticels and peel is the other half of the catalogue's trunks. [user]

### Implementation Tradeoffs

- Rows on the existing field over a second bark shader: the field already carries mottle, fissure and crest tints and fn-32's cellular partition; lichen is a patch field with a colour, lenticels a dash field with a groove, peel is a plate at band values. Three row sets, one shader. [inferred]
- After fn-32 rather than beside it: both touch the bark shader and the same measurement receipt; landing second avoids two branches rewriting one file. [paraphrase]

## Strategy Alignment

- Follows "Surface and rendering at scale": bark that makes the trunk legible from close views without species-specific paths, judged with measured cost.
- Follows "The catalogue": a species exposed an unsupported appearance and the renderer gains rows, never a branch.

## Resolved via Codebase

- The material rows today (`crates/telperion-core/src/material.rs`): bark colour and roughness, ridge and plate scale, furrow strength, roughness detail, fissure and crest tints and strengths, bark mottle scale and strength, cavity and sky occlusion; no lichen, lenticel or peel term.
- The bark field (`crates/telperion-render/src/shaders/bark.wgsl`): columns, scales, flakes, ridge mean and colour range, all keyed on ridge and plate scale; no dash or patch field.
- fn-32 (`.flow/specs/fn-32-bark-as-plates-and-scales.md`, branch at `9abe16a`, NEEDS_HUMAN for the owner's R6 and R7): adds the cellular plate network, edge lift, per-plate identity, weathering and orientation tints, directional occlusion, a depth mechanism and the measurement receipt.
- The beech's reference observation for B-BARE and B-BASE: "smooth silver-grey bark, elephant-hide on old trunks", "smooth grey bark with lichen spots" (`.flow/evidence/fn34/european-beech/references.json`); the birch's S-BARK: "white, dark lenticels and chevrons".
- The matched base and bark shots exist from fn-36 (`.flow/evidence/fn34/round3/*-B-BASE-compare.json`, `*-S-BARK-compare.json`).

## Requirement coverage

| Requirement | Task |
|---|---|
| R1–R5 | TBD during planning |
