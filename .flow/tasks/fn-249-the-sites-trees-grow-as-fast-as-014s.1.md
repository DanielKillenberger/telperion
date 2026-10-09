---
satisfies: [R1, R2, R3, R4, R5, R6]
---
# fn-249-the-sites-trees-grow-as-fast-as-014s.1 Cache the rows, exact division, placeholder framing, deposits, measured

## Description
TBD

## Acceptance
- [ ] TBD

## Done summary
R1 to R4 are in. Each change was measured in Wasm, and none changes a tree: 24 of 24 site field-answer hashes and 16 of 16 space_hash trees are the same as at 4b0f7b01. The full write-up is .flow/evidence/fn-249-the-sites-trees-grow-as-fast-as-014s/RESULT.md.

- R1: the closed-form rows are kept for the 4 most recently used keys (species Debug text, age, mode). The birch's warm grow falls from 741-813 ms to 105-172 ms. Cold is unchanged. The site does not get warm hits today, because growField instantiates a fresh Wasm instance per tree. This is a host decision; see FRICTION.md.
- R2: I256::quotient is now Knuth D on 32-bit limbs. The tests compare it against the old bitwise routine on boundary cases (the add-back branch is reached) and 400k random cases. The oak gains 125-250 ms. GPU quotient parity is green.
- R3: measured first, the placeholders cost about 51/30/10-21 ms of place for spruce/oak/birch. The unbent lay now skips laterals that hold no wood and bear none (place_bearers), but only for sagging species. Spruce gains 22-27 ms. An empty continuation must still be laid: its base is its bearer's far tip in sag::gather.
- R4: Grid::corners uses arrays instead of Vecs. The oak's shade span drops by about 25 ms. The gathering buffers (40 allocations per tree) were left as they are.
- R5, the remaining gap against 0.1.4 cold: oak 2.5-4.7x (550-910 ms), birch 2.3-2.8x (490-700 ms; 0.5-1.1x warm), spruce 1.3-1.5x (80-105 ms). Load was 3.8-5.3 on 32 cores.
- Follow-ups: instance reuse in growField, to give the birch its warm path; grow/settle.rs has the same unbent/bent pair, and place_bearers could serve it too.

stage: impl-review - ran (codex, 3 draws SHIP)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 31766be5a56651e1ad2f65eeec4a580156e3f50d, 473fae958437937352ba022bc2f3de5c17891732, 3a4737411406b355604a76a266f65a77c1e81960, 3cc60b48f58ae88d45dc03f87ae2a3ea17255619, 7735b2506fc2eeba3dd76de41cc38eca17553b09
- Tests: flock gpu.lock cargo test --profile ci -p telperion-render --no-fail-fast, cargo test --profile ci -p telperion-space|telperion-field|telperion-wasm|telperion-jev --no-fail-fast, cargo test --profile ci -p telperion-core --lib, --test (all 13), --doc; --no-default-features --lib, npm test (14 files, 155 tests)
- PRs: