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
TBD

## Evidence
- Commits:
- Tests:
- PRs:
