---
satisfies: [R1, R4]
---
# fn-259-the-release-meets-the-bar-on-every.5 The scorecard swept over every shipped species

## Description
Task 4's protocol over every shipped preset and seed, at the product default ages and at matched ages, F4 and 0.1.4, with one-build rows where only memory is wanted. Ends in `SCORECARD.md` with a regression list: every row worse than 0.1.4 that no owner tolerance covers.

**Size:** M
**Files:** `.flow/evidence/fn-259-the-release-meets-the-bar-on-every/SCORECARD.md`, `raw/`
**Touches:** [.flow/evidence/fn-259-the-release-meets-the-bar-on-every/SCORECARD.md, .flow/evidence/fn-259-the-release-meets-the-bar-on-every/FRICTION.md]

### Approach
- Run per species in the order small to large (palm, ordinary, beech, oak, birch, spruce, Laurelin, Telperion), each under `telperion-heavy <mem> --gpu <vram>` with free device memory recorded; resume from the progress file.
- A species 0.1.4 does not ship is reported F4-only with that reason.
- Tables: default ages; matched ages (diagnostic); per-tree field; download bytes. Each cell median [range] over the stated samples; failed rows marked.
- Regression list: row, measured gap (absolute and ratio), and whether an owner tolerance covers it, naming the tolerance; no other tolerance.

### Investigation targets
**Required:**
- `.flow/evidence/fn-259-the-release-meets-the-bar-on-every/PROTOCOL.md` (task 4)
- `.flow/specs/fn-237-release-the-tree-space-engine-replaces.md` - R0a and "The owner's release decisions (2026-10-16)"

### Key context
This is the capture budget's large run: one sweep, not repeated; a defect found mid-sweep is reported, not fixed (qualification only). Stop with FRICTION.md and NEEDS_HUMAN if a row exceeds 10 minutes or the GPU is held by another session.
## Acceptance
- [ ] Every shipped preset at seeds 1, 7, 42 at its default age on F4 and 0.1.4 (where shipped), plus matched-age rows, every R1 column filled or marked failed with the reason.
- [ ] Free device memory and other compute processes per row, contaminated rows rerun or marked invalid; one-build rows used for memory-only cells.
- [ ] The regression list names each worse row, its gap and the owner tolerance that covers it or "none".
## Done summary
`SCORECARD.md` now covers every shipped preset under task 4's protocol (amendment 5, `35b2d3a2b`, committed before the first task-5 row). The palm, ordinary, birch, spruce, Laurelin and Telperion are set beside 0.1.4 at seeds 1, 7 and 42 at their default ages. The European beech is reported F4-only (host decision: 0.1.4 did not ship it). The oak is task 4's. Matched-age diagnostics are reported apart, and the scorecard ends in a regression list giving each gap, its ratio, and the owner tolerance that covers it or "none".

- **Sweep (measured):** 441 rows over 85 minutes, 398 valid. Every default-age job has its full sample count. One row was invalid, a GNO process that appeared on the card, and its rerun was valid. Free device memory at row start was 7,282-8,477 MiB. No stops.
- **Probes:** the four binaries task 4 pinned, sha256 rechecked. F4's crates and scripts are unchanged since `d59dac4e7`, so no 0.1.4 worktree was rebuilt. This departs from the host note's "recreate the worktree"; the reason is in amendment 5.
- **Regressions with no tolerance (measured):**
  - fresh build on every preset, +1.7-2.8 s;
  - CPU peak on every preset, +428-543 MB;
  - the field call on every preset but the birch, where it is seed 42 only, +15 ms;
  - frames: the ordinary +0.05-0.48 ms; Laurelin and Telperion +2.7-26.6 ms at every view;
  - device memory: the palm +2 MiB, the ordinary +2-12 MiB, Laurelin +162-289 MiB, Telperion +66-369 MiB;
  - warm builds above the owner's figures: Laurelin 1,346 ms at seed 42, Telperion 1,356 ms and 1,413 ms at seeds 7 and 42;
  - the package's compressed download growth.
- **Matched age (measured):** the palm and the ordinary at 100 and the birch at 70 drew. The spruce at 100 was refused with `Oversize cycle state axes`. Laurelin and Telperion at 100 were refused by the 20,000,000-phytomer budget. Each refusal is recorded as a row, not fixed.
- **Not like for like (measured):** 0.1.4's palm took its CPU fallback, and 0.1.4's Telperion at seeds 7 and 42 is a small tree.
- **Open questions for the owner, in the scorecard:**
  1. Does "about 1.25 s" cover Laurelin's 1,272 ms at seed 7?
  2. Does the spruce field tolerance reach 0.42 s against 0.15 s natively?
  3. Do the raw ceilings cover compressed growth?
- **Friction:** one FRICTION.md entry. A refused row keeps no error text, and gpu-row has no admission deadline, so the runner carried its own 30-minute watchdog.

baseline: none (evidence-only task; no Quick command covers `.flow/evidence`)
GATE_SKIPPED:cargo-fmt:docs-only - cumulative diff classified tier-B (no executable paths touched)
GATE_SKIPPED:bindings:docs-only - cumulative diff classified tier-B (no executable paths touched)
GATE_SKIPPED:npm-test:docs-only - cumulative diff classified tier-B (no executable paths touched)
GATE_SKIPPED:cargo-test:docs-only - cumulative diff classified tier-B (no executable paths touched)
Tier: (none given) (model: claude-opus-5-5)

stage: impl-review - ran (codex route; triage-skip docs-only (3 files), VERDICT=SHIP, no reviewer draw)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 35b2d3a2b5e7f427fb6438181bb25854ac52f869, 88e8ae4c08880b93092905fd57a3fa542d3cfeb9
- Tests: GATE_SKIPPED:cargo-fmt:docs-only - cumulative diff classified tier-B (no executable paths touched), GATE_SKIPPED:bindings:docs-only - cumulative diff classified tier-B (no executable paths touched), GATE_SKIPPED:npm-test:docs-only - cumulative diff classified tier-B (no executable paths touched), GATE_SKIPPED:cargo-test:docs-only - cumulative diff classified tier-B (no executable paths touched)
- PRs: