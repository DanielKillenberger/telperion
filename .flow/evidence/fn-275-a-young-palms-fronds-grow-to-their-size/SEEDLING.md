# fn-275 R2b: a seedling's entire fronds becoming pinnate (worker, 2026-10-15)

## Design built (host's hint, no new row)

Each frond's leaflet count is the mature `leafletCount` raised to the establishment's course at the crown's age when the frond emerged: `N ^ course(born / frondEstablishment)`. The clock is R1's (`Crown::born`, shared with `Crown::established`), so the count runs on a log scale from 1 at the first frond to `N` once established, and is kept for the frond's life. The count is fractional and split by fn-277's keyed draw (`organs::frond_leaflets`, `stations.wgsl`'s `frond_leaves` through `count_word` in `organs.wgsl`). A frond made once established carries `leafletCount` exactly (no `powf`), so the palm at 40 draws the same 6,554 leaves. Dormant with no establishment, with no rosette, or with no leaflet grouping.

## Clock against the source

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
