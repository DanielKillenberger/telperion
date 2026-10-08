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
