## Goal & Context
<!-- scope: business -->

One spruce specimen holds its foliage twice, in 64 bytes per leaf where 12 carry the information. At seed 1 the spruce places 7,353,754 leaf instances, which is 470.6 MB, and `foliage::cull` calls `try_reserve(instances.matrices.len())` at `crates/telperion-core/src/foliage.rs:112` before it filters, so it allocates a second buffer at the full input length and pushes survivors into it. The live peak is 941.3 MB whatever the cull drops, and it drops nothing on the oak (715,065 in and out), the beech (4,928,780) or the spruce (7,353,754). Only the birch drops any, 260,888 down to 226,057. [user]

Measured this session, peak RSS polled from `/proc/<pid>/status` VmHWM with the test binary on the `ci` profile: `fixed_spruces` 4,994 MB, `fixed_beeches` 4,786 MB, `fixed_oaks` 1,672 MB, `fixed_birches` 820 MB. Each test holds `SEEDS_IN_FLIGHT = 4` specimens at once, and at the default harness thread count the four run together, which is 16 specimens and the OOM kill fn-84's FRICTION.md recorded. [user]

The forest target makes the same point from the other end. STRATEGY.md asks for a thousand-tree forest whose vegetation layer renders inside 4 ms on an RTX 3080. A thousand specimens at today's 639 MB each, foliage plus wood, is 639 GB against a 10 GB card. The hero tier of a tiered budget affords about 1 MB of leaf data per tree, which is 87,000 leaves at 12 bytes and 16,000 at 64. [user]

This spec cuts the resident foliage from 941.3 MB to 88.2 MB on the spruce, a factor of 10.7, by removing the second buffer and storing what the transform actually carries. [user]

## Architecture & Data Models
<!-- scope: technical -->

- **`cull` consumes its input and retains in place.** The signature becomes `pub fn cull(instances: Instances, element: &Element, envelope: Envelope, shell_depth: f64) -> Result<Instances>` and the body keeps `Vec::retain` over the buffer it was handed. No second allocation exists at any point. Capacity is not shrunk. The call sites that report `pre_cull_instances` read the count into a local before culling, and the test call sites that assert `cull(&a, ..) == a` clone their small fixture. [inferred]
- **Every leaf transform is a similarity, which is why 12 bytes is enough.** `foliage/station.rs:178` is the single constructor, reached by `short_shoots.rs:144` too. It builds `face` by Gram-Schmidt against `axis`, sets `side = axis x face`, and multiplies all three by one scalar `scale`. Three orthonormal right-handed basis vectors and a uniform scale is a rotation, a scale factor and a translation: 3 + 3 + 1 numbers, stored today as 16 floats of which 4 are the constants `Instances::validate` already enforces. [paraphrase]
- **The stored leaf is three `u32` words, 12 bytes, the same layout on the CPU and the GPU.** Word 0 is the rotation as a smallest-three quaternion: 2 bits naming the dropped largest component, 10 bits each for the other three over the range plus or minus one over root two. Word 1 is the x and y position as `u16` unorm over the reference box. Word 2 is the z position as `u16` unorm in its low half and the scale as `f16` in its high half. WGSL reads word 1 and the low half of word 2 with `unpack2x16unorm` and the scale with `unpack2x16float`. [inferred]
- **The reference box is computed before the first leaf is placed and travels with the instances.** Every translation is a station point that lies on the wood or projected onto its surface, so the box is the tree's node AABB expanded by the largest node radius, held per axis as a min and an extent. `Instances` carries it, and it is what a reader dequantizes against. A station outside the box cannot occur by construction, and a debug assertion says so. [inferred]
- **The error each approximation introduces, against a leaf that is 70 mm long on the beech.** Position: 16 bits over the spruce's 32 m axis is a 0.49 mm step and at most 0.24 mm of error. Rotation: a smallest-three quaternion at 10 bits per component is about 0.001 rad, which is 0.06 degrees. Scale: `f16` near 1.0 is a relative error of about 5e-4, which is 35 micrometres on that leaf. All three sit far below one leaf's own size, and none of them is a measurement the numeric protocol reads. [inferred]
- **The shaders compose the matrix instead of reading it.** `crates/telperion-render/src/select.rs:179` uploads the core vector through `bytemuck::cast_slice`, so the storage buffer is the bytes the generator wrote. `shaders/foliage.wgsl:10` and `shaders/select.wgsl:29` bind `array<mat4x4<f32>>` and become `array<u32>` read three words at a time. `foliage.wgsl` rebuilds the rotation for the position and the normal. `select.wgsl:70` currently derives the scale as the longest column length and reads the scale word directly instead, which is fewer instructions than it runs today. [paraphrase]
- **The committed digests move once.** `tests/species.rs:33` hashes every stored float, so the digest changes for every species. `crates/telperion-core/tests/species/digests.json` is recommitted in the same commit, and the test already names both values when one moves. This is a change detector rather than a contract, and fn-53's `--pin-note` covers preset value tables in the species pipeline, not these. [user]

