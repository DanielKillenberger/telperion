---
satisfies: [R3]
---
# fn-276-the-device-builds-every-tree-the-cpu.2 A build's start leaves room for its first move

## Description
Host 2026-10-14 from SWEEP.md: Sizes::margins starts at margin 1.0 whenever the starting tables fit, with no room for a later move that holds a group twice (the spruce at 62 refused, 63-68 built; 71 and 74-80 at the browser cap). The start margin is chosen so the starting tables plus their largest first move fit the budget.

## Acceptance
Red/green test on the spruce at 62; the spruce 60-80 at TELPERION_DEVICE_BUDGET_MIB=6144 with the card's cap reported per build; the sweep's other cases unchanged; cost measured.

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
