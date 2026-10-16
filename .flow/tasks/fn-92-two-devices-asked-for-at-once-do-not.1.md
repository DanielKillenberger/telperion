---
satisfies: [R1, R2, R3, R4, R5]
---
# fn-92-two-devices-asked-for-at-once-do-not.1 Implement Two devices asked for at once do not crash the process

## Description
Serialise `wgpu::Instance` creation in `Gpu::request` so concurrent device-backed tests stop crashing the Vulkan loader.

## Acceptance
Every R-ID in the parent spec's acceptance holds.

## Done summary
`Gpu::request` makes its instance under a process-wide lock on native targets, held around `Instance::new` alone. `bark_plates` went from 5 crashes in 12 runs on the base to 0 in 36. Render crate 5 of 5 green under default threading (95-102 s vs 153 s single-threaded base). `concurrent_request.rs` is a smoke test that has never been red.

## Evidence
- Commits: 2587b61b1f0d0bc5615a772e5d3bcd1a39981b77, 430b80e3fc2392a44f7edcd975f9bc79eb988f9e, 3b9eba06c5ecca17e6947ad96dfae76fc98db55b, b84c9aedc4ddd5548a3cdd71f48324b9e29aae5e
- Tests: bark_plates x24 at 3b9eba06 (0 crashes); cargo test --profile ci -p telperion-render x5; cargo test --profile ci --workspace --no-fail-fast (exit 0, 375 s)
- PRs:
