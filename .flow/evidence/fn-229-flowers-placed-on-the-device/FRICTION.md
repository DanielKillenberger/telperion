# Friction, fn-229

## 2026-10-08, worker, task 1: R1's inputs are not on the device

- **Doing:** reading the device path, before any edit, to place florets from the dressed fine nodes.
- **What slowed it:** R1 assumes a dressed fine node carries what a bloom is drawn from. It does not.
  - A floret is keyed by its node's `identity.birth_order()`, the phytomer's u64 key (`space.rs`, `NodeIdentity::lineage(p.key())`; `bloom.rs`, `inflorescence`).
  - The device holds that key only in the settle's packed phytomers (`ph(p, 0..1)`). The laid record (`PL` 24) stores no key, and neither do the drawn record (`CV` 26, word 24 is the u32 `Tree::key`) or the sweep's node (`NW` 27).
  - `dress.wgsl` already binds 8 storage buffers, the default per-stage limit (`device.rs:180`), so it cannot read the packed phytomers without a raised limit.
  - No device table carries `PaState::flowering`.
  - "The same pass that places leaves" is `generation/place.wgsl`, which the CPU block's leaves share. It runs over stations, not nodes, and the dressed and swept node buffers are dropped before it runs (`resident.rs`).
  - Florets need a count that drops sizes under `LEAST`, and a reference box before they are packed. fn-125 names "its own compaction".
- **Cost:** about 25 minutes of reading and about 150k tokens. No build was made and no GPU time was used.
- **What would have removed it:** a spec that names how the phytomer key and the flowering share reach the dressed nodes, and which pass compacts and boxes the florets. Those choices are listed for the host in the task's escalation.
