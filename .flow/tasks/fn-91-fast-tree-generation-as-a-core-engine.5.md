---
satisfies: [R2, R3, R4, R5]
---
# fn-91-fast-tree-generation-as-a-core-engine.5 Reduce division-heavy wood admission traversal

## Description
Reduce the CPU admission cost remaining in resident wood preparation after task .4, while retaining exact prepared output, fallback/error behavior and default CPU output. Browser .4 wood preparation is still roughly 137-215 ms depending on fixture/sample, before GPU expansion. The current prepared branch visits every triangle through Run::triangle, repeatedly dividing and taking remainders to recover strip/ring indices. Ordinary CPU emission already uses nested ring/segment loops.

Host design: replace only this division-heavy sequential triangle traversal with a bounded nested strip/cap traversal in the identical face order. Keep admitted() f64 arithmetic, every triangle check, radius recovery, canonical positions, shader, metadata, allocation and fallback rules unchanged. Do not stop after fallback if doing so could suppress a later error. Factor a small iterator/helper only if it inlines without allocating or making the default builder slower. Add focused equivalence coverage comparing old procedural enumeration with the new traversal, including caps, segments, long runs and nonzero bases; preserve existing pathological geometry checks. All prepared fields and admission outcomes must remain exact; GPU code unchanged means prior visual acceptance applies with no new screenshot or owner question.

Measure a saved before executable and new candidate on mature oak/spruce seeds1/7 in a release core preparation-only runner (solve once, first + 5 warm preparations; checksums outside timing). Prefer whole-call timers, avoiding .3 per-run observer overhead. Preserve control runner and source/build provenance. Reuse saved native resident binary from .4 and measure seed1 both species complete-frame first+3warm if preparation improves. Aim at least10% preparation and5% complete-frame gain; report misses honestly without weakening parent10x/100ms/no-peak-rise requirements. If gain is absent or mixed beyond noise, stop this hypothesis and retain evidence rather than add optimizations. If worthwhile, run current four-fixture browser completed-frame harness once and report new stage medians. No unbounded tuning or repeated full workspace builds. Keep CPU default behavior and all-field hashes unchanged; use focused core checks and existing renderer exact-field tests as appropriate. No new GPU shader or new visual qualification.

**Touches:** crates/telperion-core/src/surface.rs, crates/telperion-core/src/surface/prepared.rs, crates/telperion-core/tests/surface_prepared.rs, .flow/evidence/fn-91-fast-tree-generation-as-a-core-engine/**

Quick: explicit core surface/prepared/collapse/attachment test targets using existing pinned nextest; scoped rustfmt; Wasm check/build if candidate advances to browser; existing native example for end-to-end. Use separate benchmark/build windows. Record friction immediately. Host reviews actual diff before task completion. Task .1 and parent spec remain open; this is a new bounded admission-loop experiment, not a reset of .1's exhausted invocation budget.

## Acceptance
- Exact prepared fields, triangle order/admission, fallback/errors and unchanged default CPU output verified against the old traversal and existing fixtures.
- Before/after preparation matrix with whole-call timing and checksums, plus complete-frame native/browser evidence if the local gain justifies continuation; honest experiment decision against stated thresholds.
- Relevant focused gates green, no allocation/memory regression introduced, no shader changes or repeated visual approval; parent targets unchanged.


## Done summary
Retained the allocation-free nested wood admission traversal with exact prepared fields and default CPU surface hashes. Evidence and the host retention decision are in `.flow/evidence/fn-91-fast-tree-generation-as-a-core-engine/admission/REPORT.md`: preparation gains 9.69–11.97%, native completed-frame gains 3.51/4.15%, below the 5% aim; browser expansion stopped and parent targets remain open.

baseline: green (17 focused integration tests and correctly scoped edition-2021 rustfmt). Final 17 integration and 15 surface unit tests, Wasm compile check, formatting and whitespace checks pass. Enumeration test first failed for missing helper. Host direct diff review completed before completion; no shader/allocation/default-CPU changes. RSS oak increase and prior CPU timing attribution remain unresolved. No subagents dispatched.

Tier: session (jev intelligent 0.41; explicit IMPLEMENTER preserved)
stage: impl-review - skipped(config: REVIEW_MODE=none)
## Evidence
- Commits: ef52e093347544a39640ced5306339b5e69d5cc9
- Tests: baseline: green (17 integration tests; edition-2021 scoped rustfmt), /tmp/telperion-fn91-tools/cargo-nextest nextest run -p telperion-core --test surface --test surface_prepared --test surface_collapse --test surface_attachments (17 passed), /tmp/telperion-fn91-tools/cargo-nextest nextest run -p telperion-core --lib --test surface --test surface_prepared --test surface_collapse --test surface_attachments -E test(surface) (15 passed, 179 skipped; surface unit selection only), cargo check --target wasm32-unknown-unknown -p telperion-render, rustfmt --check --edition 2021 --config skip_children=true crates/telperion-core/src/surface.rs crates/telperion-core/src/surface/prepared.rs crates/telperion-core/tests/surface_prepared.rs, git diff --check, release preparation control/candidate: four fixtures, first+5warm, all prepared hashes and capacities exact; all CPU hashes exact, python3 .flow/evidence/fn-91-fast-tree-generation-as-a-core-engine/admission/measure-native.py
- PRs: