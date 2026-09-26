# The beech's first live run through gather, read and aggregate

Run 2026-09-26 from the bare seed (`tests/fixtures/replay/european-beech/seed`),
recorded, through Start. The trimmed recording is the replay fixture; the raw
run stays in `raw/beech-record/` (ignored).

## What it gathered and read

Gather found 16 documents: the Wikipedia article's cited sources (IUCN, POWO,
the Morton Arboretum, bomeninfo.nl's tall trees, a Taylor & Francis paper, an
archived EUFORGEN guideline, the Woodland Trust), Google Books, NC State,
Oregon State, Chicago Botanic Garden, a gardening blog, iNaturalist, a hedge
nursery, Missouri Botanical Garden and a Reddit thread. Fetch read 10. Six
were dropped: two for want of Firecrawl credits, two refused with HTTP 403,
one TLS failure, one unsupported site. The archived EUFORGEN PDF was served as
`text/html` and read as binary text (FRICTION.md).

Read asked Jev 48 labels (82 before the zero-span rule). Aggregate:

| Field | Value | Range | Sources (sites) | Confidence | Set aside |
|---|---|---|---|---|---|
| height | 17.9 m | 12.2-35 m | Oregon State, a blog, Morton, NC State (4) | thin, spread 2.19 | Woodland Trust's "up to 40m", 2.1 times the median |
| trunk diameter | 1.5 m | 1.5 m | a blog (1) | thin | |
| crown width | 13.7 m | 10.7-20 m | Oregon State, a blog, Morton, NC State (4) | thin, spread 1.87 | |
| crown base | unsourced | | | | |
| leaf length | 9.2 cm | 5-15.2 cm | Oregon State, a blog, Morton, NC State (4) | agreed, spread 1.33 | |
| leaf width | 9.2 cm | 7-15.2 cm | a blog, NC State (2) | thin, spread 1.63 | |

Start derived 17 values; the envelope height is the range's midpoint, 23.6 m.

## Against R1

R1 asks three independent sources per field where the literature has them, and
every value inside the range standard floras give (beech: a tree to 30-40 m,
leaves 4-10 cm long and 2.5-7 cm wide, trunks to 1.5-2 m). Not met:

- **Height 17.9 m** is the median of three US landscape pages (50-60 ft, 50-75
  ft) and one 35 m; the European source's 40 m was set aside as an outlier. The
  pages describe the planted tree in American parks, the floras the tree in
  Europe.
- **Leaf width 9.2 cm** rests on two sources, one of them NC State's "Leaf
  Width: 3-6 inches", a copy of its length; two cannot outvote one.
- **Trunk diameter** has one source; crown base none.
- Leaf length (agreed) and crown width sit inside the flora ranges.

## The open questions, with this run as evidence

The spec leaves these to the host; each proposal is a starting point, not a
decision.

1. **Document count.** 16 gathered, 10 read, 4 of them useful for sizes; the
   searches surfaced US extension and nursery pages first and no European
   flora. Proposal: keep 16, but add a query for the regional floras and
   silvics of the species' native range (`<taxon> Flora of <range>`), and
   count only readable documents toward the bound.
2. **Agreement threshold.** Three agreeing sites within a factor of 1.5 would
   have called leaf length agreed and nothing else. The outlier factor of 2.0
   set aside the one European height. Proposal: keep 3 sources, widen agreement
   to a factor of 2 for tree sizes, and set aside only beyond a factor of 3
   (organ sizes vary less than tree sizes; a single table may need both).
3. **Per-source weighting.** This run is the case for it: the median follows
   whichever kind of page the search found most of. Proposal: weight by the
   document's kind (a flora, silvics or forestry manual above an extension or
   nursery page), judged by Jev per document as it labels the spans, with code
   applying the weights; or aggregate the native-range literature first and
   fall back to horticultural pages only for fields it leaves unsourced.

## Photographs (R7) and Tune (R8)

Two looks over twelve Commons candidates each (habitus, solitary, veteran
tree, bark, winter) kept one bark close-up. Every whole tree in leaf was a
copper beech (refused), stand-grown in a wood, or not whole in frame. Tune
refuses without a whole tree, so R8 stops at Tune revision 1. How the search
finds an open-grown tree in leaf is open (a Commons category walk,
geograph's "beech tree in field" photographs under a common name, or a
person's photograph for the first revision).

## Cost

Firecrawl: 30 calls priced by estimate, of which about 22 were billed before
the account refused. Jev: 781 calls over three Profile runs (242 labels, 538
appearance levels; the first run failed on an HTTP 520, the second repeated
read). The appearance traits now read every document, about 180 level calls
a run. Vision: two photograph looks, two inventories, one preflight probe.
