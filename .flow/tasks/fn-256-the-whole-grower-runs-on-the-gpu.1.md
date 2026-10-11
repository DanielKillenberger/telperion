# fn-256-the-whole-grower-runs-on-the-gpu.1 Spike: busy-cycle replay on the device (COUNCIL early test 3, native)

## Description
COUNCIL.md early test 3, native only: replay one busy cycle of the oak at 80 (and Telperion at 82) on the device, covering both hierarchy schedules (ancestor walks with integer sums; bounded subtrees in workgroups) and the sweep candidates (one workgroup per direction; parallel layer dispatches); measure compile, encoding, execution, longest dispatch, scratch, and agreement with the CPU cycle's phase outputs. The Chrome probe is run separately (no WebGPU in a browser from workers). Evidence SPIKE.md; stop and report.

## Acceptance
SPIKE.md with the measured table per schedule and sweep form, agreement with the CPU phases, and the stop condition's verdict.

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
