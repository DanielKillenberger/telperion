---
satisfies: [R1, R2, R3, R4]
---
# fn-215-tree-space-the-ordinary-tree-as-the.1 Implement the ordinary tree as the space's centroid

## Description
Build the ordinary preset as the corners' centroid, per the spec.

## Acceptance
Every R-ID of the spec, as amended: R1 sized to the corners' mean; R3 passes at 40, age-80 walks deferred to fn-230 (host, 2026-10-08).

## Done summary
Centroid of beech, oak and spruce with lateral factor 0.742, so light-neutral expected kept at 80 is 3,559,562; `examples/centroid.rs` reproduces the preset byte for byte and `space_presets` tests it. Builds natively and in Wasm at 40 and 80: at 80 keeps 1.84M-3.31M, grows at most 15M (under 20M), 22-43 s. Sheet viewed; walks pass at 40. Open for the owner: build time at 80 and the default dressing. Review: codex NEEDS_WORK on R3 at 80, SHIP after the host deferred it to fn-230.

## Evidence
- Commits: fc3a22c2a8e6df93b049d7f32e52925fdd3e3e30, e25bd047b0cf4b956b80a795aea9c6fd2c19b433, f4310ac83d2f1ebf4df07e2ca67d7594706c8b83, 7a6096039ea8e588d04c2a9e8358d7bd508f2d41, 4edc4a49ac2e187a1210422b412b6b6fd0fd5df8, a5d21fcbab7e1a7b92de82a42909d2a42f59c1c6, ef33eec44186f60d8de4eac0616280b9c06d0148
- Tests: cargo test --profile ci --workspace --no-fail-fast; cargo test --profile ci -p telperion-core --test space_presets; npx vitest run src/field/field.test.ts harness/parity.test.ts
- PRs:
