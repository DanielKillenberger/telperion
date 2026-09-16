---
satisfies: [R1, R2, R3, R4]
---
# fn-44-pendulous-shoots-hang-under-gravity.1 Implement Pendulous shoots hang under gravity

## Description
The curtain fn-37 made was a set of rods: a lateral departed in one fixed
direction and ran straight. The owner on the round-5 birch: "the hanging
curtains are not affected by gravity or smth. It's clearly wrong." This task
adds `skeleton.twigs.sag` (0 to 1, neutral 0): a hanging shoot's course is
turned toward straight down along its run, steepest at the attachment,
carried as a heading beside the course so the branch law's turn limit does
not bound weight. The birch states sag 1 and its pairs are round 6b.

## Acceptance
- [x] **R1** the sag row railed and refused by name, on the wire, blended,
      on the harness dial and in the regenerated metadata; every shipped
      preset byte-identical at neutral, the birch re-pinned once with the
      reason.
- [x] **R2** the heading turns toward down monotonically as an ease-out arc
      over the pendulous length; droop, floor and separation are fn-37's.
- [x] **R3** the birch states sag 1; round 6b rendered and recorded beside
      round 5 in REPORT.md and `.flow/evidence/fn34/rounds.tsv` (the
      `round6b-fn44` rows); 48/48 protocol.
- [x] **R4** `tests/sag.rs`: neutral identity, the rail, the monotone turn
      and end angles at sag 1 and 0.5, no step turning more than the last,
      the floor, determinism, a blend walk.

## NEEDS_HUMAN — the owner's verdict on the round-6b pairs

R3 reserves the verdict to the owner. The pairs are under
`.flow/evidence/fn34/measure/pairs-fn44/`, recorded by sha256 in
`.flow/evidence/fn34/rounds.tsv` (the `round6b-fn44` rows); no visual
pass is awarded, and on the
judging page. Commits aa8aaf1, f594569, 23287c4; the host read the sag law
and the planner's course and heading split. Gates green: core (28 binaries),
render (22), clippy, typecheck, wasm, compare self-test.

One measured limit for the owner: the sag is promised over the pendulous
length (3.5 m) but the birch's shoots run 1.07 m on average before the floor
or the shell stops them, so they spend the first third of the arc (19 to 34
degrees below horizontal). Hanging them further is a shorter pendulous
length (fn-37's row, which also shortens the shoots) or a sag law over the
shoot's own run; the owner decides on the pair.

## Done summary
Hanging shoots sag toward vertical along their run by a row; neutral is the straight rod. The owner accepted the silver birch at fn-34 round 25 (2026-09-16); the European beech's verdict moved to fn-62.
## Evidence
- Commits: e140791b
- Tests: cargo fmt --all -- --check, cargo clippy --release --workspace --all-targets -- -D warnings, cargo test --release -p telperion-core --no-fail-fast, cargo test --release -p telperion-render --no-fail-fast, npm run typecheck, npm run rust:test:wasm, npm test, uv run scripts/compare-references.py --self-test, node tests/species.mjs --measure-only (48 cases, measure/protocol-round25)
- PRs: