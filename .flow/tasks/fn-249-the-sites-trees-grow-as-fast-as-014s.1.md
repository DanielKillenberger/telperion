---
satisfies: [R1, R2, R3, R4, R5, R6]
---
# fn-249-the-sites-trees-grow-as-fast-as-014s.1 Cache the rows, exact division, placeholder framing, deposits, measured

## Description
As the title says; the spec states the requirements.

## Acceptance
- The spec's R-IDs named in `satisfies`.

## Done summary
R1 to R4 in; none changes a tree (24 of 24 site field-answer hashes, 16 of 16 `space_hash` trees equal to `4b0f7b01`). Write-up: `.flow/evidence/fn-249-the-sites-trees-grow-as-fast-as-014s/RESULT.md`.
- R1: closed-form rows kept for the 4 most recent keys; birch warm grow 741-813 → 105-172 ms; cold unchanged; the site gets no warm hits while `growField` instantiates per tree (host decision).
- R2: `I256::quotient` is Knuth D on 32-bit limbs (boundary and 400k random cases); oak −125 to −250 ms; GPU quotient parity green.
- R3: placeholders measured first (~51/30/10-21 ms of place); the unbent lay skips empty, childless laterals for sagging species; spruce −22 to −27 ms.
- R4: `Grid::corners` on arrays; oak shade span ~−25 ms.
- R5: remaining gap against 0.1.4 cold: oak 2.5-4.7×, birch 2.3-2.8× (0.5-1.1× warm), spruce 1.3-1.5×.
Follow-ups: instance reuse in `growField`; `place_bearers` for `grow/settle.rs`. Impl-review SHIP.

## Evidence
- Commits: 31766be5a56651e1ad2f65eeec4a580156e3f50d, 473fae958437937352ba022bc2f3de5c17891732, 3a4737411406b355604a76a266f65a77c1e81960, 3cc60b48f58ae88d45dc03f87ae2a3ea17255619, 7735b2506fc2eeba3dd76de41cc38eca17553b09
- Tests: flock gpu.lock cargo test --profile ci -p telperion-render --no-fail-fast, cargo test --profile ci -p telperion-space|telperion-field|telperion-wasm|telperion-jev --no-fail-fast, cargo test --profile ci -p telperion-core --lib, --test (all 13), --doc; --no-default-features --lib, npm test (14 files, 155 tests)
- PRs:
