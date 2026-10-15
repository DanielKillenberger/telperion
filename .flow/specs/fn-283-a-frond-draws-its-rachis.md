# A frond draws its rachis

After the release (host, 2026-10-15, from fn-275 R2c). The owner, viewing the young date palm: "it looks nice other than having the leaves not being connected in a sensible way". For the release fn-275 crowds a young frond's segments toward its base; this spec draws the rachis itself.

## Requirements

- **R1:** a frond's leaf base, petiole and rachis are one woody axis, continuous with the retained leaf base (fn-144), hung as wood beside the leaf bases and drawn by the existing surface builder, bounds, shadows, level of detail and device path. No second renderer path.
- **R2:** one source of truth for the path: the leaflets and the axis follow the same arch (`rachisLength`, `rachisArch`), and on a young frond the axis ends at the last segment's base.
- **R3:** one new row, the rachis base width, 0.25 m for the date palm (FAO 156 ch. I, lines 230–232), scaled by the frond's size share, tapering, never below FINEST; zero is dormant, and it is dormant without leaflet grouping.
- **R4:** on the device the frond clock and segment draw move to the earliest stage that needs them (the leaf-base stage), computed once; the worker's estimate was about 1,200 lines (fn-275 FRICTION.md, 904a17ba7).
- **R5:** CPU = device; the palm at 40 under 0.5 s warm; stills viewed; the owner's verdict. A leaf-coloured axis material is a later item.
- **R6 (host, 2026-10-15, from fn-275 R2c):** the spines leave gaps without a drawn rachis: a leaflet next to a basal spine sits 1.0 to 1.88 widths from any other segment on young fronds, the spines 1.9 to 3.2 from f 0.57, and the mature frond 1.39 (fn-275 SEEDLING.md). With the rachis drawn, every leaflet and spine base lies on it.
