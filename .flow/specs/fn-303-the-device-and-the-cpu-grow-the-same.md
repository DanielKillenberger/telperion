# The device and the CPU grow the same rounds frames on the release branch

## Goal & Context

fn-298 R4 (host, 2026-10-21) holds the device's node counts exact to the CPU's on 9 `rounds` walk frames. Only spruce to birch seed 42 frame 56 was moved to fn-300. On fn-298's own branch that held.

On F4 after the fn-299, fn-298 and fn-301 merges (bb20f3877), frame 41's refusal hid the test's later frames. fn-302 makes frame 41 build, and then `the_rounds_frames_have_the_cpus_counts` fails on four frames, measured by fn-302's conductor on 2026-10-21:

| Frame | Instances, device / CPU | Nodes, device / CPU |
|---|---|---|
| spruce to birch, seed 7, frame 63 | 187,997 / 187,999 | 382,987 / 382,999 |
| oak to palm, seed 1, frame 36 | 157 / 160 | 20,205 / 20,250 |
| oak to palm, seed 1, frame 37 | within 1 | 19,985 / 20,032 |
| oak to palm, seed 1, frame 48 | within 1 | 4,785 / 4,910 |

None of the four retries under fn-302, and their device digests are identical to bb20f3877's. Which merge moved them is unknown. Candidates:
- fn-299 changed the blended timing rows, and with them the frames' trees.
- fn-301 changed the lay walk and the rest on both executors. Lay positions feed growth.

These are release film frames, so the spec joins the release (fn-237).

## Requirements

- **R1 (bisect, measured):** run the four frames' counts at fn-298's tip 55c19adef, at c3ed89f12 (fn-299 + fn-298) and at bb20f3877 (+ fn-301), each in its own target directory. Name the merge that first breaks agreement. Then find the first cycle where the device's node count leaves the CPU's, and the quantity behind it.
- **R2 (fix):** if one executor computes a quantity differently, fix it so the two agree. Then all 9 `rounds` frames hold fn-298 R4 again (nodes exact, instances within 1), except frame 56, which stays with fn-300.
- **R3:** if the divergence is fn-300's kind of last-bit amplification rather than a difference in computation, stop and bring the measurement to the host. Do not narrow the test.
- **R4:** output changes only where the fix requires it, and is measured. The gate and `npm test` are green, with a codex review per task.

## Boundaries

- Not fn-300's general agreement past age 12.
- Not fn-302's retry.
