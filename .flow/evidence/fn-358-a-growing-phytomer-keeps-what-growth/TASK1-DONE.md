# fn-358 task 1: a growing phytomer keeps what growth reads

The grower now holds each phytomer as a 72-byte `Growing` record, where the tree's `Phytomer` is 136 bytes. It keeps every field growth reads, at the width growth wrote it, and leaves out the radius. The settle widens each record back into a `Phytomer`, to the bit. On the oak at 120, the phytomers' store falls from 2.018 to 1.069 GiB (−47%) and the grower's footprint from 4.795 to 3.845 GiB (−19.8%). The spec expected about a half and a fifth.

Tier: product code, built by Opus to the host's written design (model: claude-opus-5-5).

stage: impl-review - skipped(config: REVIEW_MODE=none; one codex review at the phase's landing)

Baseline: green via handoff (narrow space and core 770 of 770 at `8d15b662c`; oak 80 at seed 1002, digest 5968021473381374776). No commit since the handoff touches a path outside `.flow/`.

## R1: the reading

`READING.md` was committed as `355ff7829`, before any product change.

The rough layout reads the geometric fields, so tip, heading and side cannot leave the grower. A lateral's frame reads its node's tip, heading, side and rank, and a woken bud's node can be old. A relay's stand reads tips, frames, scales and grown shares along its line. The leaves read the tips of older phytomers, and the light reads the tips.

Every geometric and scale field growth writes is an exact `f32` image, so each narrows to `f32`. Growth never writes `radius`, so it leaves the grower.

**What stays at full width, and why:**
- `key` (u64): read by the walk.
- `cycle` (u32): a cycle has no bound below 2^16.
- `section` and `sapwood`: already `f32`.

That is the spec's stop applied: tip, heading and side stay in the store, but at `f32`.

**Not changed, read for the next step (READING.md):**
- Once an axis closes, nothing reads its pencil. That is 272 bytes per axis, with 928,424 closed axes held at 120.
- Once an axis closes, its phytomers are read only for their cycle, plus what the settle needs (key, section, sapwood). That is 24 bytes where it now holds 72.

## What was built

- `structure/growing.rs`:
  - `Growing`, the record the grower holds.
  - `Node`, the trait the shared passes read through.
  - `Growing::settled` and `Axis<Growing>::settled`, which widen exactly.
- `structure.rs`: `Axis<P = Phytomer>`. A caller's `Axis` is unchanged.
- Growth-side code types its axes as `Axis<Growing>`:
  - `grow.rs`, `grow/unit.rs`, `grow/draw.rs`, `grow/shoot.rs` and `grow/parallel.rs`;
  - `grow/cycle/working.rs`, `grow/cycle/census.rs` and `grow/cycle/retire.rs`.
- `grow/draw.rs` writes the layout's values as `f32`. `image` asserts, in every profile, that a stored size or rank is an `f32`'s image.
- The settle widens after `mark` (`grow/settle.rs`). Retirement widens its copy of a closed subtree (`retire.rs`).
- These read through `Node`, generic over the record:
  - the walk (`Layer::step`) and sag (`sagged::sag_angle`);
  - frames, stands and attachment (`geometry.rs`, `geometry/stand.rs`);
  - `presence::relay_node`, `Woken::kept` and `dormant::carried`;
  - `girth::leaf`, `girth::bearing` and `girth::lay`.
- Docs (R4): a paragraph in `docs/pipeline.md` under "The one cycle and its grower", saying what the grower holds and what the settle adds.

**Existing tests changed:** one, a fixture following the type. In `grow/cycle/paused/tests.rs`, `livings` now widens the grower's axes (`Axis::settled`) before `presence::assign` and reads them from that copy. No assertion changed, and no digest pin moved.

## R2: equal to the bit

| Check | Result |
|---|---|
| New: `a_growing_phytomer_is_read_as_it_was` (the unlaid oak at 20, beech and spruce at 30, in light, seeds 1 and 7, through `grow::sketch`) | Pinned on the base and committed alone (`ad011cdce`, `raw/pins-base4.log`). Green after the change (`raw/new-test.log`). |
| The new test against three mutants, each reading one narrowed field as 0.0: `grown`, `size`, `rank` | Fails under each (`raw/mutant-*.log`). |
| New: `a_growing_phytomer_holds_what_growth_reads` | 72 bytes against 136. |
| Oak 80, seed 1002, `footprint tree`, before and after | Digest 5968021473381374776 both, 4,683,475 phytomers, 829,633 axes. |
| Narrow space and core (`raw/narrow.log`), fn-355's command | 772 run, 772 passed, 74 skipped, 882 s: the handoff's 770 and the two new tests; the paused, year, repeat and let-go tests among them, unchanged |

## R3: measured

Seed 1002, one thread, `footprint stores`. Before is `8d15b662c`, built in a copy of the base with its own target, and after is `89fa7b33b`. Each tree was run before then after, back to back (`raw/measure.sh`, `raw/m-*.log`).

| | Spruce 40 | Oak 80 | Oak 120 |
|---|---|---|---|
| Grower footprint, GiB | 0.715 → 0.632 (−11.6%) | 2.434 → 1.967 (−19.2%) | 4.795 → 3.845 (−19.8%) |
| `axes.phytomers`, GiB | 0.178 → 0.094 | 0.992 → 0.525 | 2.018 → 1.069 |
| Process peak VmHWM, GB | 0.740 → 0.654 | 2.744 → 2.264 | 5.426 → 4.343 |
| Heap most, GiB | 0.779 → 0.695 | 2.946 → 2.560 | 5.691 → 4.936 |
| Seconds grown, one thread | 3.5 → 3.3 | 28.5 → 27.5 | 109.0 → 104.8 (from 0) |

- **Load window.** The 1-minute load was 3.5 at the spruce, 3.4 at the oak before's start, 1.6 at the oak after's start and 1.4 at its end. It fell during the oak's pair, so the 4% in seconds is not a measured gain.
- **The whole tree at 80** (`footprint tree`, growth and settle, 8 threads): VmHWM rose from 2.99 to 3.11 GB. The settle widens every held phytomer into a new 136-byte record before it sheds, and the process peak now falls in the settle, not in growth. The cause of the extra 0.12 GB is not measured; freed 72-byte blocks the allocator cannot reuse for 136-byte ones is a guess. Widening only the axes the shed keeps would remove it, but the shed reads the assigned scales, so that is a design question for the host.

## Next step (not built)

These take the next bytes, by the reading:
- Let go of a closed axis's pencil (0.637 GiB of pencils at 120, 928,424 of 2,527,518 held axes closed).
- Shrink a closed axis's phytomers to cycle, key, section and sapwood.

Both need a second per-axis representation once an axis closes, which is design work for the host.

## Not run

- The full gate, by instruction.
- vitest: no TypeScript or Wasm surface changed.
- The two sub-second guards: left for the phase's gate.

## Friction

`FRICTION.md` has two entries. The queue had 0 to 1 GB of room from 08:13 to 09:03 while the scoreboard's 40 GB scope ran. Checks, the new tests and the mutants ran in 1 GB scopes, and one paused-test run in 1 GB was OOM-killed (`raw/narrow-paused2.log`, inconclusive). R3 and the narrow suites waited for room.
