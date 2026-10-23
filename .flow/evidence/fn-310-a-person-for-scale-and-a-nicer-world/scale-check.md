# Scale check: the person beside Telperion at 80 (fn-310 task 2)

The host's question (2026-10-23): on the sheet the person looked about 10 px tall beside a tree about 425 px tall, which would imply a 70 m tree. Measured through code at 1280x720, using the default hero shot, the generator's bounds for Telperion at age 80, seed 7 (the `TREES` table in `camera.rs`, held to `mesh::build` by `the_bounds_table_is_the_generators`), and `person::stand` and `person::person` projected through `hero_pose(..).view_projection(16/9)` (a scratch test that was then removed):

| Quantity | Value |
|---|---|
| Tree height (bounds, -0.18 to 46.28 m) | 46.46 m |
| Crown width, x by z | 56.28 m by 51.19 m |
| Foot | (22.40, 0, 20.89) m, 30.63 m from the trunk |
| Eye | (77.59, 42.81, -33.08) m; foot 88.27 m from the eye, 80.77 m deep along the view axis |
| Person's mesh height after the transform | 1.750 m (y from 0 to 1.75) |
| Person on screen, foot to top projected | 20.5 px |
| 1.75 m at 80.77 m depth from the 38 degree fov | 22.7 px (on-axis estimate; the camera looks 13 degrees down, so a vertical segment off axis is foreshortened) |
| Trunk axis, base to top projected | 548 px |

The tree-to-person ratio on screen is 548 / 20.5 = 26.7, and in metres it is 46.46 / 1.75 = 26.5. The scale is honest, so nothing was changed. The 10 px came from the sheet: each 1280 px panel is shown at about 780 px in the 2000 px wide preview, so 20 px reads as about 12 px there. Telperion at 80 is 46 m tall, and from the hero distance a 1.75 m person is that small beside it.
