# The same binary draws the same leaves every run

## Goal & Context

fn-258's FRICTION.md and RESULT.md (2026-10-10): on the fine-executor trees (spruce, Telperion), two runs of the same binary at the same seed, pose and age differ in 0.08-0.16% of pixels, all on leaves; the palm and the oak (which grow on the CPU) are byte-identical. A cut meant to change nothing could therefore only be held to that noise, not proved. AGENTS.md "Generator evolution" keeps repeatability within an implementation as a contract, and fn-254 R4 check 1 asks it of growth. Unknown, to be found first: whether the variation is in growth (the device grower), in leaf placement or packing, or in drawing (instance order under atomics, thinning ranks).

## Requirements

- **R1 (repro first):** a standalone test that builds the spruce at its default age and seed twice in fresh contexts and compares the prepared leaf instances (and the grown tree) byte for byte; red on F4. Then the first stage whose output differs, named and checked against the code.
- **R2 (fixed):** that stage made order-independent (deterministic prefix allocation or a stable sort by lineage key, never a float atomic), on the device; the test green over 10 repeats on every fine-executor preset; frame time and build time before and after.
- **R3 (proved):** two stills of the same view byte-identical for every shipped preset; gate and npm test green.

## Boundaries

Repeatability within one executor. Cross-executor agreement is fn-254's.
