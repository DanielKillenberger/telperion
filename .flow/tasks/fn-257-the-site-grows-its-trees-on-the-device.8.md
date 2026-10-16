---
satisfies: [R3]
---
# fn-257-the-site-grows-its-trees-on-the-device.8 The spruce's cost that is not kernel arithmetic

## Description
The owner accepts the spruce's about 0.51 s "only if that is the clear best performance we can do without improving base calculation speed". After tasks 4, 5 and 9, its warm build and field (`tree::drawn` through the field's query) are reprofiled (`growth_profile`, the field's stage trace), and every cost that is not kernel arithmetic is listed with its share: host waits on the device, the arena's compactions and any move, buffer creation and clearing, CPU tables, copies and readbacks, dispatch overhead, the field's own build and query. Each with a general remedy (no species branch, no change to the drawn tree) is taken; each without one is named with why. What remains is shown, by the profile, to be kernel arithmetic (fn-285).

**Touches:** WIDER, by the profile: `crates/telperion-render/src/generation/` (cycle, resident); evidence `SPRUCE.md`.

## Acceptance
- SPRUCE.md: the profile table (stage, ms, share, kernel arithmetic or not) before and after, each non-kernel cost's remedy taken or the reason none exists.
- The trees unchanged (bytes or stills); the field's R2 tolerances hold; the harness build times no worse on any preset; gate and npm test green.

## Done summary
Profiled the spruce (SPRUCE.md). Taken: CPU tables side by side (spruce -7.3 ms, Telperion -19.9), the reads of one settle submission batched (-4.8/-4.3); the 80.8 MB named (foliage level lists and selection scratch, both read; a level-list layout proposed as its own spec). Not taken, for the host: pooled growth buffers (about 33 ms against about +660 MB peak and 1.6 GB held between builds); the compaction (removing its read measured mixed, sizing it in about 100 MB). Trees byte-identical; codex SHIP.

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 5da269673, 7f08fbd12, 371426fb9
- Tests: drawn_warm paired 16 runs, drawn_hashes 24 trees
- PRs: