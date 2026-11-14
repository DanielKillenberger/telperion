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
- R5 stills: the headless example as fn-283 ran it (`.flow/evidence/fn-283-a-frond-draws-its-rachis/RESULT.md` "Stills"). The palm at 8, 12, 15, 20, seed 1, and the spruce and the birch at 40, seed 1, base against change. Add one walk still through a young palm if the film has one, and one far-view still of the palm at 12, seed 1, at a distance where thinning is active (host ruling: it measures the dip of enlarged leaves, outside R4). View at most four images. Put the pairs on contact sheets for the owner.
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
The closing proof for fn-301 is in place. `ground_reach` is un-ignored and green on all 168 rows, and it now reports each row's bark. The R3 digests, the paired build_time runs and the R5 stills are recorded in `.flow/evidence/fn-301-a-tree-rests-on-its-ground/RESULT.md` and `HASHES.md`. `docs/pipeline.md` is corrected. The device's bloom axis is now one shared WGSL function.

- **ground_reach (R1, R4).** It asserts, per vertex, that nothing lies below the ground except a leaf stalk's dip within `b·scale − h`. The largest dip is 9.3 mm, on the palm at 12. Under host ruling 2 the table gains `bark_low`: the lowest non-trunk curve centreline less its radius, reported and not asserted (`r1-change.tsv`). It is a sphere bound. It reads below 0 on 42 rows where the drawn vertices are on or above the ground, Laurelin at 80 s42 among them (−0.016 m, and her bytes are unchanged).
- **R3 digests (168 rows, base d600fa12c, each side with its own target/).**
  - The beech, Telperion and Laurelin are byte-identical on all 63 of their rows.
  - All 47 expected rows changed.
  - Three more rows changed: ordinary 40 s7, oak 40 s7, spruce 20 s42. Their bark is clear on the base (+0.0913, +0.2431, +0.0807 m). With the landing ease alone put back on the centreline (a scratch patch, reverted, empty diff), all three return to the base's bytes. That proves the cause: the landing ease reads the bark across its whole zone. R3's first bullet in the spec is amended (host, 2026-10-21).
- **bloom_axis (host ruling 3).** `place.wgsl`'s copy of `Axis::laid`'s loop is deleted, and `bloom.wgsl`'s `bloom_axis` serves `floret` and `inflorescence_rest`. The device organ words for Telperion and Laurelin at 12 and at their own ages, seeds 1, 7 and 42, are byte-identical with and without the change (scratch `t5-organ-digest.rs`). The floret, organ, stalk, route, select and shader tests pass: 16 tests (raw/t5-bloom-tests.log).
- **build_time, warm ms, base then change, two runs.**
  - beech 40: 180/177 and 174/174.
  - palm 12: 41/41 and 45/41.
  - spruce 40: 483/475 and 463/479.

  All are within noise. Instance counts and device peaks are equal on both sides.
- **Stills.**
  - The sheets are `stills-palm.jpg` (palm 8, 12, 15, 20 s1), `stills-wood.jpg` (spruce 40 s1, birch 40 s1, oak 40 s7) and `stills-far.jpg` (palm 12 at 600 m, where thinning is active). The PNGs are in `raw/stills/`, and the ordinary tree at 40 s7 is there too.
  - On the base the lower fronds are cut off at the ground. On the change they lie on it, with the dead skirt pale beneath the crown.
  - The spruce, birch, oak and ordinary tree show no visible change at the hero view.
  - The far palm is about 10 px across, and no dip is visible.
  - The film has no walk through a young palm.
  - The owner's verdict is pending.

Gate: `telperion-heavy 8 --threads 2 -- cargo test --profile ci --workspace --no-fail-fast` ran once at 110d8566f: 1,455 passed, 0 failed, 112 ignored, rc 0 (raw/t5-gate.log). `npm test` is red on `catalogue:check`: `catalogue/date-palm/README.md` differs from its regeneration. That failure is inherited, because the base d600fa12c fails the same way and this branch touches nothing under `catalogue/` or `scripts/`. vitest alone passes, 22 files and 177 tests (raw/t5-vitest.log). The page is outside the Touches and was not regenerated; it is a follow-up for the host.

Baseline: green via handoff (fn-301-a-tree-rests-on-its-ground.4, 4d21146c8).

Decisions:
- Host rulings applied: the three extra rows join R3's changing rows, with R3 amended; `bark_low` was added; `bloom.wgsl` was added to the Touches for the dedup.
- Scratch programs ran outside both worktrees or were reverted, and their sources are kept as evidence: `t5-digest-main.rs`, `t5-bark-main.rs`, `t5-organ-digest.rs`.

Open:
- `npm test` is red on an inherited stale catalogue page.
- The owner's verdict on the stills, and on the far-view dip.

Friction: three entries added to FRICTION.md (R3 digests, bark check, npm test).

Tier: implementer (session model) (model: claude-opus-5-5)

stage: impl-review - ran (codex, three lenses, round 1 SHIP; no findings)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 23cbe8fad43ad3c86644e3718512cd1c33834081, e7aa618a9c0e6c801bed136272dde5801abfe20f, 4e4a065e86955f8e56f629cb5252481abacc36d2, 0461559214300392a0cce923c8173d2f2bc63468, aa5a5e9587a5a244c7c1b3046df9b28a4e140f7f, 110d8566fa52809ad0e892fd477a374954181c77, 9cd5a394bc856f90e2c845b78fa1c866bdc0471d
- Tests: telperion-heavy 8 --threads 2 -- cargo test --profile ci --workspace --no-fail-fast (1455 passed, 0 failed, 112 ignored), cargo test --profile ci -p telperion-core --test ground_reach -- --nocapture, npm test (red: inherited catalogue:check on date-palm README; vitest 177 passed)
- PRs: