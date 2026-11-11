# fn-350 friction

## 11 Nov 2026, 11:37, builder: `--girth` sets two values, and the beech's second one is not where its file says

- **Doing:** setting `retained` on every PA of the beech through the still runner's `--girth retained,chi` flag.
- **Slowed by:** the flag also sets `leaf_girth`. I took the beech's chi from `beech.rs` (0), but the preset overwrites it with 1 (`tolerance.rs`). The first beech run therefore changed two values. I caught it because retained 0 gave 58,921 nodes against fn-349's 57,910.
- **Cost:** about 3 minutes and three re-run beech trees (0.1 s each).
- **What would have removed it:** a flag that sets `retained` alone, such as `--retained r`, or a runner that prints each PA's values after the flags apply.
