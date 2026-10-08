# fn-226 friction

## 2026-10-08, worker, task 1 (R1)

- **What I was doing:** building R1's measures (crown base live and major, the shed share), which OAK-LIGHT.md reports.
- **What slowed it:** the probe that made OAK-LIGHT.md's measures (`raw/oak-light/oak_light_probe.rs`) sat under the f2b branch's gitignored `raw/`, and was removed from the crate after its run. So the definitions had to be re-implemented from the prose: the stem rule, the 1 % major rule, and grown from the executor's counts.
- **Cost:** about 10 minutes, plus the risk that the definitions drift between reports.
- **What would remove it:** a committed measures example in `telperion-space/examples` that prints OAK-LIGHT.md's columns for any species, age, seed and light. The grown count would be part of what `grow` reports, or of `grow_roots`'s `Fine`.
