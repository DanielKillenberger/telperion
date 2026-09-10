---
satisfies: [R7]
---
# fn-24-continuous-tree-space-habit-and-leaf-as.2 One outline routine: lobes and section roundness as traits

## Description
Dissolve the element anatomy enum into a lobe count, a lobe depth and a section roundness on the numeric outline path, so the oak blade and the spruce needle are rows and every element in between builds with a level ladder (R7 element half).

**Size:** M
**Files:** `crates/telperion-core/src/foliage/element.rs` (split into outline and section modules under 400 lines each), `crates/telperion-core/src/foliage/levels.rs` (validation only), `crates/telperion-core/src/params.rs` (element rows), `crates/telperion-core/src/presets.rs` (element rows), element tests
**Touches:** [crates/telperion-core/src/foliage/element.rs, crates/telperion-core/src/foliage/outline.rs, crates/telperion-core/src/foliage/levels.rs, crates/telperion-core/src/params.rs, crates/telperion-core/src/presets.rs, crates/telperion-core/tests/**]

### Approach
- The generic outline at `element.rs:120-228` already reads widest point, base fullness, tip sharpness, cup and curl. Add a lobe term to the half-width profile: lobe count sets the number of crests along the margin and lobe depth how far each sinus cuts toward the midrib, with the oak's five-lobe table at `element.rs:364-380` as the calibration target for count 5 and depth 0.7, then delete it.
- Add a section term: section roundness blends the transverse section from the flat strip the blade uses to the four-sided shaft the needle builds at `element.rs:229-300`, then delete the needle routine and `build_anatomy`.
- Validation: axial section count at least twice the lobe count plus two when lobe depth is positive, error naming both fields; connector length rules stay.
- The level ladder in `levels.rs` reads sections and positions only; confirm it builds for count 0 and 8, depth 1 and roundness 1, and add those cases to its tests.
- Wire rows for the three traits in the `fields!` table; remove the `ElementAnatomy` `enum_wire!` row at `params.rs:192-196`. Presets set the traits from the spec's second table at `presets.rs:87,119`.
- Add the element hash step test: stepping each element trait on each shipped preset changes the element hash.

### Investigation targets
**Required** (read before coding):
- `crates/telperion-core/src/foliage/element.rs:29-60,120-228` — parameters and the generic outline path to extend
- `crates/telperion-core/src/foliage/element.rs:229-380` — the lobed and needle routines being replaced and the oak lobe table
- `crates/telperion-core/src/foliage/levels.rs:1-60` — the section-based ladder and its invariants

**Optional** (reference as needed):
- `crates/telperion-core/src/params.rs:181-196` — the enum wire rows to remove

### Key context
- The element's output shape is the mesh contract fn-23 fixed: sections, positions, indices, levels. Only how the sections are computed changes.
- Section count rounds up and lobe count rounds down in a blend so the validity rule survives rounding; note this for task 5.

## Acceptance
- [ ] No `ElementAnatomy` identifier remains; one outline routine builds every element from numeric traits
- [ ] Lobe count, lobe depth and section roundness are validated fields on every family; a section count below the lobe rule is rejected naming both fields; an anatomy tag on input is rejected naming the field
- [ ] Oak and spruce element rows build elements the fn-23 level ladder accepts, with the ladder tests extended to the trait extremes
- [ ] Stepping each element trait on each shipped preset changes the element hash
- [ ] Element modules each under 400 lines; `cargo test --release -p telperion-core` and clippy pass

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
