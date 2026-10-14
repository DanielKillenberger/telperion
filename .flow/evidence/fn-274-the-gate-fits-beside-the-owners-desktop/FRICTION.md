# fn-274 friction

## 2026-10-14 - per-test peaks: no `/usr/bin/time` on the desk

- Doing: measuring each `telperion-core` unit test's peak alone (R2), as the brief suggested with `/usr/bin/time -v`.
- Slowed by: GNU time is not installed (`time` is only the bash keyword), and the first sweep wrote 450 empty rows before I noticed. A dcg hook also refuses any `>` to a path held in a variable, so two script drafts were blocked.
- Cost: about 5 minutes and one wasted sweep.
- Would have removed it: a checked peak-measuring wrapper in `scripts/` (wait4's `ru_maxrss` per test, as the Python sweep I wrote does), or GNU time installed.

## 2026-10-14 - the gate's cgroup peak reads the cap, not the tests

- Doing: R3, the gate's peak at 4 threads after the change (`telperion-heavy 9 --threads 4`, `memory.peak` read at the end).
- Slowed by: the gate passed but its scope's `memory.peak` was exactly the 9 GiB cap, because the cgroup counts page cache (the workspace build's artifacts) and fills to whatever cap it is given. The number says nothing about the tests. The run also took 1,696 s wall (telperion-core's lib binary 358 s of it), so a second gate run only to measure is costly.
- Cost: one 28-minute gate run whose peak number is unusable; a second, test-binary-only measurement with an anon sampler.
- Would have removed it: `telperion-heavy` reporting the scope's anonymous peak (`memory.stat anon`, sampled) at exit, beside `memory.peak`.

## 2026-10-14 - the codex reviewers ran the whole-preset repeat test themselves

- Doing: the codex impl-review fan-out (three draws), while my final gate ran in its 8 GB heavy scope.
- Slowed by: two draws ran `every_preset_grows_the_same_tree_again_and_on_any_cores` (3.75 GB, about 145 s) outside `telperion-heavy`, beside the gate, so the desk carried a second heavy run that no admission check counted. The draws took 176 to 325 s.
- Cost: no failure this time; the risk is the owner's desktop swapping, which the one-heavy-run rule exists to stop.
- Would have removed it: a reviewer brief that forbids running whole-preset tests (or wraps them in `telperion-heavy`), or a review sandbox with a memory cap.

## 2026-10-14 - the final gate at an 8 GB cap was OOM-killed

- Doing: the gate once at the smallest cap my measurements supported (telperion-core's unit tests alone peaked at 7,541 MiB anonymous at 4 threads).
- Slowed by: in the gate's scope the same binary reached 7,847 MiB anonymous and the scope was killed at 8 GiB after 320 s, in the suite's whole-preset tests; the cgroup also charges cargo, kernel and cache memory, so 8 GiB leaves no room. Admission waited 2.5 minutes for room first.
- Cost: about 8 minutes and one gate run; the gate is rerun at 9 GB, where it passed before.
- Would have removed it: a gate-scope measurement mode in `telperion-heavy` (run uncapped up to the free room, report the anon and total peak), so the cap is set from the gate's own number and not from a single binary's.
