# Task .1: the device's frond curve is one function

## Equivalence (AC 2)

The scratch digest test (`t1-scratch-digest.patch`, applied to `resident/digest_tests.rs` and removed after) builds the date palm at ages 12 and 40, seed 1, on the device. It hashes three things: the drawn records once the fronds are hung (`tree::drawn` then `Hanger::hang`), what `Generator::prepare` gives the renderer (the packed leaves and the wood's points, clusters, sections and demand), and the device field (`grow_field`, n = 24). It ran on the base, 33363501d, and on the change. The lines are identical:

```
palm 12 seed 1 drawn: ids 7384 0d4dd249167d34ff
palm 12 seed 1 prepared: leaves 24980c9ff3e0f0c8 points 7376 f8b8d5be7aa162a1 clusters 488 dabd5ae12c78f43c sections cb45d8050947e0ae demand 66194174a0c37db6
palm 12 seed 1 field: 98e572e7eaa60c3d
palm 40 seed 1 drawn: ids 7606 ef499579896069e4
palm 40 seed 1 prepared: leaves 90852933cbba045d points 7919 cda08f9c2a43b771 clusters 504 e51485606e48ae08 sections c088d90a3d18085e demand b050718f802a1a8f
palm 40 seed 1 field: 60bde8d0ce6a6ef6
```

Logs: `raw/t1-digest-base.log`, `raw/t1-after.log` (ignored).

## Station form (AC 4)

The device's `rachises` samples `t = k / count`, then `t *= split` when `split < 1`, for k = 0..=count. The CPU's `rachis.rs` samples 0, then `station(k, count)` for k = 0..count-1. For a mature frond that is `(k + 1) / count`. For a young one it is `Split::segment`'s `((k + 1) / n) · f`. With the leading 0, the two produce the same values. They also use the same order of operations: a division, then a multiply by the split. The only difference is precision (f32 on the device, f64 on the CPU). `k / count · split` therefore already has `station()`'s form, and nothing changed. A frond of one leaflet is the exception: `station()` returns 0 for it. That case does not arise here, because both executors skip an axis of fewer than two leaflets (`count < 2` in `fronds.wgsl` and in `rachis.rs`'s `rachises`).
