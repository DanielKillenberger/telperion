## Goal & Context
<!-- scope: business -->

A leaf transform is stored in 64 bytes and carries 12 bytes of information. The spruce at seed 1 places 7,353,754 leaves, which is 470.6 MB of the 639 MB that specimen holds, and fn-85 has already taken the second copy of it away. [user]

`station::matrix` is the single constructor of every leaf transform, reached by `short_shoots.rs:144` too. It builds `face` by Gram-Schmidt against `axis`, sets `side = axis x face`, and multiplies all three by one scalar. Three orthonormal right-handed basis vectors and a uniform scale is a rotation, a scale factor and a translation: 3 + 3 + 1 numbers, written as 16 floats of which 4 are the constants `Instances::validate` already enforces. [paraphrase]

The forest target is the reason the remaining factor matters. STRATEGY.md asks for a thousand-tree forest whose vegetation layer renders inside 4 ms on an RTX 3080. A thousand specimens at 639 MB each is 639 GB against a 10 GB card, and the hero tier of a tiered budget affords about 1 MB of leaf data per tree, which is 87,000 leaves at 12 bytes and 16,000 at 64. [user]

Measured after fn-85, peak RSS polled from `/proc/<pid>/status` VmHWM with one test per process on the `ci` profile: `fixed_spruces` 3,244 MB, `fixed_beeches` 3,945 MB, `fixed_oaks` 1,469 MB, `fixed_birches` 778 MB. [user]

## Architecture & Data Models
<!-- scope: technical -->

- **The stored leaf is three `u32` words, 12 bytes, the same layout on the CPU and the GPU.** Word 0 is the rotation as a smallest-three quaternion: 2 bits naming the dropped largest component, 10 bits each for the other three over the range plus or minus one over root two. Word 1 is the x and y position as `u16` unorm over the reference box. Word 2 is the z position as `u16` unorm in its low half and the scale as `f16` in its high half. WGSL reads word 1 and the low half of word 2 with `unpack2x16unorm` and the scale with `unpack2x16float`. [inferred]
- **The reference box is derived from parameters, not from the tree.** It is the authored envelope, extended by the curtain band below it and by the largest node radius, held per axis as a min and an extent. It is therefore fixed for a species at its authored age and cannot drift. A tree-derived box cannot be used: `timeline::Placement` caches leaves per shoot across ages while the tree's own AABB grows, so words quantized against one age's box would decode against a different one at the next, and the growth path would need a requantisation pass over every cached leaf whenever the box moved. Precision is unaffected, because the stated error bound was already computed against the mature size. [inferred]
- **Three shaders bind the placement buffer, not two.** `shaders/foliage.wgsl:10`, `shaders/select.wgsl:29` and `shaders/shadow.wgsl:21` each declare `array<mat4x4<f32>>` and become `array<u32>` read three words at a time. `foliage.wgsl` rebuilds the rotation for the position and the normal, `shadow.wgsl:31` multiplies the placement into the light's view, and `select.wgsl:70` currently derives the scale as the longest column length and reads the scale word directly instead, which is fewer instructions than it runs today. `telperion-render/src/select.rs:179` uploads the core vector through `bytemuck::cast_slice`, so the storage buffer is the bytes the generator wrote. [paraphrase]
- **The error each approximation introduces, against a leaf 70 mm long on the beech.** Position: 16 bits over the spruce's 32 m axis is a 0.49 mm step and at most 0.24 mm of error. Rotation: a smallest-three quaternion at 10 bits per component is about 0.001 rad typically and 0.0026 rad at worst, which is 0.149 degrees. The worst case is larger than the component half-step because the dropped component reconstructs as the root of one minus the other three squared, so its sensitivity is one over itself and reaches two near half a turn; measured over 4,096 orientations it is 0.00316 rad with each component rounded alone and 0.00250 rad with the encoder taking the nearest of its cell's eight corners, which is what ships at the same thirty bits. Scale: `f16` near 1.0 is a relative error of about 5e-4, which is 35 micrometres. None of the three is a measurement the numeric protocol reads. [inferred]
- **The reading sites move with the representation.** `foliage::transform_point`, `Instances::bounds`, the bound and shell tests inside `cull`, `foliage/clumping.rs:63`, `telperion-render/src/mass.rs:32`, `timeline::Placement.transform`, the Wasm slot-5 boundary (`lib.rs:163` and `:212`, `generate.rs`, `specimen.rs:38`) and the browser TypeScript that reads sixteen floats a leaf. [paraphrase]
- **The committed digests move once.** `tests/species.rs:33` hashes every stored float, so the digest changes for every species, and `crates/telperion-core/tests/species/digests.json` is recommitted in the same commit. This is a change detector rather than a contract, and fn-53's `--pin-note` covers preset value tables, not these. [user]

## Acceptance Criteria
<!-- scope: both -->

- **R1:** One leaf occupies 12 bytes on the CPU and 12 bytes in the GPU storage buffer, asserted by `size_of` and by the buffer's byte length for a known instance count.
- **R2:** Rebuilding the transform from the stored words reproduces the pre-quantisation transform within the stated bounds: 0.25 mm on each position component, 0.0026 rad on the rotation, and 1e-3 relative on the scale. The rotation bound is the measured worst case over 4,096 orientations rather than the typical one; the owner moved it from 0.002 on 2026-09-19 because thirty bits cannot reach that and widening the rotation would break the twelve bytes this spec exists for. At 0.149 degrees it displaces a 70 mm leaf's tip by 0.18 mm, less than the position budget already allows. A test drives the round trip over the shipped presets' placements rather than constructed cases.
- **R3:** The reference box is computed from parameters alone and is identical at every age of one species, asserted by a test that builds the same species at two ages and compares the box. A station outside the box cannot occur by construction, and a debug assertion says so.
- **R4:** The five catalogue species pass `npm run species:qa` with their numeric gates unchanged, and the retained leaf count moves by less than 0.1 percent on every species, which bounds the cull decisions quantisation can flip at the shell boundary.
- **R5:** `crates/telperion-core/tests/species/digests.json` is recommitted in the commit that changes the encoding, and the evidence names the old and the new digest for one seed per species.
- **R6:** All three shaders read the new layout, and the growth path's cached placements decode correctly at an age later than the one that wrote them.
- **R7:** Peak RSS of `fixed_spruces` falls below 1,800 MB from the recorded 3,244 MB, measured by the same VmHWM poll with one test per process on the `ci` profile, and the number for all four `fixed_*` tests is recorded in the evidence.
- **R8:** One headless still per catalogue species is captured on the commit that lands the renderer change, for the owner's eye, and no still is captured before the numeric gates pass.

## Boundaries
<!-- scope: business -->

- What the generator draws does not change beyond the stated error bounds. No generator parameter moves, no preset value moves, and no preset pin in the species pipeline moves.
- The cull's second buffer is fn-85 and has landed. This spec depends on `cull` already taking its `Instances` by value.
- Cluster cards, a detail level parameter, LOD tiers, impostors and streaming placement are not this spec. They are what the forest target needs next, and each is its own spec.
- The wood mesh, 168.3 MB of the spruce's 639 MB, is not this spec.

## Decision Context

- The owner chose on 2026-09-19 to go straight to the 12-byte quantised form rather than land a 32-byte similarity first, because the same reading sites and three shaders would otherwise be rewritten twice. [user]
- The owner stated on 2026-09-19 that byte identity is not a requirement while the generator is in development, and that bytes are expected to move as quality and performance improve. The committed digests are a change detector, and a deliberate change recommits them. [user]
- The row-oriented three-word layout is chosen over a float matrix because WGSL pads a `vec3` column to 16 bytes, so a four-column `mat4x3` would still occupy 64 bytes on the GPU and return nothing for the change. [inferred]
- The parameter-derived reference box and the third shader were both found during fn-85's implementation and escalated to the host under the 2026-09-19 escalation rule rather than guessed at by the implementer. [user]
- This spec is sized to one session on purpose. fn-85 carried both halves and its first dispatch returned one criterion of eight; the friction entry recorded that a spec whose acceptance names `species:qa` across five species, four RSS measurements and five headless stills does not close inside 90 minutes whatever the implementation costs. [user]
