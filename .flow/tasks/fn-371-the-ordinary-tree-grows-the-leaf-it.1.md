# fn-371-the-ordinary-tree-grows-the-leaf-it.1 The ordinary trees leaf area is its drawn blades

## Description
TBD

## Acceptance
- [ ] TBD

## Done summary
The ordinary tree's 15 `leafArea` rows now hold its drawn blade's area, 44.245 cm². That is the default element's 42.509 cm² at size 1, times 1 + 0.35²/3, and it replaces the beech's 15.7 cm². At 40 the trunk is ×1.65 (14.1 and 16.7 cm on seeds 1001 and 1007), the grower's leaf is ×2.75 to ×2.78, and height and drawn leaf are unchanged. Three pins moved (the family digest, the walk digest and the ordinary tree's cycle pin) and `presets.generated.ts` was regenerated; no other preset moved. The host accepted the sheet on 14 Nov and recorded the gap to the base's trunk and leaf as unmet. Evidence: `.flow/evidence/fn-371-the-ordinary-tree-grows-the-leaf-it/` (`R1.md`, `R3.md`, `sheets/ordinary.jpg`).

stage: impl-review - skipped(config: REVIEW_MODE=none)
Tier: implementer opus (model: claude-opus-5-5)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: b75140dcd6e3aa80b6ffc5d2ffaf41cdc21c5b30, f253c79df301ea8e3292d89a049d8f37e41358a8
- Tests: cargo test --profile ci -p telperion-core --test catalogue_identity --test space_walks --test walk_sizes --test space_presets --test light_rows --test grown_total --test preset_values --test capability --test ground_reach --test material_detail, cargo test --profile ci -p telperion-core --lib (cycle_cost, suite::*, relay_tests, branching::*, fork_tests, family::tests, pipeline::tests), cargo test --profile ci -p telperion-core --test generation_limit_guard, cargo test --profile ci -p telperion-render --lib compile_size
- PRs: