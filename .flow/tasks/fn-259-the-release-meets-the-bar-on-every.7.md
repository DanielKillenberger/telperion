---
satisfies: [R3]
---
# fn-259-the-release-meets-the-bar-on-every.7 Old-versus-new stills and the regression ledger for the owner

## Description
R3's packet: stills of every shipped species from 0.1.4 and F4 at the same pose and seed (default ages, and matched age where they differ, labelled), the regression ledger from task 5 and task 6 (each item, its measured size, the fidelity gain offered for it), and the questions the owner answers. Ends with NEEDS_HUMAN carrying the exact questions; the owner's words are recorded when they come.

**Size:** M
**Files:** `crates/telperion-render/examples/resident_stills.rs`, `.flow/evidence/fn-259-the-release-meets-the-bar-on-every/VERDICT.md`, compressed JPEG sheets beside it (docs/evidence-retention.md: sheets an owner verdict rests on are kept outside `raw/`)
**Touches:** [crates/telperion-render/examples/resident_stills.rs, .flow/evidence/fn-259-the-release-meets-the-bar-on-every/VERDICT.md, .flow/evidence/fn-259-the-release-meets-the-bar-on-every/sheets/**]

### Approach
- F4 stills from the shipped resident device path only (`crates/telperion-render/examples/resident_stills.rs`, `Generator::prepare`), never `headless.rs` (it draws the CPU `mesh::build` reference). `resident_stills.rs` takes default age and seed only: add `AGE`/`SEED` overrides and use the hero pose from the tree's bounds, which both versions compute, so pairs match; the 0.1.4 side through task 4's adapter. Sheets with `scripts/contact-sheet.py`.
- The pixel helper (task 3) states how far each pair differs; the agent views at most four images per capture and makes no visual judgment of its own.
- Ledger: one row per regression (download included), its size from SCORECARD.md/AUDIT.md, and the fidelity gain offered named from recorded evidence (the owner's 2026-10-15 species verdicts, species specs), never invented.
- The owner's 2026-10-15 verdicts (fn-237) are cited; the question asked is old-versus-new per species and per ledger item.

### Investigation targets
**Required:**
- `.flow/specs/fn-237-release-the-tree-space-engine-replaces.md` - the owner's species verdicts table
- `docs/evidence-retention.md`
- `scripts/contact-sheet.py`

### Key context
The verdict is the owner's; the task is done when the packet is complete and the questions are asked. Its done summary carries NEEDS_HUMAN with the exact questions unless the owner has answered.
## Acceptance
- [ ] A sheet per shipped species, 0.1.4 beside F4 (F4 from the resident device path), same pose and seed, labelled with version, age and seed.
- [ ] The ledger lists every regression with its measured size and the evidence-cited fidelity gain.
- [ ] VERDICT.md ends with the owner's questions, one per species and per ledger item, and a place for the answers.
## Done summary
Task 7's packet for R3 is complete. It has eleven old-versus-new sheets in `sheets/`: seven species with 0.1.4 beside F4 at the default age and seeds 1 / 7 / 42; matched-age diagnostics for the palm and the ordinary at 100 and the birch at 70; and the beech as F4 alone. `VERDICT.md` holds the pixel-helper counts, the regression ledger L1-L24 with sizes and recorded fidelity gains, and 35 owner questions, each with an answer slot. F4's stills come from `resident_stills.rs` on the resident device path, with the `SEED`/`AGE` overrides this task added. 0.1.4's stills come from a scratch adapter (`raw/t7/v014-resident_stills.rs`) on the `frames` route of task 4's patch. The host widening is done: SCORECARD.md's download rows now give each artifact's gap and ratio for raw, gzip and brotli, and the no-worse rows keep `[min, max] (n)` per seed.

- Matched age (measured): the oak and the spruce at 100 were refused with an oversize "cycle state axes" buffer, and Laurelin and Telperion at 100 by the 20,000,000-phytomer budget. Recorded as ledger item L24.
- Friction: one entry. `scripts/contact-sheet.py` is a reviewer, not a compositor, so the sheets were made with ImageMagick from a scratch script.
- Gates: `telperion-heavy 12 --threads 2 -- cargo test --profile ci --workspace --no-fail-fast` was green: 1,401 passed, 0 failed, 111 ignored. `npm test` passed 173. `cargo fmt --all --check` was clean.

baseline: green via handoff (fn-259.3 at d59dac4e7; only .flow/ paths changed since)
Tier: not provided by the conductor (model: claude-opus-5-5)

stage: impl-review - ran (codex gpt-6.1-sol, one correctness draw, SHIP, no findings)

NEEDS_HUMAN: the owner's verdicts. Answers are recorded in VERDICT.md:
1. **T1.** Does R0a's "about 1.25 s" for Laurelin cover Laurelin's 1,272 ms warm build at seed 7? And does anything cover Laurelin's 1,346 ms at seed 42, and Telperion's 1,356 ms and 1,413 ms at seeds 7 and 42 (the owner accepted 1.33 s at seed 42 on 2026-10-16)?
2. **T2.** Does the spruce field tolerance ("about 0.35 s against 0.24 s", 2026-10-16) reach F4's 0.42 s against 0.1.4's 0.15 s measured natively (426 / 416 / 421 ms against 152 / 143 / 147 ms)?
3. **T3.** Do the raw-byte artifact ceilings raised on 2026-10-16 also cover the gzip and brotli growth: `telperion.wasm` +57,753 / +44,333 B, `telperion-render.wasm` +472,729 / +353,793 B, `wasm-source.js` +15,356 / +13,040 B, total +409,828 / +296,158 B?
4. **S1, ordinary** (`sheets/ordinary.jpg`, `sheets/ordinary-matched-age-100.jpg`). Is F4 at 40 at least as good as 0.1.4?
5. **S2, date palm** (`sheets/date-palm.jpg`, `sheets/date-palm-matched-age-100.jpg`; 0.1.4's palm is a CPU mesh). Is F4 at 40 at least as good as 0.1.4?
6. **S3, Oregon white oak** (`sheets/oregon-white-oak.jpg`; F4 at 100 refused). Is F4 at 40 at least as good as 0.1.4? Seed 7 is the release item of 2026-10-15.
7. **S4, silver birch** (`sheets/silver-birch.jpg`, `sheets/silver-birch-matched-age-70.jpg`). Is F4 at 40 at least as good as 0.1.4?
8. **S5, Norway spruce** (`sheets/norway-spruce.jpg`; F4 at 100 refused). Is F4 at 40 at least as good as 0.1.4?
9. **S6, Laurelin** (`sheets/laurelin.jpg`; F4 at 100 refused). Is F4 at 80 at least as good as 0.1.4?
10. **S7, Telperion** (`sheets/telperion.jpg`; F4 at 100 refused; 0.1.4's Telperion at seeds 7 and 42 is small). Is F4 at 82 at least as good as 0.1.4?
11. **S8, European beech** (`sheets/european-beech.jpg`, F4 only). Does F4's beech at 40 ship in 0.2.0 with no 0.1.4 to compare against (2026-10-15: "needs work, after the release")?
12. **L1.** Fresh build +1.7-2.8 s on every preset: accepted?
13. **L2.** Laurelin warm build 1,272 ms at seed 7 (see T1): accepted?
14. **L3.** Laurelin warm build 1,346 ms at seed 42: accepted?
15. **L4.** Telperion warm build 1,356 ms and 1,413 ms at seeds 7 and 42: accepted?
16. **L5.** Ordinary frames +0.05-0.48 ms (×1.10-1.77): accepted?
17. **L6.** Laurelin frames +4.3-26.6 ms (×2.50-13.96): accepted?
18. **L7.** Telperion frames +2.7-21.7 ms (×2.00-45.38): accepted?
19. **L8.** CPU peak +428-543 MB on every preset: accepted?
20. **L9.** Palm device memory +2 MiB: accepted?
21. **L10.** Ordinary device memory +2.4 / +11.8 MiB at seeds 7 / 42: accepted?
22. **L11.** Laurelin device memory +162-289 MiB: accepted?
23. **L12.** Telperion device memory +66-369 MiB: accepted?
24. **L13.** Palm field call +72-79 ms (×7.5-8.0): accepted?
25. **L14.** Ordinary field call +314-343 ms (×6.5-7.4): accepted?
26. **L15.** Oak field call +57-82 ms (×1.32-1.55): accepted?
27. **L16.** Birch field call +15 ms at seed 42: accepted?
28. **L17.** Spruce field call +273-274 ms (see T2): accepted?
29. **L18.** Laurelin field call +683-839 ms: accepted?
30. **L19.** Telperion field call +427-1,204 ms: accepted?
31. **L20.** `telperion.wasm` gzip +57,753 / brotli +44,333 B (see T3): accepted?
32. **L21.** `telperion-render.wasm` gzip +472,729 / brotli +353,793 B: accepted?
33. **L22.** `wasm-source.js` gzip +15,356 / brotli +13,040 B: accepted?
34. **L23.** Package total +409,828 B gzip / +296,158 B brotli, tarball +422,936 B: accepted?
35. **L24.** F4 refuses the oak, spruce, Laurelin and Telperion at age 100, where 0.1.4 draws a mature tree: accepted for 0.2.0, with fn-285 after it?

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 704ed20d53216f1350a091af8d1b4f2dfd1235b7
- Tests: telperion-heavy 12 --threads 2 -- cargo test --profile ci --workspace --no-fail-fast, npm test, cargo fmt --all --check
- PRs: