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
TBD

## Evidence
- Commits:
- Tests:
- PRs:
