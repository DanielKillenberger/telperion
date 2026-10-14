# fn-275 R2b: a seedling's entire fronds becoming pinnate (worker, 2026-10-15)

## Design built (host's hint, no new row)

Each frond's leaflet count is the mature `leafletCount` raised to the establishment's course at the crown's age when the frond emerged: `N ^ course(born / frondEstablishment)`. The clock is R1's (`Crown::born`, shared with `Crown::established`), so the count runs on a log scale from 1 at the first frond to `N` once established, and is kept for the frond's life. The count is fractional and split by fn-277's keyed draw (`organs::frond_leaflets`, `stations.wgsl`'s `frond_leaves` through `count_word` in `organs.wgsl`). A frond made once established carries `leafletCount` exactly (no `powf`), so the palm at 40 draws the same 6,554 leaves. Dormant with no establishment, with no rosette, or with no leaflet grouping.

## Clock against the source (host ruling 3: accepted as a known simplification)

FAO 156 ch. I: entire leaves "during the first three years after seed germination"; the snippet-only "Plants at 12–18 months old begin to produce feathered (compound) leaves" (SOURCES.md). The shared clock is `frondEstablishment` 6.75 y, not 3 y. On the log scale the two agree on the entire years: the fronds of years 1 and 2 carry 1.0 to 1.3 leaflets. They part after that: year 3's fronds carry 1.7 to 3.1, and the mature count arrives at 6.75 y. No second clock or new row was added.

## Measurements (seed 1, the palm's rows: N 113, 42 fronds over 5 years, elongation 226 days)

The leaflets each frond was made with, by the year it emerged (`Crown::leaflets`, the size share `Crown::established` beside it):

| Year | Serials | Age at emergence | Leaflets | Size share |
|--:|--:|--:|--:|--:|
| 1 | 0-7 | 0.07-0.59 | 1.00-1.05 | 0.10-0.11 |
| 2 | 8-15 | 1.04-1.56 | 1.12-1.31 | 0.12-0.15 |
| 3 | 16-24 | 2.01-2.60 | 1.70-3.09 | 0.20-0.32 |
| 4 | 25-32 | 3.06-3.57 | 6.1-15.1 | 0.45-0.62 |
| 5 | 33-41 | 4.03-4.62 | 31-61 | 0.76-0.88 |
| 6 | 42-49 | 5.07-5.59 | 82-98 | 0.94-0.97 |
| 7 | 50-57 | 6.04-6.56 | 107-112 | 0.99-1.00 |
| 8 on | 58- | 7.01- | 113 | 1 |

Drawn (CPU, test `a_seedlings_fronds_are_entire_and_become_pinnate`; the device agrees leaf for leaf at 3 and 4):

| Age | Fronds drawn | Leaves | Without the rows |
|--:|--:|--:|--:|
| 1 | 9 | 10 | 1,017 |
| 2 | 17 | 20 | 1,921 |
| 3 | 26 | 45 | 2,938 |
| 4 | 34 | 151 | 3,842 |
| 40 | 58 | 6,554 | 6,554 |

The steepest step between consecutive fronds is 0.73 in the log count, at a year's turn.

## Stills (`raw/seedling-stills/date-palm-{1,2,3,4,40}-1-1-whole.png`, `examples/unfold.rs`, the whole year)

I viewed the stills at 1, 3 and 4. At 1 the palm reads as the source describes: about nine entire strap leaves on the stump. At 3 and 4 it does not. The palm stands small at the centre, and a scatter of single leaflets lies metres out across the ground, with no frond joining them. At 4 the farthest leaf stands 5.0 m from the apex (2.6 m at 3). The sparse leaflets also widen the bounds, so the hero camera pulls back.

Cause: a frond is drawn only as its leaflets, and no rachis is drawn as geometry. The mature frond's 113 leaflets read as one frond by their density. A juvenile frond strings 2 to 15 leaflets at their mature spacing along a rachis of up to 4.2 m (7 m × the size share), so each leaflet floats on its own. The entire leaf is also short: a lone placement is one leaflet-sized blade (2.35 m × share) at the frond's base, while the frond it stands for runs 7 m × share plus a leaflet. Where a frond's draw goes from 1 leaflet to 2, its reach jumps by that rachis.

## Stopped for the host

How a frond with few leaflets reads is a design judgment, beyond the hint I was given. Candidates:

- the rachis shortens with the count;
- a leaflet's length or width grows as the count falls;
- the entire leaf spans the frond's length;
- a drawn rachis.

The full gate was not run. The task stays open until the host rules.

## Host ruling, 2026-10-15: segments of one blade, built

`Crown::segments` on the CPU and `segments` in `stations.wgsl` take `f = ln n / ln N`, the course the count already uses. A young frond's leaflets then sit as follows:

- **Placement.** They stand where a frond's leaflets always stood, at `(k + 1) / n` of the frond's rachis, which is its own length times its R1 share. The spacing is the blade length over `n`.
- **Length.** Each runs from the blade's length (the rachis over the element's reach, `crown::blade`, about 0.63 m at a size of one) to the mature leaflet's on a log scale: `size × (R / (E·S))^(1 − f)`, where R/(E·S) = 4.73 on the palm.
- **Pitch.** It runs linearly from 0 (along the rachis) to the mature 55°.
- **Width.** The leaf instance scales uniformly, so the width follows the length (blade width at f = 0).
- **Spines.** My choice, which the ruling did not cover: a young frond keeps the mature share of spines, `acanthophylls × n / N`. Without it, the palm's 8 basal spines would turn every segment of a 2-to-8-segment frond into a spine. A lone placement (n = 1) bears none on either executor.

No rows were added. The device carries R/(E·S) in the rosette's spare word (`rosette[6].z`). A mature frond is its rows exactly: the palm at 40 still draws 6,554 leaves, and the still is byte-identical (`cmp`) to the one drawn before the change.

Tests:
- `a_seedlings_fronds_are_entire_and_become_pinnate` adds two checks: the first frond's strap reaches its blade (within 1 %) at a pitch under 0.1°, and `segments` of a mature frond returns its rows unchanged.
- In `the_shipped_palm_keeps_its_height_and_narrows_only_its_foot`, the R2 check "young leaflets are smaller in the mean" now measures crown spread instead. A segment is now longer than a mature leaflet by design.
- The device test compares the drawn leaves' axes: 8.4 to 10.8° apart in the mean under the palm's 18° scatter, and 0° to three decimals with scatter off. Positions agree within 5.4e-5 of the height at 3 and 4.

### Stills (`raw/segments-stills/date-palm-{1,2,3,4,40}-1-1-whole.png`, the whole year)

I viewed four: 1, 2, 3 and 4.
- **At 1:** about nine entire strap leaves, each as long as its frond's blade (0.7 to 0.8 m). It reads as a seedling.
- **At 2:** a tuft of straps. Two long single segments float detached to one side, the outer segments of the first fronds that drew n = 2.
- **At 3:** a wide spray of long, narrow segments. Several stand off alone, metres out.
- **At 4:** a bushy rosette of broad segments. It reads closest to a young pinnate palm, still with a fringe of long segments far out.

### Where it reads wrong: segments run past the blade

Per frond (seed 1), from the code's formulas. "Reach" means the last segment's base plus its length along the rachis.

| Frond | Emerged | f | n | Blade (m) | Segment (m) | Pitch | Reach (m) | Reach / blade |
|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| 0 | 0.07 | 0.001 | 1.00 | 0.70 | 0.70 | 0° | 0.70 | 1.00 |
| 15 | 1.56 | 0.058 | 1.31 | 1.06 | 0.97 | 3° | 0.97 (n = 1) | 0.91 |
| 16 | 2.01 | 0.112 | 1.70 | 1.41 | 1.18 | 6° | 2.58 (n = 2) | 1.83 |
| 20 | 2.31 | 0.167 | 2.20 | 1.75 | 1.35 | 9° | 3.08 | 1.76 |
| 24 | 2.60 | 0.239 | 3.09 | 2.20 | 1.52 | 13° | 3.68 | 1.67 |
| 25 | 3.06 | 0.383 | 6.13 | 3.12 | 1.72 | 21° | 4.72 | 1.51 |
| 32 | 3.57 | 0.575 | 15.1 | 4.32 | 1.77 | 32° | 5.83 | 1.35 |
| mature | | 1 | 113 | 7.00 | 1.48 | 55° | 8.48 | 1.21 |

The ruling's two rules conflict. The leaflets stand at `(k + 1) / n`, so the last always sits at the blade's tip. A few-segment frond's segments are nearly the blade's length. Together these put the outer segment wholly past the blade: from the blade's end to 1.7 to 1.8 blades out at n = 2 to 3. The frond's reach also jumps where its draw goes from one segment to two: from 0.97 m to 2.58 m between fronds 15 and 16. That breaks ruling 1's "no floating leaflets past the blade". Settling it needs a ruling on where a segment's base stands, or on how long it is at small n; I did not choose either.

