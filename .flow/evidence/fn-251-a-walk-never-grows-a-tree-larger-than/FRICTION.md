# fn-251 friction

## 2026-10-09, task 1, measuring a walk frame's realized size
- Doing: R1, the oak to birch frame at t = 0.0750, age 80, seed 7, grown realized beside both ends at the frame's light.
- Slowed by: no executor reports a tree's realized grown phytomers; only `grow_stepped`'s `fine_grown + coarse_grown` does, so the measurement grows each tree through it (35-40 s a tree at 80). The oracle grows the ends at the frame's light, which walks with t wherever one end carries extinction (the oak, the ordinary tree), so the ends cannot be cached across a walk's frames: 4 grows a point, about 2 minutes a point at 80.
- Cost: the R3 sweep as written (15 pairs, 2 ages, 3 seeds, fine t) is hundreds of points; at 80 that is hours of CPU per variant measured.
- Would remove it: a realized grown count on the build's own result (`Fine` or `Structure`), and an oracle whose ends are the walk's own ends at their own light (cacheable per walk).

## 2026-10-14, task 1, re-measuring on F4 after fn-255
- Doing: R2/R3's re-measurement, 80 frames plus 37 probe frames through `examples/walk_size.rs`.
- Slowed by: the example grows both ends again at every frame, three grows a frame, though every preset now reads the one site light (`presets/space.rs`, extinction 0.5), so the ends are the same trees across a walk. At 80 the oak end alone is 15 s a frame. The heavy wrapper's admission waits added about 2 minutes.
- Cost: about 25 of the run's roughly 35 minutes of CPU went on regrowing ends.
- Would remove it: grow each end once per walk while the frame's light equals the ends' light, which holds for every shipped walk now.

## 2026-10-14, task 1, the exception test's first green run
- Doing: running `walk_sizes` (the nine exceptions and the teaser frame) by itself before the gate.
- Slowed by: the first concurrent run was killed at its 8 GB cap. The teaser test then went through `mesh::build`, and two oak-80 trees at once passed the cap. The rerun then waited more than 10 minutes in `telperion-heavy` admission behind another worker's workspace gate, whose cap is claimed in full.
- Cost: about 15 minutes so far, almost all of it waiting.
- Would remove it: the repro now grows through the engine alone, which uses less memory; admission could count a running job's measured peak rather than its whole cap.
