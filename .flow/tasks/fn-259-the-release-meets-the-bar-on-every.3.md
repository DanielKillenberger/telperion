---
satisfies: [R4]
---
# fn-259-the-release-meets-the-bar-on-every.3 Scorecard tooling: one-build memory mode, a pixel-difference helper, free device memory per row

## Description
R4's tools, built before the sweep that uses them. The native probes (`frames` for frame time at whole/mid/close, `prepare_timing` for build time and peak RSS) gain a one-build mode for rows that only need memory, every row they print carries the free device memory read when it started, and a pixel-difference helper counts differing pixels and names its metric.

**Size:** M
**Files:** `crates/telperion-render/examples/frames.rs`, `crates/telperion-render/examples/prepare_timing.rs`, a pixel helper (`scripts/pixel-diff.mjs` with `scripts/pixel-diff.test.mjs`, or a render example if PNG decoding lives only in Rust), `docs/gpu-tests.md`
**Touches:** [crates/telperion-render/examples/frames.rs, crates/telperion-render/examples/prepare_timing.rs, scripts/pixel-diff.mjs, scripts/pixel-diff.test.mjs, docs/gpu-tests.md]

### Approach
- One-build mode: a flag or env var (match each example's existing style, e.g. `SEED`, `FLOWERING`) that builds once, draws the frames memory needs (fn-258 BYTES.md: four frames at the hero pose before reading memory) and prints the same JSON keys with timing fields absent or null, never zero.
- Free device memory: read through the adapter if wgpu exposes it natively, else `nvidia-smi --query-gpu=memory.free` at row start; the JSON names its source.
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

### Acceptance
- [ ] One-build mode measured once on one small preset: one build in the log, memory keys filled, timing keys null; its memory within the spread of the same preset's multi-build run, both numbers recorded.
- [ ] Every probe row carries free device memory at start and its source.
- [ ] The pixel helper's test is red against an intensity-weighted metric (asserts the exact count) and green on the helper.
- [ ] Gate and `npm test` green.

## Acceptance
- [ ] TBD

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
