## Goal & Context
<!-- scope: business -->

`foliage::cull` filtered a crown by allocating a second buffer at the input's own length and pushing the survivors into it, so the resident foliage was two copies for the whole of the call. On the spruce at seed 1 that is 7,353,754 leaves at 64 bytes twice, 941.3 MB, and the cull drops nothing at all on the oak (715,065 in and out), the beech (4,928,780) or the spruce (7,353,754). Only the birch drops any, 260,888 down to 226,057. Three of the four shipped species paid a full second allocation to remove nothing. [user]

Measured before the change, peak RSS polled from `/proc/<pid>/status` VmHWM with one test per process on the `ci` profile: `fixed_spruces` 4,994 MB, `fixed_beeches` 4,786 MB, `fixed_oaks` 1,672 MB, `fixed_birches` 820 MB. Each test holds `SEEDS_IN_FLIGHT = 4` specimens at once, and at the default harness thread count the four run together, which is the OOM kill fn-84's FRICTION.md recorded. [user]

This spec holds one copy. The twelve-byte leaf encoding that the same measurements argue for is fn-86, which depends on this one. [user]

## Architecture & Data Models
<!-- scope: technical -->

- **`cull` consumes its input and retains in place.** The signature becomes `pub fn cull(instances: Instances, element: &Element, envelope: Envelope, shell_depth: f64) -> Result<Instances>` and the body keeps `Vec::retain` over the buffer it was handed. No second allocation exists at any point. Capacity is not shrunk, which leaves the birch, the only species that drops any leaf, holding 34,831 unused slots. [inferred]
- **The refusal `retain` cannot make is deferred.** A closure cannot return an error, so an overflowing or non-finite transform sets a flag, the pass completes, and the error is raised after it. The vector is discarded unread on that path, so the leaves already removed behind the failure point do not matter. [inferred]
- **The pre-cull count is read before the cull.** `species_measure`, `geometry_benchmark`, `measure` and the species suite report `pre_cull_instances` from the placement length after culling today, and capture it into a local first. Test call sites that assert `cull(&a, ..) == a` clone their small fixture. [inferred]

## Acceptance Criteria
<!-- scope: both -->

- **R1:** `cull` consumes its `Instances` and allocates no second leaf buffer. A test records the vector's pointer and capacity before the call and asserts the returned vector carries both, across a cull that drops some leaves and a cull that drops all of them. Errors: a cull that drops every leaf still returns the original allocation, empty.
- **R2:** The cull's result is unchanged for every species: the same leaves, in the same order. The birch still drops 260,888 to 226,057, and the oak, the beech and the spruce still drop none. A transform that overflows or is not finite still returns `ResourceLimit`.
- **R3:** Peak RSS of `fixed_spruces` falls below 3,500 MB from the recorded 4,994 MB, measured by the same VmHWM poll with one test per process on the `ci` profile, and the number for all four `fixed_*` tests is recorded in the evidence.

## Boundaries
<!-- scope: business -->

- The twelve-byte leaf encoding is fn-86. Nothing here changes what a leaf transform is, so no stored byte moves, no committed digest moves, and no shader is touched.
- The `rust:test` thread cap and `SEEDS_IN_FLIGHT` are not this spec. They bound how many specimens run at once, which is a different question from what one specimen holds.
- The wood mesh, 168.3 MB of the spruce's 639 MB, is not this spec.

## Decision Context

- The owner chose on 2026-09-19 to split the original fn-85 after its first dispatch returned this criterion alone. The one-copy cull and the twelve-byte encoding are independent: the cull needs no representation change and lands green on its own. The one-task-per-spec rule governs tasks within a spec and says nothing about sizing a spec to a session. [user]
- The measurements were taken on 2026-09-19 on the `ci` profile, peak RSS polled from `/proc/<pid>/status` VmHWM with one test per process, instance counts read from `species_measure`. The method is named so both ends of the comparison are taken the same way. [user]