Superseded by the ruling below.

## Host ruling, 2026-10-15 (second): strips whose tips converge on the blade's tip, built

`Split::segment` on the CPU and `leaflet` in `pack.wgsl` place segment k of n as follows:

- **Base.** It stands at `b_k = mix(k/n, p_k, f)` of the blade, where p_k is the station (`(k+1)/n`, or the base for a lone placement).
- **Length.** It runs `exp(mix(ln((1 − b_k)·blade), ln(mature leaflet), f))`. The device writes this as the share `((1 − b_k)·R/(E·S))^(1 − f)` of a mature leaflet, with R/(E·S) = 4.73 read from the rosette's spare word through the segment's `edge.x`.
- **No floor needed.** `1 − b_k ≥ (1 − f)/n` stays above zero below maturity, so no floor was added.

Pitch, width and spines are as built before (spines accepted). A mature frond (f = 1) takes the old code path exactly. The palm at 40 still draws 6,554 leaves, and its still is byte-identical (`cmp`).

With no size variation and no scatter, the device and the CPU agree exactly on sizes and axes at 3 and 4 (mean size apart 0, axes 0.0°). With the draws on, they differ by the draws alone: mean size 6 to 11 %, inside the 4σ bound, and axes 8.3 to 10.8° apart.

`suite::organs::an_emerging_frond_keeps_its_leaflets` pinned the palm at 1, seed 7, at 1,017 leaves, which was every frond's 113. Its seedling fronds are now entire, so it is re-pinned to 10.

### Reach per frond, in blades (seed 1; `a_seedlings_fronds_are_entire_and_become_pinnate`)

Reach is the farthest strip's base plus its length along the rachis, at the lesser and the greater whole count its draw can give.

| Frond | f | n | Reach (lesser) | Reach (greater) |
|--:|--:|--:|--:|--:|
| 0 | 0.001 | 1.00 | 0.999 | 1.000 |
| 8 | 0.024 | 1.12 | 0.963 | 0.990 |
| 15 | 0.058 | 1.31 | 0.914 | 0.979 |
| 16 | 0.112 | 1.70 | 0.840 | 0.964 |
| 18 | 0.137 | 1.91 | 0.808 | 0.960 |
| 19 | 0.151 | 2.05 | 0.958 | 0.988 |
| 24 | 0.239 | 3.09 | 0.989 | 1.005 |
| 25 | 0.383 | 6.13 | 1.033 | 1.035 |
| 32 | 0.575 | 15.1 | 1.062 | 1.061 |
| 41 | 0.868 | 60.6 | 1.135 | 1.135 |
| 57 | 0.998 | 111.8 | 1.207 | 1.207 |
| mature | 1 | 113 | 1.21 | 1.21 |

The reach stays between 0.81 and 1.21 blades. Where a frond's draw goes from one strip to two, its reach moves by at most 0.16 blades (frond 18: 0.81 against 0.96), against 1.83 blades under the first ruling. The low end is a lone strap at f 0.14, whose length is the blade's blended with a mature leaflet's on the log scale, 4.73^−f of the blade. The test holds the reach to 0.78 to 1.22 and the two counts within 0.2 of each other.

### Stills (`raw/strips-stills/date-palm-{1,2,3,4,40}-1-1-whole.png`, the whole year)

I viewed 2, 3 and 4; age 1's fronds have n ≈ 1 and f under 0.01, as in the still viewed before.
- **At 2:** a coherent tuft of entire straps. The two-strip fronds show as a bifid leaf: the outer strip starts mid-blade and runs to the blade's tip.
- **At 3:** a compact rosette of broad strips. A few outer strips of the far fronds stand apart from their frond, which has no rachis drawn.
- **At 4:** a bushy young palm of narrowing segments along each frond.

Nothing floats past the blade. The one remaining read is a frond's outer strip standing apart from its first, because no rachis is drawn (ruling 3: no rachis geometry).

## Gate

I ran `cargo test --profile ci --workspace --no-fail-fast` once: 1,406 passed, 1 failed, 101 ignored (`raw/seedling-gate.log`). The one failure was `generation_limits_are_classified`: three new guard sites in `crown.rs` were unclassified. I classified them in `docs/generation-limits-inventory.json`, and the guard target alone then passed 2 of 2. Every catalogue pin at the shipped age passed unchanged.
