# Reproduction, fn-92

Owner's desk, 2026-09-20: `vulkan-icd-loader 1.4.357.0`, `nvidia-utils 610.57.04`.
Every count is a direct run of the built `ci` test binary with libtest's
default threading, unless the row says otherwise. A crash is death by SIGSEGV.

| Binary | Code | Crashed |
|---|---|---|
| `bark_plates` | base `5a6b9aca` | 5 of 12 |
| `bark_plates` | lock from before `Instance::new` until the adapter is chosen (`2587b61b`, superseded) | 0 of 24 |
| `bark_plates` | lock around `Instance::new` alone, uncommitted experiment | 0 of 24 |
| `bark_plates` | lock around `Instance::new` alone, as shipped (`3b9eba06`) | 0 of 24, then 0 of 12 after a forced rebuild |
| `concurrent_request`, eight threads behind a barrier, eight rounds | base | 0 of 17 |
| the same with a per-thread stagger of 3, 10 and 30 ms after the barrier | base | 0 of 36 |
| scratch binary, eight plain `#[test]` functions each calling `common::gpu()` | base | 0 of 12 |

Through `cargo test --profile ci -p telperion-render --test bark_plates` on the
base: 0 of 2 as written, and 1 of 3 with `WGPU_BACKEND=vulkan` set (the spec
records 4 of 4 earlier the same day).

## The core

`coredumpctl info 218184`, a base `bark_plates` run from the table above:

- thread 218188, the one that died: `#3 vkEnumerateInstanceExtensionProperties (libvulkan.so.1 + 0x50cd1)`, reached from `Gpu::request` under `pollster::block_on`
- thread 218185: `#16 vkCreateInstance (libvulkan.so.1 + 0x51c1c)`, reached from the same `Gpu::request`

Both frames sit inside `wgpu::Instance::new`, which is the call the lock covers.

## What is not known

A bare concurrent `Gpu::request` does not reproduce the crash on this desk:
0 of 65 across the three shapes above. `bark_plates` does. What `bark_plates`
adds over a bare concurrent request is unknown, and was not investigated.

## A contaminated batch, discarded

One batch of five `cargo test --profile ci -p telperion-render` runs at
`3b9eba06` showed `bark_plates` dying in runs 2 to 5. Those runs followed a
build of the base commit from a scratch worktree that shared this worktree's
target directory; the test binary's file name does not depend on the checkout
path, and the build at `3b9eba06` that followed finished in 0.40 s without
recompiling. After a forced rebuild the binary carried the `MAKING_INSTANCE`
symbol (`nm -C`, checked after every run) and the five runs in TIMING.md were
green. That the discarded batch ran the base binary is inferred from this, not
observed: the binary was not inspected before it was rebuilt.
