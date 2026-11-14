# fn-351 speed round 2: the grower's structure, the oak at 40

Branch `fn-351-speed-round-2`, from `fn-351-candidate` at `7685ee85b`. Tree: Oregon white oak, age 40, seed 1001, grown through `grow_census` by the `early` example (`three oregon-white-oak 40 1001 cycle`); digest of the grown tree on the base `15453902402947031023` (912,157 phytomers grown, 326,166 kept on 51,271 axes). 14 Nov 2026, Zurich time. The asked aim was a faster growth with the tree the same to the bit, by structural changes fn-368 did not make.

## What the round reached

The round's two hours bought one profile, six candidate changes written as patches with their proofs, and no measured change: the one admission that builds and reads them (`speed2-build.sh`) was queued at 13:21 and had not been admitted when this was written. No change is kept, since none is measured; the branch holds the profile, the method and the candidates. The queue was frozen or full for most of it (FRICTION.md, 14 Nov 12:28, 12:39 and 13:21): the first build took 24 minutes of wall for a few minutes of work, and a 4 GB claim found no room from 12:57 on.

## The base, and why seconds cannot carry a verdict today

One thread, the tree's own clock (`cycle: … s`, growth and settle):

| when | how | clock | CPU to the grown tree | user instructions | cycles | L2 misses |
|---|---|---|---|---|---|---|
| 12:48 | in the queue, desk at load 35 to 48 | 1.48 s | 1.47 s | 15.81 G | 8.97 G | 63.9 M |
| 13:11 | in the queue, pinned to one core, load 9 | 0.95 s | 0.94 s | | | |
| 13:12 | under the sampler, load 9 | 0.98 s | 0.96 s | | | |

The same binary on the same tree spreads by half with the desk's load, so fn-368's 1.44 s is a loaded-desk reading of a tree that grows in 0.95 s on a quiet core. A 3 % verdict is read in user-space instructions (`count.py`, `perf_event_open`), which the load does not move, beside the seconds.

## The profile

`perf` is not installed and `samply` cannot profile on this desk (FRICTION.md, 13:21), so the profile is `sample.py`'s: the kernel's samples of the instruction pointer on the cycles counter at 5 kHz, user space only, named by `llvm-symbolizer` from a build with line tables (`CARGO_PROFILE_CI_DEBUG=line-tables-only`, own target directory), inlined frames included. It has no call chains (the binary has no frame pointers). The run sampled is the 13:12 one; its 8,206 samples include the example's digest of the tree (formatting every phytomer), which is no part of growth and is left out by name, leaving 4,896 samples of growth and settle (0.98 s). Raw samples: `raw/speed2/r0-base/samples.tsv`; read back with `sample_report.py`.

By the function sampled (what a symbol-level profile shows):

| share | function | what it is |
|---|---|---|
| 7.4 % | `grow::draw::draw` | the rough layout's walk, a step a phytomer grown (`Layer::step` inlined, 4.1 %) |
| 5.8 % | `geometry::lay::lay` | the settle's two lays, a step a kept phytomer each (`Layer::step` 5.5 %, of it the sag's turn 3.5 %) |
| 4.9 % | `girth::lay`'s closure | the year's wood: every open phytomer, every year (`leaf` on its inline chain: 4.8 %) |
| 4.9 % | `memcpy` (libc) | moves: growing `Vec`s, lines merged, pencils, the settled copy, compaction |
| 4.4 % | `Shoot::grow_unit` | a growth unit's nodes and buds |

Grouped, over every frame of the inline chain (the groups overlap where one is inlined in another):

