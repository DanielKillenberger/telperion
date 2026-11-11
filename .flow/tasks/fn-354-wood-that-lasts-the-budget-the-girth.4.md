# fn-354.4 The leaf account (R5)

## Description

Build `.flow/evidence/fn-354-wood-that-lasts-the-budget-the-girth/LEAF-ACCOUNT.md`, sections "The design" and "The Two Trees' leaf", as written by the host. One function `leaf(phytomer, year)` (today `girth::leaf`) takes the body the design gives; `leaf_years` becomes a trait with a value for every species; the light's deposit and the wood rule read it; the supernatural canopy size reaches the grower's `leaf_area`.

## Acceptance

- [ ] `leaf(p, y)` is the design's formula, read by the light's deposit and by the wood rule; the deposit walks every held axis that bears leaf.
- [ ] `leaf_years` has the design's value on every species with its source or the word "estimated" in the preset's comment; the walk tree takes 1 and `leaf_years` has its own walk.
- [ ] With the supernatural group on and its canopy size set, the species' `leaf_area` is multiplied by (size / natural size)²; dormant otherwise; tested.
- [ ] `ground_reach` is green, or the builder returns with the sheet.
- [ ] Measured on seeds 1001, 1007, 1012 through the scoreboard's runner, label `fn354-leaf`: spruce, birch, ordinary, beech and oak at 40; Telperion at 82 and Laurelin at 80; sheets beside base and the product before this task; leaf, trunk, shoots, cost. Oak and beech differences explained (expected: none).
- [ ] Narrow tests, both guards, fmt, tsc, catalogue check, vitest green; digests repinned with before and after; docs updated; TASK4-DONE.md written.

## Done summary
The leaf account: one girth::leaf(p, y) read by the light's deposit and the wood rule, standing while its wood is open (host correction of 12 Nov, no fade); leaf_years for every species (spruce 5.5, birch 1, ordinary 1); the supernatural canopy size multiplies leaf_area by (size / natural)^2. ground_reach is green; the oak and beech are unmoved; the Two Trees stand off the ground; the spruce's trunk is 25 to 27 cm. Details in .flow/evidence/fn-354-wood-that-lasts-the-budget-the-girth/TASK4-DONE.md.

Tier: product code, built by Opus to the host's design (model: claude-opus-5-5)

stage: impl-review - skipped(config: REVIEW_MODE=none; one codex review of the spec's diff at its end)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: bfe2fd92f, e5cedcc89, 96b4b503d
- Tests: telperion-heavy 16 --threads 4 -- cargo nextest run --cargo-profile ci -p telperion-space -p telperion-core --no-fail-fast -j 3 -E 'not test(every_setting_changes_the_tree_by_degree_and_branches_come_and_go_at_vanishing_size) & not test(the_largest_tree_and_the_palm_grow_the_same_tree_again_and_on_any_cores)' (765 of 765), cargo nextest ... every_setting_changes_the_tree_by_degree_and_branches_come_and_go_at_vanishing_size (pass, at bfe2fd92f), cargo test --profile ci -p telperion-core --test generation_limit_guard; cargo test --profile ci -p telperion-render --lib compile_size, cargo fmt --all --check, npx tsc --noEmit, npm run catalogue:check, npx vitest run (218)
- PRs: