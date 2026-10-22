---
satisfies: [R3, R4, R6]
---
# fn-308-a-walks-midpoint-draws-a-tree-halfway.3 R3's probe and the ends, held as tests

## Description
Turn R3's ruled measure into an ignored test with the base's numbers fixed in it, and add the ends' identity to the suite (R6). Then run the gate, `npm test` and the stills for the owner.

**Size:** S
**Files:** `crates/telperion-core/tests/walk_midpoint.rs`, `crates/telperion-core/tests/space_walks.rs` (or a new small test file if `space_walks` passes 400 lines)
**Touches:** [crates/telperion-core/tests/walk_midpoint.rs, crates/telperion-core/tests/space_walks.rs, crates/telperion-core/tests/walk_ends.rs, .flow/evidence/fn-308-a-walks-midpoint-draws-a-tree-halfway/**]

### Approach
- **The R3 test.** Add an ignored test to `walk_midpoint.rs` that asserts (a), (b) and (c) on oak to palm at 40, at seeds 1, 7 and 42.
  - Reuse `drawn`, `measure` and the grid code there.
  - The base's numbers are constants with their commit (`911262a6e`): largest log-wood steps 0.756, 0.900 and 0.913, and 11, 10 and 5 frames below both ends. Do not recompute them, since the base is gone once .2 lands.
  - For (b), the stretch per seed is the coarse argmax ± 0.05 at 0.0025.
  - Bound its own threads (one per seed), since R1's unbounded probe hit the 8 GB cap (FRICTION.md).
- **R6.** A suite test: for the six film walks at seeds 1, 7 and 42, `families(a, b, 0)` and `families(a, b, 1)` equal `a` and `b` (Debug text, as `the_ends_written` compares).
- **The gate and npm test**, run once each at the end, as AGENTS.md declares: the two guards first, then `telperion-heavy 8 --threads 4 -- python3 scripts/gate.py`, then `npm test`.
- **The stills.** Use `headless --preset oregon-white-oak --seed 1 --family <frame>`, hero pose, 1,024 px, at t = 0.25, 0.5 and 0.75 at the final commit. Put them beside C4's in `raw/stills/` and make one sheet. View all three.

### Investigation targets
**Required:**
- `crates/telperion-core/tests/walk_midpoint.rs`: the probe
- `.flow/evidence/fn-308-a-walks-midpoint-draws-a-tree-halfway/R1.md`: the base numbers, C5's curves
- `AGENTS.md`: the gate's declared invocation and budgets

### Acceptance
- [ ] The ignored R3 test passes at the final commit and prints its three measures per seed.
- [ ] The R6 test passes, and fails if `families` blends at t = 0 (shown once by a scratch edit).
- [ ] The gate is green, and `npm test` is green.
- [ ] C5's final stills sit beside C4's, viewed, with the paths in the evidence.

## Acceptance
- [ ] TBD

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
