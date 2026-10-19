# Smooth bark: lichen, lenticels and peel

## Goal

The bark field was ridges, furrows, plates and flakes (fn-32 making plates cellular). Beech and birch bark are the other kind: thin and smooth, read through colour structure at two scales: lichen patches on the beech, dark lenticel bands and peeling strips on the birch. The B-BASE and S-BARK pairs showed flat grey and flat white trunks. This spec added lichen, lenticel and peel layers as material rows, inert by default, judged with fn-32's instrument on fn-36's matched pairs.

## Decisions

- > user (2026-09-14, on the fn-34 round-3 pairs): "also do we have a texture spec? because the beech bark is clearly not representable with what we have atm."
- Peel reuses fn-32's plate network at horizontal band values plus curl and an inner-bark tint. Relief tied to trunk radius so the thick base darkens. A second wood pipeline so a material without these rows draws without them.
- On the round-16 close-ups the owner said "from close: round 16 looks good", then "it's all still to plasticesque", and placed the matte fix in a later spec.

## Acceptance Criteria

- **R1:** Lichen layer as rows (cellular ellipsoid patches: scale, coverage, tint, strength), footprint-faded; beech sets it against B-BASE — met.
- **R2:** Lenticel layer (horizontal dashes with a shallow groove); birch and beech set it — met.
- **R3:** Peel as fn-32's plate network at band values with curl and inner-bark tint; birch sets it against S-BARK — met.
- **R4:** Wood mesh bytes and fn-24 pins identical with rows on and off; distance, resolution, redraw tests unchanged; frame cost recorded — met (oak +0.006 ms within spread, birch +0.41, beech -0.40).
- **R5:** B-BASE and S-BARK pairs re-rendered with fn-32's receipt; owner verdict in fn-34 — met (birch accepted in fn-34; beech moved to fn-62).

## Result

Fourteen material rows. The beech reads smooth pale grey with lichen, the birch chalk white with lenticels and dark marks. The birch's values were retuned at round 24 against the owner's contrast note (near-black 40% to 6% against the photograph's 11%). The owner accepted the silver birch in fn-34; the European beech's verdict moved to fn-62.

## Open

The "plasticesque" matte finish was placed in a later spec; peeling strips as geometry are not part of this one.
