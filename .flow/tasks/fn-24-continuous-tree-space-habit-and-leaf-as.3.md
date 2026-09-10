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
The attachment enum is gone. Forward lean, lean rise and surface contact are
validated numeric fields on every family, one lean formula seats and leans every
leaf of every tree, and placement walks one path over the runs the twig layer
marks. The oak and the spruce place every leaf exactly where they did.

stage: impl-review - skipped(policy: parallel wave - conductor owns the gate)

### The three canopy rows

| Preset | forward lean | lean rise | surface contact | shoot radius |
|---|---|---|---|---|
| Oregon white oak | 0.25 | 0 | 0 | 0.0 (was 0.12, inert) |
| Norway spruce | 0.05 | 1.2 | 1.0 | 0.025 (unchanged) |
| Ordinary | 0 | 0 | 0 | 0.0 (was 0.12, inert) |
| Telperion | 0 | 0 | 0 | 0.0 (was 0.1, inert) |
| Laurelin | 0 | 0 | 0 | 0.0 (was 0.14, inert) |

The oak and spruce rows also set `outward` and `upward` to 0, which the two
retired formulas ignored and the generic one did not. That is the whole of what
the enum was carrying.

### The numeric check of the old lean axes

The conductor asked for a check at a handful of stations. `tests/identity.rs`
gives a stronger one: it hashes every retained instance matrix of both species,
and **every pin in the file is unchanged and nothing was re-recorded** — oak
placement `14162788206097926965`, spruce `6468486710499557939`, oak instances
`869310`, spruce instances `7012326`, wood counts, bounds, skeleton and element
hashes all as task 2 left them. So the oak row reproduces the retired
`Alternate` axis and the spruce row the retired `RadialNeedles` axis at every
station of both trees, not at a sample of them, and both leaf counts stay in the
bands `species_metrics::compare` gates (the whole `tests/species.rs` suite is
green).

Three things made that exact rather than approximate. The oak and spruce already
walked the twig-run path, so the collapse moved only the three colonizing
presets. `radial + tangent * (forward_lean + lean_rise * radial.y.max(0.))` is
the two retired expressions written once, and `outward.normalized() * 0.0` and
`axis.y += 0.0` are exact. The contact blend is written
`axis * (1 - w) + seat * w` rather than a `lerp`, so `w = 1` returns the seat
itself rather than `axis + (seat - axis)`.

### What the collapse actually removed

- The per-twig-node generic loop. Placement always walks `bearing_runs`, so the
  three colonizing presets now get chained runs: one `ceil` of stations per run
  instead of one per internode segment, phyllotaxis that continues along the
  whole run, and the segment tangent recomputed at every station. Their
  placements move; none of them is pinned.
- The one-station-per-internode guard. **No geometric reason for it surfaced.**
  It gated the enum, not the geometry: the divergence turn already spaces
  several stations at one internode by `TAU / stations`, and the contact
  projection is per-station and indifferent to how many share a point.
  `tests/foliage.rs` now asserts a leaf that both leans and seats takes two
  stations to an internode. The guard is gone, not kept as a validation.
- The `Attachment` arm inside the bearing predicate, which was the last species
  branch in the file. `shoot_radius` is now the one threshold for both paths:
  wood at or below that fraction of the root radius bears foliage of its own,
  beside whatever the twig layer marked. Its default drops from 0.12 to 0,
  because "only where the twig layer marks" is what four of the five presets
  want and what the default family wants; the spruce keeps its 0.025 and is the
  only preset that clothes supports. On the twig path that value was inert for
  every other preset, which is why nothing moved.

### Files

`placement.rs` 518 lines becomes `placement.rs` 283 (params, validation, the two
run-selection walks) and `foliage/station.rs` 278 (one station's distance, seat,
axis and frame). `place_run` takes a `Run` struct rather than eight arguments,
so the `too_many_arguments` allow is gone with it.

`params.rs` carries `canopy.forwardLean`, `canopy.leanRise`,
`canopy.surfaceContact` as flat rows and refuses an unknown key under `canopy`
as `unknown canopy trait`. The `enum_wire!` macro is deleted: **no field of a
family is an enum any more.**

### Tests

- `every_attachment_trait_moves_every_shipped_preset` (new): each of the three
  traits stepped on each of the five presets moves the placement hash. It goes
  through `place_on_surface`, or the contact weight would be unobservable. Run
  red first with the lean term and the contact blend removed: it failed on
  Ordinary's forward lean with identical hashes, which is the reason it exists.
- The wire test rejects `canopy.attachment` by name and refuses each trait
  outside its range by its own name — forward lean, lean rise, surface contact.
- Four attachment tests are rewritten in trait terms;
  `evergreen_needles_clothe_slender_supports_but_not_thick_limbs` becomes
  `shoot_radius_alone_clothes_slender_supports_and_spares_thick_limbs`, which is
  the same behaviour asserted of the number rather than of the enum arm.
- `frozen_parameters_resolve_without_default_substitution` now strips
  `canopy.attachment` from the frozen fn-19 file beside `skeleton.habit` and
  `element.anatomy`.

### For the conductor

The TypeScript surface still speaks `canopy.attachment` and is untouched, as
instructed: `src/browser/presets.generated.ts` and `tests/browser/bindings.mjs`.
That file's `stationsPerInternode = 2` mutation expects the retired guard's
refusal and now needs replacing — noted for task 4 in
`task3-integration-notes.md` beside the run notes.

### Integration and the host review (conductor, 2026-09-10)

Cherry-picked onto the spec branch as 83d06a4; the workspace commit bf1fb9f is retired with the worktree. Host review: one lean formula, `radial + tangent * (forward_lean + lean_rise * max(radial.y, 0))` plus the outward and upward terms, and the contact seat as a weighted blend, exactly the spec's contract; placement and station modules at 283 and 278 lines; every identity pin unchanged, so the oak and spruce leaves sit where they did. The `enum_wire!` macro is deleted with the last enum row, which closes R1's and R7's wire halves ahead of task 4. Nothing was written outside the workspace.

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 83d06a4
- Tests: worker baseline: cargo test --release --workspace green before any edit, worker: cargo test --release --workspace - 29 suites green, worker: cargo clippy --workspace --all-targets -- -D warnings - clean, worker: cargo fmt --all -- --check - clean, conductor, integrated target 83d06a4: cargo test --release --workspace - green, rc=0
- PRs: