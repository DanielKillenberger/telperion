# fn-102 results (2026-09-23)

Base `18da42fa`. First candidate `49f004e2`, whose figures are kept at the foot. Current candidate `b0a22f4d`, the review cleanup on top of `028c08e8` (the wood's vertices are the one ring store) and `531d8146` (drops `mesh::Detail`). The cleanup keeps one float32 ring store for swept and in-place rings, one stage branch, test-only counters, and the error order base ran. The tools are in `tools/`; raw rows go in `raw/`, which git ignores. The tools hard-code a session scratchpad (a base worktree at `…/scratchpad/base` with its own `target/`); edit `S` and `W` at the top of each script to replay them.

## Design as measured

- For a family with surface contact, the wood is built first, and it records each node's ring edges as vertex offsets. That holds for the serial build and for the parallel one. Leaf placement reads those rings in place from the wood's float32 `positions`: no second sweep, no copy into an f64 ring array. A request for leaves without wood sweeps the leaves' own rings, as base does, because no wood exists to read.
- Checked before relying on it: `contacts_read_from_the_wood_are_the_swept_ones` (now in `surface/attachment/tests.rs`) compares, for every node, the ring neighbourhood and a projected seat on the wood view against `AttachmentSurface::new`. It covers all 6 catalogue and 2 in-work families forced to surface contact 1.0, each under the default wood build (parallel for every family over 250k vertices, all but the palm) and a forced serial one. That includes fork sockets, caps, the flare and the palm's leaf bases. Every node matched, with no panic.
- The in-place mapping is simple. Each node's `[lo, hi, start, end]` is a wood vertex index rather than an index into a separate ring array. The flare's foot ring is included in the run's rings in both the wood and the sweep. Dropped degenerate triangles touch only indices, never positions. Ring bounds are keyed by `lo / segments`. This key is unique because consecutive ring starts in the wood lie at least one ring apart; the two cap vertices sit between runs.

## R3: byte identity (after the cleanup)

- **Mesh build** (`tools/meshhash.rs`): all 8 presets at seeds 1 and 7 are identical, 16 of 16.
- **Binding** (`tools/binding.mjs`, release Wasm): every family, both seeds, all 15 output combinations. Ordinary, oak, spruce, birch, telperion, laurelin and beech match on all 7 × 30 builds. The date palm differs on all 30, as the R3 exception allows.

## R4: peak Wasm linear memory (after the cleanup)

- No build is above base, except the palm's added wood (at most +1 MB in 16 of 30).
- Against the pre-cleanup candidate, every family is unchanged except the spruce, which falls by up to 34 MB: rings swept for leaves without wood are now float32, half the size.
- The spruce against base: surface+foliage 360 → 285 MB at seed 1, 345 → 273 MB at seed 7, as before.

## R4: timings (medians, ms, seed 1, after the cleanup)

The machine was shared, with load averages of 11 to 17 throughout (logged per preset in `raw/`).

**Native, `generation_stages`,** base and candidate interleaved: 5 rounds for each family, 8 for the spruce.

| Preset | Total, base → candidate | Placement | Peak RSS |
|---|---|---|---|
| ordinary | 155 → 135 (−13.4%) | 14 → 13 | 32 → 37 MB |
| oak | 483 → 414 (−14.2%) | 214 → 215 | 246 → 255 MB |
| spruce | 4720 → 4939 (**+4.6%**) | 4010 → 4230 (+5.5%) | 360 → 290 MB |
| birch | 2695 → 2601 (−3.5%) | 93 → 89 | 136 → 145 MB |
| telperion | 1123 → 1036 (−7.7%) | 160 → 159 | 209 → 234 MB |
| laurelin | 449 → 432 (−3.9%) | 113 → 114 | 86 → 90 MB |

- With a single float32 store, the spruce is where it was before the cleanup: +4.6% against the earlier +4.1%, and placement +5.5% against +4.8%. That is within this machine's noise. The in-place read was already float32, so the cleanup did not change the spruce's query.
- The earlier Wasm timings (spruce surface+foliage +2.3%, everything at once +7.9%) were not re-run.

## R4 verdict: NEEDS_HUMAN

- For the spruce, the whole build is slower than base: +4.1% native, +2.3% to +7.9% in Wasm. That is the per-query cost of reading float32 vertices in place. In exchange, the spruce's peak memory falls by 50 to 75 MB in Wasm and by 70 MB native.
- Every other family is no slower overall.
- The owner decides whether that trade holds for the spruce. The cleanup left it unchanged. The alternative is a faster ring read, for example a placement microbenchmark and a tuned query. Either way this is a design call for the host, not the pass.

## R7, R8, error order

- **R7** (`one_sweep_and_one_element_serve_a_request`): each request runs one sweep and builds one element, and a seated request runs in turn. The counts come from a test-only tally in `pipeline/tests.rs`, recorded on the calling thread. `Stages` no longer carries fields that only tests read.
- **R8** (`every_family_builds_the_same_bytes_under_either_schedule`, `the_earliest_failing_stage_answers`): serial and concurrent schedules produce byte-identical output for every catalogue family. They run concurrently exactly where the family has no contact. A failing wood beside failing leaves returns the wood's error.
- **Error order restored to base.** The wood's error now comes before every Plan-stage error, and the element's before the twig rows', as `mesh::assemble` ran them on base. `the_wood_fails_before_the_plan` and `the_element_fails_before_the_twig_rows` were red on `dd863e02`, which answered with the twig rows' error in both cases, and are green now. This supersedes the earlier "a failing wood beside a failing element returns the element's error". In the binding, `branch_diagnostics` still runs after the pipeline. Its only failure is `twigs.resolved()`, which `branching::generate` already checks before any stage runs, so no request can observe the order.

## Gates (HEAD, after the cleanup)

`cargo test --profile ci --workspace --no-fail-fast`: 930 passed, 0 failed, 21 ignored, on the first run. `npm test`: 124 of 124. `npm run rust:test:wasm`: passed.

## Lines (`git diff --numstat 18da42fa..`, crates)

| | Production | Tests | Examples |
|---|---|---|---|
| First build `04b34fd6` | +877 −214 = 663 | +123 −50 = 73 | −15 |
| This pass `028c08e8` | +842 −245 = 597 | +155 −50 = 105 | −15 |
| With `mesh::Detail` gone `dd863e02` | +847 −264 = 583 | +237 −208 = 29 | −20 |
| Review cleanup, HEAD | +786 −269 = 517 | +292 −208 = 84 | −20 |

## First candidate `49f004e2` (superseded)

- R3 was identical then too.
- Its sweep-first sharing raised the Wasm peak by 7 to 24 MB in trial layouts before settling on flat.
- Native totals were −0.4% (spruce) to −35.8% (oak, noisy round).
