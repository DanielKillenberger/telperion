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
R3 and R6 are held as tests.

- **`walk_midpoint::the_walk_to_the_palm_changes_by_degree` (ignored)** asserts R3's (a), (b) and (c) on oak to palm at 40, seeds 1, 7 and 42. The base numbers are constants from commit 911262a6e. On its one run:
  - (a) the largest log-wood steps are 0.680, 0.915 and 0.767, against bounds of 0.832, 0.990 and 1.004.
  - (b) the largest fine share steps are 0.183, 0.163 and 0.189, against a bound of 0.2.
  - (c) one frame per seed falls below both ends, against the base's 11, 10 and 5.
- **`walk_ends::every_film_walk_returns_its_ends_unchanged`** covers R6. It went red once with a scratch edit that made `families` blend at t = 0.
- **The workspace gate** (`telperion-heavy 8 --threads 4 -- python3 scripts/gate.py`) passed 1,476 tests, with 133 skipped, in a test run of 1,324 s against the 1,560 s limit.
- **`npm test`** passed 22 files and 177 tests.
- **C5's final stills** sit beside C4's in `raw/stills/final-vs-c4.jpg`. The owner's verdict, relayed by the host, was "nice!".

stage: impl-review - ran (codex). A single reviewer was told to run no build or test command. It returned SHIP, and its command log shows no cargo command.

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: ed16cb8e6db4159139960e58818e72f761457293, eb11ed4c42e61a56b1b4159cf3d0af12fef5c48b
- Tests: telperion-heavy 8 --threads 4 -- python3 scripts/gate.py, telperion-heavy 8 --threads 2 -- npm test, telperion-heavy 8 --threads 2 -- cargo test --profile ci -p telperion-core --test walk_midpoint -- --ignored the_walk_to_the_palm_changes_by_degree
- PRs: