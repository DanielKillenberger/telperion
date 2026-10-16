# fn-256-the-whole-grower-runs-on-the-gpu.10 Upward sums and retirement reuse unchanged subtrees

## Description
COUNCIL round 5 step 10: instrument real input changes per cycle; then per-reduction dirty bits propagated to ancestors, recompute dirty axes in canonical order from cached children, never subtract-and-add; downward passes not skipped; normaliser blocks invalidated by membership or rank change; compaction remaps or invalidates caches. Each expansion stopped if invalidation and scheduling eat its saving.

## Acceptance
Node table byte-identical to today's on every preset at caps 64/128/1,024 and ten contexts; measured saving per phase reported.

## Done summary
The per-cycle input changes were counted first (DESIGN.md §16.1). Then both expansions were built and timed. Upward sums with dirty bits: stopped and deleted. Marking cost what skipping saved, net +1.1 / +0.7 / −0.1 ms, because three quarters of the blocks hold a changed axis. Retirement kept: each living is carried up once, when its axis closes, and each child's terms are taken once (`terms_main`). Block stamps are atomic. Retirement's stamped time fell 220.9 → 60.8 ms (Telperion), 37.4 → 11.5 (oak) and 26.3 → 11.3 (beech), medians of three alternating runs. The builds' stamped GPU time fell 15.0% / 11.6% / 10.1%.

Node tables: all 240 hashes (8 presets × caps 64/128/1,024 × ten contexts) equal the post-task-9 hashes. Normaliser blocks: not cached, so nothing to invalidate. Follow-up for task 11: `inside_main` (26 ms on Telperion) still sweeps every block.

## Evidence
- Commits: de0eb93c917c067e985ff04bf0ef6be69c61fa1f, f7b74b3c2e3760f611cdb6a6d7a570ec42465e92, 53ab77e5fc4891411b6b4a32d1b2111f48c76db2
- Tests: cargo test --profile ci --workspace --exclude telperion-render --no-fail-fast, cargo test --profile ci -p telperion-render --no-fail-fast (GPU lock), NODES_CAPS=64,128,1024 NODES_CONTEXTS=10 cargo test --profile ci -p telperion-render node_tables_are_hashed -- --ignored, npm test
- PRs:
