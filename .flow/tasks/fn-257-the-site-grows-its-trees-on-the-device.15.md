---
satisfies: [R8c, R4]
---
# fn-257-the-site-grows-its-trees-on-the-device.15 The spruce's site field and held memory, paired against today; gate and npm test

## Description
R8c's proof and the gate.

**Size:** S
**Files:** `.flow/evidence/fn-257-the-site-grows-its-trees-on-the-device/R8.md`
**Touches:** [.flow/evidence/fn-257-the-site-grows-its-trees-on-the-device/**]

### Approach
- Paired warm (16 interleaved): the spruce's site field (grow + 36³ grid) at the release tip before R8 against after; oak and birch too, seeds 1, 1234, 5678, against 0.1.4 (RESULTS.md's table), naming any species still over.
- Held memory per tree after the read-out (the grid only) against RESULTS.md's field MB.
- Gate once: `telperion-heavy 12 --threads 2 -- cargo test --profile ci --workspace --no-fail-fast`; npm test.

## Acceptance
- [ ] R8.md: paired spruce site field before → after, oak and birch against 0.1.4 per seed, held memory per tree.
- [ ] Gate and npm test green, counts in the done summary.

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
