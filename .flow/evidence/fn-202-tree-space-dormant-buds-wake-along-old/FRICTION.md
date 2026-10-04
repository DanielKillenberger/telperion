# fn-202 friction

## 2026-10-04: the design's continuity claim for the release law was not checked

- **Doing:** building R3, the walks of `rate` and `delay`.
- **What slowed it:** the Design says scaling the first growth unit by the share of the cycle left makes waking continuous. It does not. Four walk runs (about 45 s each) and about 30 minutes of probing showed the remaining jumps come from the partial unit's own discrete decisions: its survival and abortion draws, its phyllotactic count, and the whole unit it adds to the lifespan count. Keying its draws by the years since the node grew and counting its rank by its share removed only part of it.
- **Cost:** about 30 minutes and four walk runs; R3's release-law walks wait on a host decision.
- **What would have removed it:** a standalone walk of a naive version run before the spec was marked ready, as AGENTS.md "Gates and checked claims" asks of an architecture claim.

## 2026-10-04: the spruce walk render was killed for low memory

- **Doing:** R5, seven 80-year spruce trees at seed 1 with sleeping branchlets on the main branches (dormant 0 to 0.8, delay 3, rate 0.15), five stills each, on a temporary branch `r5-spruce-tmp` (fn-194 rebased onto fn-202 cleanly, plus a `--dormant` walk in `space/still.rs`).
- **What slowed it:** Claude Code stopped the background render because the machine ran critically low on memory; no still was written. The reaper's note forbids restarting it unasked.
- **Cost:** about 5 minutes; R5 has no strip yet.
- **What would have removed it:** fewer concurrent worktree builds and renders on the machine, or one tree per process run. This is a local setup issue, reported rather than specced.
