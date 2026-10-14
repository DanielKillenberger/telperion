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
