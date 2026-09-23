# fn-102 results (2026-09-23)

Base `18da42fa`. First candidate `49f004e2`, whose figures are kept at the foot. Current candidate `028c08e8` (the simplification pass: the wood's vertices are the one ring store). `531d8146` only drops `mesh::Detail` and changes no output. The tools are in `tools/`; raw rows go in `raw/`, which git ignores. The tools hard-code a session scratchpad (a base worktree at `…/scratchpad/base` with its own `target/`); edit `S` and `W` at the top of each script to replay them.

## Design as measured

- For a family with surface contact, the wood is built first, and it records each node's ring edges as vertex offsets. That holds for the serial build and for the parallel one. Leaf placement reads those rings in place from the wood's float32 `positions`: no second sweep, no copy into an f64 ring array. A request for leaves without wood sweeps the leaves' own rings, as base does, because no wood exists to read.
- Checked before relying on it: `contacts_read_from_the_wood_are_the_swept_ones` compares, for every node, the ring neighbourhood and a projected seat on the wood view against `AttachmentSurface::new`. It covers the ordinary family (serial wood) and the spruce (parallel wood). Every node matched.
- The in-place mapping is simple. Each node's `[lo, hi, start, end]` is a wood vertex index rather than an index into a separate ring array. The flare's foot ring is included in the run's rings in both the wood and the sweep. Dropped degenerate triangles touch only indices, never positions. Ring bounds are keyed by `lo / segments`. This key is unique because consecutive ring starts in the wood lie at least one ring apart; the two cap vertices sit between runs.

## R3: byte identity

- **Mesh build** (`tools/meshhash.rs`): all 8 presets at seeds 1 and 7 are identical, 16 of 16.
- **Binding** (`tools/binding.mjs`, release Wasm): every family, both seeds, all 15 output combinations. Ordinary, oak, spruce, birch, telperion, laurelin and beech match on all 7 × 30 builds. The date palm differs on all 30, as the R3 exception allows.

## R4: peak Wasm linear memory

- There is no rise in any of the 7 × 30 builds, apart from the palm's added wood (at most +1 MB).
- The spruce's peak falls wherever wood and leaves are both requested, because the separate ring array is gone. Surface+foliage drops 360 → 285 MB at seed 1 and 345 → 273 MB at seed 7. With the field it drops 365 → 310 and 350 → 296 MB.
- An intermediate layout ran the Plan stage ahead of the wood and raised the oak's surface+field peak by up to 10 MB (see FRICTION.md). The plan now runs where base ran it: after the wood when the stages run in turn, and beside the wood when they run concurrently.

## R4: timings (medians, ms, seed 1)

The machine was shared, with load averages between 3 and 9 during the runs. Growth code this pass did not touch moved by up to 20% in some cells.

**Native, `generation_stages`,** base and candidate interleaved over 5 rounds (spruce: 8 rounds, after `028c08e8`):

| Preset | Total, base → candidate | Peak RSS |
|---|---|---|
| ordinary | 136 → 125 (−8.1%) | 32 → 37 MB |
| oak | 495 → 421 (−14.8%) | 246 → 255 MB |
| spruce | 4871 → 5069 (**+4.1%**) | 359 → 290 MB |
| birch | 2425 → 2402 (−1.0%) | 136 → 146 MB |
| telperion | 1083 → 1044 (−3.6%) | 211 → 234 MB |
| laurelin | 457 → 427 (−6.5%) | 86 → 90 MB |

- The spruce's placement rose 4141 → 4341 ms (+4.8%). Reading rings from the wood's float32 vertices costs more per query than reading the swept f64 ring array. In an A/B on the same binary, the gap was about +10% at first and about +3 to 5% once a query matched its ring store once.
- Every other family runs wood beside the leaves, and each is faster overall. Their wood stage's own wall time rises by 3 to 27% while it shares the cores. Their RSS rises by 4 to 23 MB for the same reason.

**Wasm binding** (serial, as in production):

- **Spruce** (after `028c08e8`, 5 builds a side): surface+foliage 5999 → 6134 ms (+2.3%). Everything at once 5809 → 6270 ms (+7.9%). Field only 158 → 138 ms (−12.7%).
- **The other five families:** `coreMs` moves between −21% and +6%, with the bytes identical. The per-cell rows are in `raw/`.

## R4 verdict: NEEDS_HUMAN

- For the spruce, the whole build is slower than base: +4.1% native, +2.3% to +7.9% in Wasm. That is the per-query cost of reading float32 vertices in place. In exchange, the spruce's peak memory falls by 50 to 75 MB in Wasm and by 70 MB native.
- Every other family is no slower overall.
- The owner decides whether that trade holds for the spruce. The alternative is a faster ring read, for example a placement microbenchmark and a tuned query. Either way this is a design call for the host, not the pass.

## R7, R8

- **R7** (`one_sweep_and_one_element_serve_a_request`): each request runs one sweep and builds one element. A seated request runs in turn.
- **R8** (`every_family_builds_the_same_bytes_under_either_schedule`, `the_earliest_failing_stage_answers`): serial and concurrent schedules produce byte-identical output for every catalogue family, and they are concurrent exactly where the family has no contact. A failing wood beside failing leaves returns the wood's error. A failing wood beside a failing element returns the element's error (the Plan's), under either schedule.

## Gates (HEAD)

`cargo test --profile ci --workspace --no-fail-fast`: 928 passed, 0 failed, 21 ignored. The first run failed on `generation_limit_guard` alone (moved loop sites; inventory updated). `npm test`: 124 of 124. `npm run rust:test:wasm`: passed.

## Lines (`git diff --numstat 18da42fa..`, crates)

| | Production | Tests | Examples |
|---|---|---|---|
| First build `04b34fd6` | +877 −214 = 663 | +123 −50 = 73 | −15 |
| This pass `028c08e8` | +842 −245 = 597 | +155 −50 = 105 | −15 |
| With `mesh::Detail` gone, HEAD | +847 −264 = 583 | +237 −208 = 29 | −20 |

## First candidate `49f004e2` (superseded)

- R3 was identical then too.
- Its sweep-first sharing raised the Wasm peak by 7 to 24 MB in trial layouts before settling on flat.
- Native totals were −0.4% (spruce) to −35.8% (oak, noisy round).
