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
TBD

## Evidence
- Commits:
- Tests:
- PRs:
