---
satisfies: [R1, R2, R3]
---
# fn-261-the-same-binary-draws-the-same-leaves.1 Leaves repeat run to run

## Description
Implement fn-261 R1-R3 as written; R1 repro first.

## Acceptance
The spec's R1-R3.

## Done summary
The stations pass now numbers leaf-bearing runs by a sorted (stub, first node) key, not an atomic rank. The same binary therefore draws the same leaves and segments on every run: ten builds of every device preset are byte-identical, and so are two stills of every one of the 8 shipped presets. Cause, timings and stills are in REPEAT.md. The device-grown node table's numbering moved to fn-256 R2 (host, 2026-10-11), with grown-probe-and-queue-experiment.patch as its check.

stage: impl-review - ran (codex, round 1 three reviewers NEEDS_WORK on R1 scope, round 2 SHIP after the host's R1 scope decision)
Follow-up: a shared device sort for sweep.wgsl's bitonic and sort.wgsl (noted by the host, left).

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: b0a613c34cbebedf9ed5d6c1db1852e056fa4172, c1538c3b727cbd820665d8158991ca51951ec1b7, 3c0ce35e72774b18261f56498224cb64c4f2827f, 98cab22e1311fc039ace074e02a0f86f726b9027, 5603259f6c5805126ff3ff5b281c24430fe9afb1, 3cbf2167a85e06d197332105ddf4e30c18666f16, d5eb9f2fdd88127dce48a0a5d9b084e32d191bae
- Tests: cargo test --profile ci -p telperion-render (GPU lock): 239 passed, cargo test --profile ci -p telperion-core/-space/-field/-wasm/-jev: green, npm test: 155 passed, REPEATS=10 every_device_preset_builds_the_same_leaves_each_time: green
- PRs: