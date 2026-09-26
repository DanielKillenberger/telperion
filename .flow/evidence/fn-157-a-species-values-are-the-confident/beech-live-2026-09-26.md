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

## After the host's decisions (same day, `--extend`, no Firecrawl call)

The recording was extended for the document kinds, the tiered appearance
reading and the category photograph search; Jev and the vision adapter
answered live, Firecrawl not at all.

- **Kinds.** Jev classed the Woodland Trust and the archived EUFORGEN
  guideline `forestry`, Morton and Chicago Botanic `garden`, NC State and
  Oregon State `extension`, the landscape designer's blog `nursery`, IUCN and
  bomeninfo `other`. 9 calls.
- **What the tiers held.** Height: forestry 1 (the Woodland Trust's 40 m),
  garden 1, extension 2, nursery 1. Only one forestry source stated a height:
  the EUFORGEN guideline was read as binary (FRICTION.md), so its 30-35 m
  never reached the aggregate. The extension tier's two agreeing pages
  decided: 17.5 m (15-23 m), `agreed`. With the guideline readable the forestry
  tier agrees and decides 36.25 m (`tests/tiers.rs`). Crown width 12.9 m and
  leaf length 8.5 cm (extension, agreed); leaf width 11.4 cm (NC State alone,
  thin); trunk diameter 1.5 m (nursery, thin); crown base unsourced.
- **Start** takes the median: envelope height 17.5 m.
- **Appearance** read the best tier only: 71 level calls, against 184 when it
  read every document (113 saved a run).
- **Photographs (R7).** The single-tree category gave "Fagus sylvatica TK
  2023-05-06 1" (CC BY-SA 4.0): the look kept it as a mature, open-grown whole
  tree in leaf. No bare tree was kept.
- **Cost.** 128 Jev calls in all (9 kinds, 48 labels replayed, 71 levels),
  two vision calls (the look and the inventory), no Firecrawl credit.

## The native-range live run and Tune (R8), 2026-09-26

A bare seed with `native_range` Europe (`de` Rotbuche, `fr` hêtre commun),
recorded in `raw/beech-live2`.

- **Height** 40.0 m (range 30-40), forestry tier, agreed: waldwissen,
  EUFORGEN and the Woodland Trust. Crown width 12.9 m and leaf length 8.5 cm,
  extension, agreed. Trunk diameter 1.5 m, nursery, thin. Leaf width 11.4 cm,
  extension, thin (NC State alone, outside the flora's 2.5-7 cm). Crown base
  unsourced. Height and diameter sit inside the flora range, so the run went on.
- **Photographs:** 12 candidates, 7 open-licence, one kept: the whole tree in
  leaf.
- **Start** derived 17 values; envelope height 40 m.
- **Tune revision 1** measured the baseline at height 39.97 m and crown
  13.4 m (both pass), trunk diameter 1.490 m against the point [1.5, 1.5]
  (fail). The failed numeric gate makes the baseline infeasible, so Tune
  stopped before its first round and Gaps never ran. How a one-source value
  should gate is the host's call (FRICTION.md).
- **Spend:** Firecrawl about 50 credits (gather and fetch); Jev 15 kind
  calls, 51 labels, 103 appearance levels and 3 Tune preparations (1,579
  tokens each); vision one look, one inventory and three Tune passes.

## After the agreement rule (host decision, same day)

A value gates only when its deciding tier agrees. The beech's height (three
forestry sources), crown width (two extension sources) gate; trunk
diameter, leaf length and leaf width are contextual, crown base unsourced.
`gaps.md` lists the contextual fields for a person to source.

- **Tune revision 1** passed the numeric gate (height 39.97 m, crown
  13.4 m) and stopped again before a round: "no matched shots or height".
  `tuning/matched.rs` compares only against references carrying a camera
  `shot`, and the photograph the Profile stage keeps carries none; the
  config's `numeric_references` name the shipped beech's hand-matched
  `B-WHOLE`. No code derives a shot. Gaps never ran. This is the host's
  call (FRICTION.md).
- **Spend** after the rule: Jev 1 Tune preparation (about 1,600 tokens),
  vision one Tune pass; aggregate reran from the recording.

## The Oregon white oak (R5), live from a bare seed

Seed: the beech's shape with the oak's names, `native_range` western North
America, British Columbia to California, with the Canadian name Garry oak;
the host's capability assessment (woody axes, lobed blade, alternate
petiole). `species oregon-white-oak --until start --record`.

| Field | Aggregate | Tier, confidence | Catalogue range | Difference |
|---|---|---|---|---|
| height_m | 21.3 m (15.2-27.4) | forestry, thin (USDA silvics) | 15-27 | inside; range 0.2 m wider each side |
| dbh_m | 0.8 m (0.6-1.0) | forestry, thin | 0.6-1.0 | identical |
| leaf_length_m | 0.10 m (0.05-0.15) | extension, thin | foliage_length 0.05-0.15 | identical |
| leaf_width_m | unsourced | | foliage_width 0.051-0.127 | not settled |
| crown_width_m | unsourced (maximum 38.4 m) | | unknown | both unsourced |
| crown_base_m | unsourced | | unknown | both unsourced |

