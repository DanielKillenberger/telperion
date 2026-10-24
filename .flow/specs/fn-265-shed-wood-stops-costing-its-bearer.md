# Shed wood stops costing its bearer upkeep

## Goal & Context

Found by fn-255 (CYCLE-COST.md, 2026-10-11): a shed branch keeps charging its bearer's upkeep in the balance for the rest of the run, because recorded wood never falls. Botanically, shed wood leaves the tree and costs nothing after it falls. Measured with a throwaway switch: the oak at 40 keeps 1.7-2.0% more phytomers, the oak at 80 10.5% more, its live crown base falls from 5.1 m to 2.4 m and its crown widens from 18.8 m to 23.5 m along x; the ordinary tree is unchanged to the bit. Changing it moves the shipped oak visibly, so the owner's verdict decides.

## Requirements

- **R1 (checked against code):** where the balance charges upkeep for wood, how shed wood stays in that charge, and which presets it reaches (inferred today: those whose laterals' remembered balance reaches a decision).
- **R2:** upkeep is charged for living wood only: a shed subtree's charge ends at its shedding cycle, on both executors, through fn-255's one cycle.
- **R3:** stills of the oak at 40 and 80 (whole, mid, close) beside the current ones, viewed; per-preset kept counts before and after; the owner's verdict; gate and npm test green.

## Boundaries

The upkeep of shed wood. After fn-255/fn-256 land. Not on the release map until the owner says.
