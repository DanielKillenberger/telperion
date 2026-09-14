# FN34: beech, ash and birch as real species

2026-09-14. Packets, value tables and the fixed/fresh numeric protocol for
European beech and silver birch. European ash has a complete profile and
reference set; its template waits for fn-33. Visual verdicts are unassessed.
`owner_feedback` is null. This report does not award a visual pass.

Raw 24-seed receipts live on disk under `.flow/evidence/fn34/measure/`
(about 106 MB) and are not committed. The compact table is
`numeric-summary.json`. Reference rasters are under ignored `.refs/fn34/`.

## What changed

Two catalogue presets, `european-beech` and `silver-birch`, are value tables
in `crates/telperion-core/src/presets/species.rs`. Materials sit beside the
oak and spruce rows. Growth traits are the oak's; age is 120 yr (beech,
32 m envelope) and 70 yr (birch, 18 m envelope) so the profile's mature
reference height is the envelope. No generator or renderer code changed.

Ash is gathered in `.flow/evidence/fn34/european-ash/`. A simple-blade
fixture exists as `european_ash_fixture` and is not in `Preset`, `from_id`
or `CATALOGUE`. A simple-blade ash is not a pass (R4).

## Numeric protocol

Seeds drawn once with `--draw-seeds` to `.flow/evidence/fn34/seeds.json`
at commit `3468ef710a6f67009c912ffd8c6f7076c5f3771b`, before the full
48-case run. Fixed list is the fn-9 twelve. Fresh seeds were not resampled.

`npm run species:measure -- --profiles .flow/evidence/fn34/profiles.json --seeds .flow/evidence/fn34/seeds.json --output .flow/evidence/fn34/measure`

Capture half (`npm run species:qa -- --capture-only` on the same profiles, seeds and output): every required still wrote a PNG. The runner exited 1 with every `visual_status` unassessed, which is the designed exit. Extra protocol stills (leaf views, first three fresh seeds, ordinary/telperion/laurelin at seed 1) stay under ignored `measure/` and are not the judging set.

| Species | Cases | Numeric | Height m | DBH m (proxy) | Crown width m | Retained failures |
|---|---|---|---|---|---|---|
| european-beech | 24 | 24 pass | 29.98–31.85 | 0.874 | 31.23–32.03 | none |
| silver-birch | 24 | 24 pass | 14.09–15.45 | 0.345 | 12.48–13.72 | none |
| european-ash | not run | unready | — | — | — | waits for fn-33 |

Gating metrics that passed on every seed: height, foliage length, foliage
width; beech also gates DBH. Crown width and crown-base stay contextual.

## Stills

Owner judging set: fixed seeds 1, 2, 3; views `whole` and `bare`; 960×720;
headless on an RTX 3080. Files in `.flow/evidence/fn34/stills/`, hashes in
`stills.json`. Every `visual_status` is `unassessed`. `owner_feedback` is
null. The implementer did not open the rasters (four-images reading rule).

| File | Preset | Seed | View |
|---|---|---|---|
| stills/european-beech-1-whole.png | european-beech | 1 | whole |
| stills/european-beech-1-bare.png | european-beech | 1 | bare |
| stills/european-beech-2-whole.png | european-beech | 2 | whole |
| stills/european-beech-2-bare.png | european-beech | 2 | bare |
| stills/european-beech-3-whole.png | european-beech | 3 | whole |
| stills/european-beech-3-bare.png | european-beech | 3 | bare |
| stills/silver-birch-1-whole.png | silver-birch | 1 | whole |
| stills/silver-birch-1-bare.png | silver-birch | 1 | bare |
| stills/silver-birch-2-whole.png | silver-birch | 2 | whole |
| stills/silver-birch-2-bare.png | silver-birch | 2 | bare |
| stills/silver-birch-3-whole.png | silver-birch | 3 | whole |
| stills/silver-birch-3-bare.png | silver-birch | 3 | bare |

## Ash

Profile, references and a closed-schema `species.json` are complete.
Required capability `pinnate-compound` is unmet. The fixture is not a
catalogue species and was not measured or rendered as a passing tree.

## Undecided / owner

- Visual verdicts on the twelve stills (R3). The spec closes only on
  accepting owner verdicts.
- Ash template after fn-33 lands: leaflet count 7–13, whole-leaf length
  0.20–0.30 m, placements vs instances.
- Growth-trait calibration through time waits for fn-30's method. Height
  by age is recorded on each ready profile.
- `tests/crown_reference.rs` is an ignored FN6 pin list and does not
  include oak or spruce; it was not extended.
