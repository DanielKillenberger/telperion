# fn-301: a tree rests on its ground — the closing proof (task .5)

Branch `fn-301-a-tree-rests-on-its-ground`; base d600fa12c (worktree `.worktrees/f4-301-base`, with its own `target/`); task .5 started at 4d21146c8.

## R1, R4: ground_reach

`crates/telperion-core/tests/ground_reach.rs` is no longer ignored, and it is green on all 168 rows (`raw/t5-ground-reach-bark.log`; the table is `r1-change.tsv`, beside `r1-base.tsv`). It asserts, per vertex, that nothing lies below the ground except a leaf stalk's dip, within `b·scale − h`. The largest dip it reports is 9.3 mm, on the palm at 12. Under host ruling 2 the table now carries `bark_low`, the lowest non-trunk curve centreline less its radius, beside the drawn vertices (`wood_low`). The column is reported, not asserted. It is a sphere bound: the curve's radius includes the flare and lobes, and it ignores a ring's tilt. On 42 rows it reads below 0 (the palm 8 to 40, the birch, the spruce at 40, the ordinary tree at 40 s42, and Laurelin at 80 s42 at −0.016 m), while the drawn vertices on those rows are on or above the ground. Laurelin's bytes are unchanged, so for her this is the bound, not a change.

## R3: digests

`HASHES.md` holds the method and the three-row check; the raw files are `raw/t5-digest-base.txt` and `raw/t5-digest-change.txt`, and the per-row table is `raw/t5-digest-compare.md`. All 168 rows of R1 were built through `pipeline::build`, and every artifact was hashed.

| Rows | Rows checked | Result |
|---|--:|---|
| beech, Telperion, Laurelin, every age and seed | 63 | byte-identical |
| palm 6–20 and 40; birch, every row; spruce 40; ordinary and oak 40 s42 | 47 | changed, as expected |
| ordinary 6–20, 40 s1; oak 6–20, 40 s1; spruce 6–15, 20 s1 and s7 | 55 | byte-identical |
| ordinary 40 s7, oak 40 s7, spruce 20 s42 | 3 | changed, via the landing ease (below) |

**The three rows (host ruling, 2026-10-21).** Their lowest non-trunk bark is clear on the base (`t5-bark-base.tsv`):

| Row | Lowest bark on the base | Lowest bark on the change |
|---|--:|--:|
| ordinary 40, seed 7 | +0.0913 m | +0.091 m |
| oregon-white-oak 40, seed 7 | +0.2431 m | +0.243 m |
| norway-spruce 20, seed 42 | +0.0807 m | +0.081 m |

With the landing ease alone put back on the centreline (`support(tip.z, …)` in `lay.rs`, a scratch patch that was never committed and was reverted, leaving an empty diff), all three come back byte-identical to the base (`raw/t5-digest-ease-centreline.txt`). The cause is proven: the landing ease reads the bark across its whole zone, `4·radius + 6·length`, so low wood inside that zone eases from the bark even where it never touches. R3's first bullet in the spec says so (host, 2026-10-21). At the hero view, the ordinary tree and the oak at 40 s7 draw the same picture: 0 and 0.06 pixels differ (AE).

**bloom_axis (host ruling 3).** `place.wgsl`'s copy of `Axis::laid`'s loop is gone, and `bloom.wgsl`'s `bloom_axis` serves both `floret` and `inflorescence_rest`. The device's organ words for Telperion and Laurelin, at 12 and at their own ages, seeds 1, 7 and 42, are byte-identical with and without the change (`t5-organ-digest.rs`, a scratch test; `raw/t5-organ-base.log`, `raw/t5-organ-change.log`). The floret, organ, stalk, route, select and shader tests pass (`raw/t5-bloom-tests.log`, 16 passed).

## R3: cost

These are paired `build_time` runs (`BUILD_PRESETS`, `BUILD_AGE`, `BUILD_SEED=1`), base worktree against change, on the same machine (RTX 3080), with the sides alternated (`raw/t5-build-time.log`). Each cell is warm ms, with cold ms in brackets.

| Tree | Base, run 1 | Change, run 1 | Base, run 2 | Change, run 2 |
|---|--:|--:|--:|--:|
| beech 40 (clear) | 180 (269) | 177 (234) | 174 (233) | 174 (236) |
| palm 12 | 41 (94) | 41 (93) | 45 (93) | 41 (93) |
| spruce 40 | 483 (546) | 475 (519) | 463 (525) | 479 (523) |

The clear tree is within noise. The palm at 12 and the spruce at 40 are also within noise of the base. Instance counts (33,427; 6,460; 4,024,570) and device peaks (201, 34, 1,587 MB) are equal on both sides.

## R5: stills

These come from the headless example, CPU `mesh::build`, at the hero view, 900x1200, base against change (`raw/t5-stills.log`). The PNGs are in `raw/stills/`. The sheets for the owner put base on the left of each pair:

- `stills-palm.jpg`: the palm at 8, 12, 15 and 20, seed 1.
- `stills-wood.jpg`: the spruce at 40 and the birch at 40, seed 1, and the oak at 40, seed 7.
- `stills-far.jpg`: the palm at 12, seed 1, from 600 m at 2° elevation, 400x400, whole and as a 5x crop.

The agent viewed all three sheets:

- **Palm.** On the base, the lower fronds at 12 to 20 are cut off where they enter the ground. On the change, they lie spread on it, so the crown reads wider at its foot. Paler leaves show under the crown at 12 to 20, most likely the dead skirt now lying on the ground instead of buried in it. At 8 the difference is small. Pixels differing (AE, 2% fuzz): 2.5%, 3.7%, 3.8% and 3.5%.
- **Spruce, birch, oak.** No visible difference at the hero view. Pixels differing: 0.02%, 1.5%, and about 0.
- **Far view.** At 400 px high and 38° the frame spans about 581 px a metre at one metre. The palm's leaflets have a reach of 0.63 at size 1 and a median scale of 2.16, so thinning is active past about 400 m. At 600 m the kept share is about 0.4, enlarged about 1.5 times. The whole palm is about 10 px across. No dip is visible on either side, because at this distance any dip of an enlarged leaf is under a pixel (inferred from those figures). The owner may rule on it as an Open item.
- **Walk still.** The film has no walk through a young palm: its palm walks are at 40 (`walk_rachis.rs`, oak to palm at 40, seed 7). So no walk still was made.

## docs

`docs/pipeline.md`: the claim that four families have no engine species is dropped. One sentence now places contact in the lay walk and the frond curve, and in the pose where instance matrices finish.

## Gate

See the task's done summary.
