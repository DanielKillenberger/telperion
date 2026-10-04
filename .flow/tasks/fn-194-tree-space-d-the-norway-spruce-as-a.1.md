---
satisfies: [R1, R2, R3, R4]
---
# fn-194-tree-space-d-the-norway-spruce-as-a.1 The Norway spruce as a point: values, stills, walk and the beech unchanged

## Description
The Norway spruce (*Picea abies*, Massart's model) as a point in the tree space, values only on the engine phase C left, following the beech.

- **Values:** `crates/telperion-space/src/spruce.rs` beside `beech.rs`, exported from `lib.rs`; the rules and values from `MODEL-SPRUCE.md` (fn-190 evidence) and its sources, each source listed in this spec's `SOURCES.md`. Massart: an orthotropic, monopodial, indeterminate trunk whose yearly growth unit ends in a whorl (`Zone::buds`), plagiotropic branches bearing second-order branchlets in one plane (trunk `form.plane`), branch tips turned up (`form.elevation`), branchlets shed (`shedding`).
- **Engine gaps go to the host**, never designed here (AGENTS.md, dispatch): MODEL-SPRUCE.md's F1 to F6 are checked against today's engine and reported.
- **Stills:** `crates/telperion-render/examples/space_spruce.rs`, sharing the beech's still code through `examples/space/`, dressed by the norway-spruce preset's rows with render-matched darker colours as the beech's were. A five-seed sheet at 80 years (seeds 1, 7, 2, 3, 4) and 10/20/40 years at seeds 1 and 7, beside `.flow/references/norway-spruce/`, into `.flow/evidence/fn-194-tree-space-d-the-norway-spruce-as-a/raw/`.
- **Measures:** `crates/telperion-space/examples/beech.rs` generalised or mirrored for height, width, dbh.
- **R3 walk** (beech to spruce at a fixed seed, still strips, `examples/strips.rs`) and **R4** (the beech's passed sheet re-rendered unchanged) run only after the host passes the look.
- **Gate:** the host views the sheet first, then Astra; the owner's verdict passes it (docs/tree-space.md).

## Acceptance
Every R-ID in the parent spec's Acceptance Criteria is satisfied (R1 to R4); judge against the spec directly.

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
