# fn-361-the-oaks-sibling-limbs-are-unequal.1 The oak's sibling limbs are unequal

## Description
TBD

## Acceptance
- [ ] TBD

## Done summary
The Oregon white oak's limb row carries `/space/ages/6/form/dominance = 0.5` (host's estimate, with its comment), mirrored in `telperion_space::oak()` (the limb 0.5, the bough, which inherits the limb's form, 0.0) under the host's amended boundary; the generated preset table and every pin the row moved are updated, and the oak's panel is drawn beside the no-row tree and the base sheet.

- R1: the row in `oregon-white-oak.values`; `presets.generated.ts` regenerated; `oak.rs` mirror lines (8f079717c).
- R2: three `catalogue_identity` digests, the oak's both-ends pair in `cycle_cost_tests`, `growing_tests`' oak seeds 1 and 7 repinned; the other seven both-ends pairs unchanged (PINS.md).
- R3: the narrow `telperion-core` integration suites, the lib filters `pipeline::cycle_cost_tests family::tests suite::drop suite::sag suite::strands`, and the whole `telperion-space` crate pass.
- Stop check at 40 and 80 (STOPCHECK.md): 20 of 24 trees lose over 25% of drawn leaf (medians -36% at 40, -35% at 80), height and trunk within 25%; the host ruled to go on.
- R4: sheets `oak-40-80-*`, `oak-120-*`, `oak-200-*`, `oak-300-*` (whole and bare) and PANEL.md's table with White's trunk ranges counted. Trees past the 24 GB scope: the row at 200 seeds 1002 and 1010, at 300 seeds 7, 1002, 1005, 1006, 1008, 1009, 1010, 1011; no row at 300 seeds 1002 and 1010 (OOM.txt).
- R5: the other six presets not redrawn.
- Code change beyond the row: the `oak.rs` mirror lines, allowed by the host's amendment (spec, Boundaries).

Tier: you execute; you do not reason about how the system should work. (model: claude-opus-5-5)
stage: impl-review - skipped(config: REVIEW_MODE=none)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: cd30d578cc884548177f47e1ffc07e7f6bb8038d, b331850a6ae0db383be557d83342e7a16f30c5fd, 8f079717c34b76673b938f04ccb2e6add7a88471, f75cfde43e0f5b6d44a55a116bac68466c865fe1, 8f940a0c7dbc8e32d342ef3f5880d6f6086730a3, f7733b941cba1c74706437a23fdcd6becbcd8c8a, e5547684fcee941c2b48b160e070263705c97284
- Tests: telperion-heavy 12 --threads 8 -- cargo test --profile ci -p telperion-core --no-fail-fast --test preset_values --test space_presets --test catalogue_identity --test capability --test walk_ends --test walk_law --test walk_midpoint --test walk_flying --test space_walks --test parameter_reference --test walk_sizes, telperion-heavy 12 --threads 8 -- cargo test --profile ci -p telperion-core --no-fail-fast --lib -- pipeline::cycle_cost_tests family::tests suite::drop suite::sag suite::strands, telperion-heavy 12 --threads 8 -- cargo test --profile ci -p telperion-space --no-fail-fast
- PRs: