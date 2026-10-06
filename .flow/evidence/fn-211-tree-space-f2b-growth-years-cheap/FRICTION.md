# fn-211 friction

## 2026-10-06, task 1 worker: a pass's time has no report below the stage

- **Doing:** profiling each pass of a growth year (balance, allot, grow, wake; sketch, re-lay, shade; settle's parts).
- **What slowed it:** `grow_staged`'s `Stage` reports one `Grown` per cycle and one `Settled`; the passes inside them, and the share of each spent on spread work, have no report, and perf is not on the desk. Timing them meant a hand patch over seven files (`profile-instrumentation.patch`), kept out of `src` and reverted.
- **Cost:** about 15 minutes of patching and a rebuild; no rerun.
- **What would remove it:** a per-pass timer behind the stage reports (a `Stage::Pass(name)` or a timing sink the probe passes in), so the next profile is a flag and not a patch. A host decision, since it widens a public enum.
