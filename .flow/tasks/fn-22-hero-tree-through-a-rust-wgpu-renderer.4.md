---
satisfies: [R5, R7]
---
# fn-22-hero-tree-through-a-rust-wgpu-renderer.4 GPU timing session and native evidence

## Description
Rewrite the prototype's timing idea as `timing.rs`: timestamp pairs around the vegetation pass, conditioning, warmup and measured samples, p50 and p95, and a verdict that can never read as a pass when the measurement is bad. Wire it into the headless example and record the native evidence and hero stills for oak and spruce.

**Size:** M
**Files:** `crates/telperion-render/src/timing.rs`, `src/lib.rs`, `crates/telperion-render/examples/headless.rs`, `crates/telperion-render/tests/timing.rs`, `.flow/evidence/fn22/oak-hero.png`, `.flow/evidence/fn22/spruce-hero.png`, `.flow/evidence/fn22/oak-native-timing.json`, `.flow/evidence/fn22/spruce-native-timing.json`, `.flow/evidence/fn22/REPORT.md`
**Touches:** [crates/telperion-render/**, .flow/evidence/fn22/**]

### Approach
- Pass-level timestamps only (`RenderPassDescriptor.timestamp_writes` with a two-entry `QuerySet`), because the inside-encoder and inside-pass features are native-only and the same code must run in the browser in task 5. Resolve into a buffer, copy to a `MAP_READ` buffer, convert with `queue.get_timestamp_period()`.
- `Session` constants named: conditioning 8, warmup 8, measured 120 (the prototype's shape). `Verdict` enum: `Valid`, `Unavailable(reason)` (no timestamp feature, or a software adapter), `Disjoint` (non-finite or non-monotonic sample), `Contended` (p95 above twice p50). `Report { adapter, backend, samples, p50_ms, p95_ms, verdict, conditioning, warmup, measured }` serialised as JSON; when the verdict is not `Valid`, the percentile fields are absent, not zero.
- Per-frame stats carry vegetation GPU ms with a validity flag when timing is on.
- `examples/headless.rs --timing <json>` runs the session after the still and writes the report; the report also records the adapter name and driver from `AdapterInfo`.
- `tests/timing.rs`: verdict logic unit-tested on synthetic sample arrays (monotonic, a NaN sample, a spread beyond the contention ratio); a device-backed session on the ordinary preset yields `Valid` or `Unavailable`, never percentiles with a non-valid verdict; skip without adapter.
- Evidence: run the headless example for oak and spruce at each preset's own seed, 1600×1000, stills at the hero pose; write `REPORT.md` with one table (species, adapter, p50, p95, verdict) and an `## Owner verdict` section with one empty slot per species for R7. View at most the two stills.

### Investigation targets
**Required** (read before coding):
- `crates/telperion-render/src/device.rs` — where `TIMESTAMP_QUERY` was requested in task 2
- `.flow/evidence/fn12/REPORT.md` — the repo's evidence report shape to mirror
- wgpu `timestamp_queries` example in the wgpu repository (features examples) — the query set, resolve and readback shape

**Optional** (reference as needed):
- `harness/stage.ts:140-220` — the harness's existing sweep vocabulary (median, unsupported reported distinctly)

### Key context
- `Device::poll` is a no-op on the WebGPU backend; keep readback callback-driven (`map_async` then poll on native, await on web) so task 5 reuses `timing.rs` unchanged.
- Percentiles are over the measured frames only; conditioning and warmup frames are never counted.

## Acceptance
- [ ] `tests/timing.rs` covers the four verdicts on synthetic data and the device-backed session never reports percentiles with a non-valid verdict; skips without adapter
- [ ] `--timing` writes a JSON report with adapter, backend, sample counts, p50, p95 and verdict; an unavailable feature yields `Unavailable` with the reason
- [ ] `.flow/evidence/fn22/` holds the oak and spruce hero stills, both native timing reports and `REPORT.md` with the numbers, their verdicts and empty owner-verdict slots
- [ ] `cargo test --release -p telperion-render` passes; clippy clean; every file under 400 lines

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
