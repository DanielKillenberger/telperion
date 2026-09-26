## Conversation Evidence

> owner (2026-09-27), after the first motion design cuts (fn-167): "problem is that telperion the tree is currently super ugyl xD"
> owner: "ok it's fine we'll make another video another time but yea a spec to make telperion look good is necessary."
> STRATEGY.md, Key metrics, "The owner's eye": "Telperion reads as Telperion and Laurelin as Laurelin, lit, with every supernatural term and every appearance row at its preset value; judged in the harness, recorded in the spec."

## Goal & Context
<!-- scope: business -->

Telperion is the project's namesake and its first demanding legendary specimen, and today it does not look good. It carried most of fn-167's video and let it down, so that video is shelved until this lands. This spec makes Telperion read as the silver tree, beautiful in the harness and at the video's close range. The owner's eye is the acceptance. [paraphrase]

## Architecture & Data Models
<!-- scope: technical -->

- **What exists.** Telperion is a shipped preset: a value file since fn-160, on a real branching base, with its supernatural character carried by the shared bias field (STRATEGY.md). The species runner's Tune stage (fn-149) tunes a preset against reference photographs with a reviewer. A legendary tree has no photographs. [checked]
- **The target comes first, in the owner's words.** Before any tuning, the owner states what Telperion should look like: silhouette, scale, bark, leaves, light and the supernatural terms. Optionally the owner adds reference images the project may use. The description is recorded in the spec and in the catalogue folder. [inferred]
- **Tuning by verdict notes.** Each round renders Telperion at its standard views. The owner writes verdict notes, Jev maps each note to the rows that answer it and the direction to move them, and code proposes values and renders them (AGENTS.md, TypeSafe: "Tuning presets"). The value that ships is one a render measured. [paraphrase]
- **Gaps.** A trait the generator cannot draw becomes its own spec, classed identity (needed for Telperion to read as Telperion) or global, as for the date palm. [paraphrase]
- **Unknown.** Whether the species runner's Tune stage can run on verdict notes and owner references alone, or needs a small mode for species without photographs; the implementer checks this and proposes. [unknown]

## Acceptance Criteria
<!-- scope: both -->

- **R1:** The owner's description of Telperion's target look is recorded in the spec and the catalogue before tuning starts. [inferred]
- **R2:** Telperion's value file is tuned in rounds of owner verdict notes. Each round's renders, notes, mapped rows and chosen values are recorded. [paraphrase]
- **R3:** Every generator gap found is its own spec, classed identity or global; identity gaps land before the owner's final look. [paraphrase]
- **R4:** The owner judges Telperion in the harness, lit, with every supernatural term and appearance row at its preset value, and approves it; the verdict is recorded in the spec. [paraphrase]
- **R5:** Every other preset is byte-identical, and the workspace gate and `npm test` are green. [inferred]

## Boundaries
<!-- scope: business -->

- Not Laurelin, which gets its own spec. Not the video (fn-167), which waits for this.

## Strategy Alignment

- Serves "Key metrics", the owner's eye, and "The catalogue": Telperion as the first demanding legendary specimen. [strategy:Key metrics]
