# Implementation candidate

The owner selected qualitative reference matching on 2026-09-19. Reference photo widths and calibrated RGB remain unknown. This candidate has no exact photo-score claim.

Host design: retain the existing height field, plate network and footprint integrals. Correct the relief coordinate walk to move away from the eye when descending below the virtual crest plane. Refine the initial walk with two damped height evaluations at the same footprint; this is an approximate intersection refinement, not a complete occlusion ray marcher. The tangent/grazing bound and geometry remain unchanged.

Enable the existing filtered grain on oak (2.5 mm, strength 0.20) and spruce (1.8 mm, strength 0.15) to break up the smooth faces. These are artistic candidate values, not measurements inferred from the unscaled photographs. Change oak bark RGB from (0.225, 0.218, 0.198) to (0.185, 0.175, 0.158), spruce from (0.147, 0.078, 0.045) to (0.20, 0.175, 0.148). Leaf rows, plate-network rows and other preset tables are unchanged.

Validation uses the existing distance, footprint, resolution and redraw bounds plus an analytic production-parallax test. Replay the eight recorded fn-32 camera/light combinations on the current direct mature generator, grouped as four images per species. Historical fn-32 differences include intervening renderer and generator changes; only the fn-42 flat baseline is the immediate shading comparison.

Performance baseline unavailable: before the candidate, the GPU reported 11% utilization. No valid before/after regression is claimed. R4 remains pending a controlled idle interval. Final visual acceptance R5 belongs to the owner.

The initial oak grain strength 0.45 failed the unchanged 4× distance agreement bound (3.304683/255 versus 3.0). With oak grain disabled the corrected parallax passed at 2.492250/255, isolating the regression to the grain. The second candidate reduces oak grain strength to 0.20; no shader filtering or test threshold changes.

Spruce grazing agreement also rejected the first grain strength (3.223863/255 against 3.0). Reduce spruce grain to 0.15 before rechecking; oak stays at its passing 0.20.
