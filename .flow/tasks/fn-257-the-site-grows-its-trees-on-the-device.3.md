---
satisfies: [R5]
---
# fn-257-the-site-grows-its-trees-on-the-device.3 Sized ahead: every build's tables from its expectations, the light rows let go in one place

## Description
TBD

## Acceptance
- Every shipped preset's hint carries a kept prediction (a test, red on the base: the birch's ground hazard left it at none).
- The birch at 40, seeds 1 and 1234, grows with no move; its warm build time measured before and after.
- One `Species::unlit` replaces the copies that zero the light rows in production code.

## Done summary
Every shipped preset's device build is sized from the closed form's expectations (one Species::unlit; the birch's ground hazard had left it unsized and moving 23 times), and the units, woken and lattice tables have room so no shipped preset moves mid-growth (spruce at 40 seeds 1,7,42,1234; Telperion and Laurelin at 1,7,42). Birch tree::drawn 383-415 -> 210-237 ms; spruce 414 -> 387-403 ms. SIZED.md. Codex impl review: NEEDS_WORK (spruce still moved) -> fixed -> SHIP.

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 959c8859a, af867b817
- Tests: cargo test --profile ci -p telperion-render --lib every_preset_is_sized_from_its_expectations, growth_profile (CYCLE_TRACE)
- PRs: