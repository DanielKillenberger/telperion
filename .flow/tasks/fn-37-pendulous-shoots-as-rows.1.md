---
satisfies: [R1, R2, R3, R4, R5]
---
# fn-37-pendulous-shoots-as-rows.1 Implement Pendulous shoots as rows

## Description
Turn the hidden curtain mode into twig rows (hang, pendulous length, pendulous radius, curtain separation), widen the canopy orientation rails to signed values, and re-render the birch's matched pairs (R1-R5).

## Acceptance
- [x] R1 `hang` continuous, neutral byte-identical, no switch frame
- [x] R2 three railed twig rows; spruce pins hold
- [x] R3 signed canopy rails, presets byte-identical
- [x] R4 birch rows set; round-4 pairs rendered; verdict in fn-34
- [x] R5 tests

## Done summary
The curtain moved into `branching/local/pendant.rs`; the birch states its curtain as rows instead of constants; every other table byte-identical at neutral. Round 4: five of eight comparable numbers moved toward the photograph (S-BARE crown base within 0.004); the droop stays capped at the old constant (about 19 degrees below horizontal). The owner accepted the silver birch in fn-34; the beech moved to fn-62.

## Evidence
- Commits: 40e42bb5
- Tests: cargo fmt --all -- --check, cargo clippy --release --workspace --all-targets -- -D warnings, cargo test --release -p telperion-core --no-fail-fast, cargo test --release -p telperion-render --no-fail-fast, ...
- PRs:
