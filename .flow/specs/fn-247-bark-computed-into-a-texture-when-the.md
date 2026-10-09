# Bark computed into a texture when the tree is built

## Goal & Context

fn-246 measured the wood's shaded pass at about 30 ms at close views on the oak and the Two Trees, the cost of supersampling the procedural bark relief 9 to 25 times a pixel; a one-sample prefilter was 3× faster but measurably noisier against a 4x supersampled truth (sharp plate walls and furrow edges need more than one sample), and no expected occlusion or tint term could remove it (FPS.md). The owner chose (2026-10-09): ship the prefilter now, and after the release compute each tree's bark into a texture when the tree is built, at runtime (never offline), mipmapped so the hardware filters it: fast and clean.

## Requirements

- **R1 (design first):** the texture parametrisation of the wood surface (per run, per ring span), resolution by the bark's own scales and the tree's size, memory budget, and when it is computed (at build, on the device, in the pipeline's expansion stage, never offline); stated before code.
- **R2 (built):** the relief and tints written into a mipmapped texture at build and sampled once a pixel with hardware filtering; the procedural per-pixel evaluation removed; both executors.
- **R3 (proved):** every bark fixture within the bark tests' original bounds of the 4x truth (mean ≤ 3.0); the oak's and the Two Trees' close wood at or under the prefilter's cost; build time and memory measured.
- **R4:** gate and npm test green.

## Boundaries

Depends on fn-246. Not a release item.
