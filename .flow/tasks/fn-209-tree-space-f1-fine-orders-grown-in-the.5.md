---
satisfies: [R1, R2]
---
# fn-209-tree-space-f1-fine-orders-grown-in-the.5 Fine growth stays on the device, and two open items

## Description
Host decisions after task 4 (2026-10-06; GPU-GEOMETRY.md). (1) The growth wave loop moves onto the device (task 4's option a): the next wave's buds expanded on the device by prefix-sum compaction and indirect dispatch, the overflow re-run as a device queue; nothing per wave read back (Astra change 3), only a completion count. (2) The sag lay's frame correction does nothing for fine chains in the CPU reference (Bounded::settled runs before their phytomers are laid), and the GPU mirrors it. Check it against whole-tree growth (what today's engine does at the same junction); if it is an accident, fix it in both executors and show its size; if it is right, record why. (3) Host decision 3, built: a fine root whose conservative extent bound from its attachment can reach the ground is grown on the CPU reference executor; the GPU's D19 flag becomes a test that fails if any such root reaches it.

## Acceptance
- No per-wave readback on the GPU path (shown by the code path and a counter); topology identical by lineage and geometry within the screen, as task 4. - Spruce and beech at 80, seeds 1 and 7: the whole fine stage warm and cold against the CPU executor on 1 and all cores. - The frame correction's finding recorded, with its size on the four species if changed. - Ground-reaching roots routed to the CPU, with a test. - Workspace gate green.

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
