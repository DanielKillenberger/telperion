# fn-256-the-whole-grower-runs-on-the-gpu.9 The lattice is dormant where nothing shades

## Description
COUNCIL round 5 step 9: when extinction is zero or no PA deposits leaf area, the lattice dispatches are not encoded; tip reads keep today's arithmetic (equivalence check on read_main's eight rounded weights against 1.0); refusals unchanged.

## Acceptance
Node tables byte-identical on every preset; Telperion about -250 ms GPU; the oak unchanged.

## Done summary
The lattice sleeps where nothing shades (`Light::dormant`: no extinction, or no PA with leaf area under an open sky of one): the densities, weights, paths, sweeps and combine are not recorded; the box, grid, deposit, read, flags and refusals are as before, and `read_main` sums each tip's eight rounded weights against a literal 1.0 (`a_dormant_lattice_reads_as_the_swept_one`, 0 reads differ). Node tables byte-identical on all 8 presets (`node_tables_are_hashed`); stamped GPU time Telperion 1,313.5 -> 1,060.9 ms, the oak 222.0 -> 222.9 ms with the same dispatches (PROOF.md). Follow-up: a dormant build still allocates the lattice's field and scratch buffers.

stage: impl-review - ran (codex, three reviewers, base 101f65378, the merge of fn-198: SHIP)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 2c267c72a
- Tests: cargo test --profile ci -p telperion-render --lib light_tests race_tests tint_rules, node_tables_are_hashed per preset (ignored measurement, raw/t8-nodes-*.txt), the_roofline_is_measured (raw/t9), cargo test --profile ci --workspace --exclude telperion-render --no-fail-fast (1066 passed), cargo test --profile ci -p telperion-render --no-fail-fast, GPU lock (272 passed, 37 ignored), npm test (162 passed)
- PRs: