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
The F4 probe (`frames.rs`) now fills every protocol column. That is `AGE`/`SEED` as asked, `fresh_ms` from `Renderer::new` to a fenced first frame, `warm_ms` for a rebuild to a fenced frame, whole/mid/close frame ms, `peak_rss_mb`, and `device_memory` with its source and wgpu's own share (`wgpu_internal_mib`). `ONE_BUILD=1` gives memory-only rows with the timing keys null. `scripts/gpu-row.mjs` runs a row: it holds the heavy lock for the whole row, samples the card every 250 ms and writes `valid: false` on a foreign process. `scripts/pixel-diff.mjs` counts differing pixels and names its metric. `docs/gpu-tests.md` documents all three, and the measurements are in `.flow/evidence/fn-259-the-release-meets-the-bar-on-every/TOOLING.md`.

- Host decision (2026-10-19): foreign means rule (a). A foreign process appears on the card during the row, was not there at the start and is not in the probe's tree. A row starts only with no other heavy unit and no telperion process on the card. Processes present at the start are recorded, and a non-Telperion compute job already running at the start is the stated residual gap.
- Ordinary at 40, seed 1 (measured): fresh 2,585.5 ms, warm 366.0 ms, frame 0.555/0.733/0.881 ms, peak RSS 797 MB, 291.8 MiB allocated (196.5 of it wgpu-internal), 1,024 MiB reserved.
- One-build against multi-build (measured): allocated less wgpu-internal is 90.6 against 95.3 MiB. Raw allocated is 97.1 against 291.8, because wgpu's indirect-validation buffers grow with the timing loop. Reserved is 768 against 1,024 MiB, and peak RSS 691 against 797 MB. Both `allocated_mib` and `wgpu_internal_mib` are reported, and task 4's protocol chooses the column (host).
- Row runner (measured): a `telperion-heavy` job started mid-row was admitted the moment the row ended. A row with a short foreign `frames date-palm` run under plain nice was written `valid: false`, naming that pid. Two rows admitted at once both finished, one after the other.
- Pixel helper: its test is green; it is red (5.56 against 37) with an intensity-weighted count.

baseline: green via handoff (1a0844c60)
gate: telperion-heavy 12 --threads 2 -- cargo test --profile ci --workspace --no-fail-fast exit 0, 1,401 passed, 0 failed, 111 ignored, 146 binaries; npm test exit 0, 173 passed
stage: impl-review - ran (codex, three lenses, then two re-reviews to SHIP; correctness draw lost to model capacity in round 1; round cap reset once, as the host's brief allows, recorded in FRICTION.md; finding "existing compute workers exempt" declined by the host decision above)
Tier: (none given) (model: claude-opus-5-5)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 0f4b66f78812e85e61183298370d20ed96826238, 7acc5e81752be72fcd2724ddc1d3495d552f0129, 5a142e0012f3ea9a8a621ff63c3c678e7033d928, ce41b2b6fcedd09b93c9a8189612e8e7b09dd8cf, 35bfa33df4fd583c831c0fece85472a4044d3a8a
- Tests: telperion-heavy 12 --threads 2 -- cargo test --profile ci --workspace --no-fail-fast, npm test, npx vitest run scripts/pixel-diff.test.mjs
- PRs: