# fn-357 task 1: a retired root keeps only a stand

A retired root now leaves every per-axis store at compaction, with the axes inside it, and keeps a 24-byte `Stand`: bearer, node, birth, vigour (kept share in) and living. The tree is the same to the bit. On the oak at 120 the grower's footprint falls from 7.106 to 4.795 GiB (−32.5%), the spec's expected third.

Tier: product code, built by Opus to the host's written design (model: claude-opus-5-5).

stage: impl-review - skipped(config: REVIEW_MODE=none; one codex review at the phase's landing)

Baseline: green via handoff (narrow space and core 771 of 771 at `cb634ffee`, vitest 218); `cargo fmt --all --check` green at the base.

## R1: the reading

`READING.md` (committed `954543d7d`, before any product change). A retired root is read only through its bearer: by the settle's fades and by any later evaluation of a subtree holding it, as long as its bearer is held and not retired. Its own fade is none for good, so the settle always drops it. Fields still read: bearer, node, birth, the vigour of its birth draws (with a woken bud's kept share, final from retirement), and its living. No read needs anything else, so no stop.

## What was built

- `shed.rs`: `Stand`. `fades` carries each stand into its bearer before the backward loop (a maximum, order-free for these values). `shed_past` and `subtree_living` take the stands.
- `compact.rs`: `gone` lets go of a retired root with its insides. `Grower::stands` builds each root's stand from the dense stores before renaming (vigour as `mark` and `assign` compute it, kept share at the age grown). It keeps earlier stands whose bearer is held, renamed, sorted by bearer.
- `retire.rs`: `Retirement::stands`. `loose` counts a root too, so compaction triggers on roots as well. An ancestor's evaluation carries the stands of each axis it descends through. `roots()` counts stands whose bearer is held and not retired, so the census's `retired` is unchanged.
- `settle.rs`: the settle carries `Retirement::standing()`, the stands whose bearer is open or kept.
- `footprint.rs`: held axes by kind (open, closed, retired roots, inside) and the `retire.stands` store. `examples/footprint.rs` prints both and the seconds grown (commit `429860382`, before the change; the before binary was built from it).
- Docs (R4): a paragraph in `docs/pipeline.md` under "The one cycle and its grower"; module docs of `compact.rs` and `retire.rs`.

## R2: equal to the bit

| Check | Result |
|---|---|
| Narrow space and core (`raw/narrow.log`), fn-355's command | 770 run, 770 passed, 74 skipped, 891 s. The handoff counted 771; that base count was not rerun here |
| New: `a_retired_root_let_go_is_read_as_it_was` (oak lit, spruce, beech; eager let-go vs records kept, at 12, 25, 40) | green. Every kept subtree's living and every retired root's (birth, vigour, living) are equal to the bit, and more than 100 stands are held |
| The new test against two mutants | fails with the stand carries removed from `fades` (`raw/mutant-new-test.log`) and with the kept share left out of a stand's vigour (`raw/mutant-nokept2.log`). `letting_go_of_retired_records_leaves_the_tree_as_it_was` passes under both (`raw/mutant-nostands.log`, `raw/mutant-nokept.log`), so it is not sensitive enough alone |
| `letting_go_of_retired_records_leaves_the_tree_as_it_was`, `a_tips_path_survives_letting_go_and_a_pause`, `a_tree_paused_and_grown_on_is_the_tree_grown_at_once` | green, unchanged |
| Oak 80, seed 1002, `footprint tree` before and after | digest 5968021473381374776 both, 4,683,475 phytomers, 829,633 axes |
| Digest pins | none moved; no existing test changed |

Probe (`raw/probe-binds.log`, reverted): in the let-go test's growth, 1,036,884 stand carries were taken, and 54,377 raised their bearer's living above its own at that moment. In the test trees, 423 roots were woken buds, one with a kept share of 0.8166 (`raw/probe-woken.log`).

## R3: measured

Seed 1002, one thread (`--threads 1`), `footprint stores`, before (`429860382`) then after (`61fcfe5db`) for each tree, back to back (`raw/measure.sh`, logs `raw/m-*.log`).

| | Spruce 40 | Oak 80 | Oak 120 |
|---|---|---|---|
| Grower footprint, GiB | 1.026 → 0.715 (−30.3%) | 3.197 → 2.434 (−23.9%) | 7.106 → 4.795 (−32.5%) |
| Process peak VmHWM, GB | 0.997 → 0.737 | 3.36 → 2.74 | 7.47 → 5.42 |
| Heap most, GiB | 1.140 → 0.779 | 3.544 → 2.946 | 8.774 → 5.691 |
| Held axes: open | 162,098 = | 618,292 = | 1,213,147 = |
| Held axes: closed | 149,939 = | 418,747 = | 928,424 = |
| Held axes: retired roots | 344,976 → 124,033 | 789,930 → 340,488 | 1,935,628 → 289,805 |
| Held axes: inside retired | 48,869 → 7,581 | 410,360 → 150,859 | 1,004,477 → 96,142 |
| Stands (24 B each) | 233,265 | 708,943 | 1,897,206 |
| Seconds grown, one thread | 3.6 → 3.6 | 30.1 → 28.9 | 115.6 → 109.2 (cumulative from 0) |

Notes on the table:
- **Retired roots after** are those retired since the last compaction. Stands include some whose bearer retired after it; the next compaction drops those.
- **Largest stores at 120, GiB, before → after:** axes 1.435 → 0.711, pencils 1.287 → 0.637, draws 0.891 → 0.441, successor 0.148 → 0.074, links 0.146 → 0.072, by_node 0.146 → 0.072. `axes.phytomers` is unchanged at 2.018.
- **The load window:** the 1-minute load averages were 24.6 at the spruce's start, 21.0 at the oak before, 4.5 at the oak after and 1.8 at its end. It fell while the oak before ran, so the oak's 6% in seconds is not a measured gain.
- **`footprint tree` at oak 80 (growth and settle, 8 threads):** VmHWM 3.49 → 2.92 GB.

## Not run

- The full gate, by instruction.
- vitest: no TypeScript or Wasm surface changed.
- The two sub-second guards: left for the phase's gate.

## Friction

None recorded. Heavy-queue waits were short; the scoreboard job left room.
