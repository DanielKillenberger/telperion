# Leaf placement holds one copy

## Goal
`foliage::cull` allocated a second buffer at the input's length and pushed survivors into it, so the crown was two copies for the whole call: on the spruce at seed 1, 7,353,754 leaves at 64 bytes twice (941.3 MB), while the cull dropped nothing on the oak, beech or spruce (only the birch drops, 260,888 to 226,057). Peak RSS before (ci profile, VmHWM, one test per process): `fixed_spruces` 4,994 MB, `fixed_beeches` 4,786, `fixed_oaks` 1,672, `fixed_birches` 820, behind fn-84's OOM kill. This spec holds one copy; the twelve-byte leaf encoding is fn-86.

## Decisions
- Owner, 2026-09-19: split the original fn-85 after its first dispatch returned this criterion alone; the one-copy cull and the twelve-byte encoding are independent, and "one task per spec" says nothing about sizing a spec to a session.
- `cull` consumes its input and `Vec::retain`s in place; capacity is not shrunk; an overflowing or non-finite transform sets a flag and the error is raised after the pass.
- Out of scope: the encoding (fn-86), the test thread cap and `SEEDS_IN_FLIGHT`, the wood mesh.

## Acceptance Criteria
- **R1:** `cull` consumes its `Instances` and allocates no second buffer; a test pins pointer and capacity across a partial and a total cull — met (`cull_retains_in_the_buffer_it_was_given`, red on the old body).
- **R2:** Same leaves in the same order for every species; overflow still returns `ResourceLimit` — met.
- **R3:** `fixed_spruces` peak RSS below 3,500 MB, all four recorded — met.

## Result
Peak RSS (ci profile, VmHWM, one test per process): spruces 4,994 -> 3,244 MB (-35%), beeches 4,786 -> 3,945 (-18%), oaks 1,672 -> 1,469 (-12%), birches 820 -> 778 (-5%). Escalated into fn-86: `shadow.wgsl` binds the placement buffer too (three shaders, not two), and `timeline::Placement` caches leaves while the tree's AABB grows, so fn-86 quantizes against a parameter-derived box. Commits c9aed1c0, 66e5229d.

## Open
None.

## Friction
- The original eight-criterion spec could not fit a 90-minute timebox; R1 alone landed -> split (owner); lesson: a dispatch carries the spec's own estimate.
- `/usr/bin/time` missing and sandbox denied `git checkout <ref> -- <path>` -> local setup.
