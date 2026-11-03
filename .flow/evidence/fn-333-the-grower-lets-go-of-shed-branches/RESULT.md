# fn-333 result: the grower lets go of shed branches' records

3 Nov 2026, worker. `oregon-white-oak`, seed 1, its preset's light, the full
CPU grow (`grow_cycle`, the settle and lay included), one thread: pinned with
`taskset -c 7` (`available_parallelism` reads the affinity, so the grower
takes one core) under `telperion-heavy … --threads 1` (`RAYON_NUM_THREADS=1`).
Tool: `crates/telperion-core/examples/footprint.rs` (`tree` prints the tree's
`tree_digest`, its seconds and VmHWM; `stores` the grower's stores). Binaries:
`before` built at 68a020dc6 (the measurement, no grower change), `after` at
7005956d4 (records let go, stores shrunk after each compaction), `kept` at
57c26df87 (the same, capacity kept). Raw lines: `raw/tree-*.txt`,
`raw/timing-*.txt`, `raw/stores-*.txt` (ignored).

## The design

The breakdown (BREAKDOWN.md) put 95 % of the axis records at 200 inside
retired subtrees, about 1 KB each across `axes`, `pencils`, `draws` and the
working arrays; the retired roots, 2 %, stand for their subtrees and stay.

Once a retired root's upkeep is frozen (`Working::frozen`), nothing reads
the axes inside its subtree again: the balance stops at the root's frozen
upkeep, an ancestor's evaluation at its fate, the girth at its frozen girth,
the settle's fades at its living. `compact.rs` drops those axes from every
store the grower keeps by axis and renames every index into them: the
`Axis` records and their parents, `draws`, `horizon`, `units`, `roots`,
`successor` (and its entries), the pencils, the living apexes, the sleeping
buds' bearers, the woken buds (those inside dropped, the rest renamed, and
the retirement's map to them), and in the working set `links`, `pa`,
`by_node` (and its entries), `frozen`, `open`, `list`, `closed`, `at`, the
pass arrays, `fate`, the frozen girths, `pending` and `fading`. The held axes
keep their order, so every pass visits and sums them in the order it did
(the open list, the preorder, `by_node`'s node-then-index order, the
retirement's sorted `todo`, the settle's index order), and parents still
precede children. It runs at the start of a year, before the year's
`advanced` list is taken, once the loose axes reach a quarter of those
held (at least 4,096), so its cost is spread; a store keeps its capacity
unless it is over twice what it holds.

Why compaction and not a sparse store: the passes address axes by dense
index in a dozen arrays; renaming once in a while touches only the held
axes and changes no pass. Why not the retired roots too: each stands in its
bearer's balance sum (in place, by node, as a float term), its girth, its
ancestor's evaluation and the settle's fades; replacing it with a compact
stand-in would touch all four for 2 % of the records.

## R2: the tree is the same to the bit

| oak, seed 1 | before | after |
|---|---|---|
| 80 | 73692596816797893 | 73692596816797893 |
| 120 | 12621815942631488525 | 12621815942631488525 |
| 200 | 8985413672836356844 | 8985413672836356844 |
| 300 | not run | 5007013851267513276 |

Every run of each binary gave its age's digest (`raw/`). The oak at 300
before the change was not run: `telperion-heavy 18` waited from 16:04 to
16:11 with 1 to 8 GB of room, and the host's own 18 GB oak-300 probe
(fn-330, `oak-300b.log`) had waited since 12:38; I withdrew mine so as not
to compete with it. Its digest and peak are unmeasured; by the 0.05 to 0.06
GB a year the oak added past 80, its peak would be about 15 to 16 GB. The pinned digests
at age 16, every preset at both ends of the dial
(`every_preset_grows_the_tree_it_grew_at_both_ends_of_the_dial`), pass in
`telperion-core --lib`. A new test,
`letting_go_of_retired_records_leaves_the_tree_as_it_was`, grows the oak (lit),
the spruce and the beech to 40 with records let go every year any are loose
against a grower that keeps every record: the settled tree at 12, 25 and 40,
the finished tree and the census (its shape and every cycle's row) are equal
to the bit, and the grower holds fewer axes (135,550 of 202,951 for the oak,
669,757 of 704,955 for the spruce, 33,098 of 39,810 for the beech).

## R3: the gain

Peak resident memory (VmHWM), the same in every run of a binary:

| age | before | after | ratio |
|---:|---:|---:|---:|
| 80 | 1.97 GB | 1.26 GB | 0.64 |
| 120 | 5.08 GB | 2.19 GB | 0.43 |
| 200 | 9.48 GB | 2.22 to 2.24 GB | 0.24 |
| 300 | not run | 2.22 GB | |

The 300 "before" is not measured, and the host declines to measure it: the
old grower needs about 18 GB there, the room the owner asked this child to
stop waiting for ("we should get memory usage down so we don't need to wait
for 18gb memory", 3 Nov 2026), and the desk had 10 GB free all afternoon.
What R3 asks the comparison to show holds without it: before, the peak
grows with every record ever laid (1.97, 5.08, 9.48 GB at 80, 120, 200);
after, it stops at the busiest years near 120 and is the same at 200 and
300.

The grower's heap at 200, end of growth: 11.58 GiB before, 1.14 GiB after
(`stores`); 589,067 axes held of 9,929,933 grown. The peak after is now the
busiest years near 120 (heap most 2.36 GiB), no longer the end: past 120 the
memory follows the live tree, 1.97 GiB held at 120, 1.14 at 200.

Seconds. The desk was shared with another checkout's gate and an 18 GB
probe for the whole run; the load average went from 2 to 54, and the same
binary's seconds moved by up to 2x. So each comparison is a pair run back to
back on the same core, before then after, and the table gives every pair
with the load at its start:

| age | load | before s (user, sys) | after or kept s (user, sys) |
|---:|---:|---|---|
| 80 | 7.1 / 6.1 | 18.5 (17.4, 1.0) | after 18.4 (17.2, 1.1) |
| 80 | 6.0 / 4.7 | 18.3 (17.1, 1.0) | after 21.9 (20.4, 1.3) |
| 120 | 5.1 / 4.8 | 59.3 (53.7, 5.2) | after 59.7 (51.8, 7.5) |
| 120 | 7.6 / ? | 60.8 (54.4, 5.5) | after 84.7 (73.5, 9.1) |
| 200 | 2.7 / 2.0 | 102.9 (92.1, 10.2) | after 106.4 (90.8, 15.1) |
| 200 | 1.5 / 7.5 | 107.1 (97.0, 9.4) | after 137.5 (120.2, 15.4), a build beside it |
| 200 | 8.1 / 54.1 | 205.9 (184.7, 17.6) | kept 208.2 (196.8, 9.3) |
| 200 | 39.3 / 30.1 | 202.9 (184.9, 15.8) | kept 125.9 (120.3, 4.6) |
| 300 | 9.3 | not run | kept 196.3 (186.9, 6.7) |

Read: the user seconds of a quiet pair are the same (92.1 against 90.8 at
200; 53.7 against 51.8 at 120). Shrinking every store after each
compaction cost 5 s of kernel time at 200 (15 s against 10), the pages
handed back and faulted in again as the tree grew into them; keeping the
capacity (57c26df87) brought it to 5 to 9 s. The two noisy after pairs
(21.9 at 80, 84.7 at 120) carry more user time too, which the change does
not add: their neighbours on the desk did.

## R4: what read the records let go

| reader | what it read | now |
|---|---|---|
| `Working::totals` / `closed` (balance) | a closed child's subtree, to freeze its upkeep | reads the root's frozen upkeep; a subtree goes only once its root's is frozen |
| `retire::evaluate` / `subtree` / `copied` | a closed subtree, stopping at any axis whose fate is decided | never reaches inside a retired root; unchanged |
| `retire::freeze` / `frozen_at` / `ratio` | the children of an axis closing now | never inside; unchanged |
| `retire::let_go` | the children of a subtree retired now | a former root's children are already gone, its phytomers too; it marks what is held |
| `retire.pending`, `retire.fading`, `faded_out` | subtrees waiting; skipped once inside | entries inside dropped, the rest renamed |
| `Grower::mark` (settle) | woken buds, skipping those inside | woken buds inside dropped; `retire.woken` and `seen` renamed |
| `presence::assign` (settle) | every axis on its own, and a relay's parent | no held axis has a parent let go |
| `girth::attachments_past` (settle) | each axis's own phytomers' terms, a root's frozen girth | the axes let go held no phytomers and added no term; same terms in the same order |
| `shed::shed_past` (settle) | the axes inside, as gone | the same axes dropped later; the kept tree and `index` are the same |
| census: `axes` ("grown hierarchy"), each cycle's `axes`, `depth_all`, `attachments`, `depth_busy`, `by_order` | each axis's depth, order and births, by index | the axes let go are counted by depth and births (final once closed); every number is the same, the test holds it |
| captures (`grow_capture`, `_hierarchy_at`, `_light_at`, `_retired`) | every axis by index, for the device's passes from scratch | keep every record (`Working::indexed`); unchanged |
| `retire.log` | every verdict by its root's index | written only where records are kept (the captures); `paused::tests::eventful` reads it from an indexed grower |
| `Paused::digest` | the grower's state | the compacted state; compaction runs from that state at a year's start, so any pause points give one digest (`a_tree_paused_and_grown_on_is_the_tree_grown_at_once` passes) |
| `successor` of a retired root | its continuation and relay, both inside | emptied; no reader reads a retired root's successors |

Changed on purpose: `Census::Cycle::axes` and `Census::axes` still count
every axis grown, held or let go. `capture.rs`'s `per_cycle_axes` is
unchanged because the captures keep records. Nothing else reads the
records.

## R5 and the narrow checks

Run through `telperion-heavy 8 --threads 4`, not the gate (the dispatch's
scope):

- `cargo test --profile ci -p telperion-space`: 224 passed, 0 failed, 7
  ignored, on 7005956d4.
- `cargo test --profile ci -p telperion-core --lib`: 477 passed, 0 failed, 46
  ignored, the pinned digests included, on 7005956d4.
- `cargo build --profile ci -p telperion-render --tests --examples`: built;
  its warnings are the render crate's own, untouched.
- `generation_limit_guard`: 2 passed. `compile_size`: 2 passed.
- On 57c26df87, which changes only when a store gives back its capacity:
  `cargo test --profile ci -p telperion-space --lib -- grow::cycle`, 13
  passed, 0 failed, 2 ignored (the new test and the paused tests among
  them); and every `kept` run above gave its age's digest.

Not run: the gate (`scripts/gate.py`) and the codex review, which R5 asks for
and the host owns.
