---
satisfies: [R1, R2]
---

# fn-8-lean-rust-tree-generation-core.3 Port local branches, twigs, radii and preset assembly

## Description
Complete the botanical generation pipeline using the crown and foundational types.

**Size:** M
**Files:** Rust branching/twig/radius/preset modules and behavioural tests
**Touches:** [crates/telperion-core/src/branch*, crates/telperion-core/src/twig*, crates/telperion-core/src/radius*, crates/telperion-core/src/preset*, crates/telperion-core/src/lib.rs, crates/telperion-core/tests/growth*, tests/migration/growth*]

## Approach
- Port the final branch-generation law, internode and lateral controls, twig anatomy, radius solve and shedding in their existing stage order.
- Carry end-radius, branch attachment and twig membership metadata through every pass. Preserve the final local taper and crown clipping, including known provisional visual behaviour.
- Move preset parameter construction into the native core while retaining current control names/meaning at the thin adapter boundary.
- Extend task-1 equivalence to complete solved trees, including repeated seeds, family/preset variation, finite clamping, headroom and capped runs.

## Investigation targets
**Required:**
- `src/skeleton/grow.ts:337`
- `src/skeleton/law.ts`
- `src/skeleton/twigs.ts:77`
- `src/skeleton/shed.ts`
- `src/radius.ts:138`
- `src/presets/two-trees.ts`
**Optional:**
- `src/skeleton/continuity.test.ts`

## Approved capture alignment
The rewritten parent capture is authoritative. Baselines diagnose drift; exact old topology or bytes are not a compatibility requirement, and known structural defects need not be reproduced. Preserve meaningful botanical and geometric invariants and report visual/numeric differences. Keep the core lean and simple.


## Acceptance
- [ ] Complete solved-tree discrete and numeric equivalence passes for all fixture classes.
- [ ] Ported biological regressions cover branch attachments, radius continuity, twig anatomy and shedding.
- [ ] Presets, family parameters and specimen seeds remain independently controllable and deterministic.
- [ ] No new botanical redesign or historical FN5 depth requirement enters the port.

## Done summary
Implemented complete native solved-tree generation: crown sampling/colonization, radius-informed local branches, fixed twig anatomy, shell shedding with run/parent remapping, and the final radius solve. Native named families carry independent specimen seeds and editable parameters; preset turn overrides stay separate from envelope-derived growth distances. Representations remain independent requests on the solved Tree. Shared API handover: /home/daniel/Projects/telperion/.git/flow-notes/fn8-rust-20260905/growth.md.

baseline: none (parent and task define no Quick commands). API absence was observed red before implementation. A valid zero-height envelope exposed a zero-step configuration rejection; its regression was observed red and fixed with the established minimum height for derived distances. Nonfinite botanical parameters now reject clearly, while finite twig/radius values retain existing clamps. This intentionally differs from TS nonfinite fallback. No new botanical design or historical depth target was introduced.

R1: All seven final FN6 fixture classes pass complete solved-tree comparison against revision fdafb099b1495519de75a6b9a66d37f7d07e47bd: ordinary 13,616 nodes, Telperion 175,035, Laurelin 104,207, empty 1, capped 20, degenerate 1, high-crown envelope-crossing 4,637. Topology, crossover, branch IDs, twig membership, shed counts and cap flags match exactly. Maximum coordinate drift is 1.30e-9 m; maximum distal/proximal/base radius drift is 1.03e-11 m. The pinned ordinary fixture explicitly overrides the default step from 0.022 to its authored 0.02. Reproduce with tests/migration/growth-reference.mjs; raw output is /tmp/fn8-growth-reference. Browser visual checks remain with the integration task; no visual verdict or total generation performance claim is made here.

R2: Eight focused tests cover deterministic complete presets and shell clipping, fixed twig anatomy, branch attachment radii, leader taper, geometric refinement preserving lateral attachments, structural fork conservation and independence from appended children, whole-run/terminal-transition shedding with remapped IDs, caps surviving removal, generation-limit diagnostics, malformed parent errors, invalid parameter errors, finite rails, usable empty/zero-budget/zero-height outputs, independent seed/family controls, and fine-step influence headroom. Full workspace verification passes 31 tests; the separate seven-case reference comparison passes. Formatting, strict workspace clippy and diff checks pass. Classifier reports FULL; no spec-defined gate receipt or skip was fabricated.

stage: impl-review - skipped(policy: parallel-wave; conductor owns lifecycle, REVIEW_MODE=none)

Task remains in_progress. No review, tracker mutation, integration or flowctl done invoked. Work is committed in the assigned isolated workspace; task-unique evidence is /tmp/fn8-growth-evidence.json. Scope includes the declared branching, twig, radius, preset, root library and growth-test paths, plus the assigned shared API note.

Conductor integrated the worker commit and verified eight growth tests in release mode.
stage: impl-review - skipped(user: none)
stage: wave-join - ran(merge and focused integrated checks)
stage: plan-sync - skipped(config: false)
## Evidence
- Commits: 53361a77474cf45c414980ca354306f4d1adff55
- Tests: baseline: none (parent and task define no Quick commands), Initial API red: cargo test -p telperion-core --test growth (missing generate/Preset); /tmp/fn8-growth-red.log, Zero-height regression red: cargo test -p telperion-core --test growth; /tmp/fn8-growth-invariants-red.log, cargo test -p telperion-core --test growth: 8 passed; /tmp/fn8-growth-invariants-final.log, cargo test --workspace: 31 passed, 4 explicitly ignored reference tests; /tmp/fn8-growth-workspace.log, REFERENCE_OUTPUT=/tmp/fn8-growth-reference node tests/migration/growth-reference.mjs: 7 pinned FN6 classes passed; /tmp/fn8-growth-reference-final.log, cargo clippy --workspace --all-targets -- -D warnings: passed; /tmp/fn8-growth-clippy.log, cargo fmt --all -- --check: passed, git diff --check: passed, flowctl gate classify --base 839bc5d68dde5678c99e76910410cd27a29dc020: FULL; no spec-defined receipts or skips, Conductor: cargo test --release -p telperion-core --test growth: 8 passed on integrated tree
- PRs: