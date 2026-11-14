# fn-364: the old oak's coiling, measured on the grown tree (builder, Opus, 14 Nov 2026; no rule changed)

The reader is `curvature/curve.rs`, run by `curvature/run.sh`. It works on the grower and the settle only, with no drawing, on two trees:
- this branch, at `8e22c8a17` (the oak is the same as at `f6c00cc49`; R8 touches only the Two Trees);
- fn-363's branch, at `c2d897a37` with its reader.

It reads the preset oak at 300 on seed 1007 and at 200 on seed 1001. The full per-axis tables are in `curvature/<branch>-a<age>-s<seed>.md`, each with ten tips.

**Definitions:**
- **A living tip** is an axis with no `apex_end` and at least one phytomer.
- **Its path** runs from its last node down to the collar, through each bearer at the node its axis stands on.
- **Tortuosity** is the outer 5 m of that path over the straight distance from the tip to the point 5 m back along it.
- **The least tortuous** are taken among tips at least 15 m of path from the collar.
- **"Turning inside"** sums the angles between consecutive phytomer headings of one axis on the path.
- **"Junction"** is the angle between an axis's first phytomer and its bearer's phytomer at the node it leaves. It is given only where the path takes the axis from its base.
- **Stretch** is the settled phytomer's stretch. fn-363 has none, shown "-".

**A caution on the "five most tortuous":** in every tree they are siblings on one twig system that share most of their outer 5 m. They read as one case, five times.

## Totals per tip

| tree | set | tortuosity | deg inside / at junctions | deg per metre | axes per metre | 5 m laid at stretch < 0.25 |
|---|---|---|---|---|---|---|
| this, 300, 1007 | most | 57.7, 50.0, 43.7, 43.1, 42.5 | 2,887 / 3,886 (four of five alike); 1,403 / 748 (one) | 1,349 to 1,360; 430 | 41.2 to 41.4; 13.2 | 25%; 0% |
| this, 300, 1007 | least | 1.01 to 1.02 | 52 to 70 / 53 to 301 | 24 to 71 | 0.59 to 2.20 | 0% |
| fn-363, 300, 1007 | most | 47.1, 34.0, 32.9, 30.0, 29.7 | 323 to 349 / 495 to 542 | 168 to 173 | 4.19 to 4.39 | - |
| fn-363, 300, 1007 | least | 1.00 to 1.01 | 35 to 45 / 0 to 52 | 7 to 18 | 0.20 to 0.60 | - |
| this, 200, 1001 | most | 13.9, 12.7, 12.6, 11.7, 11.7 | 365 to 388 / 791 to 801 | 231 to 237 | 5.40 to 5.60 | 0% |
| this, 200, 1001 | least | 1.01 to 1.02 | 41 to 51 / 65 to 213 | 21 to 53 | 1.20 to 3.00 | 0 to 3% |
| fn-363, 200, 1001 | most | 35.4, 34.5, 34.3, 30.4, 25.3 | 228 to 241 / 416 to 473 | 130 to 143 | 2.99 to 3.39 | - |
| fn-363, 200, 1001 | least | 1.00 | 38 to 64 / 379 to 405 | 87 to 88 | 2.59 to 2.79 | - |

Axes on the ten tips' outer 5 m, by origin:

| tree | relay | lateral | continuation |
|---|---|---|---|
| this, 300, 1007 | 886 | 51 | 1 |
| fn-363, 300, 1007 | 83 | 35 | 0 |
| this, 200, 1001 | 136 | 66 | 0 |
| fn-363, 200, 1001 | 73 | 72 | 2 |

## What the tables show (read, not judged)

- **Tortuosity alone does not tell the branches apart.**
  - Both branches have tips whose outer 5 m doubles back on itself: 30 to 58 at 300, and higher on fn-363's branch at 200 (25 to 35) than on this one (12 to 14).
  - fn-363's most tortuous tips at 300 stand at the end of 39.7 m of path. This branch's stand at 16 to 19 m.
- **The two branches coil differently.**
  - At 300, seed 1007, this branch's most tortuous outer 5 m carries about 41 axes a metre and turns about 1,350° a metre. fn-363's carries about 4 axes a metre and turns about 170° a metre.
  - About 890 relays sit on this branch's ten paths at 300, against 83 on fn-363's.
- **The tightest coil on this branch is a relay chain of the bough's age (PA 7), one relay a year** (`curvature/this-a300-s1007.md`, the first tip, births 211 to 268).
  - Each relay lays 5 to 7 phytomers over 0.01 to 0.12 m, at a stretch falling from 0.6 to 0.04 into the older years.
  - Each turns about 20° inside itself and about 10° at its junction, whatever its laid length. So the chain turns about 30° per relay, while a relay advances a few centimetres.
  - A quarter of that outer 5 m is laid at stretch under 0.25.
- **A relay's turn does not shrink with its laid length** (measured, as above). In fn-363's relay chains, of whole length, the same per-relay turn is spread over metres.
  - Whether that turn is the lay's pull towards the vertical, the bend memory, the take-over or a junction's own angle is not read here.
  - `Layer::step`'s wander and bend-memory turns are in proportion to the internode's length. Its pull (`bend`, fading over the axis's reach) is an angle that fades over the reach, however short. This is code read, not measured per term.
- **The least tortuous tips are nearly straight on both branches** (tortuosity 1.00 to 1.02). On this branch they carry more axes and more junction turning per metre than on fn-363's at 300: 0.6 to 2.2 axes a metre against 0.2 to 0.6.
- **At 200, seed 1001, this branch's coiling is milder than fn-363's by tortuosity** (12 to 14 against 25 to 35). It turns more per metre (231 to 237° against 130 to 143°) on more axes per metre (5.4 to 5.6 against 3.0 to 3.4), with nothing laid at stretch under 0.25.

## Not measured

- Which term of the lay makes a short relay turn its ~20°.
- The coiling at 300 on seed 1001 and on seed 1012.
- The drawn mesh's own curvature: these are the settled tree's nodes, not the sweep.
