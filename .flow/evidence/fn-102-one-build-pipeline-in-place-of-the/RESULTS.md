# fn-102 results (task .1, 2026-09-23)

Base `18da42fa`, candidate `49f004e2`. The tools are in `tools/` and the raw rows in `raw/`, which git ignores. The tools hard-code this session's scratch paths (a base worktree at `…/scratchpad/base`, which has its own `target/`). To replay them, edit `S` and `W` at the top of each script.

## R3: byte identity

- **Mesh build** (`tools/meshhash.rs`, linked against base and against candidate): all 8 presets (the catalogue plus `IN_WORK`) at seeds 1 and 7. Wood, element, instances and bounds are identical: 16 of 16.
- **Binding** (`tools/binding.mjs` on the release Wasm, a fresh instance per build): every family, both seeds, and all 15 non-empty combinations of surface, foliage, structure and field. Hashed: every output buffer, plus the metadata without timings. 7 families × 30 builds are identical: ordinary, oak, spruce, birch, telperion, laurelin and beech.
- **Date palm:** all 30 builds differ, as the R3 exception allows. The base binding built 73 nodes and 1,658 wood vertices at seed 1. The candidate builds 249 nodes and 7,182 vertices with 4,161 instances, the same as `mesh::build`.

## R4: peak Wasm linear memory

Measured per output combination, over the same 240 builds: no rise for any of the 7 families. The date palm's peak rises by at most 0.4 MB, from the leaf-base wood the fix adds.

The first two layouts that shared the spruce's ring sweep raised the Wasm peak by 7 to 24 MB, with identical bytes; see FRICTION.md. The shipped layout sweeps first only where wood and leaves run side by side. When they run in turn (Wasm), the wood sweeps and the leaves read their rings from its vertices, in base's allocation order.

## R4: timings (medians of 5, ms, seed 1)

The machine was shared: another session's test binary held a load average of about 8 on 32 cores. Code this task did not touch, such as growth, moved 5 to 40% between runs. So neither the native nor the Wasm figures here resolve a 3% gate.

**Native, `generation_stages`.** Base and candidate were interleaved over five rounds. Totals, which run concurrently on the candidate:

| Preset | Total base → candidate | Change |
|---|---|---|
| ordinary | 136 → 123 ms | −9.5% |
| oak | 679 → 436 ms | −35.8% (noisy round) |
| spruce | 4910 → 4889 ms | −0.4% |
| birch | 2427 → 2358 ms | −2.8% |
| telperion | 1221 → 1158 ms | −5.2% |
| laurelin | 458 → 432 ms | −5.6% |

The wood's own wall time under concurrency rose on every preset but laurelin:

| Preset | Wood stage base → candidate | Change |
|---|---|---|
| ordinary | 12 → 15 ms | +25% |
| spruce | 54 → 67 ms | +24% |
| birch | 37 → 39 ms | +5.1% |
| telperion | 57 → 59 ms | +3.9% |

The wood shares the cores with placement while both run. With the candidate forced serial (`GENERATION_SERIAL`, interleaved the same way), every stage sits within ±5% of base in both directions, and every total is 0.1 to 1.7% faster: `raw/r4-native-serial.txt`.

**Native peak RSS, median per process.** The rise runs from +1 MB on the oak's serial run to +23 MB on telperion's concurrent run, because wood scratch and placement are resident together. Concurrent runs: ordinary 32 → 37, oak 247 → 254, spruce 359 → 365, birch 136 → 146, telperion 211 → 234, laurelin 86 → 90 MB. Serial runs: within 1 MB of base.

**Wasm binding** (serial, as in production). Across 18 preset and combination cells, `coreMs` moves between −37% and +10%. The spread tracks the load: growth code this task did not touch moves by up to 14% in the same runs. See `raw/r4-wasm.txt`.

## R4 verdict: NEEDS_HUMAN

Read literally, R4's "no median more than 3% slower" fails on the concurrent wood stage: +24% for spruce, +25% for ordinary. That is inherent in timing a stage by its own wall time while it shares cores (R8). The totals are faster. The owner decides whether the stage gate applies to concurrent wall times or to the serial schedule, and whether this machine's noise is acceptable or the timings must be retaken on an idle one.
