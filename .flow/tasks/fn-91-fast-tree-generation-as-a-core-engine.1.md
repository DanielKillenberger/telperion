---
satisfies: [R1, R2, R3, R4, R5]
---
# fn-91-fast-tree-generation-as-a-core-engine.1 Implement fast tree generation as a core engine capability

## Description
TBD

## Acceptance
Every R-ID in the parent spec acceptance criteria is satisfied; judge this task against the spec criteria directly.

## Done summary
Blocked:
Optimization has ended at the owner-selected approximately 10× oak milestone: browser oak 9.93×/10.30× and spruce 42.64×/44.97×. Tasks .2–.14 are done and final aggregate checks pass. This original all-requirements owner task cannot be marked fully satisfied: oak seed 1 misses strict 10× by 1.1 ms, CPU-owned targets remain unmet, observed Wasm/RSS increases prevent a universal memory-nonincrease claim, and cold/phone/full-memory qualification is absent. No additional optimization or sixth original invocation is running. Completion requires an explicit scope decision or separately authorized qualification work, not another automatic experiment. Current evidence: .flow/evidence/fn-91-fast-tree-generation-as-a-core-engine/COMPLETION-ASSESSMENT.md.
## Evidence
- Commits:
- Tests:
- PRs:

## Qualification remaining after bounded follow-up tasks

The original five-invocation implementation budget remains exhausted; no sixth invocation or counter reset occurred. Separately scoped tasks .2–.14 produced the retained shared GPU/browser and native CPU improvements. Latest browser completed-frame medians are 158.1/180.8 ms for oak seeds 1/7 (9.93×/10.30× original) and 178.9/164.2 ms for spruce (42.64×/44.97×). The host ended optimization at this approximately 10× oak milestone, preserving the strict seed-1 miss of 1.1 ms instead of retiming it away.

This original all-requirements task is not fully satisfied. Native CPU output remains below the original 10× target. Browser live joint-capacity envelopes remain unchanged, but oak seed 7 Wasm linear-memory high-water increased by 26,279,936 bytes in the latest pair. Cold startup, phone and full allocator/driver memory qualification remain absent; accepted visual evidence is scoped to reviewed views. Final aggregate validation passed: 742 Rust tests (20 skipped), 108 JavaScript tests, both Wasm builds, type checking and browser lifecycle smoke. See `.flow/evidence/fn-91-fast-tree-generation-as-a-core-engine/COMPLETION-ASSESSMENT.md` and `stations/REPORT.md` for current evidence. This record does not silently waive the original requirements or declare their gaps passing.