## Acceptance Criteria
<!-- scope: both -->

- **R1:** `cull` consumes its `Instances` and allocates no second leaf buffer. A test records the vector's pointer before the call and asserts the returned vector carries the same one. Errors: a cull that drops every leaf still returns the original allocation, empty.
- **R2:** One leaf occupies 12 bytes on the CPU and 12 bytes in the GPU storage buffer, asserted by `size_of` and by the buffer's byte length for a known instance count.
- **R3:** Rebuilding the transform from the stored words reproduces the pre-quantization transform within the stated bounds: 0.25 mm on each position component, 0.002 rad on the rotation, and 1e-3 relative on the scale. A test drives the round trip over the shipped presets' placements rather than constructed cases.
- **R4:** The five catalogue species pass `npm run species:qa` with their numeric gates unchanged, and the retained leaf count moves by less than 0.1 percent on every species, which bounds the cull decisions that quantization can flip at the shell boundary.
- **R5:** `crates/telperion-core/tests/species/digests.json` is recommitted in the commit that changes the encoding, and the spec's evidence names the old and the new digest for one seed per species.
- **R6:** `transform_point`, `Instances::bounds`, `clumping::at`, `mass::grid`, `timeline::Placement` and the two WGSL shaders read through the new representation, each covered by a test that fails against the old one.
- **R7:** Peak RSS of `fixed_spruces` falls below 1,800 MB from the recorded 4,994 MB, measured by the same VmHWM poll on the `ci` profile, and the number for all four `fixed_*` tests is recorded in the evidence.
- **R8:** One headless still per catalogue species is captured on the commit that lands the renderer change, for the owner's eye, and no still is captured before the numeric gates pass.

## Boundaries
<!-- scope: business -->

- What the generator draws does not change beyond the stated error bounds. No generator parameter moves, no preset value moves, and no preset pin in the species pipeline moves.
- Cluster cards, a detail level parameter, LOD tiers, impostors and streaming placement are not this spec. They are what the forest target needs next, and each is its own spec.
- The `rust:test` thread cap and `SEEDS_IN_FLIGHT` are not this spec. They bound how many specimens run at once, which is a different question from what one specimen holds.
- The growth path's `timeline::Placement` takes the same representation, stays buildable and pinned, and is not otherwise touched.
- The wood mesh, 168.3 MB of the spruce's 639 MB, is not this spec.

## Decision Context

- The owner chose on 2026-09-19 to go straight to the 12-byte quantized form rather than land a 32-byte similarity first, because the same six reading sites and two shaders would otherwise be rewritten twice. [user]
- The owner stated on 2026-09-19 that byte identity is not a requirement while the generator is in development, and that bytes are expected to move as quality and performance improve. The committed digests are a change detector, and a deliberate change recommits them. [user]
- The row-oriented three-word layout is chosen over a float matrix because WGSL pads a `vec3` column to 16 bytes, so a four-column `mat4x3` would still occupy 64 bytes on the GPU and return nothing for the change. [inferred]
- The measurements in this spec were taken on 2026-09-19 on the `ci` profile, with peak RSS polled from `/proc/<pid>/status` VmHWM and instance counts read from `species_measure`. The method is named so the closing numbers are taken the same way. [user]
