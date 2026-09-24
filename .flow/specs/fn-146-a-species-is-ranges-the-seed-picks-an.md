# A species is ranges: the seed picks an individual, and only live settings are tuned

## Conversation Evidence

> owner (2026-09-24, looking at the tuned date palm in the harness): "one thing i'm noticing is that seeds have very little variation. Mature trees vary in shape and sizes no? how do we manage that properly?"
> owner (2026-09-24): "no we can write the spec and finish the palm."
> owner (2026-09-24): "i'm noticing that most params don't do anything for the palm tree.." and "many trees can't have 2 stems.."

## Goal & Context
<!-- scope: business -->

Mature trees of one species differ in height, crown width, girth, lean and crown fullness; a date palm runs from about 15 to 30 m. Today a seed moves only fine structure (branch angles, twig vigour, leaf size), and every whole-tree trait is one value per species, so a palm, whose rosette has little fine structure, draws nearly the same tree at every seed. The species' literature already records ranges and throws the spread away. This spec lets the seed pick a reproducible individual from the species' stated ranges, as STRATEGY.md says: "the seed selects a reproducible specimen". It is a generic capability for every species, never a palm branch. [paraphrase]

## Architecture & Data Models
<!-- scope: technical -->

- **What exists, checked 2026-09-24 on the fn-80 branch.** `envelope.rs` holds `height` as one value (default 24.0); the family's per-seed variation rows are fine-structure ones (`pitchVariation`, twig `angleVariation`, `vigourVariation`, `pendulousVariation`, canopy `sizeVariation`). No row samples a whole-tree trait per seed. [checked]
- **Shape.** [inferred]
  - A small set of whole-tree traits (height, crown spread, trunk girth, lean, crown fullness such as frond count or crown base) each gain a spread row beside their value; the seed draws the individual's values from a seeded stream of its own, so fine-structure randomness is unchanged.
  - Traits that grow together move together: one draw of "vigour" or "age within maturity" scales height, girth and bare-trunk length together, with independent residuals, rather than each trait drawing alone.
  - Spread zero reproduces today's tree byte for byte for every shipped preset.
  - The literature step's ranges fill the spreads the way its midpoints fill the values today (fn-135's derivation).
- **Live and locked settings.** A setting is live for a species when moving it across its range measurably changes that species' tree; code measures this with a sweep (the host's 2026-09-24 renders found side-branch order inert on the palm, starved by an envelope spread of 0.1). A species locks the traits it must never vary (the palm's single stem, no laterals) with zero spread. Which rows a closed gate silences, and how the harness and tuning show that, is fn-148's; this spec owns the ranges and the locks. [inferred]
- **Unknown.** Which traits need the shared draw and which draw alone; the implementer measures on the palm and the oak and proposes, the host decides. [unknown]

## Acceptance Criteria
<!-- scope: both -->

- **R1:** With spreads at zero every shipped preset is byte-identical. [inferred]
- **R2:** With the palm's spreads set from its literature ranges, 20 seeds give heights covering the sourced 15–30 m band with no individual outside it, and girth rises with height across them. [inferred]
- **R3:** A seed's individual is reproducible, and changing a spread does not change the fine structure a seed draws. [inferred]
- **R4:** A locked trait never moves with the seed or in tuning.
- **R5:** Rows in the dial table; the workspace gate is green; six seeds of the palm in one contact still, judged by the owner's eye.

## Boundaries
<!-- scope: business -->

- Not age or growth over time (the growth path stays hidden). Not the tuning loop's judging of a population, which follows once this lands. Waits for the loop's deletion pass (fn-80, SIMPLIFY.md).

## Strategy Alignment

- Serves "Our approach": the seed selects a reproducible specimen. [strategy:Our approach]
