---
satisfies: [R1, R2, R3, R4, R5, R6]
---
# fn-84-the-generator-declares-what-it-can.1 Implement The generator declares what it can express

## Description
Declare the capability vocabulary and gate species against it.

## Acceptance
Every R-ID in the parent spec's Acceptance is satisfied.

## Done summary
`capability.rs` declares `EXPRESSED` (8) and `UNEXPRESSED` (7) names with meanings and an FNV-1a `version()`. `gate.rs` computes `missing` against it, reports unrecognised names separately, files the preset-derived check as `preset-capability`, fails closed, and reads the manifest's row when the packet's list is empty. On the palm: `woody-axes` met, five organs missing.

## Evidence
- Commits: d42449478e604524f28501bdc3461d48be09c864, 6483e80b3e0ec6f9e13edd61f05e1d71d67b6ff9, c99787107eac580f368d832f4a3b1d575766c686
- Tests: cargo clippy --release --workspace --all-targets -- -D warnings, cargo test --release --workspace --no-fail-fast -- --test-threads=1, node scripts/catalogue-check.mjs
- PRs: https://github.com/DanielKillenberger/telperion/pull/43
