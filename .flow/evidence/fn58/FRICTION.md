# Friction reports, fn-58 session

## 2026-09-18 14:37 workspace test run is slow

- **Doing:** running `cargo test --release --workspace` once before a chore commit of three small hooks (a parameter overlay in core, a `--family` flag on two examples, an `ask` subcommand on jev), whose own unit tests had passed in 0.7 s.
- **Hindered by:** the run had taken 9 minutes at the time of writing and was still inside the core `species` suite, which generates full fixed-seed specimens of every catalogue species (oak, spruce, beech, birch) at about 10 s each; 30 suites had reported green before it. The background shell's 10-minute cap sits right at the run's length, so a green run can also be lost to the harness.
- **Cost:** 10 minutes of wall clock (14:27 to 14:37, exit 0, 30 suites green) and a blocked commit, for a change the species suite does not exercise.
- **What would remove it:** a `test:quick` command (or a cargo alias) that runs every suite except the specimen-generating ones, with the full run reserved for landing; or caching the generated specimens under a content key of the preset wire so a run that changes no preset rebuilds none; or scoping the pre-commit run to the crates a diff touches. Any of the three turns the pre-commit check into under a minute.
- **Early return:** not taken; the run is the landing gate and was already past its midpoint.
