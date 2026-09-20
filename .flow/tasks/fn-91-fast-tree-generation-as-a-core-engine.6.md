---
satisfies: [R2, R3, R4, R5]
---
# fn-91-fast-tree-generation-as-a-core-engine.6 Share canonical ring preparation across GPU foliage and wood

## Description
Investigate and implement shared canonical ring preparation for the existing GPU-resident generation path. Task .5 measured consistent but small3.5-4.2% total gain; duplicated ring construction is a different measured-stage hypothesis. AttachmentSurface::new emits rounded wood ring vertices before prepared::prepare independently samples/frames/emits them again. Browser .4 descriptors took19/24ms oak and78/73ms spruce while woodPrepare took180/218/147/137ms. Preserve full requested output and parent10x/100ms/no-peak-rise targets.

Host architecture: canonical wood preparation may emit an optional node-to-contact-edge map alongside positions, using the existing sorted path's vertex base, burial offset and cap exclusions. Keep ordinary prepare and default CPU builder APIs/allocations unchanged where this extra mapping isn't requested. Foliage station preparation should accept those canonical contact vertices and edge ranges without recomputing ring geometry or building segment AABBs unused by GPU placement. Use a typed borrowed representation or explicit shared input, not empty arrays falsely claiming complete contacts. Ring starts are vertex offsets; adjacent rings in a run remain contiguous and caps excluded. Preserve exact rounded contact coordinates, leaf random ordering/counts/contact projection, all wood attributes and error/fallback lifecycle.

First produce a compact design note with source-grounded API/ownership changes and CPU/GPU peak lifetime comparison, and send it to host before implementing. Host owns system design. The preferred first path borrows prepared float32 positions for GPU contact upload rather than allocating an expanded Vec<Vec3>. Keep foliage-before-wood GPU execution order. Account canonical positions held through foliage and compare actual simultaneous peaks; do not assume eliminated duplication proves lower peak. If lifetimes would increase the accounted peak, propose sharing the uploaded contact/position buffer through wood expansion or a narrower alternative to host before adding a larger GPU boundary. No speculative shader rewrite without host design approval, no GPU skeleton rewrite.

Baseline fresh before change: existing .5 native executable can be reused with SHA provenance. Compare mature oak/spruce seeds1/7 preparation/output fingerprints, native seed1 first+3warm complete-frame and RSS/counters. If a useful consistent gain survives total cost and no new accounted-memory increase, run the existing browser four-fixture completed-frame matrix and owned CPU regression checks as appropriate to changed paths. Aim >=15% total improvement for this duplicate-work boundary, but report misses honestly. Reject or stop a candidate with no total gain; do not tune endlessly. Exact output should reuse accepted visual evidence. If any output changes, first identify whether it is an unintended regression; do not automatically ask owner about microscopic pixel differences.

**Touches:** crates/telperion-core/src/surface.rs, crates/telperion-core/src/surface/**, crates/telperion-core/src/foliage/prepared.rs, crates/telperion-core/tests/**, crates/telperion-render/src/generation.rs, crates/telperion-render/src/generation/**, crates/telperion-render/examples/generation_gpu.rs, scripts/benchmarks/generation.md, .flow/evidence/fn-91-fast-tree-generation-as-a-core-engine/**

Quick: explicit core surface/attachment/prepared/foliage tests, renderer generation tests through pinned nextest; scoped rustfmt; Wasm check/build+TypeScript and browser lifecycle smoke if renderer integration changes. Use saved binaries and explicit targets, serial measurement/build windows; no full workspace LTO captures or unrelated test-artifact rewrites. Record friction immediately. Five pilot ticks/10commits maximum for this task. Do not reset .1. Commit completed units git add -A; host directly reviews before completion (configured review none).

## Acceptance
- Shared input eliminates duplicate canonical ring emission on supported GPU-resident path while preserving exact geometry/contact/order and retained CPU API behavior.
- Invalid/unsupported/degenerate inputs, renderer identity and lifecycle remain explicit; data ownership and actual simultaneous CPU/GPU memory domains are documented and measured.
- Comparable end-to-end native/browser evidence determines whether the candidate is retained; no isolated stage win or weakened parent target counts as completion. Focused correctness gates pass. An unsuccessful experiment is recorded honestly and its product changes not retained without host decision.


## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
