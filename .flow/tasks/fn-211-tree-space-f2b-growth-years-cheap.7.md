---
satisfies: [R4]
---
# fn-211-tree-space-f2b-growth-years-cheap.7 Option A's ceiling, then its contract

## Description
After HYBRID.md (delete the hybrid), the host's plan (accepted by the owner, 2026-10-07) turns to option A (ASTRA-REVIEW-OAK-GPU.md: exact fixed-point layout and light on both executors). First its ceiling, measured: of the lit oak's 80-year build (seeds 1 and 7, all cores and one core), how much is the fine roots' own work that A would move to the device (their growth, rough layout, leaf deposits, light sampling, balance and allocation within fine roots) against what stays on the CPU (the coarse structure's passes, the sweep over the lattice, settle). If the movable share is under 30 % on all cores, stop and report: A is not worth 5-7 sessions. Otherwise write A's arithmetic contract (CONTRACT-A.md) covering what Astra's table lists (positions and frames, re-lay and sag, deposits, sweep, sampling, wood and balance, allocation and sizes), each with widths, rounding and order, for the host and Astra to review. No product code.

## Acceptance
- The movable share, measured on both seeds and core counts, with the method.
- Either the stop report, or CONTRACT-A.md for review.
- No product code changed.

## Done summary
Measured option A's ceiling on the lit oak at 80 (seeds 1 and 7, cut 1,024, two load-gated sets started at load 5.9 to 7.9, plus one high-load one-core set). The fine roots' own work (items' balance, allotment, growth and waking, sketch, fine deposits and reads, the re-lay's fine share) is 50 to 56 % of the stepped build on all cores (34 to 38 % without the re-lay) and 57 to 74 % on one core: above the 30 % bar, so no stop. The settle (21 to 26 %) is most of what stays; the stepped executor's glue is 15 to 17 % on all cores. Wrote CONTRACT-A.md: formats (Q32 i64, W Q64.64 i128, T u32 turns), one rule per operation (floored products, truncating quotients, isqrt, CORDIC sin/cos/atan2, existing exp/log extended to W, wrapping sums with range checks), and the seven parts of Astra's table with widths, rounding and order, plus the inductive argument, ranges and build order. Proposals for review: the contract binds every cut-movable computation on every axis; the sweep stays f64 on the CPU with Q32 output; corner weights sum to one exactly; D16 made integer. Instrumentation reverse-applied; git diff -- crates/ empty.

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 5147b36d
- Tests: HYBRID=1 target/ci/examples/hybrid 80 {1,7} 2 (instrumented, reverted); target/ci/examples/stepcost 40 1,7 1 (same tree: true on the instrumented build)
- PRs: