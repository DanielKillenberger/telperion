# The harness's controls change the tree, and read plainly

## Goal

The owner (2026-10-15) found lean doing nothing on the date palm, height doing nothing on the ordinary tree, and "none of the params do anything", and asked for the harness's sliders to be fixed and the panel redesigned to be easier to use. Diagnosis: the harness rebuilt correctly, but of 365 shown rows about 80 changed no tree (scaffold rows the engine never reads, plus the density and torsion adapters), `reach` was a column from the removed growth path, and the engine's `/space` rows (where the palm's lean and the ordinary's height live) were not offered at all. Harness and catalogue browser table only; no tree's values or engine change. Release item.

## Decisions

- Owner (2026-10-15): "we need to add a spec to the release to fix the harness sliders. While we're at it we may as well redesign it to be more easily usable and intuitive".
- Host design (R2): one control per engine key moves all of the species' age classes together on the row's declared scale (log multiplies, linear offsets), opening to per-age values; bounds, units and meanings from one table in `telperion-space` (`species/scale/table.rs`) that `validate` also reads.
- Host design (R3): groups by what a control does — size and age; trunk and form; branching; leaves and fronds; flowers and fruit; bark and colour; supernatural; a closed "advanced" group — with plain labels, units, one-line meanings, changed-from-preset marks with resets, and a filter box.
- Deleting the old skeleton rows from the catalogue is out of scope (its own spec if no consumer reads them).

## Requirements

- **R1:** every shown control changes the species' drawn tree or says why it is dormant; unread rows never shown; `reach` derived from what the engine reads; an ordinary probe test per preset — met (80 rows leave the panel, 365 → 285; adapters removed; `tests/dormancy.rs` replaced by the on-demand probe `tests/controls.rs`).
- **R2:** the engine's `/space` rows offered, one control per key on its scale — met.
- **R3:** controls grouped, labelled plainly, marked when changed, resettable, filterable — met.
- **R4:** every change builds at once under the live budget; grow-in and play the year unchanged — met.
- **R5:** the owner's verdict in the harness on two or three species — met.

## Result

Probe (frozen binary): palm 166 live / 115 dormant / 2 refused, ordinary 154/128/1, oak 154/126/3, beech 154/128/1, birch 155/126/2; one unexplained row on each, `/canopy/clumpSystemOrder`, which carries dormancy text instead. Engine-key probe: palm 17 live / 39 dormant, ordinary 49/9, oak 51/7, beech 49/9, birch 52/6; refused by name: `zone.nodes.min` everywhere (min above max), elongation and expansion on the palm (sum over the year). Gate 1,409 passed, 0 failed; `npm test` 179.

Owner verdict (R5, 2026-10-15, release branch `f2f8a06bd`): "sliders change the tree yes. Yes settings are clear. groups seem ok to." Accepted.

## Open

- Norway spruce, Telperion and Laurelin were never probed (memory caps on the desk).
- `clumpSystemOrder` carries dormancy text instead of a probe pass.

## Friction

- A catalogue reference rewrite queued as a heavy job -> a small binary building only `telperion-core` for generated files.
- A 10 GB claim never admitted on the desk -> `telperion-heavy` explains its wait; claims sized to single-crate builds.
- The probe outgrew the admitted memory; three presets outgrew every cap -> per-preset memory estimate, probe sizing its concurrency, younger or row-at-a-time builds.
- The gate could not run on the desk while the owner's dev server and browser held memory -> free desk memory or split the gate by crate.
