# fn-234 friction

## 2026-10-09: the first measurement run was refused by the command guard

- **Doing:** task 1, the first run of `blossom_measure` under the GPU lock.
- **Hindrance:** the shell guard (dcg) refused the command because its output redirect went to a path built from a shell variable (`$SP/run.log`). The rule is `core.filesystem:redirect-truncate-dynamic-path`.
- **Cost:** about 1 minute and one re-issued command, with every path spelled out literally.
- **Remedy:** none needed in the repository. A worker writes literal scratchpad paths in redirects.

## 2026-10-09: the cherry photograph is not in this worktree

- **Doing:** viewing the whole-tree stills beside the cherry photograph that fn-217 compared against.
- **Hindrance:** the photograph lives only in fn-217's ignored `raw/distance/`. A worktree does not carry ignored evidence, and this task may not read the main checkout.
- **Cost:** no side-by-side for R1. R3 needs one.
- **Remedy:** track the reference photograph, or its path and source, where a worktree can reach it. One option is a small tracked crop under `.flow/evidence/fn-217-*/` or a references directory.

## 2026-10-09: the guard refused logs written into a new evidence directory

- **Doing:** the R2 re-measure, writing the run log straight into `raw/m2/`.
- **Hindrance:** dcg (`redirect-truncate-root-home`) refuses a truncating redirect into a directory that does not exist yet.
- **Cost:** about 1 minute.
- **Remedy:** write logs to the scratchpad and copy them into evidence afterwards.

## 2026-10-09: a shipped beech took 219 s on the device path in `generation_gpu`

- **Doing:** R3's neutral comparison. The base and the head renders of the shipped presets were compared on the CPU and the device paths (`examples/generation_gpu`, one sample each).
- **Hindrance:** the shipped European beech prepared on the device path (`gpu-render`) took 219 s (`prepareMs` 219,409), against 129 ms on the CPU. The Norway spruce's device run had not finished after several minutes.
  - At that rate the planned 24 renders would have taken about 50 minutes, so I stopped the run.
  - The comparison kept the presets that finish quickly: beech, oak and spruce on the CPU path, and the oak on the device path.
- **Cost:** about 12 minutes of waiting, and the device-path beech and spruce are missing from the neutral comparison.
- **Remedy:** the cause is not known. One cold run is the only measurement, so it may be a first-run compile or a real cost of the mature beech on the fine device path. A host check of the device prepare time for shipped presets at their default age would settle it.
