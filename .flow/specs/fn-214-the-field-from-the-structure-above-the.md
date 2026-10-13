# The field from the structure above the cut

## Goal & Context

killenberger.com builds its grove from `telperion/field`'s `growField`, then voxelizes it at n = 24-48 cells per tree (about 0.3-0.5 m a cell). On fn-198's branch the field path grows every fine branch first: the oak takes 0.62-1.6 s and the spruce about 3.5 s and 1 GB in the browser, against 0.34-0.40 s and 0.28 s on the published 0.1.4 (fn-198 FIELD-PATH.md). fn-209's cut already splits every tree into the structure above it (1-5 % of nodes) and fine roots. Owner, 2026-10-07: accepted the host's recommendation to probe a coarse field.

## Requirements

- **R1 (probe):** an option carried from the TypeScript `growField` options through the Wasm boundary into the pipeline that builds only the structure above the cut and represents each fine root by its expected occupancy (from the closed form's kept counts and envelope, or its realized totals where they are cheaper), no new species value; the default stays the full field.
- **R2:** for oak, spruce and palm at the site's request (its default age, limb order, n), time and memory in the browser build against 0.1.4 and against the full field; the cube counts and the site's own voxel render beside today's, viewed by the host.
- **R3:** if the look holds, the gate: the coarse field within the time and memory of 0.1.4 for oak and spruce; if it does not, the probe's report and stop.
- **R4:** npm test and the workspace gate green; the site itself is never edited.

## Boundaries

Depends on fn-198 and fn-209. The birch waits on fn-213. Does not change the full field or any drawn tree.

## Closed (host, 2026-10-13)

Superseded by fn-257 (the site grows its trees on the device; owner, 2026-10-11): the site no longer grows a coarse field on the CPU, so the structure-above-the-cut field and its module size are replaced by fn-257 R1-R3 and fn-237 R8.
