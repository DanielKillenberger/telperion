# fn-354-wood-that-lasts-the-budget-the-girth.7 Two leaf values: the ordinary tree's leaf, the beech's short shoot

## Description
TBD

## Acceptance
- [ ] TBD

## Done summary
The ordinary tree's leafArea is 15.7 cm² on every age (157ff25a9) and the beech's short shoot lives 12 years in the beech and both Two Trees (5cc4f117b), each with the design's comment and its repinned digests. On seeds 1001/1007/1012 the ordinary tree stands (trunk 3-4 -> 9-10 cm), the beech's grower leaf over its crown projection is 2.0-2.9 (under 5), and ground_reach is green; one test, a_tree_grows_the_same_without_what_it_is_certain_to_shed, is left red for the host (beech 0.81 over its 2/3 bound). Full record: .flow/evidence/fn-354-wood-that-lasts-the-budget-the-girth/TASK7-DONE.md.

Tier: product values, built by Opus to the host's written decision (model: claude-opus-5-5).

baseline: green (handoff: narrow space+core 769 of 769 at fn-355's head; vitest 218)

stage: impl-review - skipped(config: REVIEW_MODE=none; one codex review at the landing)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 157ff25a9fc8a42251f89294c0553c5e1605fbbb, 5cc4f117b2be1b8c73305ff39112d530d425272f, 3afe1407a383d1e056f9bea3bf87e5e67e267d7d
- Tests: telperion-heavy 16 --threads 4 -- cargo nextest run --cargo-profile ci -p telperion-space -p telperion-core --no-fail-fast -j 3 -E 'not test(every_setting_changes_the_tree_by_degree_and_branches_come_and_go_at_vanishing_size) & not test(the_largest_tree_and_the_palm)' (767/769; boundary rows repinned green; shed test red for host), cargo test --profile ci -p telperion-core --test generation_limit_guard, cargo test --profile ci -p telperion-render --lib compile_size, cargo fmt --all --check, npx tsc --noEmit, npm run wasm:build && npm test (catalogue check, vitest 218/218)
- PRs: