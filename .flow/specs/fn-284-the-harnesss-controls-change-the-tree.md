# The harness's controls change the tree, and read plainly

Release (owner, 2026-10-15: "we need to add a spec to the release to fix the harness sliders. While we're at it we may as well redesign it to be more easily usable and intuitive"). The owner found lean doing nothing on the date palm, height doing nothing on the ordinary tree, and "none of the params do anything".

## Cause (diagnosed 2026-10-15; raw tables in fn-237 evidence raw/diag-params/)

- The harness rebuilds on every change with the new values (code read, 72 harness tests green). The defect is what the panel shows.
- `shownRows()` returns 365 rows. Moving each to its window's far end on the palm, ordinary, oak and birch through `pipeline::build`: 182 change some tree; about 99 are honestly dormant for the species (82 supernatural, 16 bloom); about 80 change no tree at all: `/shellDepth`, `/skeleton/{step, attractors, samplingAttemptsPerAttractor}`, all `/skeleton/habit/*`, `/skeleton/envelope/*`, `/skeleton/bias/*`, `/skeleton/growth/*`, all `/skeleton/twigs/*` but `twig/{diameter, internodeLength, stationsPerInternode}`, `/radii/*` but `trunkRadius`; and the density and torsion adapters. On the palm 197 of 364 change nothing.
- Why: the engine reads only `space.species`, age, seed, light, canopy, `space.trunk` and `supernatural.scale` (`pipeline/space.rs:163`), and the grown envelope replaces the family's (`pipeline.rs:220`). `reach` is computed as `"mature"` unless deprecated or growth-only (`catalogue/browser.rs:121-127`), a column from the removed growth path; `tests/dormancy.rs` is ignored (`:178`, `:201`).
- The engine's `/space` rows are mapped by `presets/space.rs::read` but are not catalogue groups, so `catalogue::browser()` never emits them (`browser.rs:21-23`). Scaling each key by 1.25: on ordinary 33 keys change the tree, 11 leave their bounds, 10 are unread there; on the palm 14, 6 and 34. Their bounds and meanings live in `species/scale.rs::settings` and the engine's `validate`, not the catalogue. The palm's lean is `/space/ages/2/form/{elevation, tropism, wander}`; ordinary's height is age with `internode` and `elongation`.

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

## Owner's verdict (R5, 2026-10-15)

In the harness on the release branch (`f2f8a06bd`): "sliders change the tree yes. Yes settings are clear. groups seem ok to." Accepted. Open: Norway spruce, Telperion and Laurelin were never probed (memory); `clumpSystemOrder` carries dormancy text instead of a probe pass.
