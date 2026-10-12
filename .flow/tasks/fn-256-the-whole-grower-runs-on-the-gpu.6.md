# fn-256-the-whole-grower-runs-on-the-gpu.6 Fewer dispatches per cycle

## Description
COUNCIL.md round 4 step 4: re-fuse units kernels to a handful per round (cold compile under 2 s), collapse the hierarchy's narrow upper levels into one barrier-looped workgroup in the ruled order (words identical across caps), explain light's 46 dispatches a cycle; then step 5's re-measure of every preset against F4.

## Acceptance
At most 340 dispatches a cycle on Telperion; words identical across block caps; cold compile under 2 s; per-preset warm build against F4 reported.

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
