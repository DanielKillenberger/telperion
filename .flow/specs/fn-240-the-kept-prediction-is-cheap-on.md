# The kept prediction is cheap on fractional rows

## Goal & Context

fn-238 (2026-10-09) measured the ordinary tree's single kept prediction at 80 at 0.88 s and its grown one at 7.4 s, against 0.03-0.06 s kept for beech, oak and spruce: species with fractional rows (the ordinary tree, and every blend that inherits its rows) make the closed form's kept prediction costly. fn-238 made the grown count a running total and cached a walk's ends, but a walk frame at 80 with the ordinary tree as an end still costs several seconds in kept predictions. A test pins the predictor to the closed form bit for bit. Host (2026-10-09): after the release.

## Requirements

- **R1 (measure):** where a kept prediction's time goes on fractional rows against whole ones.
- **R2 (fix):** the kept predictor made cheap on fractional rows by the cause R1 names, stated before code, its value equal to today's (bit for bit where the pin holds, or within a stated tolerance with the host's word). *(read under AGENTS.md "Generator evolution" (owner, 2026-09-20): a measured gain may move the value without asking, with the evidence that rule names; bit-for-bit is the verification aid where the pin holds (fn-293 task 7).)*
- **R3:** every walk frame at 80 with the ordinary tree as an end solves in at most 1 s natively on one core; gate and npm test green.

- **R4 (from fn-238's review):** the running totals (`expected_length`, fn-238's grown total) use one stack frame per growth cycle; built on their own stack as the kept predictor is, or bounded by the validated maximum age with a test.

## Boundaries

Depends on fn-238. Not a release item.
