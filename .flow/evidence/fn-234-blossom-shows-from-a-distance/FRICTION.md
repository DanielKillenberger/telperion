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
