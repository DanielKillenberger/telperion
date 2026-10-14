---
satisfies: [R2, R4, R6]
---
# fn-270-growth-continues-from-any-age-in-less.6 A grow-in past the budget is refused by name, never a crash

## Description
Owner, 2026-10-14: growing an 80-year Norway spruce in the harness crashed the tab out of memory. Reproduce natively first (peak device and wasm memory of the spruce grown in to 80, whole years, as the harness does: growth state, settled tree and drawn tree together), find which memory the guard misses, then: the guard charges what the grow-in holds and refuses by name before the browser runs out, the drawn tree untouched; and the spruce at 80 grows if the measured fix (R4's levers) brings it under the browser budget.

## Acceptance
A native repro red before and green after; the guard's charge matches the measured peak within its stated margin; the spruce at 80 either grows or is refused by name with the measured numbers; gate, npm, review.

## Done summary
Fixed the device ledger, which kept charging a buffer destroyed ahead of its owner. Gave a grow-in's draws fn-235's budget rule, as builds have, under the host's 2026-10-14 ruling recorded in R2c: a draw that fits only alone lets the drawn tree go first, and the page shows "making room". Measured and dropped a finer starting-size ladder. Under 6,144 MiB, the spruce at 80 makes room for draws 75 to 78, then is refused by name at the year-79 growth (the axes move, 6,730 MiB) with the drawn tree kept. Every other preset grows in at its own age and at 80 without making room. Numbers are in GROWBUDGET.md.

Tests: `a_buffer_let_go_early_leaves_the_ledger_once` (red before, green after); `a_grow_in_draw_lets_the_drawn_tree_go_only_where_it_fits_alone`; `grows_in_under_the_page_budget`, run by name; and harness `grow-in.test.ts`.

Gate: `telperion-heavy 9 --gpu 4 --threads 4 -- cargo test --profile ci --workspace --no-fail-fast`, 1,373 passed, 0 failed, 91 ignored. npm test: 169 of 169.

stage: impl-review - ran (codex: NEEDS_WORK, 3 findings fixed in 5d33e6af4, re-review SHIP)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 4324e28f058b9de6e8057e2a328a7896d139d086, 6792856c2d1239c2504126324a96116dfd4ad8f4, 5d33e6af40b7fe10456aa77a5dedb0e82a081a20, 831c8913c13a0f29102529c56a434b6526cc6600
- Tests: telperion-heavy 9 --gpu 4 --threads 4 -- cargo test --profile ci --workspace --no-fail-fast, npm test
- PRs: