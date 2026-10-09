# Three defects found on the way to the release

## Goal & Context

The release map (fn-237) carries three defects found while building the release, each named with its evidence; this spec fixes them before the release PR (host, 2026-10-09).

## Requirements

- **R1 (the device's tree below ground):** fn-234's measurement found the resident device path's tree reaching 0.227 m below ground against the CPU path's 0.040 m (the cherry beech at 40; fn-234 MEASURE.md). Repro as a standalone test first, comparing the lowest point of both paths for the shipped species at 40 and 80; the cause found and fixed so both paths agree within fn-125's screen, or, if the difference is by design (a root flare the CPU path omits), stated with the code that makes it, and the host decides.
- **R2 (the field test's timeout):** `src/field/field.test.ts` "grows every shipped preset, byte for byte the main entry's field" runs about 60 s against its 60 s timeout and timed out twice under load. Measured where its time goes; made to finish well inside its timeout without losing what it proves (every shipped preset, byte for byte, bounds included), for example by growing each preset once and sharing it between the comparisons, or by running presets in parallel; the timeout left at or below 60 s.
- **R3 (the cold-build margin):** fn-235's cold-build test holds the first tree to 5 s besides `grow` with about 0.3 s of margin after fn-231 (MERGE-235.md; it failed at 5.04 s and 5.1 s under load). Measured which pipelines take the time; the margin restored to at least 1.5 s without changing any output (MERGE-235 names one route: the lateral place threshold's logarithm supplied from the CPU table instead of computed in `asks`), contract A parity and the CPU-versus-device tests green.
- **R4:** gate and npm test green; shipped species byte-identical.

## Boundaries

These three defects only. Release blocker for fn-237.
