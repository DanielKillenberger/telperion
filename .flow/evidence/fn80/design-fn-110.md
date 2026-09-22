# fn-110 design handoff: the palm's trunk organs

Recorded after the fact by the host (2026-09-22): fn-110 was designed and built by a strong-tier agent the host dispatched directly, outside the conductor, before the conductor opened its own design dispatch for it; this file is the design as landed (PR 62, aa488beb), not a fresh handoff.

## Interfaces

- Eight canopy rows in `CanopyParams` (now `foliage/canopy.rs`): `leaf_bases`, `leaf_base_length`, `leaf_base_radius`, `leaf_base_pitch`, `leaf_base_weathering`, `acanthophylls`, `acanthophyll_length`, `acanthophyll_pitch`; validated in `foliage/rosette.rs` (`validate`, re-exported as `validate_canopy`), each with a doc comment and a dial-table row.
- `branching/leaf_bases.rs`: a retained base is wood hung on its own attach node on the stem's axis and swept as its own run, appended after the radius solve, so no base enters the pipe model; `clothe_leaf_bases` joins `grow` in `mesh.rs`.
- The acanthophyll is the first n leaflets of a frond re-pitched inside `fan` (`foliage/rosette.rs`); no new instance and no new element.
- `persistent-leaf-base` and `acanthophyll` move to the expressed list in `capability.rs` with derivable clauses guarded on the rosette and the pinnate grouping.

## Invariants

- Both organs off by default; every shipped preset byte-identical with both absent; the palm's digest moves by design.
- The bases take their spiral from the rosette's frame, carried onto each local axis by rotation, so the trunk's wander does not turn the phase.
- The lattice's axial resolution comes from splitting the stem polyline, never a finer step distance; persistent-leaf-base requires the rosette; the acanthophyll keeps the leaflet's cross-section (host decisions).

## Verification

- `tests/leaf_bases.rs`: the spiral as arithmetic, exact to 5e-12 degrees on a straight stem; every stem node byte-equal with the organs on and off.
- `tests/capability.rs`: vocabulary and derivation; species digests over four species and `catalogue/*/pins.json` unchanged; a positive control that the palm's digest moves.
- Gate green at 7bf080a2 (120 binaries) and again at the merge f39efc46 (118 binaries, 925 tests).

## Unknowns

- A base's taper is the weathering term, not its own row; a grown specimen carries no bases since the growth path builds its own tree. Neither owed here.
