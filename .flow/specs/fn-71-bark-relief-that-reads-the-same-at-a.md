# Bark relief that reads the same at a distance

## Goal
The bark's relief changed look with distance: a trunk drawn at 4x the hero distance and reduced differed from the near draw by more than the relief tests allow. fn-55 measured it (4x series 3.134/255 against a 3.0 bound once the old sheen was gone), raised the bound to 3.25 and shipped the grain rows off. The owner had noticed the drift before fn-55 and gave this spec the cause: fix it in the filtering of relief and grain, a renderer term every table shares, never a species row.

## Decisions
- Owner, 2026-09-18: the drift at distance is a real problem, seen before this spec, and not fn-55's to fix.
- Owner, 2026-09-18, R4 rejected round one (harness, mature path): "it still looks much less detailed from a distance. There's a noticeable change in fidelity which shouldn't be the case. Fidelity should go down to a degree where it's not noticeable at that distance."
- Owner, 2026-09-18, R4 rejected round two: "there's still a jump where the relief visibly disappears. It's a bit further away now but still there." Decision: a fade window is a jump when a walk crosses it; relief may only leave by the box integral of its own height over the footprint, with no separate attenuation.
- Owner, 2026-09-18, R4 accepted round three, every amplitude fade gone: "yea much better". Round four then bought back cost under that picture, the flat footprint sweep as invariant and no fade returning.
- Measurement method: a camera-walk sweep measured framing, not shading; the sweep walks the footprint alone (same camera at 1/f size, registered pixel for pixel) via `fn71/sweep.py`.
- Boundaries: no new bark pattern, no tone-map change, no species branch.

## Acceptance Criteria
- **R1:** 4x distance series at or under 3.0/255 on oak and spruce, bound restored to 3.0 with the 3.25 reason recorded — partial: oak 4x 2.532 (from 3.134), bound 3.0 again; the spruce is held by the grazing fixture (2.70) because its twig shadows read 6.2/255 on plain wood in any strip.
- **R2:** Every resolution test writes mean and p95 to a receipt on a green run — met (`fn71/resolution/`).
- **R3:** Grain rows carry a value on beech and birch that reads as grain at 1440 tall, smooth-bark bound held at 3.0 — met; the lichen keeps its amplitude fade.
- **R4:** Close-ups re-rendered near and at 4x; the owner judges in the harness — met on round three ("yea much better").
- **R5:** Redraws byte-identical, untouched presets keep pins, frame cost recorded — met.

## Result
Two `wood.wgsl` terms: `facets = inverseSqrt(1 + variance)` scaling sun and sky by the mean cosine over lost slope variance, and `bark_rectified(t, spread)` replacing the kinks with a box mean across the shading cell. Oak mean/255 at 2x/4x went 2.463/3.134 -> 2.342/2.532. Round three removed every amplitude fade (relief, plates, dashes, grain leave by box integral); the footprint sweep is flat within 0.03 between adjacent factors. Receipts: oak 1x 1.78, 2x 2.23, 4x 2.84, beech 1.06, birch 2.54, spruce/oak grazing 2.70/2.89.

Hero-frame cost (seed 7, 1600x1000, ms p50) round 2 / 3 / 4: oak 5.9 / 10.0 / 8.91, birch 4.0 / 15.0 / 7.00, beech 11.4 / 19.6 / 14.59. Round four shares the lenticel groove per fragment, reads only the dash rows a box touches, caps the grain window at three cells and sizes shading cells per axis. The remaining ~3 ms per preset is the oak's plate network and the birch/beech dashes and grain window, in exactly the band the owner asked to keep; left standing rather than bought back with a fade. Commit d9143411.

## Open
A spruce distance fixture with a pose or mask clear of its own shadows; a box-prefiltered lichen.

## Friction
- Codex quota out again -> session model; fix is a routing line naming the fallback once a day.
- Resolution numbers printed only on failure -> R2's receipts added.
- Command guard blocked compound shell blocks and shader overwrites -> local setup, not specced.
- Camera-walk sweep measured framing, not shading (two misdirected rounds) -> sweep walks the footprint alone.
- Fade contracts pinned as mechanism in four tests -> re-pinned as convergence; lesson: state contracts as convergence.
- Dash cell window on the wrong axes -> caught by `smooth_means`, fixed.
- First no-fade build cost 20x the frame, found by four timing rounds -> lesson: per-term cost line.

## Evidence
- `.flow/evidence/fn71/resolution/` (receipts the resolution tests write and read)
- `.flow/evidence/fn71/sweep.py` (footprint sweep metric, used by `bark_sweep_evidence.rs`)
