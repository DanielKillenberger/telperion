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
TBD

## Evidence
- Commits:
- Tests:
- PRs:
