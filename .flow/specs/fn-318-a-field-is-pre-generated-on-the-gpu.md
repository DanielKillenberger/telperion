# A field is pre-generated on the GPU

## Why

The owner, 25 Oct 2026: "just pregen them with the gpu?". killenberger.com draws its pages without WebGPU from voxels the CPU field made in advance, and its WebGPU pages grow the field live on the device. fn-317 measured the voxelizer on both executors' answers: `tuft` moves up to 5% of cubes (beech), the date palm 10–24%, because the two executors part the crown's edge cells differently and draw the palm's fronds separately. Pre-generating with the device field makes the set and the live tree come from one executor, so the gap is deleted, not narrowed. It supersedes the proposed stabilisation of `tuft`'s system centres.

## Requirements

- R1: A native command grows a family's field on the device (the resident field, as `every_preset_s_named_fields_are_written_for_the_voxelizer` does) and writes the answer in the shape `growField` returns: frame, flags, woodRadius, leaves, limbs, flowers. It takes a parameter file (a family's JSON) or a shipped preset, `n` and an optional `limbOrder`; a preset is resolved to its parameters before the pipeline sees it (fn-317).
- R2: A family the device cannot grow fails with an error; no CPU fallback.
- R3: For the same family, n and limb order, its output equals `growField`'s answer on the same GPU byte for byte. A test holds it natively.
- R4: Whether different GPUs agree is unknown. The killenberger site measures a browser's WebGPU field against this command's output; the measurement is recorded in this spec before the site switches its pregen.

## Out of scope

The site's build step (killenberger.com's own repository). Release 0.2.0.
