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
A leaf or a flower that would cross the ground now lies on it, on both executors. There is one rest function on each: `foliage/rest.rs` on the CPU and `generation/rest.wgsl` on the device. Each takes the up direction nearest world up that keeps every corner of the element's wedge clear, and turns the instance about its attachment, about a horizontal axis, until that direction points up. The candidates are up itself, each corner's own bound, and each pair's two intersections, and the highest one that clears wins. Translation and scale never change. An instance that is already clear is returned untouched, and it is screened by its reach before any corner is read.

- **Leaves.** Under the host's stalk ruling, a leaf's base corners (its stalk, its seat on the wood) may dip as far as `max(h, depth·scale)` below the attachment. Every other corner clears the ground by the encoder's margin: half a position code, plus the half float's 2^-11, plus the 10-bit angle's 3/1023, the last two times the reach. A leaf rests where `rosette::fan` stores it, and the device mirrors that in `pack.wgsl`'s `packed`. `place_on` now takes the element (`drawn.rs`), and the wedge reaches the device in two new config words.
- **Flowers.** An inflorescence is turned as one about its seat (`bloom/ground.rs`, `place.wgsl` `inflorescence_rest`). The corners are the box of its axis samples, each floret's organ corners, and each pedicel's two ends with the stamen's half-width tube around them. Where no turn clears, the best effort maximises the lowest corner. The stalk reach screens first. `stalk.wgsl` keeps the rebuilt rachis vertices at `y = max(y, 0)` in tree space.
- **Field boxes.** The plan (`plan/fronds.rs` `rested`) and the device field (`field.wgsl` `blade`) box the frame each leaflet rests in. With no stored box there, they rest it with a zero position step, and the plan's 1 mm packing margin covers the difference.
- **ground_reach.** No vertex may lie below `min(0, h − depth·scale) − BELOW`, and the largest stalk dip is reported per row. The flower rachis is measured from the CPU mirror of the rebuilt rachis (`foliage::rachis_low`, from `Axis::laid`), clamped as `stalk.wgsl` draws it. Pedicels are read as `pedicel_vertex` rebuilds them.

Measured:
- **ground_reach:** all 168 rows green (raw/t4-ground-reach.log, t4-ground-reach.tsv). The largest stalk dip is 9.3 mm, on the palm at 12; the palm at 10 to 20 dips 7 to 9.3 mm, and the birch and the ordinary tree dip at most 1.3 mm.
- **R3:** the beech, Telperion and Laurelin at seeds 1, 7 and 42 are byte-identical to base d600fa12c (`artifact_digest`; raw/t4-digest-base.txt against raw/t4-digest-change.txt).
- **Palm at 12, device:** `every_rosette_is_placed_as_the_cpu_places_it` passes. The new field test's foliage Jaccard is 0.86, 0.82 and 0.78 on 24, 36 and 48 cells, with a tolerance of 0.70 (raw/t4-young-palm-field.log).

Tests:
- `rest/tests.rs`: the wedge bounds its element, a clear leaf is bit-identical, a hanging leaf rests by no more than lying flat with translation and scale kept and its lowest corner at the margin, a leaf on the finest wood lets only its stalk down, and the best effort.
- `packed/tests.rs` `a_rested_leaf_decodes_within_the_margin_it_was_rested_by`: the packing round trip at 0.2 mm and above.
- `bloom/ground/tests.rs`: a low raceme rests as one, a clear one keeps every bit, and a thick pedicel rests with its floret. The last ran red without the tube.
- `suite/field_fronds.rs` `a_rested_leaflet_lies_in_its_planned_box`: the palm at 12, CPU.
- `resident/field_tests.rs` `the_young_palm_s_rested_leaflets_are_boxed_as_the_cpu_boxes_them`: device.

Decisions:
- **Rulings applied.** The host's .4 rulings were applied: the wedge bounds, the confirmed solve, best effort for flowers, the flower box, the encoder margin, and the stalk dip (bc9d7c58d, 67c889e46). The stalk dip is applied per corner: base corners against `max(h, depth·scale)`, the rest against `h − margin`. A uniform bound let the blade's corners dip too, and the round-trip test caught that, which contradicts "covers only its stalk".
- **Edits outside the original Touches, approved by the host:** `pipeline/drawn.rs` and the `mod` line in `pipeline/foliage.rs`. `generation/rest.wgsl` is the new device file, beside `rachis.wgsl`.
- **Edits outside Touches that are not yet host-approved**, forced plumbing in the same class as .2's and .3's:
  - `pipeline/contract.rs`: exports `Wedge` for the device config and `rachis_low` for ground_reach.
  - `pipeline/foliage.rs`: the `pub use` lines for those two items.
  - `docs/generation-limits-inventory.json`: 12 classification-only entries for the rest's sites, which `generation_limit_guard` asked for.
  - `resident/florets.rs`: the tube radius; it falls under `resident/**`.
- **Test fixture.** `huge_phases_fall_back...` stood its stations on the ground; the fixture was lifted, not the assertion loosened.
- **Plan boxes.** The plan's and the device field's rest use a zero position step, because there is no stored box at plan time.

Open:
- The device's inflorescence rest copies `Axis::laid`'s eight-segment loop into `place.wgsl`, because `bloom.wgsl` is outside the Touches. A shared `bloom_axis` would delete the copy.
- The device flower rest has no direct GPU test. No shipped bloom comes within 0.44 m of the ground, and a low bloom on the device needs a grown tree.
- The device field test checks the field's agreement with the CPU's, not per-leaflet containment, which the device does not expose.

Gate: `telperion-heavy 8 --gpu 2 --threads 2 -- cargo test --profile ci --workspace --no-fail-fast` ran once, at a472f0c2d (raw/t4-gate.log): 1,453 passed, 1 failed, 113 ignored. The one failure was `generation::tests::huge_phases_fall_back_but_supported_large_ordinals_keep_orientation`. Its fixture's first segment stands at y = 0, so its level leaves now rest and turn off the radial the test measures. The fixture and its box were lifted one metre with the assertion unchanged (e3e834ca2), and the focused rerun passes (raw/t4-huge-phases.log). `generation_limit_guard` passes (raw/t4-guard-fix.log). Baseline: green via handoff (5e7364527, from .3).

Tier: implementer (session model) (model: claude-opus-5-5)

stage: impl-review - ran (codex: round 1 three lenses, NEEDS_WORK on one P1 (pedicel tube omitted from the flower box), fixed in a472f0c2d; re-review SHIP)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: cf13202b1e794a0b302ac6bfb0820e1516f52a28, bc9d7c58d7c0195fba4fffd0c43c553c40fa7c3f, 839159395782784e4159cafadd568f10ed3befce, 67c889e461e72a37d3ebba18fe6686dee964bedd, 6b54447d42edda5b2ff242ea9bdb0cc7b0ef9d46, 42924fd8de906e7aef6182ba8742f9087786f06f, a472f0c2db0a19aee8402110a3a9a9dbf3d87694, e3e834ca2a97578f8422e3142f5bd0ce142ea6da
- Tests: telperion-heavy 8 --gpu 2 --threads 2 -- cargo test --profile ci --workspace --no-fail-fast (1453 passed, 1 failed then fixed, 113 ignored), cargo test --profile ci -p telperion-render --lib -- huge_phases, cargo test --profile ci -p telperion-core --test ground_reach --test generation_limit_guard -- --include-ignored, cargo test --profile ci -p telperion-core --lib -- rest raceme_seated a_rested thick_pedicel
- PRs: