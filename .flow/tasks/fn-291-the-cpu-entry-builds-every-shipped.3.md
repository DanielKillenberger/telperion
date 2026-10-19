---
satisfies: [R2, R5]
---
# fn-291-the-cpu-entry-builds-every-shipped.3 R5 (R2e): growth's tables narrowed losslessly where created; post-growth Wasm footprint per preset

## Description
R5 (the host's R2e). The CPU growth's tables are narrowed where they are created, losslessly: counts, indices, flags and ids at the narrowest width their range allows, and padding removed. Trees stay byte-identical. Measure first. The scout's read is that the integers that can narrow sit per axis or per bud. The per-phytomer table (`Phytomer`, about 128 B) is mostly f64 values and a u64 lineage key, which fn-257's rule keeps. So the saving may be small; report what it is.

**Size:** M
**Files:** `crates/telperion-space/src/structure.rs`, `crates/telperion-space/src/grow.rs`, `crates/telperion-space/src/grow/*.rs`, `crates/telperion-space/src/presence.rs`, `crates/telperion-core/src/tree.rs` (only if a `Node` field narrows with identical output), `.flow/evidence/fn-291-the-cpu-entry-builds-every-shipped/PROBE.md`
**Touches:** [crates/telperion-space/**, crates/telperion-core/src/tree.rs, crates/telperion-core/src/pipeline/space.rs, crates/telperion-render/src/generation/cycle/**, .flow/evidence/fn-291-the-cpu-entry-builds-every-shipped/**]

### Approach
- Measure first. Take a `capacity() * size_of` dump of growth's tables at its peak cycle for telperion seed 7, natively; a throwaway, like `woodsize.rs`. Also take the post-growth Wasm memory for every preset at seeds 1, 7 and 42, using `trace-pages.py` and `trace-case.mjs`. Record both in PROBE.md before changing code.
- Narrow only a field whose range a bound in code proves. Examples: `BUDGET` is 20M, so ids become `u32`; `Species.pas` bounds `pa` to `u8`. Use a `u32::MAX` sentinel instead of `Option<usize>`, and pack the `Draws` bools into one byte. Leave every f64 and the u64 key as they are.
- Check the device side before changing any pub field type: `telperion-render/src/generation/cycle/build_grow.rs:76` and `build_arena.rs` size device tables from these types.
- Byte identity:
  - `telperion_space::digest` (`fine/canonical.rs:96`) and `pipeline/space/repeat_tests.rs` stay green.
  - The space and walk tests under `crates/telperion-space/tests/` stay green.
  - The device-against-CPU tests in `telperion-render/src/generation/cycle/` stay green.
  - `hash-dist.mjs` gives identical hashes on all 8 presets before and after.

### Investigation targets
**Required**:
- `crates/telperion-space/src/grow.rs:300-345` — `Grower`
- `crates/telperion-space/src/structure.rs:100-180` — `Axis`, `Phytomer`, `Origin`
- `crates/telperion-space/src/presence.rs:155-215` — `Draws`
- `crates/telperion-space/src/shed.rs:60-150`
- `.flow/specs/fn-257-the-site-grows-its-trees-on-the-device.md` — "compact from the start, lossless during growth"
## Acceptance
- [ ] PROBE.md tables growth's tables by bytes at the peak (telperion seed 7) and the post-growth Wasm memory per preset (seeds 1, 7, 42), before and after
- [ ] Each narrowed field names the bound in code that proves its range; no float and no lineage key narrowed
- [ ] Every preset's grown tree and dist outputs byte-identical (`digest`, repeat tests, `hash-dist.mjs` on all 8 presets)
- [ ] Device growth tests that read these types still pass
## Done summary
I measured growth's tables first and then changed two layouts so they waste no space. `Axis.joined` is now `Option<Box<Join>>`, because a join sits on only 0.7% of axes. `Draws.outlived` is now an `f64` plus a flag, read through `outlived()`. On wasm32 an `Axis` drops from 224 to 192 B and `Draws` from 112 to 104 B. No float, lineage key or index changed width. On wasm32, `usize` is already 32 bits, so the bar's target has no index to narrow.

What it bought (measured, PROBE.md "Task 3"):
- Telperion seed 7, native: the tables at the settle fall from 2,541 to 2,461 MiB. The heap peak falls from 3,003 to 2,880 MiB.
- Wasm memory after growth changes by between -94 and +15 MiB per case. Telperion goes 2,693 → 2,652 MiB at seed 7 and 2,740 → 2,646 MiB at seed 42. Laurelin at seeds 7 and 42 and the spruce at seed 7 rose by 3 to 15 MiB.
- The build's final high-water for telperion does not move (2,776 and 2,777 MiB), because the wood stage sets it.
- The saving is small, as the task expected.

Byte identity (measured):
- `hash-dist.mjs` gives identical hashes on all 8 presets at seeds 1, 7 and 42.
- Each grown tree's `telperion_space::digest` and every relay's join are bit-identical to a base worktree on all 24 cases.

Tests: `grow::tests::an_axis_and_its_draws_stay_compact` pins both sizes, and it is red on the base.

Left for the host, not narrowing: about 30% of growth's tables is `Vec` capacity slack, for example 805 MiB of phytomer capacity holding 572 MiB. `Link.parent` as a sentinel would save 8 MiB but touch about 80 lines, so it was left. `Pencil` and `Phytomer` have nothing to narrow that a bound in code proves.

Note: the range from the base includes the conductor's commit 58d6f0254 (task 5 receipt), which landed in this checkout during the task.

stage: impl-review - ran [3-reviewer codex panel: SHIP with one P2 (the size test was 64-bit only), fixed in 8c706cd63 without re-review per the attended rule]

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: cd0f2bd6d3664c99001cafd2688028fd973fc571, 58d6f02540fa0ae8c0c6c13ba97c9ed1d851f196, 500f0108ba826ee967bc96241112a0d678af6101, 007fb6155065fe831ebfe9c29ed403ee47aa14bf, 8c706cd63862f36764faad02e520513d39a46c87
- Tests: cargo test --profile ci -p telperion-space --no-fail-fast, cargo test --profile ci -p telperion-core --lib pipeline::space, cargo test --profile ci -p telperion-render --lib generation::cycle, node .flow/evidence/fn-291-the-cpu-entry-builds-every-shipped/hash-dist.mjs <dist> <seed> (8 presets x seeds 1,7,42, identical)
- PRs: