# Dead standing wood and the settle's fade: a reading, 14 Nov 2026

Read-only. Nothing was built, run or committed. Code read at `.worktrees/fn-364` (crate `crates/telperion-space`). Records read in this folder: `OLD-OAK-WOOD.md` (tables A, B, C, E, G, J and its readings; tables D, F, H, I skimmed), `SURPLUS-PROBE.md` (the settle trace and the round 3 table), `RULINGS.md`, the goal log entries of 12 to 14 Nov, and the spec `fn-359-a-subtree-fades-once.md`. This file lists remedies and does not choose: the host decides.

Tags: [code] read in the source named; [record] from a record in this folder; [source, full text] a passage of the paper's body was read; [source, abstract only]; [derived] my inference from tagged facts; [unknown] not checked.

The two defects, as recorded:

- **Dead standing wood.** 42 to 52% of an old oak's settled phytomers sit on axes with no living apex below them, held by the grower and handed to the drawing [record: OLD-OAK-WOOD table B].
- **The settle's fade.** `shed_past` clamps a limb whose presence-weighted living time lags to its most present living apex, 0.13 to 0.22 on the traced limbs, so settled limbs are drawn at about a fifth of the size the grower laid, lit and balanced them at [record: SURPLUS-PROBE, "The settle trace"].

## 1. How each mechanism works today

### 1.1 Who sets the shedding delay

- The delay is `PaState::shedding`, one number per physiological age, in cycles: "Cycles a lateral axis of this PA keeps with no living apex before it is shed, fading out over the cycle past it" (`species.rs`, `PaState`). Infinite means never shed [code].
- A preset states it as `/space/ages/<k>/shedding` (`crates/telperion-core/src/presets/space.rs`, `put`; the token `never` gives infinity). The settings table lists it as kept, in years (`species/scale/table.rs`) [code].
- The fidelity dial shortens it: `Species::coarsened` multiplies each coarsened age's delay by `shedding^share`, with a fitted row per preset (`species/coarse.rs`; `/space/coarse/shedding` is 0.243 for the oak, 0.051 for the spruce, 0.65 for the birch) [code]. The oak's comment names these rows a fit from fn-328, not a source [code: `oregon-white-oak.values`, lines 614 to 622].
- A walk between two species walks the delay (`crates/telperion-core/src/blend.rs`, line 188) [code].
- Only laterals and relays carry a delay. The seed and continuations never shed on their own; they go with their parent (`shed.rs`, `fades`, the `delay` closure; `shed_past`, `lost`) [code].

The rows, as shipped [code: `presets/*.values`]:

| preset | rows (age: delay in cycles) |
|---|---|
| oregon-white-oak | limb 6: 5, bough 7: 4, sprig 8: 3, branch 9: 3, shoot 12: 2, twig 13: 2, short 14: 2 |
| european-beech (and laurelin, telperion) | 9: 1, 12: 1, 14: 1 |
| silver-birch | 6: 3, 7: 2, 8: 2, 9: 1, 14: 1 |
| norway-spruce | 9: 3, 10: 1, 11: 1, 12: 3 |
| ordinary | 6: 17, 7: 14, 8: 11, 9: 2, 10: 1.4, 11: 1.25, 12 to 14: 1.77 |

No preset file carries a source comment on a `shedding` row [code]. The only stated basis is in the space crate's species: the oak's limb row, "oaks hold dead branches for years before they fall (fn-197 host decision 18; estimated, an arborists' account, no measured persistence found)" (`oak.rs`, line 231); the spruce's branchlets are "shed when they die (M11)" (`spruce.rs`, line 258); the birch's and the beech's rows have no stated basis [code].

### 1.2 When a dead lateral is dropped

- The law is `contract/presence.rs`, `fade(delay, birth, (lived, alive), age) = clamp(delay − idle, min(alive, 1), 1)` with `idle = (age − birth) − lived`. `shed.rs`, `faded` calls it with `delay = shedding + 1`. A lateral or relay is lost when its fade is 0 or less, or when its parent is lost (`shed_past`) [code].
- `lived` is the subtree's last living time, counted from the axis's birth in presence-weighted cycles: its own sleep plus the sum of its growth units' presences, or the most that any child carries up (`fades`, the backward pass; `carry(before, weight, gap, lived) = before + weight × (gap + lived)`) [code].
- For whole presences the axis is kept at fade 1 while `idle ≤ shedding`, at the share left between `shedding` and `shedding + 1`, and dropped from `idle = shedding + 1` [derived from the law]. So a dead short shoot of the oak (delay 2) stands in three yearly states, the year its last unit grew and two more. Table B shows exactly that: 96.4 to 98.0% of the dead standing phytomers have "years since lived" 0, 1 or 2 [record].
- The oak's short shoot has a lifespan of 3 cycles (`/space/ages/14/lifespan = 3.0`) and a delay of 2 [code], so it stands dead about as long as it lived [record: goal log, 13 Nov 14:01]. At 200 on seed 1001, short shoots are 1,996,662 of the 2,715,376 dead standing phytomers, twigs 509,934, branches 140,308 [record: table C].
- 74 to 88% of the dead standing phytomers are at applied fade 1: they are waiting out the delay, not fading [record].

### 1.3 What a dead lateral costs between death and drop

In each yearly pass (`grow.rs`, `year`: `open`, `retire`, `balance_one`, `allot_one`, the advance, `sketch`, `girth::lay`, `compact`):

- **`open`** (`grow/cycle/passes.rs`). The working set is every axis on the way to a living apex or to a sleeping bud's parent. A dead subtree with no sleeper is not in it. It enters the `closed` list once, the cycle it closes (`working.rs`, `open_from`) [code].
- **`retire`** (`grow/cycle/retire.rs`). A closed subtree beside an open axis is evaluated once: a copy is assigned (`copied`, `assign_living`) and its living read (`shed::subtree_living`). If its fade at the age grown is not yet none it is `Kept` and joins `fading`. Each later cycle `faded_out` makes one `faded()` call per fading root. When the fade reaches none it is evaluated again and `let_go` frees its phytomers and draws; its root stands for it as a 24-byte `Stand` once `compact` runs [code]. So between death and drop a dead lateral keeps every record: axis, phytomers, units, draws, pencil, links.
- **`balance_one`**. "Wood that has died is charged nothing." Its `own` is `(0, 0, false)` and `Working::totals` skips a closed child. It earns nothing and costs nothing [code].
- **`allot_one`**. Not in the list; no share [code].
- **The rough layout** (`grow/draw.rs`). Only axes that grew are drawn. A dead axis's `Pencil` stays held [code].
- **The leaf deposit** (`grow/deposit.rs`). A closed axis bears no leaf (`girth::leaf`'s callers leave it out). With `leafYears = 1` the loop over held axes does not run at all [code]. An axis whose apex ended in the last cycle is still flagged open that cycle, which is why table B counts 50,236 to 285,314 leafed phytomers on dead standing wood [record, code: `working.rs`, `closed`].
- **The light**. `read_light` builds the field from `self.leaves` only (`grow/cycle.rs`). Wood casts no shade, dead or alive [code].
- **`girth::lay`**. Every held axis is visited by `Tree::upward`; a closed one returns 0 at once. Its section is frozen and exact; "Wood is never lost" [code: `girth.rs`].
- **Memory**. At 200 on seed 1001 the dead standing class holds 564.7 MiB of the kept axes' 1,169.6 MiB (48%); on seed 1007, 1,345.8 of 2,989.8 MiB [record: table G].

In the settle (`grow/settle.rs`, `settle_shed`, `settle_lay`):

- Every held phytomer is widened (`Axis::settled`), every held axis assigned (`presence::assign`), then `shed_past` runs its fades over all of them. The kept tree, dead wood included, is then scaled (`geometry::scale`), thickened (`girth::thicken`), placed, and where the species sags, levered and placed again [code]. None of these skips dead wood.

In the drawing:

- The conversion (`crates/telperion-core/src/pipeline/space.rs`, about line 375) takes each axis's phytomers while `p.radius >= least`, the thinnest wood the view draws. It has no test for living or dead [code]. How many dead standing phytomers pass `least` at the product's views is [unknown].
- Dead standing wood is 6,639 m of the 16,598 m of internodes at 200 on seed 1001 (40%), all of it on short axes: the longest dead axis is 2.1 m [record: tables B and H readings].

### 1.4 What the fade multiplies, and who reads it

The site is `shed.rs`, `shed_past`:

```rust
let bearer = axis.origin.parent().map_or(ONE, |p| applied[index[p]]);
let own = bearer.min(q(fade[i]));
axis.vigour *= real(own / bearer);
applied.push(own);
```

- Each kept axis has an applied fade: the least of its own fade and its bearers'. The seed's is 1 and a continuation's is its parent's. Its vigour takes the ratio, so a subtree fades once (fn-359) [code].
- `Shed::applied` is handed on as `Settling::shares` [code].

Readers of the faded vigour and of the share, downstream:

| reader | what it does with it |
|---|---|
| `geometry.rs`, `scale` | `base_scale[i] = inherited × vigour × share(dominance)`; every phytomer's `scale` on the axis is multiplied by it, and every descendant inherits it through its bearer's node or base scale [code] |
| `geometry/lay.rs`, placing | a phytomer's length is its scale times the internode, so lengths carry the fade whole [code; R1-READING.md of fn-364 lists the readers of `scale`] |
| leaf | `girth::leaf` and the drawing read `leaf_area × scale × size`, so leaf area carries the fade whole [code] |
| `girth.rs`, `thicken` | `radius = sqrt(T × share / π)`: the section `T` is kept, the drawn radius carries the square root of the fade [code] |
| `sag.rs`, levers | loads and levers are taken from the scaled, thickened tree [code, not traced line by line] |
| relay joins | `thicken` sets a relay's join to its bearer's faded radius [code] |
| the drawing | the `least` cut reads the faded radius, so faded wood can fall under the thinnest drawn [derived] |

Who does not read it: the grower. Its pencils hold `base_scale = inherited × vigour × share` from the draws alone (`grow/draw.rs`, `pencil`), and the light, the balance, the allotment and the wood laid each year all run on the unfaded tree. On the traced limbs the grower's base scale is 0.789 and 0.948 against a settled vigour of 0.104 and 0.222; the grower's leaf deposit at 300 is 4.08 m² against 0.88 m² settled [record: SURPLUS-PROBE].

Why a living limb fades at all [code, derived]:

- A child's living time enters its bearer's as `weight × (gap + lived)`. For a lateral the weight is `vigour × the bearing node's scale`; for a relay or continuation it is `vigour` (`fades`, the backward pass; `Stand` carries the same product).
- At that point `vigour` is what the birth draws made of the axis (`presence::assign`), and the node's `scale` is `running × node × unit size` (`grow/draw.rs`, `draw_bent`; OLD-OAK-WOOD section J states the same). The unit size is the allotment's, a size, not a presence.
- So a limb whose own apex ended long ago and which lives on through laterals and relays counts their years at less than a year each. Its `lived` lags its age, its idle time passes its delay, and its fade falls to the floor `alive`, the same product of weights down to its most present living apex. The traced limb born at 13 had lived 222 of 287 cycles: idle 65 against a delay of 5 + 1 [record].
- fn-359's spec recorded this and left it open: "Whether an old limb with only small living wood should fade at all is not decided here", and the log adds "the fade weighs a lateral by its node's scale, a size, with its presence" [record: goal log, 12 Nov 16:31].

How wide it is [record]:

- On today's base (V0, seed 1001): the median fade of living axes is 1.00 to age 240 and 0.22 from 260; at 300 all 4,334 living axes are under 0.22 or less.
- With the height charge (V4) at 300: 16,896, 65,409 and 140,950 living axes carry a fade under 1 on three seeds, medians 0.31, 0.31, 0.02. At 40 it is 245 of 26,454.
- Table J: the shed's fade ratios along a leafed phytomer's lineage have a geometric mean of 0.86, 0.77, 0.73, 0.68 at 80, 120, 200, 300 on seed 1001; weighted by leaf area, 0.98 to 0.99.
- On the built fn-363 and fn-364 heads the fade was not measured: their evidence folders hold no count [unknown; checked by search of those folders].

### 1.5 Why the fade exists, and which tests hold it

- **fn-206, by degree.** The module doc: a kept axis "fades with its subtree's living time, each growth unit's time weighted by its presence, so a branch whose last growth a setting is unmaking shrinks away before it is shed; it is dropped once its fade reaches nothing, which for whole presences and delays is the cycle GreenLab drops it in" (`shed.rs`, head). It makes the shedding delay, and every setting that ends a subtree's life, change the tree without a pop [code].
- **The floor.** "A living subtree is never drawn smaller than its living apex" (`fades`, the comment over the last map; host decision 8, D17 in `contract/presence.rs`). The floor follows the living apex's presence so that an apex a setting is unmaking takes its bearers down with it by degree [code].
- **fn-359, once.** Before it, each relay multiplied its own fade into a base scale that already carried its parent's; a line of one lateral and 18 relays ran 0.104, 0.0137, 0.0018, 0.0002, 0 and the oak at 300 was drawn as a bare post. fn-359 made the applied fade the least along the way. It changed neither the law nor what is shed [record: the spec, "The design", "Boundaries"].

Tests that hold it [code; names read, bodies read only for `shed/tests.rs`]:

| test | what it pins |
|---|---|
| `src/shed/tests.rs`, `a_line_idle_past_its_delay_fades_once_to_its_living_apex` | a lateral and three relays, living apex at one eighth: every axis at 0.125, girth share the same |
| `src/shed/tests.rs`, `whole_presences_fade_nothing_kept` | whole presences: every share 1, the tree as grown to the bit |
| `tests/death.rs`, `laterals_without_a_living_apex_are_shed_after_their_delay`, `a_living_relay_keeps_its_parent`, `an_abortion_in_the_last_cycle_fades_by_degree` | the drop cycle; the floor following a living apex's presence |
| `tests/walks.rs`, `the_shedding_delay_changes_the_tree_by_degree_in_leaf`, `a_stop_near_its_bound_fades_nothing_its_relay_carries`; `tests/walk/mod.rs`, the fn-206 setting | the delay walked between whole cycles |
| `tests/closed_form.rs`, `a_fractional_shedding_delay_keeps_by_its_fade` and the delay-0 case near line 473 | the closed form against the grower |
| `src/closed_form/kept.rs`, `Kept::shed`; `predictor.rs`, `fade` | the oracle's own fade, `(delay + 1 − (m − s)).clamp(0, 1)`, a count in expectation (fn-231) |
| `src/grow/tests.rs`, `a_lateral_certain_to_be_shed_is_shed`, `a_tree_grows_the_same_without_what_it_is_certain_to_shed`, `nothing_is_skipped_where_shed_wood_is_read` | `shed.rs`, `lasting` and `Shoot::shed_certain`, the bound from the settings alone |
| `src/grow/cycle/paused/tests.rs`; `src/grow/growing_tests.rs` (line 26 names nested fades on oak 1, beech 1, spruce 1 and 7) | the year, pause, retirement and let-go equalities |

Whether the closed-form oracles model the floor or the size weights, or only whole trees, is [unknown]: I did not read their bodies.

## 2. What the literature says

Method and limits. The research index returned nothing on branch retention; the sources below came from web search and were read as abstracts or as one extracted passage. Quotes were returned by an extraction tool; it gave no page numbers. No source read measures an open-grown crown's dead share. No source read gives a retention time for twigs of any of the four genera. Read every number below with that in mind.

### 2.1 How long dead branches stay on

- **Oak, twigs.** Buck-Sorlin and Bell 1998, "A quantification of shoot shedding in pedunculate oak (Quercus robur L.)", Botanical Journal of the Linnean Society 127: 371 to 391. Abstract: "Almost 37 000 fragments (twigs, peduncles, inflorescences) were shed from a single mature oak tree over one year." "The majority of shed twigs were short shoots, consisting of one or few extension units, which were not older than two years." Shedding peaked in June and October; keywords abscission and cladoptosis. One mature tree, England. [source, abstract only]. The abstract does not say how long a twig stood dead before it fell. Cladoptosis is abscission at a separation layer, so a twig shed this way has no standing dead phase [derived]; the share of the oak's twig loss that goes this way is in the paper's body, not read [unknown].
- **Oak and beech, first-order branches on young stems.** Kint and others 2010, "Modelling self-pruning and branch attributes for young Quercus robur L. and Fagus sylvatica L. trees", Forest Ecology and Management 260. Abstract: "The reported models reveal similar self-pruning rates in oak and beech, but with different driving factors: early branch dying and slow shedding for oak and the other way around for beech." 399 oaks and 376 beeches, 30 plots each, young planted stands in Flanders, branches below 6 m. [source, abstract only]. No years given in the abstract.
- **Beech, branches.** Hein 2008, "Knot attributes and occlusion of naturally pruned branches of Fagus sylvatica", Forest Ecology and Management 256: 2046 to 2057. Abstract: "Larger branches showed a significantly longer occlusion time, a larger occluded branch radius, a steeper insertion angle, and a higher loose knot portion." 717 branches of 41 trees, thinning plots in south-west Germany. [source, abstract and introduction only]. The model of occlusion time against branch diameter is in the body, not read; occlusion time runs from death to the wound closing, so it bounds retention from above [derived].
- **Birch, branches.** Mäkinen 2002, "Effect of stand density on the branch development of silver birch (Betula pendula Roth) in central Finland", Trees 16: 346 to 353. Abstract: "In the lowest part of the crown, the branches remained alive for 3–4 years without forming annual rings." "Branches were occluded 7–10 years after their death. The number of years from branch initiation to occlusion was 20 on plot 400. This period decreased to 13 years on plot 5,000." 18 trees, planted at 400, 1,600 and 5,000 stems per hectare, the thickest branch or knot below the living crown. [source, abstract only]. Again death to occlusion, the thickest first-order branches, in a stand.
- **Spruce.** No measured time was read. Lim and others 2024, "Overlooked branch turnover creates a widespread bias in forest carbon accounting", PNAS: "Dead branches often remain attached to stems for years to decades, undergoing in situ mass loss before falling." and a branch turnover "ranging from 0.7% y−1 in suppressed, shade-tolerant boreal Picea abies, taking 143 y to replace all branches". [source, full text, two passages; the figures' values were not read reliably and are not used]. The statement is general, about branches on stems.
- **Direction by size.** Larger branches take longer (Hein 2008, above). Every preset's rows run the same way, longest on limbs, shortest on short shoots [code, derived].

### 2.2 What share of a crown is dead

- **Attached dead wood by mass, oak forest.** Christensen 1977, "Estimation of standing crop and turnover of dead wood in a Danish oak forest", Oikos 28: 177 to 186. Abstract: "the attached dead wood pool was estimated to 456 g m-2 and standing dead wood averaged 79 g m-2. The pool of attached and standing dead wood amounted to 4.3% of total above-ground woody biomass". A 55-year-old oak forest, a closed stand. [source, abstract only].
- **Recent twig dieback, by crown area.** USDA Forest Service, the Virginia forest inventory report (Treesearch 45260): "Crown dieback is recorded as the percentage of mortality of the terminal portion of branches that are ≤1.0 inch in diameter, and are positioned in the upper portion of the crown". "Crown dieback is considered an indication of recent stress because small dead twigs do not persist for long, and because trees typically replace lost twigs and foliage". "Average crown dieback across all plots was 2.5 percent." "Most hardwoods and softwoods had no crown dieback, 81.4 and 95.2 percent, respectively." Forest plots, all species. [source, full text, one passage; page not returned]. The measure leaves out the lower and inner crown's natural pruning by definition.
- **Open-grown crowns, by twig count or length.** Nothing found [unknown].

### 2.3 Against the presets

- The grower's dead share is counted in stem units: 42 to 52% of phytomers, 40% of internode length at 200 [record]. The two field measures are by mass of a stand (4.3%, dominated by trunks) and by crown area of recent upper-crown dieback (2.5%). Neither is the same quantity, so neither proves the grower's share wrong by its number alone [derived]. What they do show: field crowns are described as carrying few small dead twigs, which "do not persist for long", and the oak's fine shoots leave by abscission within two years of their making. A crown where four in ten metres of wood are dead twigs waiting out a delay is not what these sources describe [derived].
- **Oak, fine rows (shoot, twig, short: 2).** The one source on oak twigs has them shed by abscission, in season. The rows keep each dead short shoot standing for three yearly states. No source read supports 2 [source, abstract only; derived]. The value to put in its place needs the body of Buck-Sorlin and Bell 1998 [unknown].
- **Oak, limb 5, bough 4, branch 3.** The code names these an estimate. Kint's "slow shedding for oak" and Lim's "years to decades" agree in direction; no number read confirms 3 to 5 [source, abstract only; unknown].
- **Beech, 1 on every row.** Kint's beech sheds fast once a branch has died, which agrees in direction for branches. No number read [source, abstract only; unknown].
- **Birch, 3, 2, 2, 1, 1.** Mäkinen's 7 to 10 years from death to occlusion on the thickest branches bounds retention from above and is longer than the limb row's 3; how much of it is the stub closing over is not in the abstract [unknown].
- **Spruce, 3, 1, 1, 3 and one row `never`.** "Years to decades" on stems is longer than 3 for main branches; the preset's own comment gives no source for the delay [code; source, full text, general statement].
- **The coarse multipliers** (0.243, 0.051, 0.65) are fits per preset with no source [code].

## 3. Candidate remedies and what each would break

Owner rules that bind any remedy [record: RULINGS.md]: no value without a source or a listed estimate; nothing per species in core settings ("This shouldn't be per species", 3 Nov); no hand-tuned rule (9 Nov); no artificial limit (3 Nov); "the dead-wood bug is fixed properly" and need not keep bytes (10 Nov); no cull after growth for leaves (3 Oct). The grower's guarantees named in the brief: year-independence to the bit, the closed-form oracles, by-degree walks, "a living subtree is never drawn smaller than its living apex", wood never lost.

### 3.1 Dead standing wood

**D1. Shorter delays on the fine rows, as values.** The rows of the shoot, twig and short ages go toward 0; the parameter already takes any value from 0 and walks by degree.
- Breaks no guarantee by construction: the delay is a walked setting, and delay 0 is tested (`tests/closed_form.rs`, near line 473) [code].
- Needs a source or a listed estimate per value. Today's values are estimates with no source; the abstract read supports "short" for oak twigs, not a number [unknown].
- The rows are species rows. Whether a trait row per species falls under "nothing per species" is the host's reading; the coarse multipliers already are per preset and fitted [code].
- At delay 0 a dead shoot still stands in the settle of the year its last unit grew [derived from the law].
- Moves every digest that holds a tree with fine shed wood.

**D2. One law for the delay across species, from a measured trait** (retention against diameter or wood decay), replacing the rows.
- Breaks the bound from the settings alone: `shed.rs`, `lasting`, `Shoot::horizon_of` and `shed_certain` read one delay per age before anything grows, and the closed form's `Kept::shed` takes one delay per age. A delay that depends on a grown size is not known there [code].
- `retire.rs`, `faded_out` and `evaluate` would need the size at the time of death; year-independence holds only if that size is final when the subtree closes. Girth is frozen on closed wood, so it is [code, derived].
- Needs the law itself. No source read gives one for twigs; Hein 2008's model is for occlusion on stems [unknown].

**D3. No delay: a subtree is shed the cycle it closes; the `shedding` setting is deleted.**
- Removes the walks and tests of the delay (`the_shedding_delay_changes_the_tree_by_degree_in_leaf`, `a_fractional_shedding_delay_keeps_by_its_fade`, `laterals_without_a_living_apex_are_shed_after_their_delay`), `lasting` and `shed_certain` with their tests, and the oracle's fade [code].
- Dead limbs vanish at once, against "slow shedding for oak" and "years to decades" [source, abstract only and one passage].
- A drop with no fade pops in a walk of any setting that ends a subtree's life: by degree is lost unless something else carries it [derived].
- Wood never lost is untouched: the bearer keeps its section [code].
- It removes defect 2 with it, since nothing kept would fade [derived].

**D4. Keep the delay; let go of the records at death, and rebuild the dead twig from its key when a view resolves it** (the 0.2.0 plan's counting, applied to closed wood).
- The grower reads nothing of a closed subtree but its living, which a `Stand` already carries (`retire.rs`, `compact.rs`) [code]. The settle and the drawing need its phytomers, so they must be rebuilt.
- Year-independence and the paused, retired and let-go equalities then rest on the rebuild being exact; fn-357's `Stand` shows the carry survives, the geometry is new work [derived].
- Changes no value and no tree if exact. Saves the grower's memory (48% of kept bytes at 200), not the drawing's load [record, derived].

**D5. Leave the grower alone; the drawing leaves out dead wood it cannot resolve.** The conversion already cuts at `least`.
- A cut by living or dead in the drawing is a cull after growth; the 3 Oct ruling is about leaves, so whether it reaches wood is the host's reading [record].
- The grower's memory and the settle's passes keep the whole load [derived].
- A second rule for what is drawn, beside the fade, risks a copy of a stage outside the pipeline (`docs/principles.md`, the first question) [record: AGENTS.md].

**D6. An abscission event of its own** (a shoot that ends is shed in season, beside the delay for wood that dies otherwise).
- A new parameter must be dormant where absent and act by degree. A delay between 0 and 1 on the same rows already expresses "shed in season" by degree, so this may be D1 under another name [derived].
- Needs values from the body of Buck-Sorlin and Bell 1998 [unknown].

### 3.2 The settle's fade

**F1. Count living time by presence only.** The weights in `fades` (and in `Stand`) leave out size: the unit size inside the node's scale, and anything else that is a size.
- Needs a presence channel beside `scale` on a node, since `scale = running × node × unit size` mixes the two; a field on the growing phytomer, or a recomputation from the draws as section J did [code, record].
- The floor guarantee holds, and becomes exact for whole presences: a whole living apex gives `alive = 1` and no fade [derived].
- By degree holds where presence walks; a size that walks to 0 already shrinks its subtree through `scale` [derived].
- May not cure. The birth draws alone have a geometric mean of 0.70 to 0.78 along a leafed phytomer's lineage and each relay's weight carries its bearer's running presence [record: table J row b]; if those are presences below 1 on the shipped oak, living time still lags. Whether the lag is mostly size or mostly presence is not measured [unknown]. See 4.2.
- The retirement's verdicts use the same carry, so the grower's retired set changes with it; the equalities must be re-proved [code].

**F2. A subtree that holds a living apex does not fade** (fade 1 on open wood; only closed subtrees fade over their delay).
- Breaks by degree at the moment the last apex's presence reaches 0: the bearer jumps from 1 to its delay's share. The floor `alive` exists for exactly that, and `an_abortion_in_the_last_cycle_fades_by_degree` pins it [code].
- Keeps the floor guarantee trivially, and the two `shed/tests.rs` tests change meaning.

**F3. The grower sees the fade.** The year's fade feeds the pencils, so light, balance, wood and drawing agree.
- The fade is read at the age grown and changes each year, so wood already laid would be re-laid smaller: the rough layout is incremental (`draw.rs` lays only what grew), and "wood never lost" then conflicts with a limb drawn smaller than the year before [code, derived].
- Year-independence can hold (the fade at each age is a function of the tree at that age), at the cost of a whole-tree pass a year [derived].
- It makes real the shrinking the host's 13 Nov reading called miniaturisation [record].

**F4. Kept axes are never shrunk; the fade only decides the drop.**
- Breaks the walks over the delay and over every setting that ends a subtree (a pop at the drop), the fractional-delay closed form, and the oracle's count in expectation over the fade (fn-231) [code].
- The floor guarantee holds trivially.

**F5. Fade each idle part, not what lives beyond it.** Living descendants keep their size; only wood with nothing living below it shrinks.
- A dead bearer with living laterals cannot shrink without its laterals standing on thinner, shorter wood: attachment and placing read the bearer's scale [code]. fn-359 lists "a tree drawn larger than its rough layout" as a stop; this would be the reverse case and is untested [record].
- By degree at an apex's vanishing has the same jump as F2 unless the floor stays [derived].

**F6. Cure it upstream: limbs whose living time does not lag.** fn-363 and fn-364 change which tips live.
- Measured on the probe variants it does not go away: tens of thousands of living axes faded under 1 at 300 with the height charge [record]. On the built heads it is [unknown].

**F7. Remove the delay (D3).** No kept axis fades because none is kept past its life. See D3 for what it breaks.

Interaction [derived]: every remedy for defect 1 that shortens or removes the delay also moves defect 2, because the fade's clamp starts once idle time passes the delay. A shorter delay makes living limbs reach the floor sooner; D3 removes the fade.

## 4. What a cheap measurement would tell before any build

Scratch tooling exists under `/home/daniel/.cache/claude-tmp/claude-1000/-home-daniel-Projects-telperion/d6d5cb56-440a-44f4-803a-6a595b8a3532/scratchpad/`. All of it was written against `census-old` at `d9228bdd8` and would need porting to the fn-364 head [record].

### 4.1 Dead standing wood

1. **Does the living tree change if the fine delays go to 0?** A value-table trial, no code: the oak's rows 12 to 14 at 0, seeds 1001 and 1007 at 80 to 300. Expected from the code: nothing in the grower reads closed wood but its carry into its bearer's living, which a `Stand` keeps, so the living axes should be the same and only the settled output shrinks [derived]. The reader `oldoak/oldoak-final.patch` (tables A, B, G; `oldoak/assemble.py`) gives phytomers, bytes and the class split; a digest over the living axes would confirm or refute the expectation. One caveat to watch: a shed subtree that holds a sleeping bud is not closed in the grower, yet the settle drops it by its fade; what happens to that bud is [unknown].
2. **How much of it reaches the drawing?** Add one count to the same reader: dead standing phytomers with `radius >= least` at the product's views. If few pass, the load is the grower's and the settle's, and D4 addresses it; if most pass, the drawing carries it too.
3. **The share by wood volume**, `Σ section × length` by class, to set beside Christensen's 4.3% by mass. Table B has length, not volume.
4. **The other presets.** Table B exists for the oak only. `HELD-VS-KEPT.md` has the beech and spruce held against kept; its class split was not read for this file [unknown].
5. **What the coarse dial already shows.** The coarse rows cut the oak's delays to about a quarter at the coarsest fidelity; fn-328's own records hold trees grown that way [code; records not read].

### 4.2 The settle's fade

1. **Is it still there on the built head, and on which presets?** `surplus/extras.patch` prints the living axes with an applied fade under 1 and their median. Run on fn-364 for the oak at 120, 200, 300 and once each for beech, birch and spruce.
2. **Is the lag size or presence?** `surplus/trace.patch` prints, for each traced tip, every ancestor's `lived`, idle time and `alive` (`TR|tip`, `TR|why`). Extend it with section J's factor split (`oldoak/jtables.py`, the reader's J recomputation): the same carry computed twice, once with today's weights and once with the unit size left out. If the second gives fade 1 on the traced limbs, F1 cures; if not, the lag is in the presences and F1 does not.
3. **What the unfaded settle looks like.** The grower's own layout is that tree: its leaf deposit and longest path are already printed beside the settled ones (4.08 against 0.88 m² at 300 on V0; 2 to 7% apart on V4 and V8) [record]. No new run is needed to size the effect; a still of the grower's layout beside the settled tree would show it.
4. **Where the fades sit.** Of the living axes faded under 1, the share whose fade is at the floor (`alive`) against in the delay's last cycle, by PA row. The reader's fade classes (table B's columns) do this for dead wood only.

## Not checked

- The bodies of every paper cited as abstract only; page numbers of the two passages.
- The closed-form oracles' bodies, `sag.rs` line by line, and tables D, F, H, I of OLD-OAK-WOOD.
- Any number on the fn-363 or fn-364 built heads.
- `HELD-VS-KEPT.md` beyond its opening.
