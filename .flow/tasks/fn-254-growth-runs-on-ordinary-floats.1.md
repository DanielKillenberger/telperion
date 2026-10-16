---
satisfies: [R1, R2, R3, R4, R4a, R5]
---
# fn-254-growth-runs-on-ordinary-floats.1 Growth runs on ordinary floats

## Description
Implement fn-254 R1-R5 as written in the spec: inventory first (R1, in the evidence before code), then floats on both executors with identity exact, then the four checks and the before/after timings and stills.

## Acceptance
The spec's R1-R5, R4a included.

## Done summary
Growth runs on f32 on CPU and device; contract A's Q32/Q62 fixed point and wide products are gone. Identity stays exact: lineage keys, 53-bit draws against `ceil(p 2^53)`, ticks, indices and fixed reduction orders. Byte parity is replaced by the four R4 checks with tolerances measured on shipped presets; continuity walks use the host's f32 reading of decision 9. Cold `grow` compile 14.6 → 11.7 s; Wasm core builds 13-79 % faster; stills unchanged against the base. Astra AGREE.
Open: the birch's device gap predates the floats; the cold-compile timing test flakes under load; four catalogue pins stale; artifact ceilings already crossed at the base. Impl-review SHIP after a round-cap reset under the owner's standing rule.

## Evidence
- Commits: 7ff5ea807a51fa06c1aea35f87319d630b46ae6c, 9d129f439c7525115023f535fad528d5951ebb67, 9eb1ec8abcb1e3ab93dc7211222bc479312e9af1, 8a2e138af8d9ceb35b53f82c464b4e941c70b8a0, f35d86db4c03650baa5bc4bda8ca373eb8194896, 454e54b4a06f546bc9da3ead70c7476b4e9e65ad, ec2d2cf6a8a011dc385e69c1d6c29822b08a33a2, 0e4df0035d6ae89e4604cc129b26b29c1b2a47bb, 763068ff98a151aafdea9f04dc0426bb121b1817, b7a50f1fa85a0b81aefe8ff5e775eee598a82d50, 2666c211959c7c035f70e8ee888560e50f47b57f, 15d7e49ce5109bb26e84c2c3047b480f55d85d19, 51bc5f39d89b675498538713a20da3e5abbf3c6f, 93f3f685c601d2b46fdc2321883d37c001d39277
- Tests: cargo test --profile ci -p <each workspace package> (render under the GPU lock), npm test
- PRs:
