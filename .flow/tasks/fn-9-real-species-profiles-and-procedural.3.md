---
satisfies: [R3, R6]
---
# fn-9-real-species-profiles-and-procedural.3 Add profile-driven branching and natural bias defaults

## Description
Add profile-driven branching and natural bias defaults. See the parent spec for the botanical target and validation contract.

**Size:** M
**Files:** `crates/telperion-wasm/src/params.rs`, `crates/telperion-wasm/src/lib.rs`, `crates/telperion-core/tests/crown_reference.rs`, `tests/migration/foundation.rs`, `crates/telperion-core/src/branching/habit.rs` (if a separate habit module is warranted), `crates/telperion-core/src/branching.rs`, `crates/telperion-core/src/branching/local.rs`, `crates/telperion-core/src/twigs.rs`, `crates/telperion-core/src/bias.rs`, `crates/telperion-core/src/presets.rs`, `crates/telperion-core/tests/growth.rs`
**Touches:** [crates/telperion-wasm/src/params.rs, crates/telperion-wasm/src/lib.rs, crates/telperion-core/tests/crown_reference.rs, tests/migration/foundation.rs, crates/telperion-core/src/branching/**, crates/telperion-core/src/branching.rs, crates/telperion-core/src/branching/local.rs, crates/telperion-core/src/twigs.rs, crates/telperion-core/src/bias.rs, crates/telperion-core/src/presets.rs, crates/telperion-core/tests/growth.rs]

### Approach
- Mechanical bias consumer updates are included in Wasm source and existing native reference tests. Preserve explicit frozen reference effects; task 7 owns final wire grouping and browser metadata regeneration. The foundation file is an ordinary Rust reference test, not a shared schema migration.
- Implement only task 1's evidenced architectural gaps using existing structural/local branching stages. Address persistent leader versus spreading habit, order/position-dependent laterals and terminal taper only where the chosen profiles require them.
- Separate natural gravitropism/orientation from supernatural writhe/spiral configuration using the smallest typed grouping; keep ordinary defaults natural and explicitly configure existing Two Trees.
- Preserve deterministic seeded generation, parent-before-child topology and optional outputs. Keep existing Family wiring compiling; task 7 owns Wasm/browser propagation.
- Use focused geometric regressions for the reported crown escape, branch endpoints and tip/taper behavior. If research proves a surface-stage change is required beyond these files, record the repro and split that concrete repair before dispatch instead of hiding it inside a broad rewrite.

### Investigation targets
**Required:**
- `crates/telperion-core/src/branching/local.rs` — local rule implementation
- `crates/telperion-core/src/twigs.rs:5-105` — existing branching law
- `crates/telperion-core/src/bias.rs:12-37` — mixed default biases
- `crates/telperion-core/src/presets.rs:10-123` — explicit family defaults
- `crates/telperion-core/tests/growth.rs` — generation regressions
- `.flow/memory/bug/runtime-errors/a-trunk-region-guard-on-the-parent-node-2026-09-04.md` — child/edge boundary checks

### Quick commands
```bash
cargo test --release -p telperion-core --test growth --test colonization
```

## Acceptance
- [ ] Task 1's required architecture controls have focused synthetic tests and reproducible seeds.
- [ ] Ordinary defaults have no supernatural influence; natural tropism remains available. Explicit Two Trees effects remain enabled.
- [ ] Disabling supernatural terms matches same-seed natural output without residual deformation.
- [ ] Required branching regressions pass, and unsupported profile architecture remains explicitly open rather than silently approximated.

## Done summary
Blocked:
Owner requested implementation stop and clean transfer to Forge. Worker has stopped; existing implementation is preserved in the handoff WIP. Temporarily blocking solely to release the claim, then resetting to todo for the successor to verify and complete. No external dependency or human decision blocks resumption.
## Evidence
- Commits:
- Tests:
- PRs:
