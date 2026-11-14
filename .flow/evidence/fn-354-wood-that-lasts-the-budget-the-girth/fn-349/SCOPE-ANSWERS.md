# fn-349: the host's answers to SCOPE.md, 11 Nov 2026

Each is the host's design decision; none is the owner's ruling.

1. **A species with no value is not charged at all,** as the spec's R3 says. The host's earlier answer in fn-346 (such a species keeps today's per-metre charge) was a probe's convenience and would be a second way of charging. So `Pencil::wood` as the balance reads it, the frozen sum and `closed()` are deleted. Ordinary, laurelin and telperion then grow with no balance; their trees at their preset ages are drawn beside today's before anything merges, and that check belongs to the host's read, not to this build.
2. **Stored as the reciprocal of the potential height, per metre, one value for the species** and not per growth stage. 0 is the neutral value (not charged), and the charge rises by degree with it, so a blend or a walk between two species passes through no large charge. The preset's comment gives the height in metres and its source. Validation allows 0 to 1 (a potential height of 1 m or more). If the species struct has no place for a species-level scalar, say so and stop before inventing one.
3. **The path is one f32 per laid phytomer in `Pencil`,** the wood's own length from the root collar as the rough layout lays it. The device gets the same column when the device is built.
4. **The whole-tree copy (`grow/balance.rs`, `grow::whole`):** delete it with the tests that read it if what those tests held was the cycle's frozen and remembered sums against a whole pass, which no longer exist. If a test there holds something that still exists (the cycle's sums against a plain whole-tree sum of the new rule), the copy takes the same rule and stays for that test alone. Say which you found.
5. **The device is in this spec and not in this dispatch.** Build steps 1 to 4 now. The gate cannot be green until the device agrees, so this dispatch does not run the gate; it runs the narrow CPU tests, the two sub-second guards, and R5 on the CPU.
6. **Yes:** a lateral whose light and charge are both nothing has a balance of 0. It is part of the rule ("and 0 where both are 0", fn-346 R5-ANSWERS.md).
7. **The value leaves `tolerance.rs`'s copying entirely.** It is each species' own trait and is never copied from the oak.

## This dispatch

Steps 1 to 4 of your build order, on this branch, as product code with the R4 tests:

1. The species value and its catalogue row, the presets, the generated tables.
2. The path in `Pencil`, tested to survive compaction and a pause.
3. The CPU rule (the charge, the sums that take only the children that pay, the main line tested, balance 0 when empty), the frozen sum deleted, the whole-tree copy resolved, the three R4 tests.
4. The captures, so the workspace compiles. The device's WGSL is not changed here; say which device tests are expected to fail until it is.

Then R5 on the CPU under `telperion-heavy`: the oak on seeds 1001 and 1007 at 40, 120 and 300 against the probe's SR census (`SQUARE-RESULT.md`), held phytomers and living tips within 1% or the difference explained.

Commit as you go, each commit compiling with its narrow tests green. Stop by 09:15 Zurich whatever is done, with the tree committed and clean, and return: what is built, what R5 showed, which tests fail and why, and what the device step needs. No gate, no review, no merge in this dispatch.
