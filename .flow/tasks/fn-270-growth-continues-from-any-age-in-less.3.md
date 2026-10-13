---
satisfies: [R4]
---
# fn-270-growth-continues-from-any-age-in-less.3 Memory, one lever at a time

## Description
R4, a labelled census after each lever.

## Acceptance
R4's targets or each miss measured; guard and full-card run held.

## Done summary
R4's levers applied in order, a labelled census after each (`.flow/evidence/fn-270-growth-continues-from-any-age-in-less/MEMORY.md`). Lever 1: retirement's six per-cycle columns left `ret`, and a draw's settle scratch is freed before growing on. Levers 2 and 4, merged by host ruling: tables start at margin 1.0 and grow by 1.5x by observed counts; the compaction runs before the move, and moves go one table group at a time. Lever 3, by host ruling (B, frozen at close): a closed subtree's girth is frozen, and the records and inside axes are gone. Compactor sharing was dropped.

Measured, decimal MB, base to now:
- Telperion: standalone 4,209 to 2,484 (-41%); continuation 5,876 to 3,072 (under 4 GiB); warm 1,248 ms (base 1,244).
- Laurelin: 3,853 to 2,281 (-41%); continuation 5,403 to 2,836.
- Spruce 2,445 to 1,522; oak 1,272 to 782; birch 800 to 520; beech 304 to 282; ordinary 274 to 169; palm 32.
- Girth shift: at most 5.4e-5. The stills show no visible change.
- Split hashes 0->H == 0->a->H exact on every preset. The moved and unmoved builds are the same tree on 24 of 24. The CPU and device agree on the young presets.
- Wasm: Telperion 3,114 to 2,813.

Missed targets, floors measured:
- Beech 282 (target 250), birch 520 (450), oak 782 (650), spruce 1,522 (1,200).
- Two Trees 2,484 and 2,281 against 2,000, a target the host released.
- Oak at 80: refused at margin 1.0. It builds at 3,715 MB at margin 0.5 and 2,299 at 0.3, but costs R0a time (Telperion +10%, spruce 507 ms).

Gate: workspace 1,356 passed, 1 failed (now held to a girth bound and passing), 89 ignored; npm test 162/162.

Tier: not stated by the conductor.

stage: impl-review - ran [codex: SHIP, 3 draws; contracts P2 fixed in 1479a98b6]

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 5f21707587d6b2abe3b0f99f1e0a3d636b0d0ebe, bcc2e2b6e161ae38de69dc08500b956174d2626d, f59662392e2df0176013a8c55a77b9d6de4528de, 46865341ffe489c6ff4abd5f3aaa42362eac8167, c980c11e63ea8aff705b6bc2580182f9b8050f4a, 9acb54755676cc7ab3501235f8257302030a82d3, 95bc4b7f89a801bad2341304d2b6b0afa46acb7f, d4519b88f9158d252cf7fa7d1b3b5366ef64892d, 1616c7204af597bcc176e54518820f49c10e1967, 9581fed64c2edacabb342ab849f2bebb3f91f496, db538af8b567ae15d27c253bf1d8f45fa7b3ac0d, 1479a98b6213d533499db23f2241ac8abae2c72c
- Tests: cargo test --profile ci --workspace --no-fail-fast, npm test
- PRs: