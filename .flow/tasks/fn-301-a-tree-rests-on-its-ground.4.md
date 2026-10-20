---
satisfies: [R2, R4]
---
# fn-301-a-tree-rests-on-its-ground.4 A leaf or flower that would cross the ground lies on it

## Description
One pose function per executor, applied where each leaf and flower matrix is finished. An instance whose element bounds would cross the ground under its matrix is turned about its attachment point by the least rotation that brings those bounds onto the ground. It is never culled, scaled or translated.

**Size:** M
**Files:** a new CPU module beside `crates/telperion-core/src/pipeline/foliage/station.rs`; callers `station.rs:244`, `rosette.rs:380,399`, `bloom.rs:384`; device `generation/pack.wgsl:229-248` `packed`, `generation/place.wgsl:109-145`, the config the device uploads
**Touches:** [crates/telperion-render/src/shaders/stalk.wgsl, crates/telperion-core/tests/ground_reach.rs, crates/telperion-core/src/pipeline/foliage/**, crates/telperion-core/src/suite/**, crates/telperion-render/src/generation/pack.wgsl, crates/telperion-render/src/generation/place.wgsl, crates/telperion-render/src/generation/*.rs, crates/telperion-render/src/generation/resident/**]

### Approach
- Gate first: the element's local box (`Element::positions` min/max; `plan/fronds.rs:92-120` `Shape` already computes one) under the finished matrix. If its lowest corner is above the ground, the matrix is returned untouched, so R3 holds bit for bit.
- Otherwise, find the least rotation about the attachment point (the matrix's translation) that lifts the box's lowest corner to the ground. Lying flat always qualifies (spec R2). Apply it to the linear part only. Translation and scale stay.
- The rotation axis and angle are a closed form of the box's corners and the matrix. Name the axis convention (y up in the core). Write it once on the CPU and mirror it in WGSL in the same order of operations.
- Device: the element's local box goes into the config `pack.wgsl` reads (its `shape`/`lean`/`size` live there today), and into the bloom config for florets. Flowers use their own element's box.
- Quantisation (spec, "Quantisation"): solve the pose on the exact matrix. Compute the encoder's worst-case angle from `packed.rs:175-240` (10-bit components). Add a round-trip test that packs and decodes a rested leaf attached at the finest wood (0.2 mm) and holds each vertex within that angle times its distance from the attachment. Update `ground_reach`'s leaf and flower measure to the same per-vertex bound.
- Flowers (spec, "Flowers", host ruling D): pose an inflorescence's florets and pedicels as one about its seat. The points are the bloom axis's samples (`bloom.rs:353-382` `Axis::laid`/`at`), the florets' element corners and the pedicels' ends. Screen with `bloom.rs:240` `stalk_reach` first. In `shaders/stalk.wgsl` `rachis_vertex` (`:17-40`), clamp the rebuilt rachis vertex `y = max(y, 0)`. `foliage.wgsl:91` uses it in tree space, since `placement` is decoded against the leaf box and only `view_projection` follows. Leave the droop toward −Y unchanged. Add a unit test with an inflorescence seated low enough to rest. `ground_reach` measures the stalk from `Axis::laid` (centreline minus the stalk's radius); record that this mirror was used.
- Field boxes: the CPU leaf plan (`plan/fronds.rs:120-131`) and the device field (`resident/field.wgsl:375-382`) build leaflet boxes from the drawn frame. Both take the posed frame. Test that a rested leaflet sits inside its box on both executors.
- The quantisation box is unaffected, because the translation is unchanged (`packed.rs:112-128`).
- Unit tests: an instance clear of the ground returned bit-identical; one crossing it rested with its lowest vertex at the ground (within `BELOW`); translation and scale unchanged; the angle no larger than lying flat.

### Investigation targets
**Required:**
- `crates/telperion-core/src/pipeline/foliage/station.rs:230-280`
- `crates/telperion-core/src/pipeline/foliage/rosette.rs:369-411`
- `crates/telperion-core/src/pipeline/foliage/bloom.rs:330-410`
- `crates/telperion-render/src/generation/pack.wgsl:120-250`, `generation/place.wgsl:100-150`
**Optional:**
- `crates/telperion-core/src/pipeline/foliage/packed.rs:100-130`, `foliage/bounds_tests.rs`

### Key context
The device's leaves must agree with the CPU's within `agree_tests`' scatter-turn bound (`agree_tests.rs:290`). A rested leaf's turn is part of that.
## Acceptance
- [ ] One pose function per executor rests every leaf and flower whose bounds would cross the ground; none is culled, scaled or translated.
- [ ] Unit tests: clear instance bit-identical, crossing instance rested, translation and scale kept, angle at most lying flat.
- [ ] A packing round-trip test: a rested leaf attached at the finest wood decodes within the encoder's per-vertex bound.
- [ ] An inflorescence's florets and pedicels rest as one; the shader's rebuilt rachis is clamped to the ground, an identity on clear stalks (Telperion and Laurelin bytes unchanged); the stalk is measured from `Axis::laid`; the field's leaflet boxes contain the posed leaflets on both executors.
- [ ] The palm at 12: `agree_tests` and `field_tests` pass.
- [ ] `ground_reach`: no leaf or flower below the ground on any row.
## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
