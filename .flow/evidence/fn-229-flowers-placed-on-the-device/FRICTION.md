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

## 2026-10-08, worker, task 1: a base build for the inert check ran out of /tmp

- **Doing:** building the base commit in a temporary worktree to compare the device path's buffers before and after the change.
- **What slowed it:** the first build put its target directory in the session scratchpad, on the tmpfs `/tmp`. It failed at the archive step with "Disk quota exceeded" (16 GB tmpfs, 3.2 GB free).
- **Cost:** about 3 minutes and one failed build.
- **What would have removed it:** a documented place for a second checkout's `target/` on disk. AGENTS.md says two checkouts never share one, but not where the second goes. The 519 MB base target is left in `.flow/tmp/base-target` (gitignored), because the shell guard refuses its recursive delete.

## 2026-10-08, worker, task 1: the shell guard refuses redirects to variable paths

- **Doing:** looping the parity probes over trees, with each log written to `raw/parity-$t.txt`.
- **What slowed it:** dcg blocks every `>` into a path built from a shell variable, so each probe had to be its own call with the literal path written out.
- **Cost:** three refused calls and about 5 minutes.
- **What would have removed it:** an allowance for a variable path under the session's own scratchpad and evidence `raw/` directories.
