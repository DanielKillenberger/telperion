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
Verified integrated branching implementation at `fc2add4a01efdbbe45c63e6084e2606e99f7a747`. Spreading/tiered controls, crooked local scaffolds, clipped lateral stations, persistent leader and hanging secondaries pass focused geometric regressions. Natural bias is independent of disabled supernatural effects; Ordinary surface effects are neutral and Two Trees effects explicitly enabled. Full release workspace suite passed (54 tests, four pre-existing external-reference ignores), workspace check, format/whitespace and TypeScript check passed. No Rust edits were needed.

Wasm habit/foliage exposure and final schema grouping remain task 7; browser assets are transitional. Ordinary exact-count browser fixtures remain downstream work due to intentional default/branch changes. No browser fixture or ownership/determinism assertion was changed. Species calibration and final visual QA remain downstream; no fidelity or performance verdict is claimed.

REVIEW_MODE=none: no reviews invoked. Source-worker provenance remains in HANDOFF.md and the worker summary; completion evidence commits now reference the integrated handoff SHA.

Runtime state reconciled on 2026-09-06 from this committed completion receipt after fetching remote through 1297516. The preceding tests describe the original completion, not a fresh fidelity verdict.
stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: fc2add4a01efdbbe45c63e6084e2606e99f7a747
- Tests: baseline: green — cargo test --release -p telperion-core --test growth --test foliage --test colonization (19 tests), npm run typecheck, red: natural_bias_is_independent_of_disabled_effects — expected missing SupernaturalParams API, red: three habit tests — expected missing BranchHabit API, red: clipped_local_axis_still_subdivides_before_its_terminal_twig — 1 branch origin, required >=3, red: spreading_habit_subdivides_crooked_substantial_axes_without_effects with local bend disabled — 0 local bends, required >10, cargo test --release -p telperion-core --test growth --test foliage --test colonization — passed 23 tests before final local bend changes, cargo test --release --workspace — passed before final local bend and minimum leader segment changes; pinned external reference tests ignored by existing annotations, cargo test --release -p telperion-core --test growth — final code passed 13 tests, /tmp/fn9-branching-green-local-bends.log, cargo fmt --all — ran before final minimum leader segment signature edit, Final gate classify/full verification not run: user explicitly stopped implementation; task remains in_progress, Integrated HEAD fc2add4a01efdbbe45c63e6084e2606e99f7a747: cargo test --release -p telperion-core --test growth --test colonization — 18 passed; .flow/tmp/fn9-integrated-branching.log, Integrated: cargo test --release --workspace — 54 passed, 4 existing external-reference tests ignored; .flow/tmp/fn9-integrated-workspace-tests.log, Integrated: cargo check --workspace; cargo fmt --all --check; git diff --check — passed, Integrated: npm run typecheck — passed after npm ci --ignore-scripts restored missing local dependencies; .flow/tmp/fn9-integrated-typecheck.log
- PRs: