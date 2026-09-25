## Conversation Evidence

> owner (2026-09-25), on fn-149's offline palm check: "yes capture it"

## Goal & Context
<!-- scope: business -->

fn-149's Start stage derives the date palm's first tree from its sources alone. It agrees with the shipped palm on height, colours and frond length, but draws the leaflets at their measured size, while the shipped palm draws them about three times longer and eight times wider. Either the renderer needs oversized leaflets for a palm to read as one, or fn-80's tuning moved the palm away from its sources. Until that is known, every palm-like species the runner starts from literature begins far from what tuning will accept, and tuning spends its rounds walking back to the shipped values. [paraphrase]

## Architecture & Data Models
<!-- scope: technical -->

**Checked 2026-09-25 on the fn-149 branch (`be78faee`), from `.flow/evidence/fn-149-one-species-runner-set-stages-from-a/palm-offline-start.md`.** [checked]
- The palm's recorded profile gives leaflet length 0.4572 m (A1, "18 inches") and leaflet width 0.02 m (P4, "2cm").
- `crates/telperion-jev/data/profile-to-preset.json` maps them with `ratio` over `/canopy/size` when `/canopy/leafletCount` is above 1: Start gives `/element/length` 0.1946 and `/element/width` 0.0085.
- The shipped `date-palm` preset has `/element/length` 0.6197 (−69% from Start) and `/element/width` 0.0725 (−88%). `/radii/trunkRadius` differs by −23% on the same kind of ratio row; `metrics.crown_width_m` maps to no dial for the palm.

**Unknown.** [unknown]
- Whether `/element/*` scales a leaflet or a cluster of them on a frond, and so whether the ratio row's basis ("a leaflet is the element") is right.
- Whether the shipped values were tuned for legibility at the supported views, or drifted.

## Acceptance Criteria
<!-- scope: both -->

- **R1:** A render of the palm at its measured leaflet size, beside the shipped one at the supported views, answers whether the measured size reads as a date palm; the answer and the stills are recorded. [inferred]
- **R2:** What `/element/length` and `/element/width` scale on a pinnate frond is written down from the code, and the ratio rows in `profile-to-preset.json` match it, so Start's leaflet values equal the value that draws a leaflet of the measured size. [inferred]
- **R3:** If the measured size does not read, the reason is named as a renderer or level-of-detail gap spec; if it does, the shipped palm moves toward the measured size with the owner's look. [inferred]

## Boundaries
<!-- scope: business -->

- Not the species runner (fn-149) or the parameter table (fn-152). One species' leaflet scale and the rows that derive it.

## Strategy Alignment

- Serves "Fidelity": a species starts from what its sources measured. [strategy:Our approach]