| share | group |
|---|---|
| 16.9 % | libc in all: `malloc`, `free` and `realloc` 11.7 %, `memcpy` 5.2 % |
| 9.6 % | `Layer::step`, the rough layout's and the settle's |
| 8 % | the light's deposit: `quantised_with` and `corners` 4.5 %, gathering the leaves (`Grower::deposit`) 2.0 %, `round` 1.0 %, its share of `floorf` |
| 6.2 % | the pinned `libm` crate: `sincosf` 2.1 %, `atanf` 1.9 %, `exp` 0.8 % |
| 5.3 % | the year's wood (`girth::lay`), of it `Tree::new`, `upward` and `picked` 0.8 % |
| 4.3 % | the contract's `log2` (2.6 %) and `exp2` (1.7 %) |
| 3.3 % | libm calls for rounding: `floorf` and its fallback 2.4 %, `round` 1.0 % (x86-64 without SSE4.1 calls out for them) |
| 3.3 % | the working set's passes (`open`, `extend`, `split`, `totals`, `order`, `sizes`) |
| 3.2 % | the retirement's evaluation (`evaluate`, `copied`, `fades`, `assign_living`), its allocations not counted |
| 2.4 % | `node_wood` with `Windows::sleeping` |
| 2.3 % | `Shoot::advance`; 2.1 % `presence::assign_one`; 1.5 % `Growing::settled`; 1.1 % compaction |

What the profile says about the dispatch's suspects:

- **Allocation churn is the largest single cost**, a sixth of the run with its moves. Per axis the grower holds five lists that grow in step: its phytomers (72 bytes each), their node draws (`Draws::nodes`), their paths (`Pencil::path`), and per unit its draws and its shade. An axis ends with 6.7 phytomers on 2.3 units on average, so each list is made and grown once or twice: about eight allocations an axis over 136,804 axes, freed again at retirement, compaction or the settle. The retirement's evaluation adds about a dozen short-lived lists an evaluation. Without call chains the split between the two is an estimate.
- **Large records walked for one field.** The year's wood reads each open phytomer's cycle, scale and size for a leaf that is 0.0 for all but the year's own (the oak keeps its leaf a year), and its section and sapwood; all of it sits in the 72-byte `Growing`, one cache line a phytomer, on lists scattered an axis apiece. The first touch of each is where its samples fall (`leaf`'s `saturating_sub`, 1.9 %).
- **Passes that visit dead wood** are small: the wood's tree is rebuilt over every held axis each year (1,312,651 axis-years, a third or so of them open), but `Tree::new`, `upward` and `picked` together are 0.8 %.
- **Work recomputed that has not changed**: `node_wood`, a pure function of the PA, the zone and the cycle, is taken once a growth unit (314,077 units for about ten distinct values a cycle), 2.4 %. fn-368's READING.md named it (#2) and did not build it.
- **Inherent to a rule**: the walk's arithmetic. `Layer::step` runs once a phytomer grown (912,157) and twice a phytomer kept (326,166, once unbent for the levers, once under them): trigonometry from the pinned `libm`, the contract's `log2` and `exp2`, keyed draws. With the sag's turn that is 9.6 % + 6.2 % + 4.3 %, overlapping. It gets cheaper only by a walk that reads fewer transcendentals, which is a change to the rule's arithmetic and the tree.
- **The light's lattice is rebuilt every year because every leaf is new** (the oak's leaf lasts a year): the deposit is 8 % of the run and runs on one core, eight corners a leaf for 912,157 leaves. Its sweep is small (0.6 %): the lattice is at most 7,038 nodes.

## Candidates, ranked on paper before any was built

Share from the profile, times the fraction a change can remove; one thread unless said. The order is the order to build in.

| # | candidate | share | removable | paper gain | status |
|---|---|---|---|---|---|
| 1 | The deposit in parts on every core: the leaves cut into parts, each onto its own lattice of quanta, the lattices added (`r3-deposit-parts`) | 5.5 % of one thread, about a tenth of the clock on four | two thirds, on four cores | 0 % on one thread, 5 to 8 % on four | written, not built |
| 2 | The deposit's count and corners without libm or a test a corner (`r3r4-deposit`, on #1) | 5.5 % | a third | 1.5 to 2 % | written, not built |
| 3 | `node_wood` taken once a PA, zone and cycle a share (`r5-node-wood`) | 2.4 % | nine tenths | 2 % | written, not built |
| 4 | `floor` without libm in the lattice's `node`, the angles' `wrap` and `exp2` (`r6-floor`) | 2.4 % | three quarters | 1.5 to 2 % | written, not built |
| 5 | The year's wood asks only the phytomers in leaf for their leaf (`r2-wood-leaf`) | 4.8 % | the arithmetic, not the cache line: a third | 1 to 2 % | written, not built |
| 6 | The year's wood carried up the tree of the open axes alone (`r1-wood-tree`) | 0.8 % | two thirds | 0.5 % | written, not built |
| 7 | The retirement's evaluations of a year taken as one forest through the settle's own `assign_living` and `fades`, not a copy and a dozen lists a subtree | 3.2 % and its allocations, perhaps 6 % | half | 2 to 3 % | not built: needs `fades` to report several roots; the largest change here, and no build to prove it in |
| 8 | A phytomer's wood (section and sapwood, 8 bytes) held apart from its 72-byte record, so the year's pass reads a ninth of the memory | 5.3 % | half, if the pass is bound by memory | 2 to 3 % | not built: it moves two fields out of `Growing`, which the settle, the retirement's copy, the digest and `an_axis_and_its_draws_stay_compact` all read |
| 9 | The per-axis lists held in shared stores (one arena a column, an axis a range of it), so an axis's growth allocates nothing | 11.7 % and part of `memcpy`'s 5.2 % | half | 6 to 9 % | not built: an axis grows a unit a year, so its range must move or be chained; a design the host owns |

