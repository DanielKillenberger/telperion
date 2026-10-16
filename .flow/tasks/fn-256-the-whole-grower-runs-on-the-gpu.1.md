# fn-256-the-whole-grower-runs-on-the-gpu.1 Spike: busy-cycle replay on the device (COUNCIL early test 3, native)

## Description
COUNCIL.md early test 3, native only: replay one busy cycle of the oak at 80 (and Telperion at 82) on the device, covering both hierarchy schedules (ancestor walks with integer sums; bounded subtrees in workgroups) and the sweep candidates (one workgroup per direction; parallel layer dispatches); measure compile, encoding, execution, longest dispatch, scratch, and agreement with the CPU cycle's phase outputs. The Chrome probe is run separately (no WebGPU in a browser from workers). Evidence SPIKE.md; stop and report.

## Acceptance
SPIKE.md with the measured table per schedule and sweep form, agreement with the CPU phases, and the stop condition's verdict.

## Done summary
COUNCIL.md early test 3, native: one busy cycle of the oak at 80 (seed 1) and Telperion at 82 (seed 42) captured from the one cycle and replayed on the device in throwaway kernels, under both hierarchy schedules and both sweep forms; SPIKE.md holds the agreement and timing tables. Stop condition not met: dependency overhead is at most about 14% of the warm budget, every integer output agreed, no device loss. Four design calls go back to the host as NEEDS_HUMAN.

## Evidence
- Commits: a479c9556771ac8c8c7d68182c9e06c1477e9a43
- Tests: cargo test --profile ci -p telperion-space --lib -- light grow::cycle, gpucycle replay raw/oak-80-1.cap and raw/telperion-82-42.cap (agreement + timing), gpucycle chain
- PRs:
