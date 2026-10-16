---
satisfies: [R1, R2, R3, R4, R5]
---
# fn-177-limbs-hold-their-girth-to-the-crowns.1 Implement Limbs hold their girth to the crown's edge

## Description
Measure what thins the plane's limbs, then add `girthHold` and `girthFall` to `/radii`.

## Acceptance
- R1-R5 of the spec.

## Done summary
Every structural axis holds the pipe radius where it leaves its parent over `girthHold` of its own reach, then falls to the pipe radius by its tip. R1 measured (pipe model 96% of the thinning). At zero hold, 16 of 16 preset/seed builds match master. Hold tests red first. Plane candidate (girthHold 0.75, girthFall 4): build 771 vs 769 ms, peak 619 vs 618 MB, nodes and leaves identical. Owner rejected it as the plane's fix (hierarchy, fn-182) and merged the rows as a dial for the beech. Codex SHIP after a host-authorised round reset.

## Evidence
- Commits: 7513162a961b6a95e81f2f8cc9e599ea9cf76f47, 4e938d65bb0d9ee6d760f922a6664983918d9500, 792fc840f954a05c79d0045d2435ee0773bc53ac, 6adef593c1c23d2847472d1d98bc9faadf3e5af1, 03d55a719591bfb9152c18cc3e50d6f3f67709c3, 6ad2c7095656b6cb1262148f1191afffde00df18, d515486ea5244df9901de71329bed67a5b20e932
- Tests: cargo test --profile ci --workspace --no-fail-fast (1082 passed at 03d55a71), npm test (124 tests), cargo test --profile ci -p telperion-core --lib hold_tests
- PRs:
