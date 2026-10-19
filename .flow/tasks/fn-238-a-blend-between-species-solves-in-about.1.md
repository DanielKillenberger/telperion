---
satisfies: [R1, R2, R3, R4]
---
# fn-238-a-blend-between-species-solves-in-about.1 Measure the solve, make it cheap, prove the frames

## Description
As the title says; the spec states the requirements.

## Acceptance
- The spec's R-IDs named in `satisfies`.

## Done summary
The blend's size solve now reads the grown phytomers as a running total (`telperion_space::expected_grown`, built on the first-unit recursion; it matches the count table's sum within 1e-12 of the log, 7e-15 at 80). It also keeps a walk's ends' predictions, keyed exactly, so only a walk's first frame pays for them.

Proof:
- 161 of the walk digest's 165 factors are bit-identical. The other 4 moved by at most 3.9e-16 in their log; on the strips the largest move is 2.0e-15 at 80. The digest is re-pinned.
- The nine moved strip frames render byte-identical PNGs.
- The family digest is unchanged, so every shipped species is byte-identical.
- Frame times, best of 3 on one core:
  - 40: 0.04 to 1.38 s, against 0.29 to 4.03 s before;
  - 80 without the ordinary tree: 0.14 to 3.48 s, against 2.72 to 9.16 s;
  - 80 with it: 0.40 to 10.1 s, against 13.1 to 74.6 s.
- The 1 s bar moved to fn-240 (host decision (c)): six frames remain over it, listed in SOLVE.md.

Gate: render half and workspace half green; npm test 143/143 green.

Follow-up (review P2, contracts): `expected_grown` recurses one native stack frame per cycle along an axis, as `expected_length` does. An extreme age (about 1e6 cycles) could exhaust the stack. The reviewer traced this from code and did not run it. A heap-backed stack, as the kept predictor uses, would remove it.

## Evidence
- Commits: 8e80ac1afd1459622103ad80ef84a8285f1eb74f, 8b64433c7dd449eac7020c3a2ef621cdc55c894e, 7b593effd057eb520d2a1675bfcac27394b71a44, b95ae1a294f7878b0905242aa799d940ccfdeaf7, a0c0dc9a94d2107443e4aea4774c5374117c3291
- Tests: flock gpu.lock cargo test --profile ci -p telperion-render --no-fail-fast, cargo test --profile ci --workspace --exclude telperion-render --no-fail-fast, npm test
- PRs:
