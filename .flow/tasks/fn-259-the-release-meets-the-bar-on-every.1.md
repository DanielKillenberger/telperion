---
satisfies: [R6, R7]
---
# fn-259-the-release-meets-the-bar-on-every.1 Format the tree once, check fmt in CI, drop the two dead config keys

## Description
The housekeeping that every later task rebases on: one `cargo fmt --all` commit with nothing else in it (four diffs today: `crates/telperion-core/src/lib.rs`, `pipeline/surface.rs`, `pipeline/surface/chunks.rs`), a CI step that runs `cargo fmt --all --check`, and the two removed keys deleted from `.flow/config.json`. First in order so tasks 2-6 never carry format noise.

**Size:** S
**Files:** `crates/**` (fmt only), `.github/workflows/tests.yml`, `.flow/config.json`, `docs/principles.md`
**Touches:** [crates/**, .github/workflows/tests.yml, .flow/config.json, docs/principles.md]

### Approach
- Commit 1: `cargo fmt --all` only (`style: cargo fmt --all, once`), the same shape as the fn-237-format pass (`4acda8bbe`).
- Commit 2: a fmt job (or a step in an always-run job) in `.github/workflows/tests.yml`. It must run on every PR the workflow triggers on, so it does not sit behind the receipt cache that skips Rust suites (`receipts` job, `tests.yml:52-120`). Toolchain from `rust-toolchain.toml` (already has `rustfmt`), as `core-plan` installs it (`rustup toolchain install`).
- Commit 3: delete `artifacts` (only `html.enabled` in it) and `pipeline.chainStages` from `.flow/config.json`; keep `pipeline.qa`. Run `flowctl preflight --json` and confirm the removed-key note is gone.
- `docs/principles.md` "How they are held" (`:16`) names CI's checks; add the fmt check in one line if that list names the others.

### Investigation targets
**Required:**
- `.github/workflows/tests.yml` - job layout and the receipt skip
- `rust-toolchain.toml`
- `.flow/evidence/fn-293-each-stages-output-has-one-form-and-its/FRICTION.md:7-9,109` - the two keys

### Key context
The gate is not needed for a pure fmt commit plus YAML; `cargo build --profile ci --workspace` (or `cargo check`) proves the formatted tree still compiles. The full gate runs at the end of task 2, the first task with code changes.

### Acceptance
- [ ] One commit holds only `cargo fmt --all` output; `cargo fmt --all --check` exits 0 after it.
- [ ] CI runs `cargo fmt --all --check` on every PR and push the tests workflow triggers on, not skipped by a receipt.
- [ ] `.flow/config.json` has neither key; `flowctl preflight` prints no removed-key note; no other key changed.
- [ ] The workspace still compiles (`cargo check --workspace --all-targets`).

## Acceptance
- [ ] TBD

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
