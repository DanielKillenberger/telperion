---
satisfies: [R1, R2, R3]
---
# fn-217-tree-space-the-supernatural-group-on.2 The group's rows, scale and leaves

## Description
DESIGN.md sections 6 and 7, release half (a): X2 extended (a /supernatural group whose overrides apply over the natural rows only while enabled: flowering share, /bloom, leaf front/back/margin colour, the new reflectances, scale; off equals the base species by hash and sheet); S1 scale per section 7.2 on both executors, before folds, through the common input, tested at 0.1, 1, 100; E2 back and margin reflectance per section 7.6 with neutral identity (geometry and pixels separately) and no leaf-only term on the organ. Every passed species byte-identical with the group off.

## Acceptance
- X2 rows, gating and off-identity proved (hash and sheet).
- S1 on both executors at 0.1, 1, 100; folds, field and device handoff consistent; contact precision checked.
- E2 with neutral pixel identity; stills of a test tree with silver beneath and a gold rim, viewed.
- Gate and npm test green.

## Done summary
The /supernatural group is built per DESIGN.md sections 6 and 7 (release half a).
- X2: the group's own rows plus overrides of the leaf colours, the /bloom rows and each age's flowering share. They apply over the natural rows only while the group is enabled, through Family::applied.
- S1: the converted tree is scaled once, after the natural conversion and before the folds, on both executors. The device dresses its roots at Part::scale.
- E2: the underside and margin act as mirrors. Both are neutral at zero and none reaches the organ.

Proof is in .flow/evidence/fn-217-tree-space-the-supernatural-group-on/TASK2.md:
- With the group off, the mesh hash equals the base species'.
- space_hash: 16 of 16 identical at 33bc84cf and at HEAD.
- Neutral stills are byte-identical to the pre-E2 build.
- The real path matches the CPU path at scales 0.1 and 100.
- Contact precision was measured at 0.1, 1, 5 and 100.
- Four stills of silver beneath and a gold rim were viewed.

Open items for the host (TASK2.md):
- The gold rim turns sub-pixel leaves all gold at whole-tree distance.
- The material's metre rows (shootRadius, matureRadius, matureSpan) do not scale.
- The GPU ring-position domain is 64 m. Seated families taller than that are refused "ring positions" on the real path. This was review finding 1, declined; options are listed.
- A debug-profile overflow in telperion-space shed.rs predates this task.

Decisions:
- Override path = /supernatural + natural path.
- Flowering is overridden per age (/supernatural/flowering/age<k>).
- marginWidth is not overridden.
- One roughness row serves both mirrors.

Tier: not provided by the conductor.
baseline: none (the spec defines no Quick commands). Gate: render half and rest run; failures were fixed and re-run focused. npm test passed (13 files, 143 tests).

stage: impl-review - ran (codex; NEEDS_WORK -> one fix pass -> SHIP)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 1192044db1a2ebb9ccb0ccbc1679de4794f9b056, 761369240cae05533457e184a6a49405ea18720b, 7155dac66279c4fb0c8f68a9e20510a94e163f2b, c37d57bbcb28f21b8267d1c09e784ca7109945f5, 92ecdedf4179f91e0bdf3a6e4027b1aaf0dcf132, 6c3c8452028492ed7b150c3359f3098e1cbb3165, cf17ed982883bb28db006aeaaa25b1b03987e0ea
- Tests: cargo test --profile ci -p telperion-render --no-fail-fast, cargo test --profile ci --workspace --exclude telperion-render --no-fail-fast, npm test
- PRs: