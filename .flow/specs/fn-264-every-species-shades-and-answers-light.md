# Every species shades and answers light by its sources

## Goal & Context

COUNCIL.md (fn-245 evidence, 2026-10-11) makes the lattice the one light model for every species, species differing only by their rows. fn-255's EARLY.md measured that today only the oak and the ordinary tree carry any leaf area in the lattice: spruce, birch, beech, palm, Laurelin and Telperion deposit nothing, so under the one cycle they neither shade nor are shaded. The owner (2026-10-09): "i'm still not comfortable with only the oak growing with light". fn-251's oak → birch walk fails partly because the birch has no light rows (fn-251 Decisions). Non-zero rows are not by themselves a pass for that walk (COUNCIL.md).

## Requirements

- **R1 (inventory, checked against code):** every row the lattice and the light response read (leaf area deposited, shade hazard, shade size, balance hazard, leaf girth, and any other), per shipped preset, with its current value and which consumer reads it; the site's environment rows (extinction) named as environment, not species.
- **R1a (one environment, host 2026-10-11):** the site's light (`/space/light/extinction` and any other environment row R1 names) is one shared setting every shipped preset grows under, not a per-preset row: today the six presets omit it and `Space::default()` gives zero, so the lattice skips its sweep (`light/field.rs:177`). Its value is a sourced canopy extinction coefficient (Monsi and Saeki's Beer-Lambert k; the oak's 0.5 checked against its source), set in one place the preset entry points all read, verified by a test that every shipped preset's build reads it non-zero.
- **R2 (sourced, one task per species):** for spruce, birch, beech, palm, Laurelin and Telperion, each row from a literature value at a stated age and growing condition (leaf area index or leaf area per shoot, shade tolerance), screened by Jev and copied by code, each checked against its source text (AGENTS.md "TypeSafe"); the Two Trees take their nearest real analogues' values, named as a decision; a row a source sets to zero stays zero; a row with no source is a stated host decision, never an invented number.
- **R2a (each response proved where it lives):** per shipped preset under controlled site light, a focused test that its foliage lowers sampled light in the lattice, and that each populated response row changes growth through its actual consumer (shade hazard on survival, shade size through normalisation, balance hazard, leaf girth); where a sourced zero or the preset's structure makes a response neutral (for example the palm's single-bud groups under normalisation, or viability at 1), the neutral expectation is stated and tested instead.
- **R3 (proved):** every preset grows through fn-255's cycle with its rows; whole/mid/close stills of each beside F4's, viewed; build time and peak memory per preset; fn-251's exact frame re-measured and reported (pass or not); the owner's verdict on the stills.

## Boundaries

Species rows only; no engine change. Depends on fn-255. Release item (fn-237): the one light model ships with every species reading it.
