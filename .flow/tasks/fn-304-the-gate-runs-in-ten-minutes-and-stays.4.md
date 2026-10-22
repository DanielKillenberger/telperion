---
satisfies: [R7]
---
# fn-304-the-gate-runs-in-ten-minutes-and-stays.4 The runner and the width: measure, declare, rewrite the gate line

## Description
R7 on the shrunk suite: `cargo test` and nextest with groups at `--threads 2`, `4` and `8` inside `telperion-heavy 8`, wall and peak each; the fastest passing inside 8 GB is declared. The groups move into the checked-in nextest config.

**Size:** M
**Files:** `.config/nextest.toml`, `AGENTS.md`, RESULT.md
**Touches:** [.config/nextest.toml, AGENTS.md, .flow/evidence/fn-304-the-gate-runs-in-ten-minutes-and-stays/**]

### Approach
- Start from R1's measured groups (`raw/nextest-groups.toml`): tests over about 3 GB in one one-at-a-time group, `ground_reach` (if still large) by `threads-required`, render GPU tests one at a time. Recompute membership from a per-test RSS run at each width (RSS rises with rayon width).
- A named nextest profile (for example `[profile.gate]`) holds `test-threads` for the declared width so the checked-in profile reproduces the invocation; nextest ignores `RUST_TEST_THREADS`. CI's `cargo nextest run` keeps the default profile unless R8 needs it.
- `.config/nextest.toml` feeds `scripts/ci-key.mjs`; editing it re-keys every CI receipt, which is expected.
- Nextest runs no doc-tests; the workspace has none today. If nextest is declared, the gate command adds `cargo test --doc --workspace` or states that there are none and a check holds it.
- Rewrite `AGENTS.md:39` with the invocation, the counts and the wall time, and settle 8 GB against 12 GB.
- The budgets of task .5 need a process per test (nextest's slow-timeout and a per-process RSS). If `cargo test` is the faster runner inside 8 GB, report both figures to the host and stop before .5: libtest has no per-test timeout or memory measure without `RUSTC_BOOTSTRAP`, which breaks the trybuild tests (fn-271).
- Heavy-chain probes at threads 4 and 8 were started in R1's session (`raw/heavy-t4*`, `raw/heavy-t8*`); reuse them as the pre-shrink comparison.

### Investigation targets
**Required:**
- `.config/nextest.toml`, `.github/actions/nextest/action.yml`, `scripts/ci-key.mjs:15-20`
- `AGENTS.md:37-41`
- `.flow/evidence/fn-304-the-gate-runs-in-ten-minutes-and-stays/R1.md`
## Acceptance
- [ ] Six measurements (two runners, three widths) with wall, peak and pass/kill in RESULT.md.
- [ ] The fastest passing inside 8 GB declared; groups and width checked in.
- [ ] AGENTS.md's gate line rewritten with its measurement.
- [ ] Codex impl review.
## Done summary
R7 measured and declared. Both runners at threads 2, 4 and 8 inside telperion-heavy 8: cargo test 1,971 / 1,538 / 1,494 s; nextest with groups 1,448 / 1,241 / 1,271 s; memory tokens left device tests unscheduled and are not declared. Declared (host): telperion-heavy 8 --threads 4 -- python3 scripts/gate.py, nextest gate profile, median 1,295 s (1,241-1,348), peak 5.3 GiB; 8 GB settled. AGENTS.md gate line rewritten. R5 (ten minutes) not met: the heavy tests take one turn, about 661 + 421 = 1,080 s; follow-up spec by the host. gate.py now also fails on a doc-test the gate would not run.
stage: impl-review - ran (codex, one correctness reviewer, SHIP; two P2s fixed: device-group comment, doc-test check) (model: gpt-6.1-sol)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: af1a97f99028faf34361e955c64523eb9a8dd08f, 438e30b82d578bd5edd4dc4e1233e8dd992b1cb1, fb0c80de2501cbacbd42c9b76ffa7a5b233f7a31, cbc7fd632510911ef22fb5137ee920328a1a8c5f
- Tests: telperion-heavy 8 --threads 4 -- python3 scripts/gate.py (1473 passed, 0 failed, 1,348 s), measure.sh: cargo and nextest at threads 2, 4, 8
- PRs: