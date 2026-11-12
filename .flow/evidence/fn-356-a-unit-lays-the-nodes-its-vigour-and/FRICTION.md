# fn-356 friction

## 12 Nov 2026, 05:00, builder (task 1): long waits are moved to the background after 180 s

- **Doing:** waiting in the foreground for the narrow suites and the census, as the worker's rules ask.
- **Slowed by:** the host moves any foreground call to the background after 180 s, whatever its timeout, so every wait becomes a background task and a second wait call.
- **Cost:** a few minutes and several extra calls; no work lost.
- **What would have removed it:** nothing in the repository; a local harness setting, reported only.

## 12 Nov 2026, 04:40, builder (task 1): the shed test's second fixture was hidden behind the first

- **Doing:** the host's decision, the beech at the 0.9 shed bound.
- **Slowed by:** the test stops at its first failing species, so at the base only the beech showed red. With the beech at 0.9, the "beech with long-kept bearers" fixture fails its own 0.9 bound: seed 1 grows 234,037 of 251,179 (0.93). The rule does not touch it (that test's trees are unlit, so no unit is under its quota).
- **Cost:** one extra run; the fixture's bound is left for the host.
- **What would have removed it:** a test that reports every species's share before it fails.
