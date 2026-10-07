# fn-213 sheet 8: round 8, curtains (worker, 2026-10-07)

Round 8 aims at COMPARE.md's measured gaps, not crown shape (host, 2026-10-07): more of the fine wood hangs; strands are longer, hang straight and are unbranched; twigs on the limbs come back toward the old tree's density. The scaffold architecture of round 7 is kept. Values only, in the birch's rows; no row or number is taken from the old generator.

## What moved (against round 7, `3ff611e9`), and why

| Row | Round 7 | Round 8 | Source and reason |
|---|---|---|---|
| Hanging axis `BRANCH` (`/space/ages/9`): set-point, `tropism`, `wander`, `internodeSpread` | −1.0 rad, 1.0, 0.6, 0.6 | −π/2, 3.0, 0.15, 0.3 | JRC "branches characteristically droop", OSU "young twigs thin and usually pendulous", S3-limb-twig-curtains (strands fall straight); MU26 for the set-point mechanism (rule 12). A strand turns to vertical within about 30 cm of its bearer |
| `BRANCH` laterals | `SHOOT` 0.25 medial, 0.4 top; `SHORT` 0.5 | `SHORT` 0.5 only; the `SHOOT` age is no longer lived | S4-pendant-shoots and S3-limb-twig-curtains: long, unbranched whip-like shoots carrying short shoots (rule 8, JH87) |
| `BRANCH` lifespan, nodes a year, internode, viability | 6 years, 6 to 10 nodes (3 zones), 0.035 m, 0.95 | 12 years, 4 to 6 nodes (bare node and one zone), 0.06 m, 0.96 | A strand grows a long-shoot unit a year (3 to 7 leaves on a congener's long shoots, SU03) for up to 12 years. That is about 0.3 m a year and up to 3 m, the strand lengths in S3-limb-twig-curtains and S2-Uusimaa; apex death (viability) varies the lengths |
| `LIMB`, `BOUGH` lateral `BRANCH` (medial, top) | 0.1, 0.25 and 0.1, 0.22 | 0.4, 0.5 on both | Strands all along the limbs, as in S3-limb-twig-curtains |
| `TOP` lateral `BRANCH` (medial, top) | 0.15, 0.2 | 0.3, 0.4 | The same on the old top's forks |
| `LIMB`, `BOUGH` medial zones: sleeping `BRANCH` buds (`dormant`), `delay`, `rate` | none | 0.6 a node, 4 years, 0.15 a year | Older limb wood keeps strands as the first ones die: epicormic sprouting of *B. pendula* (Cameron and Sani 1994, abstract: "Epicormic shoots were induced to sprout on stumps and small logs"). The same engine mechanism as the oak's reserve buds |

Each number is **estimated** from the stills against the photographs and checked against the measures below. No old-generator row was read for a value.

## Measures (code: `birch_measure.rs`; raw in `raw/measure-round8.txt`)

Seed 1. The old birch is age-less (its envelope is fixed), so one column stands for both ages. Seeds 7 and 42 follow the same pattern (raw files).

| Measure | Old | Round 7 at 40 | Round 8 at 40 | Round 7 at 80 | Round 8 at 80 |
|---|--:|--:|--:|--:|--:|
| Hanging strands / fine runs | 32,630 / 44,225 | 3,394 / 40,868 | 2,174 / 30,847 | 12,248 / 139,495 | 13,904 / 178,427 |
| Strand length p50 / p90 / max, m | 0.45 / 1.32 / 3.00 | 0.30 / 0.46 / 1.23 | 0.75 / 1.26 / 2.68 | 0.28 / 0.37 / 1.29 | 0.84 / 1.56 / 2.77 |
| Strand droop p10 / p50 / p90, deg | 40 / 40 / 90 | 33 / 48 / 70 | 35 / 53 / 69 | 33 / 48 / 70 | 38 / 60 / 76 |
| Droop of all fine runs ≥ 5 cm, p50, deg | 40 | 25 | 42 | 26 | 53 |
| Fine runs per metre of wood ≥ 1 cm radius | 12.5 | 4.3 | 4.7 | 1.6 | 4.2 |
| Fine wood (< 5 mm), km | 30.1 | 3.4 | 4.5 | 10.1 | 27.6 |
| Leaves | 231,665 | 339,014 | 399,714 | 1,106,770 | 3,118,139 |
| Leaves nearest a hanging strand / a run under 5 cm / elsewhere | 67% / 0% / 33% | 25% / 18% / 57% | 34% / 16% / 50% | 27% / 15% / 58% | 34% / 12% / 53% |

A strand's chord droop counts its first, outward-arching centimetres, so a strand that hangs vertical over most of its length reads 50° to 75°; the stills show them vertical. The count of hanging *runs* fell at 40 because each strand is now one long unbranched run instead of a hanging system and its side shoots. Twigs per metre of limb at 80 rose from 1.6 to 4.2, about a third of the old tree's 12.5.

## Cost (native, `--profile ci`, sheet 8 run; limits 8 threads)

| | Round 7 | Round 8 |
|---|--:|--:|
| Nodes at 40, seeds 1 / 7 / 42 | 420,906 / 753,616 / 519,294 | 353,160 / 688,759 / 425,062 |
| Nodes at 80, seeds 1 / 7 / 42 | 1,573,152 / 1,818,916 / 1,848,829 | 2,331,473 / 2,746,150 / 2,695,068 |
| Grown at 80, s (seeds 1 / 7 / 42) | 0.62 / 0.76 / 0.80 | 0.94 / 1.16 / 1.00 |
| Dressed at 80, s | 1.3 / 1.7 / 1.6 | 2.6 / 3.0 / 2.9 |
| Leaves at 80 | 1.11M / 1.24M / 1.31M | 3.12M / 3.41M / 3.55M |

At 80 the tree costs about 1.5 times the nodes and 2.8 times the leaves of round 7, and roughly double the time; the most is 2.75M nodes, under the browser's 4M cap. The harness opens the tree at 40, which is cheaper than round 7. Nothing was trimmed to hold the cost.

## The stills

Compare sheets in `raw/compare/`: `compare8-40.png` and `compare8-80.png`. For each of seeds 1, 7 and 42 there are three rows: the old birch, round 7 and round 8 (bare, in leaf, crown, limb). Round 8's stills are in `raw/sheet8/`. The worker viewed every still.

**The worker's reading.**

- **Curtains:** round 8 hangs curtains of straight strands from the limbs, plainest in leaf and in the limb close-ups, where round 7 had short drooping sprays.
- **Bare:** the 40-year upper crown and the 80-year interiors are darker, denser masses than the old tree's.
- **Crown size:** the engine crown is narrower than the old tree's at both ages.
- **Strand ends:** the strands are of near-equal length and stop inside the crown's outline. The old curtain reaches lower, past the crown, to a common hem.

## Astra's verdict

`codex exec -m gpt-6-astra --sandbox read-only`, output `raw/astra-round8.md`. Asked to judge curtains, bare fine wood and crown shape, comparing round 8 with round 7 and with the old birch against the photographs, leaf seating left aside. Verbatim:

> ROUND 8 improves curtain coverage, but its repeated straight strands look mechanical. Leaf seating is excluded. The OLD rows are labelled age 70, so their shape comparisons are against that displayed baseline.
>
> | Age | Trait | ROUND 8 | Evidence | vs round 7 | vs OLD |
> |---|---|---|---|---|---|
> | 40 | Curtains | **Fail** | In `compare8-40.png`, all three ROUND 8 crowns carry long curtains, but the limb close-ups show parallel drops with repeated hooked starts and similar lengths, unlike the broken, varied drapery in `refs.png`’s Uusimaa photograph. | Better | Worse |
> | 40 | Bare fine wood | **Fail** | In `compare8-40.png`, especially seed 42’s limb close-up, ROUND 8 forms a comb of long wires and a dark central bundle rather than the irregular branching haze visible in the New Forest and sunset photographs in `refs.png`. | Worse | Worse |
> | 40 | Crown shape | **Pass** | In `compare8-40.png`, ROUND 8’s distant leafy crowns retain a narrow egg envelope across the three seeds, although seed 42’s detached lower curtains weaken its outline. | Same | Better |
> | 80 | Curtains | **Fail** | In `compare8-80.png`, ROUND 8 clothes much more of every crown, but seed 1’s limb close-up and seed 7’s leafy close-up show excessively straight, evenly aligned hanging strands compared with Uusimaa in `refs.png`. | Better | Worse |
> | 80 | Bare fine wood | **Fail** | In `compare8-80.png`, ROUND 8’s seed 1 and 7 bare views combine dense dark interiors with straight vertical fringes, while the winter photographs in `refs.png` show an open, irregular network of fine branching wood. | Worse | Worse |
> | 80 | Crown shape | **Fail** | In `compare8-80.png`, seed 1 approaches a broad crown, but seeds 7 and 42 remain tall and pointed, missing the broad rounded envelope shown by the Oslo lawn birch in `refs.png`. | Same | Worse |
>
> The single most useful change would be to give the hanging fine wood irregular curvature, branching and termination lengths, breaking up ROUND 8’s repeated parallel curtain strands.

**In short:** curtains are better than round 7's and worse than the old birch's: too straight, parallel and alike in length. Bare fine wood is worse than both: a comb of wires around dark interiors. Crown shape is unchanged from round 7: the 40-year egg passes and the 80-year crown fails.

## For the host

- **Not faked:** strand separation and a floor clearance. Neither has an engine mechanism. On the sheet, round 8's strands end at their own lengths inside the crown outline, while the old curtain falls to a common hem below it; whether that hem is needed is the host's call.
- **Variation:** Astra's change, irregular curvature and termination lengths, has engine rows: `wander`/`bendLength` on the strands (set low this round for straightness), `viability` and `internodeSpread` for lengths. Branching would undo the unbranched strands that this round aimed at. The two asks pull against each other; the host decides which.
