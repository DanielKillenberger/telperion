# Friction

## 2026-09-19 — Admission policy needs a host decision

Status: NEEDS_HUMAN. Stopped before implementation under the dispatch's
system-design escalation rule.

While tracing the specimen buffers and the species runner for R4, the admission
contract left two decisions unresolved:

- R4 requires a minimum of one admitted seed and a memory bound on any host.
  If even one specimen exceeds the available budget, those requirements cannot
  both hold. The host must decide the behavior in that case.
- The architecture requires a reserve but does not define its size or which
  bound the VmHWM assertion checks. The prediction describes a finished specimen,
  whereas the process also holds temporary allocations: surface construction
  allocates distance, path and frame buffers (`src/surface.rs`), and the species
  digest serializes the skeleton (`tests/species.rs`). Both paths are under
  `crates/telperion-core`. The host must define how the reserve covers this memory
  and what total-process limit R4 requires.

Cost: one preimplementation inspection pass; zero pilot ticks, build/test runs,
or implementation attempts. No repeated measurements were needed to establish
this contract conflict.

What would remove the blocker: a host clarification in the existing fn-87 spec
of the reserve/bound and the behavior when one seed cannot fit. No new spec or
admission policy was authored by this worker.
