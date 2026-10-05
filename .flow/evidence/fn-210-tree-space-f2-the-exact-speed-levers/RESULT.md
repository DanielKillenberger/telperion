# fn-210: the exact speed levers (worker, 2026-10-05)

A dispatched worker wrote this. Every lever below leaves every tree the same to the bit. The oak's shed subtrees under light (lever 1's second half) are not built: section 3 is the design question for the host.

## 1. What is built

Each item is its own commit on `fn-210-tree-space-f2-the-exact-speed-levers`, from `b5161f48` (the fn-206 port).

| Commit | Lever | What |
|---|---|---|
| `b23792de` | 1 and 3 | **Shedding decided before growth** where nothing reads shed wood while it lives. **Light's wasted work:** the rough layout only where a bud reads light; the last cycle's layout, full lay and sweep skipped. The measures example gains the palm and a light selector. |
| `0258c9a0` | (found) | **The closed form reads each sleeping bud's year once.** Not in fn-198's list: it was measured before fn-206, whose sleeping-bud law made `Windows::new` the oak's largest cost. |
| `ecca0a30`, `f579008d`, `f0793ed8` | 2, (e2) | **Growth on every core**, then **the lay on every core**, both through one spread of work over the process's spare cores (`cores.rs`). |

### Lever 1: what "certain" is (`shed.rs::lasting`, `grow/shoot.rs::shed_certain`)

- **The bound.** `lasting(p)` = ceil(lifespan(p)) + the most of `lasting` over p's continuation and every lateral PA it can bear. A lateral born in cycle c has made its last growth by c + lasting(p).
  - It is none (no bound) where p bears its own PA, where a zone of p holds sleeping buds that wake, where an age that ends relays (`relay_ended` > 0: each such relay grows a whole unit past the lifespan and may relay again), and where a failed unit relays (`relay_failed` > 0 with viability < 1).
  - Viability, abortion, shade and the balance only shorten life. An abortion's relay carries the age's time on, so it adds none.
- **Certain.** The fade of a lateral is d + 1 - (age - until), where `until` is its subtree's presence-weighted last living time (`shed::fades`), never later than its last growth. The skip asks that (d + 1) <= age - (c + lasting), the fade's own sum with `until` at its bound, for the lateral's own delay d **and for the delay of every lateral or relay it stands on** (`horizon`).
  - Then its fade is exactly 0, and a kept ancestor whose `until` it sets has a fade at or below 0 with it or without it, so the ancestor's fade is its living apex's either way.
  - Without the ancestors' delays the rule is wrong: a beech whose twigs' bearers keep 6 years changes (test below, red).
- **Where it applies.** Only where nothing reads shed wood while it lives: no rough layout (no bud reads light) and no PA with `retained` or `leaf_girth`. That is the beech, spruce and palm, under any light. The oak is excluded by its retained pipe even in neutral light.
- **The phytomer budget** now counts the wood grown, so a tree that was refused for growing what it sheds may now grow.

### Lever 2: growth and lay on every core

- **Growth.** A growth unit touches only its own axis and the axes it makes. A cycle's living apexes are cut, in order, into shares of 1,024; each share grows its apexes' axes where they stand (each apex is its own axis, so the borrows are apart, `cores::picked`) and numbers the axes it makes from the tree's count as if it came first. The shares merge in order, each shifted past the axes the shares before it made: parents, successors, the next live list and the sleeping buds' bearers. Phytomers are counted per share against the budget, and once more after the merge.
- **The lay.** An axis is laid from its parent's frame alone, so each depth's axes are laid at once. Where axes cannot be laid, the error is the first in index order, as laying in turn gives.
- **Threads.** `std::thread::scope`; the workspace has no rayon, so no dependency was added. Helpers are drawn from what the process has spare, so trees grown at once (the walk suites, a forest) do not each take every core. On this desk: 32 cores.

### What was not built from fn-198's lever (e)

- **(e1) A dormant cull.** Not free once the cull's overflow refusal is kept: at shell depth 1 the cull already stops at each leaf's first vertex, and that vertex's transform is the overflow check. It is also the pipeline's cull, outside the engine.
- **(e3) A compact phytomer, exactly.** The exact part is moving `size` and `light` (16 of 128 bytes) to per-unit arrays: about 80 MB of the spruce's 2.26 GB peak (4%), across six readers. Not built. The bytes fn-198 counted are in f32 positions and derived frames, which change the hash: a host decision.
- **(e4) One record a leaf-only shoot.** One design with (d3); a design question, not an exact lever.
- **(e5)** No gain (fn-198).

## 2. R1: the hash proof

`measures hash 80 <seed> <species> both`, seeds 1 and 7, neutral and lit (extinction 0.5, sky 0.5), base `b5161f48` against each commit. Identical at every commit:

| Tree | Neutral | Lit |
|---|---|---|
| palm 1 / 7 | `d2cd1637…` / `c3dee328…` | same |
| beech 1 / 7 | `f1eede48…` / `9eb6e2a8…` | same |
| spruce 1 / 7 | `109d37db…` / `0ffab055…` | same |
| oak 1 / 7 | `905a271a…` / `573bed1e…` | `ebad1976…` / `0ec24cfd…` |

Tests (`src/grow/tests.rs`):
- `a_lateral_certain_to_be_shed_is_shed`: every lateral the rule marks is gone from the tree (beech, spruce, palm, and a beech with long-kept bearers). Red with the skip disabled (none marked).
- `a_tree_grows_the_same_without_what_it_is_certain_to_shed`: the tree with the skip equals the tree without, and grows under two thirds of the phytomers. Red with the skip disabled (74,841 of 74,841 grown), and red with the ancestors' delays left out (the long-kept-bearer beech changes).
- `light_no_bud_reads_is_not_worked`: the beech, spruce and oak under light, with and without the unread light work.
- `many_cores_grow_the_tree_one_does`: one core against eight in shares of seven, for the beech and oak in light, the spruce, the palm, and a spruce whose lifespans end mid-cycle (its sleeping buds sleep on axes a share made). Red with the sleepers' bearers left unshifted.
- `nothing_is_skipped_where_shed_wood_is_read`: the oak marks nothing.

## 3. The design question: the oak's shed subtrees under light

Not built. The brief says to report before building if the closed-form stand-in is more than a small change. It is:

- A certain-shed subtree of the oak is read, while it lives, by four couplings, each needing a form F1's R0 has not yet given:
  1. its leaves shade the lattice at their places (`grow/relay.rs`, `sketch.rs`): a stand-in needs a place for its expected leaf area each year;
  2. it takes a share of its bearer's vigour under apical control (`allocation.rs`);
  3. it enters its bearer's remembered carbon balance, light against upkeep (`grow/balance.rs`);
  4. at retained 0.5 its pipe stays in its bearer's girth (`girth::attachments`).
- **What is at stake** (probe, not committed: the rule forced on for the oak in light, nothing standing in):

  | Oak, 80 years, lit | Grown phytomers | Grown with the skip | Laterals certain to be shed | Kept phytomers | Height |
  |---|--:|--:|--:|--:|--:|
  | seed 1 | 5.53M | 2.11M | 451k | 1.10M to 1.22M | 21.87 to 20.33 m |
  | seed 7 | 11.33M | 4.98M | 916k | 2.59M to 2.90M | 25.33 to 24.28 m |

  So 56 to 62% of the oak's growth is certain to be shed; skipping it with no stand-in changes the tree by 11% in kept wood and 1 to 1.5 m in height.
- **Questions for the host:** the stand-in's form for each of the four couplings (where its leaves stand in the lattice; how its vigour share, balance and pipe are taken from the closed form), or whether this waits for F1's R0.

## 4. R2: per-stage cost at 80 years

Release build, one process per tree, base and after run back to back, 1-minute load 2 to 5, 32 threads (`raw/` not kept; the log is in the session scratchpad). Seconds; peak resident memory of the process.

| Tree | Growth | Rough layout | Full re-lays | Light | Settle | Final lay | **Total** | Peak memory |
|---|--:|--:|--:|--:|--:|--:|--:|--:|
| palm 1, 7 | 0.000 | – | – | – | 0.000 | 0.000 | **0.002 / 0.002** (wall, s) | <0.01 GB |
| beech 1 | 2.35 / 0.44 | – | – | – | 0.75 / 0.25 | 0.26 / 0.10 | **3.56 / 0.85** | 1.95 / 0.56 GB |
| beech 7 | 3.74 / 0.59 | – | – | – | 1.25 / 0.42 | 0.45 / 0.21 | **5.82 / 1.36** | 3.17 / 0.94 GB |
| beech 1, lit | 2.25 / 0.44 | 1.34 / 0 | 1.93 / 0 | 0.36 / 0 | 0.71 / 0.24 | 0.26 / 0.10 | **7.10 / 0.84** | 2.49 / 0.56 GB |
| beech 7, lit | 3.75 / 0.61 | 2.31 / 0 | 3.38 / 0 | 0.58 / 0 | 1.19 / 0.43 | 0.45 / 0.18 | **12.12 / 1.33** | 4.10 / 0.94 GB |
| spruce 1 | 10.10 / 0.97 | – | – | – | 2.75 / 0.88 | 3.16 / 1.26 | **16.95 / 3.40** | 6.45 / 2.26 GB |
| spruce 7 | 9.97 / 0.95 | – | – | – | 2.68 / 0.83 | 3.13 / 1.24 | **16.70 / 3.32** | 6.34 / 2.24 GB |
| oak 1 | 22.88 / 1.32 | – | – | – | 1.33 / 1.31 | 0.72 / 0.34 | **25.13 / 3.20** | 2.10 / 2.12 GB |
| oak 7 | 25.78 / 2.35 | – | – | – | 3.28 / 3.10 | 1.82 / 0.92 | **31.45 / 7.17** | 4.70 / 4.77 GB |
| **oak 1, lit (production)** | 22.86 / 1.92 | 1.05 / 1.05 | 2.58 / 1.15 | 0.34 / 0.33 | 1.00 / 0.96 | 0.48 / 0.22 | **28.50 / 5.83** | 1.97 / 1.88 GB |
| **oak 7, lit (production)** | 25.94 / 3.64 | 2.35 / 2.31 | 5.80 / 2.48 | 0.75 / 0.69 | 2.30 / 2.21 | 1.19 / 0.56 | **38.85 / 12.55** | 4.23 / 3.85 GB |

Before / after in each cell. Growth includes the closed form's windows (`Windows::new`): 20.5 s of the oak's 22.9 s before, 0.5 s after; 3.6 s of the spruce's before, 0.07 s after.

**What is left, largest first:** the oak's rough layout, re-lays and settle under light (sequential; the re-lays' lay is now parallel), the spruce's and oak's settle (`assign`, `shed`, `scale`, `thicken`, sequential), the sag levers (0.38 s spruce), and the oak's grown wood that lever 1 cannot skip until section 3 is decided.

## 5. R3

- Workspace gate (`cargo test --profile ci --workspace --no-fail-fast`, once, at `f0793ed8`): 1,100 passed, 0 failed, 22 ignored, 12 min 10 s.
- Codex round 1 (`codex impl-review`, base `b5161f48`): **SHIP**, one P2: the process-wide thread budget counted helpers but not the callers, so 32 trees grown at once could run 64 workers. Fixed in the next commit: each caller counts in the budget.