No candidate that could be built in the round reaches 3 % on one thread on paper, as none reached a tenth in fn-368: the profile is flat. #1 is the only one past the bar, and only on four threads. #1 to #6 together are 7 to 9 % on one thread on paper. Candidate #6 was built first, before the profile could be taken (the queue held the profile back; the ranking then was from fn-368's per-pass split, which put the wood at 8 to 9 %); the profile then showed its target at 0.8 %.

## The candidates as built

Each is a patch against the base in `speed2-candidates/`, formatted, with its proof and, where it adds arithmetic, its own test. `speed2-build.sh` builds each on its own, keeps its `early`, and reads it against its base (`speed2-ab.sh`: the two binaries in turn, five pairs, one thread and four, each under `count.py`); `speed2-tests.sh` runs the narrow proofs.

| patch | functions touched | why the tree is the same to the bit |
|---|---|---|
| `r3-deposit-parts` | `light::field::Field::quantised_with`; new `Grid::quanta`, `Grid::quanta_in_turn`, `overfull`; test `the_deposit_in_parts_is_the_deposit_in_turn` | A quantum is a whole number of at least none (an area of at least none, asserted, times a trilinear weight of at least none, rounded), held in a u32. Whole numbers add the same in any order and any grouping, so the parts' lattices added are the lattice deposited in turn. In turn the deposit is refused where one quantum is past a u32 or a node's running sum is; every term being at least none, a running sum passes a u32 in some order exactly where the whole sum does, so the parts refuse (in a part, or at the merge) exactly the deposits the turn refuses, with the same error. One thread, or fewer than two parts' worth of leaves (a part holds at least 4,096 and at least as many as the box has nodes), takes the old path. |
| `r3r4-deposit` | the above, and new `light::field::rounded`, `Grid::inside`; `Grid::corners` made visible to the tests; test `the_deposits_count_and_corners_are_the_ones_it_took` | `rounded(x)` is `x.round()` cast as it was: `round(x) <= u32::MAX` exactly where `x < u32::MAX + 0.5` (NaN refused by both); below that `x as u32` is the whole part, exact, `x` less it is exact (a multiple of `x`'s last place below one), and half away from none rounds up exactly where that is a half or more; an `x` under none counts none in both. `inside` gives `corners`' indices and weights: a corner's index is the lowest corner's plus a step an axis, inside the box wherever the lowest and highest corners are, which is where every corner is (the same panic otherwise); a weight is `((1 × a) × b) × c` in `corners` and `(a × b) × c` here, and `1 × a` is `a` to the bit. |
| `r5-node-wood` | `grow::unit::Shoot::grow_unit`; new `Shoot::node_wood`, `shoot::NodeWood`, the field `Shoot::woods` | `unit::node_wood` reads the rules' windows and laterals and the species' state of a PA, a zone and a cycle; a share holds them by shared reference and no growth unit writes them, so a second call with the same three returns the same wood and the same sleeping buds' woods, which the share now keeps. No draw is made in it. |
| `r6-floor` | new `contract::floor`; `contract::exp2`, `geometry::q::wrap`, `light::field::node`; test `the_contracts_floor_is_floor_to_the_bit` | An f32 of magnitude 2^23 or more is whole, and is returned as it is, as an infinity and a NaN are; below that the cast to `i32` is its whole part toward none, exact, and `floor` is that, or one less where it lies above the value; a zero result keeps the value's sign, as `floor`'s does. The test holds it to `f32::floor` at 23 edges and at every 37th of the 2^32 f32s. A signalling NaN, which `f32::floor` quiets and no arithmetic makes, is the one input apart. |
| `r2-wood-leaf` | `girth::lay` | A phytomer whose age is at or past its PA's `leaf_years` has `leaf = area × clamp(leaf_years − age, 0, 1) = area × 0.0`, which is 0.0 for a finite area (−0.0 for an area under none), and `0.0 + x` is `x` to the bit for the `x` here, a sum that began at 0.0 and so is never −0.0. `bearing` finds the first phytomer in leaf by the same comparison, on phytomers laid in the order of their cycles (the premise `Grower::deposit` already rests on). An infinite or NaN leaf area, which no validated species gives, is the one input apart. |
| `r1-wood-tree` | `girth::lay` | A closed axis returned 0.0 without reading its children; its bearer added that 0.0 to a per-node sum or its tip's sum, each begun at 0.0, which leaves the sum as it was to the bit. Left out of the tree, it adds nothing. The axes left keep their index order, so each bearer still takes its children the last first. An open axis under a closed one, should one arise, lays its own wood and hands it to no one in both. |

## Measurements

None of the six was built or read: see "What the round reached". The one line that runs them all, from the worktree's root, when the queue has room:

```
TELPERION_HEAVY_RESERVE=8 telperion-heavy 4 --threads 4 -- \
  .flow/evidence/fn-351-020-the-release-candidate/speed2-build.sh \
  "r3-deposit-parts:r3-deposit-parts:r0-base r4-deposit-count:r3r4-deposit:r3-deposit-parts r5-node-wood:r5-node-wood:r0-base r6-floor:r6-floor:r0-base r2-wood-leaf:r2-wood-leaf:r0-base r1-wood-tree:r1-wood-tree:r0-base"
```

It needs the base's binary at `raw/speed2/r0-base/early` (built from `7685ee85b`; on this desk it is there). Each row prints its medians and gains by clock, CPU seconds, instructions, cycles and cache misses, one thread and four, and the digests of both sides. A candidate that passes is then applied, proved (`speed2-tests.sh <label> all`) and committed alone.

## The spruce and Telperion

Not timed: no change is kept to time them before and after.

## What remains, and its share

The oak at 40 on one thread, from the profile (shares overlap where a function is inlined in another):

- a sixth is the allocator and moves (candidate 9, and 7 for the evaluation's part);
- a fifth is the walk's arithmetic, in the rough layout and the settle's two lays (`Layer::step`, the sag's turn, `libm`, the contract's logs), inherent to the rule;
- 8 % is the light's deposit on one core (candidates 1, 2 and 4 take the cores, the rounding and the tests; what is left is eight corners a leaf);
- 5 % is the year's wood (candidates 5, 6 and 8);
- 7 % is the growth unit and its apex's decisions less `node_wood` (keyed draws, the contract's presences), inherent;
- 3 % each: the working set's passes, and the retirement's evaluation before its allocations.

## Unknowns

- Whether any candidate compiles, and what each gains: none has been through a compiler. Each was read against the code it touches and formatted with `rustfmt`, which parses it; that is all.
- `malloc`'s callers: the sampler has no call chains. The split of the allocator's 11.7 % between the per-axis lists and the retirement's evaluation is an estimate from counts (about 1.1 M list growths, and a dozen lists an evaluation).
- The conversion in `crates/telperion-core/src/pipeline/space.rs` was read and not measured: `early` stops at the engine's tree, so its seconds are not in any number here. It reserves its stores once (fn-369) and walks each kept phytomer once; nothing in it was changed.
- The profile is one run of 4,896 samples: a 1 % row is about 49 samples, so rows under 1 % are orders of magnitude, not measurements.
- The census `early` takes is in every number here (`Census::cycle`, 0.9 %); `grow` without it was not timed.
