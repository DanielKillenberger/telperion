# fn-351 speed round 2: the grower's structure, the oak at 40

Branch `fn-351-speed-round-2`, from `fn-351-candidate` at `7685ee85b`. Tree: Oregon white oak, age 40, seed 1001, grown through `grow_census` by the `early` example (`three oregon-white-oak 40 1001 cycle`); digest of the grown tree on the base `15453902402947031023` (912,157 phytomers grown, 326,166 kept on 51,271 axes). 14 Nov 2026, Zurich time. The asked aim was a faster growth with the tree the same to the bit, by structural changes fn-368 did not make.

## What the round reached

Four changes are kept, each its own commit, each proved to leave every pinned tree as it was, and the tip grows the oak at 40 in 4.3 % fewer instructions; in seconds the gain is about 3 % on one thread and is not resolved on four (see "Measurements"). The round came in two parts. Its first two hours (12:22 to 13:45) bought the profile and six candidates written as patches, none compiled: the queue was frozen or full (FRICTION.md, 14 Nov 12:28, 12:39, 13:21 and 13:44), and the round returned early. The host reopened it at 14:52 with the desk freer and a ruling on the keep rule: the one-thread candidates may be kept as a bundle that gains at least 3 % in instructions and in median seconds over five runs, each member proved alone and shown not to lose alone; `r3` judged on four threads; `r1` dropped under 0.5 %. All six compiled as written. The queue froze twice more in the second part (14:55 to 14:57, 14:58 to 15:24), and the desk never fell below a load of 4.

## The base, and why seconds cannot carry a verdict today

One thread, the tree's own clock (`cycle: … s`, growth and settle):

| when | how | clock | CPU to the grown tree | user instructions | cycles | cache misses |
|---|---|---|---|---|---|---|
| 12:48 | in the queue, two minutes after a thaw, beside every other thawed job | 1.48 s | 1.47 s | 15.81 G | 8.97 G | 63.9 M |
| 13:11 | in the queue, pinned to one core, load 9 | 0.95 s | 0.94 s | | | |
| 13:12 | in the queue under the sampler, load 9 | 0.98 s | 0.96 s | | | |

The same binary on the same tree spreads by half with what runs beside it, so fn-368's 1.44 s is a loaded-desk reading of a tree that grows in 0.95 s on a quieter core. A 3 % verdict is therefore read in user-space instructions (`count.py`, `perf_event_open`), which the load does not move, beside the seconds.

The counters are the whole process's, and `early` digests the tree after growing it: by the sampled run the digest is 40 % of the process's cycles (3,310 of 8,206 samples). Both sides of a pair digest the same tree, so the difference between them is the change's own; its share of growth is that difference over 0.60 of the base's cycles. The instructions' split between growth and digest is not measured. The cache misses are the kernel's generic event (on this Ryzen 9 5950X, by its usual mapping, misses of the 512 kB L2), 64 M of them in the process.

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
| 1 | The deposit in parts on every core: the leaves cut into parts, each onto its own lattice of quanta, the lattices added (`r3-deposit-parts`) | 5.5 % of one thread, about a tenth of the clock on four | two thirds, on four cores | 0 % on one thread, 5 to 8 % on four | kept, `41fcab07d` |
| 2 | The deposit's count and corners without libm or a test a corner (`r3r4-deposit`, on #1) | 5.5 % | a third | 1.5 to 2 % | kept, `b1add0a8a` |
| 3 | `node_wood` taken once a PA, zone and cycle a share (`r5-node-wood`) | 2.4 % | nine tenths | 2 % | kept, `03e557028` |
| 4 | `floor` without libm in the lattice's `node`, the angles' `wrap` and `exp2` (`r6-floor`) | 2.4 % | three quarters | 1.5 to 2 % | kept, `fb85fa3bf` |
| 5 | The year's wood asks only the phytomers in leaf for their leaf (`r2-wood-leaf`) | 4.8 % | the arithmetic, not the cache line: a third | 1 to 2 % | tried, not kept: 0.09 % |
| 6 | The year's wood carried up the tree of the open axes alone (`r1-wood-tree`) | 0.8 % | two thirds | 0.5 % | tried, dropped: 0.31 % |
| 7 | The retirement's evaluations of a year taken as one forest through the settle's own `assign_living` and `fades`, not a copy and a dozen lists a subtree | 3.2 % and its allocations, perhaps 6 % | half | 2 to 3 % | not built: needs `fades` to report several roots; the largest change here, and no build to prove it in |
| 8 | A phytomer's wood (section and sapwood, 8 bytes) held apart from its 72-byte record, so the year's pass reads a ninth of the memory | 5.3 % | half, if the pass is bound by memory | 2 to 3 % | not built: it moves two fields out of `Growing`, which the settle, the retirement's copy, the digest and `an_axis_and_its_draws_stay_compact` all read |
| 9 | The per-axis lists held in shared stores (one arena a column, an axis a range of it), so an axis's growth allocates nothing | 11.7 % and part of `memcpy`'s 5.2 % | half | 6 to 9 % | not built: an axis grows a unit a year, so its range must move or be chained; a design the host owns |

No candidate that could be built in the round reaches 3 % on one thread on paper, as none reached a tenth in fn-368: the profile is flat. #1 is the only one past the bar, and only on four threads. #1 to #6 together are 7 to 9 % on one thread on paper. Measured, the four kept are 4.3 % of the process's instructions, about 7 % of growth's. Candidate #6 was built first, before the profile could be taken (the queue held the profile back; the ranking then was from fn-368's per-pass split, which put the wood at 8 to 9 %); the profile then showed its target at 0.8 %.

## The candidates as built

Each is a patch against the base in `speed2-candidates/`, formatted, with its proof and, where it adds arithmetic, its own test. All six compiled as first written. `speed2-build.sh` builds each on its own, keeps its `early`, and reads it against its base (`speed2-ab.sh`: the two binaries in turn, five pairs, one thread and four, each under `count.py`); `speed2-tests.sh` runs the narrow proofs.

| patch | functions touched | why the tree is the same to the bit |
|---|---|---|
| `r3-deposit-parts` | `light::field::Field::quantised_with`; new `Grid::quanta`, `Grid::quanta_in_turn`, `overfull`; test `the_deposit_in_parts_is_the_deposit_in_turn` | A quantum is a whole number of at least none (an area of at least none, asserted, times a trilinear weight of at least none, rounded), held in a u32. Whole numbers add the same in any order and any grouping, so the parts' lattices added are the lattice deposited in turn. In turn the deposit is refused where one quantum is past a u32 or a node's running sum is; every term being at least none, a running sum passes a u32 in some order exactly where the whole sum does, so the parts refuse (in a part, or at the merge) exactly the deposits the turn refuses, with the same error. One thread, or fewer than two parts' worth of leaves (a part holds at least 4,096 and at least as many as the box has nodes), takes the old path. |
| `r3r4-deposit` | the above, and new `light::field::rounded`, `Grid::inside`; `Grid::corners` made visible to the tests; test `the_deposits_count_and_corners_are_the_ones_it_took` | `rounded(x)` is `x.round()` cast as it was: `round(x) <= u32::MAX` exactly where `x < u32::MAX + 0.5` (NaN refused by both); below that `x as u32` is the whole part, exact, `x` less it is exact (a multiple of `x`'s last place below one), and half away from none rounds up exactly where that is a half or more; an `x` under none counts none in both. `inside` gives `corners`' indices and weights: a corner's index is the lowest corner's plus a step an axis, inside the box wherever the lowest and highest corners are, which is where every corner is (the same panic otherwise); a weight is `((1 × a) × b) × c` in `corners` and `(a × b) × c` here, and `1 × a` is `a` to the bit. |
| `r5-node-wood` | `grow::unit::Shoot::grow_unit`; new `Shoot::node_wood`, `shoot::NodeWood`, the field `Shoot::woods` | `unit::node_wood` reads the rules' windows and laterals and the species' state of a PA, a zone and a cycle; a share holds them by shared reference and no growth unit writes them, so a second call with the same three returns the same wood and the same sleeping buds' woods, which the share now keeps. No draw is made in it. |
| `r6-floor` | new `contract::floor`; `contract::exp2`, `geometry::q::wrap`, `light::field::node`; test `the_contracts_floor_is_floor_to_the_bit` | An f32 of magnitude 2^23 or more is whole, and is returned as it is, as an infinity and a NaN are; below that the cast to `i32` is its whole part toward none, exact, and `floor` is that, or one less where it lies above the value; a zero result keeps the value's sign, as `floor`'s does. The test holds it to `f32::floor` at 23 edges and at every 37th of the 2^32 f32s. A signalling NaN, which `f32::floor` quiets and no arithmetic makes, is the one input apart. |
| `r2-wood-leaf` | `girth::lay` | A phytomer whose age is at or past its PA's `leaf_years` has `leaf = area × clamp(leaf_years − age, 0, 1) = area × 0.0`, which is 0.0 for a finite area (−0.0 for an area under none), and `0.0 + x` is `x` to the bit for the `x` here, a sum that began at 0.0 and so is never −0.0. `bearing` finds the first phytomer in leaf by the same comparison, on phytomers laid in the order of their cycles (the premise `Grower::deposit` already rests on). An infinite or NaN leaf area, which no validated species gives, is the one input apart. |
| `r1-wood-tree` | `girth::lay` | A closed axis returned 0.0 without reading its children; its bearer added that 0.0 to a per-node sum or its tip's sum, each begun at 0.0, which leaves the sum as it was to the bit. Left out of the tree, it adds nothing. The axes left keep their index order, so each bearer still takes its children the last first. An open axis under a closed one, should one arise, lays its own wood and hands it to no one in both. |

## Measurements

Every run is the `early` example under `count.py`, in the queue, pinned (`taskset`: core 4 for one thread, cores 4 to 7 for four), the sides in turn (`speed2-ab.sh`, `speed2-quiet.sh`). The desk's load was 4 to 9 throughout, so no core was quiet: one side's runs spread by a third (the base on one thread, 0.97 to 1.41 s over 20 runs). Instructions repeat to a part in ten thousand and carry the verdicts; seconds are given as medians with their batches. The counters include the example's digest of the tree, the same on both sides, so a saving's share of growth alone is about 1.7 times its share here. The oak's digest was `15453902402947031023` on every run of every binary.

Each candidate alone against the base (`7685ee85b`), the oak at 40. Instructions and cycles are medians of 15 pinned runs a side on one thread (5 for `r4`, 5 unpinned pairs for `r1`), of a base of 15.68 G instructions:

| candidate | instructions saved, one thread | cycles, one thread | clock, one thread (median; base 1.20 s) | four threads | verdict |
|---|---|---|---|---|---|
| `r3` deposit in parts | none (11 M more, 0.07 %) | not read alone | not read alone | 0.77 s to 0.72 s, +6.5 % (15 runs a side; by batch +1 %, +10 %, +6 %) | kept on four threads |
| `r4` count and corners, on `r3` | 464 M, 2.96 % | within the spread | 1.26 s against 1.32 s in its batch | | kept in the bundle |
| `r5` `node_wood` once | 193 M, 1.23 % | 0.3 % more | 1.22 s, within the spread | | kept in the bundle |
| `r6` `floor` without libm | none (17 M more, 0.11 %) | 2.3 % fewer | 1.21 s, within the spread | | kept in the bundle: it trades 17 M instructions for the calls' cycles |
| `r2` wood asks leaf of those in leaf | 14 M, 0.09 % | 0.8 % more | 1.22 s, within the spread | | not kept |
| `r1` wood's tree of the open axes | 49 M, 0.31 % | within the spread | within the spread | | dropped, under 0.5 % (host) |

`r2` does not lose and could have joined the bundle under the ruling; it is left out because 14 M instructions is nothing the oak can show, and it rewrites the loop of `girth::lay`, which `fn-351-oak-sapwood-by-size` changed on the candidate branch. Its patch stays in `speed2-candidates/`; the host may add it. No member's seconds can be told from the base's alone on this desk: "shown not to lose" rests on instructions for `r4` and `r5` and on cycles for `r6`.

The bundle, one thread, against the base in the same batches (five pinned runs a side a batch):

| binary | instructions | cycles | clock by batch, base then it |
|---|---|---|---|
| `r3`, `r4`, `r5`, `r2` | 4.33 % fewer | 5.1 % fewer | 1.32 and 1.20 s, 1.17 and 1.18 s, 1.15 and 1.03 s |
| the same with `r6` | 4.38 % fewer | 6.7 % fewer | 1.32 and 1.16 s, 1.17 and 1.10 s, 1.15 and 1.11 s: 12 %, 6 % and 3.5 % faster |
| the tip `fb85fa3bf` (`r3`, `r4`, `r5`, `r6`) | 4.28 % fewer | 4.1 % fewer | 1.09 and 1.06 s, 0.98 and 0.93 s, 1.22 and 1.16 s, 1.27 and 1.35 s: 2.8 %, 5.1 % and 4.9 % faster, 6.3 % slower |

**The oak at 40, before and after the kept set** (base `7685ee85b`, tip `fb85fa3bf`; 20 pinned runs a side in four batches):

| | base | tip | change |
|---|---|---|---|
| one thread, clock, median of 20 | 1.115 s | 1.080 s | 3.1 % faster |
| one thread, the quietest batch (load 4) | 0.98 s | 0.93 s | 5.1 % faster |
| one thread, instructions | 15.679 G | 15.007 G | 4.28 % fewer |
| one thread, cycles | 7.441 G | 7.136 G | 4.1 % fewer |
| four threads, clock, median of 20 | 0.770 s | 0.785 s | 1.9 % slower; by batch 5 % slower, then 3 %, 4 % and 6 % faster |
| four threads, the quietest batch | 0.65 s | 0.63 s | 3.1 % faster |
| four threads, instructions | 15.839 G | 15.177 G | 4.17 % fewer |
| four threads, cycles | 8.536 G | 8.456 G | 0.9 % fewer |

Read plainly: on one thread the tip is 4.3 % cheaper in instructions and 3 to 5 % faster on the clock where the desk allowed a reading. Against the ruling's bar, the five-member bundle (with `r2`) passed it in each of its three batches; the tip as committed passes it on the pooled median (3.1 %) and in two batches of four, misses it by 0.2 points in one and loses in the loaded one. On four threads the tip's seconds are not resolved: three batches of four gain 3 to 6 %, one loses 5 %, and the pooled median moves the wrong way inside a spread of 0.58 to 1.16 s. `r3`'s own four-thread gain rests on its 15-run read above; the tip's read neither confirms nor refutes it. If the host drops `r3`, `r4` must be ported, since it rewrites the function `r3` introduced (`Grid::quanta_in_turn`).

**Proofs** (`speed2-tests.sh`; logs in `raw/speed2/alone-*/tests/` and `raw/speed2/tip/tests/`). Each of `r3`, `r3` with `r4`, `r5`, `r2` and `r6` applied alone to the base: the paused grower's equalities, the wood's and the light's tests with the candidate's own, many cores and the retirement's three equalities against the reference (32 to 34 passed, 1 ignored); `every_preset_grows_the_tree_it_grew_at_sixteen` against its pins (passed); and `catalogue_identity` (5 passed). The tip: the same (35 passed), and `every_preset_grows_the_same_tree_again_and_on_any_cores` (passed, 260 s). `r1` was read (its digest unchanged on 10 runs) and not put through the tests, being dropped.

**Which patches touch `girth::lay`:** `r1-wood-tree` and `r2-wood-leaf`, neither kept. No kept commit touches `girth.rs`. The kept commits touch `light/field.rs` and `light/tests.rs` (`41fcab07d`, `b1add0a8a`, and `fb85fa3bf` in `node`), `grow/unit.rs` and `grow/shoot.rs` (`03e557028`), and `contract.rs`, `contract/tests.rs` and `geometry/q.rs` (`fb85fa3bf`).

## The spruce and Telperion

At their presets' ages, seed 1, base against tip, one pinned run a side (`raw/speed2/quiet-tip-spruce*`, `quiet-tip-telperion*`); each digest the same on both sides (spruce `3821892227947260491`, Telperion `2601228955259548084`).

| tree | threads | clock | CPU to the grown tree | instructions | cycles |
|---|---|---|---|---|---|
| Norway spruce | 1 | 5.51 s to 5.36 s, 2.7 % faster | 5.47 s to 5.33 s | 66.18 G to 62.23 G, 6.0 % fewer | 4.1 % fewer |
| Norway spruce | 4 | 3.65 s to 3.24 s, 11.2 % faster | 6.65 s to 6.19 s | 67.00 G to 63.11 G, 5.8 % fewer | 4.6 % fewer |
| Telperion | 1 | 23.51 s to 23.56 s, no gain | 23.36 s to 23.41 s | 243.09 G to 229.25 G, 5.7 % fewer | 1.5 % fewer |
| Telperion | 4 | 14.47 s to 11.48 s, 20.7 % faster | 31.19 s to 26.45 s | 245.02 G to 231.27 G, 5.6 % fewer | 9.2 % fewer |

One run a side on a loaded desk: the instructions are the reading (5.6 to 6.0 % fewer on both trees). The four-thread clocks point the way `r3` would, and are single runs; Telperion on one thread shows no gain in seconds at all.

## What remains, and its share

The oak at 40 on one thread, from the profile (shares overlap where a function is inlined in another):

- a sixth is the allocator and moves (candidate 9, and 7 for the evaluation's part);
- a fifth is the walk's arithmetic, in the rough layout and the settle's two lays (`Layer::step`, the sag's turn, `libm`, the contract's logs), inherent to the rule;
- 8 % is the light's deposit on one core (candidates 1, 2 and 4 take the cores, the rounding and the tests; what is left is eight corners a leaf);
- 5 % is the year's wood (candidates 5, 6 and 8);
- 7 % is the growth unit and its apex's decisions less `node_wood` (keyed draws, the contract's presences), inherent;
- 3 % each: the working set's passes, and the retirement's evaluation before its allocations.

## Unknowns

- The four-thread gain in seconds, of `r3` and of the tip: the batches disagree ("Measurements"). A desk at a load under 1 would settle it in one batch.
- Each bundle member's own seconds: within the spread on this desk; only instructions and cycles tell them from the base.
- Why Telperion's 5.7 % fewer instructions buy no second on one thread: not looked into.
- `malloc`'s callers: the sampler has no call chains. The split of the allocator's 11.7 % between the per-axis lists and the retirement's evaluation is an estimate from counts (about 1.1 M list growths, and a dozen lists an evaluation).
- The conversion in `crates/telperion-core/src/pipeline/space.rs` was read and not measured: `early` stops at the engine's tree, so its seconds are not in any number here. It reserves its stores once (fn-369) and walks each kept phytomer once; nothing in it was changed.
- The profile is one run of 4,896 samples: a 1 % row is about 49 samples, so rows under 1 % are orders of magnitude, not measurements.
- The census `early` takes is in every number here (`Census::cycle`, 0.9 %); `grow` without it was not timed.
