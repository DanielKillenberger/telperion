---
satisfies: [R3]
---
# fn-257-the-site-grows-its-trees-on-the-device.8 The spruce's cost that is not kernel arithmetic

## Description
The owner accepts the spruce's about 0.51 s "only if that is the clear best performance we can do without improving base calculation speed". After tasks 4 and 5, its warm build and field (`tree::drawn` through `growField`'s query) are reprofiled (`growth_profile` and the field's stage trace), and every cost that is not kernel arithmetic is listed with its share: host waits on the device, the arena's compactions and any move, buffer creation and clearing, CPU tables, copies and readbacks, dispatch overhead, the field's own build (stations, primitives, index) and query. Each with a general remedy (no species branch, no change to the drawn tree) is taken; each without one is named with why. What remains is shown, by the profile, to be kernel arithmetic (fn-285).

## Acceptance
- SPRUCE.md: the profile table (stage, ms, share, kernel arithmetic or not) before and after, each non-kernel cost's remedy taken or the reason none exists.
- The trees unchanged (bytes or stills); the field's R2 tolerances hold; the harness build times no worse on any preset; gate and npm test green.

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
