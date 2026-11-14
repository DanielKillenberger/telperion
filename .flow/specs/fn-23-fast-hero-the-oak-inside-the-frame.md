# Fast hero: the oak inside the frame budget

## Goal

The owner wants "a beautiful performant renderer in the browser that is botanically accurate to sota degree asap", and a website tree "growing from sapling to full tree and sway in the wind. Looking basically realistic at 60fps", reached in "small specs that are highly likely achievable". fn-22 measured the oak hero at 18 ms against the strategy's 2 ms; this spec draws the same oak with far fewer triangles by choosing, per leaf and per frame, the cheapest nested level of the one leaf element whose outline error is below a pixel — Nanite's selection principle without its streaming and hierarchy.

## Decisions

> user (edit cycle 1): "so how does this differ to fn-13 which took forever and couldn't achieve smooth rendering in the way you described?"
> user (edit cycle 1): "how does this compare to nanite (which I think fn-13 was trying to approximate)"
> user (edit cycle 1): "ok so let's nail this next spec now. Keep it lean as possible."
> user (plan, 2026-09-09): "fn-23 don't review"

- A two-triangle probe first decided the question: vertex-bound (oak 18.08 ms whole vs 1.58 ms as quads, same coverage).
- Levels are nested subsets of the element's sections chosen by outline deviation (base, tip and widest kept); per-instance selection in one compute pass with one indirect draw per level and a bucket for the unseen; stateless, no hysteresis.
- Oak gated, spruce recorded only (an aggregation problem for a later spec); no shading, growth, wind or forest here.

## Acceptance Criteria

- **R1:** Oak hero vegetation including selection p50 at or under 2 ms native, valid — met (1.750 ms).
- **R2:** Oak in the browser holds 60 fps through a ten-second orbit — met (wall p95 10.10 ms, worst 10.10 ms).
- **R3:** Core counts, bounds, skeleton and placement hashes byte-identical; every placement reproduced at every level — met.
- **R4:** Owner judges the oak still against fn-22's — accepted.
- **R5:** Every preset and twenty random sets through one path; spruce recorded — met.

## Result

Native, RTX 3080, seed 7, 1600x1000: oak 1.750 ms (selection 0.057 + vegetation 1.693), orbit 1.912 ms; spruce 32.619 ms, recorded not gated. Against fn-22: oak vegetation 10.6x faster, spruce 3.0x. Browser orbit: oak GPU 2.315 ms, wall p50 10.00 ms on a 100 Hz display. Oak levels run from 4 to 268 triangles per leaf. The still differs from fn-22's in 4.6% of pixels, mean absolute difference 0.83/255.

Owner verdict (2026-09-09): "side by side they look exactly the same so we achieved the goal"

## Open

- Spruce needles need aggregation (later spec); the browser/native spruce gap stays parked.
- A website visitor's machine is not named.

## Evidence

- `.flow/evidence/fn23/REPORT.md`, `PROBE.md`
- `.flow/evidence/fn23/oak-hero.jpg` (verdict still), `spruce-hero.jpg`
- `.flow/evidence/fn23/*-timing.json`, `*-orbit.json`, `probe-*-timing.json`
