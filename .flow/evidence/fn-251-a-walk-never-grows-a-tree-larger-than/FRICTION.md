# fn-251 friction

## 2026-10-09, task 1, measuring a walk frame's realized size
- Doing: R1, the oak to birch frame at t = 0.0750, age 80, seed 7, grown realized beside both ends at the frame's light.
- Slowed by: no executor reports a tree's realized grown phytomers; only `grow_stepped`'s `fine_grown + coarse_grown` does, so the measurement grows each tree through it (35-40 s a tree at 80). The oracle grows the ends at the frame's light, which walks with t wherever one end carries extinction (the oak, the ordinary tree), so the ends cannot be cached across a walk's frames: 4 grows a point, about 2 minutes a point at 80.
- Cost: the R3 sweep as written (15 pairs, 2 ages, 3 seeds, fine t) is hundreds of points; at 80 that is hours of CPU per variant measured.
- Would remove it: a realized grown count on the build's own result (`Fine` or `Structure`), and an oracle whose ends are the walk's own ends at their own light (cacheable per walk).
