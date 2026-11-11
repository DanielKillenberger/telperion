# fn-348 friction

## 11 Nov 2026: a jev-only change ran the whole gate in a fresh worktree

What I was doing: verifying a change confined to `crates/telperion-jev` (plus `docs/` and `.flow/`) with `telperion-heavy 8 --threads 4 -- python3 scripts/gate.py`, as the dispatch asked.

What slowed it: the worktree's `target/` was new, so `target/gate/receipts/` was empty and every suite's key was unreceipted. The gate ran rust-core, rust-render, rust-wasm, node and package as well as rust-jev: 154 s build and listing, 55 s package, a 1,218 s test run (1,547 tests), 128 s core without geometry, 180 s node. That is about 29 minutes against the roughly 2 minutes AGENTS.md gives for a commit that touches only the jev suite.

Cost: about 27 minutes of wall time and the desk's 8 GB heavy slot, inside a 90-minute timebox.

What would have removed it: seeding a new worktree's receipts from the base commit's receipts in the main checkout when the base matches (the keys hash the commit's inputs, so a receipt for an unchanged suite at the base is valid), or a gate flag that takes receipts from another checkout's `target/gate/receipts/` for keys that match exactly.
