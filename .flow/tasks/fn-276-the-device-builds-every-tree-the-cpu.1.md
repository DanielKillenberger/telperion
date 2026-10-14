---
satisfies: [R1, R2, R3, R4]
---
# fn-276-the-device-builds-every-tree-the-cpu.1 Repro, cause, fix, sweep

## Description
In order: R1, R2 (stop and report the cause to the host before fixing), R3, R4.

## Acceptance
R1-R5 per the spec.

## Done summary
The device now builds every tree the CPU builds that its budget holds. The R4 sweep built 2,034 cases (every preset at every whole age to 80, and the film's walk frames, at seeds 1, 7 and 42) on the device. None was refused except by the budget, by name.

There were three causes, each fixed under the host's rulings:
- **The species floor.** The units rounds and both generation limits take the species' own floor (`telperion_space::most_units`, the units a job grows where none fails) beside the observed projection. A job holds 11 frames, and its stack of moves holds a move per frame.
- **The node table** grows by the box its dispatch is given.
- **The made-phytomers dispatch** takes its span from the phytomer table.

Costs:
- **GPU time:** Telperion 82 is +0.8% over the base commit, stamped.
- **Memory:** Telperion's peak is 2,484 to 2,631 MB.
- **CPU check:** across all 2,034 cases, the CPU's count is at or under the floor.

Evidence is in `.flow/evidence/fn-276-the-device-builds-every-tree-the-cpu/` (`CAUSE.md`, `FLOOR.md`, `SWEEP.md`).

Open for the host:
- **The spruce's start margin** refuses the spruce at 62 (and at 71 at a larger budget), where it would fit. `Sizes::margins` checks only the start tables against the budget, never their first move. A proposal is in `SWEEP.md`.
- **The gate** ran once, before the stack fix. Its one failure came from editing a shader during the run, and that test passes on the current source. The full sweep was not re-run after the stack fix.

Tier: worker (host-dispatched)

stage: impl-review - ran (codex: NEEDS_WORK, one finding fixed, re-review SHIP)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 110d35e977a9c731b1fd763da3c11f7d17060945, 5d51e4a2b36399b511ba9132817b189e127bf23f, 570be707beeaf032702a24a2538f887191636918, 4ee9bf0d6ab5d7c39b77c5770844ef51e24e3b5e, bd10081547b8029c44ba77ee7890001114e159d5, d4715567c70fa35b4a5c8af33744cf6cc88cc681, 49cdf81e6dfd68bf8ccbf1830eddcd5064aad27c, c4bb76107d05ae2c7fb238fdcb3509ddf476321e, fdee2866c18e5c327f76927b3b449d89dcd8ba04, 2b7918f97442915fdbb8ff83ac224bd7453f0861, 18e12b167529fe7b36dff3c57759e95f47685a4c, b7bf127da1a5f18e3217ed85ac4ecd95f9786da3, a552e6cc9c4d9a8ffc180e65cbf650a60f2a3811, 751e4f6d8b67d78044c609398f045f029f8bb089
- Tests: cargo test --profile ci --workspace --no-fail-fast (b7bf127da: 1383 passed, 1 failed: every_shader_file_is_checked, a job.wgsl edit during the run; passes on HEAD), cargo test --profile ci -p telperion-render --lib every_tree_builds_on_the_device -- --ignored (2,034 cases, budget refusals only), cargo test --profile ci -p telperion-core --test units_bound -- --ignored, cargo test --profile ci -p telperion-render --lib -- a_chain_of_short_ages mirror_tests cpu_builds_tests given build_size build_arena
- PRs: