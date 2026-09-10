---
satisfies: [R7]
---
# fn-24-continuous-tree-space-habit-and-leaf-as.3 One placement path: lean and contact as traits

## Description
Dissolve the attachment enum into forward lean, lean rise and surface contact, and collapse the generic and twig-run placement paths into one, so every leaf of every tree is leaned and seated from numbers (R7 attachment half).

**Size:** M
**Files:** `crates/telperion-core/src/foliage/placement.rs` (split into placement and station modules under 400 lines each), `crates/telperion-core/src/surface.rs` (contact surface weighting), `crates/telperion-core/src/params.rs` (canopy rows), `crates/telperion-core/src/presets.rs` (canopy rows), placement tests
**Touches:** [crates/telperion-core/src/foliage/placement.rs, crates/telperion-core/src/foliage/station.rs, crates/telperion-core/src/surface.rs, crates/telperion-core/src/params.rs, crates/telperion-core/src/presets.rs, crates/telperion-core/tests/**]

### Approach
- Replace the three lean formulas at `placement.rs:302-314` with one: axis = radial + tangent times forward lean + tangent times lean rise times the upward part of the radial, plus the existing outward and upward terms, normalised. Oak row 0.25/0, spruce row 0.05/1.2, ordinary 0/0.
- Make the contact surface a weight: the station sits on the shoot axis at 0 and on the wood's contact surface at 1, blending the point the `AttachmentSurface` (`placement.rs:91-96`) returns with the axis station; the surface is built whenever the weight is positive.
- Collapse the paths: the generic branch at `placement.rs:129,154` and the one-station-per-internode guard go; placement always walks the twig runs the twig layer marks (`twig_runs` at line 155), with stations per internode staying a numeric twig parameter.
- Wire rows for the three traits in the `fields!` table; remove the `Attachment` `enum_wire!` row at `params.rs:192-196`. Presets set the traits at `presets.rs:95,128`.
- Add the placement hash step test: stepping each attachment trait on each shipped preset changes the placement hash, and the instance count stays inside each preset's fidelity band.

### Investigation targets
**Required** (read before coding):
- `crates/telperion-core/src/foliage/placement.rs:20-60,85-170` — parameters, the path fork and the twig-run loop
- `crates/telperion-core/src/foliage/placement.rs:260-340` — station frame and the lean match
- `crates/telperion-core/src/surface.rs` — the contact surface the needle path uses today

**Optional** (reference as needed):
- `crates/telperion-core/tests/growth.rs:50` — the attachment and taper test to keep meaningful

### Key context
- The placement record the renderer reads is unchanged; only where each station lands and how it leans changes.
- If a real geometric reason for the one-station guard surfaces, state it in the done summary and keep the guard as a numeric validation rather than a mode.

## Acceptance
- [ ] No `Attachment` enum identifier remains; one placement path leans and seats every leaf from numeric traits
- [ ] Forward lean, lean rise and surface contact are validated fields on every family; an attachment tag on input is rejected naming the field
- [ ] Oak and spruce canopy rows place leaf counts inside their fidelity bands
- [ ] Stepping each attachment trait on each shipped preset changes the placement hash
- [ ] Placement modules each under 400 lines; `cargo test --release -p telperion-core` and clippy pass

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
