---
satisfies: [R1, R2, R3, R4, R5, R6]
---
# fn-69-the-tests-run-in-github-actions.1 Implement The tests run in GitHub Actions

## Description
Add the GitHub Actions workflow the spec describes.

## Acceptance
Every R-ID in the parent spec's Acceptance is satisfied.

## Done summary
Added `.github/workflows/tests.yml`: one job on push to master and PRs (superseded runs cancelled, `contents: read`), toolchain from `rust-toolchain.toml`, `wasm-bindgen-cli` at the Cargo.lock version, Node from `engines`, cargo and npm caches keyed on the lockfiles, then `npm run rust:test`, `npm test`, `npm run typecheck`. No secret, browser suite, fmt or clippy gate. Locally: workspace tests rc=0 (80 suites), `npm test` 85 passed, typecheck rc=0.

## Evidence
- Commits: 8bd3e5f9ed1dde8cdee8c288e2cab23fc3c509e9
- Tests: cargo test --release --workspace, npm test, npm run typecheck
- PRs: #32
