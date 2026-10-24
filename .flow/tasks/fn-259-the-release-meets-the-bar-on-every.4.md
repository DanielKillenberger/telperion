---
satisfies: [R1]
---
# fn-259-the-release-meets-the-bar-on-every.4 The scorecard's protocol and 0.1.4 beside F4 on one preset

## Description
The proof point: write `PROTOCOL.md` before any measurement, then measure one preset (the oak at 40, a species both versions ship) on F4 and on 0.1.4 with every R1 column filled. 0.1.4 is tag `v0.1.4` (`c48a4f28a`); its renderer API differs from F4's, so the 0.1.4 side needs its own probe adapter in a scratch worktree, never committed to 0.1.4 and never sharing a `target/`.

**Size:** M
**Files:** `.flow/evidence/fn-259-the-release-meets-the-bar-on-every/PROTOCOL.md`, `.../SCORECARD.md`, raw rows under `.../raw/` (ignored); the 0.1.4 adapter as a patch file kept in evidence (`v014-probe.patch`)
**Touches:** [.flow/evidence/fn-259-the-release-meets-the-bar-on-every/PROTOCOL.md, .flow/evidence/fn-259-the-release-meets-the-bar-on-every/SCORECARD.md, .flow/evidence/fn-259-the-release-meets-the-bar-on-every/v014-probe.patch, .flow/evidence/fn-259-the-release-meets-the-bar-on-every/FRICTION.md]

