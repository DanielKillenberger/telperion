---
satisfies: [R1, R2, R3, R4, R5]
---
# fn-91-fast-tree-generation-as-a-core-engine.1 Implement fast tree generation as a core engine capability

## Description
TBD

## Acceptance
Every R-ID in the parent spec acceptance criteria is satisfied; judge this task against the spec criteria directly.

## Done summary
Closed by the owner's scope decision (2026-09-27), not satisfied as written. Accepted at the reached milestone: browser oak 9.93x and 10.30x, spruce 42.64x and 44.97x. CPU-owned output speed moves to fn-126 and growth speed to fn-172 to fn-175; the memory non-increase claim and the cold, phone and full-memory qualification are dropped. See COMPLETION-ASSESSMENT.md.

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: c4871093
- Tests:
- PRs: #50
## Qualification remaining after bounded follow-up tasks

The original five-invocation implementation budget remains exhausted; no sixth invocation or counter reset occurred. Separately scoped tasks .2–.14 produced the retained shared GPU/browser and native CPU improvements. Latest browser completed-frame medians are 158.1/180.8 ms for oak seeds 1/7 (9.93×/10.30× original) and 178.9/164.2 ms for spruce (42.64×/44.97×). The host ended optimization at this approximately 10× oak milestone, preserving the strict seed-1 miss of 1.1 ms instead of retiming it away.

This original all-requirements task is not fully satisfied. Native CPU output remains below the original 10× target. Browser live joint-capacity envelopes remain unchanged, but oak seed 7 Wasm linear-memory high-water increased by 26,279,936 bytes in the latest pair. Cold startup, phone and full allocator/driver memory qualification remain absent; accepted visual evidence is scoped to reviewed views. Final aggregate validation passed: 742 Rust tests (20 skipped), 108 JavaScript tests, both Wasm builds, type checking and browser lifecycle smoke. See `.flow/evidence/fn-91-fast-tree-generation-as-a-core-engine/COMPLETION-ASSESSMENT.md` and `stations/REPORT.md` for current evidence. This record does not silently waive the original requirements or declare their gaps passing.
