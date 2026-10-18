# fn-257 task 11: where the site field's time goes (worker, 2026-10-16)

Stopped partway, at the host's word: the owner re-scoped fn-257 (R8, the field becomes a splat of the build's drawn records), so the primitives, their index and the per-cube query are to be deleted. This file holds what was measured. No remedy is committed. The measured remedy for (a) is kept as a patch in `raw/t11/uncommitted-remedy.patch` (gitignored).

Seed 1, RTX 3080, warm. Another worker's GPU jobs kept the desk 70-99 % busy for 13 of the 16 pairs, so only the paired differences mean anything. The binaries: base `e8fc62a34` plus the measurement tests, against the same with the remedy. The script is `field-time.sh` and the raw lines are in `raw/t11/`.

## (a) The stages `growField` runs, and what the field reads

`field/make.rs` `grow`: `tree::drawn`, then `build::readied` (hang, folds, sweep), then the stations, then the field's own primitives and index. A harness build's rings, placement and handoff never ran on the field path.

Each stage, its time (single `CYCLE_TRACE` run; spruce, oak, birch, ms), and what the field reads of it:

| Stage | Spruce | Oak | Birch | The field reads |
|---|--:|--:|--:|---|
| `tree::drawn` (tables, growth, settle, plan, dress, trunk) | about 381 warm (SPRUCE.md) | 228 | 197 | The drawn records and the trunk's root and stem, all of them. The dress's unread words (colour, season) are written in the same kernel, so they are not a stage to drop. |
| hang (`bases`) | 0.05 | 0.02 | 0.02 | The hung nodes and their `BOOT` flag |
| folds → expansion | 1.8 | 1.6 | 1.6 | The leaf reach, canopy, element, bearing bounds, section and station rows |
| sweep: `gather` | (in 7.1) | (in 1.1) | (in 0.8) | The nodes laid out by id (`nodes`), which the stations and primitives read |
| sweep: `lead`, `ties`, `starts`, `walk`, the bitonic sort, `block`, `blocks`, `place`, `emit`, two count readbacks | **about 16** | **about 5** | **about 5** | **Nothing.** Their outputs (runs, table, points, clusters, the contact words 21-23) feed only the curve's handoff and each twig segment's contact words 28-31. The field reads segment words 28-31 only on frond segments, which write them themselves. |
| stations | 16.6-24 | 8.7 | 4.9 | Segment words 0-27 (28 on fronds) and each segment's node |
| primitives | 12 | 5.5 | 7.7 | Its own |
| index | 3.9-7 | 0.6 | 0.6 | Its own |

The sweep alone is 23.0 / 6.3 / 6.1 ms for spruce / oak / birch, and `gather` alone is 7.1 / 1.1 / 0.8.

**The remedy, measured and not committed.** `build::readied` was split into `folded` (hang and folds) and the sweep. `Sweeper::sweep` was split into `laid` (`gather`, the narrower entry the build passes through first) and the curve passes. The field called `folded` and `Sweeper::nodes` (`gather` alone), so it never asked for the curve. The stations then read no contact, because `gather` leaves word 21 as `NONE`. The harness's build runs the same dispatches in the same order. It is one pipeline with no second chain.

| Site field, grow + 36³ query (16 paired warm medians) | Before ms | After ms | Paired diff median | After faster |
|---|--:|--:|--:|--:|
| spruce | 498.5 (grow 431.1, query 67.2) | 487.4 (419.5, 67.7) | −11.7 | 16 of 16 |
| oak | 259.8 (246.6, 13.2) | 255.7 (241.8, 13.9) | −3.5 | 14 of 16 |
| birch | 237.7 (206.4, 31.3) | 234.6 (203.2, 31.3) | −3.6 | 16 of 16 |

- **Field answers:** every preset at 24³, 36³ and 48³, byte-identical. That covers 24 of 24 hashes of the box and answer words (`field_answers`, `raw/t11/answers-*.txt`).
- **Harness build:** every preset's segments, leaves, masses and wood curve are byte-identical, 8 of 8 (`build_bytes`, `raw/t11/bytes-*.txt`).
- **Harness `build_time`, 16 paired warm runs:** oak 258.0 → 257.5, birch 238.5 → 237.5, spruce 476.0 → 475.0, every paired median within ±1.5 ms. Not worse.

Under R8 the same question still applies to the splat, which needs the drawn records and not the sweep's curve.

## (b) The query's 65 ms (spruce, 36³ = 46,656 cells)

One `ask`: three buffers (cells, answers, uniform) and one dispatch of 729 workgroups. One submission and one readback. No cell of the spruce, oak or birch was crowded, so there were no extra windows. `field/split_tests.rs` is kept as `raw/t11/split_tests.rs.txt`. It asked the query three ways on the field's own buffers and took the median of nine:

| ms (spruce / oak / birch) | Spruce | Oak | Birch |
|---|--:|--:|--:|
| `query_field` as the package calls it | 68.5 | 14.2 | 32.6 |
| buffer creation (cells, answers) | 0.07 | 0.05 | 0.07 |
| recording (uniform, bind group, one dispatch) | 0.03 | 0.01 | 0.02 |
| launch (the kernel stopped before its first bucket) | 0.07 | 0.10 | 0.08 |
| traversal (buckets walked, each listed primitive's box and bucket tests) | 50.5 | 10.4 | 20.9 |
| arithmetic (the whole kernel less the traversal) | 15.9 | 2.2 | 6.2 |
| readback of the answers (746 KB) | 0.34 | 0.15 | 0.19 |

Every cost that is not kernel work is under 0.5 ms together, so there is no remedy to take. Most of the query is the kernel's own bucket walk. The spruce's index lists 3.59 M entries over 151,704 buckets: a primitive sits in every bucket its box meets. Making that walk shorter would mean redesigning the index, and R8 deletes it.
