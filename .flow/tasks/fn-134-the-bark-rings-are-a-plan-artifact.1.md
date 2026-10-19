---
satisfies: [R1, R2, R3, R4, R5, R6]
---
# fn-134-the-bark-rings-are-a-plan-artifact.1 Implement The bark rings are a plan artifact

## Description
Split the wood into a ring step and a mesh step so every family runs one pipeline flow.

## Acceptance
Spec R1-R6.

## Done summary
`surface/rings.rs` and `surface::faces`; rings shared through a `OnceLock`; the seated fork and its tests removed. Bytes identical (16/16 mesh, 240/240 binding), Wasm peak equal in all 240, whole builds within noise (spruce 0.9-2.0% faster), parallel wood 10-19 ms faster with no zero fill (`parallel/unfilled.rs`, the crate's first `unsafe`). Production net +341 lines.

## Evidence
- Commits: 7d51f3abb686a600d7ddedb5487fd9ed377268cd
- Tests: cargo test --profile ci --workspace --no-fail-fast (932 passed); cargo test --profile ci -p telperion-core --lib -- pipeline:: surface::; npm run rust:test:wasm
- PRs:
