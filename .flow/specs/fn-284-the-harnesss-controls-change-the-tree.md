# The harness's controls change the tree, and read plainly

Release (owner, 2026-10-15: "we need to add a spec to the release to fix the harness sliders. While we're at it we may as well redesign it to be more easily usable and intuitive"). The owner found lean doing nothing on the date palm, height doing nothing on the ordinary tree, and "none of the params do anything".

## Cause (checked so far; the diagnosis in flight completes it)

- The harness shows one control per catalogue row with `reach: "mature"` (`harness/rows.ts` `shownRows`): skeleton 76, supernatural 83, material 111, canopy 41, bloom 20, element 14, surface 10, radii 8.
- `/skeleton/bias/lean` is read only by `GrowthBias` in the old branching path (`pipeline/bias.rs`); the tree-space engine, which grows every shipped species, never reads it. The skeleton rows are suspected dead the same way; unconfirmed per row.
- The engine's own rows, `/space/...` (ordinary 714 values, oak 388, palm 18; per axis age: internode, divergence, insertion, elongation, apical control, tropism, sag, lifespan and more), are absent from `src/browser/parameters.generated.ts`, so the harness offers none of them.
- Whether the harness also drops or delays a change is under diagnosis.

## Requirements

- **R1 (owner):** every control the harness shows changes the drawn tree of the species on screen, or says beside it that it is dormant for this species and why. A row no shipped tree reads is never shown. An ordinary test builds every shipped preset, moves each shown control within its window, and asserts the tree changes or the row is declared dormant for that preset. A row's `reach` is derived from what the engine reads, never authored by hand.
- **R2 (owner, host design):** the engine's `/space` rows are offered. One control per key moves all of the species' age classes together on the row's declared scale (`species::scale`: a log-scale row multiplies, a linear row offsets), and opens to the per-age values for finer work.
- **R3 (owner: "more easily usable and intuitive"; host design):**
  - Controls are grouped by what they do to the tree, most telling first: size and age; trunk and form; branching; leaves and fronds; flowers and fruit; bark and colour; supernatural (dormant unless the species has it).
  - Each control has a plain label with units and a one-line meaning from the catalogue.
  - Each control and group shows when it differs from the species' preset, with a reset per control, per group and for the whole tree.
  - A filter box finds a control by name or meaning.
  - Engine-internal rows (sampler tries, level-of-detail and similar) sit in a closed "advanced" group.
- **R4:** every change still builds at once under the existing live budget (fn-270); grow-in and play the year are unchanged.
- **R5:** the owner's verdict in the harness on two or three species.

## Boundaries

- No change to any tree's values or the engine; the harness and the catalogue's browser table only.
- Deleting the old skeleton rows from the catalogue is out of scope unless no consumer reads them; if none does, that is its own spec.
