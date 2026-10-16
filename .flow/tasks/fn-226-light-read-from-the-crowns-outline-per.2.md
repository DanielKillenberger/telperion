---
satisfies: [R2]
---
# fn-226-light-read-from-the-crowns-outline-per.2 Delete the re-lay, measured

## Description
The spec's R2: the periodic whole-tree re-lay removed; light reads the rough layout's positions; measured and viewed against today before it lands.
## Acceptance
- Re-lay deleted on both executors.
- Lit oak at 40 and 80, seeds 1, 7, 42: crown width, clear trunk, phytomers grown, build time against today; hero stills before and after viewed; a visible change reported before landing (reported; accepted by the host, 2026-10-08, R2.md).
- Gate and npm test green, except `walks::light_changes_the_tree_by_degree`'s apical_control walks for states[0] and states[1] (31.1 and 32.1 against 30): traced to light read at rough positions without the re-lay, not a defect (R2.md); host decision 2026-10-08: that walk is judged again in task 3, which replaces the lattice reading, and task 3's gate carries it. The bound is unchanged.
## Done summary
The periodic re-lay deleted on both executors (with code only it used); light reads the rough layout every cycle. Lit oak 27-35 % faster at 80, clear trunk unchanged, crown within -8 to +5 % (R2.md); the limb-level change at 80 accepted by the host (2026-10-08). The apical_control light walk (31.1 and 32.1 against 30) deferred to task 3's gate by the host; traced to light at rough positions, not a defect. Review: codex 3 draws SHIP.

## Evidence
- Commits: 3d4b354905c4171361544d4d00389f56d940a06a, b60381d5e140eeef5d68812388228403707f6aae, 3304848b4e22cb6035bd2d20532d5206f9b82e04, 6408a00572f1cad44296674f33580516e0f4976f
- Tests: flock gpu.lock cargo test --profile ci -p telperion-render --no-fail-fast (green), cargo test --profile ci --workspace --exclude telperion-render --no-fail-fast (1 walk deferred to task 3 by host), npm test (green, 143 tests)
- PRs: