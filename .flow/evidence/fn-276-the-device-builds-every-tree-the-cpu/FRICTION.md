# fn-276 friction

## 2026-10-14, task 1 worker: each lifted limit showed the next

- **What I was doing:** R2, finding which limit each walk frame passes.
- **What slowed it:** a refusal names only the first flag raised. Lifting UnitRounds showed LayGenerations, and lifting that showed HierGens. Finding all three took three device runs and two diagnostic patches.
- **Cost:** about 6 minutes and three heavy slots.
- **What would have removed it:** a test switch that records every stepped limit at its bound and prints each cycle's counts against the limits it would have had. `CYCLE_TRACE` prints the limits but not the counts that pass them.

## 2026-10-14, task 1 worker: the CPU has no per-apex units count

- **What I was doing:** R2, comparing the device's units rounds against the CPU's count.
- **What slowed it:** the CPU census (`grow_census`) records the lattice per cycle, but not the most units one apex grows in a cycle. That count is what the device's rounds track (2u - 1). It took a throwaway depth counter on `Shoot::advance`.
- **Cost:** about 5 minutes.
- **What would have removed it:** the census counting it.

## 2026-10-14, task 1 worker: the shell guard refuses a redirect to a variable path

- **What I was doing:** sending logs to `raw/` through a `$R` variable.
- **What slowed it:** dcg blocked the command, and the logs had to be written to literal paths.
- **Cost:** 1 minute.
- **What would have removed it:** nothing in the repository; this is a local setup matter.

## 2026-10-14, task 1 worker: the CPU check of every preset at every age ran out of memory

- **What I was doing:** fix 1's check that the closed form is at least the CPU's count. It grows every preset at every age from 1 to 80 at three seeds, two at a time.
- **What slowed it:** the run was OOM-killed under its 8 GB cap at the Norway spruce at 78, after 23 minutes and 716 of about 1,950 trees. Two spruces near 80 grown at once do not fit in 8 GB. The rest had to be rerun with one worker for the large trees.
- **Cost:** about 25 minutes of a heavy slot, of which the lost tail was a few minutes.
- **What would have removed it:** the test sizing its workers by each tree's expected size, or one worker above a stated count of phytomers.

## 2026-10-14, task 1 worker: editing a shader while the gate ran failed the gate

- **What I was doing:** fixing the review's finding in `job.wgsl` while the one gate this task gets was running on the commit before.
- **What slowed it:** `every_shader_file_is_checked` reads the WGSL files from disk and compares them with the sources compiled into the binary. The edited file no longer matched, so the one gate run carries a false failure, and the stack fix has no gate of its own.
- **Cost:** the task's only gate shows one red test that is no defect, and the host's merged-branch gate has to carry the proof.
- **What would have removed it:** running the gate from a frozen copy of the tree, or holding source edits until the gate ends.