Every value the aggregate settled lands within its catalogue range
(`tests/replay.rs`). None has three agreeing sources: R1's bar is not met
for the oak. Six of 22 documents were refused (four POWO pages and FEIS
answered 403, CNPS failed TLS). No photograph: the Commons category files
no single oak (its subcategories are range maps, historical images and one
preserve) and geograph, a British archive, has none.

- **Spend:** Firecrawl 10 searches and 22 scrapes plus one PDF parse;
  Jev 14 kind calls, 21 labels, 60 appearance levels; vision none.

## Fixtures

Both recordings are trimmed, checked and pruned to what a replay opens:
beech 425 files, 9.6 MB (the Commons photographs most of it); oak 244
files, 1.4 MB. Trimming them found four ways a trimmed tape replayed
differently or kept too much, each fixed with a test first: a one-word
passage a kind question was asked over, an appearance sentence of two
words, a page no stage read keeping its licence tags (it then read as a
page), and a PDF whose parse stayed whole (the forstpraxis yield table and
the EUFORGEN guideline in full) and whose scrape lost the PDF magic.

## The chosen shot and Tune through Gaps (R8), 2026-09-26

- **Shot.** Six candidate cameras rendered from the Start tree; the look
  chose camera-2 (eye 8 degrees below the aim, fill 0.8). The render's tree
  box, measured once the horizon's one-row streak no longer widened it,
  was half the photograph's crown width; of six boxes up to twice as wide
  the look chose D (twice the width), crown-base line 5 (half the box
  height) and a sun from the camera's left. Two looks, about 1,150 output
  tokens.
- **Tune revision 1** ran seven rounds. The tree it kept is round 2's
  half-strength bundle (score 0.490 against the baseline's 0.622): envelope
  spread 0.161 to 0.211 with crown base, fullness, shoulder and
  irregularity rows, twig hang, droop and reach rows, bark roughness 0.775
  to 0.70, leaf-front green 0.195 to 0.27, and crown, interior and sky
  shading. It then stopped on the runaway rule: from
  round 3 every bundle widened the crown past the gating crown width
  (10.7-18.3 m, two American extension pages) toward the photograph's
  broad crown, and every candidate failed that gate.
- **Gaps** (`gaps.md`): 5 reachable (codominant V fork, broad domed crown,
  ascending then arching limbs, fine layered twig sprays, light translucent
  foliage; each named against the envelope spread dial), 0 identity,
  0 global, 3 unsourced (crown base unsourced; trunk diameter and leaf
  width contextual). The run stops at the owner's look.
- **Spend.** No Firecrawl call. Jev: 63 calls and about 2.04 million
  tokens in the revision (every round asked about all 227 dials, 32 a
  call). Vision: the shot's two looks, one new inventory (the references
  renamed by view), seven visual passes in the revision, and three looks
  lost to failed attempts (a baseline pass before references were named by
  view, a camera look before the box measurement was fixed, and the baseline
  pass of the revision refused for its 227-dial Jev request).

## Follow-up for non-European species

The oak kept no photograph: geograph is a British archive and Commons files
single trees in categories that are mostly European. A North American
species needs another source of open-licence whole-tree photographs; not
built here (host, 2026-09-26).

## After the tier gating rule: Tune capped at three rounds (host, 2026-09-26)

Only a flora, forestry or garden tier gates now. The beech's crown width
(two US extension pages) and leaf length (two extension pages) became
contextual; height (three forestry sources) still gates. Tune reran one
revision from the recording (`--extend`, no Firecrawl call), capped at
three rounds (`max_rounds` 3).

- **Round 1**: all four strengths failed generation ("stems pass through
  each other": the bundle added a second stem).
- **Round 2**: all four feasible, crown widths 17.7, 22.1, 31.3 and 17.9 m.
  Before the rule, every candidate wider than 18.3 m failed the gate. The
  sheet kept the half-strength bundle: crown 17.7 m against the
  baseline's 13.4 m, envelope spread 0.161 to 0.211, habit, twig, bark and
  shading rows.
- **Round 3**: feasible crowns of 22.7, 27.2 and 33.5 m, with lower
  numeric distances (0.25 to 0.37) than the kept tree (0.46). The sheet
  judged none of them better, so the round kept nothing, and the cap ended
  the revision. Wider crowns now reach the reviewer; the reviewer has not
  yet kept one.
- **Gaps**: 7 reachable (single clear bole, codominant V fork, broad domed
  crown, ascending then arching limbs, fine layered twig sprays, light
  translucent foliage, leaf shape unresolved), 0 identity, 0 global,
  5 unsourced (crown base; crown width, trunk diameter, leaf length and
  leaf width contextual).
- **Spend**: no Firecrawl. Jev: 26 calls and about 0.84 million tokens
  (the budget charged 0.86 million). Vision: 7 visual passes in the
  revision (3 sheets).
