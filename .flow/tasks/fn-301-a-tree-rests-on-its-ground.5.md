---
satisfies: [R1, R3, R4, R5, R6]
---
# fn-301-a-tree-rests-on-its-ground.5 Prove it, measure it, show it

## Description
The spec's closing proof: R1 green, R3's identity and cost, R5's stills, the doc fix, and the gate.

**Size:** M
**Files:** `crates/telperion-core/tests/ground_reach.rs`, `docs/pipeline.md`, `.flow/evidence/fn-301-a-tree-rests-on-its-ground/RESULT.md`
**Touches:** [crates/telperion-core/tests/ground_reach.rs, docs/pipeline.md, .flow/evidence/fn-301-a-tree-rests-on-its-ground/**]

### Approach
- Un-ignore `ground_reach.rs`, and update its doc comment to say it holds contact. It must be green (R1, R4). Record `r1-change.tsv` beside `r1-base.tsv`.
- R3 identity: `crates/telperion-core/examples/artifact_digest.rs` on the base (d600fa12c, in its own worktree with its own `target/`, AGENTS.md) and on the change. Cover the beech, Telperion and Laurelin on every R1 row, and the palm at 40. The example takes preset and seed; add age only if needed, or use a scratch test like fn-283's `HASHES.md`. Record `HASHES.md`.
- R3 cost: paired `build_time` (`crates/telperion-render/src/generation/build_time_tests.rs`, `BUILD_PRESETS`, `BUILD_AGE`, `BUILD_SEED`, `--ignored --nocapture`), base against change on the same machine. Cover a clear tree (the beech at 40), the palm at 12 and the spruce at 40. Two runs each.
- R5 stills: the headless example as fn-283 ran it (`.flow/evidence/fn-283-a-frond-draws-its-rachis/RESULT.md` "Stills"). The palm at 8, 12, 15, 20, seed 1, and the spruce and the birch at 40, seed 1, base against change. Add one walk still through a young palm if the film has one. View at most four images. Put the pairs on contact sheets for the owner.
- `docs/pipeline.md:25`: drop the claim that four families have no engine species. Add one sentence where the stage is described: contact lives in the lay walk and the frond curve, and the pose where instance matrices finish.
- The gate once at the end, `telperion-heavy 8 --threads 2 -- cargo test --profile ci --workspace --no-fail-fast`, and `npm test`.

### Investigation targets
**Required:**
- `crates/telperion-core/tests/ground_reach.rs`
- `crates/telperion-core/examples/artifact_digest.rs`
- `crates/telperion-render/src/generation/build_time_tests.rs`
- `.flow/evidence/fn-283-a-frond-draws-its-rachis/RESULT.md`

### Key context
At most one full-forest capture per commit; this task needs none. Raw logs go to `raw/` (gitignored).

## Acceptance
- [ ] `ground_reach` is un-ignored and green.
- [ ] `HASHES.md`: the beech, Telperion and Laurelin on every R1 row and the palm at 40 are byte-identical to the base.
- [ ] Paired `build_time` recorded: the clear tree within noise; the palm at 12 and the spruce at 40 recorded.
- [ ] The stills (base against change) are in `RESULT.md`, with their paths.
- [ ] `docs/pipeline.md` is corrected; the gate and `npm test` are green.

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
