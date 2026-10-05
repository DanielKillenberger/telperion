# Friction: fn-204

## 2026-10-05, worker, task 1

- **Doing:** `cargo fmt -p telperion-core` after the move.
- **Slowed by:** the crate is not rustfmt-clean on this toolchain: it reformatted 19 unrelated files, and `git restore` is blocked by the dcg hook, so they were reset from `git show HEAD:<file>` by a script.
- **Cost:** about 5 minutes.
- **Would remove it:** a fmt-clean tree (or a CI fmt check), or `rustfmt` on the touched files only, which is what the rest of this build uses.

## 2026-10-05, worker, task 1

- **Doing:** telperion-core's crate tests.
- **Slowed by:** `suite::species::fixed_beeches_pass_geometry_and_profile_gates_with_repeatable_varied_specimens` failed once on its peak-resident ceiling with other workers' builds on the machine, and passed when run alone.
- **Cost:** one 35 s rerun.
- **Would remove it:** nothing in the repository; a shared machine's memory pressure.
