# The Two Trees' light: emission, cycle, dew and two trees in one frame

## Goal & Context

fn-217's design (DESIGN.md in its evidence, sections 3.3-3.5, 6 and 7) split the supernatural group: scale, leaves and blossom for the release; emitted light, its cycle, the dew of light and two trees drawn together after it. Owner, 2026-10-07: full creative freedom for the Two Trees; the brief VISION-TWO-TREES.md asks for flowers that give light on a cycle, two trees in counterphase mingling at the crossover, and a falling dew of light. The old generator's trees never glowed (2026-09-14, luminosity left for later), so this is not release parity.

## Requirements

- **R1:** C2: a frame clock on the renderer (`set_time`), separate from the build's season; the harness and the still runner set it.
- **R2:** L1 floret emission in the organ block; L2 an analytic emitter per tree in the lit shaders, with the lighting interface given world position (Astra: ambient() lacks it; the sky never calls ambient(), so G1 has its own sky integration); G1 analytic airlight, no post-process pass.
- **R3:** K1 the raised-cosine cycle (period, phase, depth, shape) per tree; D1 stateless additive dew from the organ buffer.
- **R4:** C3 T2: two renderers drawing into one target for the Two Trees' strip, each lit by the other's emitter.
- **R5:** every row dormant at neutral, every passed species pixel-identical with the group off; frame cost measured per term; a time strip of both trees, viewed.

## Boundaries

After the release. Depends on fn-217, fn-218, fn-219.