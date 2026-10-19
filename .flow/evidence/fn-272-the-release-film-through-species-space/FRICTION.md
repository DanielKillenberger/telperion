# fn-272 friction

## 2026-10-14, task 1 worker: the device build refuses walk frames

- **Doing:** the first cut, its walks built on the device (`Generator::prepare`, resident), as the spec's goal names ("one device build per frame").
- **Hindered:** the first walk frame, beech to spruce at 40, seed 7, t = 0.00037, was refused: "the build passed its sizes at cycle 9: UnitRounds (0x1)". A probe found the same refusal at t = 0.01 and 0.1, and at seed 1; t = 0.5, 0.9 and both ends build. The Laurelin at 20 built on its own is refused too ("Given (0x1)"). The units-round slack (`build_size.rs`, `ROUND_SLACK`) was measured on shipped presets at their default ages only.
- **Cost:** about 10 minutes and one 2-minute run lost (the beech segment rendered, then the walk stopped it).
- **Would have removed it:** a test building walk frames and other ages on the device, not only shipped presets at their default ages. The film's walks now use the walk path `headless --walk` uses (`mesh::build`); the refusal is reported to the host as a defect of its own.

## 2026-10-14, task 1 worker: the walk path rebuilds the whole tree on the CPU each frame

- **Doing:** the cut's walks through `mesh::build`, as `headless --walk` builds them.
- **Hindered:** 2.9 s a frame at 40 (beech to spruce, 257 s for 90 frames; spruce to birch 306 s) and 13 s a frame at 80 (Laurelin to Telperion, 1,171 s for 90 frames). The Laurelin and the Telperion share one species, so that walk regrows the same 80-year structure 90 times to change its dressing. The cut took 36.5 minutes, 32 of them in walks; growth segments took seconds (beech 40 years 3.4 s build).
- **Cost:** about 32 minutes of a heavy slot for one cut.
- **Would have removed it:** the device build for walk frames (0.2-0.6 s a frame by the probe, about 15 times faster), once it stops refusing them (entry above); or a walk whose species does not move reusing its grown structure.

## 2026-10-14, task 1 worker: the overlay's map point is solved before the first frame

- **Doing:** the film's first frames.
- **Hindered:** the map point of each walk frame goes through `blend::families`, which runs the size solve the map then discards; about 1.5 minutes before the first frame, for 510 walk frames.
- **Cost:** 1.5 minutes a run.
- **Would have removed it:** a public walk of the rows without the size match.

## 2026-10-14, task 1 worker: waiting for room on a shared desk

- **Doing:** the oracle run and the gate.
- **Hindered:** `telperion-heavy` held each for want of RAM while another worker's gate ran: the oracle 10 minutes, the gate 63 minutes without starting (room 6-7 GB for 9, the desk busy with other projects), so the worker stopped it and returned to the host.
- **Cost:** about 75 minutes of wall time; the gate is not run.
- **Would have removed it:** nothing in this task; a queue position or an estimate in the wrapper's waiting line would let a worker plan around it.

## 2026-10-19, task 1 rework: a clip declared at the whole film's size

- **Doing:** the first short clips after the merge (two 1-second device walks), under `telperion-heavy 10 --gpu 5`, the declaration cut 3 used.
- **Hindered:** the desk had 5-9 GB of room and the job waited 10 minutes for 10 GB it does not need (cut 3's peak resident was 6.4 GB, in the CPU walk at 80 that the device build now replaces).
- **Cost:** about 10 minutes of wall time.
- **Would have removed it:** a measured peak per run kind (fn-270 and fn-277 proposed the same); here, declaring the clip's size, which is what this rework now does.

## 2026-10-19, task 1 rework: the review receipt lived in /tmp and was lost

- **Doing:** the codex impl-review of the rework.
- **Hindered:** `review-route` stopped with `lost_receipt`: the 2026-10-14 NEEDS_WORK cycle's receipt (`/tmp/impl-review-receipt-*`) was gone, so flowctl could not tell which findings were open. The findings were recovered from codex's own session logs (`~/.codex/sessions/2026/10/14/rollout-2026-10-14T11-12-10-*`): oak to palm's six oracle frames (made fn-273's tenth named exception by the host, 2026-10-14), stale trailing stills (fixed at 781d4ea03), colliding labels (fixed at 8478aa16e). None was left open, so the cycle was reset (`flowctl spec reset-review-rounds --task .1`, under the owner's allowance to reset with a recorded reason) and a fresh fan-out reviews the whole task.
- **Cost:** about 5 minutes.
- **Would have removed it:** a receipt kept under the repository's ignored state (or `$XDG_STATE_HOME`), not `/tmp`, which does not survive a reboot.