### Approach
- PROTOCOL.md states, before measuring: artifact identities (F4 commit, 0.1.4 tag, the `.wasm`/`.js` sha256 of each); hardware (RTX 3080, driver, CPU, RAM); presets and which ship in each version; seeds (1, 7, 42); viewport 1920×1080 and MSAA as `frames.rs`; poses whole/mid/close; product default age per version and the matched-age diagnostic rows, reported apart; fresh start (shader compilation from renderer creation) apart from warm rebuilds; start and rebuild timed to a completed, fenced frame; the field call timed to the return of the CPU field and query answers, preparation and voxelisation in, no renderer (fn-257's `field-time.sh` and R8.md method); sample count and aggregate (median and range); a run that fails, is refused or is invalid is a row marked so, never a pass; free device memory and machine load per row; which rows are covered by an owner tolerance (spec, "Owner decisions") and that every other worse row is a regression.
- Download bytes: the shipped package artifacts raw/gzip/brotli, both versions (`npm pack` of each, or `dist/` after `npm run build`).
- 0.1.4 side: `git worktree add` a detached scratch checkout outside the repo's worktree set (scratchpad), its own `CARGO_TARGET_DIR`, an adapter example that measures the same columns on 0.1.4's API as task 3's F4 probe does on F4's. Record the patch in evidence.
- Run the oak row on both. Per-species progress file in `raw/` so a stopped sweep resumes.
- GPU isolation (R4), for the whole row and both versions' probes: (1) exclusion: the row holds `${XDG_RUNTIME_DIR}/telperion-heavy.lock` (the lock every `telperion-heavy` job takes to be admitted; AGENTS.md routes every heavy job through it) from before its first sample to after its last, so no new heavy job starts during it, and it starts only when no other `telperion-heavy-*` unit is running and no other compute process is on the card; (2) detection: a watcher samples `nvidia-smi --query-compute-apps` every 250 ms for the whole row and the row's JSON lists every foreign process seen; a row with any is invalid, rerun once, then recorded invalid. A GPU user outside `telperion-heavy` that lives under 250 ms is the residual the protocol names. Graphics clients (browser, desktop) are recorded with their memory and utilisation, not killed. Free device memory is a separate column.
- A row that takes over 10 minutes, or a card that stays shared for 30 minutes, is a FRICTION.md entry and a stop with NEEDS_HUMAN, not a wait.

### Investigation targets
**Required:**
- `.flow/evidence/fn-258-a-prepared-tree-meets-masters-memory/BYTES.md`, `RESULT.md` - the last protocol and its columns
- `.flow/evidence/fn-257-the-site-grows-its-trees-on-the-device/field-time.sh`, `R8.md`, `RESULTS.md` - the per-tree field method and 0.1.4's numbers
- `.flow/evidence/fn-259-the-release-meets-the-bar-on-every/RELEASE.md` - the 2026-10-15 partial scorecard
- `crates/telperion-core/presets/*.values` - `/age =` default ages
**Optional:**
- `.flow/evidence/fn-245-a-breakthrough-in-generation-time/ASTRA-ARCH.md` §6 - the warnings

### Key context
Native wgpu only; never WebGPU in a browser. AGENTS.md budget: read summaries, at most four images per capture. No gate needed (evidence only) unless code changed.
## Acceptance
- [ ] PROTOCOL.md committed before the first measured row (commit order shows it).
- [ ] The oak row on F4 and 0.1.4: build time warm and fresh, frame time whole/mid/close, resident memory (CPU peak and device), per-tree field call, download bytes; each with samples and aggregate.
- [ ] The 0.1.4 adapter kept as a patch in evidence; raw rows in `raw/`; progress file present.
- [ ] If 0.1.4 cannot be measured on the same protocol, the gap is named and the task stops with NEEDS_HUMAN.
## Done summary
`PROTOCOL.md` was committed (`dd09b47b9`) before the first measured row. `SCORECARD.md` holds the Oregon white oak on F4 beside 0.1.4, at seeds 1, 7 and 42, with every R1 column filled. The 0.1.4 adapter is kept as `v014-probe.patch`, and F4's per-tree field probe as `f4-field-probe.patch` (never committed to crates). Raw rows and the progress file are in `raw/` (ignored).

- **0.1.4 measured on the same protocol (measured).** Its adapter is a detached scratch worktree of `v0.1.4` with its own target directory. Both sides use `--profile ci`, Rust 1.98.1 and wgpu 30.0.1, on the harness route (`setTreeGpu`). The scratch worktree has been removed.
- **Regressions at the default age, every seed (measured).**
  - Fresh build is +1.83-1.95 s: 4.45-4.57 s against 2.60-2.62 s, cold shader compile included.
  - CPU peak is +462-484 MB: 773-783 MB against 299-313 MB.
  - The per-tree field call is +57-82 ms: 230-253 ms against 151-189 ms.
  - Downloads: the gzip and brotli growth of both Wasm modules and `wasm-source.js`, the package total (+296,158 brotli) and the tarball (+422,936).
- **Covered by owner tolerances (measured).** The warm build is +79-114 ms but under the 0.5 s budget, and every artifact's raw size is under its ceiling.
- **No worse (measured).** Frames at whole, mid and close are 2.4-10.4 ms against 15.6-33.7 ms. Device memory, allocated less wgpu-internal, is 137-163 MiB against 319-356 MiB.
- **Matched-age diagnostic (measured).** F4 refuses the oak at age 100 at every seed: `Oversize { buffer: "cycle state axes", 2558845952 > 2147483644 }`. fn-258 drew it on 2026-10-10.
- **Open questions:**
  - For the owner: do the raw ceilings also cover compressed growth?
  - For the host, before task 5: the European beech is not in 0.1.4's package catalogue (`IN_WORK`). Should F4's beech be set beside 0.1.4's unshipped research preset, or reported alone?
- **Follow-up, from review P2, not fixed:** the runner bounds a whole job at 40 minutes, not admission at 30. `gpu-row.mjs` has no deadline for its wait for a quiet card.
- **Review fixes:**
  - The first 0.1.4 field probe had been built together with `telperion-render`, so cargo feature unification gave the core its `geometry` feature. It was rebuilt alone and the field rows were rerun; the superseded rows are not counted.
  - F4's field start now includes `prepare_programs`.
- **Friction:** one FRICTION.md entry, for a memory cap the desk could not admit.

baseline: none (evidence-only task; no Quick command covers `.flow/evidence`)
Tier: (none given) (model: claude-opus-5-5)

stage: impl-review - ran (codex, three lenses NEEDS_WORK with 4 kept findings, fixes in e27b07876 and f4d36d787, re-review SHIP; 1 P2 left as follow-up)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: dd09b47b9321868dc33065945d5fb5c306d6f577, 1960622035322d4d5a0ed3b79f29752d35ed435f, 5c76b516a7aba1c0cfb03d761a5c1001284bd616, e27b07876ebf6c7afa302dab3a637d6bc9f8b12d, f4d36d7875e5d424ddfb24835866a06d816ef1c8, 0ac851b5c2cb71d6bb4c9178b447ccfa51efab05
- Tests: GATE_SKIPPED:fmt:docs-only - cumulative diff classified tier-B (no executable paths touched), GATE_SKIPPED:bindings:docs-only - cumulative diff classified tier-B (no executable paths touched), GATE_SKIPPED:npm-test:docs-only - cumulative diff classified tier-B (no executable paths touched), GATE_SKIPPED:workspace-test:docs-only - cumulative diff classified tier-B (no executable paths touched)
- PRs: