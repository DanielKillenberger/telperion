---
satisfies: [R4]
---
# fn-259-the-release-meets-the-bar-on-every.3 Scorecard tooling: F4 probes that fill every protocol column, a one-build memory mode, a pixel-difference helper

## Description
R4's tools and F4's side of R1's protocol, built and validated before the proof point. Today `prepare_timing.rs` times `Generator::prepare` only (no renderer creation, no submission, no fence) and `frames.rs` has no age override and prints no device memory. The F4 probe (extending `frames.rs`, or `prepare_timing.rs` beside it) must yield every column task 4's protocol names: fresh start from renderer creation (shader compilation included) to a completed, fenced first frame; warm rebuild to a fenced frame; frame time whole/mid/close; CPU peak RSS and resident device memory; the preset, age and seed asked. It gains a one-build mode for memory-only rows, every row carries the GPU state at its start and end, and a pixel-difference helper counts differing pixels and names its metric.

**Size:** M
**Files:** `crates/telperion-render/examples/frames.rs`, `crates/telperion-render/examples/prepare_timing.rs`, a pixel helper (`scripts/pixel-diff.mjs` with `scripts/pixel-diff.test.mjs`, or a render example if PNG decoding lives only in Rust), `docs/gpu-tests.md`
**Touches:** [crates/telperion-render/examples/frames.rs, crates/telperion-render/examples/prepare_timing.rs, scripts/pixel-diff.mjs, scripts/pixel-diff.test.mjs, scripts/gpu-row.mjs, docs/gpu-tests.md]

### Approach
- Columns: `AGE` and `SEED` overrides (env, matching `SEED`/`FLOWERING`); fresh timing from `Renderer::new` through `prepare` to the first frame drawn and `device.poll(wait)` returned; warm timing as a second `prepare` plus fenced frame in the same process; device memory from wgpu's allocator report if the native backend exposes one (fn-258 BYTES.md read GPU process MB and allocated MiB), else `nvidia-smi --query-compute-apps` for this pid, the source named.
- One-build mode: a flag or env var (match each example's existing style, e.g. `SEED`, `FLOWERING`) that builds once, draws the frames memory needs (fn-258 BYTES.md: four frames at the hero pose before reading memory) and prints the same JSON keys with timing fields absent or null, never zero.
- GPU state per row: free device memory (`nvidia-smi --query-gpu=memory.free`) at the row's start, in the JSON. A row runner (`scripts/gpu-row.mjs`) wraps any probe command, either version's, with task 4's isolation: it holds `${XDG_RUNTIME_DIR}/telperion-heavy.lock` for the whole row, starts only with no other `telperion-heavy-*` unit and no other compute process on the card, samples `nvidia-smi --query-compute-apps` every 250 ms while the probe runs, and appends the probe's JSON with the foreign processes seen and `valid: false` when there were any.
- Pixel helper: counts pixels where any channel differs and where any channel differs by more than 8/255, over the whole frame, and prints both with the metric named; refuses images of different sizes by name. A test with a fixture of known count (e.g. two generated PNGs differing in exactly N pixels, some by 1/255) proves it counts pixels, not intensity.
- `docs/gpu-tests.md` gets the two probe flags and the helper in its release-check section.

### Investigation targets
**Required:**
- `crates/telperion-render/examples/frames.rs` (whole)
- `crates/telperion-render/examples/prepare_timing.rs` (whole)
- `.flow/evidence/fn-258-a-prepared-tree-meets-masters-memory/BYTES.md` - the archived probe's protocol
- `.flow/evidence/fn-258-a-prepared-tree-meets-masters-memory/FRICTION.md` - what R4 answers
**Optional:**
- `scripts/contact-sheet.py`, `scripts/compare-references.py` - existing image tooling

### Key context
GPU runs go through `telperion-heavy <mem> --gpu <vram> --threads N --`; at most one probe run per check here, preset `date-palm` or `ordinary` at 40 (small before large). The gate and `npm test` once at the end.
## Acceptance
- [ ] One-build mode measured once on one small preset: one build in the log, memory keys filled, timing keys null; its memory within the spread of the same preset's multi-build run, both numbers recorded.
- [ ] On one small preset (palm or ordinary at 40), the probe prints every protocol column: fresh-to-fenced-frame, warm-rebuild-to-fenced-frame, whole/mid/close frame ms, peak RSS, device memory with its source, age and seed as asked.
- [ ] The row runner holds the lock for the whole row (a second `telperion-heavy` job started meanwhile waits, shown once), and a row during which a foreign compute process ran is written `valid: false` (shown once with a short foreign GPU job).
- [ ] The pixel helper's test is red against an intensity-weighted metric (asserts the exact count) and green on the helper.
- [ ] Gate and `npm test` green.
## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
