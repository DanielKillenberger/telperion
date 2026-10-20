---
satisfies: [R5, R6, R7]
---
# fn-294-the-release-reviews-findings-are-fixed.3 Reproduce the metadata panic and the oversized lattice; fix or close each

## Description
The two plausible findings. Each starts with a standalone repro test; the fix follows only if it reproduces. Record the outcome of each in `.flow/evidence/fn-294-the-release-reviews-findings-are-fixed/REPROS.md`.

**Size:** M
**Files:** `crates/telperion-core/src/params.rs`, `crates/telperion-core/src/params/tests.rs`, `crates/telperion-space/src/light/field.rs`, `crates/telperion-space/src/light/tests.rs`, callers of `overlay` and `Field::on`
**Touches:** [crates/telperion-wasm/src/lib.rs, crates/telperion-core/src/params.rs, crates/telperion-core/src/params/**, crates/telperion-core/examples/**, crates/telperion-core/src/catalogue/**, crates/telperion-space/src/light/**, crates/telperion-space/src/grow/**, .flow/evidence/fn-294-the-release-reviews-findings-are-fixed/**]

### Approach
- R5: test in `params/tests.rs` builds a shipped family, sets `space.light` to a valid non-`SITE` `Light`, calls `params::overlay`. If it panics: `metadata` returns `Result<Value>` (or `overlay` checks `space::rows` before calling it) so `overlay` returns the refusal by name; update callers (`examples/*.rs`, `catalogue/tests.rs`, `crates/telperion-wasm/src/lib.rs` preset/catalogue). No fallback.
- R6: test drives the public build (`telperion_space` grow or `pipeline::build`) with a species inside `species/validate.rs` bounds (internode ≤ 100 m, mean nodes ≤ 500, budget) chosen to span more than 2^28 lattice nodes while coordinates stay under 4096 m. Must not allocate the lattice: the check runs before allocation. If reachable: `Grid::spanning` returns `Result`, refusing with `Error::Refused` via `refuse(...)` as `geometry/lay.rs:93` does, propagated through `Field::on` and `grow/sketch.rs:107`. If unreachable, record which bound stops it.

### Investigation targets
**Required**:
- `crates/telperion-core/src/params.rs:20-45,~125` and `crates/telperion-core/src/presets/space.rs:195-232`
- `crates/telperion-space/src/light/field.rs:47-70,126-215`
- `crates/telperion-space/src/grow/sketch.rs:100-115`
- `crates/telperion-space/src/species/validate.rs`, `crates/telperion-space/src/error.rs`
- `crates/telperion-space/src/geometry/lay.rs:28,93,367`

### Key context
- The wire cannot state a light; only a Rust-built family can, so R5's repro is a Rust test.
- A repro that really allocates ~2^28 nodes costs gigabytes; keep the test cheap (tiny budget or few long internodes).

### Acceptance
- [ ] Each repro committed as a standalone test, run before any fix; outcome recorded in REPROS.md.
- [ ] Reproduced: refused by name, no panic, no allocation of the oversized lattice; test green. Not reproduced: requirement closed with the recorded reason and the test kept if it pins the bound.
- [ ] Gate and `npm test` green; codex impl-review SHIP.

## Acceptance
- [ ] TBD

## Done summary
Both plausible findings reproduced as standalone red tests before any fix (REPROS.md), so both were fixed.
- R5: `params::overlay` panicked at the metadata `expect` for a family under `Light::NEUTRAL`; it now writes the wire through a fallible `wire()` and refuses with `InvalidValue { field: "space", value: <space::rows's reason> }`. `metadata` keeps its signature for presets and parsed wires.
- R6: the oak with 5 m internodes at 8 years (10,000 budget) asserted at `light/field.rs:66` through the public `grow`; spanning is now fallible and refuses `light.lattice` before any lattice allocation. Codex's own candidate (100 m internodes) is refused earlier by the ground bound.

Gate: 1406 passed, 0 failed, 111 ignored. npm test: 175 passed.

stage: impl-review - ran (codex, 3-lens panel: correctness SHIP, contracts SHIP, integration SHIP) (model: gpt-6.1-sol)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 53545a1f4cfe6e89e1501f2cc89ac44e9e7c8e13, 6b2ba9725b29bb94a390b4940732985385f6a2d4, 4d0525dcb9de8610ae838aff06a7e6b3bee5fe4c
- Tests: telperion-heavy 8 --threads 2 -- cargo test --profile ci --workspace --no-fail-fast, npm test
- PRs: