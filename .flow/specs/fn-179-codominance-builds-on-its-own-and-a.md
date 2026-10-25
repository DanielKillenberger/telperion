## Goal

The beech's first runner Tune after fn-61, fn-170 and fn-177 (fn-62, 2026-09-27) kept nothing: all 16 bundles raised `codominance` while `forkDivergence` and `forkLean` stayed at their default 0, so every build was refused ("fork parts pass through each other"); the loop redrew the same refused bundle for five rounds (1,453,019 Jev tokens), and the run needed two hand patches to use the beech's curated references. `codominance` must build on its own, a bundle that fails to build must be taken apart, and a run must read references as the catalogue stores them.

## Decisions

- owner (2026-09-27), on the host's three fixes from the beech run (fork defaults, splitting a bundle that fails to build, curated references read as stored): "go"
- Fork rows get nonzero defaults, dormant while `codominance` is 0; the birch states its own values and is unchanged.
- A bundle failing to build at every strength is halved and its halves tried one a round; a lone failing move is dropped for the revision; no bundle drawn twice.
- Curated references take their view from the shot's scale and their image from `.refs/` by hash, copied into the run's cache and verified.
- Out of scope: fn-168's tuning cost work, branching hierarchy, the beech's values; fn-62 reruns Tune on top.

## Acceptance Criteria

- **R1:** `codominance` 0.05 alone builds on every preset at seeds 1 and 7; at 0 byte-identical; birch unchanged — met (`forkDivergence` default 90, `forkLean` 26; `suite::codominance::a_little_codominance_alone_builds_on_every_table`, red on the old defaults).
- **R2:** A bundle that fails to build is split, a lone failing move dropped and logged, no bundle drawn twice — met (5 tests in `tests/tuning_engine.rs`, regression from `tests/fixtures/fn62-refused-bundle.json`; halves rounds buy no proposal and do not count toward the runaway limit).
- **R3:** A run seeded with the beech's curated references needs no hand patch and runs no photograph search — met (`tests/curated_references.rs`; a mismatched file is refused).
- **R4:** Gates green — met (workspace 1,088 with a GPU, `npm test`, `scripts/test-reviewers.py`).

## Result

The beech Tune round was extended live (8 Jev calls, 2 Opus sheet calls) and 27 answers no replay opens were removed. Identity digests re-pinned (refusal wording). Codex SHIP after a host-reset review cycle.

## Open

The proposal tape key hashes the whole effective wire; the third live re-record it has cost.

## Friction

- Two changed fork defaults re-recorded the beech tape again (fn-170, fn-177) and the replay needs examples built by hand -> a tape key not hashing the whole wire, still open.
- The review cap of 2 refused the re-review of the last fix -> host reset; a higher cap for tuning control-flow tasks proposed.
