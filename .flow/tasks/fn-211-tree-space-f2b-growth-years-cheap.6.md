---
satisfies: [R4]
---
# fn-211-tree-space-f2b-growth-years-cheap.6 How much a CPU-led hybrid would save the oak

## Description
Owner, 2026-10-07: accepted the host's recommendation (Astra, ASTRA-REVIEW-OAK-GPU.md): before committing to exact fixed-point layout and light (option A), measure a CPU-led hybrid. The CPU keeps the oak's whole light loop (rough layout, deposits, sweep, sampling, balance, allocation; exact), the GPU does only the fine roots' growth bookkeeping from integer inputs the CPU supplies. Measurement only: how much of the oak's 80-year build (seeds 1 and 7) the CPU would still spend, the records and bytes per cycle that would cross both ways, and the time the GPU part could take off, from the existing stepped executor's per-phase timings and fn-209's GPU executor's per-root costs. A verdict: worth building, or delete the idea.

## Acceptance
- HYBRID.md with the remaining CPU time, the crossing records and bytes per cycle, the estimated saving, labelled measured or estimated.
- A one-line verdict for the host.
- No product code changed.

## Done summary
HYBRID.md: measured on the stepped executor's phases (oak 80, seeds 1 and 7, all cores and one), the fine roots' growth the GPU could take is 11 to 12 % of the build on all cores (25 to 26 % on one); the CPU keeps the light loop (56 to 58 %) and the settle (25 to 28 %). Crossing per build: up 9.5M / 19.8M records (1.09 / 2.27 GB as held, 0.23 / 0.47 GB lean), down 1.7M / 3.5M. Estimated saving on all cores +7 % to -11 %, ceiling 8 to 10 %; a free GPU still leaves the oak slower than today's grow. Verdict: delete the idea. No product code changed (instrumentation patch reverted, kept as evidence).

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 0773709f
- Tests: HYBRID=1 target/ci/examples/hybrid 80 {1,7} 2 (patched, reverted), stepcost 40,80 1,7: same tree true on the instrumented build
- PRs: