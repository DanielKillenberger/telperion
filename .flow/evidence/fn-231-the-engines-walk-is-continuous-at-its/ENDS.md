# fn-231: the engine's walk at its ends (R1)

These measurements come from `target/ci/examples/walk_ends [40|80]` (`crates/telperion-core/examples/walk_ends.rs`), run on the raw `telperion_space::blend` before fn-230's size factor, light-neutral. The full output, with every jumping row, is in `raw/before-40.txt` and `raw/before-80.txt` (gitignored). Each row gives the predicted kept phytomers (`expected_kept`), the grown ones (`expected_counts`, shed included), and **wood**. Wood is `expected_length`: the expected metres of wood, where each phytomer counts as its internode times the size of the unit it stands on. That makes wood the size-weighted mass. The seconds are the time the three predictions take together, which is the R5 cost before any change.

## Result: the guess is wrong

**The counts jump at the ends, but the size-weighted mass does not.** In every pair, at both ages, wood a millionth off an end is within 1e-5 of the end's wood. The kept and grown counts are 1.3 to 85 times the end's.

**The newly lived ages are not the cause.** For each end, I put back to unlived every age the walk starts living in (lifespan back to 0, continuation 1). The counts barely move: beech to oak at 40, 1e-6, keeps 1,652,207 against 1,652,214, and at 80 it keeps 108,547,640 against 108,547,668. I also put back to the end's value every row that moves off the end by more than its share. Those rows are the relay shares that host decision 12 mixes as outcomes. Putting them back changes nothing either.

**What the cause is.** I checked it by putting one row of the near point into the exact end, one row at a time (the probe's source is `walk_probe.rs.txt`). Every row that moves the count is a row whose value leaves a whole number, a grid point or zero. A count changes when the row only just leaves that value:

| Pair, end, age 40 | Row (age) | Row moves | Kept, end → with that row alone |
|---|---|---|---|
| beech → oak, beech | `zone.buds` (4, 6, 7, 8, 9) | 1 → 1.000002 | 147,362 → 174,102 to 287,808 each |
| beech → oak, beech | `lifespan` (9) | 10 → 10.0000018 | 147,362 → 155,657 |
| beech → oak, beech | `shedding` (12, 14) | 1 → 1.0000007 | 147,362 → 150,054, 165,860 |
| beech → spruce, beech | `zone.buds` (zones 1 and 2, ages 4 to 9) | 1 → 1.000001 | → 168,540 to 287,808 |
| beech → spruce, beech | `lifespan` (10, a newly lived age) | 0 → 2.4e-6 | → 148,979 (the only lived-age effect, 1 %) |
| beech → oak, oak | `lifespan` (2, 3, 8, 9) | 12 → 11.9999966 and others | 457,386 → 616,920, 480,788, … |
| oak → spruce, spruce | `lifespan` (0, 1, 8, 10, 12) | 2 → 2.0000015 and others | 1,149,893 → up to 1,546,303 |
| oak → spruce, spruce | `zone.buds` (8, 9) | 2 → 2.000001 | → 1,216,129, 1,292,692 |
| oak → ordinary, ordinary | `zone.buds` (7, 8, 9) | 2 → 2.000001 | 394,267 → up to 631,503 |

This is the engine's rule for an element a setting has not finished drawing (`closed_form/events.rs`: "every bud place grows, the last a fraction's at its share of the size: counted whole, sized by the mean share"). The same rule covers a partial unit when a lifespan leaves a whole number, and a sleeping bud's stage. Phase B built every element to grow in from nothing **by its size**, so a tree changes by degree in its geometry. The element's phytomers are still grown whole. So a bud count of 1.000002 grows a whole second lateral, with its whole subtree, at about 2e-6 of its size. The phytomers jump; the wood does not.

The rows look continuous at the ends because they are: every row's value moves O(t), apart from the relay shares, which change no count. The jump comes from the engine's response to a row that leaves a whole value, not from `blend`.

## A second finding: a walk off the ordinary tree is refused

A walk with the ordinary tree at one end is refused at t = 1e-6 at its other end: `states[0].lifespan` (and `states[3]`) "is none or at least half a tick, 2^-21 cycle". An age only the ordinary tree lives in mixes on `Log { floor: 1 }` to (L+1)^t - 1. That is about 1e-6 × ln(L+1), below half a tick when L is small. Inside 1e-6 to about 5e-6, beech, oak and spruce to the ordinary tree have no tree at all. R3's t = 1e-6 cannot pass there whatever the fade-in rule.

## The measurement

Kept / grown / wood (m), with the seconds for the three. Where t = 1e-6 is refused, the first point off the end is 1e-3.

**At 40**

| Pair | t = 0 | 1e-6 | 1e-3 | 0.01 | 0.99 | 1 - 1e-3 | 1 - 1e-6 | 1 |
|---|---|---|---|---|---|---|---|---|
| beech → oak | 147k / 346k / 4,040 (0.06) | 1.65M / 2.73M / 4,040 (0.23) | 1.65M / 2.72M / 4,052 | 1.61M / 2.67M / 4,152 | 635k / 1.43M / 8,256 | 630k / 1.42M / 8,387 | 630k / 1.42M / 8,402 (0.12) | 457k / 1.06M / 8,402 (0.05) |
| beech → spruce | 147k / 346k / 4,040 | 2.89M / 4.85M / 4,040 (0.10) | 2.89M / 4.85M / 4,064 | 2.87M / 4.84M / 4,275 | 1.97M / 4.82M / 55,624 | 1.82M / 4.50M / 52,764 | 1.80M / 4.46M / 52,439 (0.14) | 1.15M / 2.75M / 52,439 (0.05) |
| beech → ordinary | 147k / 346k / 4,040 | refused | 2.88M / 4.84M / 4,048 (0.34) | 2.82M / 4.75M / 4,114 | 403k / 994k / 5,481 | 395k / 978k / 5,459 | 394k / 976k / 5,456 (0.46) | 394k / 976k / 5,456 (0.47) |
| oak → spruce | 457k / 1.06M / 8,402 | 2.00M / 3.58M / 8,402 (0.19) | 2.00M / 3.58M / 8,390 | 1.98M / 3.55M / 8,279 | 2.17M / 5.33M / 51,965 | 2.14M / 5.28M / 52,394 | 2.14M / 5.28M / 52,439 (0.42) | 1.15M / 2.75M / 52,439 |
| oak → ordinary | 457k / 1.06M / 8,402 | refused | 1.06M / 2.31M / 8,383 (0.26) | 1.05M / 2.28M / 8,219 | 750k / 1.72M / 5,420 | 753k / 1.73M / 5,452 | 753k / 1.73M / 5,456 (0.45) | 394k / 976k / 5,456 |
| spruce → ordinary | 1.15M / 2.75M / 52,439 | refused | 1.81M / 4.47M / 52,467 (0.55) | 1.85M / 4.53M / 52,707 | 526k / 1.28M / 5,687 | 514k / 1.25M / 5,479 | 513k / 1.25M / 5,456 (0.43) | 394k / 976k / 5,456 |

**At 80**

| Pair | t = 0 | 1e-6 | 1e-3 | 0.01 | 0.99 | 1 - 1e-3 | 1 - 1e-6 | 1 |
|---|---|---|---|---|---|---|---|---|
| beech → oak | 2.19M / 9.80M / 106,325 (0.58) | 108.5M / 289.0M / 106,325 (4.8) | 107.9M / 287.4M / 106,581 (8.0) | 102.6M / 273.9M / 108,862 (7.5) | 6.02M / 25.1M / 137,772 (2.7) | 5.85M / 24.6M / 140,225 | 5.83M / 24.6M / 140,507 (2.9) | 4.43M / 18.6M / 140,507 (0.85) |
| beech → spruce | 2.19M / 9.80M / 106,325 (0.97) | 186.2M / 513.2M / 106,325 (0.79) | 185.2M / 510.8M / 107,016 | 176.5M / 490.1M / 113,261 | 7.35M / 32.0M / 379,914 | 6.64M / 29.2M / 354,844 | 6.56M / 28.8M / 352,024 (0.75) | 4.65M / 18.9M / 352,021 (0.21) |
| beech → ordinary | 2.19M / 9.80M / 106,325 | refused | 185.3M / 510.8M / 106,556 (5.6) | 177.2M / 489.8M / 108,621 | 3.73M / 13.5M / 68,803 (6.7) | 3.58M / 13.0M / 67,403 | 3.56M / 12.98M / 67,251 (6.6) | 3.56M / 12.98M / 67,251 (6.7) |
| oak → spruce | 4.43M / 18.6M / 140,507 (0.79) | 27.7M / 94.2M / 140,507 (1.4) | 27.6M / 93.9M / 140,605 | 27.0M / 91.8M / 141,453 | 7.96M / 34.7M / 348,968 (7.5) | 7.85M / 34.2M / 351,731 | 7.83M / 34.1M / 352,021 (7.5) | 4.65M / 18.9M / 352,021 |
| oak → ordinary | 4.43M / 18.6M / 140,507 | refused | 11.9M / 49.6M / 140,240 (3.6) | 11.7M / 48.8M / 137,882 | 11.3M / 35.6M / 67,216 (6.6) | 11.5M / 35.9M / 67,244 | 11.5M / 35.9M / 67,251 (6.6) | 3.56M / 12.98M / 67,251 (6.6) |
| spruce → ordinary | 4.65M / 18.9M / 352,021 (0.21) | refused | 6.66M / 28.9M / 352,308 (8.5) | 6.77M / 29.5M / 354,767 | 5.47M / 19.3M / 69,342 (6.6) | 5.49M / 19.2M / 67,463 | 5.49M / 19.2M / 67,251 (6.6) | 3.56M / 12.98M / 67,251 (6.5) |

Beech to spruce at 80 a millionth off the beech predicts 186M kept and 513M grown phytomers, against the beech's 2.19M and 9.8M. That is far over the engine's 20M grown budget, for a tree whose wood is the beech's to six digits. Beech → ordinary is continuous at the ordinary end. Its whole-valued rows that differ from the beech's move down from the ordinary tree's value (buds 2 toward 1): a bud count just under 2 still grows two places, so no place is added. The oak's and the spruce's walks to the ordinary tree move its `zone.buds` up (2 → 2.000001) and jump.

## The fade-in rule: not stated (BLOCKED, the host decides)

The task asked for a lived-age fade-in, and the measurement rules it out. Fading the newly lived ages in moves the counts by under 1 % (beech → spruce at 40: 2.87M against 2.89M). The count jump belongs to the engine's way of building a fractional element: grown whole, sized by its share. Making counts continuous means choosing between ways of building, which the dispatch reserves for the host. Here are the candidate ways, unweighed:

- **The engine counts a fractional element at its share.** A fractional bud place, partial unit or stage grows its phytomers in by count, not only by size: drawn by chance at its share, or thinned. This changes the engine's phase-B growth model for every point with a fractional row. Whether a shipped species' grown tree moves is unknown, not checked: it moves only if a shipped species carries a fractional bud count, lifespan or stage.
- **The walk mixes whole-valued rows as whole numbers** (`Scale::Whole`, as fn-223 does for clusters). The count steps at a half instead of at the ends, and a visible step at the half is the pop R2 forbids.
- **Continuity is judged by size, not count.** R3 would test the size-weighted mass (continuous today) and the geometry, not the raw counts. The engine's budget would still see 513M grown at beech → spruce 1e-6, at 80.

Whichever way is chosen also has to settle the walk off the ordinary tree, which is refused near t = 0 (see the second finding above).

## Host decision (2026-10-08)

**Way 1, in this form: a fractional element is present at its share, and when present it is whole.**

- **The rule.** Where a row's value leaves a whole number (`zone.buds`, `lifespan`, `shedding` and any other row the measurement names), the last element is present with probability equal to the fraction and grows at full size when present. The draw is keyed by its own lineage through the lineage key registry: a new key, mirrored in WGSL with its distinctness and mirror tests. The "counted whole, sized by the mean share" rule in `closed_form/events.rs` and the grower's matching rule are replaced, not kept beside it.
- **The predictor.** The closed form counts the element at its share (weight × fraction) and sizes it whole. Expected kept and grown counts, and the expected mass, are then continuous in every such row.
- **Why this way.** It is the botanical reading: a zone of 1.5 buds per node has half its nodes bearing two buds, not every node bearing a second, half-sized bud. It changes the tree by degree in expectation, it removes the burst of tiny subtrees rather than hiding it (way 3), and it does not move the step to the half (way 2).
- **Lifespan below a tick.** The same rule covers walks off the ordinary tree. A lifespan of 0.3 ticks lives one tick with probability 0.3. Validation's half-tick floor drops to "above 0", and t = 1e-6 is then valid on every pair. The floor's tests are updated to the new floor; a lifespan of 0 or less is still refused by name.
- **Shipped species.** Any shipped species with a fractional value in these rows changes. Under the generator-evolution policy (AGENTS.md), measure `space_hash`, node counts and cost for beech, spruce, oak, palm and birch, at 40 and 80, seeds 1 and 7. For every species that changed, make strips before and after, near and whole tree, and view them. Accept the change if no regression is visible, re-pin with that evidence, and list each change in the task's report. R4's "byte-identical" is read as: identical where no row is fractional.
- **R3.** Unchanged, at both ends, on raw counts, and at 40 and 80. Per seed, a realized tree may gain one whole element where its lineage draw crosses the fraction. That is a step in one element, not a burst. The geometry degree test bounds it, and the strips judge it.

## The rule built for bud places (2026-10-08, parked on `fn-231-places-wip`)

I built the host's rule for `zone.buds` on both executors and the closed form, at commit `df5e5445` on branch `fn-231-places-wip`, off the task branch:

- A new `PLACED` key (`u64::MAX - 14`, under a place), mirrored in `lineage.wgsl`. The last place stands when its draw is under its fraction, and is then whole.
- The CPU grower (`grow/unit.rs`, `grow/wake.rs`) and the chain kernel (`apex.wgsl`, `sleep.wgsl`, and `zones.wgsl`'s `stands`) draw it. `assign.wgsl` no longer scales a lateral's presence by its share.
- The closed form counts the place at its share and sizes it whole: `events.rs`, the predictor's generating function (`axis.rs`, factor 1 + s(φ − 1) for the last place), and the kept tables (`path.rs`, `section.rs::zoned`).

**Executors.** `every_chain_grows_on_the_gpu_as_on_the_cpu` passes with a new case, "beech, fractional buds" (buds × 1.3 + 0.2), at seeds 1 and 7: 8,026 and 10,629 chains, axis for axis. `the_wgsl_keys_are_the_cpus` passes. Both executors apply the rule identically.

**Counts.** At 40, a millionth off the beech, beech → oak keeps 182,593, down from 1,652,214, against the beech's 147,362. Beech → spruce keeps 184,175, down from 2,890,685. Every walk is continuous at the ordinary tree's end: the oak's at 1 − 1e-6 keeps 238,256 against 238,119. The lifespan rows' share remains: the oak end keeps 629,627 against 457,386, and oak → spruce at the oak end keeps 694,635 against 457,386 (`raw/places-40.txt`). The ordinary tree changes: it keeps 238,119 at 40, down from 394,267, with its wood unchanged at 5,456 m.

**What fails: fn-192's by-degree walks.** On the space crate's suite, `every_setting_changes_the_tree_by_degree` and `branches_are_made_and_unmade_at_vanishing_size` fail on every `zone.buds` walk. At a fixed seed, a place that starts to stand arrives whole. That is a jump the refinement does not shrink, 3 to 19 % of the tree's measure (`states[0].zones[1].buds` seed 1 at 1.28: 0.189 at every refinement), and the wood at the crossing is 0.009 of the tree, against the bound of 1e-6. The host decision expected the geometry degree test to bound this, but the test refuses it. `a_fractional_bud_place_is_counted_whole` (`tests/closed_form.rs`) fails as expected, because it pins the old rule.

## Questions for the host before the rest is built

1. **Whole, or grown in?** Under "whole when present", fn-192 R2 and R4 ("a branch a setting makes enters at vanishing size", "every setting changes the tree by degree") no longer hold for `zone.buds`. One reading keeps both. The place draw becomes one more making draw, like a lateral's (host decision 8): it stands by chance, so it is counted at its share, and its presence is its draw's lead over a window. A realized tree then grows the element in from nothing as the fraction passes its draw. On the GPU this adds a presence to the lateral and woken asks (`assign.wgsl`) and needs the place's key kept for the woken ask. Which reading should be built? If whole, the two fn-192 tests need an exception for these rows, which only the owner can grant.
2. **Lifespan: the unit or the tick?** The decision gives both. "The last element is present with probability equal to the fraction" makes a cycle the element: a lifespan of 10.3 lives 10 or 11 whole units. "A lifespan of 0.3 ticks lives one tick with probability 0.3" makes a tick the element. The tick reading removes the half-tick refusal, but not the jump: 10 + 1e-6 still grows a partial unit, about 2 ticks, counted whole. The unit reading removes the jump, but it replaces host decision 7, under which an age's last unit is partial and its successor grows the rest of the cycle. It also needs answers to three things:
   - **(a)** What an apex that draws no unit (L < 1) does. It could take its age's end at once, continuation or relay, as the limit of a tiny lifespan does today. Or it could pass through the age as an unlived one does.
   - **(b)** Whether a relay shares its axis's draw. It can, by keying the draw under the axis lineage, which a relay keeps (`make_relay`).
   - **(c)** How a sleeping bud that ages through stages (`dormant.rs`, `slept < state.life()`) reads a drawn lifespan.
   The closed form then mixes each age's two lives at every bud's entry, through every reader of `state.life()`: 43 sites in the space crate, plus `apex.wgsl` and `carry.wgsl`.
3. **Shedding.** A fractional delay is a fade on the device (`fades.wgsl`), not a count: a lateral shed after 1 + ε cycles is faded ε, not gone. Under the rule, does it stay a whole cycle more with chance ε, or does the fade stay and only the kept prediction's `ceil` (`kept.rs`) change to an expectation?

## Host answers (2026-10-08, second)

One principle covers all three: **every fractional element is a making draw (host decision 8).** The executors make it by its draw, and it grows in from vanishing size as the fraction passes the draw's lead. The predictor counts it at its share. No element is ever "whole when present", and fn-192's two tests stand unchanged; there is no owner exception.

1. **Bud places: grown in.** The place draw is one more making draw: present by its draw's lead over the window, counted at its share. This includes the presence on the GPU's lateral and woken asks (`assign.wgsl`), with the place key kept for the woken ask. The parked `fn-231-places-wip` is reworked to this; nothing "whole" stays.
2. **Lifespan: the tick, by degree. Host decision 7 stands.** The age's last unit stays partial, and its successor grows the rest of the cycle. The unit reading is not taken, so (a), (b) and (c) do not arise. The jump that remains comes from the partial unit's last tick, which is counted whole. That tick becomes a making draw: its growth is present by its draw's lead and grows in from nothing, and the predictor counts it at its share. Validation's floor is "above 0", per the first decision. If the tick is not where the count jumps (measure first), stop and report.
3. **Shedding: the fade stays.** The device's fade is already by degree. Only the kept prediction's `ceil` (`kept.rs`) becomes an expectation, matching what the executors do.

Re-measure every shipped species as decided before. The ordinary tree's drop (394,267 to 238,119 at 40 with its wood unchanged) is expected: its fractional places were growing whole subtrees at tiny sizes. Make its strips before and after; if no regression is visible, accept it.

## Built on the second answers, and the lifespan measured (2026-10-08)

**Places (answer 1), built at `cee0782c`.** The last place stands by its `PLACED` draw. Its presence is its lead under the share's threshold, over the bud's window (`lineage::place_presence`), multiplied into the lateral's making presence on the CPU grower and in the GPU's lateral ask (`assign.wgsl`). For a woken bud it is taken at sleep time over the sleeping window (`dormant::draw`; `sleep.wgsl` stores it in the sleeper and pend records, and `made_q` multiplies it into B0). Nothing is whole. The closed form counts the place at its share.
- `every_root_grows_whole_on_the_gpu_as_on_the_cpu`, which compares presences, passes with two new cases, "beech, fractional buds" and "oak, unlit, fractional buds", whose sleeping buds wake on fractional places.
- The chain parity test and the key mirror pass.
- fn-192's two walks pass on every `zone.buds` setting, unchanged.
- `a_fractional_bud_place_is_counted_at_its_share` replaces the old pin. It checks the count at buds 1.3 is three tenths of the way from 1 to 2, and that the grower's mean over 20,000 seeds matches.

**Shedding (answer 3), built at `5482d7ca`.** `Kept::shed` and the predictor's `fade` count a fading subtree at its fade, not whole while any fade is left. Whole delays are unchanged to the bit. `a_fractional_shedding_delay_keeps_by_its_fade` checks that the beech at 20 with its delays plus 1e-6 keeps within 1e-5 of the beech; under the old rule it kept a whole cycle's worth.

**After both, at 40** (`raw/places-shed-40.txt`): a millionth off the beech, beech → oak keeps 156,002 (the beech: 147,362) and beech → spruce keeps 157,546. Every walk is continuous at the ordinary tree's end. What remains is at the oak and spruce ends: beech → oak at 1 − 1e-6 keeps 629,579 against the oak's 457,386, oak → spruce at 1e-6 keeps 610,032, and beech → spruce at 1 − 1e-6 keeps 1,802,504 against 1,149,893.

**The lifespan, measured (answer 2): the jump is not in the last tick.** I moved one age's lifespan off a whole number tick by tick, at 40 (`raw/ticks-40.txt`; source `tick_probe.rs.txt`):

| Age moved | 0 ticks | 1 tick | 2 | 3 | 16 | 1,024 | 65,536 | 524,288 (half a cycle) |
|---|---|---|---|---|---|---|---|---|
| beech age 9, 10 up | 147,362 | 155,657 | 155,657 | 155,657 | 155,657 | 155,656 | 155,630 | 155,442 |
| oak age 2, 12 down | 457,386 | 616,920 | 616,920 | 616,920 | 616,920 | 616,920 | 616,920 | 616,920 |
| oak age 8, 10 up | 457,386 | 462,416 | 462,416 | 462,416 | 462,416 | 462,415 | 462,364 | 462,011 |
| spruce age 12, 2 up | 1,149,893 | 1,546,303 | 1,546,303 | 1,546,303 | 1,546,302 | 1,546,263 | 1,543,725 | 1,526,146 |

(kept phytomers; grown counts move alike; wood moves by degree throughout: 8,401.8, 8,401.8, … 8,745.8 m for the oak's age 2.)

The whole jump arrives with the **first** tick and does not grow with the share. A partial unit of one tick and one of half a cycle count the same. The element counted whole is the partial unit itself, at any share, as the last age's partial unit when the lifespan rises past a whole number, or as the successor's partial first unit when it falls below one. It is not a sub-tick or last-tick remainder. A lifespan on the tick grid has no fractional tick, so a draw on its last tick has a chance of 1 and would change no count. As answer 2 instructs, I stop here.

**For the host:** the element that would have to be the making draw is the partial unit of share s (s of a cycle, ticks over 2^20): it stands at chance s, grows in from its lead, and is counted at s. Decision 7 then keeps its timing (the successor grows the rest of the cycle), but the unit's size when it stands needs a ruling:
- At its share, as now. The expected size becomes s² and the wood is no longer linear in the lifespan.
- Whole. The expected size stays s, and the fade comes from the lead.

The half-tick floor ("above 0") rides on the same choice, because a lifespan below a tick is a partial unit of share under 2^-20.

## Host answer (2026-10-08, third): a partial unit lays its share of the nodes

Neither offered option is taken. Sizing at share gives s², and "full size" turns a partial unit into a whole one; neither is what decision 7 means. The count jump has a cause: a partial unit of any share lays all of its unit's nodes, crowded into its share of the length. So the fractional element is the node, not the unit.

- **The rule.** A partial unit of share s lays the nodes of its unit's first share s, at the unit's own spacing. The whole nodes below s·n are laid. The last, fractional node is a making draw (decision 8): it grows in from its draw's lead and the predictor counts it at its share. The successor still grows the rest of the cycle, as decision 7 says.
- **What follows.** Wood and counts are both linear in the lifespan, and nothing is laid whole. At s = 1 nothing changes. The floor stays "above 0": a share below 2^-20 lays its first node with that chance.
- **Measure first, then build.** Confirm that a partial unit today lays all n nodes. If the count jump does not come from crowded nodes, stop and report.
- **Then finish the task.** Build the rule on both executors, then re-measure the shipped species. Any species whose partial units change must have its strips made before and after, viewed, and re-pinned with that evidence. Then R3 at both ends, the gate, npm test, the review and `flowctl done`.

## The third answer measured: the nodes are crowded (2026-10-08)

**Confirmed.** A probe (`crowd_probe.rs.txt`) grows an age of 4 nodes a unit at lifespan 2 + x into an age of lifespan 10, to 5 cycles, on the CPU grower:

| x | PA 0 phytomers (cycle 3's scale) | Closed form PA 0 |
|---|---|---|
| 0 | 8 | 8 |
| 1 tick | 12 (four at scale 0.000) | 12 |
| 0.25 | 12 (four at 0.250) | 12 |
| 0.5 | 12 (four at 0.500) | 12 |

The successor's first unit lays all four of its nodes at 0.75 and 0.5. A partial unit lays every node of its unit, each at the unit's share of its length, and the count jump is those crowded nodes. As the answer instructs, the build comes next. Two things decide its shape, and both are the host's:

1. **"The unit's first share s", across its zones or within each?** Read as written, the cut falls along the whole unit, so its first share s may hold the bare and medial zones and none of the top whorl. In the grower that is plain: lay the first ⌊sN⌋ of the unit's N nodes in `ALONG` order, and draw the next. In the closed form it is not. Which zone each laid node falls in depends on every zone's count together. The predictor's factor for a unit (`predictor/axis.rs`, `kept/path.rs`, `section.rs`) then becomes a sum over the joint law of up to four zones' counts, conditioned on their total. That is about 10^5 to 10^6 terms a unit by dynamic programming, inside the recursion every blend solve runs, where fn-230 already measured 50 to 60 s a solve. The within-zone reading, where each zone lays its first share s of its own nodes, gives every zone a thinned count law: exact, as cheap as today, and linear in s. But it takes a share of the top whorl too, which a shoot cut short would not.
2. **Every partial unit.** The rule as stated covers all partial units, not just the last unit of a fractional lifespan: a failed unit's lived part (fn-199; `lived_extent` scales its nodes today), a relay's rest of a cycle, and a woken bud's first unit. The beech, oak and spruce all have such units (viability below 1, sleeping buds), so every shipped species' grown tree changes. The strips cover that, as the answer says. Should the build take all of them, or only the units a lifespan's fraction makes?

## Host answer (2026-10-08, fourth)

1. **The cut is per zone.** Each zone lays its own first share s of its nodes, at the unit's spacing. Its last, fractional node is a making draw that grows in. This is exact, as cheap as today, and linear in s. It also takes a share of the top whorl that a shortened shoot would not take. I accept that as a difference of degree: one rule, nothing whole, and no joint sum inside every blend solve (docs/principles.md, "Question, delete, then optimise").
2. **Scope: the partial units a fractional lifespan makes, at both ends of the cut.** This covers the age's last unit, of share s, and the successor's rest of the cycle, of share 1 − s, which crowds in the same way as s → 1. Both are needed for continuity in the lifespan row.
   - Failed units' lived parts (fn-199), relays from other causes and woken buds' first units are left as they are. Their shares come from draws, not from a row, so they cause no jump in any setting, and changing them would move every shipped species' tree before the release with no requirement asking for it.
   - Laying a share of the nodes in those units too is a separate botanical improvement. The host captures it as a post-release spec. It is not part of this task.
   - If the successor's rest of the cycle cannot be told apart from a relay's rest that arises from another cause, stop and report.

## The fourth answer built, and what the tick grid does to it (2026-10-08)

**Built, at `4d0a5d7e` on `fn-231-cut-wip`** (parked off the task branch for the reason below):
- A unit a lifespan's fraction cuts lays each zone's first ⌊s n⌋ nodes by index, at full spacing. The next node is a `LAID` draw (`u64::MAX - 15`, under the node, mirrored in WGSL) that grows in from its lead over the node window (`lineage::laid`; `apex.wgsl`).
- Which units are cut: the age's last unit, when it ends inside its cycle's room, and a successor's first unit when that successor is carried on in the cycle its age before ended in. The flags are `Draws::carried` and `IS_CARRIED`. A woken bud's growing axis is marked (`Draws::woke`, `IS_WOKE`) and never cut. Failed units' lived parts and every other relay are unchanged.
- A unit's time share and its nodes' size are now separate fields (`Unit::size`, `U_LAID`), so the cut unit's nodes are whole while its time stays partial.
- The closed form lays the thinned count law (`Count::laid_pmf`) in the events, the predictor and the kept tables. An age the fraction ends inside its cycle ends its axis that share into it, in expectation over that cycle and the one before (`Step::ends_at`). Without that last piece, the kept count still jumped at the spruce end (1,149,893 → 1,278,581 at one tick), because the end was read as a whole cycle.
- The half-tick floor is gone: a lifespan above 0 grows, a unit cut to none lays nothing, and a negative lifespan is refused by name (`fine/tests/ticks.rs`).

**Checked:**
- **Lifespan tick probe at 40.** The beech age 9 moves 147,362, 147,362, 147,362 … 150,859 at half a cycle. The oak age 2 moves 457,386 … 477,550 and the spruce age 12 moves 1,149,893, 1,149,893 … 1,399,161, both by degree.
- **Crowd probe.** At lifespan 2.25 and 2.5 the age lays 9 and 10 phytomers, all whole, and the closed form agrees.
- **Walk ends at 40 are continuous in every pair** on kept, grown and wood (`raw/cut-40.txt`). The ordinary tree's walks now build at t = 1e-6.
- **GPU against CPU.** Chain and realized parity pass, presences included, with a new "beech, sub-tick ages" case: unlived ages at a tenth of a tick.
- **Space suite.** It passes but for one test.

**What fails: `branches_are_made_and_unmade_at_vanishing_size` on three lifespan walks** (`cross_probe.rs.txt`):

| Walk | Seed 1 | Seed 2 | Seed 3 |
|---|---|---|---|
| `states[0].lifespan` | 6.9e-7 | 8.9e-7 | 2.4e-7 |
| `states[1].lifespan` | 7.8e-7 | **1.9e-6** | **2.3e-6** |
| `states[2].lifespan` | 0 | 3.5e-7 | **2.5e-6** |

(The wood of the branch a walk makes, at the bisected crossing, as a share of the tree; the bound is 1e-6.)

**The cause is the tick grid.** A lifespan moves in ticks, 2^-20 of a cycle, however finely the walk bisects. Under the crowded rule, the first tick a unit exists sized its nodes at one tick, about 9.5e-7, which is why the bound is 1e-6. Under the cut rule, one tick moves the laid fraction by n·2^-20. The node grows in by the log-odds lead of a threshold that just passed its draw, over the node window, and one tick of that is about 1e-4 of presence. In seed 2's case, the node made at the crossing stands at 1.5e-4 and bears a 12-phytomer lateral, 1.9e-6 of the tree.

**For the host:** one way keeps fn-192 unchanged. The lifespan's part below the tick grid would set the laid fraction finely, while time stays on ticks. That means:
- A Q32 sub-tick word per PA beside its ticks.
- `life()` floored rather than rounded. Whole lifespans are unchanged.
- The fraction computed in integers on both executors.
- The successor's rest carrying its predecessor's sub-tick.

This is FINE-CONTRACT D7's grid extended by 32 bits for one reading, the laid fraction. Alternatively the owner bounds lifespan walks at the tick's resolution. Which should be built?

## Host answer (2026-10-08, fifth): lifespan crossings are held at the tick's resolution

No sub-tick word. A 32-bit fraction per age, carried on both executors through the successor, would be new machinery in contract A only to move a made branch from 2.5e-6 of the tree to below 1e-6. Both are invisible, and the walk is already continuous in counts, mass and geometry (docs/principles.md, "Question, delete, then optimise").

- **The rule.** `branches_are_made_and_unmade_at_vanishing_size` holds a walk's crossing at 1e-6 because its bisection reaches the crossing. A lifespan cannot be bisected below one tick (2^-20 of a cycle), and the test's 1e-6 was itself one tick's size under the old rule. A lifespan walk is held at what one tick can make instead.
- **In code.** It becomes a named list beside `ROUGH_LAYOUT` (e.g. `TICK_RESOLUTION`), with its derivation in the doc comment, as for the apical-control pair. It names `states[*].lifespan` for the walked species and holds them at 1e-5. Every other row stays at 1e-6.
- **Who decided.** The host decided this, as it decided `ROUGH_LAYOUT`. The release PR lists it under Decisions so that the owner sees it.
- **Then finish the task.** Merge `fn-231-cut-wip` onto the task branch. The three additions you made are accepted: woken axes are never cut, death is spread over two cycles in expectation, and the floor is above 0. Then run the shipped-species pass with strips wherever a species changes, R3 at both ends, the gate, npm test, the review and `flowctl done`.
