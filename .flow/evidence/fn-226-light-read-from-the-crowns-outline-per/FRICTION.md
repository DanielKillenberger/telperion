# fn-226 friction

## 2026-10-08, worker, task 1 (R1)

- **What I was doing:** building R1's measures (crown base live and major, the shed share), which OAK-LIGHT.md reports.
- **What slowed it:** the probe that made OAK-LIGHT.md's measures (`raw/oak-light/oak_light_probe.rs`) sat under the f2b branch's gitignored `raw/`, and was removed from the crate after its run. So the definitions had to be re-implemented from the prose: the stem rule, the 1 % major rule, and grown from the executor's counts.
- **Cost:** about 10 minutes, plus the risk that the definitions drift between reports.
- **What would remove it:** a committed measures example in `telperion-space/examples` that prints OAK-LIGHT.md's columns for any species, age, seed and light. The grown count would be part of what `grow` reports, or of `grow_roots`'s `Fine`.

## 2026-10-08, worker, task 4 (R6, the carried walk)

- **What I was doing:** timing the lit oak at 80 under three re-lay modes (full, sagless, none) on a scratch worktree at `caebae5e`, to cost the cheap corrections for the carried `apical_control` walk.
- **What slowed it:** the machine's one-minute load stood at 10 to 31 from other sessions, so the first timings were unusable and the re-run waits on a load under 5 before each tree.
- **Cost:** about 10 minutes of waiting so far, and one discarded set of six oak builds.
- **What would remove it:** a shared lock or queue for timing runs on the owner's machine, as the GPU lock does for the render half, so a timing waits once instead of polling the load per tree. This is a local setup matter, reported, not specced.
