# Counted foliage: a design proposal, 14 Nov 2026

By a Fable agent, read-only, for the host to decide. Nothing was built, run or measured. Code was read at `.worktrees/cand` (`7685ee85b`, branch `fn-351-candidate`).

**Tags.** [code] read in source, with file and line under `crates/telperion-space/src/` unless a crate is named. [record] a file under `.flow`. [arithmetic] computed here from tagged inputs. [unknown] not determined. A design rule carries no tag: it is a proposal.

**The answer in short.** Most of the tree is fine shoots: 69 to 76% of held phytomers, 82 to 97% of held axes and 85 to 96% of living tips on the three measured species. A design exists that carries them without phytomers, axes or geometry and keeps every keyed draw, so no expectation replaces a draw. On paper it takes the oak at 40 from 1.44 s to about 0.3 s while the twigs stay unbuilt, and to about 0.7 s when every twig is built for a still. It does not reach 0.1 s. The growth-pass gain is the same size fn-340 measured (3.9 times); what this design adds is the tree staying inside tolerance, which fn-340's did not.

## 1. Where the phytomers are

### What is on record

The only split by kind is the Phase 5 census: seed 1002, 12 Nov, at `48edda0a7`, before fn-357, fn-358, fn-363 and the candidate's merges [record: `PHASE5-CENSUS.md`, the M1 tables per tree]. "Fine" below is the kinds section 2 would count: oak shoot, twig and short (ages 12 to 14); beech short (14); spruce spur and shoot (11, 12).

| Tree | Held phytomers | In fine kinds | Held axes in fine kinds | Living tips in fine kinds | Year's leaf on fine kinds |
|---|---|---|---|---|---|
| Oak 80 | 5,693,800 | 4,292,999 (75.4%) | 2,160,714 of 2,237,329 (96.6%) | 358,757 of 374,527 (95.8%) | 93.1% |
| Oak 120 | 11,558,758 | 8,052,406 (69.7%) | not summed here | 686,799 of 719,340 (95.5%) | 93.3% |
| Beech 40 | 571,516 | 434,200 (76.0%); with the shoot, age 12, 533,463 (93.3%) | 53,329 of 65,136 (81.9%) | 28,232 of 33,230 (85.0%); with the shoot 97.8% | 82.3%; with the shoot 95.9% |
| Spruce 40 | 1,167,938 | 803,983 (68.8%); with the branchlet, age 10, 99.0% | 685,487 of 705,882 (97.1%) | 98,433 of 106,843 (92.1%) | 68.9% |

[arithmetic on the census rows.] The structural wood that remains is mostly one kind. On the oak it is the branch (age 9): 1,338,791 phytomers at 80 (23.5%) and 3,328,799 at 120 (28.8%). Trunk, limbs and boughs together are 1.1 to 1.5% [arithmetic on the same rows].

- **The oak at 40:** no split by kind on any build [unknown]. The nearest records: fn-339 found 0.925 of the cycle's growth units inside twig and short cohorts at 40, on a build before fn-349 [record: `fn-339/R3-RESULT.md`]; the oak at 40 on seed 42 keeps 348,790 phytomers [record: `.worktrees/fn-369/.../raw/table.md`]; 136 k axes are grown and 155 k closed subtrees evaluated [record: `fn-368/READING.md`].
- **Telperion:** no split by kind [unknown]. Its preset states the beech's fine ages: shoot (12) lifespan 5, short (14) lifespan 12 with a cluster of 4, `shadeShort` 9 and 12 into 14 [code: `telperion-core/presets/telperion.values:564-662`]. It keeps 3.70, 5.26 and 6.12 M phytomers at 82 on seeds 1, 7 and 42 [record: `fn-369/CEILING.md`, table E]. The beech's shares are the only basis for a guess, and they were taken at 40, not 82.
- **Time by kind:** no record splits the grower's seconds by kind [unknown]. The per-pass split exists [record: `fn-368/READING.md`]: on the oak at 40 the six yearly passes take 1.06 s of 1.39 s (Units 0.30, Balance 0.26, Sketch 0.27, Wood 0.11, ShadeField 0.08, Allot 0.04) and the settle and the rest 0.33 s. Units, Balance and Allot visit living tips and open axes; Sketch and the deposit visit the year's new phytomers; Wood visits every held open phytomer [record: the same file, "The six passes"]. So the fine kinds' share of each pass follows their share of tips (85 to 96%), of the year's leaf (69 to 93%) and of held phytomers (69 to 76%), if an item costs the same in every kind. That last condition is not measured; fn-245 found a scaffold axis costs 900 to 2,270 ns a cycle in the sweeps against 115 to 347 ns for a short-lived one [record: `fn-245/COUNCIL.md`, "The scale split, measured"], so the structure's share of time is larger than its share of items.
- **The one old measurement of the floor:** fn-340's probe without its cohort step ran 9.5 times cheaper than the cycle on the oak at 40 (0.104 s against 0.987 s), 4.2 on the spruce, 4.9 on the beech [record: `fn-340/RESULT.md`, "The share of the probe's time"]. That is the structure alone, on the older grower.

### The one measurement that closes it

Port the census reader to the candidate (`.worktrees/census-p5`, uncommitted: `grow/cycle/phase5.rs`, `Paused::phase5()`, `telperion-core/examples/p5census.rs`) and run its M1 tables for the oak at 40, the beech at 40, the spruce at 40 and Telperion at 82, seed 1001. Add two counters summed over all years by physiological age: apex-units grown and phytomers laid, both in `grow_unit` (`grow/unit.rs:16`). The tables give the share of kept phytomers, axes and tips; the counters give member-years, the input the cost arithmetic of section 3 lacks. I stop guessing here.

## 2. The design

### What is counted

A lateral is a **member** when its kind passes two tests, both read from existing quantities and neither from a species name:

- **Closed by table.** `shed::lasting` gives the kind a bound: a bounded lifespan, no sleeping bud that wakes, no relay, every kind it bears also bounded [code: `shed.rs:27-61`]. The function exists but the one cycle sets every entry to none (`grow/cycle/paused.rs:104`); the design reads it again, for this purpose only.
- **Shorter than a cell.** The kind's extent bound, at base scale 1, is at most `light::CELL`, 0.5 m [code: `light.rs:19`; `closed_form/kept/bound.rs:27-104`]. The bound is test-only today (`closed_form.rs:20-21`) and becomes product code.

By the shipped rows this counts the oak's 12, 13 and 14, the beech's and Telperion's 14, the spruce's 11 and 12, the birch's 14, and leaves out the beech's shoot (0.86 m) and the spruce's branchlet (0.80 m) [record: `PHASE5-CODE-ANSWERS.md`, section 3]. A bud drawn as a member's kind that turns kind under shade is tested at its turned kind, as `shed_certain` is today [code: `grow/unit.rs:85-98`].

### What a member is, and what the count is

A member is a lateral that has no `Axis`, no `Draws`, no `Pencil` and no phytomers. It lives in a flat list on its bearer: the bearer's growth unit, node and slot name it, and its lineage key is the key the bud would have had (`place.onto(keyed)`, `grow/unit.rs:124`). A member's bearer is an axis or another member (an oak short on an oak twig).

Its state, about 40 bytes and 12 more a growth unit:

- birth year, kind, node and slot on the bearer, units grown, the year its apex ended;
- `base_scale`, `running`, `kept`, the balance `memory` and the year's light, as a pencil holds them (`grow/cycle/passes.rs:56-58, 132`);
- per unit: `survive`, `persist`, `size` (what `presence::Unit` holds, in f32);
- its base section and sapwood, `[T, S]`;
- its base direction, drawn once from its keyed roll and insertion on the bearer's frame.

Node counts, bud places, angles, wander and cluster leaves are functions of the key and are never stored [record: `PHASE5-CODE-ANSWERS.md`, section 5, the table of draws].

The **count** is what the tree-wide passes read of a bearer's members: one set of sums a bearer (living members, lit presence, light, charge, leaf by node). The passes never visit a member.

### The yearly law, pass by pass

One new pass, the member step, runs once a year over each bearer's list, bearers in the working set's order and members in node and slot order. Each line below names what a member does and the existing function it calls. The rule that holds the design to one law: **`advance` is split into a decision (key, state, cycle, light, carried share in; outcome and presences out) and the laying of geometry, and the member step calls the same decision. Nothing in `grow/advance.rs` is written a second time.**

| Pass | An individual today | A member | Exact? |
|---|---|---|---|
| Extension | survival draw `decide.child(VIABILITY)` against `shaded(viability, hazard, light)`, presence from the lead (`advance.rs:66-103, 160-176`); zone node draws; phytomers pushed | the same draw, key and threshold; node counts drawn, their presences summed into the unit's leaf; nothing pushed | The same draw. Its light is read at another point (below). |
| Branching | bud places drawn per node and slot; `sprout_keyed` makes an axis (`unit.rs:78-124`) | the same place draws; a bud of a counted kind becomes a member of this member; a bud whose kind is not counted **promotes** this member (below) | The same draws. |
| Light read | trilinear at its last phytomer's tip (`grow/sketch.rs:100-119, 237-239`) | trilinear at its **chord tip**: base place + base direction x the length of its units | Approximate: the chord leaves out wander, tropism and sag. |
| Light deposit | each new phytomer's leaf at its tip (`grow/deposit.rs:20-27`) | each new unit's whole leaf (the sum of `girth::leaf` over its nodes) at the unit's chord midpoint | Area exact; up to 4 or 5 deposits become 1, moved by at most half a unit's length. |
| Balance | `q = size x light`, `c = size x (height x inverse height)^2`, then `contract::shed::balance` with its `SHED` draw, memory and window (`passes.rs:43-117, 153-163`; `contract/shed.rs:53-79`) | the same function, draw and memory; height from the chord tip; `keep` and `shed` handed down from its bearer as the walk does | The same draw. Height is the chord's. |
| Balance, the bearer | `Working::totals` adds each open child's surplus `max(0, light - cost)` (`working.rs:205-224`) | the bearer's `own` adds each living member's surplus, in node order | Exact but for f32 order. |
| Allotment | lit presence into `Working::split`; size `(v/p)^psi x total / weighted` (`working.rs:230-256`; `allocation.rs:147-183`) | members stand in the bearer's split as children at their node; their `[p, p(v/p)^psi]` partials are summed in list order and merged with the axes' | Where `shade_size` is 0 the size is exactly 1 (`allocation.rs:148-150`): beech, spruce, birch. On the oak (`shadeSize` 1 on 12 to 14) exact but for f32 order. |
| Wood | `girth::lay` gives each phytomer `ring` under the leaf above; a lateral hands its leaf sum to its bearer at its node (`girth.rs:64-126`) | the member hands the same sum to its bearer's node; its own base `[T, S]` takes the same `ring` under its whole leaf | The bearer's wood is exact. A member's inner sections are rebuilt at build time (below). |
| Open, retire | an open lateral holds its bearer open; a closed subtree is copied and settled to find its living (`retire.rs:260`), 0.12 of the oak at 40's 1.39 s [record: fn-368] | a bearer with a living member stays open; a member whose apex ended keeps its end year and leaves the list after its kind's `shedding` delay, its last living entering the bearer's fade as a `Stand` does (`shed.rs:88-95`) | The same rule; no copy, no evaluation. |
| Wake, relay | none on a closed kind (`grow/wake.rs:18-57`) | none; a draw that would make one promotes | n/a |
| Flowering | read by the conversion from the built axis's row (`telperion-core/src/pipeline/space.rs`, `unfolded`) | read from the built twig, unchanged | Exact once built. |

**Promotion.** A member whose draw makes something a count does not carry (a relay, a continuation, an abortion's relay, a sleeping bud, a lateral of a kind that is not counted) is built from its key that year and goes on as an ordinary axis. A walk that raises such a row from 0 promotes members at that row's probability, so the tree changes by degree in the row.

### From a count back to individuals

**Built from its key.** The build runs the existing layout on the member's record: its frame from the bearer's node (`grow/draw.rs`, `pencil`), each unit laid by `draw_bent` with the stored `survive`, `persist` and `size` and the keyed node counts, then its members on its nodes in turn. The result is an `Axis` the settle and the conversion read as they read any other. Two things the record cannot give exactly:

- **Inner sections.** A phytomer's section is `ring` over its own leaf history. The build replays `ring` from the stored unit leaves and the members standing at build time; a member shed earlier is missing from that history. This is wood of a few millimetres at most, drawn only.
- **Sag.** The built twig bends under the settle's rules; the growth read its chord. The twig's history is fixed by then, so the tree is consistent, and the same twig comes out every time it is built.

**When.** Three consumers, in the order they can be built:

1. **The settle of a still or a mesh** builds every member. The drawn tree is then the same kind of tree as today, through the one pipeline, and the leaf plan is untouched.
2. **A view** builds the members of the bearers its error admits (the curve's cluster test, `pipeline/surface/curve/tessellate/plan.rs`). No consumer builds on demand today [record: `PHASE5-SURVEY-2026-11-13.md`, section 3]; this is Phase 6's.
3. **The field** reads a bearer's leaf by node from the count and builds nothing. It reads the drawn tree today (`pipeline/field.rs`), so this is also Phase 6's.

### Determinism

- **Same seed, same tree.** Every decision is a keyed draw against a threshold that reads the member's own state and the lattice. Member lists are ordered by node and slot. The lattice's quanta are order-free (`light/field.rs:157`). Partials merge in list order, as `size_sums` already requires (`allocation.rs:157-173`).
- **Pause equals grown through.** A member's state at year y is a function of years 1 to y only; nothing reads the age asked for. `lasting` is read as a table of the species, not through `shed_certain`, which reads `rules.age` and stays off [record: `PHASE5-CODE-ANSWERS.md`, section 2].
- **Any cores.** The member step is per bearer and touches only that bearer's list and sums.
- **A test that holds the law to one.** Under neutral light with no potential height, no pass reads a position, so a counted tree built in full must equal the individual tree to the bit. That is a red/green test for the split of `advance`, and it keeps the closed form's oracles, which grow that way [record: `PHASE5-CODE-ANSWERS.md`, section 5, "Under neutral light"; not run].

### Exact, approximate, expectation

- **No expectation replaces a draw anywhere.** Every survival, shedding, node count and bud place is the individual's keyed draw.
- **Exact:** counts of units, nodes and buds given the same light; leaf area into the lattice; leaf handed to the bearer's wood; the balance and allotment arithmetic.
- **Approximate:** the point where a member reads light and height (chord tip against bent tip), the point where its leaf enters the lattice (one a unit), and its inner sections.
- **What fn-245's tolerance says.** Its bar was kept phytomers within 5% and stills not told apart at whole or mid view. A stand-in grown from the same draws against the right field passed at +1.2 to 1.5%. The closed-form expectation failed at +14 to 17%. A field held stale since a twig's birth failed on the oak at +2.5 to 4.2% [record: `fn-245/COUNCIL.md`, "The scale split, measured"]. This design sits between the first and the third: the right year's field, read some centimetres from the true tip. The oak's failure at 2.5% says the margin is thin and the probe must measure it.

## 3. Expected result on paper

### Inputs

- Oak at 40, one thread: 1.44 s by its own clock; six passes 1.06 s, the rest 0.33 s [record: fn-368 `READING.md`, `ROUNDS.md`].
- Fine kinds' shares on the oak, taken from the census at 80 because none exists at 40: tips 96%, held phytomers 75%, axes 97%, year's leaf 93% [record; the transfer to 40 is an assumption].
- A member-year's cost: one trilinear read, one shaded threshold, two hashes and compares, four node draws, one deposit, one `ring`. About 0.2 to 0.25 µs [arithmetic, an estimate; not measured]. fn-340's carried cohort step cost 0.59 µs a class-year [arithmetic: 0.100 s over 169,000 class-years, `fn-339/RESULT.md` and `fn-340/RESULT.md`].
- Member-years on the oak at 40: 136 k axes x 0.97 x about 3 years of life, about 400 k [arithmetic; the life is a guess, the counter of section 1 replaces it].

### The oak at 40

| | Today | Counted, twigs unbuilt | Counted, every twig built |
|---|---|---|---|
| Six passes on the structure | 1.06 s | about 0.14 s (tips 4%, new phytomers 7%, held phytomers 25%) | 0.14 s |
| Member step | | about 0.09 s (400 k x 0.22 µs) | 0.09 s |
| Settle and the rest | 0.33 s | about 0.08 s (a quarter of the phytomers) | about 0.33 s, plus the layout of the twigs, about 0.14 s (93% of today's 0.15 s of `Layer::step`) |
| **Total** | **1.39 to 1.44 s** | **about 0.3 s, 4.6 times** | **about 0.7 s, 2 times** |
| Phytomers held in the grower | about 349 k | about 87 k, and about 100 k member records | the same while growing |

[arithmetic.] Cross-check: fn-340's probe measured the oak at 40 at 0.407 s against 1.556 s, 3.88 times, with its twigs unbuilt [record].

**The aim of 0.1 s is not reached.** The structure alone was 0.104 s on the older grower [record: fn-340], and today's cycle is 1.4 times slower than that one.

### Telperion and Laurelin

No measurement of either by pass or by kind [unknown]. By the beech's shares at 40 and the beech at 80's passes (23.7 s in the six passes of 27.4 s [record: fn-368]):

- **The short shoot only** (the design as stated; tips 85%, phytomers 76%): the six passes fall to about 6.4 s with about 1.0 s of member step inside that, the total to about 7.3 s unbuilt, 3.7 times [arithmetic]. Laurelin's 170 s would be about 45 s, and the grower would hold about a quarter of its 5 to 6 M phytomers, 1.2 to 1.5 M.
- **With the shoot counted too** (question 2 of section 6; tips 98%, phytomers 93%): about 3 s, 9 times; Laurelin about 19 s; 0.3 to 0.4 M phytomers held.

### Memory

- **The grower.** The oak at 120 holds 3.845 GiB after fn-357 and fn-358, 1.069 GiB of it phytomers [record: goal log, 12 Nov 09:24]. Counting removes 70% of the phytomers and 96% of the live axes and adds about 2 M member records at 40 to 80 bytes. About 0.6 GiB [arithmetic], beside fn-340's measured 2.18 to 0.61 GiB [record].
- **The built tree.** A tree built in full for a mesh peaks in the conversion and the foliage, at 96 bytes a node and more [record: `fn-369/CEILING.md`, "At the high-water mark"]. Counting does not lower that peak. It lowers it only for a consumer that leaves twigs unbuilt.

### What the drawn tree loses or changes

- Built in full, every twig comes from the same layout code and the same keyed angles, so a twig looks as a twig looks today, and leaves stand where the plan puts them today.
- Which twigs live changes, because each read its light a few centimetres from where it would have. Every seed grows a different individual of the same family. Digests are re-pinned, which the generator-evolution rule allows.
- Leaf area in the light is conserved to the quantum. Girth is conserved on every structural phytomer given the same members, because the bearer receives the same leaf sum; fn-340's trunk bias came from pipes entered into a mean, which this design does not do.
- A member's inner taper is rebuilt approximately. Sub-millimetre to a few millimetres.

## 4. What stopped the earlier attempts, and whether this falls the same way

| Attempt | What stopped it | This design |
|---|---|---|
| fn-245 form (a), closed-form stand-in | oak +14 to 17%, half the sheds [record] | No expectation is used. |
| fn-245 form (c), grown at birth | stale field, oak +2.5 to 4.2% [record] | Reads the current year's lattice. |
| fn-245 form (d), flat yearly pass | exact, but the twigs' growth and layout (380 to 507 ns a phytomer) stayed, so the gain was only the sweeps [record] | The same scheduling, and it drops the phytomers and the layout. That is where it stops being exact. |
| fn-339, fn-340, cohorts in expectation | trunk 10 to 36% thick, leaf 0.81 at 120, a fifth fewer living tips open-loop; the step took 49 to 68% of the probe; 3.9 times, not 5 [record] | Draws kept, so no open-loop bias by construction. **The speed is no better than fn-340's.** If the bar is five times on the oak, this fails it too. |
| fn-340, twigs never drawn | cohort leaf was counted and never drawn [record] | A member builds into an ordinary axis through the existing layout. Not yet proven: it is part of the first probe. |
| fn-356, five rules on faint shoots | a rule that treats a faint element differently breaks the closed form, the independence of draws or the extent bound [record: survey of 13 Nov, section 2] | Membership is by kind, never by faintness; every draw and count stands. Under neutral light the tree is equal to the bit (unrun). |
| Census candidate C, scaled templates | realised subtrees at 0.04 to 0.36 of a light-free expectation [record] | Members read light every year. |
| fn-370, the drawing draws the grown leaf | the plan gives no leaves to a third to a half of the oak's leafed wood; the beech's grower under-leafs [record: `fn-370/R1.md`, host verdict 14 Nov] | Untouched while every twig is built: the plan runs as today. **It returns the day leaves are drawn from a count without their twig** (consumers 2 and 3), which needs fn-370's questions answered first. |

**What it does not solve.** The oak's faint branches: 1.74 M of the 4.45 M faint phytomers at 120 are of the branch kind, which relays and is not closed [record: census]. After counting, the branch kind is most of what the old oak holds.

## 5. The smallest first build

**One probe, the oak's short shoot as a member, on the oak at 40.** A patch behind a switch, as fn-339's was; no product change. The short shoot is the simplest member (no laterals, so no nesting) and the largest class: 45% of held phytomers, 81% of axes, 77% of living tips, 71% of the year's leaf at 80 [arithmetic on the census].

Steps, each stopping the probe if it fails:

1. **Counters, about an hour.** The census tables and the two counters of section 1, on the candidate. They replace this section's assumed shares before the bar is read.
2. **The chord, about two hours, no behaviour change.** In today's grower, for every living short shoot every year, read the light and the height at the chord tip beside the true tip, and re-decide survival and the balance at both.
3. **The member, the rest of the day.** Short buds become members; the member step; the hooks in `totals`, `split`, `lay`, the deposit and the open pass; the build before the settle.

**The bar, derived before the run** [arithmetic]:

- **Step 2 passes** if the net change in deaths (deaths added less deaths removed) is at most 1% of member-years, and the mean distance from chord tip to true tip is at most 0.05 m, a tenth of a cell. Why 1%: a short shoot lives at most 3 years, so a 1% yearly bias moves the standing population by at most 3%, inside fn-245's 5%.
- **Step 3, time.** The six passes on seed 1001, one thread, by fn-368's timers: 1.06 s today. Shorts removed: Units 0.30 x 0.75, Balance 0.26 x 0.77, Sketch 0.27 x 0.67, Wood 0.11 x 0.45, ShadeField 0.08 x 0.25, Allot 0.04 x 0.5, together 0.70 s; the member step adds about 0.05 s (about 260 k member-years x 0.2 µs). Expected 0.41 s. **Pass at 0.50 s or under. Kill over 0.70 s:** a third or less saved by the largest class means the full design cannot reach three times.
- **Step 3, the tree.** Seeds 1001 to 1012 at 40, every member built: the medians of height, spread, trunk diameter, the year's leaf and kept phytomers each within 5% of the cycle's. **Kill if any is over 10%** (fn-340's rule).
- **Step 3, determinism.** `a_tree_paused_and_grown_on_is_the_tree_grown_at_once` (`grow/cycle/paused/tests.rs:70`) and `many_cores_grow_the_tree_one_does` (`grow/tests.rs:153`) pass with the switch on; the neutral-light tree equals the individual tree to the bit.

A pass on all four says the twig and the shoot are worth building as nested members. A pass on time with a fail on the tree says the chord is too coarse, and the next attempt carries one bent tip a member (12 more bytes, one `Layer::step` a unit instead of one a phytomer).

## 6. Open questions

### The owner's

1. **The bar.** This design's paper gain is 4.6 times on the oak at 40 with twigs unbuilt and 2 times with every twig built, against an aim of 14 times. Is "3x cheaper is pretty good already" (10 Nov) the bar for Phase 5, with the rest left to Phase 6's views?
2. **The beech's shoot and the Two Trees.** The shoot is closed by table and reaches 0.86 m, so it leaves a cell and is not a member under the ruling's words. Counting it moves the Two Trees from about 3.7 to about 9 times on paper. Does "cannot leave a cell" bind, or is "closed by table" enough?
3. **The tolerance.** Is fn-245's (kept within 5%, stills not told apart at whole or mid) the measured tolerance AGENTS.md cites for counting? No other is on record.
4. **The old oak's faint branches** are outside what was approved on 11 Nov and outside this design.

### The host's

5. **By degree at the membership line.** A walk that moves a kind's bound across 0.5 m, or opens a closed kind, switches its laterals between two ways of reading light. Promotion makes row-driven openings gradual; the reach line stays a step. Whether any walk test crosses it is [unknown]. Dropping the reach test (question 2) removes the step and keeps the chord's error growing by degree with reach.
6. **Where the member step sits in the year,** and whether the split of `advance` into decision and laying is acceptable as a refactor of the one cycle.
7. **`Kept::bound` and `shed::lasting` as product code:** the first is test-only, the second is switched off in the one cycle.
8. **A consumer for unbuilt counts.** Until the field or a view reads a count, every member is built at the settle and the gain is 2 times. Principle 4 asks each new output to have a reader.
9. **Sag of a bearer with unbuilt members** needs their load from the record (length x section), which no rule above defines.
10. **The oak's `shadeSize`** puts members into the allotment's normaliser. The merge order of member partials with axis partials (blocks of 1,024 buds) needs a ruling like form (d)'s [record: `fn-245/COUNCIL.md`].

## What was not read or run

Nothing was run. Not read: `grow/unit.rs`, `grow/draw.rs` and `grow/cycle/retire.rs` past the lines cited, which rest on `PHASE5-CODE-ANSWERS.md`'s reading at `8f6b017e2`; the walk tests; the foliage plan and the field past their module docs; fn-339's `R1-LAW.md` and fn-340's ledgers; Telperion's and Laurelin's rows beyond the fine ages. The cost of a member-year (0.2 to 0.25 µs) is an estimate from operation counts and is the weakest input in section 3.
