# fn-361 friction

## 13 Nov 2026, 03:13 (builder, Opus)

- **Doing:** R3, the narrow suites after adding `/space/ages/6/form/dominance = 0.5` to the oak's values.
- **What hindered it:** `space_presets::every_engine_preset_carries_the_engines_species` asserts the preset's `/space` species equals `telperion_space::oak()`, built in `crates/telperion-space/src/oak.rs`. That function has no dominance at the limb (0.0), so the row cannot pass without a change under `crates/*/src`, which the spec ("no code changes") and the brief forbid. The assertion's only difference is `dominance: 0.5` against `0.0`.
- **Cost:** about 25 minutes and one narrow-suite build before the stop showed; no pin could be repinned.
- **What would have removed it:** the spec checking whether each engine preset row is mirrored in the space crate's species function (the birch's 0.2 is, at `birch.rs:314`) before saying the row needs no code.

## 13 Nov 2026, 04:09 (builder, Opus)

- **Doing:** R4, the panel at 120 with the row, then without it, then 200 and 300.
- **What slowed it:** with no PAUSE since 03:53, an oak at 120 takes about two minutes a tree (03:53 to 04:08 for eight trees) in a 24-thread scope while another worktree's gate runs; the probe grows every tree twice (fn-352's "two grows"), and both runs are drawn whole at every age, so 120 to 300 for two runs is about 70 trees and, by this rate and the trials' slower 200 and 300 trees, two to three hours.
- **Cost:** the wait itself; no rerun so far.
- **Later, 04:42:** the row's panel at 200 was killed by the kernel OOM killer at its 16 GB scope (journal: `telperion-heavy-1076042.scope: Failed with result 'oom-kill'`, 16G memory peak) on seed 1002, after 9 minutes; `run.sh`'s panel stage declares a fixed 16 GB. The rest was rerun through the same `_panel` stage under a 24 GB declaration (`scratchpad/fn361-rest.sh`). Cost: about 10 minutes and the restart.
- **What would have removed it:** `run.sh` taking the panel's declared memory as an option, as it takes the seeds; and the scoreboard reusing the no-row panel from a run on the phase's branch (none exists at `b13c0d1d0`; the newest is `phase2b` on fn-355), so a variant draws one run, not two.
