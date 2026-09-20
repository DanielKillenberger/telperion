---
satisfies: [R3, R4, R5]
---
# fn-91-fast-tree-generation-as-a-core-engine.10 Screen radius-only surface run ordering

## Description
Screen one exact-output optimization selected by task .9: the first surface run-ordering pass constructs complete Sample positions merely to find largest radius. Measured native first-pass cost is5.61/6.85ms oak and4.19/4.04ms spruce; this is the maximum removable budget, not promised savings. Eliminate unused sample position/vector work and Sample buffer writes while sharing the canonical radius formulas. Prefer a generic internal sample visitor or equivalent lean shared calculation, allowing radius-only consumption without duplicated botanical formulas. Apply to both ordinary CPU surface and compact GPU input preparation. Preserve stable ordering, ties, signed-zero/max behavior, burial/flare/fork girth/socket/swelling math and all existing errors. No geometry approximation, all-sample cache, new allocation, species branch or unrelated optimization.

Screen cheaply before broad qualification: preserve .9 baseline diagnostic binary and use identical release mature4fixtures first+3warm for compact and full CPUwood, outside timing verify complete canonical wood and compact arrays. Aim>=5% compact preparation improvement for each oak seed without material regressions elsewhere. If this fails, report/revert candidate after one paired screen; no repeated tuning loop or browser/image matrix for a failed screen. This bounded experiment may complete with a measured rejection; parent acceptance remains open. If it passes, run shared browser completed-frame first+5warm all4 against saved .8 Wasm; verify candidate actualpath and compare stage/total/memory envelopes. Keep attribution separate from delivered speed. No extra native GPU matrix needed for an exact shared CPU change after focused renderer tests. No new visual captures if exactoutput evidence passes: reuse accepted .8 views. Any unexpected byte difference requires explanation before broad work, not automatically rejection of all future improvements.

Focused verification should compare radius-only selection with full samples over trunks, branches, fork transitions, flare0/nonzero, modulated profiles and curved runs. Preserve all full-output fields, topology and contact maps on mature4fixtures. Existing scoped release surface/prepared/collapse/attachment tests and Wasm build suffice with selected renderer generation tests on final retained code. No fullworkspace LTO/debug allpreset suite. Account no new live buffers and benchmark control variation honestly. Fresh timing pairs serialized with builds, no infinite repeat to obtain desired result.

Touches: crates/telperion-core/src/surface.rs, crates/telperion-core/src/surface/samples.rs, crates/telperion-core/src/surface/compact.rs, crates/telperion-core/tests/surface_prepared.rs, .flow/evidence/fn-91-fast-tree-generation-as-a-core-engine/**. Quick: /tmp/telperion-fn91-tools/cargo-nextest nextest run --release -p telperion-core --test surface --test surface_prepared --test surface_collapse --test surface_attachments; scoped edition2021 rustfmt; retained candidate npm run render:build and renderer generation suite. Reuse .8 green baseline. Five ticks/10commitsmax, no reset .1. Record friction immediately; host reviews retention/rejection before done/showreceipt. Handovers .flow/tmp/fn91-radius-order-summary.md and fn91-radius-order-evidence.json. Git add-A commits; no push/rebase. Host continues from measured outcome.

## Acceptance
- Shared radius-only ordering preserves canonical formulas and exact output without new allocation or expanded scope.
- One paired native release screen reports all4 unchanged mature fixtures, raw samples/provenance, compact/fullwood timing and full-array output checks; >=5% compact gain on each oak is the screen target, all misses retained.
- A rejected screen is reverted and explained; a retained candidate passes focused checks and fresh browser completed-frame/memory comparison, without claiming parent10x/100ms targets unless actually met.
- Host retention decision precedes verified Flow completion and receipt commit.


## Done summary
The single radius-only surface ordering candidate was rejected after missing the 5% compact-preparation gain on both oak seeds. Warm compact changes were +9.80/-1.67% for oak and +0.12/-0.24% for spruce; complete wood/compact/contact byte comparisons and all repeats passed. Production source is restored exactly, and the host reviewed and approved rejection before completion.

Report, raw samples, provenance, candidate patch and reproduction scripts: `.flow/evidence/fn-91-fast-tree-generation-as-a-core-engine/radius-order/REPORT.md`. The single paired screen completed without tuning or browser qualification. Baseline/candidate native builds and exact comparisons passed; final diff is evidence-only. Parent delivery, memory, cold and phone requirements remain open.

baseline: green via .8 handoff (core20/20, renderer18/18, mature production1/1, Wasm, TypeScript, browser lifecycle).
GATE_SKIPPED:unittest:docs-only - cumulative diff classified tier-B (no executable paths touched)
Tier: session (jev intelligent 0.33; explicit IMPLEMENTER preserved).
stage: impl-review - skipped(config: REVIEW_MODE=none)

Rollback-hook friction cost about one minute and is recorded in FRICTION.md. The archived patch enabled a recoverable reverse application. Host approved no additional tests/builds after exact production restoration.
## Evidence
- Commits: 7b031c1babfb139dfc3828ecf653976b849dbfc1
- Tests: baseline: green via .8 handoff (core20/20, renderer18/18, mature production1/1, Wasm, TypeScript, browser lifecycle), cargo build --release -p telperion-core --example radius_order (baseline and candidate scratch builds, both exit 0), timeout 600s bash .flow/evidence/fn-91-fast-tree-generation-as-a-core-engine/radius-order/screen.sh (exit 0; 4 complete baseline/candidate cmp and 24 repeat comparisons), python3 .flow/evidence/fn-91-fast-tree-generation-as-a-core-engine/radius-order/analyze.py (assertions pass; oak performance target false), git diff --exit-code -- crates/telperion-core/src/surface.rs crates/telperion-core/src/surface/samples.rs crates/telperion-core/src/surface/compact.rs (exact production restoration), rustfmt --edition 2021 --check .flow/evidence/fn-91-fast-tree-generation-as-a-core-engine/radius-order/radius_order.rs, bash -n .flow/evidence/fn-91-fast-tree-generation-as-a-core-engine/radius-order/screen.sh, git diff --check, GATE_SKIPPED:unittest:docs-only - cumulative diff classified tier-B (no executable paths touched)
- PRs: