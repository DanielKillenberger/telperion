# fn-209 R0: trees resolved on demand, chain by chain (worker proposal, 2026-10-05)

A dispatched worker wrote this proposal for the host. Nothing here is built, and nothing here is decided.

Each claim carries a mark:
- **checked** means the code was read on this branch at `56261529` (master plus F2). Paths are relative to `crates/telperion-space/src/` unless they name another crate.
- **measured** means the claim was run on this desk: release build, one core under `taskset`, 1-minute load under 1.
- **estimate** means the claim is derived from measured numbers; the derivation is given.
- **unknown** means nobody has run it.

The measurements come from two evidence probes in this directory:
- `f1-subtree-probe.rs`, output in `probe-80.txt`. It covers chains and subtree sizes, the coupling tree, a fly-in, whole-tree views, junctions, and bud light with gathered leaves.
- `f1-sizes-probe.rs`. It gives struct sizes and the closed form's cost.

Per-stage times on one core are in `stages-1core-80.txt`. Every tree is 80 years old, at seeds 1 and 7. The oak is lit at extinction 0.5 and sky 0.5, which is production. The other three species are neutral.

## 0. The short of it

- **The unit of resolution is the chain.** A chain is a seed or lateral axis plus every continuation and relay that carries it on.
  - It is one run of the sweep (`telperion-render/examples/space/tree.rs:3-4`, checked).
  - Its sleeping buds wake only on itself (`dormant.rs:105-140`, checked).
  - Chains are small. Measured, the median chain holds 4 to 7 phytomers, p99 57 to 78, and the largest 404 to 624.
- **One rule makes streamed equal one-pass by construction.** A chain reads only these:
  - what its ancestors recorded;
  - the coarse fields;
  - closed-form stand-ins for what it bears.

  It never reads the realized state of a fine descendant. A test then compares chains by lineage, to the bit.
- **The host's aim 2 is refined: the coupling tree.** Chains whose expected subtree is larger than the light lattice's cell (0.5 m, `light.rs:296`, checked) are grown together in one per-cycle pass with today's exact feedback. Everything finer is a stand-in in that pass.
  - The coupling tree is 13 to 25 % of the kept phytomers (measured): oak 15 to 19 %, beech 25 %, spruce 13 %.
  - Gathering each finer subtree's leaves at its centroid moves bud light by 0.0005 to 0.0007 on a mean of 0.40 to 0.44 (measured, oak).
  - So the light grid does not see the cut: it is built from the coupling tree and stand-ins alone, and is identical whatever is resolved. That answers (d).
- **Girth and sag use one level of look-ahead:** a child's own chain exactly, plus the closed form for what the child bears.
  - Without it, a stand-in's expected girth at the oak's junctions is off by a factor of 0.1 to 2.0 in radius (p5 to p95, measured).
  - With it, a joint is continuous by construction, because parent and child sum the same thing.
- **The cost is not what the realtime goal assumes at whole-tree views.**
  - A conservative half-pixel rule (a stand-in's error is its whole box) resolves 71 to 98 % of a tree's phytomers at 40 m, and 37 to 53 % at 200 m (measured).
  - On-demand resolution therefore saves little at the first view of a whole tree. It saves the oak's shed waste and every light pass on fine wood, and it makes resolution parallel by chain.
  - It pays in close-ups and fly-ins: after the first frame, at most 10k to 44k phytomers resolve in any one frame (measured).
  - That is about 10 to 70 ms on one core, and 2 to 9 ms on 8 (estimate).
  - Two design questions follow (section 8): the stand-in's error metric, and the first-view bar.
- **F2's open oak question is answered by F1:** a shed subtree is never grown. Its stand-in carries all four couplings (section 6).

## 1. (a) The couplings today, and their coarse form

"Axis to axis" means one axis's growth, presence, light, allocation, sag, girth, shedding, balance or retained girth reads another axis.
- **Down** means it reads an ancestor. That stays exact: an ancestor is always resolved first.
- **Up** means it reads a descendant.
- **Wide** means it reads the whole tree.

Up and wide couplings need a coarse form.

| # | Coupling | Where (checked) | Reads | Coarse form | Lost against exact feedback |
|---|---|---|---|---|---|
| 1 | Growth draws | `grow/shoot.rs:1-6`; `grow/advance.rs:20`; `grow/unit.rs:336-358` | none (lineage keys) | none needed: already per axis (F2 grows apexes apart, `grow/parallel.rs:1-6`) | nothing |
| 2 | Presence windows | `presence.rs:31-40`, `grow/unit.rs:341-350` | the species' closed form at the tree's age | none needed: settings only, one table per species and age | nothing |
| 3 | Vigour of a continuation or relay | `presence.rs:212-216` | its parent's end presence | none needed: within the chain | nothing |
| 4 | Base scale and relay node | `geometry.rs:250-283` | parent's phytomer scale at the node; parent's last unit | none needed: down | nothing |
| 5 | Frame and lay | `geometry.rs:343-425`, `geometry/place.rs:1-5, 37-78` | parent's laid frame; the chain's own reach (`geometry.rs:306-319`, relays only) | none needed: down and in-chain | nothing |
| 6 | Light at a bud | `grow/sketch.rs:83-91`, `light.rs:445-452`; leaves of the cycle `grow/sketch.rs:265`, `grow/relay.rs:307-314` | every leaf grown last cycle, at the rough layout's places | **coupling-tree grid**: coupling chains deposit their own leaves; each fine root deposits its expected leaf area (closed-form count in the cycle × `leaf_area` × presence) at its expected centroid; a fine root's buds read **one** value at its centroid | measured, oak: gathering realized fine leaves at centroids moves bud light by mean 0.0005 to 0.0007 (p95 0.0015 to 0.0023). One read per fine root differs from each bud's own by median 0.003 to 0.005, p95 0.022 to 0.036, on a mean of 0.40 to 0.44. Expected against realized leaf area: **unknown** |
| 7 | Shade hazard | `grow/advance.rs:44-53` | 6 | 6 | as 6 |
| 8 | Vigour split (Borchert–Honda) | `grow/sketch.rs:96-142`, `allocation.rs:130-189` | every subtree's light, Q, per cycle (up) | within the coupling tree, exact. A fine root's Q is E[living apexes in the cycle] × presence × its one light read; its incoming vigour V(c) is recorded. Inside a fine subtree, a chain splits its V(c) between its own apex and its laterals' stand-in Q | the realized size of fine subtrees no longer feeds back into their share; the self-reinforcement of a big twig below 0.5 m is cut |
| 9 | Per-age normalisation of size (decision 23) | `allocation.rs:200-226`, group = PA, `grow/sketch.rs:137-138` | presence-weighted mean of (v/p)^ψ over every growing bud of the PA (wide) | **a coarse field per cycle and PA**, r̄(c, PA), taken in the coupling pass over coupling apexes (realized) and fine roots (each its expected growing buds per PA in the cycle, from the closed form's count table, times its (V/p)^ψ). Stored as 15 × age floats; every chain reads it | conservation: a PA's fine buds no longer average a size of exactly 1 (`allocation.rs:195-197`), so total growth drifts by the stand-ins' error (**unknown**; to measure in R1) |
| 10 | Carbon balance and its shedding | `grow/balance.rs:25-100` | the lateral subtree's light and upkeep × wood (up); `kept` passed down to every apex in it | the draw belongs to the lateral's own chain: Q and C are its own chain's realized values plus its children's stand-ins; `kept` is passed down as a recorded series K(c) | a lateral's starvation reads the expected grandchildren, not the realized ones |
| 11 | Full re-lay every 10 cycles | `grow/relay.rs:249-316`, `grow.rs:225-228` | the whole standing tree, sized, thickened and sagged (wide) | coupling pass only: the coupling chains, with fine roots as stand-in loads and sections (no look-ahead) | fine wood is not re-laid while it grows; it is laid once, at the end |
| 12 | Rough layout | `grow/sketch.rs:63-79, 151-276` | parent's walk | coupling pass only; fine chains have no rough layout (they read one light value per fine root) | as 6 |
| 13 | Girth (pipe model) | `girth.rs:66-126` | every section carried (up) | **one-level summary**: each child's own chain exactly (topology and scales; no geometry, `grow.rs:258`), plus the closed form's expected section for what the child bears | a grandchild's realized girth. Parent and child sum the same terms, so the joint is continuous |
| 14 | Leaf-light girth, χ | `girth.rs:40-54`, `girth/disused.rs:196-211` | tree-wide pipe-weighted mean of light^χ (wide), and each phytomer's own light | the mean is a coarse scalar from the coupling pass; a phytomer's light is its chain's read | the mean is taken over stand-ins |
| 15 | Retained girth (disused pipes) | `girth/disused.rs:192-290` | each shed branch's radius as grown, and its fade (up) | retained share × the stand-in's expected section at its last growth (closed form); a kept child's taken share from its one-level fade | a shed subtree's realized radius |
| 16 | Sag | `sag.rs:53-153`, `grow.rs:274-277` | every load beyond each phytomer: mass and its moment on the unbent tree (up) | one-level summary: each child's own chain, laid unbent from the parent's unbent frame, plus its children's expected mass at their expected centroid (needs a closed-form extent) | the realized moments of grandchildren |
| 17 | Shedding fade | `shed.rs:314-426` (the subtree's living time walked to the root, `:382-416`) | the subtree's presence-weighted last living time (up) | one-level: the chain's own living time plus each child's expected living time (needs a closed-form extinction curve); decided by the chain itself | whether a subtree whose own chain stopped long ago still holds a living twig is expected, not known |
| 18 | Woken buds' share | `grow.rs:238-241`, `dormant.rs:322-366` | the bearer chain's successors | none needed: within the chain | nothing |
| 19 | Budget | `grow/unit.rs:372-377`, `grow/parallel.rs:86-90` | the whole tree's grown count (wide) | refused up front by the closed form's expected count (`closed_form.rs:23-40`), with a memory guard per resolution | a tree is no longer refused for a realized excess |
| 20 | `Collapsed` and `BelowGround` | `grow.rs:254-256, 279-286`, `geometry/place.rs:72-91` | the whole tree; the first axis in index order | the trunk's chain decides `Collapsed`. Whether a lateral can raise `BelowGround` at all (its base sits on laid wood that rests on the ground, `geometry/lay.rs:288`) is **unknown**; R1 needs a test first | an error is not found late during a zoom |

**The per-age mean, as asked.** A streamed tree takes r̄(c, PA) from the coupling pass (row 9), which runs before any chain is resolved and is the same pass in both modes, so the mean is identical however much is resolved. The alternative, a mean over resolved buds, would change with the camera and break both exactness and the pass's meaning.

**Where light is neutral** (beech, spruce and palm in production), rows 6 to 12 and 14 are dormant. Today's code already skips them (`grow.rs:166-178`, `species.rs:387-394`, checked). Those three species keep only the upward geometry couplings, rows 13 and 15 to 17. Their topology (row 1) is independent per chain today.

## 2. (b) The stand-in per unresolved bud

**What the closed form gives today** (`closed_form.rs`, checked):
- expected phytomers by PA and cycle for a bud of PA k with m cycles left: `CountTable` per (k, m, carried) in the memo (`:305-341`);
- the log of expected metres of wood (`:347-418`).

So it already gives, per bud:
- **expected leaves in each cycle:** phytomers of the cycle times `leaf_area`, since a leaf lives one cycle (`grow/sketch.rs:265`);
- **expected living apexes in each cycle:** one unit per living apex;
- **expected wood.**

It is neutral-light: no shade hazard, no balance, no allocation. Shedding is outside the formula (`closed_form.rs:13`).

**What must be added:**

| Quantity | Needed by | Form |
|---|---|---|
| Expected section Σ(own pipe)^e of the kept subtree | girth (13), retained (15) | the same recursion over each phytomer's `pipe` × `ripe(age - cycle)`; scale² taken at presence 1 (**estimate**: unbiased where presences are whole) |
| Expected mass and the first moment of the mass about the bud | sag (16) | mass from the section and wood; the moment needs the envelope's centroid |
| **Envelope**: expected along-length, across-radius and centroid offset in the bud's frame, bent once towards the PA's elevation | drawing; light deposit (6); sag (16); coupling classification | a new recursion: own axis length (units × internode × share) ⊕ each lateral's envelope at its insertion angle along it |
| **Extinction curve**: P(subtree still has a living apex at t) | fade (17), and a stand-in's leaves fading out before it is shed | a branching-process recursion on probabilities, memoised on (k, m, carried) as the counts are |
| Light response of the counts | 6 and 8 for shaded stand-ins | **open (Q5):** the neutral form overstates a shaded fine subtree's growth |

Each new form needs an oracle check before anything is built on it (docs/tree-space.md): the mean of many keyed subtrees of one PA and age against the form.

**How a stand-in is drawn:**
- **Wood.** Under the resolution rule below, a stand-in's whole box is under half a pixel, so its wood is sub-pixel. It is drawn as nothing; F3's true-width ribbons serve resolved wood.
- **Foliage.** Expected leaves in the current cycle × P(alive), as keyed leaf instances through the existing leaf path (`select.rs`), placed by keyed draws within the envelope's density. Leaves stay leaves, with no alpha coverage. fn-208 deleted coverage ribbons because they read pale (RESULT.md, "Deleted").
- **The risk:** a cloud of stand-in leaves may read unlike resolved twigs at the same distance. It is judged on stills at R3.

**The error metric that decides resolution.** A stand-in is resolved when its envelope's projected box exceeds half a pixel. That is the conservative bound: the stand-in places nothing exactly, so its error is its extent.

A tighter metric needs an envelope that matches the realized hull. The probe takes a share of the box as the error:

| View | Share 1 | Share 0.5 | Share 0.25 |
|---|--:|--:|--:|
| Oak at 40 m, phytomers resolved | 71 to 80 % | 57 to 67 % | 42 to 49 % |
| Beech at 40 m | 86 % | 69 to 73 % | 45 % |
| Spruce at 40 m | 98 % | 89 to 90 % | 53 % |
| Oak at 200 m | 37 to 43 % | 25 to 28 % | 16 to 20 % |
| Beech at 200 m | 43 % | 38 % | 29 to 30 % |
| Spruce at 200 m | 53 % | 50 % | 25 % |

Measured: 960 × 720, 45° vertical field, the tree at mid-height in view.

## 3. (c) The order of resolution

**A chain grows alone.** Its growth reads:
- its key, PA and birth cycle (in its bud record, made by its parent);
- per cycle: its light L(c), its incoming vigour V(c), and its ancestors' survival of shedding K(c);
- r̄(c, PA), and Windows (section 1).

Its laterals and woken buds become **bud records** (key, PA, cycle, node, slot, presences made), not living apexes. Today's `Shoot` already grows one share's apexes and the axes they make (`grow/shoot.rs:72-98`, checked). A chain is a `Shoot` whose `next` keeps only the chain's own continuations and relays; its laterals stay records.

Its geometry is laid once, from its parent's final frame at its node (`geometry.rs:343-425`). Before that, its children's own chains are grown (topology only), so that girth, sag and fade can read their one-level summaries. A drawn chain therefore always has one level of grown topology beneath it, cached and reused when the child is resolved.

**What it needs from its ancestors:**

| State | From |
|---|---|
| Frame at its node (after sag) | the parent's final lay |
| Base scale and its node's scale | the parent's phytomers (`geometry.rs:254-266`) |
| V(c), K(c) | recorded by the parent's growth; or by the coupling pass for a fine root |
| L(c) | the coupling pass's grid: read at its tip (a coupling chain), or once at its fine root's centroid |
| Age, birth and sleep | its bud record (`dormant.rs:43-62` for woken buds) |

Presence and the sag pose come down this way; nothing comes up.

**Today's loop split:**

| Today (checked) | Under F1 |
|---|---|
| `step`: `balance`, `allot`, `grow_live`, `wake` (`grow.rs:366-372`) | **per chain**: grow, wake. `balance` is per chain (its laterals' draws are its own). `allot` is per chain inside fine subtrees, and coarse in the coupling pass |
| `grow_live` shares on every core (`grow/parallel.rs:70-92`) | per chain; chains of one depth in parallel, as `place` lays depths (`geometry/place.rs:37-92`) |
| `sketch` (rough layout), `shade` (grid sweep), `relay` (full re-lay every 10 cycles) (`grow.rs:222-231`) | **coarse pass only**, over coupling chains plus stand-ins |
| `assign`, `shed`, `scale`, `thicken` (`grow.rs:242-260`) | per chain, with children's one-level summaries |
| `attachments`/`disused`, leaf mean (`grow.rs:246-248`) | per chain from summaries; the leaf mean is coarse |
| `place`, `sag::levers`, `place` again (`grow.rs:271-277`) | per chain, top-down; levers from the chain itself and its children's one-level summaries |

**The coupling pass.** It is today's per-cycle loop over the coupling chains only. Every bud whose expected subtree extent at the tree's age exceeds the lattice's cell is grown in it; every other bud is a stand-in.
- It records:
  - per fine root: L(c), V(c), K(c);
  - per cycle: r̄(c, PA);
  - once: the leaf mean.
- Coupling chains are resolved chains like any other and are cached; they run through the same chain-growth function, called cycle by cycle.
- Where light is neutral, the pass has no per-cycle work, and coupling chains are grown alone like fine ones.

## 4. (d) Exactness

**The test (R1).** For every species at seeds 1 and 7:
- **One-pass:** run the coupling pass, then resolve every chain in index order.
- **Streamed:** run the coupling pass, then resolve along a scripted camera (the probe's fly-in), evicting under a memory cap and re-resolving, then resolve the rest.

The two are compared per chain by lineage, to the bit (every phytomer's tip, frame, radius and scale), and the coarse records are compared as well. A second test runs on 1 and on 8 cores, as F2's does.

**Why it holds, and the risks:**

| Risk | Status |
|---|---|
| **The light grid's dependence on what is resolved** | none: the grid is built in the coupling pass from coupling chains and stand-ins only. A fine chain never deposits, and no camera reaches the pass. **Yes:** built only from stand-ins and coarse axes at a fixed rule, it is identical whatever is resolved. The rule is expected extent over a cell, not a fixed depth |
| **Presence windows** | settings and age only (`presence.rs:31-40`); identical |
| **The re-lay schedule** (`RELAY_EVERY`, `grow/relay.rs:249`) | coupling pass only; fine chains never re-lay and read it only through L(c) |
| **Float summation order** | every wide sum happens in the coupling pass, in its own fixed order: the deposit at `light.rs:447-449`, the group sums at `allocation.rs:212-215`, the leaf mean at `girth.rs:45-52`. A chain's own sums run in its own index order, and a child's summary is a value, not a sum the parent shares. Indices differ between modes, so nothing may key by global index; records key by lineage |
| **Cores** | F2's guarantee (`cores.rs`) holds per chain |
| **Errors found late** | row 20: `BelowGround` from a lateral is **unknown** |
| **Classification continuity** | a bud whose expected extent crosses the cell flips from stand-in to coupling chain: a jump in a walk (phase B). **Open (Q3)** |

## 5. (e) The cost

**One chain's resolution.** The single-core per-stage times at 80 years (measured) per kept phytomer are:

| Tree | Growth | Settle | Final lay (two with sag) | All |
|---|--:|--:|--:|--:|
| Beech | 0.47 | 0.17 | 0.18 | **0.81 µs** |
| Spruce | 0.57 | 0.19 | 0.75 | **1.51 µs** |
| Oak, neutral | 2.64 (lit, it grows about 5 phytomers per kept one, F2) | 1.24 | 0.73 | **4.6 µs** |

The oak grown in F1 grows no shed subtree beyond one level, so it should cost about 1 to 2 µs a kept phytomer (estimate, from the beech and spruce).

So one chain costs:
- the median chain (4 to 7 phytomers): about 5 to 15 µs, plus its children's own chains for the one-level summaries, about 2 to 3 times that (estimate);
- the largest chain, the trunk (404 to 624 phytomers): 0.5 to 1.5 ms (estimate).

**The closed form** per species and age, measured on one core with today's recursion:

| Species | Time | Expected phytomers |
|---|--:|--:|
| Oak | 1.54 s | 17.2M |
| Beech | 0.44 s | 9.6M |
| Spruce | 0.22 s | 18.9M |
| Palm | 0.3 ms | |

It is paid once per species and age, at load time, not per tree. The new stand-in quantities extend the same memo: about 2 to 4 times this (estimate).

**The coupling pass** (oak only today). The grid sweep is about 0.33 s on one core whatever is resolved; the grid's size is the crown's. The rough layout and re-lays scale with the coupling share, about 0.17 × 2.78 s ≈ 0.47 s. Growth of the coupling chains is about 0.1 to 0.25 s. **About 1 s on one core** against today's 7.85 s for the whole oak (estimate). The sweep and the lay of each depth parallelise as today.

**A fly-in from the whole tree to a twig at 5 cm.** 2 s at 60 fps, 40 m to 5 cm, geometric; conservative half-pixel rule; ancestors first (measured, probe):

| Tree | First frame (40 m) | Next 119 frames, total | Most in one frame | Resolved in all |
|---|--:|--:|--:|--:|
| Oak 1 / 7 | 745k / 1.90M | 243k / 427k | 10k / 20k | 90 % |
| Beech 1 / 7 | 1.30M / 2.18M | 249k / 394k | 16k / 31k | 95 to 97 % |
| Spruce 1 / 7 | 4.59M / 4.58M | 143k / 108k | 44k / 39k | 99 % |

Per frame after the first, at 1 to 1.5 µs a phytomer: the worst frame is 10 to 70 ms on one core, 2 to 9 ms on 8 (estimate). Prefetch spreads it over the frames before: a chain is resolved when its error, extrapolated along the camera's motion over the resolution latency, crosses half a pixel.

The first frame at 40 m is 0.75M to 4.6M phytomers: 0.6 to 7 s on one core, about 0.1 to 0.35 s on 32 if chains parallelise ideally (estimate). It is not one frame.

**Memory** (measured struct sizes: a `Phytomer` is 128 B, an `Axis` 224 B):
- The coupling pass keeps its chains: oak 167k to 483k phytomers, 21 to 62 MB.
- Its records: about 12 B a fine root per living cycle, so 12k to 36k roots is under 35 MB (estimate).
- A close-up holds its ancestors and the visible chains, 50k to 200k phytomers: 6 to 26 MB, plus F3's 32 B curve point each.
- Today's peak memory, after F2, is 0.6 to 3.9 GB a tree (F2's R2 table).
- **Unknown:** the size of the closed form's memo per species.

## 6. (f) The change to the trees

**Beech, spruce, palm.** Light is neutral, so growth reads no coupling: **their topology is unchanged to the bit** (rows 1 to 5, 18). Only geometry changes:
- **Girth.** The palm is one chain, with no change. The spruce's junction spread is small: a fine root's base radius against its kind's mean is p5 0.71 to p95 1.20. The beech's is p5 0.33 to 0.40, p95 1.29 to 1.34 (measured). With the one-level summary only grandchildren are expected, and a big axis sums many of them. **Estimate:** trunk and limb girth within a few percent.
- **Sag.** Spruce and oak only: loads beyond one level are expected, so droop moves by the grandchildren's spread. **Unknown**, measured in R1.
- **Shedding.** A lateral certain to be shed (F2's bound, `shed.rs:270-303`) sheds exactly as before. Others now read their children's expected extinction, so a few keep or shed decisions near a delay change: **unknown**, small (estimate).

**The oak.** It changes where light couples:
- **What is measured:** the grid's cut costs 0.1 to 0.2 % of bud light at realized leaf areas. A fine root's one read costs a p95 of 5 to 9 % of a bud's light, with a median of 1 %.
- **What is not measured:** the closed form's neutral expectation against the realized leaf area of a shaded fine subtree.
- **The spread is large:** a fine root's base radius against its kind's mean is p5 0.09 to 0.10, p95 2.0 to 2.1, measured. Stand-ins are unbiased in sum only if the form answers light (Q5).
- **The bound from F2's probe:** skipping the shed subtrees with **no** stand-in removed 56 to 62 % of the oak's grown wood from every coupling, and cost 11 % of kept wood and 1.0 to 1.5 m of height.
- **The stand-in:** it keeps all four couplings, the same expected leaves, vigour share, balance and retained pipe.
- **Estimate:** height within ±0.5 m and kept wood within ±5 %. The inner crown's density of fine shoots, which light sorts, is where the look may move. To be judged on stills (R3).

**F2's oak question, answered.** A certain-shed subtree of the oak is not grown. Its stand-in carries the four couplings F2 named:
1. **Shade:** expected leaves deposited each year at its envelope until its bound.
2. **Vigour share:** E[apexes] × its light read, in its bearer's split.
3. **Bearer's balance:** its expected light and upkeep in the bearer's sums.
4. **Retained pipe:** retained × expected section at its last growth.

The same holds for any fine subtree the camera never reaches.

## 7. (g) Pipeline fit

**Today:** the engine sits outside the pipeline (`lib.rs:9`, "Nothing here is wired into the pipeline", checked). The stills runner converts a `Structure` into a pipeline `Tree` (`telperion-render/examples/space/tree.rs:107`, checked). F4 (fn-198) integrates it.

**Under F1:** the skeleton stage's product becomes a lazy `Structure`:
- the coupling pass's records;
- the resolved chains, keyed by lineage;
- the bud records, each a closed-form stand-in;
- one function: `refine(view, error) -> chains`.

The one pipeline holds if one-pass `build` is "resolve every chain" through the same stage function, so a second chain does not compile, as now.

**What consumes per chain:**

| Consumer | Today (checked) | Under F1 |
|---|---|---|
| Curve (F3) | `Curve` runs: a lateral starts a run (`tree.rs:3-4`; `docs/pipeline.md`, "The wood's curve") | a run per chain, appended as chains resolve; clusters are built per run already |
| Leaf plan, bearing runs | `pipeline/foliage/plan.rs:270-290`; the slender test reads `tree.stem_radius` (tree-wide) | per chain; `stem_radius` from the trunk chain, resolved first |
| Leaf reference box | `pipeline/foliage/reference.rs:91` reads the box off the grown tree (tree-wide) | the box of the coupling tree and the stand-ins' envelopes, known before any fine chain |
| Twig placement, stations | per run (**unknown** whether any reads beyond its run) | per chain, to be checked in F4 |

**What moves:**
- `grow::run` (`grow.rs:146-289`) splits into the coupling pass and `resolve(chain)`.
- `sketch.rs`, `balance.rs` and `relay.rs` serve the coupling pass.
- `shed.rs` fades, `girth.rs`, `disused.rs` and `sag.rs` gain one-level summaries.
- `closed_form.rs` gains the stand-in tables.

**A new door:** `refine` is a door in `pipeline::executor` (`docs/pipeline.md`: what the pipeline exposes is what may be called). That is a host decision.

## 8. (h) Recommendation, order of steps, and open questions

**Recommendation.** Take the host's three aims with four refinements:
1. **The chain** is the unit of resolution.
2. **The read rule** in section 0.
3. **The coupling tree** at the lattice's cell, with exact feedback above it and stand-ins below.
4. **One-level look-ahead** for girth, sag and fade.

Then build the light-free species first: there, topology is unchanged and only geometry moves.

**Order of steps:**
1. **Closed-form stand-in tables:** section, mass, envelope, extinction. Each is checked against the mean of keyed subtrees (an oracle test) before anything reads it.
2. **A chain grown alone**, reading only its records and stand-ins. One-pass means resolve all. Beech, spruce and palm first: their topology stays bit-identical to today, which is a test.
3. **Girth, sag and fade by one-level summaries.** Measure the change on the three species' stills against their passed sheets.
4. **The coupling pass for light** (the oak). Measure height, kept wood and the inner crown against round 10 and against F2's probe.
5. **The streamed-equals-one-pass test** (section 4) for all four species at seeds 1 and 7.
6. **R2:** the error rule, prefetch, and the fly-in with every frame's resolution time recorded.
7. **R3:** stills. **R4:** the gate and Codex.

**Questions this proposal cannot settle:**
- **Q1, the stand-in's error metric.** The owner set half a pixel. Taken as positional error over a stand-in's whole box, it resolves 71 to 98 % of a tree at 40 m and 37 to 53 % at 200 m. Is a foliage stand-in judged by position, or by whether stills show a difference (a statistical criterion measured on stills)? This decides what a forest costs.
- **Q2, the first-view bar.** "Detail within one frame of becoming visible, at any camera speed" cannot hold for a whole tree appearing at 40 m: 0.75M to 4.6M phytomers, about 0.1 to 0.35 s on 32 cores (estimate). The options:
  - stand-ins shown first and refined over a few frames;
  - the bar held along continuous camera paths with prefetch, and a cut stated apart.
- **Q3, coupling-scale continuity.** Classifying a bud by expected extent over a cell is a switch in a walk. Possible fixes:
  - a ramp, where a bud near the cell is grown in the pass and its couplings blended by its distance from the cell in log extent;
  - accepting a measured step.
  The cell itself is an existing engine constant (`light.rs:296`), named for that reason; any ramp width would be a new one.
- **Q4, conservation.** Taking r̄ from stand-ins makes growth no longer exactly conserved within an order (`allocation.rs:195-197`). Accept, or normalise inside the coupling pass with a correction term?
- **Q5, the light response of stand-ins.** The closed form is neutral-light, so shaded fine subtrees are over-expected. The options:
  - closed-form tables at a few light levels, interpolated linearly, so the response stays continuous: memo × levels;
  - accept the bias and measure it.
- **Q6, the pipeline door.** `refine(view)` in the executor interface, or another shape.
- **Q7, one light read per fine root.** It costs a p95 of 5 to 9 % of a bud's light inside the subtree (measured). Per-bud reads would need each fine chain's position history from the coupling pass's frames, which costs memory per re-lay. Accept the one read?
