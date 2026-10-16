---
satisfies: [R7]
---
# fn-24-continuous-tree-space-habit-and-leaf-as.2 One outline routine: lobes and section roundness as traits

## Description
Dissolve the element anatomy enum into lobe count, lobe depth and section roundness on the numeric outline path, so the oak blade and spruce needle are rows (R7 element half).

## Acceptance
- No `ElementAnatomy` identifier; one outline routine builds every element
- Lobe and section traits validated; a section count below the lobe rule is rejected naming both fields
- Oak and spruce rows build elements the fn-23 level ladder accepts

## Done summary
One outline routine builds every element; lobes cut sinuses into the outline and section roundness rolls the strip into the needle's four-sided shaft. After the owner asked for broader lobes and narrower sinuses, the sinus term `(1 + cos)/2` is raised to the sixth power (commit 543627a): broad top 60.2% and notch 17.2% of a lobe period, matching the retired table's 60.2%/16.9%, with no new field. A lobe half-extent of 0.115 to 0.15 is unreachable with five evenly spread lobes (capped at 0.1); the retired table only exceeded it by overlapping ellipses.

## Evidence
- Commits: 6d11d09, 5e6d7d8, 68a3e76
- Tests: worker: cargo test --release --workspace - 131 tests, 29 suites, green at each of the three workspace commits, worker: cargo clippy --release --workspace --all-targets - clean, worker: cargo fmt --all -- --check - clean, ...
- PRs:
