# A station's compound leaf draws its rachis

**Superseded by fn-309** (host ruling, 2026-10-22): R1's second branch is taken there. A placement on wood draws no rachis, so its `rachisLength` is dormant and its leaflets stand at the placement point; no axis is drawn for a station. Close this spec with fn-309.

Gap found by fn-283 (host, 2026-10-19). On the film's oak-to-palm walk (age 40, seed 7), the oak's cauline share still clothes twig stations at the walk's start, and those placements fan into compound leaves on rachises nobody draws. Measured on fn-283's base (`.flow/evidence/fn-283-a-frond-draws-its-rachis/raw/walk-probe2.tsv`): at t = 0.1, 0.2 and 0.3 there are 1,445, 87 and 56 such leaves, of which 38, 17 and 28 stand over 0.5 m from wood (median 0.12, 0.25, 0.51 m); from t = 0.4 none remain. fn-283 draws the frond's axis only; this is the station-borne case, a second design because stations are placed on the swept surface after the wood is drawn.

Not in the release unless the owner, viewing the film re-rendered with fn-283, judges these leaves visible (owner decision).

## Requirements

- **R1:** a compound leaf borne at a station draws its rachis as wood through the one pipeline, or the walk never places a compound leaf where no axis can be drawn. Which of the two is the design question this spec answers first, measured on the walk.
- **R2:** on the film's walk, every station-borne leaflet base lies on drawn wood across t = 0 to 1 (fn-283's probe, split by source).
- **R3:** shipped presets that bear no compound station leaves build byte-identical meshes.

