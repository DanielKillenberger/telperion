---
satisfies: [R8c, R4]
---
# fn-257-the-site-grows-its-trees-on-the-device.15 The spruce's site field and held memory, paired against today; gate and npm test

## Description
R8c's proof and the gate.

**Size:** S
**Files:** `.flow/evidence/fn-257-the-site-grows-its-trees-on-the-device/R8.md`
**Touches:** [.flow/evidence/fn-257-the-site-grows-its-trees-on-the-device/**]

### Approach
- Paired warm (16 interleaved): the spruce's site field (grow + 36³ grid) at the release tip before R8 against after; oak and birch too, seeds 1, 1234, 5678, against 0.1.4 (RESULTS.md's table), naming any species still over.
- Held memory per tree after the read-out (the grid only) against RESULTS.md's field MB.
- Gate once: `telperion-heavy 12 --threads 2 -- cargo test --profile ci --workspace --no-fail-fast`; npm test.

## Acceptance
- [ ] R8.md: paired spruce site field before → after, oak and birch against 0.1.4 per seed, held memory per tree.
- [ ] Gate and npm test green, counts in the done summary.

## Done summary
R8c is measured in `.flow/evidence/fn-257-the-site-grows-its-trees-on-the-device/R8.md`, from 16 interleaved warm pairs. The before side is the release tip 23fb243e3, the commit this branch merged at 4b371f99c, so the pair differs only by tasks 13 and 14 (worker's choice of commit, reason in R8.md). The after side is d9193b9ce. Both are grow plus a 36³ read-out. The method is `site-time.sh` with a scratch `site_time` test applied in two disposable worktrees, each with its own target dir (the patches are kept beside the script). Raw lines are in `raw/t15/` (gitignored).

- Spruce site field (paired median): seed 1 501.2 → 433.2 ms (−67.7), seed 1234 538.7 → 440.4 (−97.5), seed 5678 518.5 → 432.2 (−87.8). After was faster in 16 of 16 pairs at every seed.
- Oak against 0.1.4 (290/273/332 ms), after R8: 246.1 / 285.6 / 258.6 ms. **Seed 1234 is still 12.6 ms over 0.1.4.** In the same pairs it was 31.5 over before R8, and FRICTION.md's busy-desk reading was about 65 over. Reported only, not decided.
- Birch against 0.1.4 (292/352/354 ms): 202.6 / 223.2 / 204.2 ms, under at every seed.
- Still over 0.1.4: the oak at seed 1234, and the spruce at every seed (about +190-200 ms against about 240, and against the owner's "about 0.35 s" bar).
- Held on the device after the read-out: 0 B for every species and seed. The release tip held 6.3-58.5 MB (spruce 56.5-58.5). RESULTS.md's older field MB column was 12.1-102.3. The grid on the host is 0.75 MB in the package's form at 36³.
- Desk load: no other process on the GPU (777 MiB used, desktop only). The CPU's 1-minute load fell from 5.0 to 1.9 while another worktree's gate compiled. A smoke run 3 minutes earlier, under that gate's 19-core test, doubled both sides and is not used.

Gate (run once, at the end, output in `raw/t15/gate.txt`): `telperion-heavy 12 --threads 2 -- cargo test --profile ci --workspace --no-fail-fast` exited 0, with 149 test binaries, 1,417 passed, 0 failed and 109 ignored.
npm test (`raw/t15/npm-test.txt`): exited 0, with 19 test files and 170 tests passed.
baseline: none (evidence-only task; the gate is the end-of-task run).

Follow-up (outside the Touches): `field_time` could take `FIELD_SEEDS` and print the ledger's live bytes, so a later pairing needs no scratch patch (FRICTION.md, 2026-10-18). On the release tip, the ledger keeps 2.24 MB after the field is dropped; the cause was not measured, and R8 removes that path.

stage: impl-review - ran (codex, one correctness reviewer, SHIP. Its one P2 was site-time.sh's missing output directory and swallowed failures, fixed in d02d87599 after finalize and checked with `bash -n`)
Tier: (none given) (model: claude-opus-5-5)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 71d02e497074a915f9ca2e7e1a985146c531030b, d02d8759917411108c4bd26a6f85a9fca2e22f61
- Tests: telperion-heavy 12 --threads 2 -- cargo test --profile ci --workspace --no-fail-fast (1417 passed, 0 failed, 109 ignored), npm test (170 passed), site-time.sh 16 interleaved pairs, 23fb243e3 vs d9193b9ce
- PRs: