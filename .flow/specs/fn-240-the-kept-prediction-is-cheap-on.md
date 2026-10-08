# The kept prediction is cheap on fractional rows

## Goal & Context

fn-238 (2026-10-09, SOLVE.md) measured the ordinary tree's single kept prediction at 80 at 0.88 s and its grown one at 7.4 s, against 0.03-0.06 s kept for beech, oak and spruce: species with fractional rows (the ordinary tree, and every blend that inherits its rows) make the closed form's kept prediction costly. fn-238 made the grown count a running total and cached a walk's ends, but a walk frame at 80 with the ordinary tree as an end still costs several seconds in kept predictions. A test pins the predictor to the closed form bit for bit. Host (2026-10-09): after the release.

## Requirements

- **R1 (measure):** where a kept prediction's time goes on fractional rows against whole ones.
- **R2 (fix):** the kept predictor made cheap on fractional rows by the cause R1 names, stated before code, its value equal to today's (bit for bit where the pin holds, or within a stated tolerance with the host's word).
- **R3:** every walk frame at 80 with the ordinary tree as an end solves in at most 1 s natively on one core; gate and npm test green.

## Boundaries

Depends on fn-238. Not a release item.
