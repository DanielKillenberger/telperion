# Baseline environment

Conductor target: `/home/daniel/Projects/telperion/.worktrees/fn-9-real-species-profiles-and-procedural`, plan commit `7ce571c` over merged FN8 `2fcac35`.

Native growth and foliage suites pass (14 tests); typecheck and wasm:build pass. Rust environment: PATH prefix `/tmp/telperion-cargo/bin`, RUSTUP_HOME `/tmp/telperion-rustup`, CARGO_HOME `/tmp/telperion-cargo`. Primary node_modules can be symlinked for existing dependencies but lacks Playwright; browser tests require PLAYWRIGHT_MODULE `/tmp/fn20-browser/node_modules/playwright/index.mjs` and CHROMIUM_EXECUTABLE `/usr/bin/chromium`.

Browser integration pins old Ordinary counts (13,616 nodes / 59,810 instances). Natural-default changes intentionally alter these. Preserve meaningful determinism and ownership tests and explicitly document any changed fixture; do not silently weaken gates. Giant frozen migration inputs may separately pin legacy nonzero bias.

Task 2's planned helper `examples/species_metrics.rs` would be auto-discovered as a binary by Cargo. Prefer a support subdirectory module, with a declared task update before adding files. The core currently has no dependencies; parsing profile JSON should not introduce a runtime dependency or a hand-written JSON parser. If tooling needs serde_json, prefer a dev dependency and coordinate Cargo files with the conductor.
