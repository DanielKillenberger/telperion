# The device and the CPU agree past age 12

## Goal & Context

fn-298 (host, 2026-10-20) measured the CPU and the device on walk frames at age 40. The trees match node for node; the positions do not.

| Tree | Nodes | Largest gap |
|---|---|---|
| Beech to spruce, seed 7, t = 0.2160 | 275,557 matched | 0.342 m in position, 0.035 m in radius (tolerances 1e-3 m and 1e-5 m) |
| Base walk frame, t = 0.1 | – | 0.080 m |
| Base walk frame, t = 0.5 | – | 0.072 m |
| Spruce alone at 40 | one node off | – |

`agree_tests` only holds presets at age 12, so nothing in the gate covers this. The cause is unknown. Two readings are possible:
- **A float difference amplified by growth.** Forty years of feedback through light and space colonisation magnify a difference at the last bit. Then the contract has to say what agreement means past a given age.
- **A defect.** One executor computes something differently, and it shows only on older or blended trees.

After the release (host, 2026-10-20). The film and the package each build one way per frame, so a viewer never sees the two side by side.

## Requirements

- **R1:** measure the gap against age, at 12, 20, 30 and 40, for every shipped preset and for the fn-298 walk frames, seeds 1, 7 and 42. Find the first cycle and the first node where the two diverge past tolerance, and the quantity behind it.
- **R2:** if a defect is found, fix it. Agreement then holds at age 40 to `agree_tests`' tolerances.
- **R3:** if the divergence is amplification of a last-bit difference, state the contract instead.
  - At any age, topology (node counts) agrees exactly.
  - Positions agree to a stated bound that grows with age.
  - The test holds both. The owner sees the bound before it is written in.
- **R4:** the gate is green, with a codex review per task.
